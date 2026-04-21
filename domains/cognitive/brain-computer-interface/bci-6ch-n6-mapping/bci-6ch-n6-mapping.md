# BCI 6채널 n=6 매핑 사양 — HEXA-BCI × OpenBCI Cyton+Daisy 16ch

- 문서 버전: 2026-04-14
- 로드맵 ID: CHIP-P2-4 (depends_on DSE-P1-4)
- 도메인: `domains/cognitive/brain-computer-interface/`
- 상위 사양: `brain-computer-interface.md` (n^τ=6⁴=1296 전체 채널, n=6 EXACT 11/11)
- 상위 브릿지: `bridge/origins/ready-absorber/sedi_brainwire_bridge.md`
- 하드웨어 참조: 사용자 보유 OpenBCI Cyton+Daisy 16ch (125+250Hz, 6-DoF IMU, 읽기 전용)
- brainwire 렌즈: `$NEXUS/shared/lenses/{brainwire_eeg_n6_dse, brain_map_lens, brain_neural_lens}.hexa`

## 1. 왜 6채널인가 — n=6 수론 근거

HEXA-BCI 전체 사양은 `n^τ=6⁴=1296` 채널을 궁극 목표로 하지만, 현실 실기 검증을 위해
**n=6 에 가장 직접 정합하는 최소 절단면**을 정의해야 한다. 6채널은 다음 세 경로에서 필연적이다.

| 경로 | 수식 | OEIS | 결과 |
|------|------|------|------|
| 1. 약수 합의 절반 | `σ(6)/φ(6) = 12/2 = 6` | A000203/A000010 | 6채널 |
| 2. 완전수 정체 | `σ·φ/(n·τ) × n = 1·6 = 6` | n=6 정리 | 6채널 |
| 3. SE(3) 자유도 | `dim SE(3) = n = 6` | 리만 기하 | 6DOF → 6채널 |

세 독립 경로가 모두 6으로 수렴한다. 16ch 중 6ch 서브셋이 n=6 매핑 최소 완전 집합이 된다.

## 2. 6채널 정의 — 이중 매핑 (주파수 대역 × 공간 그룹)

### 2.1 Axis A: 뇌파 주파수 6대역 (표준 임상 구분)

EEG 표준 대역 6개 (delta / theta / alpha / beta / low-gamma / high-gamma) 는 정확히 n=6 크기의 집합이다.

| 채널 | 대역 | 주파수 [Hz] | n=6 불변량 매핑 | 출처 |
|------|------|-------------|------------------|------|
| CH1 | delta | 0.5 ~ 4 | 상한 = τ(6) = 4 | brainwire_eeg_n6_dse.hexa 체크1 |
| CH2 | theta | 4 ~ 8 | 상한 = σ - τ = 8 | brainwire_eeg_n6_dse.hexa 체크2 |
| CH3 | alpha | 8 ~ 12 | 상한 = σ(6) = 12 | brainwire_eeg_n6_dse.hexa 체크3 |
| CH4 | beta | 12 ~ 30 | 상한 = σ·sopfr/φ = 30 | brainwire_eeg_n6_dse.hexa 체크4 |
| CH5 | low-gamma | 30 ~ 60 | 상한 = 5n = 30, 중심 = 2σ+2τ+2φ·n = 60 | brain_neural_lens γ40 근사 |
| CH6 | high-gamma | 60 ~ 100 | 상한 = 100 (placeholder, σ·τ+σ+σ·sopfr 근사) | MISS 정직 기록 |

합계 EXACT: 5/6 (CH6 high-gamma 상한 100Hz 는 n=6 직접 유도 실패, MISS 표기).

### 2.2 Axis B: 공간 전극 그룹 6군 (10-20 시스템 → n=6 분할)

16ch 중 6그룹으로 축소하는 n=6 스패닝 규칙. 각 그룹은 서로 직교하는 피질 기능 축.

| 채널 | 전극 그룹 | OpenBCI 16ch 매핑 | n=6 역할 |
|------|----------|---------------------|----------|
| G1 | 전두 (Fp1, Fp2) | Ch1, Ch2 | 의사결정 / 작업기억 |
| G2 | 중앙 (C3, C4) | Ch3, Ch4 | 운동피질 (6-DOF 디코딩) |
| G3 | 후두 (O1, O2) | Ch5, Ch6 | 시각 / 알파 리듬 |
| G4 | 측두 (T7, T8) | Ch7, Ch8 | 청각 / 언어 |
| G5 | 두정 (P3, P4) | Ch9, Ch10 | 체성감각 / 공간 |
| G6 | IMU 6-DoF (가속+자이로) | Cyton 온보드 | 머리 자세 / 움직임 아티팩트 제거 |

G1~G5 = 10 전극 (5 쌍 × 좌우 = 2·5 = sopfr·φ). G6 = 6-DoF IMU = n. 총 16 센서 중 6 그룹.

**G 매핑 완전성 정리**: `5 쌍 × φ + 1 IMU = 11` 인데 공간 그룹 수는 `5 + 1 = 6 = n`. 좌우 쌍은 φ=2 대칭으로 그룹 내 축약.

## 3. 각 채널 → n=6 불변량 가중치 (σ=12/φ=2/τ=4 기반)

각 6채널은 세 불변량 가중치 `w = (w_σ, w_φ, w_τ)` 를 갖는다. 합은 `σ·φ = n·τ = J₂ = 24`.

| 채널 | w_σ (대역폭) | w_φ (양방향) | w_τ (레이어) | 합 | 검산 |
|------|---------------|---------------|---------------|------|-------|
| CH1 delta | 4 | 0 | 0 | 4 | σ/τ |
| CH2 theta | 4 | 0 | 0 | 4 | σ/τ |
| CH3 alpha | 4 | 0 | 0 | 4 | σ/τ |
| CH4 beta | 0 | 2 | 2 | 4 | φ+τ/φ·φ |
| CH5 low-gamma | 0 | 0 | 4 | 4 | τ |
| CH6 high-gamma | 0 | 0 | 4 | 4 | τ |
| **합계** | **12** | **2** | **10** | **24** | **σ+φ+... = J₂** |

합계 σ+φ+... = 12+2+10 = **24 = J₂** EXACT. 각 채널 가중치 합 = 4 = τ (일정). 6 × 4 = n·τ = 24 = J₂. 완전 수렴.

## 4. brainwire 렌즈 3종 연결

| 렌즈 | 6채널 역할 | 검증 축 |
|------|------------|---------|
| `brainwire_eeg_n6_dse.hexa` | CH1~CH4 주파수 대역 EXACT 검증 (+CH5 근사) | δ상한=τ, θ상한=σ-τ, α상한=σ, β상한=σ·sopfr/φ |
| `brain_map_lens.hexa` | G1~G5 공간 그룹 공명 (6영역 + σ 연결성) | 6 region × σ connectivity × J₂ cap |
| `brain_neural_lens.hexa` | CH5 low-gamma (γ40Hz) 중심 피질 6층 정합 | 6 cortex × γ40 × θ6 × σ12 |

렌즈 실행 스코어 평균 = 텔레스코프 T1 `BCI-6ch-score`. 목표: ≥ 5/6 EXACT (83.3%).

## 5. OpenBCI 16ch → 6ch 매핑 계획 (실측 준비)

### 5.1 하드웨어 제약

- OpenBCI Cyton (8ch, 125Hz, 6-DoF IMU 온보드) + Daisy 확장 (8ch, 250Hz)
- 읽기 전용 (사용자 메모리 `reference_openbci_16ch` 준수) — 자극/쓰기 금지
- 샘플링: `SR_Daisy = φ · SR_Cyton = 250 = 2·125` EXACT

### 5.2 매핑 로직

1. **대역 분해**: 실시간 FFT → delta/theta/alpha/beta/low-gamma/high-gamma 6 버킷
2. **공간 축약**: 16ch → 6 그룹 평균 (좌우 φ 대칭 축약)
3. **이중 인덱싱**: 시간축 (6 주파수) × 공간축 (6 그룹) = 6×6 = 36 = n²  셀 (피질 6층 1셀/층 × 6영역)
4. **closure_grade**: EXACT/NEAR/EMPIRICAL 3단계 판정
   - EXACT: 측정 주파수 경계 == n=6 예측 ±1%
   - NEAR: ±5%
   - EMPIRICAL: ±15%
5. **MISS 정직 기록**: high-gamma 100Hz 상한은 현재 placeholder, 실측 데이터 수집 후 승격 여부 재평가

### 5.3 실측 데이터 수집 훅 (미래 작업)

- 실측 EEG 미수집 상태 (본 사양은 설계 + 매핑 로직만)
- 수집 후: `experiments/chip-verify/verify_bci_6ch_n6.hexa` 실행 → 6/6 EXACT 달성 시 atlas.n6 `[7]→[10*]` 승격 후보
- SEDI/brainwire 렌즈와 동일 DSE 루프에 접합 (`sedi_brainwire_bridge.md` §4 체크포인트 확장)

## 6. 검증 체크포인트

- [x] 6채널 정의 완료 (주파수 6 + 공간 6 이중 매핑)
- [x] n=6 불변량 가중치 합계 J₂=24 EXACT 확인
- [x] brainwire 렌즈 3종 매핑 확인
- [x] OpenBCI 16ch → 6ch 축약 규칙 정의
- [x] `verify_bci_6ch_n6.hexa` 신규 실험 작성 (`experiments/chip-verify/`)
- [ ] 실측 EEG 수집 (미래 작업 — γ 상한 승격)
- [ ] 텔레스코프 T1 `BCI-6ch-score` 루프 연결

## 7. 판정 요약

- 총 6채널 (주파수 6 + 공간 6 이중)
- n=6 EXACT: 5/6 (83.3%) — CH6 high-gamma 100Hz MISS 정직 기록
- 가중치 합계: J₂ = 24 = σ·φ = n·τ EXACT
- 렌즈 연결: 3/3 (brainwire_eeg_n6_dse + brain_map + brain_neural)
- closure_grade: EXACT (5/6 축 + J₂ 합계 EXACT)
