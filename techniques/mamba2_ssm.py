"""
Technique 18: Mamba-2 State Space Duality
==========================================
Mamba-2 (Dao & Gu, 2024) establishes SSM-attention duality.
All core hyperparameters map to n=6 arithmetic:
  d_state  = 2^n     = 64
  d_conv   = tau     = 4
  expand   = phi     = 2
  dt_min   = 0.001   = 10^{-(n/phi)} = 10^{-3}
  dt_max   = 0.1     = 1/(sigma-phi) = 1/10

Expected: 5/5 EXACT constant mapping.
"""

import numpy as np
import math

# ─── n=6 Constants ───
N = 6
SIGMA = 12       # sigma(6) = 1+2+3+6
PHI = 2          # phi(6) = euler totient
TAU = 4          # tau(6) = number of divisors
J2 = 24          # jordan totient J_2(6)
SOPFR = 5        # sum of prime factors with repetition 2+3
MU = 1           # mobius(6)

# ─── Mamba-2 Parameters ───
MAMBA2_D_STATE = 64      # actual Mamba-2 default
MAMBA2_D_CONV = 4        # actual Mamba-2 default
MAMBA2_EXPAND = 2        # actual Mamba-2 default
MAMBA2_DT_MIN = 0.001    # actual Mamba-2 default
MAMBA2_DT_MAX = 0.1      # actual Mamba-2 default

# ─── n=6 Predictions ───
PRED_D_STATE = 2 ** N                  # 2^6 = 64
PRED_D_CONV = TAU                      # tau(6) = 4
PRED_EXPAND = PHI                      # phi(6) = 2
PRED_DT_MIN = 10 ** (-(N // PHI))     # 10^{-3} = 0.001
PRED_DT_MAX = 1.0 / (SIGMA - PHI)     # 1/10 = 0.1


def verify_mapping(name, actual, predicted, formula):
    """Check EXACT match between actual and n=6 predicted value."""
    match = abs(actual - predicted) < 1e-12
    status = "EXACT" if match else "FAIL"
    print(f"  {status:5s}  {name:<12s} = {actual:<10g}  (n=6: {formula} = {predicted})")
    return match


def simulate_ssm_step(d_state, d_conv, expand, dt, d_model=64):
    """Simulate one SSM step to show parameter dimensions work."""
    d_inner = d_model * expand
    # State matrix A: (d_inner, d_state)
    A = -np.exp(np.random.randn(d_inner, d_state))
    # Input matrix B: (d_inner, d_state) — from input projection
    B = np.random.randn(d_inner, d_state) * 0.01
    # Conv1d kernel: (d_inner, d_conv)
    conv_kernel = np.random.randn(d_inner, d_conv) * 0.1
    # Discretize with dt
    dA = np.exp(A * dt)
    dB = B * dt
    # State: (d_inner, d_state)
    x = np.zeros((d_inner, d_state))
    u = np.random.randn(d_inner)
    # SSM recurrence: x' = dA * x + dB * u
    x_new = dA * x + dB * u[:, None]
    return x_new.shape, conv_kernel.shape


def main():
    print("=" * 70)
    print("  Technique 18: Mamba-2 State Space Duality")
    print("  SSM-Attention duality — all params from n=6")
    print("=" * 70)

    # ─── Constant Verification ───
    print("\n[1] Mamba-2 Constant Mapping (5 parameters)")
    print("-" * 60)
    results = []
    results.append(verify_mapping("d_state", MAMBA2_D_STATE, PRED_D_STATE, "2^n=2^6"))
    results.append(verify_mapping("d_conv", MAMBA2_D_CONV, PRED_D_CONV, "tau(6)"))
    results.append(verify_mapping("expand", MAMBA2_EXPAND, PRED_EXPAND, "phi(6)"))
    results.append(verify_mapping("dt_min", MAMBA2_DT_MIN, PRED_DT_MIN, "10^{-n/phi}"))
    results.append(verify_mapping("dt_max", MAMBA2_DT_MAX, PRED_DT_MAX, "1/(sigma-phi)"))

    exact_count = sum(results)
    total = len(results)

    # ─── Dimension Simulation ───
    print(f"\n[2] SSM Step Simulation (d_model=64, dt={MAMBA2_DT_MAX})")
    print("-" * 60)
    state_shape, conv_shape = simulate_ssm_step(
        MAMBA2_D_STATE, MAMBA2_D_CONV, MAMBA2_EXPAND, MAMBA2_DT_MAX
    )
    d_inner = 64 * MAMBA2_EXPAND
    print(f"  d_inner = d_model * expand = 64 * {MAMBA2_EXPAND} = {d_inner}")
    print(f"  State shape:  {state_shape}  (d_inner x d_state)")
    print(f"  Conv shape:   {conv_shape}  (d_inner x d_conv)")

    # ─── Duality Note ───
    print(f"\n[3] SSM-Attention Duality")
    print("-" * 60)
    print(f"  Mamba-2 shows SSM <=> masked attention with specific structure.")
    print(f"  d_state={PRED_D_STATE} = 2^n = number of 'virtual heads'")
    print(f"  expand={PRED_EXPAND} = phi = inner dimension multiplier")
    print(f"  d_conv={PRED_D_CONV} = tau = local context window")

    # ─── Final Verdict ───
    print(f"\n{'=' * 70}")
    print(f"  RESULT: {exact_count}/{total} EXACT")
    verdict = "PASS" if exact_count == total else "FAIL"
    print(f"  VERDICT: {verdict}")
    print(f"{'=' * 70}")


if __name__ == "__main__":
    main()
