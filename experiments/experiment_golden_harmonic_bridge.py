"""
Experiment: Golden Harmonic Bridge
====================================
H(12) - ln(12) = 0.618304 ≈ 1/φ (0.618034), 오차 0.04%.
질문: H(n) - ln(n) = 1/φ를 정확히 만족하는 n은 무엇인가?
      그 n이 n=6 산술로 표현 가능한가?

알고리즘:
  1. H(n) - ln(n)을 n=1..10000까지 계산
  2. |H(n) - ln(n) - 1/φ| 최소인 n 탐색
  3. 연속 근사: γ + 1/(2n) ≈ 1/φ → n ≈ 12.17 → sigma(6)=12 정수 최적
  4. sigma(m)에 대해 m=1..100 스캔
  5. 몬테카를로 유일성 검증
"""

import math
import random

# === 상수 ===
PHI = (1 + math.sqrt(5)) / 2  # 황금비
INV_PHI = 1.0 / PHI            # 1/φ = 0.6180339887...
EULER_GAMMA = 0.5772156649015329  # 오일러-마스케로니 상수


def harmonic(n):
    """조화급수 H(n) = 1 + 1/2 + ... + 1/n"""
    s = 0.0
    for k in range(1, n + 1):
        s += 1.0 / k
    return s


def sigma_func(m):
    """약수 합 sigma(m)"""
    s = 0
    for d in range(1, m + 1):
        if m % d == 0:
            s += d
    return s


# ======================================================================
# 단계 1: H(n) - ln(n) 계산 (n=1..10000)
# ======================================================================
print("=" * 70)
print("단계 1: H(n) - ln(n) 을 n=1..10000 까지 계산")
print("=" * 70)

best_n = 1
best_err = float("inf")
results_top = []

H_acc = 0.0
for n in range(1, 10001):
    H_acc += 1.0 / n
    diff = H_acc - math.log(n) if n > 0 else float("inf")
    err = abs(diff - INV_PHI)
    if err < best_err:
        best_err = err
        best_n = n
    if n <= 20 or n == 12 or err < 0.001:
        results_top.append((n, diff, err))

print(f"\n1/phi = {INV_PHI:.10f}")
print(f"\nn=1..20 에서 H(n)-ln(n) 값:")
for n, diff, err in results_top[:20]:
    marker = " << sigma(6)" if n == 12 else ""
    print(f"  n={n:>5d}  H(n)-ln(n) = {diff:.10f}  |오차| = {err:.10f}{marker}")


# ======================================================================
# 단계 2: 최적 n 보고
# ======================================================================
print("\n" + "=" * 70)
print("단계 2: |H(n) - ln(n) - 1/phi| 최소인 n")
print("=" * 70)

H_best = harmonic(best_n)
diff_best = H_best - math.log(best_n)
print(f"\n  최적 n = {best_n}")
print(f"  H({best_n}) - ln({best_n}) = {diff_best:.10f}")
print(f"  1/phi                      = {INV_PHI:.10f}")
print(f"  오차                       = {abs(diff_best - INV_PHI):.2e}")
print(f"  상대오차                   = {abs(diff_best - INV_PHI) / INV_PHI * 100:.6f}%")


# ======================================================================
# 단계 3: n=12 = sigma(6) 확인
# ======================================================================
print("\n" + "=" * 70)
print("단계 3: n=12 = sigma(6) 최적 여부")
print("=" * 70)

H12 = harmonic(12)
diff12 = H12 - math.log(12)
err12 = abs(diff12 - INV_PHI)

print(f"\n  sigma(6) = {sigma_func(6)}")
print(f"  H(12) - ln(12) = {diff12:.10f}")
print(f"  1/phi           = {INV_PHI:.10f}")
print(f"  오차            = {err12:.2e}")
print(f"  상대오차        = {err12 / INV_PHI * 100:.6f}%")

if best_n == 12:
    print("\n  결론: n=12 = sigma(6)이 n=1..10000 범위에서 정수 최적!")
else:
    H_bn = harmonic(best_n)
    diff_bn = H_bn - math.log(best_n)
    err_bn = abs(diff_bn - INV_PHI)
    print(f"\n  n={best_n}이 더 나음 (오차 {err_bn:.2e} vs n=12 오차 {err12:.2e})")


# ======================================================================
# 단계 4: 연속 근사 분석
# ======================================================================
print("\n" + "=" * 70)
print("단계 4: 연속 근사 -- gamma + 1/(2n) = 1/phi")
print("=" * 70)

gap = INV_PHI - EULER_GAMMA
n_continuous = 1.0 / (2.0 * gap)

print(f"\n  오일러-마스케로니 gamma = {EULER_GAMMA:.10f}")
print(f"  1/phi                  = {INV_PHI:.10f}")
print(f"  1/phi - gamma          = {gap:.10f}")
print(f"  n = 1/(2(1/phi - gamma)) = {n_continuous:.4f}")
print(f"  가장 가까운 정수       = {round(n_continuous)}")
print(f"  sigma(6)               = 12")
print(f"\n  이론적 최적점 n ~ {n_continuous:.4f} -> 정수 최적 = {round(n_continuous)} = sigma(6)")


# ======================================================================
# 단계 5: 점근 전개 정밀도
# ======================================================================
print("\n" + "=" * 70)
print("단계 5: 점근 전개 H(n) ~ ln(n) + gamma + 1/(2n) - 1/(12n^2) + ...")
print("=" * 70)

for n in [12, 11, 13, 6, 24, 100]:
    approx_1 = EULER_GAMMA + 1.0 / (2 * n)
    approx_2 = approx_1 - 1.0 / (12 * n * n)
    approx_3 = approx_2 + 1.0 / (120 * n**4)
    H_exact = harmonic(n)
    exact_diff = H_exact - math.log(n)
    print(f"  n={n:>3d}: 실제={exact_diff:.8f}  "
          f"근사2항={approx_1:.8f}  근사3항={approx_2:.8f}  "
          f"|실제-1/phi|={abs(exact_diff - INV_PHI):.2e}")


# ======================================================================
# 단계 6: sigma(m) 스캔 (m=1..100)
# ======================================================================
print("\n" + "=" * 70)
print("단계 6: sigma(m) 에 대한 H(sigma(m))-ln(sigma(m)) 스캔 (m=1..100)")
print("=" * 70)

sigma_results = []
for m in range(1, 101):
    sm = sigma_func(m)
    if sm < 2:
        continue
    Hsm = harmonic(sm)
    diff_sm = Hsm - math.log(sm)
    err_sm = abs(diff_sm - INV_PHI)
    sigma_results.append((m, sm, diff_sm, err_sm))

sigma_results.sort(key=lambda x: x[3])

print(f"\n  상위 10개 (1/phi에 가장 근접한 sigma(m)):")
print(f"  {'m':>5s}  {'sigma(m)':>10s}  {'H(s)-ln(s)':>14s}  {'|오차|':>12s}  {'상대오차%':>10s}")
print(f"  {'-'*5}  {'-'*10}  {'-'*14}  {'-'*12}  {'-'*10}")
for m, sm, d, e in sigma_results[:10]:
    marker = " << n=6" if m == 6 else ""
    print(f"  {m:>5d}  {sm:>10d}  {d:>14.10f}  {e:>12.2e}  {e/INV_PHI*100:>9.6f}%{marker}")

# m=6 순위 확인
rank_6 = next(i + 1 for i, (m, _, _, _) in enumerate(sigma_results) if m == 6)
print(f"\n  m=6 순위: {rank_6}위 / {len(sigma_results)}개")


# ======================================================================
# 단계 7: 몬테카를로 유일성 검증
# ======================================================================
print("\n" + "=" * 70)
print("단계 7: 몬테카를로 -- 무작위 n이 n=12보다 나은 비율")
print("=" * 70)

random.seed(42)
N_TRIALS = 100000
count_better = 0

# n=12 오차 미리 계산
err_ref = abs(harmonic(12) - math.log(12) - INV_PHI)

# H(n) 누적 테이블 구축
H_table = [0.0] * 1001
for i in range(1, 1001):
    H_table[i] = H_table[i - 1] + 1.0 / i

for _ in range(N_TRIALS):
    rn = random.randint(1, 1000)
    err_rn = abs(H_table[rn] - math.log(rn) - INV_PHI) if rn > 0 else float("inf")
    if err_rn < err_ref:
        count_better += 1

ratio = count_better / N_TRIALS * 100
print(f"\n  시행 횟수: {N_TRIALS:,}")
print(f"  n=12 오차: {err_ref:.2e}")
print(f"  무작위 n 이 n=12 보다 나은 횟수: {count_better}")
print(f"  비율: {ratio:.3f}%")
print(f"\n  해석: [1,1000] 범위 무작위 정수 중 {ratio:.3f}% 만이 n=12보다 1/phi에 가까움")


# ======================================================================
# 최종 요약
# ======================================================================
print("\n" + "=" * 70)
print("최종 요약: Golden Harmonic Bridge")
print("=" * 70)
print(f"""
  핵심 발견:
    H(n) - ln(n) -> gamma (오일러-마스케로니) 수렴에서
    1/phi와의 간극을 정확히 메우는 정수가 n = 12 = sigma(6).

  수학적 경로:
    H(n) ~ ln(n) + gamma + 1/(2n) - 1/(12n^2) + ...
    gamma + 1/(2n) = 1/phi  ->  n = 1/(2*(1/phi - gamma)) ~ {n_continuous:.4f}
    정수 최적 = {round(n_continuous)} = sigma(6)

  n=6 연결:
    sigma(6) = 1+2+3+6 = 12
    12는 H(n)-ln(n) = 1/phi 의 정수 최적해
    phi (황금비)와 sigma (약수합)이 조화급수를 통해 연결

  정밀도:
    H(12) - ln(12) = {diff12:.10f}
    1/phi           = {INV_PHI:.10f}
    상대오차        = {err12/INV_PHI*100:.4f}%

  유일성:
    m=1..100 중 sigma(m) 기준 {rank_6}위
    무작위 대비 {100-ratio:.2f}% 확률로 n=12이 우월
""")
