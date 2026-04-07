# brainwire/ Python Package Guide

## 설치

```bash
pip install -e ".[dev]"
```

의존성: Python 3.11+, PyYAML 6.0+, pytest 7.0+ (개발)

---

## 패키지 구조

```
brainwire/
├── __init__.py              # v2.0.0
├── variables.py             # 12변수 상수, 카테고리, 텐션 가중치
│
├── profiles/                # 의식 상태 프로파일 (YAML)
│   ├── base.py              # StateProfile, Envelope, SafetyOverrides, PidHint
│   ├── __init__.py          # load_profile(), list_profiles()
│   ├── thc.yaml             # State profile A
│   ├── lsd.yaml             # State profile L
│   ├── psilocybin.yaml      # State profile P
│   ├── dmt.yaml             # State profile D
│   ├── mdma.yaml            # State profile M
│   └── flow.yaml            # Flow State (endogenous)
│
├── hardware/                # 하드웨어 추상화
│   ├── devices.py           # Device 데이터클래스, DeviceRegistry, 11개 CORE_DEVICES
│   ├── hal.py               # HAL — 핫플러그, Tier 감지, 슬롯 관리
│   ├── safety.py            # SafetyEngine — 4-레이어 안전 아키텍처
│   └── configs.py           # TIER_CONFIGS (Tier 1-4 프리셋), get_tier_params()
│
├── engine/                  # 계산 엔진
│   ├── transfer.py          # TransferEngine — 12var x N device 전달 함수
│   ├── tension.py           # PureField 텐션 (compute_tension, compute_match)
│   ├── pid.py               # PIDController + PIDBank (12 독립 PID)
│   └── interpolation.py     # lerp_states, blend_states, envelope_value
│
├── optimizer.py             # 프로파일별 좌표 하강 최적화
├── simulator.py             # 시간축 세션 시뮬레이터 (PID + Anima 호흡 리듬)
├── eeg_feedback.py          # G=D×P/I EEG 피드백 — 골든존 타겟팅
├── bench.py                 # 멀티-스테이트 벤치마크 CLI
└── calc.py                  # 확장 계산기 (민감도, 갭 분석, 블렌딩)
```

---

## CLI 명령어

### 벤치마크 (`brainwire.bench`)

```bash
python -m brainwire.bench bench thc --tier 4       # 단일 상태
python -m brainwire.bench compare thc lsd dmt      # 비교
python -m brainwire.bench tiers thc                 # Tier 1-4 비교
python -m brainwire.bench all                       # 전체 매트릭스
```

### 시뮬레이터 (`brainwire.simulator`)

```bash
python -m brainwire.simulator flow --tier 4 --duration 600
python -m brainwire.simulator --all --tier 4
python -m brainwire.simulator flow --concentration  # 농도별
python -m brainwire.simulator flow --no-pid         # 오픈루프
python -m brainwire.simulator flow --no-breathing   # 호흡 리듬 없이
```

### 최적화 (`brainwire.optimizer`)

```bash
python -m brainwire.optimizer                        # 전 프로파일 최적화
python -m brainwire.optimizer --state thc --tier 4   # 특정 상태
python -m brainwire.optimizer --iters 200            # 반복 횟수
```

### EEG 피드백 (`brainwire.eeg_feedback`)

```bash
python -m brainwire.eeg_feedback                    # G=D×P/I 전 상태 분석
```

### 계산기 (`brainwire.calc`)

```bash
python -m brainwire.calc sensitivity thc --tier 3   # 파라미터 민감도
python -m brainwire.calc gap dmt --tier 3           # 미달 변수 분석
python -m brainwire.calc blend --states thc flow --weights 0.7 0.3
```

### 가설 벤치마크

```bash
python bench_hypotheses.py                          # 50 가설 검증
```

### 레거시 (v1)

```bash
python bench_thc_vars.py                            # Joywire 벤치마크
python bench_thc_vars.py --levels                   # 농도별
python calc.py sensitivity                          # 민감도 분석
python calc.py gap                                  # 미달 분석
python calc.py optimize --budget 500                # 예산 최적화
```

---

## 테스트

```bash
python -m pytest tests/ -v                          # 112 tests
python -m pytest tests/test_integration.py -v       # 11 E2E tests
python bench_hypotheses.py                          # 50 hypotheses
```

| 테스트 파일 | 테스트 수 | 커버리지 |
|---|---|---|
| test_variables.py | 7 | 12변수 상수, 카테고리, 가중치 |
| test_profiles.py | 11 | 6 프로파일 로딩, 검증, 스케일링 |
| test_hardware.py | 7 | 디바이스, HAL, Tier 감지 |
| test_safety.py | 10 | 4-레이어 안전, 세션 관리 |
| test_transfer.py | 10 | 전달 함수, Tier 4 계수 |
| test_tension.py | 5 | 텐션 계산, 크로스-스테이트 |
| test_pid.py | 8 | PID, 안티와인드업, 힌트 |
| test_interpolation.py | 12 | lerp, 블렌드, 엔벨로프 |
| test_configs.py | 5 | Tier 1-4 프리셋 |
| test_bench.py | 5 | 벤치마크 CLI |
| test_calc.py | 3 | 계산기 함수 |
| test_optimizer.py | 5 | 최적화 수렴, 개선 |
| test_simulator.py | 6 | 세션 시뮬레이션 |
| test_eeg_feedback.py | 7 | G=D×P/I 계산 |
| test_integration.py | 11 | 전체 파이프라인 E2E |
