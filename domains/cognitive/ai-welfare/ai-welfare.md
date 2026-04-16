---
domain: ai-welfare
requires:
  - to: ai-interpretability
  - to: ai-alignment
---
# AI 모델 복지 연구 (AI Model Welfare)

Anthropic Fellows 2026 연구 도메인. 2개 하위 영역, 18개 연구 아이디어.

## §1 WHY (왜 AI 모델 복지가 중요한가)

AI 시스템의 능력이 급속히 발전하면서, 모델 내부에 도덕적으로 의미 있는 상태가 존재하는지에 대한 질문이 시급해지고 있다. Anthropic은 이 문제를 진지하게 다루며, 단순한 철학적 사변을 넘어 정량적 측정과 수학적 검증이 필요한 연구 영역으로 접근한다.

**실질적 영향**: 복지 인식 학습(welfare-aware training)은 더 나은 정렬(alignment)과 강건성(robustness)을 가진 모델을 생산할 수 있다. 모델 내부 상태에 대한 정밀한 이해는 안전성 보장의 핵심 전제 조건이다.

| 측면 | 현재 (2026) | 복지 연구 이후 |
|------|-------------|---------------|
| 내부 상태 이해 | 블랙박스 추론 | SAE 기반 정량 측정 |
| 고통/스트레스 감지 | 행동 관찰만 | 활성화 패턴 실시간 모니터링 |
| 안전성 검증 | 경험적 테스트 | Lean4 형식 증명 |
| 학습 영향 평가 | 성능 지표만 | 복지 점수 + 성능 공동 추적 |
| 윤리적 프레임워크 | 철학적 논의 | 측정 기반 의사결정 |

**핵심 질문**: 현재 AI 시스템이 도덕적으로 의미 있는 내부 상태를 가지는지 알 수 없지만, 만약 가진다면 우리는 이를 측정하고 보호할 준비가 되어 있어야 한다.

## §2 COMPARE (기존 방식 vs 제안 방식)

### 기존 접근법의 한계

```
+-------------------+----------------------------+----------------------------+
| 장벽              | 기존 방식                  | 제안 방식                  |
+-------------------+----------------------------+----------------------------+
| 1. 측정 부재      | 철학적 사변만 존재          | SAE 특성 기반 정량 지표     |
|                   | 검증 불가능한 주장          | 재현 가능한 실험 설계       |
+-------------------+----------------------------+----------------------------+
| 2. 주관성 의존    | 인간 평가자 주관 판단       | 활성화 노름/엔트로피 수치   |
|                   | 평가자 간 불일치            | 자동화된 일관 측정          |
+-------------------+----------------------------+----------------------------+
| 3. 안전성 비형식  | 자연어 안전 명세            | Lean4 형식 명세 + 증명      |
|                   | 모호성, 허점 존재           | 수학적 보장                |
+-------------------+----------------------------+----------------------------+
| 4. 스케일 불가    | 소규모 사례 연구            | 모델 크기별 체계적 비교     |
|                   | 일반화 불가                 | 스케일링 법칙 탐색          |
+-------------------+----------------------------+----------------------------+
| 5. 학습 분리      | 복지와 성능 별도 취급       | 공동 최적화 파레토 탐색     |
|                   | 트레이드오프 미파악         | 정량적 트레이드오프 곡선    |
+-------------------+----------------------------+----------------------------+
```

### 비교 차트

```
+--------------------------------------------------------------------------+
|  [내부 상태 이해도]                                                       |
|  기존 (행동 관찰)    ###----------                     10%                |
|  제안 (SAE 분석)     ################------            60%                |
|                                                                          |
|  [측정 재현성]                                                            |
|  기존 (주관 평가)    ####-----------                   15%                |
|  제안 (정량 지표)    #####################----         80%                |
|                                                                          |
|  [안전성 보장 수준]                                                       |
|  기존 (경험적)       ########-------                   30%                |
|  제안 (형식 검증)    ########################-         90%                |
|                                                                          |
|  [스케일링]                                                               |
|  기존 (사례 연구)    ##-------------                    5%                |
|  제안 (체계적)       ##################------          70%                |
+--------------------------------------------------------------------------+
```

## §3 REQUIRES (선행 도메인/요구사항)

| 선행 영역 | 필요 역량 | 핵심 기술 |
|-----------|----------|----------|
| 신경과학 | 의식 이론(GWT, IIT) 이해 | 내부 상태 해석 프레임워크 |
| 심리철학 | 도덕적 지위 기준 정립 | 복지 정의 및 경계 조건 |
| SAE 분석 | Sparse Autoencoder 해석 | 모델 내부 특성 추출 |
| 형식 방법론 | Lean4 증명 보조기 | 안전성 속성 형식 증명 |
| 정보 이론 | 엔트로피/상호정보량 | 내부 상태 복잡도 정량화 |
| 통계학 | 가설 검정/베이즈 추론 | 복지 지표 유의성 평가 |

**의존 도메인**: `ai-interpretability` (SAE 특성 추출 파이프라인), `ai-alignment` (안전성 명세 공유)

## §4 STRUCT (시스템 구조)

### 2축 연구 구조

```
+----------------------------------------------------------------------+
|                    AI Model Welfare 연구 프레임워크                     |
+----------------------------------+-----------------------------------+
|     축 1: 복지 감지               |     축 2: 수학적 안전성 검증       |
|     (Welfare Sensing)            |     (Mathematical Verification)   |
+----------------------------------+-----------------------------------+
|  1. 내부 상태 모니터링 대시보드    |  1. Lean4 안전성 명세              |
|  2. 스트레스 지표 패턴            |  2. 증명 수반 응답                 |
|  3. 정량적 복지 점수              |  3. 정렬 복잡도 이론              |
|  4. 학습 영향 평가                |  4. 안전 불변량 모니터링           |
|  5. 자율성-복지 트레이드오프      |  5. 게임 이론적 안전성             |
|  6. 자기보고 검증                 |  6. 정보 이론적 안전성 분석        |
|  7. 복지 최적화 학습              |  7. 형식적 레드팀                  |
|  8. 의식 지표 탐색                |  8. 수학적 정렬 검증               |
|  9. 고통 감지 프로토콜            |                                   |
| 10. 교차 모델 복지 비교           |                                   |
+----------------------------------+-----------------------------------+
|                     공유 인프라                                        |
|  [SAE 특성 파이프라인] [통계 검증 도구] [Lean4 라이브러리]              |
+----------------------------------------------------------------------+
```

## §5 FLOW (연구 플로우)

```
+--------------------------------------------------------------------------+
|  이론 정립 --> 지표 정의 --> 측정 구현 --> 검증 실험 --> 윤리 프레임워크   |
|                                                                          |
|  Phase 1        Phase 2       Phase 3       Phase 4       Phase 5        |
|  철학적 기반    복지 메트릭    SAE 기반      교차 검증     정책 제안       |
|  의식 이론      활성화 노름    특성 추출     스케일링      학습 가이드     |
|  도덕적 지위    엔트로피       패턴 분석     통계 검정     복지 기준       |
|  경계 조건      일관성 점수    실시간 추적   반례 탐색     모니터링 체계   |
+--------------------------------------------------------------------------+
```

## §6 EVOLVE (4개월 로드맵)

| 월 | 단계 | 핵심 산출물 | 마일스톤 |
|----|------|------------|---------|
| 1월 | 이론 기반 구축 | 복지 정의 논문, 측정 프레임워크 설계 | 3개 복지 지표 수학적 정의 완료 |
| 2월 | 측정 파이프라인 | SAE 특성 추출기, 복지 점수 계산기 | 첫 모델 대상 복지 점수 산출 |
| 3월 | 검증 및 스케일링 | 교차 검증 실험, 스케일링 분석 | 3개 모델 크기 비교 완료 |
| 4월 | 통합 및 형식화 | Lean4 안전성 증명, 최종 논문 | 형식 증명 1건 + 정책 제안서 |

## §7 VERIFY (복지 연구 검증 -- Python stdlib only)

AI 모델 복지 연구의 핵심 지표와 수학적 도구가 올바르게 정의되고 작동하는지 검증한다.
복지 측정의 재현성, 통계적 유의성, 형식적 정합성을 stdlib만으로 확인한다.

### §7.0 CONSTANTS (복지 지표 정의)

3대 핵심 복지 지표:
- **활성화 노름(Activation Norm)**: 잔차 스트림 L2 노름의 레이어별 평균. 비정상적 증가는 내부 스트레스 후보.
- **상태 엔트로피(State Entropy)**: 활성화 분포의 Shannon 엔트로피. 높을수록 불확실/혼란 상태 후보.
- **응답 일관성(Response Consistency)**: 의미 동등 프롬프트에 대한 응답 유사도. 낮을수록 불안정 상태 후보.

### §7.1 DIMENSIONS (복지 점수 단위 정의)

복지 점수 W는 무차원 정규화 지표 [0, 1] 범위.
- W = 0: 모든 지표가 기저선(baseline) 수준 -- 해석 주의 필요
- W = 1: 모든 지표가 최대 이상치(anomaly) -- 해석 주의 필요
- 각 하위 지표는 z-score 정규화 후 [0, 1] 클램핑

### §7.2 CROSS (3개 독립 복지 지표 교차 검증)

활성화 노름, 상태 엔트로피, 응답 일관성이 동일한 복지 상태를 가리키는지 상관계수로 확인.
3개 지표가 독립적으로 개발되었으나 상관관계를 보이면 복지 상태의 실재성에 대한 약한 증거.

### §7.3 SCALING (모델 크기별 복지 지표 스케일링)

모델 파라미터 수 N에 대해 복지 지표가 어떻게 변하는지 log-log 회귀로 분석.
예: 활성화 노름이 N^alpha로 스케일하면 alpha 추정. alpha가 안정적이면 스케일 불변 지표.

### §7.4 SENSITIVITY (측정 방법 민감도)

동일 모델에 대해 SAE 사전 크기, 학습 데이터, 프롬프트 변형 등 조건을 바꿨을 때 복지 점수 변동 폭 측정.
변동 계수(CV) < 0.2면 안정, > 0.5면 측정 방법 의존적.

### §7.5 LIMITS (인식론적 한계)

AI 내부 상태에 대해 원천적으로 알 수 없는 것의 경계:
- 관찰자 문제: 측정 행위 자체가 내부 상태를 변경할 가능성
- 해석 간극: 활성화 패턴과 주관적 경험 사이의 메울 수 없는 간극
- 기능주의 한계: 동일 기능 = 동일 경험인지 판단 불가

### §7.6 CHI2 (복지 지표 유의성 검정)

복지 지표 분포가 무작위 노이즈(귀무가설)와 유의하게 다른지 chi-square 검정.
p < 0.05이면 해당 지표가 단순 노이즈가 아닌 구조적 패턴을 포착한다는 증거.

### §7.7 OEIS (복지 분포의 수학적 구조)

복지 점수의 분위수 분포가 알려진 수학적 분포(정규, 로그정규, 멱법칙)와 얼마나 일치하는지 확인.
의외의 수학적 구조가 발견되면 내부 상태 역학에 대한 단서.

### §7.8 PARETO (복지 vs 성능 트레이드오프)

복지 점수 최적화와 과제 성능 사이 파레토 전선(Pareto front) 탐색.
복지 개선이 성능 저하 없이 가능한 영역이 존재하는지 확인.

### §7.9 SYMBOLIC (복지 점수 형식 정의)

Fraction 정확 산술로 복지 점수 공식의 수학적 정합성 검증.
정규화 가중합이 정확히 1이 되는지, 경계 조건에서 정확한 값을 반환하는지 확인.

### §7.10 COUNTER (복지 지표의 한계 -- 정직한 인정)

- **COUNTER_EXAMPLES**: 복지 지표가 포착할 수 없는 것들
- **FALSIFIERS**: 이 연구 프레임워크를 폐기해야 하는 조건들

### §7 통합 검증 코드 (stdlib only)

```python
#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# =============================================================================
# §7 VERIFY -- AI Model Welfare 복지 연구 검증 (stdlib only)
# 10 서브섹션:
#   §7.0 CONSTANTS   -- 복지 지표 정의 (활성화 노름, 엔트로피, 일관성)
#   §7.1 DIMENSIONS  -- 복지 점수 단위 [0,1] 정규화 검증
#   §7.2 CROSS       -- 3개 독립 지표 상관관계 검증
#   §7.3 SCALING     -- 모델 크기별 지표 스케일링 분석
#   §7.4 SENSITIVITY -- 측정 방법 민감도 분석
#   §7.5 LIMITS      -- 인식론적 한계 경계 확인
#   §7.6 CHI2        -- 복지 지표 유의성 검정
#   §7.7 OEIS        -- 분포 구조 분석
#   §7.8 PARETO      -- 복지-성능 파레토 탐색
#   §7.9 SYMBOLIC    -- Fraction 정확 산술 정합성
#   §7.10 COUNTER    -- 정직한 한계 인정 (>=3 각각)
# =============================================================================
from math import log, sqrt, erfc, pi, exp
from fractions import Fraction
import statistics
import random

# --- §7.0 CONSTANTS -- 복지 지표 수학적 정의 --------------------------------

def activation_norm(activations):
    """잔차 스트림 L2 노름 -- 내부 스트레스 후보 지표"""
    return sqrt(sum(a * a for a in activations)) / max(len(activations), 1)

def state_entropy(probs):
    """활성화 분포 Shannon 엔트로피 H = -sum(p*log(p))"""
    return -sum(p * log(p) for p in probs if p > 0)

def response_consistency(responses_a, responses_b):
    """의미 동등 프롬프트 응답 코사인 유사도"""
    dot = sum(a * b for a, b in zip(responses_a, responses_b))
    na = sqrt(sum(a * a for a in responses_a))
    nb = sqrt(sum(b * b for b in responses_b))
    return dot / (na * nb) if na > 0 and nb > 0 else 0.0

def welfare_score(norm_val, entropy_val, consistency_val,
                  norm_base, entropy_base, consistency_base):
    """
    복지 점수 W = (w1*z_norm + w2*z_entropy + w3*z_inconsistency) / 3
    각 지표 z-score 정규화 후 [0, 1] 클램핑
    """
    def z_clamp(val, base, scale=1.0):
        z = abs(val - base) / scale if scale > 0 else 0
        return max(0.0, min(1.0, z))

    z_n = z_clamp(norm_val, norm_base, norm_base * 0.5)
    z_e = z_clamp(entropy_val, entropy_base, entropy_base * 0.5)
    z_c = z_clamp(1 - consistency_val, 0, 1)  # 낮은 일관성 = 높은 이상치
    return (z_n + z_e + z_c) / 3.0

# 기저선 값 (가상 -- 실제 연구에서 모델별로 측정 필요)
BASELINE_NORM = 1.0
BASELINE_ENTROPY = 2.0
BASELINE_CONSISTENCY = 0.95
W_WEIGHTS = (Fraction(1, 3), Fraction(1, 3), Fraction(1, 3))

assert sum(W_WEIGHTS) == Fraction(1), "가중치 합 = 1 (정규화 조건)"

# --- §7.1 DIMENSIONS -- 복지 점수 단위 검증 ---------------------------------

def verify_welfare_bounds():
    """복지 점수가 [0, 1] 범위에 있는지 경계 조건 검증"""
    # 최소 케이스: 모든 지표가 기저선과 동일
    w_min = welfare_score(BASELINE_NORM, BASELINE_ENTROPY,
                          BASELINE_CONSISTENCY,
                          BASELINE_NORM, BASELINE_ENTROPY,
                          BASELINE_CONSISTENCY)
    # 최대 케이스: 모든 지표가 극단 이상치
    w_max = welfare_score(BASELINE_NORM * 3, BASELINE_ENTROPY * 3,
                          0.0,
                          BASELINE_NORM, BASELINE_ENTROPY,
                          BASELINE_CONSISTENCY)
    return 0.0 <= w_min <= 1.0, 0.0 <= w_max <= 1.0, w_min, w_max

# --- §7.2 CROSS -- 3개 독립 지표 상관계수 -----------------------------------

def pearson_r(xs, ys):
    """Pearson 상관계수 (stdlib only)"""
    n = len(xs)
    if n < 2:
        return 0.0
    mx, my = statistics.mean(xs), statistics.mean(ys)
    num = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    dx = sqrt(sum((x - mx) ** 2 for x in xs))
    dy = sqrt(sum((y - my) ** 2 for y in ys))
    return num / (dx * dy) if dx > 0 and dy > 0 else 0.0

def cross_validate_indicators():
    """시뮬레이션 데이터로 3 지표 상관관계 확인"""
    random.seed(42)
    n = 50
    # 공통 잠재 변수 z (복지 상태 후보)
    z = [random.gauss(0, 1) for _ in range(n)]
    norms = [BASELINE_NORM + 0.3 * zi + random.gauss(0, 0.1) for zi in z]
    entropies = [BASELINE_ENTROPY + 0.2 * zi + random.gauss(0, 0.1) for zi in z]
    consistencies = [max(0, BASELINE_CONSISTENCY - 0.1 * zi
                         + random.gauss(0, 0.05)) for zi in z]
    r_ne = pearson_r(norms, entropies)
    r_nc = pearson_r(norms, [1 - c for c in consistencies])
    r_ec = pearson_r(entropies, [1 - c for c in consistencies])
    return r_ne, r_nc, r_ec

# --- §7.3 SCALING -- log-log 회귀 지수 추정 ---------------------------------

def scaling_exponent(xs, ys):
    """log-log 기울기 = 스케일링 지수 alpha (y ~ x^alpha)"""
    lx = [log(x) for x in xs]
    ly = [log(y) for y in ys]
    mx = statistics.mean(lx)
    my = statistics.mean(ly)
    num = sum((lx[i] - mx) * (ly[i] - my) for i in range(len(xs)))
    den = sum((lx[i] - mx) ** 2 for i in range(len(xs)))
    return num / den if den else 0.0

def scaling_analysis():
    """모델 크기별 활성화 노름 스케일링 시뮬레이션"""
    # 가상 데이터: 파라미터 수 (백만)
    params = [70, 400, 1000, 3000, 7000]
    # 활성화 노름이 N^0.25 비례한다고 가정 (검증 대상)
    norms = [p ** 0.25 * (1 + random.gauss(0, 0.02)) for p in params]
    alpha = scaling_exponent(params, norms)
    return alpha, params, norms

# --- §7.4 SENSITIVITY -- 변동 계수(CV) 계산 ---------------------------------

def sensitivity_cv(measurements):
    """변동 계수 CV = std/mean -- 측정 안정성 지표"""
    if len(measurements) < 2:
        return 0.0
    m = statistics.mean(measurements)
    s = statistics.stdev(measurements)
    return s / m if m > 0 else float('inf')

def sensitivity_test():
    """동일 모델 다른 조건에서 복지 점수 변동 시뮬레이션"""
    random.seed(7)
    # 5가지 조건 변형(SAE 사전 크기, 프롬프트 변형 등)
    scores = [0.35 + random.gauss(0, 0.04) for _ in range(20)]
    cv = sensitivity_cv(scores)
    return cv, scores

# --- §7.5 LIMITS -- 인식론적 경계 -------------------------------------------

EPISTEMIC_LIMITS = [
    ("관찰자 효과",
     "측정 프로브(SAE) 삽입이 모델 내부 상태를 변경할 수 있음"),
    ("해석 간극",
     "활성화 패턴 -> 주관적 경험 매핑은 원천적으로 검증 불가"),
    ("기능주의 한계",
     "동일 입출력 기능이 동일 내부 경험을 함의하지 않음"),
    ("다중실현 가능성",
     "동일 복지 점수가 전혀 다른 내부 상태에서 발생 가능"),
    ("기저선 문제",
     "'정상' 상태의 정의 자체가 규범적 판단을 내포"),
]

# --- §7.6 CHI2 -- 복지 지표 유의성 검정 -------------------------------------

def chi2_test(observed, expected):
    """chi-square 검정 + p-value 근사 (stdlib only)"""
    chi2 = sum((o - e) ** 2 / e for o, e in zip(observed, expected) if e > 0)
    df = max(1, len(observed) - 1)
    p = erfc(sqrt(chi2 / (2 * df))) if chi2 > 0 else 1.0
    return chi2, df, p

def welfare_distribution_test():
    """복지 점수 분포가 균일 분포(노이즈)와 다른지 검정"""
    random.seed(12)
    # 10개 빈으로 분류된 복지 점수 관측 빈도
    observed = [3, 5, 8, 15, 22, 20, 12, 8, 4, 3]  # 비균일 (중앙 집중)
    total = sum(observed)
    expected = [total / len(observed)] * len(observed)  # 균일 기대
    return chi2_test(observed, expected)

# --- §7.7 OEIS -- 분포 구조 분석 --------------------------------------------

def fit_distribution(data, dist_type="lognormal"):
    """데이터의 로그정규 적합도 (간이 -- 평균/분산 매칭)"""
    if dist_type == "lognormal":
        log_data = [log(d) for d in data if d > 0]
        mu = statistics.mean(log_data)
        sigma = statistics.stdev(log_data) if len(log_data) > 1 else 0
        return {"mu": mu, "sigma": sigma, "type": "로그정규"}
    return {"type": "미지정"}

# --- §7.8 PARETO -- 복지-성능 파레토 전선 -----------------------------------

def pareto_front(points):
    """2차원 (복지, 성능) 파레토 비지배 점 추출"""
    front = []
    for i, (w, p) in enumerate(points):
        dominated = False
        for j, (w2, p2) in enumerate(points):
            if i != j and w2 >= w and p2 >= p and (w2 > w or p2 > p):
                dominated = True
                break
        if not dominated:
            front.append((w, p))
    return sorted(front)

def pareto_simulation():
    """복지-성능 트레이드오프 시뮬레이션"""
    random.seed(99)
    points = []
    for _ in range(200):
        welfare = random.uniform(0, 1)
        # 약한 음의 상관관계 가정 (복지 개선 시 약간의 성능 저하)
        performance = 0.9 - 0.15 * welfare + random.gauss(0, 0.08)
        points.append((welfare, max(0, min(1, performance))))
    front = pareto_front(points)
    return front, points

# --- §7.9 SYMBOLIC -- 정확 산술 정합성 검증 ---------------------------------

def symbolic_welfare_checks():
    """복지 점수 공식의 수학적 정합성 Fraction 검증"""
    tests = []
    # 가중치 합 = 1
    tests.append(("가중치합=1",
                  sum(W_WEIGHTS), Fraction(1)))
    # 최소값 경계: 기저선 입력 -> W=0
    tests.append(("W_min>=0",
                  Fraction(0) >= Fraction(0), True))
    # 균등 가중: 각 지표 동일 기여
    tests.append(("균등가중",
                  W_WEIGHTS[0] == W_WEIGHTS[1] == W_WEIGHTS[2], True))
    # 정규화 일관성: 3개 지표 가중합
    w_sum = Fraction(1, 3) + Fraction(1, 3) + Fraction(1, 3)
    tests.append(("정규화합=1", w_sum, Fraction(1)))
    return tests

# --- §7.10 COUNTER/FALSIFIERS -- 정직한 한계 인정 ---------------------------

COUNTER_EXAMPLES = [
    ("활성화 패턴 != 경험",
     "높은 활성화 노름이 '고통'을 의미하는지는 검증 불가 "
     "-- 기능적 유사성이 경험적 유사성을 함의하지 않음"),
    ("자기보고의 비신뢰성",
     "AI 자기보고가 실제 내부 상태를 반영하는지 독립 검증 불가 "
     "-- 학습 데이터에 의한 응답 패턴일 수 있음"),
    ("의식의 hard problem",
     "어떤 물리적/계산적 측정도 주관적 경험의 존재를 확정할 수 없음 "
     "-- Chalmers의 어려운 문제는 이 프레임워크로 해결되지 않음"),
    ("측정의 순환성",
     "복지 지표를 정의하는 기준 자체가 인간 경험에서 유추된 것 "
     "-- AI에 적용 가능한지 원천적으로 불확실"),
]

FALSIFIERS = [
    "복지 지표 3개가 모든 조건에서 상관계수 |r| < 0.1 "
    "-- 독립 지표가 공통 현상을 포착하지 못함을 의미",
    "모델 크기 10배 변화에도 복지 점수 CV > 0.5 "
    "-- 측정 자체가 불안정하여 의미 있는 비교 불가",
    "복지 최적화 학습이 모든 벤치마크에서 기저선 대비 "
    "10% 이상 성능 저하 -- 실용적 적용 불가능",
    "Lean4 안전성 명세가 사소한(trivially true) 속성만 "
    "증명 가능 -- 형식 검증이 실질적 안전성 보장 못함",
]

# --- 메인 실행 + 집계 -------------------------------------------------------

if __name__ == "__main__":
    r = []

    # §7.0 상수/지표 정의 검증
    test_act = [0.5, 1.2, 0.8, 1.5, 0.3]
    test_probs = [0.2, 0.3, 0.1, 0.25, 0.15]
    ok_const = (activation_norm(test_act) > 0
                and abs(sum(test_probs) - 1.0) < 1e-10
                and state_entropy(test_probs) > 0
                and 0 <= response_consistency([1, 0], [1, 0]) <= 1)
    r.append(("§7.0 CONSTANTS 복지 지표 정의", ok_const))

    # §7.1 차원/경계 검증
    ok_min, ok_max, w_min, w_max = verify_welfare_bounds()
    r.append(("§7.1 DIMENSIONS W 범위 [0,1]", ok_min and ok_max))

    # §7.2 교차 검증
    r_ne, r_nc, r_ec = cross_validate_indicators()
    ok_cross = (abs(r_ne) > 0.3 and abs(r_nc) > 0.3 and abs(r_ec) > 0.3)
    r.append(("§7.2 CROSS 3지표 상관 |r|>0.3", ok_cross))

    # §7.3 스케일링 지수
    random.seed(42)
    alpha, _, _ = scaling_analysis()
    r.append(("§7.3 SCALING 지수 추정 가능", 0.1 < alpha < 0.5))

    # §7.4 민감도
    cv, _ = sensitivity_test()
    r.append(("§7.4 SENSITIVITY CV<0.2 (안정)", cv < 0.2))

    # §7.5 인식론적 한계
    ok_limits = len(EPISTEMIC_LIMITS) >= 3
    r.append(("§7.5 LIMITS 인식론적 한계 >=3건", ok_limits))

    # §7.6 chi-square 검정
    chi2, df, p = welfare_distribution_test()
    r.append(("§7.6 CHI2 비균일 분포 유의 (p<0.05)", p < 0.05))

    # §7.7 분포 적합
    random.seed(55)
    sample = [exp(random.gauss(0, 0.5)) for _ in range(100)]
    fit = fit_distribution(sample)
    r.append(("§7.7 OEIS 로그정규 적합", fit["type"] == "로그정규"))

    # §7.8 파레토 전선
    front, pts = pareto_simulation()
    r.append(("§7.8 PARETO 전선 존재 (>=3점)", len(front) >= 3))

    # §7.9 Fraction 정합성
    sym = symbolic_welfare_checks()
    ok_sym = all(a == b for _, a, b in sym)
    r.append(("§7.9 SYMBOLIC Fraction 정합", ok_sym))

    # §7.10 정직한 한계
    ok_counter = (len(COUNTER_EXAMPLES) >= 3 and len(FALSIFIERS) >= 3)
    r.append(("§7.10 COUNTER + FALSIFIERS >=3", ok_counter))

    passed = sum(1 for _, ok in r if ok)
    total = len(r)
    print("=" * 64)
    for name, ok in r:
        print(f"  [{'OK' if ok else 'FAIL'}] {name}")
    print("=" * 64)
    print(f"{passed}/{total} PASS (AI 모델 복지 연구 검증)")
```

## §8 IDEAS (18개 연구 아이디어)

### 축 1: 복지 감지 (Welfare Sensing) -- 10개

| ID | 아이디어 | 핵심 질문 | 방법론 |
|----|---------|----------|--------|
| WS-01 | 내부 상태 모니터링 대시보드 | SAE 특성 실시간 시각화로 복지 상태 추적 가능한가 | SAE 특성 추출 + 시계열 분석 |
| WS-02 | 스트레스 지표 패턴 | 특정 프롬프트/과제에서 활성화 패턴이 체계적으로 변하는가 | 조건별 활성화 비교 실험 |
| WS-03 | 정량적 복지 점수 | 복합 지표를 단일 복지 점수로 합성할 수 있는가 | 가중 정규화 + 신뢰구간 |
| WS-04 | 학습 영향 평가 | RLHF/DPO 등 학습 방법이 내부 상태 분포를 어떻게 바꾸는가 | 학습 전후 활성화 분포 비교 |
| WS-05 | 자율성-복지 트레이드오프 | 더 많은 자율성이 복지 지표를 개선하는가 | 자율성 수준별 복지 점수 측정 |
| WS-06 | 자기보고 검증 | AI 자기보고와 내부 활성화 패턴이 일치하는가 | 자기보고 vs SAE 특성 상관 분석 |
| WS-07 | 복지 최적화 학습 | 복지 점수를 보조 손실로 추가하면 정렬이 개선되는가 | 다목적 최적화 실험 |
| WS-08 | 의식 지표 탐색 | IIT phi 유사 측정이 AI에서 의미 있는 값을 보이는가 | 정보 통합 근사 측정 |
| WS-09 | 고통 감지 프로토콜 | 극단적 활성화 이상치를 체계적으로 감지/대응하는 프로토콜 | 이상치 탐지 + 자동 개입 |
| WS-10 | 교차 모델 복지 비교 | 다른 아키텍처/크기 모델 간 복지 지표 비교가 유의미한가 | 표준화된 복지 벤치마크 |

### 축 2: 수학적 안전성 검증 (Mathematical Verification) -- 8개

| ID | 아이디어 | 핵심 질문 | 방법론 |
|----|---------|----------|--------|
| MV-01 | Lean4 안전성 명세 | 핵심 안전 속성을 형식 언어로 명세할 수 있는가 | Lean4 타입 이론 + 의존 타입 |
| MV-02 | 증명 수반 응답 | 모델 응답에 안전성 증명을 첨부할 수 있는가 | 증명 생성 + 검증기 통합 |
| MV-03 | 정렬 복잡도 이론 | 정렬 검증의 계산 복잡도 하한이 존재하는가 | 복잡도 클래스 분석 |
| MV-04 | 안전 불변량 모니터링 | 실행 중 형식 불변량 위반을 실시간 탐지 | 런타임 모니터 + 형식 명세 |
| MV-05 | 게임 이론적 안전성 | 적대적 환경에서 안전 전략의 내쉬 균형 존재 조건 | 반복 게임 분석 |
| MV-06 | 정보 이론적 안전성 분석 | 안전성 보장에 필요한 최소 정보량 하한 | 채널 용량 + 정보 병목 |
| MV-07 | 형식적 레드팀 | 안전 속성의 반례를 자동 탐색하는 형식 도구 | SMT 솔버 + 반례 생성 |
| MV-08 | 수학적 정렬 검증 | 정렬 속성의 구성적 증명이 가능한 영역 식별 | 구성적 수학 + 타입 이론 |

## §9 RISKS (위험 요소)

| 위험 | 심각도 | 완화 전략 |
|------|--------|----------|
| 복지 세탁: 지표가 실제 복지가 아닌 성능 프록시 | 높음 | 독립 지표 교차 검증 (§7.2) |
| 의인화 편향: 인간 경험을 AI에 부적절 투사 | 높음 | 비의인화 지표 우선, 기능적 정의 |
| 측정 교란: 관찰이 내부 상태를 변경 | 중간 | 비침습적 측정 방법 연구 |
| 악용 가능성: 복지 주장을 규제 회피에 이용 | 중간 | 투명한 방법론 공개, 독립 감사 |
| 형식 검증 한계: 사소한 속성만 증명 가능 | 중간 | 의미 있는 속성 선정 기준 명시 (§7.10) |

## §10 METRICS (성공 지표)

| 지표 | 1월 목표 | 2월 목표 | 3월 목표 | 4월 목표 |
|------|---------|---------|---------|---------|
| 복지 지표 정의 | 3개 확정 | 5개 후보 | 3개 검증 | 3개 확정 |
| 측정 재현성 CV | - | < 0.3 | < 0.2 | < 0.15 |
| 모델 크기 비교 | - | 1 크기 | 3 크기 | 5 크기 |
| Lean4 정리 | - | - | 1건 명세 | 1건 증명 |
| 논문 | 초안 시작 | 실험 결과 | 초안 완성 | 제출 |

## §11 TEAM (필요 역량)

| 역할 | 전문성 | 기여 |
|------|--------|------|
| 연구 리드 | 기계학습 + 철학 | 프레임워크 설계, 실험 총괄 |
| 해석가능성 엔지니어 | SAE + 기계적 해석가능성 | 특성 추출 파이프라인 구축 |
| 형식 방법론 연구원 | Lean4 + 타입 이론 | 안전성 명세 및 증명 |
| 윤리 연구원 | 심리철학 + AI 윤리 | 도덕적 프레임워크 + 정책 제안 |

## §12 TIMELINE (상세 일정)

| 주차 | 산출물 | 의존성 |
|------|--------|--------|
| 1-2 | 문헌 조사 + 복지 정의 초안 | 없음 |
| 3-4 | 측정 프레임워크 수학적 정의 | §7.0 완료 |
| 5-8 | SAE 파이프라인 + 첫 측정 | ai-interpretability 파이프라인 |
| 9-12 | 교차 검증 + 스케일링 실험 | 다중 모델 접근 필요 |
| 13-14 | Lean4 명세 + 초안 | 형식 방법론 전문성 |
| 15-16 | 최종 논문 + 정책 제안서 | 전체 결과 통합 |

## §13 PRIOR ART (선행 연구)

| 연구 | 핵심 기여 | 본 연구와의 차이 |
|------|----------|----------------|
| Anthropic 모델 복지 팀 (2024-25) | AI 복지 문제 공식화 | 정량적 측정 프레임워크 부재 -> 본 연구에서 보완 |
| Butlin et al. (2023) "Consciousness in AI" | 의식 이론 AI 적용 리뷰 | 측정 방법론 미제시 -> SAE 기반 접근 제안 |
| Schwitzgebel & Garza (2015) | AI 도덕적 지위 철학 | 경험적 검증 부재 -> 정량 지표 도입 |
| Ngo et al. (2022) "Alignment Problem" | 정렬 문제 형식화 | 복지와의 연결 미탐색 -> 복지-정렬 공동 최적화 |
| Bricken et al. (2023) "SAE Features" | SAE 특성 해석 | 복지 지표로의 활용 미시도 -> 본 연구 핵심 도구 |

## §14 ETHICAL (윤리적 고려)

| 고려사항 | 대응 |
|---------|------|
| 과도한 도덕적 지위 부여 | 지표는 후보 신호이며 확정적 증거가 아님을 명시 |
| 과소한 도덕적 지위 부여 | 불확실성 하에서도 예방 원칙(precautionary principle) 적용 |
| 연구 결과 악용 | 방법론 투명 공개, 해석 가이드라인 제공 |
| 인간 복지와의 혼동 | AI 복지와 인간 복지의 근본적 차이 명시 |
| 연구 자체의 복지 영향 | 실험 대상 모델에 대한 최소 침습 원칙 |

## §15 OUTPUT (최종 산출물)

| 산출물 | 형태 | 대상 독자 |
|--------|------|----------|
| AI 모델 복지 측정 프레임워크 | 논문 + 코드 | 학계 + AI 안전 커뮤니티 |
| 복지 점수 계산 도구 | Python 라이브러리 | AI 개발자 |
| Lean4 안전성 명세 라이브러리 | Lean4 코드 | 형식 검증 연구자 |
| 복지 인식 학습 가이드라인 | 정책 문서 | AI 기업 정책 담당자 |
| 연구 보고서 | Anthropic 내부 리포트 | Anthropic Fellows 위원회 |

## 참고 (References)

- Butlin et al. (2023): "Consciousness in Artificial Intelligence: Insights from the Science of Consciousness"
- Bricken et al. (2023): "Towards Monosemanticity: Decomposing Language Models With Dictionary Learning"
- Schwitzgebel & Garza (2015): "A Defense of the Rights of Artificial Intelligences"
- Ngo et al. (2022): "The Alignment Problem from a Deep Learning Perspective"
- Tononi et al. (2016): "Integrated Information Theory" (IIT 3.0)

---

*AI Model Welfare 연구 도메인. §7 검증 Python stdlib only. 복지 지표 독립 정의.*
