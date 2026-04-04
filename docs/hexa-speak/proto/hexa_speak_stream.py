#!/usr/bin/env python3
"""
HEXA-SPEAK Streaming Pipeline
===============================
Real-time audio synthesis pipeline with:
  - VAD FSM (τ=4 states, anima/vad-rs compatible)
  - Ring buffer (240ms = σ·(J₂-τ))
  - First-packet latency 100ms = (σ-φ)²
  - Chunk scheduler (12 frames = σ)
  - Crossfade (6ms = n)
  - PLC (packet loss concealment, gap_max = 60ms)
"""

import numpy as np
from collections import deque
from enum import Enum
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from hexa_speak_model import C, HexaSpeakModel


class VADState(Enum):
    """τ=4 FSM states (anima/vad-rs pattern)."""
    SILENT = 0
    STARTING = 1   # pre-speech buffer (lookback=5 frames)
    SPEAKING = 2
    TRAILING = 3   # post-speech (1.5s = turn_taking_ms)


class VADFSM:
    """4-state VAD machine with sopfr=5 frame lookback."""
    def __init__(self, energy_threshold=0.01):
        self.state = VADState.SILENT
        self.lookback = deque(maxlen=C.VAD_LOOKBACK)  # 5 frames
        self.trailing_frames_left = 0
        self.energy_threshold = energy_threshold

    def step(self, frame_energy):
        """Process one 20ms frame, return new state."""
        self.lookback.append(frame_energy > self.energy_threshold)
        speech = sum(self.lookback) >= C.N_PHI if hasattr(C, 'N_PHI') else 3

        if self.state == VADState.SILENT:
            if speech:
                self.state = VADState.STARTING
        elif self.state == VADState.STARTING:
            self.state = VADState.SPEAKING if speech else VADState.SILENT
        elif self.state == VADState.SPEAKING:
            if not speech:
                self.state = VADState.TRAILING
                # trailing = turn_taking_ms / hop_ms = 1500/20 = 75 frames
                self.trailing_frames_left = C.TURN_TAKING_MS // C.HOP_MS
        elif self.state == VADState.TRAILING:
            self.trailing_frames_left -= 1
            if speech:
                self.state = VADState.SPEAKING
            elif self.trailing_frames_left <= 0:
                self.state = VADState.SILENT

        return self.state


class RingBuffer:
    """240ms ring buffer = σ·(J₂-τ) ms = 12·20."""
    def __init__(self):
        self.max_samples = C.RING_BUFFER_MS * C.SAMPLE_RATE // 1000  # 5760
        self.buf = deque(maxlen=self.max_samples)

    def push(self, chunk):
        self.buf.extend(chunk)

    def pull(self, n):
        if len(self.buf) < n:
            return None
        out = np.array([self.buf.popleft() for _ in range(n)])
        return out

    def level_ms(self):
        return len(self.buf) * 1000 // C.SAMPLE_RATE


def crossfade(prev_tail, curr_head):
    """6ms = n·ms crossfade between chunks."""
    n = C.CROSSFADE_MS * C.SAMPLE_RATE // 1000  # 144 samples
    n = min(n, len(prev_tail), len(curr_head))
    if n == 0: return curr_head
    fade_out = np.linspace(1, 0, n)
    fade_in = np.linspace(0, 1, n)
    mixed = prev_tail[-n:] * fade_out + curr_head[:n] * fade_in
    return np.concatenate([mixed, curr_head[n:]])


def plc(prev_samples, gap_ms):
    """Packet loss concealment: extend last 6ms crossfade with decay."""
    if gap_ms > C.PLC_GAP_MS:
        # Gap too long, emit silence
        return np.zeros(gap_ms * C.SAMPLE_RATE // 1000)
    # Loop last segment with fade
    n_out = gap_ms * C.SAMPLE_RATE // 1000
    n_loop = min(len(prev_samples), C.CROSSFADE_MS * C.SAMPLE_RATE // 1000)
    if n_loop == 0: return np.zeros(n_out)
    tail = prev_samples[-n_loop:]
    reps = (n_out // n_loop) + 1
    filled = np.tile(tail, reps)[:n_out]
    fade = np.linspace(1, 0.3, n_out)
    return filled * fade


class StreamingEngine:
    """End-to-end streaming synthesis with chunk scheduling."""
    def __init__(self, model: HexaSpeakModel):
        self.model = model
        self.ring = RingBuffer()
        self.vad = VADFSM()
        self.first_packet_sent = False
        self.last_tail = np.zeros(0)
        self.stats = {
            "chunks_generated": 0,
            "samples_produced": 0,
            "first_packet_ms": None,
            "state_transitions": [],
        }

    def synthesize_chunk(self, intent, emotion, prosody, voice_id):
        """Generate chunk_frames=σ=12 frames at a time."""
        import time
        t0 = time.time()
        chunk_sec = C.CHUNK_FRAMES / C.FPS  # 12/50 = 0.24s
        out = self.model.generate(intent, emotion, prosody, voice_id, duration_s=chunk_sec)
        latency_ms = (time.time() - t0) * 1000

        audio = out["audio"][0]  # B=1
        # Crossfade with previous chunk tail
        if len(self.last_tail) > 0:
            audio = crossfade(self.last_tail, audio)
        self.last_tail = audio[-(C.CROSSFADE_MS * C.SAMPLE_RATE // 1000):]

        # VAD processing frame-by-frame
        for i in range(C.CHUNK_FRAMES):
            start = i * C.SAMPLES_PER_FRAME
            end = start + C.SAMPLES_PER_FRAME
            frame = audio[start:end]
            energy = np.sqrt((frame ** 2).mean())
            new_state = self.vad.step(energy)
            self.stats["state_transitions"].append(new_state.name)

        self.ring.push(audio)
        self.stats["chunks_generated"] += 1
        self.stats["samples_produced"] += len(audio)

        if not self.first_packet_sent:
            self.first_packet_sent = True
            self.stats["first_packet_ms"] = round(latency_ms, 1)

        return audio, latency_ms


def write_wav(path, audio, sample_rate=24000):
    """Write 16-bit PCM WAV without scipy."""
    import struct
    audio = np.clip(audio, -1.0, 1.0)
    pcm = (audio * 32767).astype(np.int16)
    with open(path, "wb") as f:
        n = len(pcm)
        data_size = n * 2
        f.write(b'RIFF')
        f.write(struct.pack('<I', 36 + data_size))
        f.write(b'WAVE')
        f.write(b'fmt ')
        f.write(struct.pack('<I', 16))
        f.write(struct.pack('<H', 1))       # PCM
        f.write(struct.pack('<H', 1))       # mono
        f.write(struct.pack('<I', sample_rate))
        f.write(struct.pack('<I', sample_rate * 2))
        f.write(struct.pack('<H', 2))
        f.write(struct.pack('<H', 16))
        f.write(b'data')
        f.write(struct.pack('<I', data_size))
        f.write(pcm.tobytes())


def demo():
    print("="*70)
    print("HEXA-SPEAK Streaming Demo — AI Voice Synthesis Pipeline")
    print("="*70)

    print("\n[Init] Building model + streaming engine...")
    model = HexaSpeakModel(seed=6)
    engine = StreamingEngine(model)

    print(f"  Ring buffer:     {C.RING_BUFFER_MS}ms = {engine.ring.max_samples} samples")
    print(f"  VAD states:      {C.VAD_STATES} (τ)")
    print(f"  Chunk frames:    {C.CHUNK_FRAMES} (σ) = {C.CHUNK_FRAMES * C.HOP_MS}ms")
    print(f"  Crossfade:       {C.CROSSFADE_MS}ms (n)")
    print(f"  First-pkt target: {C.FIRST_PACKET_MS}ms ((σ-φ)²)")

    print("\n[Stream] Generating 3 chunks (0.72s total speech)...")
    rng = np.random.default_rng(42)
    chunks = []
    for i in range(3):
        # Different emotions per chunk
        emo_idx = i % C.EMOTIONS
        emotion = np.eye(C.EMOTIONS)[[emo_idx]]
        intent = rng.standard_normal((1, C.EMBED_DIM)).astype(np.float32) * 0.5
        prosody = rng.standard_normal((1, C.PROSODY_DIMS))
        voice_id = rng.standard_normal((1, C.VOICE_ID_DIM)) * 0.3

        audio, latency = engine.synthesize_chunk(intent, emotion, prosody, voice_id)
        chunks.append(audio)
        print(f"  chunk {i+1}: {len(audio)} samples, latency={latency:.1f}ms, "
              f"vad={engine.vad.state.name}, ring={engine.ring.level_ms()}ms")

    full = np.concatenate(chunks)
    print(f"\n[Output] Total: {len(full)} samples = {len(full)/C.SAMPLE_RATE:.3f}s")

    # Write WAV
    out_path = os.path.join(os.path.dirname(__file__), "hexa_speak_demo.wav")
    write_wav(out_path, full)
    print(f"  WAV saved: {out_path}")
    print(f"  Size: {os.path.getsize(out_path)} bytes")

    # Stats
    print("\n[Stats]")
    print(f"  first_packet_ms:  {engine.stats['first_packet_ms']}ms (target ≤100ms)")
    print(f"  chunks:           {engine.stats['chunks_generated']}")
    print(f"  samples:          {engine.stats['samples_produced']}")
    print(f"  VAD transitions:  {len(engine.stats['state_transitions'])} frames")
    unique_states = set(engine.stats['state_transitions'])
    print(f"  States visited:   {unique_states}")

    print("\n" + "="*70)
    print("✅ Streaming pipeline 완료 — WAV 파일 출력 성공")
    print("="*70)


if __name__ == "__main__":
    demo()
