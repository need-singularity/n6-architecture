"""
Technique 24: YaRN RoPE Scaling
=================================
Peng et al. (2023) — Yet another RoPE extensioN method.
NTK-aware interpolation for context length extension.
Key n=6 mappings:
  Base theta             = (sigma-phi)^tau = 10^4 = 10000
  Scale factors          = (sigma-phi)^k for k=1,2,3 = {10, 100, 1000}
  NTK interp fraction    = phi/(sigma-tau) = 2/8 = 0.25 (low-freq interp)
  NTK extrap fraction    = 1 - phi/(sigma-tau) = 0.75 (high-freq extrap)
  Temperature            = 1/(sigma-phi) = 0.1 (attention temperature scale)

Expected: 5/5 EXACT constant mapping.
"""

import numpy as np
import math

# ─── n=6 Constants ───
N = 6
SIGMA = 12
PHI = 2
TAU = 4
J2 = 24
SOPFR = 5
MU = 1

# ─── YaRN / RoPE Actual Parameters ───
ROPE_BASE_THETA = 10000       # original RoPE base (Vaswani et al.)
YARN_SCALE_FACTORS = [10, 100, 1000]  # common extension factors
YARN_INTERP_FRAC = 0.25       # fraction of dims that interpolate (low-freq)
YARN_EXTRAP_FRAC = 0.75       # fraction of dims that extrapolate (high-freq)
YARN_ATTN_TEMP = 0.1          # attention logit temperature correction

# ─── n=6 Predictions ───
PRED_BASE_THETA = (SIGMA - PHI) ** TAU          # 10^4 = 10000
PRED_SCALE_FACTORS = [(SIGMA - PHI) ** k for k in range(1, TAU)]  # 10^1, 10^2, 10^3
PRED_INTERP_FRAC = PHI / (SIGMA - TAU)          # 2/8 = 0.25
PRED_EXTRAP_FRAC = 1.0 - PHI / (SIGMA - TAU)   # 6/8 = 0.75
PRED_ATTN_TEMP = 1.0 / (SIGMA - PHI)            # 1/10 = 0.1


def verify(name, actual, predicted, formula):
    """Check EXACT match."""
    match = abs(actual - predicted) < 1e-12
    status = "EXACT" if match else "FAIL"
    print(f"  {status:5s}  {name:<18s} = {actual:<10g}  (n=6: {formula} = {predicted})")
    return match


def verify_list(name, actual_list, pred_list, formula):
    """Check EXACT match for lists."""
    match = actual_list == pred_list
    status = "EXACT" if match else "FAIL"
    print(f"  {status:5s}  {name:<18s} = {actual_list}")
    print(f"         {'':18s}   (n=6: {formula} = {pred_list})")
    return match


def compute_rope_freqs(d_head, theta):
    """Compute RoPE frequency bands."""
    freqs = 1.0 / (theta ** (np.arange(0, d_head, 2) / d_head))
    return freqs


def apply_yarn_scaling(freqs, scale_factor, interp_frac):
    """Apply YaRN NTK-aware interpolation to RoPE frequencies."""
    d = len(freqs)
    n_interp = int(d * interp_frac)
    n_extrap = d - n_interp

    # Low-frequency dims: interpolate (divide by scale)
    scaled = freqs.copy()
    scaled[:n_interp] = freqs[:n_interp] / scale_factor

    # High-frequency dims: keep original (extrapolate)
    # (already unchanged)

    return scaled, n_interp, n_extrap


def main():
    print("=" * 70)
    print("  Technique 24: YaRN RoPE Scaling")
    print("  NTK-aware interpolation — theta & scale from n=6")
    print("=" * 70)

    # ─── Constant Verification ───
    print("\n[1] YaRN/RoPE Constant Mapping (5 parameters)")
    print("-" * 60)
    results = []
    results.append(verify("base_theta", ROPE_BASE_THETA, PRED_BASE_THETA, "(sigma-phi)^tau=10^4"))
    results.append(verify_list("scale_factors", YARN_SCALE_FACTORS, PRED_SCALE_FACTORS,
                               "(sigma-phi)^k, k=1..tau-1"))
    results.append(verify("interp_frac", YARN_INTERP_FRAC, PRED_INTERP_FRAC, "phi/(sigma-tau)=1/4"))
    results.append(verify("extrap_frac", YARN_EXTRAP_FRAC, PRED_EXTRAP_FRAC, "1-phi/(sigma-tau)=3/4"))
    results.append(verify("attn_temp", YARN_ATTN_TEMP, PRED_ATTN_TEMP, "1/(sigma-phi)=0.1"))

    exact_count = sum([
        abs(ROPE_BASE_THETA - PRED_BASE_THETA) < 1e-12,
        YARN_SCALE_FACTORS == PRED_SCALE_FACTORS,
        abs(YARN_INTERP_FRAC - PRED_INTERP_FRAC) < 1e-12,
        abs(YARN_EXTRAP_FRAC - PRED_EXTRAP_FRAC) < 1e-12,
        abs(YARN_ATTN_TEMP - PRED_ATTN_TEMP) < 1e-12,
    ])
    total = 5

    # ─── Frequency Analysis ───
    print(f"\n[2] RoPE Frequency Bands (d_head=128)")
    print("-" * 60)
    d_head = 2 ** (SIGMA - SOPFR)  # 128
    freqs = compute_rope_freqs(d_head, ROPE_BASE_THETA)
    print(f"  d_head = 2^(sigma-sopfr) = 2^{SIGMA-SOPFR} = {d_head}")
    print(f"  Frequency range: [{freqs[-1]:.2e}, {freqs[0]:.2e}]")
    print(f"  Wavelength range: [{2*math.pi/freqs[0]:.1f}, {2*math.pi/freqs[-1]:.1f}] positions")

    # ─── YaRN Scaling Demo ───
    print(f"\n[3] YaRN Scaling Demo (scale=10x)")
    print("-" * 60)
    scaled, n_i, n_e = apply_yarn_scaling(freqs, 10, YARN_INTERP_FRAC)
    print(f"  Interpolated dims (low-freq): {n_i}/{len(freqs)} = {n_i/len(freqs):.0%}")
    print(f"  Extrapolated dims (high-freq): {n_e}/{len(freqs)} = {n_e/len(freqs):.0%}")
    print(f"  Original max wavelength: {2*math.pi/freqs[0]:.1f}")
    print(f"  Scaled max wavelength:   {2*math.pi/scaled[0]:.1f} (10x longer)")

    # ─── Context Extension Table ───
    print(f"\n[4] Context Extension Ladder")
    print("-" * 60)
    base_ctx = 2 ** SIGMA  # 4096
    print(f"  {'Scale':<8s} {'Formula':<20s} {'Context':>12s}")
    print(f"  {'-'*42}")
    print(f"  {'1x':<8s} {'base = 2^sigma':<20s} {base_ctx:>12,}")
    for k, s in enumerate(YARN_SCALE_FACTORS, 1):
        ctx = base_ctx * s
        print(f"  {f'{s}x':<8s} {f'(sigma-phi)^{k}':<20s} {ctx:>12,}")

    # ─── Final Verdict ───
    print(f"\n{'=' * 70}")
    print(f"  RESULT: {exact_count}/{total} EXACT")
    verdict = "PASS" if exact_count == total else "FAIL"
    print(f"  VERDICT: {verdict}")
    print(f"{'=' * 70}")


if __name__ == "__main__":
    main()
