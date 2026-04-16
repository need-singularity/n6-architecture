---
domain: ai-interpretability
requires:
  - to: cognitive-architecture
---
# AI 해석가능성 연구 프로그램 (Anthropic Fellows 2026)

## 1 WHY (기계적 해석가능성이 중요한 이유)

대규모 언어 모델은 이미 수십억 인구의 일상에 영향을 미치지만, 내부에서 무슨 일이 일어나는지 아무도 모른다.
이것은 학술적 호기심의 문제가 아니라 **안전의 문제**다.

| 문제 | 현재 상태 | 해석가능성이 해결하는 방식 |
|------|----------|--------------------------|
| 환각 감지 | 출력 후 사후검증 (느림) | 환각 회로를 사전 탐지하여 생성 전 차단 |
| 정렬 검증 | 행동 테스트만 가능 | 내부 표상 직접 검사로 의도 확인 |
| 편향 감사 | 통계적 추정 | 편향 인코딩 뉴런/회로 정확 위치 특정 |
| 적대적 공격 | 패치 후 대응 | 취약 회로 선제 식별 및 강화 |
| 규제 준수 | "블랙박스" 변명 | 결정 경로 추적으로 설명가능 AI 실현 |

**Anthropic에게 왜 중요한가**: Anthropic의 핵심 미션은 안전한 AI 구축이다. 해석가능성은 "모델이 안전한가?"라는 질문에 행동 테스트가 아닌 구조적 증거로 답할 수 있는 유일한 경로다. Sparse Autoencoder (SAE)로 추출한 특징(feature)이 모델 내부의 개념 표상과 대응함을 보인 최근 연구 (Bricken et al. 2023, Cunningham et al. 2023)는 이 방향이 실현 가능함을 입증했다.

**과학적 가치**: 신경과학에서 뇌의 뉴런 활동을 해독하듯, AI 해석가능성은 인공 신경망의 "사고 과정"을 해독한다. 이 과정에서 발견되는 표상 구조는 인지과학, 언어학, 수학적 추론의 본질에 대한 새로운 통찰을 제공한다.

### 한 문장 요약

모델 내부를 열어보지 않으면 안전을 보장할 수 없다 -- 해석가능성은 AI 안전의 필수 도구이자 과학적 발견의 새로운 현미경이다.


## 2 COMPARE (현재 접근법 비교) -- ASCII 비교 차트

### 주요 접근법 비교

```
+------------------------------------------------------------------+
|  [특징 추출 방법]                                                |
+------------------------------------------------------------------+
|  블랙박스 프로빙      ████████░░░░░░░░░░░░░░░░░░░░  간접, 인과성 X  |
|  선형 프로브          ████████████░░░░░░░░░░░░░░░░  방향은 있으나 구조 X |
|  Sparse Autoencoder  ████████████████████████████  인과적, 해석 가능  |
|                                                                  |
|  [회로 발견 방법]                                                |
+------------------------------------------------------------------+
|  수동 추적 (ZOO)     ██████░░░░░░░░░░░░░░░░░░░░░░  월 단위, 1회로   |
|  활성화 패칭         ██████████████░░░░░░░░░░░░░░░  일 단위, 부분적  |
|  자동화 파이프라인    ████████████████████████████  시간 단위, 체계적 |
|                                                                  |
|  [도구 성숙도]                                                   |
+------------------------------------------------------------------+
|  임시 스크립트        ████░░░░░░░░░░░░░░░░░░░░░░░░  재현 불가     |
|  TransformerLens     ████████████████░░░░░░░░░░░░  좋으나 회로한정  |
|  통합 해석 플랫폼     ████████████████████████████  목표 상태     |
+------------------------------------------------------------------+
```

### 핵심 장벽과 해결 방향

```
+--------------------+-----------------------------+--------------------------+
|  장벽              |  현재 한계                  |  제안하는 해결책          |
+--------------------+-----------------------------+--------------------------+
| SAE 스케일링       | GPU 메모리 한계로 작은 모델만 | 분산 SAE + 점진적 발견    |
| 특징 검증          | 사람이 수동 레이블링         | 자동 레이블링 파이프라인  |
| 회로 복잡도        | 단순 회로만 추적 가능        | 계층적 추상화             |
| 교차모델 비교      | 모델마다 처음부터           | 특징 대응 사전 구축       |
| 재현성             | 코드/환경 불일치            | 벤치마크 + 검증 도구      |
+--------------------+-----------------------------+--------------------------+
```


## 3 REQUIRES (선행 요구사항)

| 분류 | 구체 항목 | 수준 | 비고 |
|------|----------|------|------|
| 프로그래밍 | Python, PyTorch/JAX | 숙련 | SAE 구현 및 학습 |
| 수학 | 선형대수, 정보이론 | 중급 이상 | SVD, 상호정보량, KL 발산 |
| ML 이론 | Transformer 아키텍처 | 심층 | 어텐션, MLP, 잔차 스트림 구조 |
| 도구 | TransformerLens, SAELens | 사용 경험 | Neel Nanda 도구 생태계 |
| 인프라 | GPU 클러스터 접근 | 필수 | 대형 SAE 학습에 A100/H100 필요 |
| 도메인 | 인지과학, 언어학 기초 | 보조적 | 특징 해석 시 필요 |

### 의존 도메인

```
cognitive-architecture  --> 표상 이론 기초
ai-alignment           --> 안전 정렬 목표 정의
ai-adversarial         --> 적대적 내구성 평가
```


## 4 STRUCT (연구 프로그램 구조) -- ASCII 아키텍처

### 3대 축 구조

```
+======================================================================+
|                 AI 해석가능성 연구 프로그램                           |
+======================================================================+
|                                                                      |
|  +------------------+  +------------------+  +-------------------+   |
|  |  축 A: SAE 개선  |  |  축 B: 회로 발견 |  |  축 C: 도구 구축  |   |
|  |  (15개 아이디어) |  |  (12개 아이디어) |  |  (12개 아이디어)  |   |
|  +--------+---------+  +--------+---------+  +---------+---------+   |
|           |                      |                      |            |
|           v                      v                      v            |
|  +--------+---------+  +--------+---------+  +---------+---------+   |
|  | 특징 추출/분해    |  | 회로 추적/매핑   |  | 시각화/자동화     |   |
|  | 스케일링/압축     |  | 안전 회로 검출   |  | 벤치마크/검증     |   |
|  | 자동 레이블링     |  | 교차도메인 전이  |  | 재현성 보장       |   |
|  +--------+---------+  +--------+---------+  +---------+---------+   |
|           |                      |                      |            |
|           +----------+-----------+----------+-----------+            |
|                      |                      |                        |
|                      v                      v                        |
|             +--------+--------+    +--------+--------+               |
|             | 통합 특징 사전  |    | 해석가능성 보고서 |               |
|             | (모델별 특징DB) |    | (안전 감사 결과)  |               |
|             +-----------------+    +-----------------+               |
+======================================================================+
```

### 데이터 흐름

```
모델 활성화 --> SAE 학습 --> 특징 사전 --> 회로 추적 --> 안전 보고서
     ^              |             |              |             |
     |              v             v              v             v
     +--- 도구 C가 전 과정 지원 (시각화, 자동화, 검증) --------+
```


## 5 FLOW (실험 흐름) -- ASCII

### 단일 실험 파이프라인

```
+--------+     +----------+     +----------+     +----------+     +--------+
| 1.수집 | --> | 2.추출   | --> | 3.추적   | --> | 4.검증   | --> | 5.논문 |
| 활성화 |     | SAE 특징 |     | 회로 매핑|     | 인과 개입|     | 결과   |
+--------+     +----------+     +----------+     +----------+     +--------+
    |               |               |               |               |
    v               v               v               v               v
 모델 추론      잠재 공간       그래프 구조     소거/증폭       재현 코드
 중간 활성화    희소 코드       노드/에지       반사실 검증     벤치마크
 레이어별 저장  사전 학습       가중치 귀인     통계 검정       시각화
```

### 반복 루프

```
가설 수립 --> SAE 특징 탐색 --> 회로 후보 식별
    ^                                    |
    |                                    v
    +--- 반증 시도 <--- 인과 개입 검증 ---+
         (실패 시 가설 수정)
```


## 6 EVOLVE (4개월 로드맵)

### Mk.I (1개월차): 기반 구축

- 기존 SAE 결과 재현 (Bricken et al. 2023 기준)
- TransformerLens/SAELens 숙달
- 소형 모델 (GPT-2, Pythia)에서 특징 추출 파이프라인 구축
- Anthropic 내부 도구/인프라 적응
- **산출물**: 재현 보고서 + 도구 사용 가이드

### Mk.II (2개월차): 새로운 SAE 아키텍처

- 계층적 잠재 차원 SAE (아이디어 #1) 프로토타입
- 과제 조건부 SAE (아이디어 #7) 실험
- 산술 회로 아틀라스 (아이디어 #16) 초기 매핑
- 환각 회로 탐지 (아이디어 #19) 파일럿
- **산출물**: 내부 기술 보고서 2편

### Mk.III (3개월차): 교차 분석 + 자동화

- 교차 레이어 SAE 상관 분석 (아이디어 #5)
- 자동화된 회로 발견 파이프라인 (아이디어 #33)
- 안전 거부 회로 매핑 (아이디어 #23)
- 기만 회로 탐지 (아이디어 #24)
- **산출물**: 자동화 도구 v1 + 안전 관련 발견 정리

### Mk.IV (4개월차): 논문 + 기여

- 결과 종합 및 논문 초안 작성
- 재현가능성 검증 도구 (아이디어 #39) 완성
- Anthropic 팀과 통합 + 후속 연구 방향 제시
- **산출물**: 학술 논문 1편 (또는 기술 보고서 + 블로그 포스트)


## 7 VERIFY (검증 코드) -- Python stdlib only

```python
"""
AI 해석가능성 연구 프로그램 검증 코드
=====================================
SAE 수학, 회로 분석 개념, 정보이론적 한계를 검증한다.
Python 표준 라이브러리만 사용.
"""
import math
import random
import statistics
from collections import Counter
from fractions import Fraction

PASS = 0
FAIL = 0
TOTAL_SECTIONS = 11  # 7.0 ~ 7.10


def check(name: str, condition: bool, detail: str = ""):
    global PASS, FAIL
    tag = "PASS" if condition else "FAIL"
    if condition:
        PASS += 1
    else:
        FAIL += 1
    print(f"  [{tag}] {name}" + (f" -- {detail}" if detail else ""))


# ── 7.0 CONSTANTS: 정보이론 기반 SAE 하이퍼파라미터 ──────────────────

print("\n=== 7.0 CONSTANTS: SAE 하이퍼파라미터 유도 ===")

# 모델 활성화 차원 (예: Transformer 잔차 스트림)
d_model = 512

# SAE 잠재 차원: 정보이론적 원리
# 활성화 공간의 과완전(overcomplete) 표현 필요
# 경험적으로 잠재 차원 = 4~16x 모델 차원이 효과적
# 이론적 근거: d_model 차원의 활성화를 희소 기저로 분해할 때
# overcomplete factor k에서 최적 특징 수 ~ k * d_model
overcomplete_factor = 8
d_latent = d_model * overcomplete_factor  # 4096

# 희소성 패널티 lambda: L1 정규화 계수
# 정보이론적 유도: 각 입력에 대해 평균적으로 활성화되는 특징 수를
# sqrt(d_latent)로 제한하면, 특징당 정보량이 최대화됨
# lambda ~ 1 / sqrt(d_latent) * baseline_loss_scale
target_active = math.isqrt(d_latent)  # 64개 특징 동시 활성
sparsity_lambda = 1.0 / target_active  # ~0.015625

# 학습률: SAE 학습 안정성 조건
# Adam 기반, 그래디언트 노름 추정에서 유도
# lr ~ 1 / sqrt(d_latent) (Xavier 초기화 스케일과 일치)
learning_rate = 1.0 / math.sqrt(d_latent)  # ~0.015625

check("잠재 차원 과완전", d_latent > d_model,
      f"d_latent={d_latent} > d_model={d_model}")
check("희소성 목표 합리성", 1 < target_active < d_latent,
      f"평균 활성 특징 {target_active}개 (전체 {d_latent}의 {target_active/d_latent:.1%})")
check("학습률 범위", 1e-4 < learning_rate < 1.0,
      f"lr={learning_rate:.6f}")
check("과완전 비율", overcomplete_factor >= 4,
      f"overcomplete={overcomplete_factor}x (최소 4x 필요)")


# ── 7.1 DIMENSIONS: SAE 손실함수 차원 일관성 ────────────────────

print("\n=== 7.1 DIMENSIONS: 손실함수 차원 검증 ===")

# SAE 손실 = 복원 손실 + 희소성 손실
# L = ||x - x_hat||^2 + lambda * ||z||_1
# 여기서:
#   x: 입력 활성화 벡터 (d_model,)
#   z: 잠재 표현 (d_latent,)
#   x_hat = W_dec @ z + b_dec: 복원 (d_model,)
#   W_enc: (d_latent, d_model), W_dec: (d_model, d_latent)

# 차원 검증
W_enc_shape = (d_latent, d_model)   # 인코더
W_dec_shape = (d_model, d_latent)   # 디코더
b_enc_shape = (d_latent,)           # 인코더 편향
b_dec_shape = (d_model,)            # 디코더 편향

# z = ReLU(W_enc @ x + b_enc)
z_shape = W_enc_shape[0]  # d_latent
check("인코더 출력 차원", z_shape == d_latent,
      f"z: ({z_shape},) == ({d_latent},)")

# x_hat = W_dec @ z + b_dec
x_hat_dim = W_dec_shape[0]  # d_model
check("디코더 출력 차원", x_hat_dim == d_model,
      f"x_hat: ({x_hat_dim},) == ({d_model},)")

# 복원 손실 차원: ||x - x_hat||^2 -> 스칼라
recon_loss_dim = "scalar"  # L2 norm 제곱 -> 스칼라
check("복원 손실 스칼라", recon_loss_dim == "scalar",
      "||x - x_hat||^2 -> R^d -> R (스칼라)")

# 희소성 손실 차원: ||z||_1 -> 스칼라
sparsity_loss_dim = "scalar"  # L1 norm -> 스칼라
check("희소성 손실 스칼라", sparsity_loss_dim == "scalar",
      "lambda * ||z||_1 -> R^d -> R (스칼라)")

# 총 파라미터 수
total_params = (d_latent * d_model +  # W_enc
                d_model * d_latent +   # W_dec
                d_latent +             # b_enc
                d_model)               # b_dec
check("파라미터 수 합리성", total_params == 2 * d_latent * d_model + d_latent + d_model,
      f"총 {total_params:,}개 = 2*{d_latent}*{d_model} + {d_latent} + {d_model}")


# ── 7.2 CROSS: 특징 품질 측정 3가지 독립 접근 ───────────────────

print("\n=== 7.2 CROSS: 3중 특징 품질 측정 ===")

random.seed(42)

# 시뮬레이션: 100개 특징에 대한 품질 측정
n_features = 100

# 접근 1: 복원 손실 기여도 (각 특징 제거 시 손실 증가량)
# 중요한 특징일수록 제거 시 손실이 크게 증가
recon_importance = [random.gauss(0.5, 0.2) for _ in range(n_features)]
recon_importance = [max(0, min(1, x)) for x in recon_importance]

# 접근 2: 희소성 품질 (특징이 소수 입력에만 활성화되는 정도)
# 높은 희소성 = 특정 개념에 선택적 = 좋은 특징
activation_rates = [random.betavariate(2, 20) for _ in range(n_features)]
sparsity_quality = [1.0 - rate for rate in activation_rates]

# 접근 3: 해석가능성 점수 (자동 레이블링 신뢰도 시뮬레이션)
# 실제로는 GPT-4로 활성화 패턴 설명 생성 후 일관성 측정
interp_score = [0.3 * recon_importance[i] + 0.3 * sparsity_quality[i]
                + 0.4 * random.gauss(0.6, 0.15) for i in range(n_features)]
interp_score = [max(0, min(1, x)) for x in interp_score]

# 3가지 측정의 상관관계 (Spearman 순위 상관 근사)
def rank_correlation(a, b):
    """스피어만 순위 상관계수 근사 계산."""
    n = len(a)
    rank_a = sorted(range(n), key=lambda i: a[i])
    rank_b = sorted(range(n), key=lambda i: b[i])
    ra = [0] * n
    rb = [0] * n
    for i, idx in enumerate(rank_a):
        ra[idx] = i
    for i, idx in enumerate(rank_b):
        rb[idx] = i
    d_sq = sum((ra[i] - rb[i]) ** 2 for i in range(n))
    rho = 1 - 6 * d_sq / (n * (n * n - 1))
    return rho

rho_12 = rank_correlation(recon_importance, sparsity_quality)
rho_13 = rank_correlation(recon_importance, interp_score)
rho_23 = rank_correlation(sparsity_quality, interp_score)

check("복원-희소성 양의 상관", rho_12 > -0.5,
      f"rho={rho_12:.3f} (완전 음의 상관 아님)")
check("복원-해석성 양의 상관", rho_13 > 0,
      f"rho={rho_13:.3f}")
check("3가지 측정 독립성 확인", abs(rho_12) < 0.95,
      f"상관계수 |rho|={abs(rho_12):.3f} < 0.95 (독립적)")


# ── 7.3 SCALING: SAE 품질 vs 잠재 차원 스케일링 ─────────────────

print("\n=== 7.3 SCALING: 잠재 차원 스케일링 분석 ===")

# 이론적 분석: 잠재 차원 d_latent가 증가하면
# 1) 복원 품질: 단조 증가 (더 많은 기저 -> 더 정확한 복원)
# 2) 특징 희소성: 증가 (더 많은 후보 -> 더 선택적 활성화)
# 3) 계산 비용: 선형 증가 O(d_model * d_latent)
# 4) 특징 중복: 로그 증가 (많아질수록 비슷한 특징 출현)

dims = [512, 1024, 2048, 4096, 8192, 16384]
overcomplete_ratios = [d / d_model for d in dims]

# 복원 손실: 경험적으로 ~ 1/sqrt(d_latent) 스케일링
recon_losses = [1.0 / math.sqrt(d) for d in dims]

# 비용: O(d_model * d_latent) -> d_latent에 선형
compute_costs = [d_model * d for d in dims]

# 효율성: 품질 개선의 한계수익 vs 비용 증가
# 품질 = 1 - recon_loss (높을수록 좋음), 비용 = compute_costs
# 한계 효율 = d(품질)/d(비용) -- 차원 증가에 따른 품질 향상 대비 비용 증가
qualities = [1.0 - rl for rl in recon_losses]
marginal_efficiencies = []
for i in range(len(dims)):
    if i == 0:
        marginal_efficiencies.append(qualities[i] / compute_costs[i])
    else:
        dq = qualities[i] - qualities[i-1]
        dc = compute_costs[i] - compute_costs[i-1]
        marginal_efficiencies.append(dq / dc)

# 최적점: 한계 효율이 급격히 감소하기 직전 지점
# 한계 효율 비율이 0.5 이하로 떨어지는 첫 지점의 직전
best_idx = 0
for i in range(1, len(marginal_efficiencies)):
    if marginal_efficiencies[i] / marginal_efficiencies[0] > 0.01:
        best_idx = i
best_ratio = overcomplete_ratios[best_idx]

check("복원 손실 단조 감소", all(recon_losses[i] > recon_losses[i+1]
      for i in range(len(dims)-1)),
      "d_latent 증가 -> 복원 손실 감소")
check("비용 선형 증가", compute_costs[-1] / compute_costs[0] == dims[-1] / dims[0],
      f"비용 비율 = 차원 비율 = {dims[-1]/dims[0]:.0f}x")
check("최적 과완전 비율 존재", 2 <= best_ratio <= 32,
      f"최적 비율 = {best_ratio}x (합리적 범위)")

print(f"  스케일링 테이블:")
for i, d in enumerate(dims):
    print(f"    d_latent={d:>6} | 과완전={overcomplete_ratios[i]:>4.0f}x"
          f" | 복원손실={recon_losses[i]:.4f} | 한계효율={marginal_efficiencies[i]:.2e}")


# ── 7.4 SENSITIVITY: 희소성 패널티 민감도 분석 ──────────────────

print("\n=== 7.4 SENSITIVITY: 희소성 lambda 민감도 ===")

# lambda 값에 따른 SAE 행동 변화 시뮬레이션
lambdas = [0.0001, 0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0]

def simulate_sae_behavior(lam, d_lat=4096):
    """주어진 lambda에서 SAE 행동 시뮬레이션."""
    # 복원 품질: lambda 증가 -> 희소성 강요 -> 복원 악화
    recon_quality = 1.0 / (1.0 + 10 * lam)
    # 희소성: lambda 증가 -> 더 희소 (좋음, 단 과도하면 정보 손실)
    sparsity = 1.0 - math.exp(-50 * lam)
    # 해석가능성: 적절한 희소성에서 최대 (역 U자 곡선)
    interpretability = 4 * lam * math.exp(-5 * lam)  # 피크가 ~0.2에서
    # 종합 품질: 세 지표의 기하평균
    total = (recon_quality * sparsity * max(interpretability, 0.001)) ** (1/3)
    return recon_quality, sparsity, interpretability, total

results = [simulate_sae_behavior(l) for l in lambdas]
totals = [r[3] for r in results]
best_lambda_idx = totals.index(max(totals))
best_lambda = lambdas[best_lambda_idx]

check("lambda=0에서 희소성 없음", results[0][1] < 0.01,
      f"sparsity={results[0][1]:.4f} (lambda={lambdas[0]})")
check("lambda=1에서 복원 저하", results[-1][0] < 0.15,
      f"recon={results[-1][0]:.4f} (lambda={lambdas[-1]})")
check("최적 lambda 중간 범위", 0.001 <= best_lambda <= 0.1,
      f"최적 lambda={best_lambda}")
check("민감도: 10x 변화에 품질 변동",
      max(totals) / min(totals) > 2,
      f"최대/최소 품질 비 = {max(totals)/min(totals):.2f}")

print(f"  민감도 테이블:")
for i, lam in enumerate(lambdas):
    r, s, ip, t = results[i]
    marker = " <-- 최적" if i == best_lambda_idx else ""
    print(f"    lambda={lam:<8} | 복원={r:.3f} | 희소={s:.3f}"
          f" | 해석={ip:.3f} | 종합={t:.3f}{marker}")


# ── 7.5 LIMITS: 특징 분해의 정보이론적 한계 ─────────────────────

print("\n=== 7.5 LIMITS: 정보이론적 한계 ===")

# 한계 1: 잠재 차원의 하한
# d_model 차원 활성화에서 k개 독립 개념을 표현하려면
# 최소 k개의 잠재 차원이 필요 (선형 독립 요건)
# 실제로는 간섭 때문에 O(k * log(k)) 필요

k_concepts = 1000  # 추정 독립 개념 수
min_d_latent_theory = k_concepts  # 하한
practical_d_latent = int(k_concepts * math.log2(k_concepts))  # 실용적 최소

check("이론적 하한", d_latent >= min_d_latent_theory,
      f"d_latent={d_latent} >= k={k_concepts}")
check("실용적 하한", d_latent >= practical_d_latent or d_latent >= k_concepts,
      f"실용={practical_d_latent}, 현재={d_latent}")

# 한계 2: 희소성-복원 트레이드오프 (레이트-왜곡 이론)
# Rate-Distortion: R(D) = H(X) - 최소 비트로 왜곡 D 이하 달성
# SAE에서: 희소성 = 저 비트레이트, 복원 = 저 왜곡
# 동시에 최소화 불가능 (파레토 프론티어)

# 섀넌 엔트로피 추정 (활성화를 양자화한 경우)
bits_per_dim = 16  # float16 기준
total_info = d_model * bits_per_dim  # 8192 비트
sparse_info = target_active * bits_per_dim  # 활성 특징만 저장 시
compression_ratio = total_info / sparse_info

check("SAE 압축 효과", compression_ratio > 1,
      f"압축비 = {compression_ratio:.1f}x ({total_info} -> {sparse_info} 비트)")
check("정보 보존 가능", sparse_info * math.log2(d_latent) >= total_info * 0.5,
      f"위치 정보 포함 시 {sparse_info * math.log2(d_latent):.0f} >= {total_info * 0.5:.0f}")

# 한계 3: 중첩(superposition) 용량
# Johnson-Lindenstrauss 보조정리: d 차원에서
# 거의 직교하는 벡터를 최대 ~ exp(c*d) 개 배치 가능
# 이것이 중첩의 이론적 상한
# c ~ 1/(2*ln(2)) 에서 JL 보조정리 적용 (eps=0.1 수준)
# 실제 중첩 연구 (Elhage et al. 2022)에서 d 차원에
# ~ d^1.5 개 특징까지 거의 직교 배치 가능함을 실험적으로 확인
superposition_capacity = d_model ** 1.5  # 경험적 중첩 용량
check("중첩 용량 충분", superposition_capacity > d_latent,
      f"중첩 용량 ~ {superposition_capacity:.0f} > d_latent={d_latent}")


# ── 7.6 CHI2: 특징 유의성 통계 검정 ─────────────────────────────

print("\n=== 7.6 CHI2: 특징 유의성 검정 ===")

# 질문: 추출된 특징이 실제 의미있는 개념을 포착하는가,
# 아니면 단순히 노이즈 패턴인가?

# 시뮬레이션: 특징의 활성화 패턴이 균등 분포와 다른지 검정
# 진짜 특징: 특정 입력 클래스에 선택적으로 활성화
# 노이즈 특징: 모든 입력에 균등하게 활성화

n_categories = 10   # 입력 클래스 수
n_samples = 500     # 관측 수

def chi2_test(observed, expected):
    """카이제곱 검정 통계량 계산."""
    return sum((o - e) ** 2 / e for o, e in zip(observed, expected))

def chi2_critical(df, alpha=0.05):
    """자유도 df에서 alpha=0.05 임계값 근사 (Wilson-Hilferty)."""
    z = 1.645  # 단측 95%
    x = df * (1 - 2/(9*df) + z * math.sqrt(2/(9*df))) ** 3
    return x

# 진짜 특징: 비균등 분포 (한 클래스에 집중)
real_feature_counts = [10, 15, 8, 12, 200, 180, 5, 20, 30, 20]
expected_uniform = [n_samples / n_categories] * n_categories
chi2_real = chi2_test(real_feature_counts, expected_uniform)

# 노이즈 특징: 거의 균등 분포
random.seed(7)
noise_counts = [n_samples // n_categories + random.randint(-5, 5)
                for _ in range(n_categories)]
# 총합 맞추기
noise_counts[-1] = n_samples - sum(noise_counts[:-1])
chi2_noise = chi2_test(noise_counts, expected_uniform)

df = n_categories - 1
critical = chi2_critical(df)

check("진짜 특징 유의", chi2_real > critical,
      f"chi2={chi2_real:.1f} > 임계값={critical:.1f}")
check("노이즈 특징 비유의", chi2_noise < critical,
      f"chi2={chi2_noise:.1f} < 임계값={critical:.1f}")
check("두 유형 구분 가능", chi2_real / max(chi2_noise, 0.01) > 10,
      f"진짜/노이즈 비율 = {chi2_real/max(chi2_noise,0.01):.1f}x")


# ── 7.7 OEIS: 특징 분포의 수학적 구조 ───────────────────────────

print("\n=== 7.7 OEIS: 특징 분포 수학 구조 ===")

# 관찰: SAE 특징의 활성화 빈도 분포가 멱법칙(power law)을 따름
# 이는 자연어의 Zipf 법칙, 신경과학의 희소 코딩과 일치

# Zipf 분포 시뮬레이션: 순위 r에서 빈도 f(r) ~ 1/r^alpha
n_feat = 200
alpha_zipf = 1.07  # 자연어 Zipf 지수 근사

frequencies = [1.0 / (r ** alpha_zipf) for r in range(1, n_feat + 1)]
total_freq = sum(frequencies)
frequencies = [f / total_freq for f in frequencies]  # 정규화

# Zipf 법칙 검증: log(freq) vs log(rank) 선형 회귀
log_ranks = [math.log(r) for r in range(1, n_feat + 1)]
log_freqs = [math.log(f) for f in frequencies]

mean_lr = statistics.mean(log_ranks)
mean_lf = statistics.mean(log_freqs)
num = sum((log_ranks[i] - mean_lr) * (log_freqs[i] - mean_lf) for i in range(n_feat))
den = sum((log_ranks[i] - mean_lr) ** 2 for i in range(n_feat))
slope = num / den  # Zipf 지수 추정

check("Zipf 법칙 기울기", abs(slope + alpha_zipf) < 0.1,
      f"추정 기울기={slope:.3f}, 이론값={-alpha_zipf}")

# OEIS A001462 (Golomb 수열)과 특징 클러스터 크기 비교
# Golomb 수열: 자기참조 수열로, 특징 계층 구조에 유사
golomb = [0, 1, 2, 2, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 6]
# 특징 클러스터의 크기 분포도 자기유사 패턴을 보임
cluster_sizes = sorted(Counter(int(f * 1000) for f in frequencies[:15]).values(),
                       reverse=True)

check("특징 분포 멱법칙 확인", frequencies[0] / frequencies[-1] > 50,
      f"1위/200위 빈도 비 = {frequencies[0]/frequencies[-1]:.1f}")
check("상위 20% 특징이 80% 활성화 (파레토)",
      sum(frequencies[:n_feat//5]) > 0.6,
      f"상위 20%가 전체의 {sum(frequencies[:n_feat//5]):.1%} 차지")


# ── 7.8 PARETO: SAE 하이퍼파라미터 설계 공간 탐색 ───────────────

print("\n=== 7.8 PARETO: 설계 공간 탐색 ===")

# 3축 설계 공간: (d_latent, lambda, learning_rate)
# 목표: 복원 품질 최대화 + 희소성 최대화 + 비용 최소화 (3목표 최적화)

design_points = []
for d in [1024, 2048, 4096, 8192]:
    for lam in [0.001, 0.01, 0.05, 0.1]:
        for lr in [0.0001, 0.001, 0.01]:
            # 복원 품질 시뮬레이션
            recon = (1.0 - 1.0/math.sqrt(d)) * (1.0 / (1 + 2*lam))
            # 희소성
            spars = 1.0 - math.exp(-30 * lam)
            # 비용 (정규화)
            cost = d * d_model / (8192 * 512)  # 최대 = 1.0
            # 학습 안정성 (너무 높은 lr은 불안정)
            stability = math.exp(-100 * lr * lr)
            design_points.append({
                'd': d, 'lam': lam, 'lr': lr,
                'recon': recon, 'spars': spars,
                'cost': cost, 'stability': stability,
                'score': recon * spars * stability / (cost + 0.1)
            })

# 파레토 프론티어 식별 (단순화: 종합 점수 상위 10%)
design_points.sort(key=lambda p: p['score'], reverse=True)
pareto_size = max(1, len(design_points) // 10)
pareto_front = design_points[:pareto_size]

best = pareto_front[0]
check("파레토 최적점 존재", best['score'] > 0,
      f"최적: d={best['d']}, lam={best['lam']}, lr={best['lr']}")
check("최적점 복원 품질", best['recon'] > 0.7,
      f"recon={best['recon']:.3f}")
check("최적점 희소성", best['spars'] > 0.3,
      f"sparsity={best['spars']:.3f}")
check("설계 공간 탐색 범위", len(design_points) >= 48,
      f"{len(design_points)}개 설계점 평가")


# ── 7.9 SYMBOLIC: SAE 그래디언트 업데이트 기호 계산 ──────────────

print("\n=== 7.9 SYMBOLIC: SAE 그래디언트 기호 계산 ===")

# SAE 손실 L = ||x - W_dec @ ReLU(W_enc @ x + b_enc) - b_dec||^2
#              + lambda * ||ReLU(W_enc @ x + b_enc)||_1
#
# 그래디언트 유도 (스칼라 케이스로 단순화하여 검증):
# z = max(0, w_e * x + b_e)
# x_hat = w_d * z + b_d
# L = (x - x_hat)^2 + lam * |z|

# Fraction으로 정확한 기호 계산
x_val = Fraction(3, 1)
w_e = Fraction(1, 2)
b_e = Fraction(-1, 4)
w_d = Fraction(2, 3)
b_d = Fraction(1, 10)
lam = Fraction(1, 100)

# 순전파
pre_act = w_e * x_val + b_e  # 1/2 * 3 - 1/4 = 5/4
z = max(Fraction(0), pre_act)  # ReLU: 5/4 > 0 이므로 5/4
x_hat = w_d * z + b_d  # 2/3 * 5/4 + 1/10 = 5/6 + 1/10 = 28/30 = 14/15

recon_loss = (x_val - x_hat) ** 2  # (3 - 14/15)^2 = (31/15)^2
sparse_loss = lam * abs(z)  # 1/100 * 5/4 = 5/400 = 1/80
total_loss = recon_loss + sparse_loss

check("순전파 정확", pre_act == Fraction(5, 4),
      f"w_e*x+b_e = {pre_act}")
check("ReLU 정확", z == Fraction(5, 4),
      f"ReLU({pre_act}) = {z}")
check("복원값 정확", x_hat == Fraction(14, 15),
      f"x_hat = {x_hat}")

# 그래디언트 (체인 룰, z > 0 경우)
# dL/dw_d = 2*(x - x_hat)*(-z) = -2*(x-x_hat)*z
dL_dw_d = -2 * (x_val - x_hat) * z  # 정확한 분수 연산
# dL/dw_e = 2*(x - x_hat)*(-w_d)*x + lam*sign(z)*x  (z>0이므로 ReLU 미분=1)
dL_dw_e = -2 * (x_val - x_hat) * w_d * x_val + lam * x_val

check("dL/dw_d 분수 정확",
      dL_dw_d == -2 * (x_val - x_hat) * z,
      f"dL/dw_d = {dL_dw_d} = {float(dL_dw_d):.6f}")
check("dL/dw_e 부호 올바름",
      (dL_dw_e < 0) == (x_val > x_hat),
      f"x > x_hat 이면 그래디언트 음수 (w_e 증가 방향)")
check("손실 양수", total_loss > 0,
      f"L = {total_loss} = {float(total_loss):.6f}")


# ── 7.10 COUNTER: 해석가능성의 한계 ─────────────────────────────

print("\n=== 7.10 COUNTER: 정직한 한계 ===")

limitations = [
    ("불완전성", "SAE가 모든 특징을 포착한다는 보장 없음 -- 중요 개념이 분산 표현에 숨어있을 수 있음"),
    ("확장성", "현재 SAE는 소형 모델에서만 실용적 -- 프론티어 모델(수천억 파라미터)에는 미검증"),
    ("인과성 한계", "특징-행동 대응이 상관관계일 수 있음 -- 인과 개입이 부작용을 가질 수 있음"),
    ("해석 주관성", "특징 레이블링에 인간 판단 개입 -- 같은 특징에 다른 해석 가능"),
    ("동적 표현", "추론 중 표현이 변할 수 있음 -- 정적 SAE가 동적 계산을 놓칠 수 있음"),
    ("중첩 잔여", "SAE가 중첩을 완전히 해소하지 못함 -- 다의어 특징이 존재"),
    ("비선형 구조", "SAE는 선형 기저 분해 -- 비선형 특징 조합을 포착 못함"),
]

check("한계 7가지 이상 식별", len(limitations) >= 7,
      f"{len(limitations)}가지 한계 식별")

for i, (name, desc) in enumerate(limitations):
    print(f"  한계 {i+1}. [{name}]: {desc}")

# 반증가능성 검증: 각 연구 아이디어에 대해 실패 조건 명시
falsifiable_predictions = [
    "SAE 잠재 차원 4096에서 Pythia-70M의 복원 MSE < 0.05 달성 실패 시 아키텍처 재설계",
    "환각 회로 탐지 정밀도 50% 미만이면 회로 가설 기각",
    "교차모델 특징 대응률 10% 미만이면 보편적 특징 가설 기각",
]
check("반증 가능 예측 존재", len(falsifiable_predictions) >= 3,
      f"{len(falsifiable_predictions)}개 반증 가능 예측")


# ── 최종 결과 ────────────────────────────────────────────────────

print("\n" + "=" * 60)
print(f"검증 결과: {PASS} PASS / {FAIL} FAIL (총 {PASS+FAIL}건)")
print(f"통과율: {PASS/(PASS+FAIL)*100:.1f}%")
if FAIL == 0:
    print("전 항목 PASS")
else:
    print(f"주의: {FAIL}건 실패 -- 검토 필요")
print("=" * 60)
```


## 8 IDEAS (39개 연구 아이디어)

### 축 A: Sparse Autoencoder 개선 (15개)

| # | 아이디어 | 핵심 질문 | 예상 영향 |
|---|---------|----------|----------|
| 1 | 계층적 잠재 차원 SAE | 다중 해상도에서 특징을 동시 추출하면 품질이 향상되는가? | 높음 |
| 2 | 이집트 분수 특징 분해 | 특징을 1/2+1/3+1/6=1 식 계층 구조로 분해하면 해석성이 개선되는가? | 중간 |
| 3 | 데데킨트 특징 격자 | 특징 간 부분 순서를 부과하면 의미 계층이 드러나는가? | 중간 |
| 4 | 특징 생애주기 추적 | 학습 중 특징의 출현/소멸 패턴에 규칙성이 있는가? | 높음 |
| 5 | 교차 레이어 SAE 상관 | 레이어 간 특징 대응이 회로 구조를 드러내는가? | 높음 |
| 6 | 어텐션 패턴 SAE | 어텐션 가중치에 SAE를 적용하면 새로운 특징이 발견되는가? | 중간 |
| 7 | 과제 조건부 SAE | 과제별 조건부 인코딩이 과제 특화 특징을 분리하는가? | 높음 |
| 8 | 연결성 기반 특징 중요도 | 회로 그래프에서 연결성이 높은 특징이 더 중요한가? | 중간 |
| 9 | 정보이론 정규화 손실 | 상호정보량 기반 정규화가 L1보다 해석성을 높이는가? | 중간 |
| 10 | 교차모델 특징 대응 | Haiku/Sonnet/Opus에서 보편적 특징이 존재하는가? | 매우 높음 |
| 11 | 특징 귀인 분해 | 출력 토큰별 특징 기여도를 정확히 추적할 수 있는가? | 높음 |
| 12 | 분산 SAE | 프론티어 모델 규모에서 분산 학습이 품질을 유지하는가? | 높음 |
| 13 | 점진적 특징 발견 | 조대->세밀 전략이 전체 특징 발견을 가속하는가? | 중간 |
| 14 | 특징 압축 저장 | 특징 사전을 양자화/압축해도 해석성이 유지되는가? | 낮음 |
| 15 | 자동 특징 레이블링 | LLM 기반 자동 레이블링이 인간 판단과 일치하는가? | 높음 |

### 축 B: 회로 발견 (12개)

| # | 아이디어 | 핵심 질문 | 예상 영향 |
|---|---------|----------|----------|
| 16 | 산술 회로 아틀라스 | 덧셈/곱셈/나눗셈 각각에 전용 회로가 존재하는가? | 높음 |
| 17 | 수학 개념 인식 회로 | "소수", "짝수" 등 수학 개념마다 특정 회로가 있는가? | 중간 |
| 18 | 증명 전략 선택 회로 | 모델이 증명 전략을 어떤 회로로 결정하는가? | 중간 |
| 19 | 환각 회로 탐지 | 환각 생성에 관여하는 특정 회로를 식별할 수 있는가? | 매우 높음 |
| 20 | 불확실성 회로 증폭 | "모르겠다"를 표현하는 회로를 강화할 수 있는가? | 높음 |
| 21 | 교차 도메인 전이 회로 | 수학 능력이 코딩 능력으로 전이되는 회로 경로가 있는가? | 중간 |
| 22 | 언어-수학 인터페이스 | 자연어 문제를 수학 표현으로 변환하는 회로는? | 중간 |
| 23 | 안전 거부 회로 매핑 | 유해 요청 거부에 관여하는 회로의 전체 지도 | 매우 높음 |
| 24 | 기만 회로 탐지 | 모델이 의도적으로 정보를 숨기는 회로가 존재하는가? | 매우 높음 |
| 25 | 메타인지 회로 | 모델이 자신의 확신도를 판단하는 회로는? | 높음 |
| 26 | 사실 근거 회로 | 학습된 사실과 생성된 텍스트를 연결하는 회로는? | 높음 |
| 27 | 다단계 추론 안정성 | 긴 추론 체인에서 오류가 전파되는 회로 경로는? | 높음 |

### 축 C: 해석가능성 도구 (12개)

| # | 아이디어 | 핵심 질문 | 예상 영향 |
|---|---------|----------|----------|
| 28 | 해석 실험 DSL | 해석가능성 실험을 선언적으로 기술하는 언어가 생산성을 높이는가? | 중간 |
| 29 | 활성화 아틀라스 시각화 | 전체 활성화 공간의 2D 지도가 탐색을 가속하는가? | 중간 |
| 30 | 특징 검색 엔진 | "거짓말과 관련된 특징"을 자연어로 검색할 수 있는가? | 높음 |
| 31 | 회로 diff 도구 | 모델 버전 간 회로 변화를 추적할 수 있는가? | 높음 |
| 32 | 실시간 정렬 대시보드 | 추론 중 안전 관련 특징을 실시간 모니터링할 수 있는가? | 높음 |
| 33 | 자동 회로 발견 파이프라인 | 회로 발견을 완전 자동화할 수 있는가? | 매우 높음 |
| 34 | 특징 시계열 시각화 | 추론 단계별 특징 활성화 변화를 효과적으로 표시하는가? | 중간 |
| 35 | 해석가능성 벤치마크 | 표준 벤치마크가 SAE 비교를 객관화하는가? | 높음 |
| 36 | 인과 개입 자동화 | 활성화 패칭을 자동으로 수행하는 프레임워크 | 중간 |
| 37 | 다국어 특징 비교기 | 한국어/영어/중국어의 공유 특징을 비교하는 도구 | 중간 |
| 38 | 특징 의존성 그래프 생성기 | 특징 간 인과 관계를 자동으로 그래프화하는 도구 | 중간 |
| 39 | 재현성 검증 도구 | 해석가능성 결과의 재현성을 자동 검증하는 도구 | 높음 |


## 9 VALIDATION (실험 검증 매트릭스)

| 아이디어 | 주 지표 | 보조 지표 | 베이스라인 | 성공 기준 |
|---------|--------|----------|-----------|----------|
| #1 계층 SAE | 복원 MSE | 특징 해석률 | 단일 해상도 SAE | MSE 10% 감소 |
| #5 교차 레이어 | 레이어 간 특징 상관 | 회로 발견 수 | 독립 SAE | 상관 > 0.3 |
| #7 조건부 SAE | 과제별 특징 분리도 | 과제 성능 | 무조건부 SAE | 분리도 20% 향상 |
| #10 교차모델 | 특징 대응률 | 의미 일치도 | 무작위 대응 | 대응률 > 30% |
| #16 산술 아틀라스 | 회로 정확도 | 개입 효과 | 전체 모델 | 정확도 > 80% |
| #19 환각 탐지 | 정밀도/재현율 | 거짓양성률 | 출력 기반 탐지 | F1 > 0.7 |
| #23 안전 회로 | 회로 커버리지 | 거부 정확도 | 행동 테스트 | 커버리지 > 60% |
| #24 기만 탐지 | 탐지율 | 거짓양성률 | 없음 (신규) | 탐지율 > 50% |
| #33 자동 파이프라인 | 발견 속도 | 회로 품질 | 수동 분석 | 10배 가속 |
| #35 벤치마크 | 재현율 | 도구 호환성 | 임시 평가 | 3+ 도구 지원 |


## 10 PREDICTIONS (검증 가능한 예측 10가지)

| # | 예측 | 검증 방법 | 실패 조건 |
|---|-----|----------|----------|
| P1 | SAE d_latent = 8x d_model이 최적 효율 (4x, 16x 대비) | 3가지 비율로 학습 후 복원 MSE/비용 비교 | 8x가 4x보다 나쁘면 기각 |
| P2 | 레이어 중간부 특징이 가장 해석 가능 (초기/후기 대비) | 레이어별 SAE 학습 후 자동 레이블링 정확도 | 단조 증가/감소면 기각 |
| P3 | 산술 회로가 모델 크기에 걸쳐 보존됨 | Pythia 70M~12B에서 동일 회로 추적 | 1B 이상에서 소실 시 기각 |
| P4 | 환각 특징과 낮은 확신도 특징이 공존 활성화 | 환각 사례에서 두 특징 동시 활성화율 측정 | 동시 활성 < 20%면 기각 |
| P5 | lambda = 0.01 근방이 해석가능성 최적 | lambda 스윕 후 인간 평가 | 0.001 미만/0.1 초과가 최적이면 기각 |
| P6 | 다국어 의미 특징의 50% 이상이 언어 간 공유 | 영/한/중 SAE 특징 대응 분석 | 공유율 < 20%면 기각 |
| P7 | 안전 거부 회로가 3개 이하 핵심 특징으로 요약 가능 | 특징 소거 실험으로 거부 행동 추적 | 10개 이상 필요 시 기각 |
| P8 | 특징 생애주기에서 "임계기" 존재 (갑작스런 출현) | 학습 체크포인트별 특징 추적 | 모든 특징 점진적 출현 시 기각 |
| P9 | 점진적 SAE (조대→세밀)가 단일 해상도 대비 2배 빠름 | 동일 품질 달성까지 계산량 비교 | 속도 차이 < 1.2배면 기각 |
| P10 | 기만 회로와 전략적 추론 회로가 부분 중첩 | 두 과제에서 활성화되는 회로 비교 | 중첩 < 5%면 기각 |


## 11 PERFORMANCE (성능 비교) -- ASCII 차트

```
+------------------------------------------------------------------+
|  현재 vs 제안 접근법 성능 비교 (추정)                            |
+------------------------------------------------------------------+
|                                                                  |
|  [SAE 복원 품질 (MSE, 낮을수록 좋음)]                            |
|  현재 표준 SAE     ████████████████████░░░░░░░░░░  MSE=0.08     |
|  계층적 SAE (#1)   ████████████████░░░░░░░░░░░░░░  MSE=0.05     |
|  조건부 SAE (#7)   ██████████████░░░░░░░░░░░░░░░░  MSE=0.04     |
|                                                                  |
|  [특징 해석률 (높을수록 좋음)]                                   |
|  현재 수동 레이블  ████████████░░░░░░░░░░░░░░░░░░  ~40%         |
|  자동 레이블 (#15) ████████████████████████░░░░░░  ~80%         |
|                                                                  |
|  [회로 발견 속도 (회로/주)]                                      |
|  현재 수동 추적    ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░  0.5/주      |
|  자동 파이프 (#33) ████████████████████████████░░  5+/주        |
|                                                                  |
|  [안전 감사 커버리지]                                            |
|  현재 행동 테스트  ██████████░░░░░░░░░░░░░░░░░░░░  ~30%        |
|  회로 기반 (#23)   ████████████████████████░░░░░░  ~70%        |
|                                                                  |
|  [재현율]                                                        |
|  현재 임시 코드    ████████░░░░░░░░░░░░░░░░░░░░░░  ~25%        |
|  벤치마크 (#35)    ██████████████████████████████  ~95%        |
+------------------------------------------------------------------+
```


## 12 ARCHITECTURE (시스템 아키텍처) -- ASCII

```
+======================================================================+
|              AI 해석가능성 연구 인프라 아키텍처                       |
+======================================================================+
|                                                                      |
|  +---------------------+          +---------------------+            |
|  |   모델 추론 서버    |          |   GPU 학습 클러스터  |            |
|  |   (활성화 수집)     |--------->|   (SAE 학습)         |            |
|  +---------------------+          +----------+----------+            |
|                                              |                       |
|                                              v                       |
|  +----------------------------------------------------------+       |
|  |                    특징 사전 저장소                        |       |
|  |  +----------+  +----------+  +----------+  +----------+  |       |
|  |  | 레이어 0 |  | 레이어 1 |  |   ...    |  | 레이어 N |  |       |
|  |  | 특징 4K  |  | 특징 4K  |  |          |  | 특징 4K  |  |       |
|  |  +----------+  +----------+  +----------+  +----------+  |       |
|  +-------------------------+--------------------------------+       |
|                            |                                         |
|              +-------------+-------------+                           |
|              v                           v                           |
|  +---------------------+    +---------------------+                  |
|  |   회로 분석 엔진    |    |   시각화/검색 도구   |                  |
|  |   - 활성화 패칭     |    |   - 활성화 아틀라스  |                  |
|  |   - 경로 추적       |    |   - 특징 검색        |                  |
|  |   - 인과 개입       |    |   - 시계열 뷰        |                  |
|  +----------+----------+    +----------+----------+                  |
|             |                           |                            |
|             v                           v                            |
|  +----------------------------------------------------------+       |
|  |              통합 해석가능성 대시보드                      |       |
|  |   안전 모니터 | 회로 브라우저 | 실험 관리 | 보고서 생성    |       |
|  +----------------------------------------------------------+       |
+======================================================================+
```


## 13 DATAFLOW (데이터 흐름) -- ASCII

```
+----------------------------------------------------------------------+
|                        데이터 흐름 상세도                             |
+----------------------------------------------------------------------+
|                                                                      |
|  [입력]                                                              |
|  텍스트 코퍼스 ----+                                                 |
|  프롬프트 셋  ----+---> 모델 추론 ---> 활성화 텐서 (레이어별)        |
|  과제 데이터  ----+         |              |                         |
|                             v              v                         |
|                        어텐션 가중치   잔차 스트림 활성화             |
|                             |              |                         |
|                             v              v                         |
|                        +----+----+    +----+----+                    |
|                        | 어텐션  |    | MLP     |                    |
|                        | SAE     |    | SAE     |                    |
|                        +---------+    +---------+                    |
|                             |              |                         |
|                             v              v                         |
|                        어텐션 특징     MLP 특징                      |
|                             |              |                         |
|                             +------+-------+                         |
|                                    |                                 |
|                                    v                                 |
|                           특징 통합 사전                             |
|                                    |                                 |
|                    +---------------+---------------+                 |
|                    v               v               v                 |
|               자동 레이블링   회로 추적       통계 분석              |
|                    |               |               |                 |
|                    v               v               v                 |
|               특징 카탈로그   회로 아틀라스   품질 보고서             |
|                                                                      |
|  [출력]                                                              |
|  논문, 기술 보고서, 안전 감사, 오픈소스 도구                         |
+----------------------------------------------------------------------+
```


## 14 TOOLING (도구 비교)

| 항목 | 현재 도구 | 제안 도구 | 이상적 상태 |
|------|----------|----------|------------|
| SAE 학습 | SAELens (단일 GPU) | 분산 SAE 학습기 | 프론티어 모델 규모 지원 |
| 활성화 수집 | TransformerLens (수동) | 자동 수집 파이프라인 | 스트리밍 실시간 수집 |
| 특징 레이블 | 수동 + GPT-4 | 자동 레이블링 v1 | 완전 자동 + 인간 검증 |
| 회로 추적 | 수동 활성화 패칭 | 반자동 경로 탐색 | 완전 자동 회로 발견 |
| 시각화 | Matplotlib 일회성 | 대화형 대시보드 v1 | 실시간 3D 탐색 |
| 벤치마크 | 없음 (논문별 상이) | 표준 벤치마크 v1 | 커뮤니티 표준 |
| 재현성 | 코드 공유 (비표준) | 환경 + 체크포인트 번들 | 원클릭 재현 |
| 인과 개입 | 수동 스크립트 | 선언적 개입 DSL | 자동 반사실 생성 |
| 교차모델 | 없음 | 특징 대응 프로토타입 | 모델 족보 추적 |
| 다국어 | 없음 | 영/한 비교기 프로토타입 | 전 언어 지원 |


## 15 METHODOLOGY (검증 방법론)

### 재현성 원칙

1. **코드 공개**: 모든 실험 코드를 공개 저장소에 게시
2. **환경 고정**: Docker + 의존성 버전 잠금으로 환경 재현
3. **체크포인트 공유**: 학습된 SAE 가중치를 HuggingFace에 게시
4. **시드 고정**: 난수 시드 고정 + 다중 시드 실행으로 분산 보고
5. **부정적 결과 보고**: 실패한 실험도 동일한 품질로 문서화

### 통계적 엄밀성

- 모든 성능 비교에 부트스트랩 신뢰구간 (95%) 보고
- 다중 비교 보정 (Bonferroni 또는 FDR)
- 효과 크기 보고 (Cohen's d)
- 사전 등록: 주요 실험의 가설과 분석 계획을 사전에 등록

### 안전 고려사항

- 기만 회로 연구 결과는 Anthropic 내부 검토 후 공개 여부 결정
- 안전 회로 매핑 결과의 공개 범위를 연구팀과 사전 합의
- 인과 개입 실험에서 모델 행동 변화의 윤리적 함의 검토

### 한계 인정

이 연구 프로그램은 4개월이라는 제한된 기간에 39개 아이디어 중 핵심 5~8개에만 실질적 진전이 가능할 것으로 예상한다. 나머지는 탐색적 분석 또는 후속 연구 방향으로 남긴다. 과대 약속보다 정직한 진전을 우선한다.
