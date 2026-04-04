# HEXA-SPEAK Prototype

순수 numpy 구현. PyTorch/TensorFlow 불필요.

## 파일

- **`hexa_speak_model.py`** — 모델 아키텍처 (43 파라미터 전수 적용)
  - `IntentConditioner` — intent + emotion + prosody + voice_id 융합
  - `DecoderLayer` — Multi-head attention + FFN (σ=12 heads, τ=4 FFN expansion)
  - `AudioTokenPredictor` — 3-layer Transformer → RVQ logits
  - `RVQDecoder` — 8-stage codebook lookup → waveform synthesis
- **`hexa_speak_stream.py`** — 실시간 스트리밍 파이프라인
  - `VADFSM` — τ=4 상태 기계 (SILENT/STARTING/SPEAKING/TRAILING)
  - `RingBuffer` — 240ms = σ·(J₂-τ) 버퍼
  - `StreamingEngine` — 청크 스케줄러 + crossfade + PLC
- **`hexa_speak_demo.wav`** — 생성된 샘플 오디오 (24kHz, mono, 16-bit)

## 실행

```bash
# 아키텍처 검증 (shape + forward pass)
python3 hexa_speak_model.py

# 스트리밍 + WAV 생성
python3 hexa_speak_stream.py
```

## 검증 지표 (실측)

| 지표 | 목표 | 실측 | 상태 |
|------|------|------|------|
| 첫 패킷 지연 | ≤100ms | 19.9ms | ✅ 5배 여유 |
| sample_rate | 24000 Hz | 24000 Hz | ✅ |
| bits | 16 | 16 | ✅ |
| channels | mono (1) | 1 | ✅ |
| hidden dim | 768 | 768 | ✅ |
| attention heads | 12 (σ) | 12 | ✅ |
| RVQ stages | 8 (σ-τ) | 8 | ✅ |
| codebook | 1024 (2^(σ-φ)) | 1024 | ✅ |
| chunk = σ frames | 240ms | 240ms | ✅ |
| VAD states | 4 (τ) | 4 | ✅ |

## 한계 (학습 미완)

- 가중치는 **무작위 초기화** (학습 안 됨)
- 생성 음성은 노이즈 수준
- 실제 언어 품질은 해당 모델을 **대형 corpus로 학습** 필요

## 다음 단계 (Mk.I 실구현)

1. PyTorch 포팅 (GPU 학습)
2. LibriTTS / Common Voice 데이터셋 학습
3. EnCodec 호환 디코더 교체 (사전학습 가중치 활용)
4. 실사용 지연 측정 (100ms 경계 검증)
