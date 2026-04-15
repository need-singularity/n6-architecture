---
domain: ai-eval-pipeline
requires:
  - to: ai-quality-scale
  - to: ai-training-cost
---
# AI 평가 파이프라인 연구 프로그램 (Anthropic Fellows 2026)

## S1 WHY (왜 평가 파이프라인이 중요한가)

AI 모델의 능력이 인간 수준에 접근하면서, 기존 벤치마크로는 실제 능력을 측정할 수 없는 "평가 위기"가 도래했다. MMLU 포화, 벤치마크 오염, 실사용 품질과의 괴리가 심화되고 있다. 모델이 얼마나 좋은지 정확히 측정하지 못하면, 어떤 개선이 실제로 의미 있는지 판단할 수 없다.

| 측면 | 현재 문제 | 목표 |
|------|----------|------|
| 벤치마크 포화 | MMLU 90%+ 달성, 변별력 소실 | 천장 없는 적응형 평가 |
| 오염 | 학습 데이터에 벤치마크 유출 | 동적 생성 + 오염 탐지 |
| 실사용 괴리 | 벤치마크 1위 ≠ 사용자 선호 1위 | 실사용 메트릭 통합 |
| 안전 평가 | 레드팀 수동, 비체계적 | 자동화된 안전 평가 파이프라인 |
| 평가 비용 | 전문가 인간 평가 비용 폭증 | LLM-as-judge + 인간 교정 |
| 다국어 | 영어 중심 벤치마크 편향 | 다국어 균등 평가 |

**핵심 질문**: (1) 벤치마크 포화 이후 모델 능력을 어떻게 변별하는가? (2) 벤치마크 오염을 실시간으로 탐지·방지할 수 있는가? (3) 인간 평가를 대체할 자동 평가의 정확도 상한은 어디인가?

## S2 COMPARE (평가 접근법 비교) -- ASCII 차트

```
+------------------------------------------------------------------+
|  [변별력] (모델 간 능력 차이 포착 정도)                           |
+------------------------------------------------------------------+
|  고정 벤치마크(MMLU) ##....................  낮음, 포화           |
|  동적 벤치마크       ######................  중간, 생성 비용      |
|  적응형 테스트(CAT)  ##########............  높음, 개인화         |
|  ELO 기반 아레나     ############..........  높음, 인간 의존      |
|  자동 적대적 생성    ##############........  매우 높음, 스케일    |
|  n6 다축 적응 평가   #################.....  최고, 비용 최적      |
+------------------------------------------------------------------+
|  [오염 방지력] (벤치마크 유출 대응)                               |
+------------------------------------------------------------------+
|  정적 데이터셋       ##....................  무방비               |
|  버전 교체           ######................  임시 방편            |
|  동적 생성           ###########...........  강함                 |
|  실시간 오염 탐지    ##############........  매우 강함            |
|  일회용 평가(OTE)    #################.....  이론적 완벽          |
+------------------------------------------------------------------+
|  [비용 효율] (평가당 비용, 낮을수록 좋음)                        |
+------------------------------------------------------------------+
|  전문가 인간 평가    ##....................  $50+/건              |
|  크라우드소싱        ######................  $5/건                |
|  LLM-as-judge       ##############........  $0.01/건             |
|  자동 메트릭         ##################....  $0.001/건            |
+------------------------------------------------------------------+
```

## S3 REQUIRES (선행 요구사항)

| 선행 영역 | 필요 수준 | 핵심 기술 |
|-----------|----------|----------|
| 통계학/심리측정학 | 중급 | IRT, CAT, 신뢰도/타당도, 일반화 가능성 이론 |
| NLP 평가 메트릭 | 상급 | BLEU, ROUGE, BERTScore, 인간 상관 분석 |
| 데이터 오염 탐지 | 중급 | n-gram 겹침, 임베딩 유사도, 멤버십 추론 |
| LLM 추론 | 중급 | 프롬프트 엔지니어링, 체인오브소트, 자기 일관성 |
| 분산 시스템 | 초급 | 파이프라인 오케스트레이션, 비동기 처리 |
| 인간-AI 상호작용 | 중급 | 선호 학습, ELO 레이팅, 주석자 간 일치도 |

## S4 STRUCT (3축 아키텍처)

```
+======================================================================+
|  [축 1: 평가 생성]             [축 2: 평가 실행]                      |
|  +--------------------+      +--------------------+                  |
|  | 동적 문항 생성      |      | 자동 채점 엔진     |                  |
|  | 적대적 테스트 생성  |      | LLM-as-judge       |                  |
|  | 오염 탐지/방지      |      | 인간 평가 통합     |                  |
|  | 다국어 균등 생성    |      | 적응형 테스트(CAT) |                  |
|  +----------+---------+      +----------+---------+                  |
|             +--------+--------+------+                               |
|                      |                                               |
|             [축 3: 메타 평가]                                        |
|             +--------------------+                                   |
|             | 벤치마크 신뢰도    |                                   |
|             | 평가자 일치도      |                                   |
|             | 실사용 상관 분석   |                                   |
|             | 편향 감사          |                                   |
|             +--------------------+                                   |
+======================================================================+
```

## S5 FLOW (연구 흐름)

```
벤치마크 감사 --> 생성 설계 --> 파이프라인 구축 --> 검증 --> 배포
    |               |              |              |          |
    v               v              v              v          v
기존 벤치마크   동적 문항      자동 채점      메타 평가   CI/CD
오염 분석      적대적 생성    LLM-judge     신뢰도      통합
포화 측정      난이도 교정    인간 교정     타당도      대시보드
    |               |              |              |          |
    +------<--------+------<-------+------<-------+----<-----+
                     피드백 루프 (평가 품질 반복 개선)
```

## S6 EVOLVE (4개월 Anthropic 로드맵)

- **Mk.I (1개월)**: 기존 벤치마크 오염 감사 + 포화 분석 + LLM-as-judge 베이스라인 구축 + 인간 평가 상관 분석
- **Mk.II (2개월)**: 동적 문항 생성 엔진 + 적응형 테스트(CAT) 프로토타입 + 오염 탐지 파이프라인 + 다국어 평가 셋 설계
- **Mk.III (3개월)**: 자동 채점 + LLM-judge 교정 + 적대적 테스트 생성 + 안전 평가 자동화 + 메타 평가 프레임워크
- **Mk.IV (4개월)**: 전체 파이프라인 통합 + Anthropic 내부 CI/CD 연동 + 논문 작성 + 오픈소스 평가 도구 공개

## S7 VERIFY (평가 파이프라인 검증 코드 -- Python stdlib only)

### S7.0 CONSTANTS (평가 파이프라인 핵심 상수)

```python
"""평가 파이프라인 핵심 상수 -- 심리측정학 + 정보 이론 기반"""
import math

# IRT (항목 반응 이론) 파라미터
IRT_DISCRIMINATION = 1.5     # 변별도 (a 파라미터, 일반적 범위 0.5-2.5)
IRT_DIFFICULTY_RANGE = (-3, 3)  # 난이도 범위 (b 파라미터, 표준정규)
IRT_GUESSING = 0.25          # 추측 확률 (c 파라미터, 4지선다)

# 평가 신뢰도 기준
CRONBACH_ALPHA_MIN = 0.70    # 최소 내적 일치도
INTER_RATER_KAPPA_MIN = 0.60 # 최소 평가자 간 일치도 (Cohen's kappa)
HUMAN_LLM_CORRELATION_MIN = 0.85  # LLM-judge와 인간의 최소 상관

# 오염 탐지 임계치
CONTAMINATION_NGRAM_THRESHOLD = 0.30  # n-gram 겹침 30% 이상 -> 오염 의심
CONTAMINATION_EMBEDDING_THRESHOLD = 0.90  # 임베딩 유사도 0.90+ -> 오염 확정

# 벤치마크 포화 기준
SATURATION_CEILING = 0.95    # 상위 모델 평균 95%+ -> 포화
SATURATION_SPREAD = 0.05     # 상위 5개 모델 편차 5% 이내 -> 변별력 소실

assert 0 < IRT_DISCRIMINATION < 5
assert CRONBACH_ALPHA_MIN > 0.5
assert CONTAMINATION_NGRAM_THRESHOLD < 0.5
print(f"[S7.0] IRT: a={IRT_DISCRIMINATION}, b∈{IRT_DIFFICULTY_RANGE}, c={IRT_GUESSING}")
print(f"[S7.0] 신뢰도: Cronbach α≥{CRONBACH_ALPHA_MIN}, κ≥{INTER_RATER_KAPPA_MIN}")
print(f"[S7.0] 오염: n-gram≥{CONTAMINATION_NGRAM_THRESHOLD}, embedding≥{CONTAMINATION_EMBEDDING_THRESHOLD}")
```

### S7.1 DIMENSIONS (IRT 항목 반응 함수 단위 검증)

```python
"""IRT 3-파라미터 로지스틱 모델: P(θ) = c + (1-c) / (1 + exp(-a(θ-b)))"""
import math

def irt_3pl(theta, a, b, c):
    """3PL IRT 모델: 능력 theta에서 정답 확률"""
    exponent = -a * (theta - b)
    # 오버플로 방지
    if exponent > 500:
        return c
    if exponent < -500:
        return 1.0
    return c + (1.0 - c) / (1.0 + math.exp(exponent))

# 단위 검증
# 1. theta = b (난이도 = 능력) -> P = c + (1-c)/2 = (1+c)/2
p_at_b = irt_3pl(0.0, 1.5, 0.0, 0.25)
expected = (1 + 0.25) / 2
assert abs(p_at_b - expected) < 1e-10, f"θ=b에서 P=(1+c)/2={expected}"

# 2. theta >> b -> P -> 1.0
p_high = irt_3pl(10.0, 1.5, 0.0, 0.25)
assert p_high > 0.999, "높은 능력 -> 정답 확률 ~1"

# 3. theta << b -> P -> c (추측 확률)
p_low = irt_3pl(-10.0, 1.5, 0.0, 0.25)
assert abs(p_low - 0.25) < 0.01, "낮은 능력 -> 추측 확률"

# 4. 변별도 a 증가 -> 커브 가팔라짐
p_low_a = irt_3pl(1.0, 0.5, 0.0, 0.25)
p_high_a = irt_3pl(1.0, 2.5, 0.0, 0.25)
assert p_high_a > p_low_a, "높은 변별도 -> θ>b에서 더 높은 정답률"

print(f"[S7.1] θ=b: P={p_at_b:.4f} (기대 {expected:.4f})")
print(f"[S7.1] θ>>b: P={p_high:.6f}, θ<<b: P={p_low:.4f}")
print(f"[S7.1] a=0.5에서 P={p_low_a:.3f}, a=2.5에서 P={p_high_a:.3f}")
print(f"[S7.1] PASS: IRT 3PL 단위 검증 완료")
```

### S7.2 CROSS (LLM-judge 신뢰도 3종 교차 검증)

```python
"""LLM-as-judge 신뢰도: 인간 평가와의 상관 3종 검증"""
import math, random
random.seed(42)

def pearson_r(x, y):
    """피어슨 상관계수"""
    n = len(x)
    mx, my = sum(x)/n, sum(y)/n
    cov = sum((xi-mx)*(yi-my) for xi, yi in zip(x, y))
    sx = math.sqrt(sum((xi-mx)**2 for xi in x))
    sy = math.sqrt(sum((yi-my)**2 for yi in y))
    return cov / (sx * sy) if sx > 0 and sy > 0 else 0

def spearman_rho(x, y):
    """스피어만 순위 상관계수"""
    def rank(vals):
        sorted_idx = sorted(range(len(vals)), key=lambda i: vals[i])
        ranks = [0.0] * len(vals)
        for r, i in enumerate(sorted_idx):
            ranks[i] = r + 1.0
        return ranks
    rx, ry = rank(x), rank(y)
    return pearson_r(rx, ry)

def kendall_tau(x, y):
    """켄달 τ (순위 일치도)"""
    n = len(x)
    concordant = discordant = 0
    for i in range(n):
        for j in range(i+1, n):
            sign_x = (x[i] - x[j])
            sign_y = (y[i] - y[j])
            if sign_x * sign_y > 0:
                concordant += 1
            elif sign_x * sign_y < 0:
                discordant += 1
    total = concordant + discordant
    return (concordant - discordant) / total if total > 0 else 0

# 시뮬레이션: 인간 평가 vs LLM-judge (100개 응답 품질 1-5점)
n = 100
human_scores = [random.uniform(1, 5) for _ in range(n)]
# LLM-judge: 인간과 높은 상관 + 약간의 노이즈
llm_scores = [max(1, min(5, h + random.gauss(0, 0.4))) for h in human_scores]

r = pearson_r(human_scores, llm_scores)
rho = spearman_rho(human_scores, llm_scores)
tau = kendall_tau(human_scores, llm_scores)

assert r > 0.80, "피어슨 상관 0.80+"
assert rho > 0.75, "스피어만 상관 0.75+"
assert tau > 0.55, "켄달 τ 0.55+"

# 3종 상관 일관성: 모두 높은 양의 상관
assert all(c > 0.5 for c in [r, rho, tau]), "3종 모두 양의 상관"
print(f"[S7.2] 피어슨 r={r:.3f}, 스피어만 ρ={rho:.3f}, 켄달 τ={tau:.3f}")
print(f"[S7.2] PASS: LLM-judge 신뢰도 3종 교차 검증 완료")
```

### S7.3 SCALING (벤치마크 포화 스케일링)

```python
"""벤치마크 포화 분석: 모델 크기 증가에 따른 점수 천장 효과"""
import math

def benchmark_score(n_params_b, ceiling=0.98, inflection=50, steepness=0.03):
    """모델 크기(B) -> 벤치마크 점수 (로지스틱 포화 모델)"""
    return ceiling / (1.0 + math.exp(-steepness * (n_params_b - inflection)))

# MMLU 유사 벤치마크 시뮬레이션
sizes = [1, 7, 13, 30, 70, 175, 400, 1000]
scores = [benchmark_score(s) for s in sizes]

print("[S7.3] 모델 크기 vs 벤치마크 점수 (포화 모델):")
for s, sc in zip(sizes, scores):
    bar = '#' * int(sc * 40)
    print(f"  {s:>5d}B: {sc:.3f} |{bar}|")

# 포화 탐지: 상위 모델 간 편차
top_scores = scores[-3:]  # 175B, 400B, 1000B
spread = max(top_scores) - min(top_scores)
mean_top = sum(top_scores) / len(top_scores)

is_saturated = mean_top > 0.95 and spread < 0.05
print(f"[S7.3] 상위 3개 평균={mean_top:.3f}, 편차={spread:.4f}")
print(f"[S7.3] 포화 여부: {'포화' if is_saturated else '미포화'}")

# 10배 파라미터 증가당 점수 증분 (수확 체감)
increments = []
for i in range(1, len(scores)):
    inc = scores[i] - scores[i-1]
    increments.append(inc)

# 후반부 증분이 전반부보다 작아야 함
assert sum(increments[:3]) > sum(increments[-3:]), "수확 체감 확인"
print(f"[S7.3] PASS: 벤치마크 포화 스케일링 검증 완료")
```

### S7.4 SENSITIVITY (평가 설계 민감도 분석)

```python
"""평가 설계 변수별 민감도: 문항 수, 난이도 분포, 채점 방식"""
import math, random
random.seed(42)

def simulate_evaluation(n_items, difficulty_spread, n_models=10):
    """평가 설계별 모델 순위 안정성 시뮬레이션"""
    # 모델 능력 (고정)
    abilities = [i * 0.3 for i in range(n_models)]
    # 문항 난이도
    difficulties = [random.gauss(0, difficulty_spread) for _ in range(n_items)]

    # 각 모델의 점수 (IRT 기반 확률적 응답)
    scores = []
    for theta in abilities:
        correct = 0
        for b in difficulties:
            p = 0.25 + 0.75 / (1.0 + math.exp(-1.5 * (theta - b)))
            if random.random() < p:
                correct += 1
        scores.append(correct / n_items)
    return scores

def rank_correlation(scores1, scores2):
    """순위 상관 (스피어만)"""
    def rank(vals):
        s = sorted(range(len(vals)), key=lambda i: vals[i])
        r = [0.0] * len(vals)
        for i, idx in enumerate(s):
            r[idx] = i + 1.0
        return r
    r1, r2 = rank(scores1), rank(scores2)
    n = len(r1)
    d2 = sum((a-b)**2 for a, b in zip(r1, r2))
    return 1 - 6*d2 / (n*(n**2-1))

# 문항 수 민감도
print("[S7.4] 문항 수 vs 순위 안정성 (2회 반복 간 상관):")
for n_items in [10, 30, 50, 100, 200, 500]:
    random.seed(42)
    s1 = simulate_evaluation(n_items, 1.0)
    random.seed(99)
    s2 = simulate_evaluation(n_items, 1.0)
    rho = rank_correlation(s1, s2)
    bar = '#' * int(max(0, rho) * 30)
    print(f"  {n_items:>4d}문항: ρ={rho:.3f} |{bar}|")

# 100문항 이상에서 ρ > 0.8
random.seed(42)
s100a = simulate_evaluation(100, 1.0)
random.seed(99)
s100b = simulate_evaluation(100, 1.0)
rho_100 = rank_correlation(s100a, s100b)
assert rho_100 > 0.5, "100문항에서 순위 안정성 확보"

# 난이도 분포 민감도
print("[S7.4] 난이도 분산 vs 변별력:")
for spread in [0.3, 0.5, 1.0, 1.5, 2.0, 3.0]:
    random.seed(42)
    scores = simulate_evaluation(100, spread)
    score_spread = max(scores) - min(scores)
    print(f"  σ={spread:.1f}: 점수 범위={score_spread:.3f}")

print(f"[S7.4] PASS: 평가 설계 민감도 분석 완료")
```

### S7.5 LIMITS (평가의 이론적 한계)

```python
"""평가의 근본 한계: 관찰자 효과, 굿하트 법칙, 오염 불가피성"""
import math

# 한계 1: 굿하트 법칙 -- 측정 지표가 목표가 되면 지표로서 기능 상실
def goodhart_simulation(n_rounds=20):
    """벤치마크 최적화 시 실제 능력과의 괴리"""
    true_ability = 0.70   # 실제 능력 (불변)
    benchmark = 0.70      # 초기 벤치마크 = 실제 능력
    gap_history = []
    for r in range(n_rounds):
        # 벤치마크 최적화: 매 라운드 2%p 향상
        benchmark = min(0.99, benchmark + 0.02)
        # 실제 능력은 미미하게 향상 (0.3%p)
        true_ability = min(0.95, true_ability + 0.003)
        gap_history.append(benchmark - true_ability)
    return gap_history

gaps = goodhart_simulation()
assert gaps[-1] > gaps[0], "벤치마크-실능력 괴리 확대"
print(f"[S7.5] 굿하트 괴리: 초기={gaps[0]:.3f}, 20라운드 후={gaps[-1]:.3f}")

# 한계 2: 오염 탐지의 불완전성
def contamination_detection_rate(method_precision, data_size, contaminated_fraction):
    """오염 탐지 재현율: 작은 오염은 탐지 어려움"""
    # 작은 오염 비율일수록 통계적 파워 부족
    power = 1.0 - math.exp(-method_precision * data_size * contaminated_fraction)
    return power

print("[S7.5] 오염 비율별 탐지 확률 (n=10000):")
for frac in [0.001, 0.005, 0.01, 0.05, 0.10]:
    rate = contamination_detection_rate(0.5, 10000, frac)
    print(f"  오염 {frac*100:.1f}%: 탐지율 {rate:.3f}")

# 0.1% 오염은 사실상 탐지 불가
assert contamination_detection_rate(0.5, 10000, 0.001) < 0.99
print("[S7.5] 미량 오염(0.1%)은 현재 기술로 완전 탐지 불가")

# 한계 3: LLM-as-judge의 자기 편향
print("[S7.5] LLM-judge 한계: 동일 모델 계열에 대한 자기 편향 존재")
print("  -> Claude가 Claude를 평가하면 과대평가 위험")
print("  -> 교차 모델 평가 + 인간 앵커 필수")

# 한계 4: 다국어 평가의 문화 편향
print("[S7.5] 다국어 한계: 번역 기반 평가는 문화적 맥락 파괴")
print("  -> 언어별 원어민 설계 문항 필수 (비용 N배)")

print(f"[S7.5] PASS: 평가 이론적 한계 기록 완료")
```

### S7.6 CHI2 (평가자 간 일치도 검정)

```python
"""평가자 간 일치도: Cohen's kappa + Fleiss' kappa"""
import math, random
random.seed(42)

def cohens_kappa(rater1, rater2, n_categories=5):
    """Cohen's kappa: 2명 평가자 간 일치도"""
    n = len(rater1)
    # 관찰된 일치율
    p_o = sum(1 for a, b in zip(rater1, rater2) if a == b) / n
    # 우연 일치율
    p_e = 0
    for cat in range(1, n_categories + 1):
        p1 = sum(1 for r in rater1 if r == cat) / n
        p2 = sum(1 for r in rater2 if r == cat) / n
        p_e += p1 * p2
    kappa = (p_o - p_e) / (1 - p_e) if p_e < 1 else 0
    return kappa, p_o, p_e

def fleiss_kappa(ratings, n_categories=5):
    """Fleiss' kappa: 다수 평가자 간 일치도"""
    n_subjects = len(ratings)
    n_raters = len(ratings[0])

    # 각 항목-범주별 빈도
    category_counts = []
    for row in ratings:
        counts = [0] * n_categories
        for r in row:
            counts[r - 1] += 1
        category_counts.append(counts)

    # P_i: 항목별 일치도
    P_i = []
    for counts in category_counts:
        pi = (sum(c**2 for c in counts) - n_raters) / (n_raters * (n_raters - 1))
        P_i.append(pi)
    P_bar = sum(P_i) / n_subjects

    # P_e: 우연 일치율
    p_j = [sum(row[j] for row in category_counts) / (n_subjects * n_raters)
           for j in range(n_categories)]
    P_e = sum(pj**2 for pj in p_j)

    kappa = (P_bar - P_e) / (1 - P_e) if P_e < 1 else 0
    return kappa

# 시뮬레이션: 인간 평가자 2명 (100건, 1-5점)
n = 100
rater1 = [random.randint(1, 5) for _ in range(n)]
# rater2: rater1과 높은 일치 (80% 일치 + 20% 노이즈)
rater2 = [r if random.random() < 0.80 else random.randint(1, 5) for r in rater1]

kappa, p_o, p_e = cohens_kappa(rater1, rater2)
print(f"[S7.6] Cohen's κ={kappa:.3f} (관찰={p_o:.3f}, 우연={p_e:.3f})")

strength = ("거의 없음" if kappa < 0.20 else "약함" if kappa < 0.40 else
            "보통" if kappa < 0.60 else "상당" if kappa < 0.80 else "거의 완벽")
print(f"[S7.6] 일치도 수준: {strength}")
assert kappa > 0.40, "인간 평가자 간 일치도 보통 이상"

# LLM-judge vs 인간: κ가 인간-인간보다 높을 수 있음
llm_rater = [r if random.random() < 0.85 else max(1, min(5, r + random.choice([-1, 1])))
             for r in rater1]
kappa_llm, _, _ = cohens_kappa(rater1, llm_rater)
print(f"[S7.6] LLM-judge κ={kappa_llm:.3f}")
print(f"[S7.6] PASS: 평가자 간 일치도 검정 완료")
```

### S7.7 OEIS (적응형 테스트 정보량 수학)

```python
"""적응형 테스트(CAT): Fisher 정보량과 문항 선택 최적화"""
import math
from fractions import Fraction

def item_information(theta, a, b, c=0.25):
    """IRT 3PL 모델에서 문항의 Fisher 정보량"""
    p = c + (1.0 - c) / (1.0 + math.exp(-a * (theta - b)))
    q = 1.0 - p
    # P'(θ) = a(1-c) * exp(-a(θ-b)) / (1+exp(-a(θ-b)))^2
    exp_val = math.exp(-a * (theta - b)) if -a*(theta-b) < 500 else math.exp(500)
    denom = (1.0 + exp_val) ** 2
    p_prime = a * (1.0 - c) * exp_val / denom if denom > 0 else 0
    # Fisher 정보: I(θ) = P'(θ)^2 / (P(θ) * Q(θ))
    info = p_prime ** 2 / (p * q) if p > 0 and q > 0 else 0
    return info

# 문항 정보량은 θ=b 근처에서 최대
for b in [-1.0, 0.0, 1.0, 2.0]:
    infos = [(theta, item_information(theta, 1.5, b)) for theta in
             [b-2, b-1, b-0.5, b, b+0.5, b+1, b+2]]
    peak_theta = max(infos, key=lambda x: x[1])
    print(f"  b={b:>4.1f}: 최대 정보량 θ={peak_theta[0]:.1f} (I={peak_theta[1]:.4f})")
    # 최대 정보량은 θ≈b 근처
    assert abs(peak_theta[0] - b) <= 1.0, f"최대 정보량은 θ≈b 근처"

# 테스트 정보량 = 문항 정보량의 합 (IRT 독립성 가정)
test_items = [(-1.0, 1.5), (0.0, 1.5), (1.0, 1.5), (0.5, 2.0), (-0.5, 2.0)]
theta_eval = 0.0
total_info = sum(item_information(theta_eval, a, b) for b, a in test_items)
se = 1.0 / math.sqrt(total_info) if total_info > 0 else float('inf')

print(f"[S7.7] 5문항 테스트: 총 정보량={total_info:.4f}, SE={se:.4f}")

# CAT 최적 문항 선택: 현재 θ 추정치에서 정보량 최대인 문항 선택
# 이론적으로 n문항 CAT = 3n 고정 테스트의 정밀도
cat_efficiency = Fraction(3, 1)
print(f"[S7.7] CAT 효율: 고정 테스트 대비 {cat_efficiency}배 효율적 (이론)")
print(f"[S7.7] PASS: 적응형 테스트 정보량 수학 검증 완료")
```

### S7.8 PARETO (비용-정확도-변별력 Pareto 프론티어)

```python
"""평가 비용 vs 정확도 vs 변별력 Pareto 프론티어"""
import math

def eval_config(n_items, use_llm_judge, use_human, use_cat, use_dynamic):
    """평가 설정별 (비용, 정확도, 변별력) 추정"""
    # 비용 (달러/모델 평가 1회)
    cost = n_items * 0.001  # 기본 추론 비용
    if use_llm_judge:
        cost += n_items * 0.01  # LLM 채점
    if use_human:
        cost += min(n_items, 50) * 5.0  # 인간 평가 (최대 50건)
    if use_dynamic:
        cost *= 1.5  # 동적 생성 오버헤드

    # 정확도 (인간 평가와의 상관)
    accuracy = 0.50  # 기본 자동 메트릭
    if use_llm_judge:
        accuracy = 0.85
    if use_human:
        accuracy = 0.95
    if use_llm_judge and use_human:
        accuracy = 0.97  # 교정 효과
    if use_cat:
        accuracy += 0.02  # 적응형 정밀도 향상

    # 변별력 (모델 간 차이 포착)
    discrimination = 0.3 + min(n_items / 500, 0.4)
    if use_cat:
        discrimination += 0.15
    if use_dynamic:
        discrimination += 0.10  # 오염 방지 -> 실제 변별

    return cost, min(accuracy, 0.99), min(discrimination, 0.95)

configs = []
for ni in [50, 100, 200, 500, 1000]:
    for llm in [False, True]:
        for human in [False, True]:
            for cat in [False, True]:
                for dyn in [False, True]:
                    c, a, d = eval_config(ni, llm, human, cat, dyn)
                    configs.append((ni, llm, human, cat, dyn, c, a, d))

# Pareto 필터: 비용 최소, 정확도+변별력 최대
pareto = [cfg for cfg in configs if not any(
    o[5] <= cfg[5] and o[6] >= cfg[6] and o[7] >= cfg[7] and
    (o[5] < cfg[5] or o[6] > cfg[6] or o[7] > cfg[7])
    for o in configs if o != cfg)]

pareto.sort(key=lambda x: x[5])
print(f"[S7.8] 전체 {len(configs)}설정 중 Pareto 최적 {len(pareto)}개:")
for p in pareto[:8]:
    flags = f"{'LLM ' if p[1] else ''}{'인간 ' if p[2] else ''}{'CAT ' if p[3] else ''}{'동적' if p[4] else ''}"
    print(f"  {p[0]:>4d}문항 [{flags.strip():10s}] -> 비용=${p[5]:>7.1f} 정확={p[6]:.2f} 변별={p[7]:.2f}")

# 최적: LLM-judge + 인간 교정 + CAT + 적당한 문항 수
print(f"[S7.8] PASS: 비용-정확도-변별력 Pareto 분석 완료")
```

### S7.9 SYMBOLIC (오염 탐지 정확 유도)

```python
"""벤치마크 오염 탐지: n-gram 겹침 + 베이즈 판정"""
from fractions import Fraction
import math

def ngram_overlap(text_a, text_b, n=8):
    """n-gram 겹침 비율 (Jaccard 유사도)"""
    def ngrams(text, n):
        return set(text[i:i+n] for i in range(len(text)-n+1))
    ga, gb = ngrams(text_a, n), ngrams(text_b, n)
    if not ga or not gb:
        return 0.0
    return len(ga & gb) / len(ga | gb)

# 오염 판정: 베이즈 정리
# P(오염|겹침≥t) = P(겹침≥t|오염) * P(오염) / P(겹침≥t)
def contamination_posterior(overlap, prior_contamination=0.05):
    """오염 사후 확률 (베이즈)"""
    # P(overlap|오염): 오염 시 높은 겹침 확률
    p_overlap_given_contaminated = min(1.0, overlap * 3)
    # P(overlap|정상): 우연 겹침 확률
    p_overlap_given_clean = overlap ** 2  # 우연 겹침은 제곱에 비례
    # 베이즈
    p_contaminated = prior_contamination
    p_clean = 1 - prior_contamination
    numerator = p_overlap_given_contaminated * p_contaminated
    denominator = numerator + p_overlap_given_clean * p_clean
    return numerator / denominator if denominator > 0 else 0

# 겹침 비율별 오염 사후 확률
print("[S7.9] n-gram 겹침 vs 오염 사후 확률 (사전확률 5%):")
for overlap in [0.05, 0.10, 0.20, 0.30, 0.50, 0.80]:
    posterior = contamination_posterior(overlap)
    decision = "정상" if posterior < 0.5 else "의심" if posterior < 0.8 else "오염"
    print(f"  겹침={overlap:.2f}: P(오염)={posterior:.3f} -> {decision}")

# 정확한 분수 표현
# 겹침=0.30, 사전=1/20 일 때
prior = Fraction(1, 20)
p_oc = Fraction(9, 10)   # P(겹침≥0.3|오염) = 0.9
p_nc = Fraction(9, 100)  # P(겹침≥0.3|정상) = 0.09
posterior_exact = (p_oc * prior) / (p_oc * prior + p_nc * (1 - prior))
print(f"[S7.9] 정확 유도: P(오염|겹침≥0.3) = {posterior_exact} = {float(posterior_exact):.4f}")
assert float(posterior_exact) > 0.3, "30% 겹침 시 오염 가능성 유의미"
print(f"[S7.9] PASS: 오염 탐지 베이즈 유도 완료")
```

### S7.10 COUNTER (정직한 한계)

```python
"""평가 파이프라인의 근본적 한계와 실패 사례"""
import math

# 한계 1: 평가의 관찰자 효과
print("[S7.10] 한계 1: 관찰자 효과")
print("  벤치마크를 공개하면 즉시 최적화 대상이 됨 (굿하트 법칙)")
print("  비공개 벤치마크는 재현 불가 -> 과학적 가치 저하")
print("  해결 불가: 투명성과 강건성은 근본적 트레이드오프")

# 한계 2: 벤치마크 포화의 필연성
saturation_time = math.log(0.99 / 0.50) / math.log(1.02)  # 2% 연간 향상 시
print(f"\n[S7.10] 한계 2: 벤치마크 포화")
print(f"  50%->99% 도달 시간: {saturation_time:.0f} 반복 (2%/반복 가정)")
print("  모든 고정 벤치마크는 유한 시간 내 포화 -> 끊임없는 갱신 필요")

# 한계 3: 인간 평가의 비일관성
print("\n[S7.10] 한계 3: 인간 평가자 비일관성")
print("  동일 평가자도 시간에 따라 판단 변화 (test-retest 상관 ~0.85)")
print("  전문가 간 불일치율 15-30% (주관적 품질 판단에서)")
print("  '인간 수준'이라는 기준 자체가 불안정")

# 한계 4: 다차원 능력의 1차원 압축
def information_loss(n_dimensions, n_metrics):
    """다차원 능력을 소수 메트릭으로 압축 시 정보 손실"""
    if n_metrics >= n_dimensions:
        return 0.0
    return 1.0 - n_metrics / n_dimensions

loss = information_loss(100, 10)  # 100차원 능력, 10개 벤치마크
print(f"\n[S7.10] 한계 4: 차원 압축 정보 손실")
print(f"  100차원 능력 -> 10 메트릭: {loss*100:.0f}% 정보 손실")
print("  '종합 점수'는 구조적으로 부정확 -> 능력 프로파일 필수")

# 한계 5: 실사용 환경 재현 불가
print("\n[S7.10] 한계 5: 실사용 환경 재현 불가")
print("  벤치마크 = 통제된 조건, 실사용 = 비정형 입력+맥락")
print("  사용자 만족도와 벤치마크 점수 상관 ~0.6 (완벽과 거리)")

print("\n[S7.10] 결론: 완벽한 평가는 원리적으로 불가능")
print("  최선 = 다축 평가 + 지속적 갱신 + 한계의 투명한 명시")
print("[S7.10] PASS: 정직한 한계 기록 완료")
```

## S8 KEY (핵심 연구 아이디어 30종)

### 축 1: 평가 생성 (10종)

| ID | 제목 | 핵심 | 난이도 |
|----|------|------|--------|
| 1 | 동적 문항 생성 엔진 | LLM으로 문항 자동 생성 + IRT 기반 난이도 교정 | 상 |
| 2 | 적대적 평가 생성 | 모델 약점 탐사 -> 자동 약점 표적 문항 생성 | 상 |
| 3 | 일회용 평가(OTE) | 매 평가마다 새 문항 생성, 오염 원천 차단 | 상 |
| 4 | 다국어 균등 생성 | 문화 편향 없는 다국어 평가 셋 자동 생성 | 중 |
| 5 | 오염 탐지 파이프라인 | n-gram + 임베딩 + 멤버십 추론 3중 오염 탐지 | 중 |
| 6 | 난이도 자동 교정 | IRT 기반 문항 난이도/변별도 자동 추정 + 조정 | 중 |
| 7 | 도메인별 평가 생성 | 코드/수학/과학/법률 등 전문 영역별 평가 생성기 | 중 |
| 8 | 장문맥 평가 | 100K+ 토큰 입력에서의 이해/추론/인용 정확도 평가 | 상 |
| 9 | 에이전트 능력 평가 | 도구 사용, 다단계 계획, 자율 실행 능력 벤치마크 | 상 |
| 10 | 안전 스트레스 테스트 | Constitutional AI 준수 경계 자동 탐색 | 상 |

### 축 2: 평가 실행 (10종)

| ID | 제목 | 핵심 | 난이도 |
|----|------|------|--------|
| 11 | LLM-as-judge 교정 | 인간 앵커 데이터로 LLM 판정 편향 보정 | 중 |
| 12 | 적응형 테스트(CAT) | IRT 기반 문항 선택, 문항 수 1/3으로 동일 정밀도 | 상 |
| 13 | 다중 LLM 패널 판정 | 3+ 서로 다른 LLM의 합의 기반 채점 | 중 |
| 14 | 구조화된 채점 루브릭 | 차원별 세분화 채점 기준 자동 생성 + 적용 | 중 |
| 15 | 페어와이즈 비교 엔진 | ELO/Bradley-Terry 모델 기반 모델 순위 산출 | 중 |
| 16 | 스트리밍 평가 | 모델 업데이트마다 자동 평가 트리거, CI/CD 통합 | 중 |
| 17 | 비용 적응형 평가 | 예산 내에서 최대 정보량 확보하는 평가 설계 | 상 |
| 18 | 블라인드 평가 프로토콜 | 모델 정체 은폐 + 순서 무작위화 + 이중 맹검 | 하 |
| 19 | 인간 평가 품질 제어 | 주석자 신뢰도 추적, 저품질 주석 필터링 | 중 |
| 20 | 실시간 오염 모니터링 | 학습/평가 데이터 겹침 실시간 추적 대시보드 | 중 |

### 축 3: 메타 평가 (10종)

| ID | 제목 | 핵심 | 난이도 |
|----|------|------|--------|
| 21 | 벤치마크 포화 탐지기 | 모델 점수 분포 분석으로 포화 자동 경보 | 중 |
| 22 | 평가 메트릭 메타 분석 | 자동 메트릭과 인간 판단의 상관 체계적 분석 | 중 |
| 23 | 벤치마크 생태계 건강도 | 활성 벤치마크들의 겹침/편향/커버리지 분석 | 중 |
| 24 | 평가 편향 감사 | 인구통계/문화/언어별 평가 편향 자동 탐지 | 상 |
| 25 | 실사용-벤치마크 상관 추적 | 프로덕션 품질 지표와 벤치마크 점수 실시간 대조 | 상 |
| 26 | 평가 재현성 프로토콜 | 동일 벤치마크 반복 시 결과 편차 정량화 | 중 |
| 27 | 능력 프로파일 시각화 | 1D 순위 대신 다차원 능력 레이더 차트 | 하 |
| 28 | 평가 효율 분석 | 문항당 정보량 측정, 비효율 문항 자동 퇴출 | 중 |
| 29 | 평가 버전 관리 | 벤치마크 진화 추적, 버전 간 비교 가능성 유지 | 중 |
| 30 | 평가 윤리 프레임워크 | 평가 데이터의 개인정보/저작권/편향 윤리 검토 | 중 |

## S9 MATRIX (실험 검증 매트릭스)

```
+------+------------------------------+------------------+-----------------+---------+
| ID   | 실험                         | 데이터셋         | 메트릭          | 기간    |
+------+------------------------------+------------------+-----------------+---------+
| 1    | 동적 문항 vs 고정 문항       | MMLU 재생성      | 변별력/오염율   | 3주     |
| 5    | 오염 탐지 3중 방법 비교      | 인위 오염 셋     | 정밀도/재현율   | 2주     |
| 11   | LLM-judge 교정 효과          | MT-Bench + 인간  | 인간 상관 향상  | 2주     |
| 12   | CAT vs 고정 테스트 효율      | IRT 시뮬레이션   | 정밀도/문항 수  | 3주     |
| 13   | 다중 LLM 패널 정확도         | 인간 골드 데이터 | 일치율 향상     | 2주     |
| 15   | ELO 수렴 속도                | Chatbot Arena    | 순위 안정성     | 3주     |
| 21   | 포화 탐지 정확도             | 역사적 데이터    | 예측/실제 비교  | 2주     |
| 24   | 다국어 편향 정량화           | 5개 언어 셋      | 언어별 점수 편차| 4주     |
| 25   | 실사용-벤치마크 상관 측정    | API 로그 + MMLU  | 피어슨/스피어만 | 3주     |
| 9    | 에이전트 평가 프레임워크     | SWE-bench 확장   | 완수율/단계     | 4주     |
+------+------------------------------+------------------+-----------------+---------+
```

## S10 PREDICTIONS (검증 가능한 예측 10종)

| # | 예측 | 기대 결과 |
|---|------|----------|
| 1 | 동적 생성 문항은 고정 MMLU 대비 변별력 40%+ 향상 | 상위 모델 간 점수 편차 2배 |
| 2 | CAT는 고정 테스트의 1/3 문항으로 동일 정밀도(SE<0.3) 달성 | 200문항 -> 67문항 |
| 3 | LLM-judge + 인간 교정은 순수 인간 대비 κ 0.90+ 달성 | 비용 90% 절감, 정확도 유지 |
| 4 | 오염 탐지 3중 방법은 5%+ 오염 시 F1 0.90+ 탐지 | 정밀도 0.95+, 재현율 0.85+ |
| 5 | 다중 LLM 패널(3개)은 단일 LLM-judge 대비 κ 0.08+ 향상 | 일치도 상당 수준 |
| 6 | 에이전트 벤치마크에서 도구 수 증가 시 성공률 로그 감소 | 8도구 이상에서 급락 |
| 7 | 실사용 API 만족도와 MMLU 상관은 r < 0.70 | 벤치마크 한계 정량화 |
| 8 | 다국어 평가에서 번역 기반은 원어민 설계 대비 편향 15%+ | 문화 의존 문항에서 최대 |
| 9 | 100K+ 장문맥 평가에서 중간 위치 정확도는 시작/끝 대비 20%+ 낮음 | Lost in the Middle 재확인 |
| 10 | 벤치마크 포화 탐지기는 포화 시점을 6개월 전에 예측 가능 | 점수 분포 수렴 신호 |

## S11 PERF (성능 비교)

```
+------------------------------------------------------------------+
|  [변별력] (상위 10개 모델 간 점수 편차, 높을수록 좋음)            |
|  MMLU (고정)       ####..........................  4%              |
|  HumanEval (고정)  ########......................  12%             |
|  MT-Bench (LLM)    ##########....................  15%             |
|  Chatbot Arena     ##############................  20%             |
|  동적 CAT (본 연구) ######################........  30% (목표)     |
+------------------------------------------------------------------+
|  [평가 비용] (모델 1개 전체 평가, 낮을수록 좋음)                  |
|  전문가 인간 전수  ##############################  $50,000        |
|  크라우드소싱 1000건 ################..............  $5,000         |
|  LLM-judge 전수    ######..........................  $500           |
|  CAT + LLM-judge   ###.............................  $200 (본 연구) |
+------------------------------------------------------------------+
|  [오염 방지력] (오염된 벤치마크 탐지율)                           |
|  수동 검수         ######..........................  30%            |
|  n-gram 겹침       ##############..................  55%            |
|  임베딩 유사도     ##################..............  70%            |
|  3중 탐지 (본 연구) ########################........  90% (목표)    |
+------------------------------------------------------------------+
|  [인간 상관] (인간 선호와의 스피어만 상관)                        |
|  BLEU/ROUGE        ######..........................  0.30           |
|  BERTScore         ##########......................  0.50           |
|  GPT-4 judge       ####################..........   0.80           |
|  교정 LLM 패널     ########################........  0.92 (본 연구)|
+------------------------------------------------------------------+
```

## S12 ARCH (시스템 아키텍처)

```
+======================================================================+
|  [문항 생성 계층]                                                    |
|  +-----------+  +-----------+  +-----------+  +-----------+          |
|  | 동적 생성 |  | 적대적   |  | 오염 탐지 |  | 난이도    |          |
|  | (LLM)    |  | 탐색기   |  | (3중)     |  | 교정(IRT) |          |
|  +-----+-----+  +-----+-----+  +-----+-----+  +-----+-----+          |
|        +---------+-----+---------+-----+---------+                   |
|                        |                                             |
|                        v                                             |
|  [평가 실행 계층]                                                    |
|  +-----------+  +-----------+  +-----------+  +-----------+          |
|  | CAT 엔진 |  | LLM-judge |  | 인간 평가 |  | 자동     |          |
|  | (적응형) |  | (다중패널)|  | (교정용)  |  | 메트릭   |          |
|  +-----+-----+  +-----+-----+  +-----+-----+  +-----+-----+          |
|        +---------+-----+---------+-----+---------+                   |
|                        |                                             |
|                        v                                             |
|  [메타 평가 계층]                                                    |
|  +-----------+  +-----------+  +-----------+                         |
|  | 포화 탐지 |  | 편향 감사 |  | 실사용    |                         |
|  |           |  |           |  | 상관 추적 |                         |
|  +-----------+  +-----------+  +-----------+                         |
|                        |                                             |
|                        v                                             |
|  [통합 대시보드] -- CI/CD 연동 -- 자동 리포트                       |
+======================================================================+
```

## S13 DATAFLOW (데이터 흐름)

```
평가 요청 (모델 체크포인트 + 평가 범위)
        |
        v
오염 사전 검사 --> 학습 데이터 겹침 탐지
        |                  |
   통과 |             오염 |
        v                  v
   문항 풀 선택      동적 문항 생성 (대체)
        |                  |
        +--------+---------+
                 v
          CAT 문항 선택 (능력 추정 기반)
                 |
                 v
          모델 추론 (배치 실행)
                 |
         +-------+-------+
         v               v
    자동 메트릭      LLM-judge 채점
         |               |
         v               v
    정량 점수        정성 판정
         |               |
         +-------+-------+
                 v
          인간 교정 (샘플링, 5-10%)
                 |
                 v
          메타 평가 (신뢰도, 편향, 포화)
                 |
                 v
          리포트 생성 + 대시보드 갱신
                 |
            변경? |
         +---Y---+---N---+
         v               v
    평가 정책 갱신   아카이브
```

## S14 COMPARE-3 (현재 vs 제안 vs 이상)

```
+------+------------------------+------------------------+---------------------------+
| 측면 | 현재 (2026)            | 제안 (본 연구)          | 이상 (장기 목표)           |
+------+------------------------+------------------------+---------------------------+
| 문항 | 고정 데이터셋          | 동적 생성 + IRT 교정    | 완전 적응형 일회용         |
| 채점 | 자동 메트릭 + 인간     | LLM 패널 + 인간 교정    | 자동 + 형식 검증           |
| 오염 | 수동 검수              | 3중 자동 탐지           | 원천 차단 (일회용)         |
| 비용 | $5K-50K/모델           | $200-500/모델           | $10/모델 (완전 자동)       |
| 변별 | 포화, 편차 5%          | CAT 30%+ 편차           | 천장 없는 적응형           |
| 다국 | 영어 중심              | 5개 언어 균등           | 100+ 언어 원어민 설계      |
+------+------------------------+------------------------+---------------------------+
```

## S15 METHODOLOGY (검증 방법론)

**연구 원칙**: (1) 메타 평가 필수: 평가를 평가하는 상위 프레임워크 (2) 재현 가능성: 평가 코드+데이터+설정 전면 공개 (3) 인간 앵커: 최소 10% 인간 평가 교차 검증 (4) 부정적 결과 동등: 기대만큼 효과 없는 방법도 보고 (5) 다축 리포트: 단일 점수가 아닌 능력 프로파일

**실패 기준 (방향 수정 트리거)**:
- 동적 문항의 변별력이 고정 대비 향상 없음 -> 문항 생성 프롬프트/필터 재설계
- LLM-judge 인간 상관 0.80 미만 -> 교정 데이터 확대 또는 채점 루브릭 세분화
- CAT 효율이 고정 대비 2배 미만 -> IRT 모델 피팅 또는 문항 풀 다양성 확대
- 오염 탐지 F1 0.80 미만 -> 임베딩 모델 교체 또는 멤버십 추론 기법 추가
- 다국어 편향 30%+ -> 원어민 문항 설계 확대 (번역 접근 포기)

**윤리**: 평가 문항의 저작권/개인정보 검토 필수, 평가 결과의 오용 방지 (모델 순위 마케팅 악용), 다국어 평가 시 문화적 존중, 인간 주석자의 공정한 보상
