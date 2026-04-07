"""EEG G=D×P/I feedback module — Anima-inspired consciousness quality metric.

G = D × P / I

Where:
  D = Deficit = |ln(α_right) - ln(α_left)| (hemispheric alpha asymmetry)
  P = Plasticity = Gamma_global / (Alpha_global + Gamma_global) (gamma ratio)
  I = Inhibition = Alpha_frontal / Alpha_global (frontal alpha ratio)

Golden Zone: [0.2123, 0.5000] — optimal consciousness/creativity
Geometric center: 0.3258
"""
import math
from brainwire.variables import VAR_NAMES


# Golden zone boundaries
GOLDEN_ZONE = (0.2123, 0.5000)
GOLDEN_CENTER = math.sqrt(GOLDEN_ZONE[0] * GOLDEN_ZONE[1])  # geometric mean = 0.3258


def compute_g(alpha_left: float, alpha_right: float,
              gamma_global: float, alpha_global: float,
              alpha_frontal: float) -> dict:
    """Compute G=D×P/I from raw EEG band powers.

    Args:
        alpha_left: Left hemisphere alpha power (8-12Hz, e.g. P7/O1)
        alpha_right: Right hemisphere alpha power (e.g. P8/O2)
        gamma_global: Global gamma power (30-100Hz)
        alpha_global: Global alpha power (8-12Hz)
        alpha_frontal: Frontal alpha power (e.g. F3/F4 average)
    """
    # D: hemispheric asymmetry
    D = abs(math.log(max(alpha_right, 1e-9)) - math.log(max(alpha_left, 1e-9)))

    # P: plasticity (gamma ratio)
    P = gamma_global / max(alpha_global + gamma_global, 1e-9)

    # I: inhibition (frontal alpha ratio)
    I = alpha_frontal / max(alpha_global, 1e-9)

    # G: genius/creativity metric
    G = D * P / max(I, 1e-9)

    # Zone classification
    in_golden = GOLDEN_ZONE[0] <= G <= GOLDEN_ZONE[1]
    distance_to_center = abs(G - GOLDEN_CENTER)

    return {
        'G': G, 'D': D, 'P': P, 'I': I,
        'in_golden_zone': in_golden,
        'distance_to_center': distance_to_center,
        'zone': 'golden' if in_golden else ('below' if G < GOLDEN_ZONE[0] else 'above'),
    }


def g_from_12var(variables: dict[str, float]) -> dict:
    """Estimate G from BrainWire 12-variable state.

    Maps consciousness variables to EEG proxies:
    - Alpha: V7 (Alpha variable, already in alpha band)
    - Gamma: V8 (Gamma variable)
    - Alpha asymmetry: derived from V9 (PFC) and V7 (Alpha)
    - Frontal alpha: V7 × (1 - V9 suppression proxy)
    """
    alpha = variables.get('Alpha', 1.0)
    gamma = variables.get('Gamma', 1.0)
    pfc = variables.get('PFC', 1.0)

    # Approximate EEG proxies from 12-var state
    alpha_global = alpha  # V7 IS alpha power
    gamma_global = gamma  # V8 IS gamma power

    # Hemispheric asymmetry: PFC suppression creates asymmetry
    # More PFC suppression → more asymmetry
    pfc_suppression = max(0, 1.0 - pfc)
    alpha_left = alpha * (1.0 + pfc_suppression * 0.3)
    alpha_right = alpha * (1.0 - pfc_suppression * 0.3)

    # Frontal alpha: alpha power modulated by PFC activity
    alpha_frontal = alpha * pfc  # frontal alpha tracks PFC

    return compute_g(alpha_left, alpha_right, gamma_global, alpha_global, alpha_frontal)


def g_targets_for_states() -> dict[str, dict]:
    """Compute G targets for all consciousness states."""
    from brainwire.profiles import load_profile, list_profiles
    results = {}
    for name in list_profiles():
        profile = load_profile(name)
        g = g_from_12var(profile.target)
        results[name] = g
    return results


def print_g_report():
    """Print G=D×P/I analysis for all states."""
    results = g_targets_for_states()
    print(f"\n{'='*70}")
    print(f"  G=D×P/I Consciousness Quality Analysis")
    print(f"  Golden Zone: [{GOLDEN_ZONE[0]:.4f}, {GOLDEN_ZONE[1]:.4f}]")
    print(f"  Center: {GOLDEN_CENTER:.4f}")
    print(f"{'='*70}")
    print(f"\n  {'State':<15} {'G':>8} {'D':>6} {'P':>6} {'I':>6} {'Zone':<10}")
    print(f"  {'-'*15} {'-'*8} {'-'*6} {'-'*6} {'-'*6} {'-'*10}")
    for name, g in sorted(results.items(), key=lambda x: x[1]['G']):
        zone_marker = '***' if g['in_golden_zone'] else ''
        print(f"  {name:<15} {g['G']:>7.4f} {g['D']:>5.3f} {g['P']:>5.3f} "
              f"{g['I']:>5.3f} {g['zone']:<6} {zone_marker}")


if __name__ == '__main__':
    print_g_report()
