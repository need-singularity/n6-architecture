# 6 Consciousness States — 12-Variable Profiles

BrainWire Universal Consciousness Controller(UCC)가 지원하는 6가지 의식 상태 프로파일.

각 상태는 12변수 타겟 벡터 + 엔벨로프 타이밍 + PID 힌트 + 안전 오버라이드로 정의됨.

---

## 상태 요약

| 상태 | 카테고리 | 핵심 드라이버 | 특징 |
|------|----------|--------------|------|
| **THC** | cannabinoid | eCB, DA | 동기화↑, NE↓, 이완형, G=골든존 |
| **LSD** | psychedelic | 5HT (3.5x) | 탈동기화↓, 엔트로피↑, PFC↑ |
| **Psilocybin** | psychedelic | 5HT + Theta (3.5x) | 신비 경험, Theta 폭발 |
| **DMT** | psychedelic | 5HT (4.5x) | 30초 onset, 현실 대체, Sensory 5.0x |
| **MDMA** | empathogen | 5HT + DA | Body 3.0x, 공감↑, 동기화↑ |
| **Flow** | endogenous | eCB, DA | Alpha↑, Coherence 2.5x (최고), 안전 |

---

## 12-Variable Comparison Matrix

```
Variable     THC    LSD    Psilo  DMT    MDMA   Flow
──────────────────────────────────────────────────────
V1  DA       2.5    1.8    1.5    2.2    2.5    1.8
V2  eCB      3.0    1.3    1.4    1.2    1.8    2.0
V3  5HT      1.5    3.5    3.0    4.5    4.0    1.3
V4  GABA     1.8    0.6    0.7    0.3    1.2    1.5
V5  NE       0.4    2.0    1.6    2.5    2.0    1.2
V6  Theta    2.5    3.0    3.5    4.0    1.5    2.0
V7  Alpha    0.5    0.3    0.4    0.1    1.2    1.5
V8  Gamma    1.8    2.5    2.0    3.5    2.0    2.0
V9  PFC      0.5    1.5    1.2    2.0    1.8    0.7
V10 Sensory  2.0    3.5    2.5    5.0    2.5    1.5
V11 Body     2.5    1.5    2.0    0.8    3.0    1.8
V12 Cohere   2.0    0.4    0.5    0.2    1.8    2.5
```

---

## 패턴 분류

### 이완형 (Sync-Up): THC, Flow
- eCB 주도, Coherence 증가, NE 억제
- Alpha/PFC 억제 (자기비판 OFF)
- G=D×P/I 골든존 근처

### 엔트로피형 (Desync): LSD, Psilocybin, DMT
- 5HT 주도, Coherence 감소, NE 활성화
- Alpha 거의 소멸, PFC 활성화 (사고 가속)
- G=0 (대칭 프로파일)

### 공감형 (Hybrid): MDMA
- 5HT + DA 동시 폭발, Coherence 증가
- Alpha 유지 (편안한 각성)
- Body 최고 (3.0x)

---

## 개별 프로파일 상세

### THC Strong (25%)

- **메커니즘**: CB1 수용체 작용 → eCB 시스템 활성화 → 12변수 변조
- **온셋**: 300초 (sigmoid)
- **유지**: 3600초
- **G=D×P/I**: 0.4731 (골든존 내)
- **안전**: 세션 60분, NE 비상 모니터링
- **YAML**: [`brainwire/profiles/thc.yaml`](../brainwire/profiles/thc.yaml)

### LSD (100ug)

- **메커니즘**: 5-HT2A 강력 작용 → DMN 붕괴 → 엔트로피 폭발
- **온셋**: 1800초 (sigmoid)
- **유지**: 21600초 (6시간)
- **특이사항**: NE↑, PFC↑ (THC와 반대), Coherence↓ (탈동기화)
- **안전**: 세션 40분, GABA/NE 비상 모니터링
- **YAML**: [`brainwire/profiles/lsd.yaml`](../brainwire/profiles/lsd.yaml)

### Psilocybin (25mg)

- **메커니즘**: 5-HT2A (LSD보다 짧고 부드러움) + kappa-opioid
- **온셋**: 1200초 (sigmoid)
- **유지**: 14400초 (4시간)
- **특이사항**: Theta 3.5x (신비 경험 마커), Body load 2.0x
- **안전**: 세션 40분, GABA 비상 모니터링
- **YAML**: [`brainwire/profiles/psilocybin.yaml`](../brainwire/profiles/psilocybin.yaml)

### DMT Breakthrough (30mg inhaled)

- **메커니즘**: 5-HT2A 초강력 + sigma1 + TAAR1
- **온셋**: 30초 (exponential) — 전 물질 중 가장 빠름
- **유지**: 600초 (10분)
- **특이사항**: Sensory 5.0x (현실 대체), Coherence 0.2x (최대 엔트로피)
- **안전**: 세션 20분, GABA/NE/Sensory 비상, 첫 세션 Sensory 3.0x 제한
- **YAML**: [`brainwire/profiles/dmt.yaml`](../brainwire/profiles/dmt.yaml)

### MDMA (125mg)

- **메커니즘**: SERT/DAT/NET 역전달 → 5HT/DA/NE 동시 폭발 + 옥시토신
- **온셋**: 1800초 (sigmoid)
- **유지**: 10800초 (3시간)
- **특이사항**: Body 3.0x (최고), Alpha 유지 (편안한 각성), Coherence↑
- **안전**: 세션 40분, 5HT/NE 비상, 체온 모니터링
- **YAML**: [`brainwire/profiles/mdma.yaml`](../brainwire/profiles/mdma.yaml)

### Flow State

- **메커니즘**: 내인성 — eCB + DA + NE sweet spot
- **온셋**: 600초 (linear)
- **유지**: 7200초 (2시간)
- **특이사항**: Alpha 1.5x (유일하게 α 증가), Coherence 2.5x (전 상태 최고)
- **안전**: 세션 90분 (가장 긴 허용), 비상 변수 없음
- **YAML**: [`brainwire/profiles/flow.yaml`](../brainwire/profiles/flow.yaml)

---

## CLI 사용

```bash
# 단일 상태 벤치마크
python -m brainwire.bench bench thc --tier 4

# 복수 상태 비교
python -m brainwire.bench compare thc lsd dmt mdma flow --tier 3

# 모든 상태 × 모든 Tier
python -m brainwire.bench all

# 상태 블렌딩
python -m brainwire.calc blend --states thc flow --weights 0.7 0.3

# G=D×P/I 분석
python -m brainwire.eeg_feedback
```
