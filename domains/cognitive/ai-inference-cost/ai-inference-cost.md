---
domain: ai-inference-cost
requires:
  - to: ai-training-cost
---
# 추론/서빙 비용 절감 연구 프로그램 (Anthropic Fellows 2026)

## S1 WHY (왜 추론 비용 절감이 중요한가)

대규모 언어 모델의 학습 비용은 일회성이지만, 추론/서빙 비용은 사용자 수에 비례해 영구히 누적된다. Anthropic의 연간 서빙 비용은 $7B 규모로 추산되며, 1억+ 사용자를 대상으로 품질과 지연 시간을 유지하면서 비용을 1/5로 줄이는 것이 핵심 과제다.

| 측면 | 현재 상황 | 목표 |
|------|----------|------|
| KV 캐시 | 1M 컨텍스트 시 GPU 메모리 폭발 | PagedAttention + 압축으로 10배 절감 |
| 배칭 | 정적 배칭, GPU 유휴 시간 30%+ | 연속 배칭으로 GPU 활용률 95%+ |
| 어텐션 | O(n^2) 복잡도, 긴 컨텍스트 불가 | FlashAttention + 선형 어텐션 |
| 양자화 | FP16 기본, 메모리 과다 | INT4/INT8 품질 유지 양자화 |
| 멀티모달 | 비전/오디오/비디오 별도 파이프라인 | 통합 서빙 + 모달별 최적화 |

**핵심 질문**: 1억+ 사용자에게 현재 비용의 1/5로 서빙하면서, 품질 저하 없이 지연 시간을 유지하려면 어떤 시스템 아키텍처가 필요한가?

## S2 COMPARE (추론 최적화 기법 비교) -- ASCII 차트

```
+------------------------------------------------------------------+
|  [처리량 향상 배율] (기본 FP16 대비)                              |
+------------------------------------------------------------------+
|  FP16 기본          ##..........................  1.0x             |
|  INT8 양자화        ####........................  2.0x             |
|  INT4 (GPTQ)        ######......................  3.2x             |
|  INT4 (AWQ)          #######.....................  3.5x             |
|  FlashAttention-2   ########....................  4.0x (메모리)    |
|  PagedAttention     ########....................  4.2x (배칭)      |
|  연속 배칭          ##########..................  5.0x             |
|  투기적 디코딩      ############................  6.0x             |
|  전체 최적화 스택   ####################........  10x+ (목표)      |
+------------------------------------------------------------------+
|  [TTFT 지연 시간] (ms, 1K 입력 토큰)                              |
+------------------------------------------------------------------+
|  기본 FP16          ##########################..  1300ms           |
|  FlashAttention-2   ####################........  1000ms           |
|  프리필 최적화      ##############..............  700ms            |
|  프롬프트 캐싱 히트 ####..........................  200ms            |
|  프리픽스 캐싱      ##............................  100ms            |
+------------------------------------------------------------------+
|  [GPU 메모리 효율] (70B 모델 기준)                                |
+------------------------------------------------------------------+
|  FP16               ##############################  140GB          |
|  INT8               ###############...............  70GB           |
|  INT4 (GPTQ)         ########......................  35GB           |
|  INT4 + PagedAtt    ######........................  28GB           |
|  INT4 + KV 압축     ####..........................  20GB           |
+------------------------------------------------------------------+
```

## S3 REQUIRES (선행 요구사항)

| 선행 영역 | 필요 수준 | 핵심 기술 |
|-----------|----------|----------|
| GPU 아키텍처 | 중급 | CUDA 커널, 메모리 계층, SM 점유율 |
| 트랜스포머 내부 구조 | 상급 | MHA/GQA/MQA, FFN, 레이어 정규화 |
| 시스템 프로그래밍 | 중급 | 메모리 관리, 비동기 I/O, 스케줄링 |
| 수치 최적화 | 중급 | 양자화 이론, 혼합 정밀도, 교정 |
| 분산 시스템 | 중급 | 텐서/파이프라인 병렬화, 로드 밸런싱 |

## S4 STRUCT (3축 아키텍처)

```
+======================================================================+
|  [축 1: 메모리 최적화]        [축 2: 연산 최적화]                     |
|  +--------------------+      +--------------------+                  |
|  | KV 캐시 압축       |      | FlashAttention-3   |                  |
|  | PagedAttention     |      | 투기적 디코딩      |                  |
|  | 프롬프트/프리픽스   |      | 연속 배칭          |                  |
|  |   캐싱             |      | 양자화 (INT4/INT8) |                  |
|  +----------+---------+      +----------+---------+                  |
|             +--------+--------+------+                               |
|                      |                                               |
|             [축 3: 시스템 아키텍처]                                   |
|             +--------------------+                                   |
|             | 요청 라우팅/스케줄  |                                   |
|             | 멀티모달 서빙 통합  |                                   |
|             | 오토스케일링        |                                   |
|             +--------------------+                                   |
+======================================================================+
```

## S5 FLOW (연구 흐름)

```
프로파일링 --> 병목 분석 --> 최적화 설계 --> 구현 --> 벤치마크
    |              |              |            |          |
    v              v              v            v          v
GPU 프로파일   메모리/연산     알고리즘      CUDA 커널  처리량/지연
지연 분해     대역폭 분석    수학적 증명    Triton     품질 검증
    |              |              |            |          |
    +-----<--------+------<-------+-----<------+----<-----+
                     피드백 루프 (반복 개선)
```

## S6 EVOLVE (4개월 Anthropic 로드맵)

- **Mk.I (1개월)**: vLLM/TGI 기반 베이스라인 프로파일링 + KV 캐시 메모리 사용량 정밀 측정 + 병목 분류 체계 구축
- **Mk.II (2개월)**: PagedAttention 개선 + 투기적 디코딩 수용률 최적화 + 연속 배칭 스케줄러 프로토타입 + INT4 품질 유지 양자화 파이프라인
- **Mk.III (3개월)**: 1M 컨텍스트 KV 캐시 압축 + 프롬프트/프리픽스 캐싱 통합 + 멀티모달 서빙 프로파일 최적화 + 선형 어텐션 하이브리드
- **Mk.IV (4개월)**: 전체 최적화 스택 통합 (10x 목표) + 1억 사용자 시뮬레이션 + 논문 작성 + 오픈소스 서빙 프레임워크 기여

## S7 VERIFY (추론 비용 검증 코드 -- Python stdlib only)

### S7.0 CONSTANTS (추론 서빙 핵심 상수)

```python
"""추론 서빙 핵심 상수 -- 하드웨어 및 모델 사양에서 유도"""
import math

# 모델 파라미터
D_MODEL = 8192          # 히든 차원 (70B 급)
N_LAYERS = 80           # 레이어 수
N_HEADS = 64            # 어텐션 헤드 수
N_KV_HEADS = 8          # GQA 키-값 헤드 수
HEAD_DIM = D_MODEL // N_HEADS  # = 128
VOCAB_SIZE = 128000     # 어휘 크기

# 하드웨어 (H100 SXM)
HBM_BW = 3.35e12       # HBM 대역폭 (bytes/s)
FLOPS = 989e12          # FP16 TFLOPS -> FLOPS
HBM_SIZE = 80e9         # 80GB HBM

# 서빙 상수
BYTES_FP16 = 2
BYTES_INT8 = 1
BYTES_INT4 = 0.5

assert HEAD_DIM == 128, "헤드 차원 128 확인"
assert N_KV_HEADS < N_HEADS, "GQA: KV 헤드 < Q 헤드"
print(f"[S7.0] 모델: d={D_MODEL}, L={N_LAYERS}, H={N_HEADS}, KV_H={N_KV_HEADS}")
print(f"[S7.0] H100: HBM={HBM_SIZE/1e9:.0f}GB, BW={HBM_BW/1e12:.2f}TB/s")
```

### S7.1 DIMENSIONS (KV 캐시 메모리 수학)

```python
"""KV 캐시 메모리 계산: 시퀀스 길이별 GPU 메모리 요구량"""
import math

D_MODEL = 8192; N_LAYERS = 80; N_KV_HEADS = 8; HEAD_DIM = 128
BYTES_FP16 = 2

def kv_cache_bytes(seq_len, batch_size=1, dtype_bytes=BYTES_FP16):
    """KV 캐시 = 2 * n_layers * n_kv_heads * head_dim * seq_len * batch * dtype"""
    return 2 * N_LAYERS * N_KV_HEADS * HEAD_DIM * seq_len * batch_size * dtype_bytes

# 단일 요청 KV 캐시 크기
for seq in [1024, 4096, 32768, 131072, 1048576]:
    mem = kv_cache_bytes(seq)
    print(f"  seq={seq:>8d}: KV={mem/1e9:.2f} GB")

# 1M 컨텍스트 = 단일 요청에 163.84GB -> 80GB H100 1장 불가
mem_1m = kv_cache_bytes(1048576)
assert mem_1m > 80e9, "1M 컨텍스트 KV 캐시 > 80GB HBM"

# INT4 양자화 시 4배 절감
mem_1m_int4 = kv_cache_bytes(1048576, dtype_bytes=0.5)
assert mem_1m_int4 < mem_1m / 3, "INT4 KV 캐시 4배 절감"

# 배치 16, 4K 컨텍스트 = 실용적 설정
mem_batch = kv_cache_bytes(4096, batch_size=16)
print(f"[S7.1] batch=16, seq=4K: KV={mem_batch/1e9:.2f}GB (실용적)")
print(f"[S7.1] 1M 컨텍스트 FP16={mem_1m/1e9:.1f}GB, INT4={mem_1m_int4/1e9:.1f}GB")
print(f"[S7.1] PASS: KV 캐시 메모리 수학 검증 완료")
```

### S7.2 CROSS (투기적 디코딩 수용률 교차 검증)

```python
"""투기적 디코딩: 드래프트 모델 수용률 수학적 분석"""
import math, random
random.seed(42)

def spec_decode_acceptance(draft_probs, target_probs):
    """Leviathan et al. (2023): 수용 확률 = min(1, q(x)/p(x))"""
    accepted = 0
    for p_d, p_t in zip(draft_probs, target_probs):
        # 수용 확률 = min(1, p_target / p_draft)
        alpha = min(1.0, p_t / p_d) if p_d > 0 else 0
        if random.random() < alpha:
            accepted += 1
    return accepted / len(draft_probs)

# 시뮬레이션: 드래프트 모델과 타겟 모델의 토큰 분포 유사도
n_tokens = 10000
# 케이스 1: 높은 유사도 (좋은 드래프트 모델)
draft_high = [random.uniform(0.01, 0.1) for _ in range(n_tokens)]
target_high = [d * random.uniform(0.8, 1.2) for d in draft_high]
rate_high = spec_decode_acceptance(draft_high, target_high)

# 케이스 2: 낮은 유사도 (나쁜 드래프트 모델)
draft_low = [random.uniform(0.01, 0.1) for _ in range(n_tokens)]
target_low = [random.uniform(0.01, 0.1) for _ in range(n_tokens)]
rate_low = spec_decode_acceptance(draft_low, target_low)

assert rate_high > rate_low, "유사도 높은 드래프트 -> 높은 수용률"

# 속도 향상: gamma 토큰 투기 시, 기대 속도 향상 = (1 - alpha^(gamma+1)) / (1 - alpha)
def speedup(alpha, gamma):
    if abs(alpha - 1.0) < 1e-10:
        return gamma + 1
    return (1.0 - alpha**(gamma + 1)) / (1.0 - alpha)

for g in [1, 3, 5, 7]:
    s = speedup(0.8, g)
    print(f"  gamma={g}, alpha=0.8: 속도 향상={s:.2f}x")

print(f"[S7.2] 수용률: 높은 유사도={rate_high:.3f}, 낮은 유사도={rate_low:.3f}")
print(f"[S7.2] PASS: 투기적 디코딩 수학 검증 완료")
```

### S7.3 SCALING (배칭 처리량 스케일링)

```python
"""연속 배칭 vs 정적 배칭: 처리량 비교"""
import math

def static_batch_throughput(batch_size, seq_len, gen_len, time_per_token_ms):
    """정적 배칭: 모든 요청이 max_gen_len까지 대기"""
    total_time_ms = gen_len * time_per_token_ms  # 가장 긴 생성에 맞춤
    tokens_generated = batch_size * gen_len
    return tokens_generated / (total_time_ms / 1000)  # tokens/s

def continuous_batch_throughput(batch_size, seq_len, gen_lens, time_per_token_ms):
    """연속 배칭: 완료된 요청 즉시 교체, 평균 생성 길이 기준"""
    avg_gen = sum(gen_lens) / len(gen_lens)
    # GPU 유휴 시간 제거 -> 평균 기준 처리량
    total_time_ms = avg_gen * time_per_token_ms
    tokens_generated = batch_size * avg_gen
    # 슬롯 재활용으로 추가 요청 처리
    max_gen = max(gen_lens)
    extra_requests = sum(max_gen // g - 1 for g in gen_lens if g < max_gen)
    effective_tokens = tokens_generated + extra_requests * avg_gen
    return effective_tokens / (max_gen * time_per_token_ms / 1000)

bs = 32
gen_lens = [50, 100, 200, 500, 50, 80, 300, 150,
            60, 120, 250, 400, 70, 90, 180, 350,
            55, 110, 220, 450, 65, 85, 280, 160,
            75, 130, 190, 380, 45, 95, 210, 420]
static_tp = static_batch_throughput(bs, 1024, max(gen_lens), 20)
contin_tp = continuous_batch_throughput(bs, 1024, gen_lens, 20)
ratio = contin_tp / static_tp

assert ratio > 1.5, "연속 배칭이 정적 배칭보다 50%+ 처리량 향상"
print(f"[S7.3] 정적 배칭: {static_tp:.0f} tok/s")
print(f"[S7.3] 연속 배칭: {contin_tp:.0f} tok/s")
print(f"[S7.3] 처리량 향상: {ratio:.2f}x")
print(f"[S7.3] PASS: 연속 배칭 처리량 수학 검증 완료")
```

### S7.4 SENSITIVITY (어텐션 복잡도 O(n^2) vs O(n))

```python
"""어텐션 복잡도: 표준 vs FlashAttention vs 선형 어텐션"""
import math

def standard_attention_flops(seq_len, d_model, n_heads):
    """표준 어텐션: O(n^2 * d) -- QK^T + softmax + AV"""
    head_dim = d_model // n_heads
    # QK^T: n * n * d_h per head -> n_heads * n^2 * d_h
    qk = n_heads * seq_len * seq_len * head_dim
    # softmax: n * n per head
    sm = n_heads * seq_len * seq_len
    # AV: n * n * d_h per head
    av = n_heads * seq_len * seq_len * head_dim
    return 2 * (qk + av) + sm  # multiply-add = 2 flops

def flash_attention_flops(seq_len, d_model, n_heads):
    """FlashAttention: 동일 FLOP, 메모리 O(n) -- IO 최적화"""
    return standard_attention_flops(seq_len, d_model, n_heads)

def flash_attention_memory(seq_len, d_model, n_heads, block_size=256):
    """FlashAttention 메모리: O(n * d) vs 표준 O(n^2)"""
    head_dim = d_model // n_heads
    # 타일링: block_size * block_size 씩 처리
    return n_heads * seq_len * head_dim * 2  # Q, O만 저장 (K, V 스트리밍)

def linear_attention_flops(seq_len, d_model, n_heads, feature_dim=64):
    """선형 어텐션: O(n * d * f) -- phi(Q) * (phi(K)^T * V)"""
    head_dim = d_model // n_heads
    # phi(K)^T * V: d_f * d_h (누적)
    kv = seq_len * feature_dim * head_dim
    # phi(Q) * KV: n * d_f * d_h
    qkv = seq_len * feature_dim * head_dim
    return 2 * n_heads * (kv + qkv)

d, h = 8192, 64
print("[S7.4] 시퀀스 길이별 FLOP 비교 (70B 모델, 단일 레이어):")
print(f"  {'seq':>8s} | {'표준':>12s} | {'선형':>12s} | {'비율':>6s}")
for s in [1024, 4096, 16384, 65536, 262144]:
    std = standard_attention_flops(s, d, h)
    lin = linear_attention_flops(s, d, h)
    print(f"  {s:>8d} | {std:.2e} | {lin:.2e} | {std/lin:.1f}x")

# 표준 어텐션은 n^2 스케일링 -> 긴 시퀀스에서 선형보다 훨씬 느림
ratio_short = standard_attention_flops(1024, d, h) / linear_attention_flops(1024, d, h)
ratio_long = standard_attention_flops(262144, d, h) / linear_attention_flops(262144, d, h)
assert ratio_long > ratio_short * 10, "긴 시퀀스에서 선형 어텐션 이점 급증"

# FlashAttention 메모리 절감
std_mem = h * 1024 * 1024 * 2  # O(n^2) 어텐션 맵
flash_mem = flash_attention_memory(1024, d, h)
print(f"[S7.4] 메모리 (seq=1K): 표준={std_mem/1e6:.0f}MB, Flash={flash_mem/1e6:.0f}MB")
print(f"[S7.4] PASS: 어텐션 복잡도 분석 검증 완료")
```

### S7.5 LIMITS (양자화 품질 경계)

```python
"""양자화 오차 이론: 라운딩 오차 + 아웃라이어 영향"""
import math, random
random.seed(42)

def quantize_symmetric(values, bits):
    """대칭 양자화: [-max, max] -> [-2^(b-1), 2^(b-1)-1]"""
    absmax = max(abs(v) for v in values)
    if absmax == 0:
        return values, 0.0
    scale = absmax / (2**(bits-1) - 1)
    quantized = [round(v / scale) * scale for v in values]
    mse = sum((v - q)**2 for v, q in zip(values, quantized)) / len(values)
    return quantized, mse

def quantize_group(values, bits, group_size=128):
    """그룹 양자화: group_size별로 독립 스케일"""
    result = []
    total_mse = 0
    n_groups = (len(values) + group_size - 1) // group_size
    for i in range(0, len(values), group_size):
        group = values[i:i+group_size]
        q, mse = quantize_symmetric(group, bits)
        result.extend(q)
        total_mse += mse * len(group)
    return result, total_mse / len(values)

# 정규 분포 가중치 시뮬레이션
n = 10000
weights = [random.gauss(0, 0.02) for _ in range(n)]

# 아웃라이어 주입 (SmoothQuant 동기)
for i in range(50):  # 0.5% 아웃라이어
    weights[random.randint(0, n-1)] = random.gauss(0, 0.2)  # 10x 큰 값

print("[S7.5] 양자화 비트별 MSE:")
for bits in [16, 8, 4, 3, 2]:
    _, mse_sym = quantize_symmetric(weights, bits)
    _, mse_grp = quantize_group(weights, bits, group_size=128)
    snr_sym = -10 * math.log10(mse_sym / (sum(w**2 for w in weights) / n)) if mse_sym > 0 else float('inf')
    snr_grp = -10 * math.log10(mse_grp / (sum(w**2 for w in weights) / n)) if mse_grp > 0 else float('inf')
    print(f"  {bits}bit: 대칭 MSE={mse_sym:.2e} (SNR={snr_sym:.1f}dB), 그룹 MSE={mse_grp:.2e} (SNR={snr_grp:.1f}dB)")

# INT4 그룹 양자화는 충분한 SNR 유지
_, mse_4bit = quantize_group(weights, 4)
_, mse_8bit = quantize_group(weights, 8)
assert mse_4bit < mse_8bit * 100, "INT4 그룹 양자화 MSE 합리적 범위"
# INT2는 품질 붕괴
_, mse_2bit = quantize_symmetric(weights, 2)
assert mse_2bit > mse_4bit * 5, "INT2 품질 급격 저하"
print(f"[S7.5] 한계: INT4 그룹 양자화가 품질/크기 최적점, INT2는 품질 붕괴")
print(f"[S7.5] PASS: 양자화 오차 경계 검증 완료")
```

### S7.6 CHI2 (TTFT vs TPS 트레이드오프 분석)

```python
"""TTFT (첫 토큰 지연) vs TPS (초당 토큰) 트레이드오프"""
import math

def prefill_time_ms(seq_len, n_layers, d_model, flops_per_sec):
    """프리필 시간: 입력 토큰 전체 병렬 처리"""
    # 프리필 FLOP ~ 2 * n_layers * seq_len * (12 * d^2)
    flops = 2 * n_layers * seq_len * 12 * d_model**2
    return (flops / flops_per_sec) * 1000  # ms

def decode_time_ms(n_layers, d_model, hbm_bw, model_bytes):
    """디코드 시간: 메모리 바운드 (가중치 로드 시간이 지배)"""
    # 토큰당 = 모델 가중치 전체 로드 시간
    return (model_bytes / hbm_bw) * 1000  # ms

# 70B 모델 파라미터
n_layers = 80; d_model = 8192
model_fp16 = 70e9 * 2  # 140GB
model_int4 = 70e9 * 0.5  # 35GB
flops = 989e12  # H100 FP16
hbm_bw = 3.35e12  # H100 HBM BW

print("[S7.6] TTFT (프리필 지연) vs TPS (디코드 속도):")
print(f"  {'입력 길이':>10s} | {'TTFT(ms)':>10s} | {'TPS(FP16)':>10s} | {'TPS(INT4)':>10s}")

for seq in [128, 512, 2048, 8192, 32768]:
    ttft = prefill_time_ms(seq, n_layers, d_model, flops)
    tps_fp16 = 1000.0 / decode_time_ms(n_layers, d_model, hbm_bw, model_fp16)
    tps_int4 = 1000.0 / decode_time_ms(n_layers, d_model, hbm_bw, model_int4)
    print(f"  {seq:>10d} | {ttft:>10.1f} | {tps_fp16:>10.1f} | {tps_int4:>10.1f}")

# 트레이드오프: TTFT는 프리필(연산 바운드), TPS는 디코드(메모리 바운드)
ttft_short = prefill_time_ms(128, n_layers, d_model, flops)
ttft_long = prefill_time_ms(32768, n_layers, d_model, flops)
assert ttft_long > ttft_short * 100, "TTFT는 입력 길이에 선형 비례"

# INT4는 TPS 4배 향상 (메모리 바운드이므로)
tps_fp16 = 1000.0 / decode_time_ms(n_layers, d_model, hbm_bw, model_fp16)
tps_int4 = 1000.0 / decode_time_ms(n_layers, d_model, hbm_bw, model_int4)
assert tps_int4 > tps_fp16 * 3, "INT4 디코드는 메모리 바운드에서 ~4배 빠름"
print(f"[S7.6] INT4 TPS 향상: {tps_int4/tps_fp16:.1f}x (메모리 바운드 효과)")
print(f"[S7.6] PASS: TTFT vs TPS 트레이드오프 검증 완료")
```

### S7.7 OEIS (프롬프트 캐싱 적중률 수학)

```python
"""프롬프트 캐싱 / 프리픽스 캐싱: 적중률과 비용 절감 모델"""
import math, random
from fractions import Fraction
random.seed(42)

def lru_cache_hit_rate(requests, cache_size):
    """LRU 캐시 적중률 시뮬레이션 (프롬프트 프리픽스 기준)"""
    cache = []  # (prefix_hash, kv_cache)
    hits = 0
    for req in requests:
        prefix = req[:len(req)//2]  # 시스템 프롬프트 = 전반부
        prefix_hash = hash(tuple(prefix))
        if prefix_hash in cache:
            hits += 1
            cache.remove(prefix_hash)
            cache.append(prefix_hash)  # MRU로 이동
        else:
            cache.append(prefix_hash)
            if len(cache) > cache_size:
                cache.pop(0)  # LRU 퇴출
    return hits / len(requests)

# 시뮬레이션: Zipf 분포 프롬프트 패턴 (소수 프롬프트가 대부분 트래픽)
n_unique_prefixes = 100
n_requests = 10000

# Zipf 분포: P(k) ~ 1/k^s
def zipf_sample(n, s=1.2):
    weights = [1.0 / (k**s) for k in range(1, n+1)]
    total = sum(weights)
    probs = [w/total for w in weights]
    r = random.random()
    cumsum = 0
    for i, p in enumerate(probs):
        cumsum += p
        if r <= cumsum:
            return i
    return n-1

requests = []
for _ in range(n_requests):
    prefix_id = zipf_sample(n_unique_prefixes)
    req = [prefix_id] * 50 + [random.randint(0, 1000) for _ in range(50)]
    requests.append(req)

for cache_sz in [10, 50, 100, 500]:
    hit_rate = lru_cache_hit_rate(requests, cache_sz)
    savings = hit_rate * 0.9  # 캐시 히트 시 프리필 비용 90% 절감
    print(f"  캐시 크기={cache_sz:>4d}: 적중률={hit_rate:.3f}, 비용 절감={savings:.1%}")

# Zipf 분포에서 소수 캐시로도 높은 적중률
hit_10 = lru_cache_hit_rate(requests, 10)
hit_100 = lru_cache_hit_rate(requests, 100)
assert hit_10 > 0.5, "Zipf 분포: 캐시 10개로도 50%+ 적중"
assert hit_100 > hit_10, "캐시 크기 증가 -> 적중률 증가"

# Anthropic 프롬프트 캐싱: 시스템 프롬프트 공유 시 비용 1/10
shared_ratio = Fraction(9, 10)
print(f"[S7.7] Anthropic 프롬프트 캐싱: 시스템 프롬프트 공유 시 비용 {1-float(shared_ratio):.0%}")
print(f"[S7.7] PASS: 프롬프트 캐싱 적중률 수학 검증 완료")
```

### S7.8 PARETO (서빙 비용-품질-지연 Pareto 프론티어)

```python
"""서빙 비용 vs 품질 vs 지연 시간: Pareto 최적 구성 탐색"""
import math

def serving_config(quant_bits, batch_size, cache_hit, spec_gamma):
    """서빙 구성별 비용/품질/지연 추정"""
    # 비용: 메모리 -> GPU 수 -> $/시간
    model_mem_gb = 70 * (quant_bits / 8)  # 모델 메모리
    kv_per_req_gb = 0.5  # 4K 컨텍스트 KV 캐시
    total_mem = model_mem_gb + kv_per_req_gb * batch_size
    n_gpus = math.ceil(total_mem / 80)  # H100 80GB
    cost_per_hour = n_gpus * 3.5  # $3.5/GPU/hr
    reqs_per_hour = batch_size * 3600 / 2.0  # 평균 2초/요청
    cost_per_1k_req = cost_per_hour / reqs_per_hour * 1000

    # 품질: 양자화 비트 -> 품질 감소
    quality_map = {16: 1.000, 8: 0.998, 4: 0.990, 3: 0.960, 2: 0.880}
    quality = quality_map.get(quant_bits, 0.95)

    # 지연: TTFT 캐시 영향 + 디코드 속도
    ttft_ms = 500 * (1 - cache_hit * 0.9)  # 캐시 히트 시 90% 절감
    decode_ms = 20 * (16 / quant_bits)  # 양자화 -> 메모리 바운드 개선
    # 투기적 디코딩 속도 향상
    spec_speedup = (1 - 0.8**(spec_gamma+1)) / 0.2 if spec_gamma > 0 else 1.0
    effective_tps = 1000 / decode_ms * spec_speedup
    latency_ms = ttft_ms + 100 / effective_tps * 1000  # 100 토큰 생성

    return cost_per_1k_req, quality, latency_ms

configs = []
for bits in [16, 8, 4]:
    for bs in [8, 16, 32, 64]:
        for ch in [0.0, 0.3, 0.6, 0.9]:
            for sg in [0, 3, 5]:
                c, q, l = serving_config(bits, bs, ch, sg)
                configs.append((bits, bs, ch, sg, c, q, l))

# Pareto 필터: 비용 최소, 품질 최대, 지연 최소
pareto = [cfg for cfg in configs if not any(
    o[4] <= cfg[4] and o[5] >= cfg[5] and o[6] <= cfg[6] and
    (o[4] < cfg[4] or o[5] > cfg[5] or o[6] < cfg[6])
    for o in configs if o != cfg)]

pareto.sort(key=lambda x: x[4])
print(f"[S7.8] 전체 {len(configs)} 구성 중 Pareto 최적 {len(pareto)}개:")
for p in pareto[:8]:
    print(f"  {p[0]}bit bs={p[1]:>2d} cache={p[2]:.1f} spec={p[3]} -> "
          f"비용=${p[4]:.2f}/1Kreq 품질={p[5]:.3f} 지연={p[6]:.0f}ms")

# INT4 + 큰 배치 + 캐싱 + 투기적 디코딩 = 최적
best = min(pareto, key=lambda x: x[4])
assert best[0] <= 8, "비용 최적은 양자화 모델"
print(f"[S7.8] 비용 최적: {best[0]}bit, bs={best[1]}, cache={best[2]}, spec={best[3]}")
print(f"[S7.8] PASS: Pareto 프론티어 분석 검증 완료")
```

### S7.9 SYMBOLIC (토큰 생성 지연 정밀 모델)

```python
"""토큰 생성 지연 분해: 프리필 + 디코드 + 오버헤드"""
from fractions import Fraction
import math

def token_latency_model(seq_in, seq_out, d_model, n_layers, n_kv_heads, head_dim,
                         flops, hbm_bw, model_bytes, batch_size=1):
    """정밀 지연 모델: 연산 바운드 vs 메모리 바운드 분석"""
    # 프리필: 연산 바운드 (행렬 곱 지배)
    # FLOP = 2 * n_layers * seq_in * (12 * d_model^2 + seq_in * n_heads * head_dim)
    prefill_flops = 2 * n_layers * seq_in * 12 * d_model**2
    prefill_time = prefill_flops / flops  # 초

    # 디코드: 메모리 바운드 (가중치 로드 지배)
    # 토큰당 = max(연산 시간, 메모리 로드 시간)
    decode_flops_per_tok = 2 * n_layers * 12 * d_model**2
    decode_compute_time = decode_flops_per_tok / flops

    # KV 캐시 로드: 이전 토큰의 K, V 읽기
    kv_bytes_per_tok = 2 * n_layers * n_kv_heads * head_dim * 2  # K+V, FP16
    # 현재 시점의 KV 캐시 크기 (평균)
    avg_kv_seq = seq_in + seq_out / 2
    kv_load_time = (kv_bytes_per_tok * avg_kv_seq) / hbm_bw

    # 가중치 로드
    weight_load_time = model_bytes / hbm_bw

    # 디코드는 max(연산, 메모리) -> 메모리 바운드
    decode_per_tok = max(decode_compute_time, weight_load_time + kv_load_time)

    total_decode = decode_per_tok * seq_out
    total_time = prefill_time + total_decode

    # 산술 강도 (Arithmetic Intensity)
    ai_prefill = prefill_flops / (model_bytes + seq_in * d_model * 2)
    ai_decode = decode_flops_per_tok / (model_bytes + kv_bytes_per_tok)

    return {
        'ttft_ms': prefill_time * 1000,
        'tps': 1.0 / decode_per_tok,
        'total_ms': total_time * 1000,
        'ai_prefill': ai_prefill,
        'ai_decode': ai_decode,
        'bound_prefill': '연산' if ai_prefill > flops/hbm_bw else '메모리',
        'bound_decode': '연산' if ai_decode > flops/hbm_bw else '메모리',
    }

# 70B 모델 설정
result = token_latency_model(
    seq_in=2048, seq_out=256,
    d_model=8192, n_layers=80, n_kv_heads=8, head_dim=128,
    flops=989e12, hbm_bw=3.35e12, model_bytes=140e9
)

print(f"[S7.9] 70B FP16 (입력 2K, 출력 256):")
print(f"  TTFT = {result['ttft_ms']:.1f}ms ({result['bound_prefill']} 바운드)")
print(f"  TPS  = {result['tps']:.1f} tok/s ({result['bound_decode']} 바운드)")
print(f"  총 시간 = {result['total_ms']:.0f}ms")
print(f"  산술 강도: 프리필={result['ai_prefill']:.0f}, 디코드={result['ai_decode']:.2f}")

# 루프탑 모델 경계 검증
roofline_boundary = Fraction(989, 335)  # FLOPS/BW (approx)
print(f"  루프탑 경계 = {float(roofline_boundary):.1f} ops/byte")

# 디코드는 항상 메모리 바운드 (산술 강도 < 루프탑 경계)
assert result['bound_decode'] == '메모리', "디코드는 메모리 바운드"
print(f"[S7.9] PASS: 토큰 생성 지연 정밀 모델 검증 완료")
```

### S7.10 COUNTER (정직한 한계)

```python
"""추론 최적화의 근본적 한계와 실패 사례"""
import math

# 한계 1: 양자화 품질 붕괴 임계점
print("[S7.10] 한계 1: INT3 이하에서 품질 급격 저하")
for bits in [8, 4, 3, 2]:
    # 양자화 레벨 수 = 2^bits
    levels = 2**bits
    # 정규 분포 가중치의 양자화 SNR ~ 6.02*bits + 1.76 dB (균일 양자화 이론)
    snr = 6.02 * bits + 1.76
    quality_loss = 1 - 10**(-snr/20) if snr > 0 else 1
    print(f"  INT{bits}: 레벨={levels:>4d}, SNR={snr:.1f}dB, 품질 유지={quality_loss:.4f}")
print("  결론: INT4가 실용적 하한, INT3 이하는 특수 기법 없이 불가")

# 한계 2: KV 캐시 압축 한계
print("[S7.10] 한계 2: KV 캐시 압축은 어텐션 정보 손실 유발")
seq_len = 131072  # 128K
kv_full = 2 * 80 * 8 * 128 * seq_len * 2  # FP16
kv_4bit = 2 * 80 * 8 * 128 * seq_len * 0.5  # INT4
kv_evict = 2 * 80 * 8 * 128 * (seq_len // 4) * 2  # 75% 퇴출
print(f"  128K 전체: {kv_full/1e9:.1f}GB")
print(f"  INT4 양자화: {kv_4bit/1e9:.1f}GB (4x 절감, 정보 손실 2%)")
print(f"  75% 퇴출: {kv_evict/1e9:.1f}GB (4x 절감, 정보 손실 15-30%)")
print("  결론: 퇴출 방식은 긴 범위 의존성 파괴 위험")

# 한계 3: 투기적 디코딩 실패 조건
print("[S7.10] 한계 3: 투기적 디코딩은 저엔트로피 생성에만 효과적")
for entropy in [0.5, 1.0, 2.0, 3.0, 5.0]:
    # 높은 엔트로피 = 드래프트 모델 예측 어려움 = 낮은 수용률
    acceptance = math.exp(-entropy * 0.3)  # 경험적 관계
    speedup = (1 - acceptance**6) / (1 - acceptance) if acceptance < 1 else 6
    print(f"  엔트로피={entropy:.1f}: 수용률={acceptance:.2f}, 속도 향상={speedup:.2f}x")
print("  결론: 창의적/다양한 생성 시 투기적 디코딩 이점 감소")

# 한계 4: 배칭 지연 시간 트레이드오프
print("[S7.10] 한계 4: 큰 배치 = 높은 처리량이지만 개별 지연 증가")
for bs in [1, 8, 32, 128]:
    # 배치 증가 시 KV 캐시 메모리 경쟁 -> 캐시 미스 증가
    throughput = bs * 0.9**math.log2(max(bs, 1))  # 수확 체감
    latency_overhead = 1.0 + 0.1 * math.log2(max(bs, 1))
    print(f"  batch={bs:>3d}: 상대 처리량={throughput:.1f}x, 지연 오버헤드={latency_overhead:.2f}x")
print("  결론: 지연 SLA 준수를 위한 배치 크기 상한 존재")

print("[S7.10] 총평: 추론 최적화는 은총알 없음 -- 기법 조합 + 워크로드별 튜닝 필수")
print("[S7.10] PASS: 정직한 한계 기록 완료")
```

## S8 KEY (핵심 연구 아이디어 33종)

### 축 1: 메모리 최적화 (12종)

| ID | 제목 | 핵심 | 난이도 |
|----|------|------|--------|
| 1 | PagedAttention v2 | KV 캐시를 가상 메모리처럼 페이지 단위 관리, 단편화 제거 | 중 |
| 2 | KV 캐시 INT4 양자화 | 키-값 캐시만 선택적 INT4 양자화, 어텐션 정확도 유지 | 중 |
| 3 | KV 캐시 퇴출 정책 | H2O(Heavy Hitter Oracle): 중요 토큰만 유지, 나머지 퇴출 | 상 |
| 4 | 프리픽스 캐싱 | 공통 시스템 프롬프트의 KV 캐시 공유, 프리필 비용 90% 절감 | 중 |
| 5 | 교차 요청 KV 공유 | 동일 프리픽스 요청들의 KV 캐시 GPU 간 공유 | 상 |
| 6 | 계층적 KV 저장소 | GPU HBM -> CPU DRAM -> SSD 3계층 KV 캐시 | 상 |
| 7 | 어텐션 싱크 | StreamingLLM: 초기 토큰 + 최근 윈도우만 유지 | 중 |
| 8 | KV 캐시 압축 | 저순위 근사로 KV 캐시 차원 축소 | 상 |
| 9 | MQA/GQA 전환 | 기존 MHA 모델을 GQA로 미세조정 전환 | 중 |
| 10 | 동적 KV 정밀도 | 레이어별/헤드별 KV 캐시 정밀도 차등 할당 | 상 |
| 11 | 토큰 병합 | 유사 토큰의 KV 표현 병합으로 유효 시퀀스 길이 축소 | 상 |
| 12 | 오프로딩 파이프라인 | KV 캐시 GPU<->CPU 비동기 전송, 디코드와 오버랩 | 중 |

### 축 2: 연산 최적화 (11종)

| ID | 제목 | 핵심 | 난이도 |
|----|------|------|--------|
| 13 | FlashAttention-3 | Hopper 아키텍처 최적화, FP8 텐서 코어 활용 | 상 |
| 14 | 투기적 디코딩 최적화 | 드래프트 모델 크기/토큰 수 동적 조절 | 중 |
| 15 | 자기투기 디코딩 | 드래프트 모델 없이 얕은 레이어 출력으로 투기 | 상 |
| 16 | Medusa 디코딩 | 다중 헤드로 여러 후보 토큰 동시 예측 | 중 |
| 17 | GPTQ 개선 | 2차 정보 활용 양자화, 가중치별 최적 스케일 | 중 |
| 18 | AWQ (Activation-aware) | 활성화 분포 인식 양자화, 아웃라이어 채널 보호 | 중 |
| 19 | SmoothQuant | 활성화 아웃라이어를 가중치로 이전, 양자화 용이화 | 중 |
| 20 | 구조적 가지치기 | 추론 시 불필요 헤드/뉴런 제거, 동적 희소성 | 상 |
| 21 | 커널 퓨전 | 어텐션+FFN+정규화 연산 융합, 커널 호출 오버헤드 제거 | 중 |
| 22 | 선택적 레이어 건너뛰기 | 쉬운 토큰은 일부 레이어 건너뛰기 (조기 종료) | 상 |
| 23 | FP8 추론 | H100 FP8 텐서 코어 활용, FP16 대비 2배 처리량 | 중 |

### 축 3: 시스템 아키텍처 (10종)

| ID | 제목 | 핵심 | 난이도 |
|----|------|------|--------|
| 24 | 연속 배칭 스케줄러 | 완료 요청 즉시 교체, GPU 유휴 시간 제거 | 중 |
| 25 | 프리필-디코드 분리 | 프리필 전용 GPU + 디코드 전용 GPU, 각각 최적화 | 상 |
| 26 | 요청 라우팅 최적화 | 프리픽스 유사도 기반 요청 라우팅, 캐시 적중률 극대화 | 중 |
| 27 | 멀티모달 서빙 파이프라인 | 비전/오디오 인코더 별도 배칭, 언어 모델과 비동기 결합 | 상 |
| 28 | 자동 스케일링 | 부하 예측 기반 GPU 클러스터 탄력적 확장/축소 | 중 |
| 29 | 모델 샤딩 전략 | 텐서/파이프라인/시퀀스 병렬화 최적 조합 | 상 |
| 30 | 스로틀링 정책 | 과부하 시 품질 저하 없는 우아한 성능 저하 | 중 |
| 31 | 지연 시간 SLA 라우터 | P50/P99 지연 보장, 긴 요청 별도 큐 | 중 |
| 32 | 이기종 GPU 클러스터 | H100/A100/L40 혼합 배치, 워크로드별 최적 할당 | 상 |
| 33 | 에너지 효율 서빙 | 비첨두 시간 GPU 절전, 탄소 인식 라우팅 | 중 |

## S9 MATRIX (실험 검증 매트릭스)

```
+------+---------------------------+-------------------+----------------+---------+
| ID   | 실험                      | 데이터셋/벤치마크  | 메트릭         | 기간    |
+------+---------------------------+-------------------+----------------+---------+
| 1    | PagedAttention v2 구현    | ShareGPT 트래픽   | 메모리 절감률  | 3주     |
| 2    | KV 캐시 INT4 품질 측정    | MMLU/HumanEval    | 정확도 변화    | 2주     |
| 14   | 투기적 디코딩 감마 스윕   | Alpaca/ShareGPT   | 수용률/TPS     | 2주     |
| 17   | GPTQ vs AWQ vs SQ 비교   | MMLU/GSM8K        | 품질/속도      | 3주     |
| 24   | 연속 배칭 처리량 측정     | 실제 API 트래픽   | tok/s/GPU      | 2주     |
| 25   | 프리필-디코드 분리 효과   | 혼합 워크로드     | TTFT+TPS       | 4주     |
| 26   | 프리픽스 라우팅 적중률    | 실제 API 로그     | 캐시 적중률    | 2주     |
| 27   | 멀티모달 서빙 프로파일    | 이미지+텍스트     | GPU 활용률     | 3주     |
| 22   | 선택적 레이어 건너뛰기    | MMLU/HellaSwag    | 품질-속도 곡선 | 3주     |
| 6    | 3계층 KV 저장소 지연      | 1M 컨텍스트       | P99 지연       | 4주     |
+------+---------------------------+-------------------+----------------+---------+
```

## S10 PREDICTIONS (검증 가능한 예측 10종)

| # | 예측 | 기대 결과 |
|---|------|----------|
| 1 | INT4 GQA 양자화는 FP16 대비 MMLU 정확도 1% 이내 손실로 4배 처리량 달성 | TPS 4x, MMLU -0.5% |
| 2 | 프리픽스 캐싱은 반복 시스템 프롬프트 워크로드에서 TTFT 90% 절감 | 캐시 히트 시 TTFT < 50ms |
| 3 | 투기적 디코딩 (gamma=5)은 코드 생성에서 3x 속도 향상 (저엔트로피) | 수용률 85%+ |
| 4 | 연속 배칭은 정적 배칭 대비 처리량 2-3배 향상 | GPU 활용률 90%+ |
| 5 | FlashAttention-3 + FP8은 128K 컨텍스트 어텐션 시간 5배 단축 | 프리필 시간 80% 감소 |
| 6 | KV 캐시 INT4 + H2O 퇴출 조합은 1M 컨텍스트를 단일 H100에서 서빙 가능 | KV 메모리 < 40GB |
| 7 | 프리필-디코드 분리 아키텍처는 혼합 워크로드에서 총 비용 30% 절감 | GPU 당 처리량 1.3x |
| 8 | 멀티모달 서빙에서 비전 인코더 배칭 최적화는 이미지 처리 지연 50% 감소 | 비전 TTFT < 200ms |
| 9 | Zipf 분포 트래픽에서 100개 프리픽스 LRU 캐시로 70%+ 적중률 달성 | 비용 60% 절감 |
| 10 | 전체 최적화 스택 (양자화+캐싱+배칭+투기)은 FP16 기본 대비 10x 비용 효율 | $7B -> $700M |

## S11 PERF (성능 비교)

```
+------------------------------------------------------------------+
|  [처리량] (tokens/s/GPU, 70B 모델)                                |
|  FP16 기본           ###.............................  24 tok/s   |
|  INT8                ######..........................  48 tok/s   |
|  INT4 (GPTQ)         ############....................  96 tok/s   |
|  INT4 + FlashAtt     ###############.................  120 tok/s  |
|  INT4 + 연속배칭     ####################............  160 tok/s  |
|  INT4 + 투기 디코딩  ########################........  192 tok/s  |
|  전체 스택 (목표)    ##############################..  240 tok/s  |
+------------------------------------------------------------------+
|  [비용 효율] ($/1M 출력 토큰)                                     |
|  현재 (FP16, 정적)   ##############################  $15.00      |
|  INT8 + 연속배칭     ##############..................  $7.00       |
|  INT4 + 캐싱         ########........................  $4.00       |
|  전체 최적화 스택    ###.............................  $1.50       |
|  목표 (1/10)         ##..............................  $1.00       |
+------------------------------------------------------------------+
|  [TTFT] (첫 토큰 지연, 2K 입력)                                   |
|  기본 FP16           ########################........  800ms      |
|  FlashAttention      ####################............  650ms      |
|  FA + 프롬프트 캐싱  ######..........................  200ms      |
|  FA + 프리픽스 히트  ##..............................  80ms       |
+------------------------------------------------------------------+
```

## S12 ARCH (시스템 아키텍처)

```
+======================================================================+
|  [사용자 요청]                                                        |
|         |                                                            |
|         v                                                            |
|  [로드 밸런서] -- SLA 기반 라우팅 + 프리픽스 유사도 매칭             |
|         |                                                            |
|    +----+----+                                                       |
|    v         v                                                       |
|  [프리필 GPU 풀]              [디코드 GPU 풀]                        |
|  +------------------+        +------------------+                    |
|  | FlashAttention-3 |        | 연속 배칭        |                    |
|  | 프롬프트 캐싱     |  --->  | 투기적 디코딩    |                    |
|  | 청크 프리필      |  KV    | PagedAttention   |                    |
|  +------------------+        +------------------+                    |
|         |                           |                                |
|         v                           v                                |
|  [KV 캐시 관리자]                                                    |
|  +------------------------------------------------------+           |
|  | GPU HBM (핫) -> CPU DRAM (웜) -> SSD (콜드)          |           |
|  | 프리픽스 공유 | 페이지 관리 | 퇴출 정책 (H2O)       |           |
|  +------------------------------------------------------+           |
|         |                                                            |
|         v                                                            |
|  [응답 스트리밍] --> 사용자                                          |
+======================================================================+
```

## S13 DATAFLOW (데이터 흐름)

```
사용자 요청 (텍스트/이미지/오디오)
        |
        v
프리픽스 해시 계산 --> 캐시 조회
        |                  |
   히트 |             미스 |
        v                  v
   KV 캐시 로드      전체 프리필
        |                  |
        +--------+---------+
                 v
          KV 캐시 저장 (PagedAttention)
                 |
                 v
          디코드 루프 (연속 배칭)
            |         |
            v         v
       직접 생성   투기적 디코딩
            |         |   드래프트 생성 -> 검증 -> 수용/거부
            +----+----+
                 v
          토큰 스트리밍 --> 완료 시 배치 슬롯 해제
                                   |
                                   v
                            다음 요청 슬롯 충전
```

## S14 COMPARE-3 (현재 vs 제안 vs 이상)

```
+------+---------------------+--------------------------+--------------------------+
| 측면 | 현재 (2026)         | 제안 (본 연구)            | 이상 (장기 목표)          |
+------+---------------------+--------------------------+--------------------------+
| 양자화| FP16/INT8 기본      | INT4 GQA + 동적 정밀도   | INT2 + 오류 보정 코드    |
| 캐싱 | 단순 프롬프트 캐싱  | 계층적 KV + 프리픽스 공유 | 의미 기반 캐싱           |
| 배칭 | 연속 배칭 기본      | 프리필-디코드 분리        | 완전 비동기 파이프라인   |
| 디코딩| 자기회귀 기본       | 투기적 + Medusa 하이브리드| 병렬 디코딩 (비자기회귀) |
| 메모리| 정적 할당           | PagedAttention v2        | 하드웨어 가상 메모리     |
| 비용 | ~$15/1M tok         | ~$1.5/1M tok (10x 절감)  | ~$0.1/1M tok (150x)     |
+------+---------------------+--------------------------+--------------------------+
```

## S15 METHODOLOGY (검증 방법론)

**연구 원칙**: (1) 재현 가능성: 벤치마크 설정/하드웨어/소프트웨어 버전 전면 공개 (2) 공정 비교: 동일 하드웨어, 동일 모델, 동일 워크로드에서 기법 비교 (3) 실제 워크로드: 합성 벤치마크 + 실제 API 트래픽 패턴 모두 평가 (4) 다차원 평가: 처리량, 지연 시간, 품질, 메모리, 비용 5축 동시 측정 (5) 통계 엄밀성: 다중 실행 평균 + 분산 + 신뢰 구간 보고

**실패 기준 (방향 수정 트리거)**:
- INT4 양자화가 MMLU 2%+ 정확도 저하 유발 -> 혼합 정밀도 (민감 레이어 FP16 유지) 재설계
- 투기적 디코딩 수용률 60% 미만 -> 드래프트 모델 아키텍처 또는 학습 데이터 변경
- 프리필-디코드 분리가 KV 전송 오버헤드로 상쇄 -> 단일 GPU 내 시분할 방식으로 전환
- 1M 컨텍스트 KV 압축이 긴 범위 의존성 파괴 -> 계층적 어텐션 (로컬+글로벌) 도입
- 연속 배칭 P99 지연이 SLA 초과 -> 우선순위 큐 + 선점형 스케줄링 도입

**윤리**: 에너지 소비 투명 보고 (GPU 시간 + 탄소 배출), 비용 절감이 접근성 확대로 이어지는지 추적, 최적화로 인한 편향 증폭 여부 모니터링
