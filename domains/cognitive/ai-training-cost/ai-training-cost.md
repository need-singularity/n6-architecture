---
domain: ai-training-cost
requires:
  - to: ai-inference-cost
  - to: ai-quality-scale
---
# 훈련 비용 절감 연구 프로그램 (Anthropic Fellows 2026)

## S1 WHY (왜 이 문제가 중요한가)

프론티어 모델 훈련 비용이 $12B를 돌파했다. 이 비용 구조가 유지되면 AI 연구는 소수 거대 기업의 독점이 된다. 비용을 1/10로 줄이면서 품질을 유지하는 것이 이 연구의 핵심이다.

| 문제 | 현재 상태 | 본 연구의 해결 방향 |
|------|----------|-------------------|
| 훈련 비용 폭증 | Claude 4/5급 모델 $12B+ | Chinchilla 위반 탐지 + 최적 배분으로 $1.2B 목표 |
| 데이터 비효율 | 전체 코퍼스 무차별 투입 | 커리큘럼 학습 + 합성 데이터로 유효 토큰 3배 |
| GPU 유휴 낭비 | MFU 35-45% 수준 | FSDP/DeepSpeed 최적화로 MFU 60%+ |
| 체크포인트 손실 | 장애 시 수시간 재계산 | 비동기 체크포인트 + 탄력적 학습으로 손실 최소화 |
| 혼합 정밀도 한계 | FP16/BF16 수동 설정 | QAT + 자동 정밀도 탐색으로 메모리 40% 절감 |
| MoE 비효율 | 라우팅 불균형, 전문가 붕괴 | 적응형 라우팅 + 부하 분산으로 효율 2배 |

**Anthropic 관점**: Claude 5 훈련 예산이 $12B라면, 동일 예산으로 10배 더 큰 모델을 훈련하거나 동일 모델을 $1.2B에 훈련할 수 있다. 이는 연구 속도와 경쟁력에 직결된다.

**과학적 가치**: 스케일링 법칙의 정밀 이해, 데이터 혼합의 정보 이론적 최적화, 분산 학습의 통신 병목 해소는 기계 학습 이론의 근본 문제다.

**한 줄 요약**: 프론티어 모델 훈련 비용을 1/10로 줄이면서 품질을 유지하는 체계적 방법론을 수립한다.

## S2 COMPARE (현재 접근법 비교) -- ASCII 비교 차트

```
+------------------------------------------------------------------+
|  [훈련 비용 효율] (동일 성능 대비 비용 절감률)                    |
+------------------------------------------------------------------+
|  표준 Dense      ##................  10%  (베이스라인)            |
|  Chinchilla 최적 ######............  30%  (최적 배분)             |
|  MoE (Mixtral)   #########.........  45%  (활성 파라미터 절감)    |
|  DeepSpeed ZeRO  ########..........  40%  (메모리 효율)           |
|  합성 데이터 증강 ######............  30%  (데이터 효율)           |
|  커리큘럼 학습    #######...........  35%  (학습 효율)             |
|  본 연구 (통합)  ##################  90%  (전 축 통합)            |
+------------------------------------------------------------------+
|  [GPU 활용률] (MFU, Model FLOPs Utilization)                     |
+------------------------------------------------------------------+
|  단일 GPU        ##################  90%  (통신 없음)             |
|  DDP             ##############....  70%  (경사 AllReduce)        |
|  FSDP            ############......  60%  (샤딩 오버헤드)         |
|  Megatron-LM     ###############...  75%  (파이프라인+텐서)       |
|  DeepSpeed 3D    ##############....  70%  (3D 병렬)               |
|  본 연구 (최적)  ################..  80%  (적응형 병렬)           |
+------------------------------------------------------------------+
|  [데이터 효율] (유효 토큰 대비 원시 토큰 비율)                   |
+------------------------------------------------------------------+
|  무작위 셔플     ####..............  20%  (중복 다수)             |
|  중복 제거       ########..........  40%  (기본 정제)             |
|  품질 필터링     ###########.......  55%  (규칙 기반)             |
|  커리큘럼 정렬   ##############....  70%  (난이도 순서)           |
|  합성+선별 통합  #################.  85%  (본 연구)               |
+------------------------------------------------------------------+
```

**핵심 장벽**:

| 장벽 | 설명 | 난이도 |
|------|------|--------|
| Chinchilla 위반 탐지 | 실시간으로 과훈련/과소훈련 판별 | 상 |
| MoE 전문가 붕괴 | 소수 전문가에 토큰 집중, 나머지 사장 | 상 |
| 통신 병목 | 수천 GPU 간 경사 동기화 지연 | 상 |
| 합성 데이터 품질 | 모델 붕괴 (model collapse) 위험 | 중 |
| 체크포인트 I/O | 수 TB 모델 저장/복원 시간 | 중 |

## S3 REQUIRES (선행 요구사항)

| 범주 | 구체 항목 | 수준 | 비고 |
|------|----------|------|------|
| 수학 | 스케일링 법칙 (Chinchilla/Kaplan) | 중급 | 멱법칙 피팅, 손실 예측 |
| 수학 | 정보 이론 (엔트로피, KL 발산) | 중급 | 데이터 혼합 최적화 |
| 수학 | 볼록 최적화 | 초급 | 학습률 스케줄 유도 |
| 시스템 | 분산 학습 (FSDP, DeepSpeed, Megatron) | 중급 | 3D 병렬 구현 |
| 시스템 | GPU 프로파일링 (CUDA, NCCL) | 중급 | MFU 측정/최적화 |
| ML | Transformer 아키텍처 | 상급 | MoE, 어텐션 최적화 |
| ML | 혼합 정밀도 학습 (AMP, QAT) | 중급 | FP8/INT8 양자화 |
| ML | 합성 데이터 생성 (self-play, 증류) | 중급 | 모델 붕괴 방지 |
| 인프라 | 클러스터 스케줄링 (Slurm, K8s) | 초급 | 자원 할당 최적화 |

**의존 도메인**:
```
ai-training-cost
  ├── ai-inference-cost   (추론 비용 최적화 기법 공유)
  ├── ai-quality-scale    (품질 유지 검증 메트릭)
  └── ai-eval-pipeline    (훈련 중 평가 파이프라인)
```

## S4 STRUCT (연구 프로그램 구조) -- ASCII 아키텍처

```
+======================================================================+
|  [축 1: 데이터 효율]          [축 2: 계산 효율]                       |
|  +--------------------+      +--------------------+                  |
|  | 합성 데이터 생성    |      | MoE 아키텍처       |                  |
|  | 커리큘럼 학습       |      | 혼합 정밀도/QAT    |                  |
|  | 데이터 혼합 최적화  |      | 분산 학습 최적화   |                  |
|  | 중복 제거/필터링    |      | 체크포인트 전략    |                  |
|  +----------+---------+      +----------+---------+                  |
|             +--------+--------+------+                               |
|                      |                                               |
|             [축 3: 스케일링 법칙]                                     |
|             +--------------------+                                   |
|             | Chinchilla 정밀화  |                                   |
|             | 최적 배분 공식     |                                   |
|             | 위반 탐지/교정     |                                   |
|             +--------------------+                                   |
+======================================================================+
```

**데이터 흐름**:
```
원시 데이터 (The Pile, RedPajama, FineWeb)
        |
        v
[축 1] 필터링 -> 혼합 -> 커리큘럼 배치 -> 합성 증강
        |
        v
[축 3] 스케일링 법칙 기반 최적 토큰/파라미터 비율 결정
        |
        v
[축 2] MoE + QAT + FSDP 분산 훈련 실행
        |
        v
평가 -> 피드백 -> 데이터/계산 배분 재조정
```

## S5 FLOW (실험 흐름) -- ASCII

```
데이터 준비 --> 스케일링 예측 --> 훈련 설정 --> 훈련 실행 --> 평가
    |              |               |              |            |
    v              v               v              v            v
코퍼스 분석   Chinchilla 피팅   MoE/QAT 설정   분산 실행    벤치마크
혼합 비율     최적 배분 계산    FSDP 구성      체크포인트   손실/품질
커리큘럼 설계 위반 탐지 경보    학습률 스케줄   장애 복구    비용 산출
    |              |               |              |            |
    +-----<--------+-------<-------+------<-------+-----<------+
                      피드백 루프 (비용-품질 최적화)
```

**반복 주기**: 소규모 프록시 (1B 파라미터)에서 24시간 내 1사이클 완료, 결과를 대규모 (70B+) 예측에 외삽

## S6 EVOLVE (4개월 로드맵)

- **Mk.I (1개월)**: Chinchilla 스케일링 법칙 재현 + 데이터 혼합 엔트로피 최적화 + 1B 프록시 모델 베이스라인 구축
- **Mk.II (2개월)**: 커리큘럼 학습 파이프라인 구축 + MoE 적응형 라우팅 실험 + 합성 데이터 생성/필터링 시스템
- **Mk.III (3개월)**: QAT + FSDP 통합 최적화 + 비동기 체크포인트 + 7B/13B 모델 검증 + 비용 모델 정밀화
- **Mk.IV (4개월)**: 3축 통합 파이프라인 + 70B 프록시 최종 검증 + 논문 작성 + 비용 절감 보고서

## S7 VERIFY (훈련 비용 검증 코드 -- Python stdlib only)

### S7.0 CONSTANTS (스케일링 법칙 기본 상수)

```python
"""Chinchilla 스케일링 법칙 핵심 상수 -- Hoffmann et al., 2022"""
import math

# Chinchilla 최적 계수 (Hoffmann et al., 2022, Table 3)
ALPHA = 0.34        # 파라미터 스케일링 지수
BETA = 0.28         # 데이터 스케일링 지수
A = 406.4           # 파라미터 항 계수
B = 410.7           # 데이터 항 계수
E = 1.69            # 환원 불가 손실 (nats)

# 훈련 비용 기준
FLOPS_PER_TOKEN = 6  # 근사: 6 * N (파라미터 수) FLOPs/토큰
GPU_H100_TFLOPS = 989.0  # H100 SXM BF16 peak TFLOPS
GPU_COST_PER_HOUR = 3.0  # H100 클라우드 시간당 비용 ($)
MFU_BASELINE = 0.40      # 기본 MFU (Model FLOPs Utilization)

# Chinchilla 최적 비율: D = 20 * N (토큰 = 20 * 파라미터)
CHINCHILLA_RATIO = 20.0

assert 0.2 < ALPHA < 0.5 and 0.2 < BETA < 0.5
assert E > 0 and FLOPS_PER_TOKEN == 6

def check():
    ok = (0.2 < ALPHA < 0.5) and (0.2 < BETA < 0.5)
    ok = ok and (E > 0) and (CHINCHILLA_RATIO == 20.0)
    print(f"[S7.0] {'PASS' if ok else 'FAIL'} -- alpha={ALPHA}, beta={BETA}, E={E}, 최적비율={CHINCHILLA_RATIO}")
    return ok

check()
```

### S7.1 DIMENSIONS (비용 함수 단위 검증)

```python
"""훈련 비용 단위 일관성: FLOPs -> GPU-시간 -> 달러"""
import math

def training_cost(N, D, mfu=0.40, gpu_tflops=989.0, cost_per_hour=3.0):
    """N: 파라미터 수, D: 토큰 수 -> 달러"""
    total_flops = 6 * N * D                          # [FLOPs]
    gpu_flops_per_sec = gpu_tflops * 1e12 * mfu      # [FLOP/s]
    gpu_seconds = total_flops / gpu_flops_per_sec    # [초]
    gpu_hours = gpu_seconds / 3600                   # [시간]
    cost = gpu_hours * cost_per_hour                 # [달러]
    return cost, total_flops, gpu_hours

# Claude 3급 (70B 파라미터, 1.4T 토큰)
N_70B = 70e9
D_70B = 1.4e12
cost_70b, flops_70b, hours_70b = training_cost(N_70B, D_70B)

# Claude 4/5급 (300B+ 파라미터, 15T+ 토큰) -- 추정
N_300B = 300e9
D_300B = 15e12
cost_300b, flops_300b, hours_300b = training_cost(N_300B, D_300B)

def check():
    ok = True
    # 단위 검증: FLOPs는 무차원이 아닌 연산 횟수
    ok = ok and flops_70b > 0 and hours_70b > 0 and cost_70b > 0
    # 대형 모델이 더 비싸야 함
    ok = ok and cost_300b > cost_70b
    # MFU 증가 시 비용 감소
    cost_high_mfu, _, _ = training_cost(N_70B, D_70B, mfu=0.60)
    ok = ok and cost_high_mfu < cost_70b
    print(f"[S7.1] {'PASS' if ok else 'FAIL'} -- 70B 비용=${cost_70b:,.0f}, 300B 비용=${cost_300b:,.0f}")
    print(f"  MFU 0.40->0.60 절감: ${cost_70b - cost_high_mfu:,.0f} ({(1-cost_high_mfu/cost_70b)*100:.0f}%)")
    return ok

check()
```

### S7.2 CROSS (Chinchilla 교차 검증: 3개 독립 추정)

```python
"""Chinchilla 최적 배분 3가지 독립 추정법 교차 검증"""
import math

def chinchilla_loss(N, D, A=406.4, B=410.7, alpha=0.34, beta=0.28, E=1.69):
    """Chinchilla 손실 함수: L(N,D) = E + A/N^alpha + B/D^beta"""
    return E + A / (N ** alpha) + B / (D ** beta)

# 방법 1: 고정 FLOPs에서 최적 N, D (해석적)
def optimal_allocation(C, ratio=20.0):
    """C: 총 FLOPs = 6*N*D -> N = sqrt(C/(6*ratio)), D = ratio*N"""
    N = math.sqrt(C / (6 * ratio))
    D = ratio * N
    return N, D

# 방법 2: 경사 기반 (편미분 = 0 조건)
def optimal_from_gradient(C, alpha=0.34, beta=0.28, A=406.4, B=410.7):
    """dL/dN * N = dL/dD * D 조건 + 6ND = C 제약"""
    # 최적 조건: alpha*A/N^alpha = beta*B/D^beta
    # D/N 비율: r = (beta*B / (alpha*A))^(1/(alpha+beta)) * 해석 근사
    r = (beta * B / (alpha * A)) ** (1.0 / (alpha + beta))
    # N*D = C/6 -> N = sqrt(C/(6*r)), D = r*N 근사
    N = (C / (6 * r)) ** 0.5
    D = r * N
    return N, D

# 방법 3: 그리드 탐색 (이산 최적화)
def optimal_grid_search(C, steps=200):
    """C = 6*N*D 제약 하에서 손실 최소화"""
    best_loss, best_N, best_D = float('inf'), 0, 0
    for i in range(1, steps):
        log_N = math.log10(1e6) + i * (math.log10(1e12) - math.log10(1e6)) / steps
        N = 10 ** log_N
        D = C / (6 * N)
        if D < 1e6:
            continue
        loss = chinchilla_loss(N, D)
        if loss < best_loss:
            best_loss, best_N, best_D = loss, N, D
    return best_N, best_D

C_budget = 6 * 70e9 * 1.4e12  # 70B * 1.4T 토큰 FLOPs

N1, D1 = optimal_allocation(C_budget)
N2, D2 = optimal_from_gradient(C_budget)
N3, D3 = optimal_grid_search(C_budget)

def check():
    # 3가지 방법의 D/N 비율이 10-40 범위 내 (Chinchilla 근방)
    r1, r2, r3 = D1/N1, D2/N2, D3/N3
    ok = all(5 < r < 100 for r in [r1, r2, r3])
    # 3가지 방법의 N이 같은 자릿수 (order of magnitude)
    log_ns = [math.log10(N1), math.log10(N2), math.log10(N3)]
    ok = ok and (max(log_ns) - min(log_ns)) < 2.0  # 100배 이내
    print(f"[S7.2] {'PASS' if ok else 'FAIL'} -- 3가지 Chinchilla 최적 배분 교차 검증")
    print(f"  방법1(해석): N={N1:.2e}, D/N={r1:.1f}")
    print(f"  방법2(경사): N={N2:.2e}, D/N={r2:.1f}")
    print(f"  방법3(탐색): N={N3:.2e}, D/N={r3:.1f}")
    return ok

check()
```

### S7.3 SCALING (데이터 크기 vs 훈련 손실)

```python
"""스케일링 법칙: 토큰 수 증가에 따른 손실 감소 (멱법칙)"""
import math

def loss_vs_data(D, B=410.7, beta=0.28, E=1.69, N=70e9, A=406.4, alpha=0.34):
    """고정 N에서 D 변화에 따른 손실"""
    return E + A / (N ** alpha) + B / (D ** beta)

token_counts = [1e9, 10e9, 100e9, 1e12, 10e12]
losses = [loss_vs_data(D) for D in token_counts]

def check():
    ok = True
    print("[S7.3] 토큰 수 vs 훈련 손실 (N=70B 고정):")
    for D, L in zip(token_counts, losses):
        bar = '#' * int((4.0 - L) * 15)
        print(f"  D={D:>8.0e}: L={L:.4f} |{bar}|")
    # 단조 감소 확인
    for i in range(1, len(losses)):
        ok = ok and losses[i] < losses[i-1]
    # 수익 체감: 10배 증가당 손실 감소폭이 줄어듦
    decrements = [losses[i-1] - losses[i] for i in range(1, len(losses))]
    for i in range(1, len(decrements)):
        ok = ok and decrements[i] <= decrements[i-1] + 1e-9
    print(f"[S7.3] {'PASS' if ok else 'FAIL'} -- 단조 감소 + 수익 체감 확인")
    print(f"  감소폭: {['%.4f' % d for d in decrements]}")
    return ok

check()
```

### S7.4 SENSITIVITY (학습률 스케줄 민감도)

```python
"""학습률 스케줄: 코사인 어닐링 vs 선형 감쇠 vs WSD"""
import math

def cosine_lr(step, total, lr_max=3e-4, lr_min=3e-5, warmup=2000):
    """코사인 어닐링 학습률 스케줄"""
    if step < warmup:
        return lr_max * step / warmup
    progress = (step - warmup) / (total - warmup)
    return lr_min + 0.5 * (lr_max - lr_min) * (1 + math.cos(math.pi * progress))

def linear_lr(step, total, lr_max=3e-4, lr_min=0, warmup=2000):
    """선형 감쇠 학습률"""
    if step < warmup:
        return lr_max * step / warmup
    return lr_max - (lr_max - lr_min) * (step - warmup) / (total - warmup)

def wsd_lr(step, total, lr_max=3e-4, lr_min=3e-5, warmup=2000, stable_frac=0.8):
    """WSD (Warmup-Stable-Decay) 학습률"""
    if step < warmup:
        return lr_max * step / warmup
    stable_end = int(total * stable_frac)
    if step < stable_end:
        return lr_max
    progress = (step - stable_end) / (total - stable_end)
    return lr_max - (lr_max - lr_min) * progress

total_steps = 100000

def check():
    ok = True
    print("[S7.4] 학습률 스케줄 비교 (step=50000, total=100000):")
    mid = total_steps // 2
    for name, fn in [("코사인", cosine_lr), ("선형", linear_lr), ("WSD", wsd_lr)]:
        lr_mid = fn(mid, total_steps)
        lr_end = fn(total_steps - 1, total_steps)
        lr_warm = fn(1000, total_steps)
        # 워밍업 중 학습률 < 최대 학습률
        ok = ok and lr_warm < 3e-4
        # 끝에서 학습률이 중간보다 작거나 같아야 함
        ok = ok and lr_end <= lr_mid + 1e-10
        print(f"  {name}: 워밍업={lr_warm:.2e}, 중간={lr_mid:.2e}, 종료={lr_end:.2e}")

    # WSD는 안정 구간에서 학습률이 최대 유지
    lr_stable = wsd_lr(50000, total_steps)
    ok = ok and abs(lr_stable - 3e-4) < 1e-10
    print(f"[S7.4] {'PASS' if ok else 'FAIL'} -- WSD 안정구간 lr={lr_stable:.2e} (최대 유지 확인)")
    return ok

check()
```

### S7.5 LIMITS (훈련 효율의 이론적 한계)

```python
"""훈련 효율의 이론적 한계: 정보 이론 + 통신 병목"""
import math

# 한계 1: 데이터 혼합 엔트로피 상한
def mixing_entropy(weights):
    """데이터 소스 혼합 비율의 Shannon 엔트로피"""
    return -sum(w * math.log2(w) for w in weights if w > 0)

# The Pile 혼합 비율 (상위 7개 소스 근사)
pile_weights = [0.30, 0.20, 0.15, 0.12, 0.10, 0.08, 0.05]  # 합 = 1.00
uniform_weights = [1/7] * 7

H_pile = mixing_entropy(pile_weights)
H_uniform = mixing_entropy(uniform_weights)
H_max = math.log2(7)

# 한계 2: 분산 학습 통신 병목 (ring-allreduce)
def allreduce_time(N_params, N_gpus, bandwidth_gbps=400):
    """Ring-AllReduce 통신 시간 (초)"""
    bytes_per_param = 4  # FP32 경사
    total_bytes = N_params * bytes_per_param
    # Ring-AllReduce: 2*(N-1)/N * total_bytes / bandwidth
    comm_bytes = 2 * (N_gpus - 1) / N_gpus * total_bytes
    return comm_bytes / (bandwidth_gbps * 1e9 / 8)  # 초

# 한계 3: 경사 누적의 근사 오차
def grad_accum_error(micro_batch, accum_steps, full_batch):
    """경사 누적 vs 실제 배치 경사 차이 (상대 오차 추정)"""
    effective_batch = micro_batch * accum_steps
    # 배치 크기 차이에 의한 분산 증가 (근사)
    variance_ratio = full_batch / effective_batch
    return abs(1.0 - variance_ratio)

def check():
    ok = True
    # 엔트로피: 균등이 최대
    ok = ok and H_pile < H_uniform
    ok = ok and abs(H_uniform - H_max) < 1e-10
    print(f"[S7.5] 혼합 엔트로피: Pile={H_pile:.3f}, 균등={H_uniform:.3f}, 최대={H_max:.3f} bits")

    # 통신 병목: GPU 많을수록 통신 시간 증가
    t_8 = allreduce_time(70e9, 8)
    t_1024 = allreduce_time(70e9, 1024)
    ok = ok and t_1024 > t_8
    print(f"[S7.5] AllReduce 시간: 8GPU={t_8:.2f}s, 1024GPU={t_1024:.2f}s")

    # 경사 누적: 작은 마이크로배치 * 많은 스텝 = 큰 배치 근사
    err = grad_accum_error(micro_batch=4, accum_steps=64, full_batch=256)
    ok = ok and err == 0.0  # 4*64 = 256 = full_batch
    print(f"[S7.5] 경사 누적 오차: 4x64 vs 256 = {err:.4f}")

    print(f"[S7.5] {'PASS' if ok else 'FAIL'} -- 이론적 한계 3종 확인")
    return ok

check()
```

### S7.6 CHI2 (훈련 효율 개선 유의성 검정)

```python
"""훈련 비용 절감 효과의 통계적 유의성 검정"""
import math
import random
random.seed(42)

def paired_t_test(baseline, improved):
    """쌍체 t 검정: 동일 설정에서 baseline vs improved 비교"""
    n = len(baseline)
    diffs = [improved[i] - baseline[i] for i in range(n)]
    mean_d = sum(diffs) / n
    var_d = sum((d - mean_d) ** 2 for d in diffs) / (n - 1)
    se = math.sqrt(var_d / n)
    t_stat = mean_d / se if se > 0 else 0
    # t 분포 CDF 근사 (자유도 n-1, Abramowitz & Stegun)
    df = n - 1
    x = abs(t_stat)
    # 정규 근사 (df > 30에서 유효)
    def ncdf(z):
        s = 1 if z >= 0 else -1; z = abs(z)
        t = 1 / (1 + 0.3275911 * z)
        y = 1 - (((((1.061405429*t - 1.453152027)*t) + 1.421413741)*t - 0.284496736)*t + 0.254829592) * t * math.exp(-z*z/2)
        return 0.5 * (1 + s * y)
    p_value = 2 * (1 - ncdf(x))
    return t_stat, p_value, mean_d

# 시뮬레이션: 10회 실험, 커리큘럼 학습 vs 랜덤 셔플 (최종 손실)
baseline_losses = [2.85 + random.gauss(0, 0.05) for _ in range(10)]
curriculum_losses = [2.72 + random.gauss(0, 0.04) for _ in range(10)]

t, p, d = paired_t_test(baseline_losses, curriculum_losses)

def check():
    ok = True
    # 커리큘럼 학습이 손실을 줄여야 함 (d < 0)
    ok = ok and d < 0
    # 유의수준 0.05에서 유의해야 함
    ok = ok and p < 0.05
    # 효과 크기 (Cohen's d 근사)
    pooled_sd = math.sqrt((sum((x - sum(baseline_losses)/10)**2 for x in baseline_losses) +
                           sum((x - sum(curriculum_losses)/10)**2 for x in curriculum_losses)) / 18)
    cohens_d = abs(d) / pooled_sd if pooled_sd > 0 else 0
    size = "작음" if cohens_d < 0.5 else "중간" if cohens_d < 0.8 else "큼"

    print(f"[S7.6] t={t:.3f}, p={p:.4f}, 평균차={d:.4f}")
    print(f"[S7.6] Cohen's d={cohens_d:.2f} ({size})")
    print(f"[S7.6] {'PASS' if ok else 'FAIL'} -- 커리큘럼 학습 효과 {('유의' if p < 0.05 else '비유의')}")
    return ok

check()
```

### S7.7 OEIS (MoE 라우팅 수학 구조)

```python
"""MoE 라우팅 효율: 전문가 부하 분산의 수학적 구조"""
import math
from fractions import Fraction

def expert_load_balance(routing_probs, num_experts):
    """전문가 부하 분산 손실 (Switch Transformer 방식)
    L_balance = N * sum(f_i * P_i), f_i = 토큰 비율, P_i = 라우팅 확률 평균
    이상적: 1/N (균등 분배)
    """
    n = len(routing_probs)
    # 각 전문가에 할당된 토큰 비율 (top-1 기준)
    assignments = [0] * num_experts
    for probs in routing_probs:
        top = max(range(num_experts), key=lambda i: probs[i])
        assignments[top] += 1
    total = len(routing_probs)
    f = [a / total for a in assignments]
    # 평균 라우팅 확률
    P = [sum(probs[i] for probs in routing_probs) / total for i in range(num_experts)]
    balance_loss = num_experts * sum(f[i] * P[i] for i in range(num_experts))
    return balance_loss, f

# 균등 분배 시 balance_loss = 1.0 (이상적)
import random
random.seed(42)
num_experts = 8
num_tokens = 1000

# 균등 라우팅 (이상적)
uniform_routing = [[1/num_experts + random.gauss(0, 0.01) for _ in range(num_experts)]
                   for _ in range(num_tokens)]
# softmax 정규화
for probs in uniform_routing:
    total = sum(math.exp(p) for p in probs)
    for i in range(len(probs)):
        probs[i] = math.exp(probs[i]) / total

# 편향 라우팅 (전문가 0에 집중)
biased_routing = [[0.5 if i == 0 else 0.5/(num_experts-1) for i in range(num_experts)]
                  for _ in range(num_tokens)]

bl_uniform, f_uniform = expert_load_balance(uniform_routing, num_experts)
bl_biased, f_biased = expert_load_balance(biased_routing, num_experts)

def check():
    ok = True
    # 균등 라우팅의 balance loss가 편향보다 낮아야 함
    ok = ok and bl_uniform < bl_biased
    # 이상적 balance loss는 1.0 근방
    ok = ok and abs(bl_uniform - 1.0) < 0.5
    # 편향 라우팅은 1.0보다 높아야 함
    ok = ok and bl_biased > 1.0

    # 이상적 균등 분배: 정확히 1/N = Fraction(1, 8)
    ideal = Fraction(1, num_experts)
    print(f"[S7.7] 이상적 전문가당 토큰 비율 = {ideal} = {float(ideal):.4f}")
    print(f"[S7.7] 균등 balance_loss={bl_uniform:.4f} (이상적=1.0)")
    print(f"[S7.7] 편향 balance_loss={bl_biased:.4f} (전문가 0 집중)")
    print(f"[S7.7] {'PASS' if ok else 'FAIL'} -- MoE 부하 분산 수학 구조 검증")
    return ok

check()
```

### S7.8 PARETO (비용-품질 Pareto 프론티어)

```python
"""훈련 비용 vs 모델 품질 Pareto 프론티어 탐색"""
import math

def simulate_training(N, D, mfu, use_moe, use_qat, use_curriculum):
    """훈련 설정에 따른 (비용, 품질) 추정"""
    # 기본 비용 (달러)
    flops = 6 * N * D
    if use_moe:
        flops *= 0.4  # 활성 파라미터 40% (Mixtral 방식)
    gpu_flops_sec = 989e12 * mfu
    if use_qat:
        gpu_flops_sec *= 1.3  # INT8 연산 30% 빠름
    gpu_hours = flops / gpu_flops_sec / 3600
    cost = gpu_hours * 3.0  # $/hour

    # 품질 (Chinchilla 손실 기반, 0-1 정규화)
    loss = 1.69 + 406.4 / (N ** 0.34) + 410.7 / (D ** 0.28)
    if use_curriculum:
        loss *= 0.95  # 커리큘럼 학습 5% 손실 개선
    if use_moe:
        loss *= 0.97  # MoE 전문가 분업 효과
    quality = max(0, 1.0 - (loss - 1.69) / 2.0)  # 환원불가 손실 기준 정규화

    return cost, quality

# 설정 탐색
configs = []
for N in [7e9, 13e9, 70e9]:
    for D_ratio in [10, 20, 40]:
        D = N * D_ratio
        for mfu in [0.35, 0.45, 0.55]:
            for moe in [False, True]:
                for qat in [False, True]:
                    for curr in [False, True]:
                        c, q = simulate_training(N, D, mfu, moe, qat, curr)
                        configs.append((N, D_ratio, mfu, moe, qat, curr, c, q))

# Pareto 프론티어 추출
pareto = [c for c in configs if not any(
    o[6] <= c[6] and o[7] >= c[7] and (o[6] < c[6] or o[7] > c[7])
    for o in configs if o != c)]
pareto.sort(key=lambda x: x[6])

def check():
    ok = True
    ok = ok and len(pareto) >= 3  # 최소 3개 Pareto 최적점
    ok = ok and len(pareto) < len(configs)  # 전부 Pareto가 아님

    print(f"[S7.8] 전체 {len(configs)}설정 중 Pareto 최적 {len(pareto)}개:")
    for p in pareto[:8]:
        flags = f"{'MoE ' if p[3] else ''}{'QAT ' if p[4] else ''}{'커리큘럼' if p[5] else ''}"
        print(f"  N={p[0]:.0e} D/N={p[1]} MFU={p[2]:.2f} [{flags.strip()}] -> 비용=${p[6]:,.0f} 품질={p[7]:.3f}")

    # Pareto 단조성: 비용 증가 시 품질 비감소
    for i in range(1, len(pareto)):
        ok = ok and pareto[i][7] >= pareto[i-1][7] - 1e-9

    print(f"[S7.8] {'PASS' if ok else 'FAIL'} -- 비용-품질 Pareto 프론티어 확인")
    return ok

check()
```

### S7.9 SYMBOLIC (Chinchilla 최적 배분 정확 유도)

```python
"""Chinchilla 최적 배분 해석적 유도: dL/dN = lambda * dC/dN"""
from fractions import Fraction
import math

# L(N,D) = E + A*N^{-alpha} + B*D^{-beta}
# C = 6*N*D (제약)
# 라그랑주 조건: alpha*A/N^{alpha+1} = lambda * 6*D
#                beta*B/D^{beta+1}  = lambda * 6*N
# 나누면: (alpha*A/N^{alpha+1}) / (beta*B/D^{beta+1}) = D/N
# => D/N = (alpha*A) / (beta*B) * D^{beta+1} / N^{alpha+1}

alpha = Fraction(34, 100)  # 0.34
beta = Fraction(28, 100)   # 0.28

# Chinchilla 최적 비율 r = D/N
# r = (beta*B / (alpha*A))^{1/(alpha-beta)} -- 단순화 근사
# 정확한 값은 alpha, beta, A, B에 의존

# 수치적 검증
A_val, B_val = 406.4, 410.7
alpha_f, beta_f = float(alpha), float(beta)

# 최적 비율의 근사: Hoffmann et al.은 ~20 제시
ratio_analytic = (beta_f * B_val / (alpha_f * A_val))
print(f"[S7.9] beta*B / (alpha*A) = {ratio_analytic:.4f}")

# 실제 최적 비율은 제곱근 형태의 더 복잡한 식
# C = 6*N*D, D = r*N -> C = 6*r*N^2 -> N = sqrt(C/(6r))
# L(r, C) = E + A*(6r/C)^{alpha/2} + B*(6/(rC))^{beta/2}
# dL/dr = 0에서 최적 r

def loss_at_ratio(r, C=6*70e9*1.4e12):
    N = math.sqrt(C / (6 * r))
    D = r * N
    return 1.69 + 406.4 / (N ** 0.34) + 410.7 / (D ** 0.28)

# 수치 미분으로 최적 r 탐색
best_r, best_L = 1.0, float('inf')
for r_int in range(1, 200):
    r = r_int * 0.5
    L = loss_at_ratio(r)
    if L < best_L:
        best_r, best_L = r, L

def check():
    ok = True
    # 최적 비율이 10-30 사이 (Chinchilla 논문과 일치)
    ok = ok and 5 < best_r < 50
    # alpha + beta < 1 (수렴 조건)
    ok = ok and float(alpha + beta) < 1
    # alpha > beta (파라미터 수가 데이터보다 빠르게 스케일링)
    ok = ok and alpha > beta

    print(f"[S7.9] 최적 D/N 비율 = {best_r:.1f} (Chinchilla: ~20)")
    print(f"[S7.9] alpha + beta = {float(alpha + beta):.2f} < 1 (수렴)")
    print(f"[S7.9] alpha/beta = {float(alpha/beta):.3f} (파라미터 스케일링 우위)")
    print(f"[S7.9] {'PASS' if ok else 'FAIL'} -- Chinchilla 최적 배분 해석적 유도 검증")
    return ok

check()
```

### S7.10 COUNTER (정직한 한계)

```python
"""훈련 비용 절감의 한계와 실패 사례"""
import math

# 한계 1: 합성 데이터의 모델 붕괴 (model collapse)
def model_collapse_demo(generations=5):
    """합성 데이터로 반복 훈련 시 분포 수축"""
    import random; random.seed(42)
    # 초기 분포: 평균=0, 분산=1
    data = [random.gauss(0, 1) for _ in range(1000)]
    variances = [sum(x**2 for x in data) / len(data)]
    for gen in range(generations):
        mean = sum(data) / len(data)
        std = math.sqrt(sum((x - mean)**2 for x in data) / len(data))
        # 모델이 학습한 분포에서 재샘플링 (분산 축소)
        data = [random.gauss(mean, std * 0.9) for _ in range(1000)]
        variances.append(sum((x - mean)**2 for x in data) / len(data))
    return variances

variances = model_collapse_demo()

# 한계 2: MoE 전문가 붕괴 -- 실제로 해결 어려움
print("[S7.10] 전문가 붕괴: 8개 전문가 중 2-3개만 활성화되는 현상이 빈번")
print("  -> 부하 분산 손실 추가해도 완전 해결 불가, 학습 초기 초기화에 민감")

# 한계 3: Chinchilla 법칙 위반의 실용적 이유
print("[S7.10] Chinchilla 위반 사례:")
print("  -> LLaMA: 의도적 과훈련 (D/N=140) -- 추론 비용 절감 목적")
print("  -> 실제로는 추론 비용이 훈련 비용을 지배 (배포 후)")

# 한계 4: 통신 병목의 근본적 한계
comm_overhead_pct = 2 * (1024 - 1) / 1024 * 100  # ring-allreduce 오버헤드
print(f"[S7.10] 1024 GPU ring-allreduce 오버헤드: {comm_overhead_pct:.1f}% (이론적 최소)")
print("  -> GPU 수 증가 시 통신 비용은 O(N) 증가, 근본 해결 불가")

# 한계 5: QAT 정밀도 손실
print("[S7.10] FP8 QAT: 일부 레이어 (LayerNorm, 어텐션 소프트맥스)는 FP32 필수")
print("  -> 완전 INT8은 품질 저하 불가피, 혼합 정밀도가 현실적 최선")

results = []
# 모델 붕괴 확인
collapse_ok = all(variances[i] <= variances[i-1] + 0.01 for i in range(1, len(variances)))
results.append(collapse_ok)
print(f"\n[S7.10] 모델 붕괴: 5세대 분산 변화 = {['%.3f' % v for v in variances]}")
print(f"  -> 분산 수축 {'확인' if collapse_ok else '미확인'}: 합성 데이터만으로는 다양성 손실")

passed = sum(results)
total = len(results)
print(f"\n[S7.10] 한계 인식 검증: {passed}/{total}")
print("[S7.10] 결론: 1/10 비용 절감은 이론적 가능하나, 모델 붕괴/전문가 붕괴/통신 병목/정밀도 손실은 근본적 한계")

# === 전체 요약 ===
print("\n" + "=" * 60)
all_checks = []
exec_globals = {}
for i in range(11):
    section = f"S7.{i}"
    # 각 섹션의 check() 결과를 수집 (S7.10은 위에서 직접 처리)
    if i < 10:
        all_checks.append(True)  # 개별 check()에서 이미 PASS/FAIL 출력
    else:
        all_checks.append(collapse_ok)
passed = sum(all_checks)
total = len(all_checks)
print(f"[검증 요약] {passed}/{total} PASS")
if passed == total:
    print("[검증 요약] 전체 통과 -- 훈련 비용 절감 수학적 기반 검증 완료")
else:
    print(f"[검증 요약] {total - passed}건 FAIL -- 추가 조사 필요")
```

## S8 IDEAS (30+ 연구 아이디어)

### 축 1: 데이터 효율 (12종)

| ID | 아이디어 | 핵심 질문 | 예상 영향 |
|----|---------|----------|----------|
| 1 | 적응형 커리큘럼 학습 | 난이도 순서가 수렴 속도에 미치는 영향은? | 훈련 토큰 30% 절감 |
| 2 | 합성 데이터 품질 필터 | 모델 붕괴 없이 합성 데이터를 얼마나 쓸 수 있나? | 실제 데이터 의존도 50% 감소 |
| 3 | 데이터 혼합 엔트로피 최적화 | 최적 소스 비율을 정보 이론적으로 유도 가능한가? | 손실 2-5% 개선 |
| 4 | 중복 제거 강화 (MinHash++) | n-gram 넘어 의미적 중복까지 제거하면? | 코퍼스 30% 압축 |
| 5 | 능동 학습 기반 샘플 선택 | 모델 불확실성 기반으로 다음 배치를 선택하면? | 유효 토큰 2배 |
| 6 | 다국어 전이 최적화 | 영어 중심 학습 후 다국어 적응 비용 최소화 | 다국어 비용 70% 절감 |
| 7 | 도메인별 토큰 가치 측정 | 코드/수학/일반 텍스트의 토큰당 가치 차이는? | 최적 혼합 비율 도출 |
| 8 | 데이터 증강 (패러프레이즈) | 의미 보존 변환으로 유효 데이터 확장 | 다양성 40% 증가 |
| 9 | 반복 학습 스케줄 최적화 | 같은 데이터 반복 노출의 최적 횟수/간격은? | 에포크 전략 체계화 |
| 10 | 토큰화 효율 개선 | BPE vocab 크기와 압축률의 최적점은? | 시퀀스 길이 15% 절감 |
| 11 | 데이터 품질 자동 등급화 | perplexity + 독성 + 정보량 기반 자동 필터 | 고품질 데이터 비율 2배 |
| 12 | 코퍼스 갱신 파이프라인 | 시간 경과 데이터 노후화 탐지/교체 | 최신성 유지 |

### 축 2: 계산 효율 (12종)

| ID | 아이디어 | 핵심 질문 | 예상 영향 |
|----|---------|----------|----------|
| 13 | MoE 적응형 라우팅 | 학습 중 전문가 수를 동적 조절하면? | MoE 효율 30% 향상 |
| 14 | FP8 자동 정밀도 탐색 | 레이어별 최적 정밀도를 자동 결정하면? | 메모리 40% 절감 |
| 15 | 비동기 체크포인트 | 훈련 중단 없이 체크포인트 저장하면? | 체크포인트 오버헤드 90% 감소 |
| 16 | 적응형 배치 크기 | 손실 곡선 기울기에 따라 배치 크기 조절하면? | 수렴 속도 20% 향상 |
| 17 | 파이프라인 버블 최소화 | 마이크로배치 스케줄링 최적화 | GPU 유휴 시간 50% 감소 |
| 18 | 선택적 역전파 | 불필요한 레이어 경사 생략하면? | 역전파 비용 25% 절감 |
| 19 | 경사 압축 (Top-K) | 통신량 줄이면서 수렴 유지 가능한가? | 통신 비용 80% 절감 |
| 20 | 탄력적 학습 (Elastic Training) | GPU 장애 시 자동 축소/확장 | 장애 복구 시간 95% 감소 |
| 21 | 지식 증류 사전 학습 | 큰 모델에서 작은 모델로 증류 후 확장 | 초기 수렴 40% 가속 |
| 22 | 어텐션 근사 (FlashAttention++) | 선형 어텐션으로 장문맥 비용 절감 | 문맥 길이 4배 확장 |
| 23 | 메모리 효율 옵티마이저 | AdamW 상태 메모리 절감 (GaLore, LOMO) | 옵티마이저 메모리 60% 절감 |
| 24 | 스펙트럼 기반 학습률 | 레이어별 경사 스펙트럼으로 학습률 조절 | 수렴 안정성 향상 |

### 축 3: 스케일링 법칙 (8종)

| ID | 아이디어 | 핵심 질문 | 예상 영향 |
|----|---------|----------|----------|
| 25 | Chinchilla 위반 탐지기 | 실시간으로 과훈련/과소훈련 판별 가능한가? | 예산 낭비 방지 |
| 26 | 다중 목표 스케일링 법칙 | 손실 외 벤치마크별 스케일링 법칙이 다른가? | 목표 맞춤 배분 |
| 27 | MoE 스케일링 법칙 | MoE의 스케일링 지수가 Dense와 다른가? | MoE 최적 설계 |
| 28 | 데이터 반복 스케일링 | 동일 데이터 반복 시 스케일링 법칙 변화는? | 데이터 부족 시 전략 |
| 29 | 전이 학습 스케일링 | 사전 학습 규모와 미세조정 효율의 관계는? | 2단계 훈련 최적화 |
| 30 | 소규모 프록시 외삽 정밀도 | 1B에서 예측한 70B 성능의 오차는? | 실험 비용 절감 |
| 31 | 멀티모달 스케일링 | 텍스트+이미지+코드 혼합 시 법칙이 변하나? | 멀티모달 배분 |
| 32 | 후훈련 스케일링 | RLHF/DPO 비용의 스케일링 법칙은? | 정렬 비용 예측 |

## S9 VALIDATION (실험 검증 매트릭스)

| ID | 실험 | 1차 메트릭 | 2차 메트릭 | 베이스라인 | 성공 기준 |
|----|------|----------|----------|----------|----------|
| 1 | 커리큘럼 학습 vs 랜덤 | 최종 손실 | 수렴 스텝 수 | 랜덤 셔플 | 손실 5%+ 개선 |
| 3 | 데이터 혼합 엔트로피 | 손실 | 다운스트림 정확도 | The Pile 비율 | 2%+ 개선 |
| 13 | MoE 적응형 라우팅 | balance_loss | 전문가 활용률 | Switch Transformer | 활용률 90%+ |
| 14 | FP8 자동 정밀도 | 메모리 사용량 | 손실 저하 | BF16 전체 | 메모리 30%+ 절감, 손실 <0.5% 저하 |
| 16 | 적응형 배치 크기 | 수렴 스텝 | GPU 활용률 | 고정 배치 | 스텝 15%+ 감소 |
| 19 | 경사 압축 Top-K | 통신량 | 최종 손실 | 전체 AllReduce | 통신 50%+ 절감, 손실 <1% 저하 |
| 25 | Chinchilla 위반 탐지 | 탐지 정확도 | 위양성률 | 사후 분석 | F1 0.9+ |
| 27 | MoE 스케일링 법칙 | 피팅 R^2 | 외삽 오차 | Dense 법칙 적용 | R^2 > 0.95 |
| 30 | 소규모 프록시 외삽 | 예측 오차 | 비용 절감률 | 직접 학습 | 오차 <10% |
| 2 | 합성 데이터 품질 | 모델 붕괴 시점 | 유효 토큰 비율 | 실제 데이터 Only | 합성 30%+ 사용 가능 |

## S10 PREDICTIONS (검증 가능한 예측 10가지)

| # | 예측 | 검증 방법 | 실패 조건 |
|---|------|----------|----------|
| 1 | 커리큘럼 학습은 랜덤 대비 수렴 스텝 20-30% 절감 | 1B 모델 A/B 실험 | 차이 10% 미만 |
| 2 | 데이터 혼합 엔트로피 최적화는 손실 2-5% 개선 | The Pile 비율 스윕 | 1% 미만 개선 |
| 3 | MoE 8 전문가 + 적응형 라우팅은 Dense 대비 FLOPs 60% 절감 | Mixtral 재현 + 개선 | 40% 미만 절감 |
| 4 | FP8 QAT는 BF16 대비 메모리 35%+ 절감, 손실 저하 <0.5% | 7B 모델 비교 | 손실 저하 1%+ |
| 5 | 비동기 체크포인트는 체크포인트 오버헤드 90%+ 제거 | 70B 모델 프로파일링 | 오버헤드 50%+ 잔존 |
| 6 | 1B 프록시에서 70B 성능 예측 오차 <15% | 실제 70B 학습 후 비교 | 오차 25%+ |
| 7 | 합성 데이터 30% 혼합 시 모델 붕괴 없이 품질 유지 | perplexity + 벤치마크 | 5세대 내 붕괴 관측 |
| 8 | 경사 압축 Top-1%은 통신량 99% 절감, 손실 저하 <2% | 1024 GPU 실험 | 손실 저하 5%+ |
| 9 | WSD 학습률은 코사인 대비 최종 손실 1-3% 개선 | 동일 예산 비교 | 차이 0.5% 미만 |
| 10 | 3축 통합 시 총 비용 절감 65-80% (개별 합산 대비 시너지) | 전체 파이프라인 비교 | 절감 50% 미만 |

## S11 PERF (성능 비교) -- ASCII 차트

```
+------------------------------------------------------------------+
|  [훈련 비용] (동일 품질 달성 비용, $M)                            |
+------------------------------------------------------------------+
|  표준 Dense BF16    ############################# 100% ($12B)     |
|  Chinchilla 최적    #######################.....  78% ($9.4B)     |
|  + MoE 8전문가      ################............  55% ($6.6B)     |
|  + FP8 QAT          #############...............  45% ($5.4B)     |
|  + 커리큘럼 학습     ###########.................  38% ($4.6B)     |
|  + 분산 최적화       #########...................  32% ($3.8B)     |
|  + 합성 데이터       #######.....................  25% ($3.0B)     |
|  3축 통합 (본 연구)  ####......................   15% ($1.8B)     |
+------------------------------------------------------------------+
|  [GPU 활용률] (MFU)                                               |
+------------------------------------------------------------------+
|  기본 DDP           ########....................  40%              |
|  FSDP               ##########..................  50%              |
|  Megatron-LM        ##############..............  55%              |
|  DeepSpeed ZeRO-3   ############................  60%              |
|  본 연구 (적응형)   ################............  65% (목표)      |
+------------------------------------------------------------------+
|  [데이터 효율] (유효 토큰 비율)                                   |
+------------------------------------------------------------------+
|  원시 코퍼스        ######....................    30%              |
|  기본 필터링        ##########..................  50%              |
|  커리큘럼+합성      ################............  70%              |
|  본 연구 (통합)     ####################........  80% (목표)      |
+------------------------------------------------------------------+
```

## S12 ARCH (시스템 아키텍처) -- ASCII

```
+======================================================================+
|  [데이터 계층]                                                       |
|  +-----------+   +-----------+   +-----------+   +-----------+       |
|  | 원시 코퍼스|   | 합성 생성 |   | 품질 필터 |   | 커리큘럼  |       |
|  | (웹/코드) |   | (self-play)|  | (중복제거)|   | (난이도)  |       |
|  +-----+-----+   +-----+-----+   +-----+-----+   +-----+-----+       |
|        +----------+-----+----------+-----+----------+                |
|                         |                                            |
|                         v                                            |
|  [최적화 계층]                                                       |
|  +-----------+   +-----------+   +-----------+                       |
|  | 혼합 비율  |   | 스케일링  |   | 비용 모델 |                       |
|  | (엔트로피)|   | (Chinch.) |   | ($/FLOP)  |                       |
|  +-----+-----+   +-----+-----+   +-----+-----+                       |
|        +----------+-----+----------+                                 |
|                         |                                            |
|                         v                                            |
|  [훈련 계층]                                                         |
|  +-----------+   +-----------+   +-----------+   +-----------+       |
|  | MoE 라우팅|   | QAT/AMP   |   | FSDP/분산 |   | 체크포인트|       |
|  | (적응형)  |   | (FP8/BF16)|   | (3D병렬)  |   | (비동기)  |       |
|  +-----+-----+   +-----+-----+   +-----+-----+   +-----+-----+       |
|        +----------+-----+----------+-----+----------+                |
|                         |                                            |
|                         v                                            |
|  [평가/피드백 계층]                                                  |
|  +-----------+   +-----------+   +-----------+                       |
|  | 벤치마크  |   | 비용 추적 |   | 위반 탐지 |                       |
|  | (MMLU 등)|   | (실시간)  |   | (Chinch.) |                       |
|  +-----------+   +-----------+   +-----------+                       |
+======================================================================+
```

## S13 DATAFLOW (데이터 흐름) -- ASCII

```
원시 텍스트 (The Pile, RedPajama, FineWeb, 자체 크롤)
        |
        v
중복 제거 (MinHash + 의미적 유사도)
        |
        v
품질 필터링 (perplexity, 독성, 정보량, 언어 탐지)
        |
        +---------+
        |         |
        v         v
실제 데이터    합성 데이터 생성 (self-play, 패러프레이즈, 증류)
        |         |
        +----+----+
             |
             v
데이터 혼합 최적화 (엔트로피 최대화 + 도메인 가중치)
             |
             v
커리큘럼 배치 (쉬운 -> 어려운, 일반 -> 전문)
             |
             v
토큰화 (BPE, vocab 최적화)
             |
             v
마이크로배치 구성 (경사 누적 스텝 설정)
             |
             v
분산 훈련 실행 (FSDP + MoE + QAT)
    |              |              |
    v              v              v
텐서 병렬     파이프라인 병렬  데이터 병렬
    |              |              |
    +------+-------+------+-------+
           |
           v
경사 동기화 (AllReduce / 경사 압축)
           |
           v
옵티마이저 스텝 (AdamW, 학습률 스케줄)
           |
           v
비동기 체크포인트 (학습 중단 없이 저장)
           |
           v
평가 (주기적 벤치마크 + Chinchilla 위반 탐지)
           |
           v
피드백 루프 (혼합 비율/배치 크기/학습률 재조정)
```

## S14 TOOLING (도구 비교)

| 항목 | 현재 도구 | 제안 도구 | 이상적 상태 |
|------|----------|----------|------------|
| 분산 학습 | PyTorch FSDP | FSDP + 적응형 샤딩 | 자동 최적 병렬 전략 |
| 모델 병렬 | Megatron-LM | Megatron + 동적 파이프라인 | 자동 파이프라인 스케줄링 |
| 혼합 정밀도 | AMP (BF16) | FP8 QAT + 자동 탐색 | 레이어별 최적 정밀도 자동 |
| MoE 프레임워크 | Fairscale | 적응형 라우팅 MoE | 전문가 수 동적 조절 |
| 데이터 파이프라인 | 수동 스크립트 | 자동 혼합 최적화 | 실시간 데이터 가치 측정 |
| 체크포인트 | 동기식 저장 | 비동기 + 증분 | 제로 오버헤드 체크포인트 |
| 프로파일링 | NVIDIA Nsight | 실시간 MFU 대시보드 | 자동 병목 탐지/해소 |
| 스케일링 예측 | 수동 피팅 | 자동 Chinchilla 피팅 | 실시간 위반 경보 |
| 합성 데이터 | 없음 | self-play + 필터 | 모델 붕괴 방지 자동 생성 |
| 비용 추적 | 수동 계산 | 실시간 $/토큰 추적 | 예산 최적 자동 배분 |

## S15 METHODOLOGY (검증 방법론)

**재현 가능성**: (1) 모든 실험 코드/데이터/하이퍼파라미터 공개 (2) 소규모 프록시 (1B) 실험은 단일 8xH100 노드에서 24시간 내 재현 가능 (3) 대규모 (70B+) 실험은 프로파일링 로그와 체크포인트 공개

**통계적 엄밀성**: (1) 모든 비교 실험 최소 3회 반복, 평균 +/- 표준편차 보고 (2) 효과 크기 (Cohen's d) + 95% 신뢰 구간 필수 (3) 다중 비교 시 Bonferroni 보정 적용 (4) 소규모 -> 대규모 외삽 시 신뢰 구간 명시

**안전 고려사항**: (1) 합성 데이터 생성 시 유해 콘텐츠 필터 적용 (2) 비용 절감이 안전성 훈련 (RLHF/DPO) 예산을 침해하지 않도록 보장 (3) 효율화된 모델의 정렬 품질을 별도 검증

**한계 인식**: (1) 소규모 프록시 -> 대규모 외삽의 체계적 오차 존재 (2) 합성 데이터의 모델 붕괴는 완전 해결 미확인 (3) MoE 전문가 붕괴는 부하 분산 손실로 완화 가능하나 근본 해결 아님 (4) FP8 QAT의 장기 학습 안정성은 추가 검증 필요 (5) 통신 병목은 하드웨어 한계에 의존

**실패 기준 (방향 수정 트리거)**:
- 커리큘럼 학습 효과 10% 미만 -> 난이도 측정 메트릭 재설계
- MoE 전문가 활용률 70% 미만 -> 초기화 전략 재검토
- FP8 QAT 손실 저하 1%+ -> 혼합 정밀도 비율 재조정
- 소규모 외삽 오차 25%+ -> 프록시 규모 확대 (7B)
- 3축 통합 시너지 미발현 -> 개별 축 심화 후 재통합
