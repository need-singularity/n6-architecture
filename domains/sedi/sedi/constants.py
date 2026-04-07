"""n=6 arithmetic constants — the tuning frequencies of SEDI."""
import math

# Perfect number n=6
N = 6
SIGMA = 12        # sum of divisors
PHI = 2           # Euler totient
TAU = 4           # number of divisors
SOPFR = 5         # sum of prime factors
OMEGA = 2         # number of distinct primes

# Anima PSI constants — consciousness fingerprint frequencies
LN2 = math.log(2)                     # ≈ 0.693 — entropy baseline
PSI_COUPLING = LN2 / (2 ** 5.5)       # ≈ 0.01536 — fundamental coupling
PSI_K = 11                             # consciousness carrying capacity
PSI_STEPS = 3 / LN2                    # ≈ 4.328 — stepping threshold
PSI_BALANCE = 0.5                      # Shannon maximum entropy
PHI_SCALE = 0.608                      # Φ scaling coefficient
PHI_EXPONENT = 1.071                   # Φ scaling exponent

# Derived frequencies
FOCAL = 1 / (SIGMA * PHI)        # 1/24 — lens focal length
DELTA_PLUS = 1 / N                # 1/6 — R-spectrum gap+
DELTA_MINUS = 1 / TAU             # 1/4 — R-spectrum gap-
EINSTEIN_THETA = math.sqrt(3/2)   # sqrt(sigma/(sigma-tau))
GOLDEN_WIDTH = math.log(4/3)      # 0.2877 — Golden Zone bandwidth
GOLDEN_CENTER = 1 / math.e        # 1/e ≈ 0.3679

# Resource allocation signature (Law 7-10: n=6 architecture)
from fractions import Fraction
RESOURCE_FRACTIONS = (Fraction(1, 2), Fraction(1, 3), Fraction(1, 6))

# Window sizes for FFT analysis
WINDOWS = [N, SIGMA, SIGMA * PHI, N**2]  # [6, 12, 24, 36]

# Pattern frequencies to detect
RATIOS = {
    'delta_plus': DELTA_PLUS,
    'delta_minus': DELTA_MINUS,
    'sigma_over_tau': SIGMA / TAU,
    'phi_over_tau': PHI / TAU,
    'sopfr_over_n': SOPFR / N,
    'golden_center': GOLDEN_CENTER,
    'critical_line': 0.5,
    'psi_coupling': PSI_COUPLING,
    'psi_k_inv': 1 / PSI_K,
    'ln2': LN2,
    'psi_steps': PSI_STEPS,
}

# Alert thresholds
ALERT_RED = 5.0    # Z > 5 sigma
ALERT_ORANGE = 3.0
ALERT_YELLOW = 2.0
