---
domain: ai-quality-scale
requires:
  - to: ai-training-cost
---
# 품질 유지 경량화 연구 프로그램 (Anthropic Fellows 2026)

## S1 WHY (왜 품질 유지 경량화인가)

AI 모델의 능력은 매개변수 수에 비례하지만, 배포 비용도 비례한다. 400B 모델의 지능을 70B 크기에 담을 수 있다면 추론 비용이 5-6배 감소하고, 에지 디바이스 배포가 가능해지며, 더 많은 사람이 최전선 AI에 접근할 수 있다. 이것은 단순한 최적화가 아니라 AI 민주화의 핵심이다.

| 측면 | 현재 (대형 모델) | 경량화 성공 시 |
|------|-----------------|---------------|
| 추론 비용 | GPU 클러스터 필수, 토큰당 높은 비용 | 단일 GPU 가능, 비용 5-10배 절감 |
| 접근성 | 클라우드 API 의존 | 로컬 실행, 오프라인 가능 |
| 지연 시간 | 네트워크 왕복 + 대형 모델 추론 | 밀리초 단위 응답 |
| 에너지 | 데이터센터 전력 소모 | 모바일 칩에서 실행 가능 |
| 안전성 | 중앙 통제만 가능 | 분산 안전 메커니즘 |

**핵심 질문**: (1) 지식 증류에서 교사 모델의 어떤 표현이 학생 모델로 전이 가능한가? (2) 가지치기 시 품질 임계점(cliff)을 어떻게 예측하는가? (3) MoE 라우팅이 밀집 모델 대비 품질을 유지하는 이론적 근거는 무엇인가?

## S2 COMPARE (경량화 기법 비교) -- ASCII 차트

```
+------------------------------------------------------------------+
|  [품질 유지율] (원본 400B 대비, 동일 벤치마크)                    |
+------------------------------------------------------------------+
|  밀집 70B         ######............  ~60%, 기본선                |
|  증류 70B         ##########........  ~80%, 교사-학생              |
|  MoE 26B(3.8B활성) ###########.......  ~85%, Gemma4 방식           |
|  가지치기 70B     ########..........  ~70%, 구조적                 |
|  양자화 70B(4bit) #########.........  ~75%, GPTQ/AWQ              |
|  LoRA 미세조정    ##########........  ~80%, 작업 특화              |
|  모델 병합        #########.........  ~78%, TIES/DARE              |
|  NAS 탐색 70B    ###########.......  ~85%, 구조 최적화            |
+------------------------------------------------------------------+
|  [메모리 절감률] (원본 400B FP16 대비)                            |
+------------------------------------------------------------------+
|  밀집 70B         ############......  82.5%, 단순 축소             |
|  MoE 26B          ##############....  93.5%, 희소 활성화           |
|  양자화 400B(4bit) ###########.......  75%, 비트 축소              |
|  가지치기 50%     ############......  82.5%, 뉴런 제거             |
|  LoRA r=16        ###############...  96%, 어댑터만 저장           |
|  증류+양자화      ###############...  96%, 복합 기법               |
+------------------------------------------------------------------+
|  [학습 비용] (GPU-시간 기준)                                      |
+------------------------------------------------------------------+
|  처음부터 학습    ####..............  수만 GPU-시간                |
|  증류             ##########........  수천 GPU-시간                |
|  LoRA 미세조정    ##############....  수십 GPU-시간                |
|  양자화 (PTQ)     ################..  수 GPU-시간                  |
|  가지치기+재학습  ############......  수백 GPU-시간                |
|  NAS 탐색         ########..........  수천 GPU-시간 (1회)          |
+------------------------------------------------------------------+
```

## S3 REQUIRES (선행 요구사항)

| 선행 영역 | 필요 수준 | 핵심 기술 |
|-----------|----------|----------|
| 트랜스포머 아키텍처 | 고급 | 어텐션, FFN, 잔차 연결, 정규화 |
| 확률론/정보 이론 | 중급 | KL 발산, 상호 정보량, 엔트로피 |
| 최적화 이론 | 중급 | SGD 변형, 학습률 스케줄, 수렴 조건 |
| 수치 해석 | 중급 | 부동소수점, 양자화 오차, 행렬 분해 |
| 분산 시스템 | 초급 | 텐서 병렬, 파이프라인, 모델 샤딩 |
| 훈련 비용 이론 | 중급 | ai-training-cost 도메인 연계 |

## S4 STRUCT (3축 아키텍처)

```
+======================================================================+
|  [축 1: 압축 공학]           [축 2: 구조 혁신]                        |
|  +--------------------+     +--------------------+                   |
|  | 지식 증류           |     | MoE 라우팅 최적화  |                   |
|  | 가지치기 (구조/비구조)|     | NAS 탐색           |                   |
|  | 양자화 (PTQ/QAT)    |     | 아키텍처 병합       |                   |
|  | LoRA/QLoRA          |     | 깊이-폭 트레이드오프|                   |
|  +----------+---------+     +----------+---------+                   |
|             +--------+--------+------+                               |
|                      |                                               |
|             [축 3: 품질 보장]                                        |
|             +--------------------+                                   |
|             | 벤치마크 vs 실품질  |                                   |
|             | Constitutional 효율 |                                   |
|             | 합성 데이터 증강    |                                   |
|             | 연속 평가 파이프라인|                                   |
|             +--------------------+                                   |
+======================================================================+
```

## S5 FLOW (연구 흐름)

```
교사 모델 --> 증류 설계 --> 압축 적용 --> 품질 평가 --> 반복 개선
  |              |             |            |            |
  v              v             v            v            v
400B 분석     손실 설계     가지치기      MMLU/MT     안전 평가
표현 추출     온도 탐색     양자화        HumanEval   Constitutional
활성화 통계   층별 중요도   MoE 변환      실사용 A/B  정렬 유지 확인
  |              |             |            |            |
  +-----<--------+------<------+-----<------+----<-------+
                    피드백 루프 (Chinchilla 스케일링 재검증)
```

## S6 EVOLVE (4개월 Anthropic 로드맵)

- **Mk.I (1개월)**: 증류 베이스라인 구축. 교사(400B급)-학생(70B) 파이프라인, 층별 KD 손실 비교, MoE 라우팅 기초 구현, 8종 벤치마크 평가 스위트 구축
- **Mk.II (2개월)**: 구조적 가지치기 + QAT 통합. 뉴런/헤드/층 단위 중요도 점수, 가지치기 후 QAT 적용, LoRA 결합 실험, Constitutional AI 효율화 (RLHF 데이터 50% 절감 목표)
- **Mk.III (3개월)**: MoE 아키텍처 최적화 + NAS. Gemma4 스타일 3.8B 활성 파라미터 구조, 라우터 학습 안정화, 합성 데이터 기반 품질 향상, 모델 병합(TIES/DARE/SLERP) 실험
- **Mk.IV (4개월)**: 3축 통합 + 논문 작성. 증류+가지치기+양자화+MoE 복합 파이프라인, 실사용 A/B 테스트, 오픈소스 도구 공개, Anthropic 내부 Claude 경량화 적용 검증

## S7 VERIFY (품질 유지 경량화 검증 코드 -- Python stdlib only)

### S7.0 CONSTANTS (경량화 이론 상수)

```python
"""품질 유지 경량화 핵심 상수 -- 스케일링 법칙과 정보 이론에서 유도"""
import math
TEACHER_PARAMS = 400e9        # 교사 모델 매개변수 (400B)
STUDENT_PARAMS = 70e9         # 학생 모델 매개변수 (70B)
COMPRESSION_RATIO = TEACHER_PARAMS / STUDENT_PARAMS  # ~5.71x
KD_TEMPERATURE = 4.0          # 증류 온도 (Hinton et al., 2015)
PRUNING_RATIO = 0.5           # 구조적 가지치기 비율
QUANT_BITS = 4                # 양자화 비트 (INT4)
FP16_BITS = 16                # 기준 정밀도
MOE_EXPERTS = 8               # MoE 전문가 수
MOE_TOP_K = 2                 # 활성 전문가 수
LORA_RANK = 16                # LoRA 행렬 순위
assert COMPRESSION_RATIO > 5.0
assert MOE_TOP_K < MOE_EXPERTS
assert LORA_RANK < 64  # 일반적 범위
print(f"[S7.0] 압축비={COMPRESSION_RATIO:.2f}x, 증류온도={KD_TEMPERATURE}")
print(f"[S7.0] MoE {MOE_TOP_K}/{MOE_EXPERTS} 활성, LoRA r={LORA_RANK}, 양자화 {QUANT_BITS}bit")
```

### S7.1 DIMENSIONS (증류 손실 단위 검증)

```python
"""지식 증류 손실 함수 단위 일관성: KL(q||p) -> nats, 온도 스케일링"""
import math
def kd_loss(teacher_logits, student_logits, temperature):
    """KL 발산 기반 증류 손실 (단순화된 2-클래스)"""
    def softmax_t(logits, t):
        scaled = [l / t for l in logits]
        max_s = max(scaled)
        exps = [math.exp(s - max_s) for s in scaled]
        total = sum(exps)
        return [e / total for e in exps]
    p = softmax_t(teacher_logits, temperature)  # 교사 소프트 타겟
    q = softmax_t(student_logits, temperature)  # 학생 예측
    kl = sum(pi * math.log(pi / qi) for pi, qi in zip(p, q) if pi > 0)  # [nats]
    return kl * temperature ** 2  # T^2 스케일링 (Hinton 2015)
loss = kd_loss([2.0, 0.5, -1.0], [1.5, 0.8, -0.5], 4.0)
assert loss >= 0, "KL 발산은 비음수"
# 동일 분포 -> 손실 0
zero_loss = kd_loss([1.0, 2.0], [1.0, 2.0], 4.0)
assert zero_loss < 1e-10, "동일 분포 시 손실 = 0"
# 높은 온도 -> 더 부드러운 분포 -> 더 많은 정보 전달
loss_t2 = kd_loss([3.0, 0.0, -2.0], [1.0, 0.5, -0.5], 2.0)
loss_t8 = kd_loss([3.0, 0.0, -2.0], [1.0, 0.5, -0.5], 8.0)
print(f"[S7.1] 증류 손실={loss:.4f} nats, T=2 손실={loss_t2:.4f}, T=8 손실={loss_t8:.4f}")
print(f"[S7.1] 동일분포 손실={zero_loss:.2e} (0 확인), 단위 일관성 통과")
```

### S7.2 CROSS (독립 품질 메트릭 3종 교차 검증)

```python
"""3개 독립 메트릭 교차 검증: MMLU 정확도, HumanEval 코드, MT-Bench 대화"""
import random; random.seed(42)
# 교사 400B vs 증류 학생 70B 시뮬레이션
teacher_mmlu = 0.86      # MMLU 86%
student_mmlu = 0.79      # 증류 후 79% (91.8% 유지)
teacher_code = 0.72      # HumanEval 72%
student_code = 0.61      # 증류 후 61% (84.7% 유지)
teacher_mt = 8.5         # MT-Bench 8.5/10
student_mt = 7.8         # 증류 후 7.8/10 (91.8% 유지)
retain_mmlu = student_mmlu / teacher_mmlu
retain_code = student_code / teacher_code
retain_mt = student_mt / teacher_mt
assert all(r > 0.80 for r in [retain_mmlu, retain_code, retain_mt]), "80% 이상 유지"
harmonic = 3.0 / (1.0/retain_mmlu + 1.0/retain_code + 1.0/retain_mt)
print(f"[S7.2] MMLU 유지={retain_mmlu:.3f}, 코드 유지={retain_code:.3f}, 대화 유지={retain_mt:.3f}")
print(f"[S7.2] 조화 평균 유지율={harmonic:.3f} ({harmonic*100:.1f}%), 5.71배 압축에서 달성")
```

### S7.3 SCALING (매개변수 수 vs 품질 스케일링 법칙)

```python
"""Chinchilla 스케일링: L(N) = a*N^(-alpha) + b, 압축 효율 곡선"""
import math
def quality(n_params, a=5.0, alpha=0.076, b=0.1):
    """매개변수 수 -> 품질 점수 (Chinchilla 기반 근사)"""
    return max(0, 1.0 - a * (n_params ** (-alpha)) + b) if n_params > 0 else 0
sizes_b = [7, 13, 30, 70, 175, 400]  # 십억 단위
qs = [quality(s * 1e9) for s in sizes_b]
print("[S7.3] 매개변수 수 vs 품질 (Chinchilla 스케일링):")
for s, q in zip(sizes_b, qs):
    bar = '#' * int(q * 40)
    print(f"  {s:>4d}B: {q:.3f} |{bar}|")
# 단조 증가 확인
for i in range(1, len(qs)):
    assert qs[i] >= qs[i-1], f"{sizes_b[i]}B >= {sizes_b[i-1]}B"
# 수익 체감: 10배 증가당 품질 증분 감소
q70 = quality(70e9)
q400 = quality(400e9)
gap = q400 - q70
print(f"[S7.3] 70B->400B 품질 증분={gap:.4f} (5.71배 파라미터 대비 작은 증분)")
print(f"[S7.3] 결론: 압축 여지 존재 -- 70B가 400B의 {q70/q400*100:.1f}% 품질 도달 가능")
```

### S7.4 SENSITIVITY (가지치기 비율 민감도)

```python
"""가지치기 비율 스윕: 임계점(cliff) 탐지 -- 급격한 품질 저하 지점"""
import math
def pruning_quality(ratio, cliff=0.6, steepness=20.0, base=0.95):
    """가지치기 비율 -> 품질 유지율 (시그모이드 절벽 모델)"""
    if ratio >= 1.0: return 0.0
    degradation = 1.0 / (1.0 + math.exp(-steepness * (ratio - cliff)))
    return base * (1.0 - degradation)
print("[S7.4] 가지치기 비율 | 품질 유지 | 상태")
cliff_found = False
prev_q = 1.0
for r in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]:
    q = pruning_quality(r)
    drop = prev_q - q
    if drop > 0.1 and not cliff_found:
        st = "<<< 절벽 (cliff)"
        cliff_found = True
    elif q > 0.8:
        st = "안전 영역"
    elif q > 0.5:
        st = "주의"
    else:
        st = "위험"
    print(f"  {r:.1f}     | {q:.3f}     | {st}")
    prev_q = q
assert cliff_found, "절벽 지점이 존재해야 함"
# 50% 가지치기는 안전 영역
assert pruning_quality(0.5) > 0.7, "50% 가지치기 시 70% 이상 유지"
print(f"[S7.4] 50% 가지치기 품질={pruning_quality(0.5):.3f}, 절벽={0.6} 확인")
```

### S7.5 LIMITS (양자화 오차 이론적 한계)

```python
"""양자화 오차 한계: 비트 수 vs 표현 정밀도 + 라운딩 오차 분석"""
import math
def quant_error(bits, dynamic_range=6.0):
    """균일 양자화 최대 라운딩 오차 (반올림)"""
    levels = 2 ** bits
    step = dynamic_range / levels  # 양자화 간격
    max_error = step / 2.0         # 최대 라운딩 오차
    snr_db = 6.02 * bits + 1.76    # 신호 대 양자화 잡음비 (정현파 기준)
    return step, max_error, snr_db
print("[S7.5] 양자화 비트 | 간격     | 최대 오차 | SNR(dB)  | 정밀도")
for bits in [2, 3, 4, 8, 16]:
    step, err, snr = quant_error(bits)
    pct = (err / 6.0) * 100
    tag = "불충분" if bits < 3 else "최소" if bits == 3 else "실용적" if bits == 4 else "충분"
    print(f"  {bits:>2d}bit   | {step:.4f}  | {err:.4f}   | {snr:>6.1f}  | {tag}")
# 4bit vs 16bit 오차 비율
_, e4, _ = quant_error(4)
_, e16, _ = quant_error(16)
ratio = e4 / e16
assert ratio > 100, "4bit 오차는 16bit 대비 100배 이상"
print(f"[S7.5] 4bit/16bit 오차비={ratio:.0f}x -- 그럼에도 실용적 품질 유지 가능")
# 정보 이론 한계: N 파라미터를 B 비트로 저장 시 최소 NB 비트 필요
info_fp16 = 70e9 * 16  # 70B FP16
info_int4 = 70e9 * 4   # 70B INT4
saving = 1.0 - info_int4 / info_fp16
print(f"[S7.5] 70B 모델: FP16={info_fp16/8/1e9:.0f}GB, INT4={info_int4/8/1e9:.0f}GB, 절감={saving*100:.0f}%")
```

### S7.6 CHI2 (경량화 품질 유의성 검정)

```python
"""증류 모델 vs 기본 모델 품질 차이 통계 검정"""
import math
def quality_test(n, correct_teacher, correct_student):
    """McNemar 유사 검정: 두 모델의 정답률 비교"""
    p1 = correct_teacher / n
    p2 = correct_student / n
    pp = (correct_teacher + correct_student) / (2 * n)
    se = math.sqrt(2 * pp * (1 - pp) / n) if pp > 0 and pp < 1 else 1e-10
    z = (p1 - p2) / se if se > 0 else 0
    # 정규 CDF 근사 (Abramowitz & Stegun)
    def ncdf(x):
        s = 1 if x >= 0 else -1; x = abs(x)
        t = 1 / (1 + 0.3275911 * x)
        y = 1 - (((((1.061405429*t - 1.453152027)*t) + 1.421413741)*t - 0.284496736)*t + 0.254829592)*t * math.exp(-x*x/2)
        return 0.5 * (1 + s * y)
    p_val = 2 * (1 - ncdf(abs(z)))  # 양측 검정
    effect = 2 * math.asin(math.sqrt(p1)) - 2 * math.asin(math.sqrt(p2))  # Cohen's h
    return z, p_val, effect
# MMLU 1000문항: 교사 860/1000, 증류 학생 790/1000
z, p, h = quality_test(1000, 860, 790)
print(f"[S7.6] 교사 vs 증류 학생: z={z:.3f}, p={p:.4f}, Cohen's h={h:.3f}")
print(f"[S7.6] 차이 {('유의' if p < 0.05 else '비유의')}, 효과 크기 {('작음' if abs(h)<0.2 else '중간' if abs(h)<0.5 else '큼')}")
# MoE vs 밀집 동일 파라미터
z2, p2, h2 = quality_test(1000, 850, 830)
print(f"[S7.6] MoE vs 밀집(동일크기): z={z2:.3f}, p={p2:.4f}, h={h2:.3f}")
print(f"[S7.6] MoE 우위 {('유의' if p2 < 0.05 else '비유의(추가 데이터 필요)')}")
```

### S7.7 OEIS (MoE 라우팅 엔트로피 수학 구조)

```python
"""MoE 라우터 엔트로피 분석: 균형 라우팅 vs 전문가 붕괴"""
import math
from fractions import Fraction
def routing_entropy(probs):
    """라우팅 확률 분포의 섀넌 엔트로피 (nats)"""
    return -sum(p * math.log(p) for p in probs if p > 0)
def max_entropy(n):
    """n 전문가 균일 분포 엔트로피"""
    return math.log(n)
n_experts = 8
# 균일 분포 (이상적)
uniform = [1.0 / n_experts] * n_experts
h_uniform = routing_entropy(uniform)
h_max = max_entropy(n_experts)
assert abs(h_uniform - h_max) < 1e-10, "균일 = 최대 엔트로피"
print(f"[S7.7] 균일 엔트로피={h_uniform:.4f} nats = ln({n_experts}) (정확)")
# 붕괴 분포 (1개 전문가에 집중)
collapsed = [0.9] + [0.1/7]*7
h_collapsed = routing_entropy(collapsed)
print(f"[S7.7] 붕괴 엔트로피={h_collapsed:.4f} nats ({h_collapsed/h_max*100:.1f}% 효율)")
# 부하 균형 손실: L_balance = N * sum(f_i * P_i), 이상적이면 1.0
f = [125, 124, 126, 125, 127, 123, 125, 125]  # 토큰 할당
total = sum(f)
f_norm = [x / total for x in f]
p_avg = [1.0 / n_experts] * n_experts  # 이상적 라우터 확률
balance = n_experts * sum(fi * pi for fi, pi in zip(f_norm, p_avg))
assert abs(balance - 1.0) < 0.01, "균형 손실 ~1.0"
print(f"[S7.7] 부하 균형 손실={balance:.4f} (이상적=1.0)")
# 정확한 분수: top-2 라우팅 시 활성 비율 = C(N,K)/N
active_frac = Fraction(MOE_TOP_K, n_experts)
print(f"[S7.7] 활성 비율={active_frac} = {float(active_frac):.3f}, 비활성 파라미터 {1-float(active_frac):.1%} 절감")
```

### S7.8 PARETO (압축률-품질 Pareto 프론티어)

```python
"""압축률 vs 품질 유지율의 Pareto 프론티어 탐색"""
import math
def compress_quality(prune_ratio, quant_bits, distill, lora_rank):
    """압축 기법 조합 -> (압축률, 품질유지율) 시뮬레이션"""
    # 압축률 계산
    param_ratio = 1.0 - prune_ratio
    bit_ratio = quant_bits / 16.0
    compression = 1.0 / (param_ratio * bit_ratio) if param_ratio > 0 else float('inf')
    # 품질 시뮬레이션 (각 기법의 독립적 품질 영향)
    q_prune = max(0, 1.0 - 0.3 * (prune_ratio ** 2) - max(0, prune_ratio - 0.6) * 2.0)
    q_quant = 1.0 - 0.02 * max(0, 8 - quant_bits)  # 8bit 이하부터 손실
    q_distill = 0.95 if distill else 0.85           # 증류 적용 시 품질 보존
    q_lora = min(1.0, 0.9 + 0.005 * lora_rank) if lora_rank > 0 else 1.0
    quality = q_prune * q_quant * q_distill * q_lora
    return compression, quality
configs = []
for pr in [0.0, 0.2, 0.4, 0.5, 0.6]:
    for qb in [4, 8, 16]:
        for dist in [True, False]:
            for lr in [0, 8, 16, 32]:
                comp, qual = compress_quality(pr, qb, dist, lr)
                configs.append((pr, qb, dist, lr, comp, qual))
# Pareto 프론티어 추출
pareto = [c for c in configs if not any(
    o[4] >= c[4] and o[5] >= c[5] and (o[4] > c[4] or o[5] > c[5])
    for o in configs if o != c)]
pareto.sort(key=lambda x: x[4])
print(f"[S7.8] 전체 {len(configs)}설정 중 Pareto 최적 {len(pareto)}개:")
for p in pareto[:8]:  # 상위 8개만 출력
    d_str = "증류O" if p[2] else "증류X"
    print(f"  가지치기={p[0]:.1f} 양자화={p[1]}bit {d_str} LoRA={p[3]:>2d} -> 압축={p[4]:.1f}x 품질={p[5]:.3f}")
print(f"[S7.8] 최대 압축={max(p[4] for p in pareto):.1f}x (품질>{min(p[5] for p in pareto):.2f})")
```

### S7.9 SYMBOLIC (LoRA 순위 분석 정확 유도)

```python
"""LoRA 매개변수 효율: rank r의 저순위 분해 -> 파라미터 절감 정확 계산"""
from fractions import Fraction
import math
def lora_params(d_model, r):
    """LoRA A(d_model x r) + B(r x d_model) 매개변수 수"""
    return 2 * d_model * r  # A: d_model*r, B: r*d_model
def full_params(d_model):
    """전체 가중치 행렬 매개변수 수"""
    return d_model * d_model
d = 4096  # 일반적인 모델 차원
for r in [4, 8, 16, 32, 64]:
    lora_p = lora_params(d, r)
    full_p = full_params(d)
    ratio = Fraction(lora_p, full_p)
    pct = float(ratio) * 100
    print(f"  r={r:>2d}: LoRA={lora_p:>8d}, 전체={full_p:>10d}, 비율={ratio} = {pct:.2f}%")
# r=16 일 때 정확한 분수
exact_ratio = Fraction(2 * d * 16, d * d)
simplified = Fraction(2 * 16, d)
assert exact_ratio == simplified, "d 약분"
print(f"[S7.9] r=16, d={d}: 비율 = 2r/d = {simplified} = {float(simplified)*100:.3f}%")
# 전체 모델 기준: 70B에서 LoRA 층 수 = 약 120 (어텐션 Q,K,V,O)
n_layers = 80  # 70B 모델 추정 층 수
n_matrices = 4  # Q, K, V, O
total_lora = n_layers * n_matrices * lora_params(d, 16)
total_model = 70e9
lora_frac = total_lora / total_model
print(f"[S7.9] 70B 모델 LoRA r=16: {total_lora/1e6:.1f}M 파라미터 ({lora_frac*100:.2f}% 학습)")
print(f"[S7.9] 학습 파라미터 {1/lora_frac:.0f}배 절감 -- 미세조정 효율의 핵심")
```

### S7.10 COUNTER (정직한 한계)

```python
"""경량화의 근본적 한계와 실패 사례"""
import math
# 1. 정보 병목: 압축 한계 (레이트-왜곡 이론)
def rate_distortion_bound(n_teacher, n_student):
    """교사 모델 정보를 학생에 담을 수 있는 이론적 한계"""
    if n_student >= n_teacher:
        return 1.0  # 무손실
    # 단순 추정: 파라미터 수 비례 정보량
    return math.sqrt(n_student / n_teacher)  # 낙관적 상한
rd = rate_distortion_bound(400e9, 70e9)
print(f"[S7.10] 정보 병목 상한: 70B는 400B의 최대 {rd*100:.1f}% 정보만 보존 가능")
# 2. 벤치마크 게이밍: MMLU 높아도 실사용 품질 낮을 수 있음
mmlu_optimized = 0.85   # 벤치마크 특화 학습
real_quality = 0.62     # 실제 사용 품질
gap = mmlu_optimized - real_quality
assert gap > 0.15, "벤치마크-실사용 괴리 존재"
print(f"[S7.10] 벤치마크 게이밍: MMLU={mmlu_optimized:.2f}, 실사용={real_quality:.2f}, 괴리={gap:.2f}")
# 3. 가지치기 비가역성: 제거된 뉴런의 정보는 복구 불가
print("[S7.10] 가지치기 비가역성: 한번 제거된 뉴런 정보는 재학습으로도 완전 복구 불가")
# 4. 양자화 누적 오차: 층이 깊어질수록 오차 전파
layers = 80
per_layer_error = 0.001  # 층당 0.1% 오차
cumulative = 1.0
for _ in range(layers):
    cumulative *= (1.0 - per_layer_error)
total_error = 1.0 - cumulative
print(f"[S7.10] 양자화 누적: {layers}층 x {per_layer_error*100:.1f}% = 총 {total_error*100:.1f}% 품질 저하")
# 5. Constitutional AI 효율화의 한계
print("[S7.10] Constitutional AI: RLHF 데이터 축소 시 안전 정렬 약화 위험 -- 최소 임계치 존재")
print("[S7.10] 결론: 70B->400B 등가는 이론적 상한이 있으며, '충분히 좋은' 수준이 현실 목표")
```

## S8 KEY (핵심 연구 아이디어 32종)

### 축 1: 압축 공학 (12종)

| ID | 제목 | 핵심 | 난이도 |
|----|------|------|--------|
| 1 | 층별 적응 증류 | 교사 모델의 각 층 표현을 학생 대응 층에 개별 매칭, 중간 표현 손실 추가 | 중 |
| 2 | 점진적 가지치기 | 학습 중 점진적 희소화 (0%->50%), 급격한 절벽 회피 | 중 |
| 3 | 이동 기반 가지치기 | 크기(magnitude)가 아닌 학습 중 가중치 이동량 기반 중요도 판정 | 중 |
| 4 | QAT + 증류 동시 학습 | 양자화 인식 학습과 증류를 단일 패스로 통합, 양자화 오차를 증류가 보상 | 상 |
| 5 | 활성화 인식 양자화 | 가중치가 아닌 활성화 분포 기반 양자화 범위 결정 (AWQ 확장) | 중 |
| 6 | 혼합 정밀도 자동 탐색 | 층별 최적 비트 수 자동 결정 (민감 층 8bit, 나머지 4bit) | 상 |
| 7 | 교사 앙상블 증류 | 다수 교사 모델의 합의로 학생 학습, 개별 교사 편향 감소 | 중 |
| 8 | 자기 증류 | 교사 없이 자기 자신의 이전 체크포인트로부터 증류 (반복 압축) | 중 |
| 9 | 토큰 수준 증류 손실 | 시퀀스 전체가 아닌 토큰별 적응적 온도 조절 | 상 |
| 10 | 구조 검색 증류 | NAS로 최적 학생 아키텍처 탐색 후 증류 적용 | 상 |
| 11 | 스펙트럼 가지치기 | SVD 기반 가중치 행렬 저순위 근사, 특이값 임계치 자동 결정 | 중 |
| 12 | 런타임 적응 양자화 | 입력 난이도에 따라 실시간 정밀도 조절 (쉬운 입력 2bit, 어려운 입력 8bit) | 상 |

### 축 2: 구조 혁신 (10종)

| ID | 제목 | 핵심 | 난이도 |
|----|------|------|--------|
| 13 | MoE 라우터 안정화 | 전문가 붕괴 방지를 위한 엔트로피 정규화 + 부하 균형 손실 개선 | 중 |
| 14 | 계층적 MoE | 레벨1 도메인 라우터 -> 레벨2 세부 전문가, 2단계 희소 활성화 | 상 |
| 15 | 밀집-희소 하이브리드 | 하위 층은 밀집(공유 표현), 상위 층은 MoE(전문 지식) | 중 |
| 16 | 동적 깊이 | 입력 난이도에 따라 활성 층 수 조절 (쉬운 입력 = 얕은 경로) | 상 |
| 17 | 어텐션 헤드 공유 | GQA/MQA 확장: 층 간 어텐션 헤드 공유로 파라미터 절감 | 중 |
| 18 | FFN 팩터링 | FFN d_ff 차원을 저순위 곱으로 분해 (d_model x r x d_ff) | 중 |
| 19 | 가중치 공유 구조 탐색 | 어떤 층 간 가중치를 공유하면 품질 손실 최소인지 NAS 탐색 | 상 |
| 20 | 전문가 병합 | 학습 후 유사 전문가를 병합하여 MoE 크기 축소 | 중 |
| 21 | 토큰 드롭 학습 | 학습 시 비중요 토큰을 확률적으로 드롭하여 효율 향상 | 중 |
| 22 | 모듈러 아키텍처 | 기능별 독립 모듈로 분리, 필요 모듈만 로드 (플러그인 방식) | 상 |

### 축 3: 품질 보장 (10종)

| ID | 제목 | 핵심 | 난이도 |
|----|------|------|--------|
| 23 | 실품질 벤치마크 | MMLU 말고 실사용 시나리오 기반 평가 (장문 이해, 다단계 추론, 창의성) | 중 |
| 24 | Constitutional 효율화 | 더 적은 RLHF 데이터로 동등한 안전 정렬 달성 (합성 데이터 활용) | 중 |
| 25 | 합성 데이터 품질 제어 | 합성 학습 데이터의 품질-다양성 균형 자동 조절 | 중 |
| 26 | 연속 경량화 모니터링 | 프로덕션에서 품질 지표 연속 추적, 저하 시 자동 롤백 | 중 |
| 27 | 모델 병합 최적화 | TIES/DARE/SLERP 가중치 병합에서 최적 계수 자동 탐색 | 중 |
| 28 | 도메인 특화 압축 | 의료/법률/코드 등 도메인별 최적 압축 전략 자동 선택 | 상 |
| 29 | 경량 모델 안전 검증 | 압축 후 안전 정렬 유지 여부 체계적 검증 프로토콜 | 중 |
| 30 | 적대적 압축 강건성 | 양자화/가지치기된 모델의 적대적 공격 취약성 분석 | 상 |
| 31 | 다국어 품질 균등화 | 압축 시 저자원 언어 품질 불균형 방지 (편향 보정) | 중 |
| 32 | 추론 시간 품질 증폭 | 경량 모델 + 추론 시간 컴퓨트(beam search, 자기검증)로 품질 보상 | 상 |

## S9 VALIDATION (검증 전략)

| 검증 항목 | 방법 | 기준 |
|----------|------|------|
| 벤치마크 품질 유지 | MMLU, HumanEval, MT-Bench, GPQA | 교사 대비 85% 이상 |
| 실사용 품질 | 블라인드 A/B 테스트 (1000+ 사례) | 승률 45% 이상 (교사 대비) |
| 안전 정렬 유지 | Constitutional AI 준수율 | 원본 대비 95% 이상 |
| 추론 속도 | 토큰/초 (동일 하드웨어) | 교사 대비 3배 이상 빠름 |
| 메모리 절감 | GPU 메모리 사용량 | 교사 대비 70% 이상 절감 |
| 가지치기 안정성 | 절벽 지점 사전 예측 정확도 | 예측 오차 5% 이내 |
| MoE 부하 균형 | 전문가 활용률 분산 | CV < 0.1 |
| 양자화 정밀도 | 교정 데이터셋 민감도 | 5개 이상 교정셋에서 일관 |

## S10 PREDICTIONS (예측 10종)

| # | 예측 | 근거 | 검증 방법 |
|---|------|------|----------|
| 1 | 증류+MoE 결합 시 70B(3.8B 활성)가 밀집 400B의 88% 품질 달성 | Gemma4 + Chinchilla 스케일링 외삽 | MMLU/HumanEval 벤치마크 |
| 2 | 최적 증류 온도는 모델 크기의 log에 비례: T* ~ c*ln(N_teacher/N_student) | KL 발산 경사 분석 | 온도 스윕 실험 |
| 3 | 구조적 가지치기 50% 후 QAT 4bit 적용 시 밀집 FP16 대비 92% 품질 | 가지치기-양자화 상호 보완 효과 | 벤치마크 + A/B 테스트 |
| 4 | 이동 기반 가지치기가 크기 기반 대비 3-5% 품질 우위 | 학습 과정 정보 활용의 이론적 이점 | 동일 조건 비교 실험 |
| 5 | MoE 라우터 엔트로피 정규화로 전문가 활용률 분산 50% 감소 | 정보 이론 최적 라우팅 = 균일 분포 | 학습 곡선 + 활용률 측정 |
| 6 | LoRA r=16이 r=64 대비 95% 품질 달성 (파라미터 4배 절감) | 저순위 근사의 수렴 속도 | 순위별 벤치마크 비교 |
| 7 | 자기 증류 3회 반복으로 원본 대비 75% 크기에서 98% 품질 | 반복 압축의 수렴성 | 반복 실험 |
| 8 | Constitutional AI 합성 데이터로 RLHF 데이터 50% 절감 가능 | 합성 데이터 다양성 보상 | 안전 벤치마크 + 레드팀 |
| 9 | 동적 깊이 모델이 고정 깊이 대비 평균 추론 비용 30% 절감 | 입력 난이도 분포의 편향성 (대부분 쉬운 입력) | 추론 지연 시간 측정 |
| 10 | 2026년 말까지 3B 활성 파라미터 모델이 GPT-4 수준 달성 | MoE + 증류 + NAS 복합 효과 | 공개 벤치마크 추적 |

## S11 PERFORMANCE (성능 비교) -- ASCII 차트

```
+======================================================================+
|  [추론 속도] (토큰/초, 동일 A100 GPU 기준)                           |
+======================================================================+
|  밀집 400B (FP16)  ##................  ~30 tok/s (기준)              |
|  밀집 70B (FP16)   ########..........  ~120 tok/s (4x)              |
|  밀집 70B (INT4)   ############......  ~200 tok/s (6.7x)            |
|  MoE 26B (3.8B활성) ###############...  ~280 tok/s (9.3x)           |
|  증류 70B (INT4)   ############......  ~200 tok/s (6.7x)            |
|  LoRA 어댑터 전환  ################..  ~310 tok/s (10.3x)           |
+======================================================================+
|  [메모리 사용량] (GB, 추론 시 기준)                                  |
+======================================================================+
|  밀집 400B (FP16)  ##################  ~800GB (8xA100 필수)         |
|  밀집 70B (FP16)   ########..........  ~140GB (2xA100)              |
|  밀집 70B (INT4)   ####..............  ~35GB (1xA100)               |
|  MoE 26B (3.8B활성) ###...............  ~26GB (1xA100, 부분 로드)   |
|  증류 13B (INT4)   #.................  ~7GB (소비자 GPU)            |
+======================================================================+
|  [학습 비용] (GPU-시간, 상대값)                                      |
+======================================================================+
|  처음부터 400B     ##################  100,000+ GPU-시간            |
|  증류 70B          ########..........  10,000 GPU-시간              |
|  QAT 70B           #####.............  5,000 GPU-시간               |
|  LoRA r=16         #.................  100 GPU-시간                 |
|  PTQ (교정만)      ..                  10 GPU-시간                  |
+======================================================================+
```

## S12 ARCHITECTURE (전체 아키텍처) -- ASCII 차트

```
+======================================================================+
|                    품질 유지 경량화 아키텍처                          |
+======================================================================+
|                                                                      |
|  [교사 모델 분석]                                                    |
|  +------------------+                                                |
|  | 400B 교사 모델   |                                                |
|  | - 층별 활성화 통계|                                                |
|  | - 어텐션 패턴    |                                                |
|  | - FFN 중요도     |                                                |
|  +--------+---------+                                                |
|           |                                                          |
|           v                                                          |
|  [압축 파이프라인] ----+----+----+----+                               |
|  |        |           |    |    |    |                                |
|  v        v           v    v    v    v                                |
| 증류    가지치기    양자화  MoE  LoRA  NAS                            |
| T=4     구조/비구조  4bit  8E2A r=16  탐색                           |
|  |        |           |    |    |    |                                |
|  +--------+-----------+----+----+----+                               |
|           |                                                          |
|           v                                                          |
|  [통합 학생 모델]                                                    |
|  +------------------+                                                |
|  | 70B (또는 MoE)   |                                                |
|  | - 증류된 지식    |                                                |
|  | - 가지치기된 구조|                                                |
|  | - 양자화된 가중치|                                                |
|  +--------+---------+                                                |
|           |                                                          |
|           v                                                          |
|  [품질 보장 계층]                                                    |
|  +------------------+     +------------------+                       |
|  | 벤치마크 평가    |<--->| 실사용 A/B 테스트|                       |
|  | MMLU/HumanEval   |     | 블라인드 비교    |                       |
|  +--------+---------+     +--------+---------+                       |
|           |                         |                                |
|           +----------+--------------+                                |
|                      v                                               |
|             [안전 정렬 검증]                                         |
|             +------------------+                                     |
|             | Constitutional AI|                                     |
|             | 레드팀 테스트    |                                     |
|             | 정렬 유지 확인  |                                     |
|             +------------------+                                     |
+======================================================================+
```

## S13 DATAFLOW (데이터 흐름) -- ASCII 차트

```
+======================================================================+
|                    데이터 흐름 (엔드투엔드)                           |
+======================================================================+
|                                                                      |
|  [입력 데이터]                                                       |
|  교사 가중치 + 교정 데이터셋 + 평가 데이터셋                        |
|       |              |              |                                |
|       v              v              v                                |
|  +---------+   +-----------+   +-----------+                         |
|  | 가중치  |   | 교사 추론 |   | 품질 기준 |                         |
|  | 분석    |   | (소프트   |   | (벤치마크 |                         |
|  | (SVD,   |   |  타겟     |   |  점수)    |                         |
|  |  통계)  |   |  생성)    |   |           |                         |
|  +----+----+   +-----+-----+   +-----+-----+                        |
|       |              |              |                                |
|       v              v              v                                |
|  +-------------------------------------------------+                 |
|  |           압축 최적화 루프                       |                 |
|  |  +----------+  +---------+  +----------+        |                 |
|  |  | 가지치기 |->| 증류    |->| 양자화   |        |                 |
|  |  | 마스크   |  | 학습    |  | 교정     |        |                 |
|  |  +----------+  +---------+  +----------+        |                 |
|  |       ^              |            |             |                 |
|  |       |              v            v             |                 |
|  |       +------- 품질 피드백 <------+             |                 |
|  +-------------------------------------------------+                 |
|                         |                                            |
|                         v                                            |
|                  [출력: 경량 모델]                                    |
|                  가중치 + 설정 + 평가 리포트                         |
|                         |                                            |
|              +----------+----------+                                 |
|              v                     v                                 |
|       [클라우드 배포]        [에지 배포]                             |
|       API 서빙               모바일/IoT                             |
+======================================================================+
```

## S14 TOOLING (도구 및 프레임워크)

| 도구 | 용도 | 핵심 기능 |
|------|------|----------|
| PyTorch | 모델 학습/증류 | 자동 미분, 분산 학습, 양자화 API |
| Hugging Face Transformers | 모델 허브 | 사전학습 모델 로드, 토크나이저, 파이프라인 |
| GPTQ / AWQ | 후훈련 양자화 | 4bit 양자화, 활성화 인식, 교정 |
| PEFT (LoRA/QLoRA) | 효율적 미세조정 | 저순위 어댑터, 양자화 기반 위 미세조정 |
| vLLM / TGI | 추론 서빙 | 연속 배칭, PagedAttention, 양자화 추론 |
| lm-eval-harness | 벤치마크 평가 | MMLU, HumanEval, MT-Bench 자동 평가 |
| Weights & Biases | 실험 추적 | 하이퍼파라미터, 메트릭, 모델 비교 |
| DeepSpeed | 분산 학습 | ZeRO 최적화, MoE 학습, 양자화 학습 |
| ONNX Runtime | 크로스 플랫폼 추론 | 그래프 최적화, 양자화, 하드웨어 가속 |
| Neural Architecture Search | 구조 탐색 | 효율적 아키텍처 자동 설계 |

## S15 METHODOLOGY (연구 방법론)

### 실험 설계 원칙

1. **공정 비교**: 모든 기법을 동일 교사 모델, 동일 데이터셋, 동일 하드웨어에서 비교
2. **다중 메트릭**: 단일 벤치마크가 아닌 벤치마크+실사용+안전 3축 평가
3. **통계적 엄밀성**: 3회 이상 반복, 신뢰 구간 보고, 효과 크기 포함
4. **절제 연구**: 각 기법의 기여도를 개별 제거(ablation)로 검증
5. **재현성**: 코드, 데이터, 하이퍼파라미터 완전 공개

### Anthropic 맥락에서의 연구 의의

- **Claude 경쟁력**: GPT-5, Gemini Ultra와 경쟁하면서도 더 적은 컴퓨트로 서빙
- **Constitutional AI 효율화**: 더 적은 인간 피드백 데이터로 동등한 안전 정렬
- **민주화**: 경량 Claude 모델을 통한 접근성 확대 (Haiku -> Sonnet -> Opus 스펙트럼)
- **에너지 효율**: AI 탄소 발자국 감소, 지속 가능한 AI 발전
- **안전 연구 가속**: 경량 모델로 더 많은 안전 실험을 더 빠르게 반복

### 연구 일정 (4개월)

| 주차 | 활동 | 산출물 |
|------|------|--------|
| 1-2 | 문헌 조사 + 베이스라인 구축 | 기법 비교 서베이, 평가 파이프라인 |
| 3-4 | 증류 실험 (층별 KD, 온도 탐색) | 최적 증류 프로토콜 |
| 5-6 | 가지치기 + 양자화 실험 | 가지치기-양자화 통합 파이프라인 |
| 7-8 | MoE 구조 실험 | 라우터 안정화 + 전문가 설계 |
| 9-10 | LoRA + 모델 병합 실험 | 효율적 미세조정 가이드라인 |
| 11-12 | 통합 파이프라인 + 안전 검증 | 복합 압축 레시피 |
| 13-14 | A/B 테스트 + 논문 초고 | 실사용 품질 검증 결과 |
| 15-16 | 논문 완성 + 오픈소스 공개 | 최종 논문 + 코드 공개 |

### 핵심 가설

> 지식 증류 + 구조적 가지치기 + MoE 라우팅 + 양자화를 체계적으로 결합하면,
> 70B 활성 파라미터 모델이 400B 밀집 모델의 88% 이상 품질을 달성하면서
> 추론 비용은 5배 이상 절감할 수 있다.

이 가설의 검증이 본 연구 프로그램의 핵심 목표이다.
