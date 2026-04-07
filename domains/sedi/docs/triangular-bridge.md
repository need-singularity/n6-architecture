# Triangular Bridge: SEDI ↔ TECS-L ↔ n6

> 세 프로젝트 간 데이터/상수/가설의 삼각 동기화 체계

## Data Flow

```
        SEDI (물리 검증)
       ╱              ╲
      ↙                ↘
  가설 등급              검증된 상수
  sedi-grades.json      verified constants
      ↓                    ↓
   TECS-L (수학 증명) ←→ n6 (엔지니어링)
     math_atlas.json     DSE domains
     hypotheses/         atlas-constants.md
```

### 순환 구조

```
SEDI → TECS-L:  물리 검증 결과 → 수학 atlas 등록
TECS-L → n6:    수학 증명 + 계산기 → 엔지니어링 도구
n6 → SEDI:      발견 엔진 후보식 → SEDI 물리 검증
TECS-L → SEDI:  새 가설 + 상수 → SEDI 데이터 소스로 검증
```

## 각 프로젝트 기여

### SEDI 기여 (물리 검증)

| 기여 항목 | 대상 | 파일/경로 |
|---|---|---|
| 가설 n=6 등급 (A/B/C/D/E) | TECS-L | data/sedi-grades.json |
| 검증된 상수 (오차% 포함) | TECS-L atlas | sedi/constants.py, docs/hypotheses/ |
| 678개 가설 검증 결과 | TECS-L, n6 | docs/hypotheses/H-*.md |
| R-spectrum 이상 탐지 | n6 | sedi/receiver.py |
| SETI 신호 후보 | TECS-L, n6 | sedi/sources/ (77개 소스) |

### TECS-L 기여 (수학 증명)

| 기여 항목 | 대상 | 파일/경로 |
|---|---|---|
| 수학 증명 (R(6)=1 등) | SEDI, n6 | docs/hypotheses/ |
| 계산기 (calc/, tecsrs/) | SEDI, n6 | .shared/calculators.json |
| Math Atlas (가설 DB) | SEDI, n6 | .shared/math_atlas.json |
| DFS 탐색 결과 | n6 | .shared/dse/domains/*.toml |
| 상수 발견 | SEDI | calc/ Python + Rust 계산기 |

### n6 기여 (엔지니어링 패턴)

| 기여 항목 | 대상 | 파일/경로 |
|---|---|---|
| DSE 도메인 정의 | TECS-L | tools/universal-dse/domains/*.toml |
| Atlas 상수 문서 | TECS-L | docs/atlas-constants.md |
| Rust 계산기 | TECS-L, SEDI | tools/*.rs |
| 아키텍처 패턴 | SEDI | n6 설계 → SEDI receiver 구조 |

## Sync Scripts

### 기존 동기화 (TECS-L 주도)

| 스크립트 | 방향 | 내용 |
|---|---|---|
| `.shared/sync-calculators.sh` | TECS-L → all | 계산기 레지스트리 + README 동기화 |
| `.shared/scan-calculators.py` | scan all repos | 계산기 스캔 → calculators.json |
| `.shared/scan_math_atlas.py` | scan all repos | 가설 스캔 → math_atlas.json |

### 신규 동기화 (SEDI 주도)

| 스크립트 | 방향 | 내용 |
|---|---|---|
| `scripts/auto_grade_n6.py` | SEDI internal | 678 가설 → n=6 Bayesian 등급 산정 |
| `scripts/sync_to_atlas.py` | SEDI → TECS-L | 검증 상수 + 등급 → atlas 등록 |

### 역동기화 (sync-calculators.sh 내)

| 단계 | 방향 | 내용 |
|---|---|---|
| Phase 3/4 | n6 → TECS-L | DSE TOML + atlas-constants 역동기화 |
| Phase 4/5 (신규) | SEDI → TECS-L | sedi-grades.json + 검증 상수 복사 |

## 공통 상수 (3개 프로젝트 공유)

```python
# n=6 기본 산술 — 모든 프로젝트의 공통 기반
N = 6                    # 첫 번째 완전수
SIGMA = 12               # σ(6) = 1+2+3+6
TAU = 4                  # τ(6) = 4개 약수
PHI = 2                  # φ(6) = Euler totient
SOPFR = 5                # 소인수 합 2+3
R = SIGMA*PHI/(N*TAU)    # R(6) = 1 (유일한 R-balance)

# 파생 상수
SIGMA_MINUS_TAU = 8      # σ-τ = 8 (Bott 주기성)
SIGMA_PHI = 24           # 24 = 4! (시공간 차원)
EINSTEIN_THETA = √(3/2)  # 중력렌즈 초점
GOLDEN_CENTER = 1/e       # Golden Zone 중심

# 완전수 사다리
P1, P2, P3 = 6, 28, 496  # 완전수열
```

### 프로젝트별 상수 사용

| 상수 | SEDI | TECS-L | n6 |
|---|---|---|---|
| N=6 | 수신기 주파수 | 증명 기반 | 아키텍처 차원 |
| σ=12 | FFT 윈도우 | 약수합 정리 | 레이어 수 |
| τ=4 | R-spectrum gap | 약수 개수 | 게이트 수 |
| φ=2 | 엔트로피 기준 | Euler totient | 이진 분할 |
| R=1 | 검증 목표 | 유일성 증명 | 균형 조건 |
| 1/2+1/3+1/6 | 자원 할당 | Egyptian fraction | MoE 비율 |

## 실행 방법

```bash
# 1. SEDI: 가설 등급 산정
cd ~/Dev/sedi
python3 scripts/auto_grade_n6.py --save

# 2. SEDI → TECS-L: 상수/등급 동기화
python3 scripts/sync_to_atlas.py --save --grades

# 3. TECS-L: 전체 동기화 (계산기 + atlas)
cd ~/Dev/TECS-L
bash .shared/sync-calculators.sh
```
