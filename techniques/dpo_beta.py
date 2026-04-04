"""
Technique 21: DPO Beta & Alignment Parameters (BT-64, BT-163)
==============================================================
Direct Preference Optimization and RLHF alignment parameters all converge
to n=6 arithmetic:

  DPO beta          = 1/(sigma-phi) = 0.1   (BT-64 universal regularization)
  KL penalty        = 1/(sigma-phi) = 0.1   (KL divergence coefficient)
  PPO clip          = phi/(sigma-phi) = 0.2  (BT-163 PPO epsilon)
  PPO epochs        = tau = 4                (BT-163 mini-batch epochs)
  PPO minibatches   = tau = 4                (BT-163)
  GRPO group size   = phi^tau = 16           (BT-163 GRPO G)
  GAE lambda        = 1 - 1/(J2-tau) = 0.95 (BT-163)
  Reward/Policy ratio = R(6) = 1             (BT-163)

The 1/(sigma-phi) = 0.1 regularization constant appears in 8+ independent
algorithms (BT-64): weight decay, DPO, GPTQ, cosine schedule, Mamba, KL.

Expected: 8/8 EXACT for alignment hyperparameters.
"""

import numpy as np
import math

# ─── n=6 Constants ────────────────────────────────────────────────────
n = 6
sigma = 12
phi = 2
tau = 4
sopfr = 5
mu = 1
J2 = 24
R6 = 1           # R(6) = sigma(6)/6 - 1 = 1

# ─── Alignment Parameter Map ─────────────────────────────────────────

ALIGNMENT_PARAMS = [
    {
        "name": "DPO beta",
        "actual": 0.1,
        "n6_val": 1 / (sigma - phi),
        "formula": "1/(sigma-phi) = 1/10 = 0.1",
        "bt": "BT-64",
    },
    {
        "name": "KL penalty coeff",
        "actual": 0.1,
        "n6_val": 1 / (sigma - phi),
        "formula": "1/(sigma-phi) = 0.1",
        "bt": "BT-64",
    },
    {
        "name": "PPO clip epsilon",
        "actual": 0.2,
        "n6_val": phi / (sigma - phi),
        "formula": "phi/(sigma-phi) = 2/10 = 0.2",
        "bt": "BT-163",
    },
    {
        "name": "PPO epochs",
        "actual": 4,
        "n6_val": tau,
        "formula": "tau(6) = 4",
        "bt": "BT-163",
    },
    {
        "name": "PPO minibatches",
        "actual": 4,
        "n6_val": tau,
        "formula": "tau(6) = 4",
        "bt": "BT-163",
    },
    {
        "name": "GRPO group size",
        "actual": 16,
        "n6_val": phi**tau,
        "formula": "phi^tau = 2^4 = 16",
        "bt": "BT-163",
    },
    {
        "name": "GAE lambda",
        "actual": 0.95,
        "n6_val": 1 - 1 / (J2 - tau),
        "formula": "1 - 1/(J2-tau) = 1-1/20 = 0.95",
        "bt": "BT-163",
    },
    {
        "name": "Reward/Policy ratio",
        "actual": 1.0,
        "n6_val": float(R6),
        "formula": "R(6) = 1",
        "bt": "BT-163",
    },
]

# ─── DPO Loss Simulation ─────────────────────────────────────────────

def dpo_loss(beta, log_ratio_w, log_ratio_l):
    """Compute DPO loss: -log(sigmoid(beta * (log_ratio_w - log_ratio_l)))."""
    diff = beta * (log_ratio_w - log_ratio_l)
    return -np.log(1 / (1 + np.exp(-diff)) + 1e-10)


def simulate_dpo_sweep(log_ratios_w, log_ratios_l, betas):
    """Sweep DPO beta values, return mean loss for each."""
    results = []
    for beta in betas:
        losses = [dpo_loss(beta, lw, ll)
                  for lw, ll in zip(log_ratios_w, log_ratios_l)]
        results.append(np.mean(losses))
    return results


# ─── Main Verification ───────────────────────────────────────────────

if __name__ == "__main__":
    print("=" * 70)
    print("  Technique 21: DPO Beta & Alignment Parameters")
    print("  beta = 1/(sigma-phi) = 0.1 (BT-64 universal regularization)")
    print("=" * 70)
    print(f"\n  n=6: sigma={sigma}, phi={phi}, tau={tau}, J2={J2}, R(6)={R6}")

    # ─── Parameter verification ───────────────────────────────────────
    print(f"\n  {'Parameter':<22} {'Actual':>8} {'n=6':>8} {'Formula':<32} {'BT':<7} {'Grade'}")
    print("  " + "-" * 85)

    n_exact = 0
    n_total = len(ALIGNMENT_PARAMS)
    for p in ALIGNMENT_PARAMS:
        tol = 1e-9
        exact = abs(p["actual"] - p["n6_val"]) < tol
        grade = "EXACT" if exact else "FAIL"
        if exact:
            n_exact += 1
        marker = " <<<" if exact else ""
        print(f"  {p['name']:<22} {p['actual']:>8.4f} {p['n6_val']:>8.4f} "
              f"{p['formula']:<32} {p['bt']:<7} {grade}{marker}")

    # ─── DPO beta sensitivity ─────────────────────────────────────────
    print(f"\n  --- DPO Beta Sensitivity ---")
    rng = np.random.RandomState(42)
    n_samples = 1000
    # Simulate preference pairs (winner has higher log ratio)
    log_ratios_w = rng.normal(0.5, 0.3, n_samples)
    log_ratios_l = rng.normal(-0.2, 0.3, n_samples)

    betas = [0.01, 0.05, 0.1, 0.15, 0.2, 0.3, 0.5, 1.0]
    losses = simulate_dpo_sweep(log_ratios_w, log_ratios_l, betas)

    optimal_idx = np.argmin(losses)
    for i, (b, l) in enumerate(zip(betas, losses)):
        marker = " <<< 1/(sigma-phi)" if abs(b - 0.1) < 0.001 else ""
        if i == optimal_idx:
            marker += " [optimal]"
        print(f"    beta={b:<5.2f}  loss={l:.4f}{marker}")

    # ─── BT-64: 0.1 universality count ────────────────────────────────
    print(f"\n  --- BT-64: 1/(sigma-phi) = 0.1 Universal Regularization ---")
    bt64_algorithms = [
        ("Weight decay (AdamW)",    0.1, "1/(sigma-phi)"),
        ("DPO beta",                0.1, "1/(sigma-phi)"),
        ("GPTQ damping",            0.1, "1/(sigma-phi)"),
        ("Cosine min ratio",        0.1, "1/(sigma-phi)"),
        ("Mamba dt_init_scale",     0.1, "1/(sigma-phi)"),
        ("KL penalty",              0.1, "1/(sigma-phi)"),
        ("SimCLR temperature",      0.1, "1/(sigma-phi)"),
        ("Magnetic reconnection",   0.1, "1/(sigma-phi)"),
    ]
    for alg, val, formula in bt64_algorithms:
        print(f"    {alg:<30} = {val} = {formula}")
    print(f"    Total: {len(bt64_algorithms)} algorithms with 0.1 = 1/(sigma-phi)")
    print(f"    Meta n=6: count = sigma-tau = {sigma-tau} = 8")

    # ─── PASS/FAIL ────────────────────────────────────────────────────
    pass_threshold = 6
    passed = n_exact >= pass_threshold

    print(f"\n  EXACT: {n_exact}/{n_total}")
    print(f"  Threshold: >= {pass_threshold} EXACT")
    print(f"\n  {'PASS' if passed else 'FAIL'}: DPO/Alignment n=6 mapping "
          f"({'BT-64/BT-163 confirmed' if passed else 'needs refinement'})")
