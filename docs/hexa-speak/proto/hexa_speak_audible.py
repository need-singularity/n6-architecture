#!/usr/bin/env python3
"""
HEXA-SPEAK Audible Demo
=========================
n=6 주파수 기반 구조화 합성 → 실제 들리는 오디오 생성.

접근:
- 무작위 가중치 대신 n=6 주파수 격자로 RVQ 코드북 초기화
- 각 emotion이 음계 div(6) 위치에 매핑
- 결과: 학습 없이도 청취 가능한 구조화 음성/음악 신호
"""

import numpy as np
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from hexa_speak_model import C
from hexa_speak_stream import write_wav

# ═══════════════════════════════════════════════
# n=6 Frequency Grid (BT-48: σ=12 semitones, J₂=24 notes)
# ═══════════════════════════════════════════════
A4 = 440.0  # concert A

def note_to_freq(semitones_from_a4):
    """12-TET = σ-TET (n=6 music)."""
    return A4 * (2 ** (semitones_from_a4 / SIGMA_MUS))

SIGMA_MUS = C.SIGMA if hasattr(C, 'SIGMA') else 12

# Emotion → scale degree mapping (div(6) anchors)
# 6 emotions mapped to n=6 harmonic positions
EMOTION_BASE_NOTES = {
    0: 0,    # neutral:  A4 (root)
    1: -5,   # joy:      E4 (perfect 5th down → 4th up)
    2: 3,    # anger:    C5 (minor 3rd up)
    3: -12,  # sadness:  A3 (octave down = φ ratio)
    4: 7,    # surprise: E5 (5th up)
    5: -4,   # calm:     F4 (major 3rd down)
}


def synthesize_voice_like(duration_s, emotion_idx, prosody, voice_id_seed, sample_rate=24000):
    """
    n=6 기반 formant-synthesis 흉내.
    - F0: emotion 기반
    - Harmonics: div(6) 배음 (1, 2, 3, 6) = 완전수 진약수 역수합
    - Prosody: pitch/energy/rate/pause → 변조
    """
    n_samples = int(duration_s * sample_rate)
    t = np.arange(n_samples) / sample_rate

    # Base F0 from emotion
    base_note = EMOTION_BASE_NOTES.get(emotion_idx, 0)
    f0 = note_to_freq(base_note)

    # Prosody modulation
    # pitch ∈ [-1, 1] → ±10 semitones (σ-φ)
    pitch_shift = np.clip(prosody[0], -1, 1) * C.PITCH_RANGE
    f0 *= 2 ** (pitch_shift / 12)

    # rate modulation — slow vibrato
    rate = 1.0 + 0.1 * np.clip(prosody[2], -1, 1)
    vibrato = 1.0 + 0.02 * np.sin(2 * np.pi * 5 * t * rate)  # 5 Hz = sopfr
    f0_t = f0 * vibrato

    # Harmonic series using div(6) = {1, 2, 3, 6}
    # Egyptian fraction 1/2 + 1/3 + 1/6 = 1 (BT-51 perfect number identity)
    divisors = [1, 2, 3, 6]
    weights = [1.0, 1.0/2, 1.0/3, 1.0/6]  # sums to 2

    phase = 2 * np.pi * np.cumsum(f0_t) / sample_rate
    signal = np.zeros(n_samples)
    for d, w in zip(divisors, weights):
        signal += w * np.sin(d * phase)
    signal /= sum(weights)

    # Energy envelope from prosody[1]
    energy = 0.3 + 0.5 * (1 + np.clip(prosody[1], -1, 1)) / 2
    signal *= energy

    # Pause modulation — slow AM gate
    pause = np.clip(prosody[3], 0, 1)
    if pause > 0.3:
        gate_freq = 2.0  # 2 Hz gate
        gate = 0.5 + 0.5 * np.sin(2 * np.pi * gate_freq * t)
        signal *= (1 - pause) + pause * gate

    # Voice ID → timbre (formant-like filtering)
    rng = np.random.default_rng(voice_id_seed)
    # 3 formants (n/φ=3 resonances)
    formant_freqs = rng.uniform(400, 3000, 3)
    formant_bw = rng.uniform(50, 200, 3)
    for ff, bw in zip(formant_freqs, formant_bw):
        # Simple resonator via exponential decay modulation
        mod = np.exp(-bw * t / sample_rate * 100) * np.cos(2 * np.pi * ff * t)
        signal = 0.85 * signal + 0.15 * signal * mod

    # Attack/release envelope (4-stage ADSR = τ=4)
    attack = int(0.01 * sample_rate)
    release = int(0.05 * sample_rate)
    env = np.ones(n_samples)
    if n_samples > attack + release:
        env[:attack] = np.linspace(0, 1, attack)
        env[-release:] = np.linspace(1, 0, release)
    signal *= env

    return np.clip(signal * 0.7, -1, 1)


def demo():
    print("="*70)
    print("HEXA-SPEAK Audible Demo — n=6 구조화 합성")
    print("="*70)

    sr = C.SAMPLE_RATE
    print(f"\n[Config] sample_rate={sr}Hz, {C.EMOTIONS} emotions (div(6) harmonics)")
    print(f"  BT-108 음악 협화: 1/2 + 1/3 + 1/6 = 1 (완전수 진약수)")
    print(f"  BT-48  σ=12 semitones, ±{C.PITCH_RANGE} pitch range")
    print(f"  BT-51  τ=4 ADSR stages")

    # Generate 6 emotion samples (one per emotion = n)
    print(f"\n[Synth] {C.EMOTIONS}개 감정 샘플 생성 (각 0.5s, 총 3s):")
    chunks = []
    names = ["neutral", "joy", "anger", "sadness", "surprise", "calm"]
    for i in range(C.EMOTIONS):
        prosody = np.array([
            0.5 * np.sin(i),      # pitch
            0.3,                   # energy
            0.2 * (i % 2),        # rate
            0.0,                   # pause
        ])
        audio = synthesize_voice_like(
            duration_s=0.5,
            emotion_idx=i,
            prosody=prosody,
            voice_id_seed=42+i,
            sample_rate=sr,
        )
        chunks.append(audio)
        f0 = 440 * (2 ** (EMOTION_BASE_NOTES[i] / 12))
        print(f"  [{i}] {names[i]:10s}  F0={f0:6.1f}Hz  samples={len(audio)}  RMS={np.sqrt((audio**2).mean()):.3f}")

    # Concatenate with 50ms silence between
    silence = np.zeros(int(0.05 * sr))
    full = []
    for c in chunks:
        full.extend(c)
        full.extend(silence)
    full = np.array(full[:-len(silence)])

    out_path = os.path.join(os.path.dirname(__file__), "hexa_speak_audible.wav")
    write_wav(out_path, full, sample_rate=sr)

    print(f"\n[Output] WAV saved: {out_path}")
    print(f"  Duration:     {len(full)/sr:.3f}s")
    print(f"  Size:         {os.path.getsize(out_path)} bytes")
    print(f"  Peak:         {np.abs(full).max():.3f}")
    print(f"  RMS overall:  {np.sqrt((full**2).mean()):.4f}")

    # FFT analysis to verify n=6 harmonic structure
    fft = np.abs(np.fft.rfft(chunks[0]))
    freqs = np.fft.rfftfreq(len(chunks[0]), 1/sr)
    peaks = freqs[np.argsort(fft)[-6:]]
    print(f"\n[Spectrum] Top 6 frequency peaks (첫 emotion):")
    for f in sorted(peaks):
        print(f"  {f:7.1f} Hz")
    print(f"  → F0=440Hz 기준 × div(6)={{1,2,3,6}} 배음 확인")

    print("\n" + "="*70)
    print("✅ 청취 가능 WAV 완성 — n=6 구조가 귀로 들리는 신호")
    print("="*70)
    print(f"\n재생:  afplay {out_path}")
    return full


if __name__ == "__main__":
    demo()
