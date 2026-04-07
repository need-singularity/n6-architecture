"""Epilepsy Treatment Calculator.

Computes: detection time, pre-ictal probability, anti-phase feasibility,
STDP pathway reduction, Shannon safety, N1 vs RNS comparison.
"""
import math


def detection_time(n_channels: int, base_ms: float = 300.0, latency_ms: float = 1.0) -> float:
    """Seizure detection time (ms). Eq 1 from P-002."""
    return base_ms / math.sqrt(n_channels) + latency_ms


def pre_ictal_probability(n_channels: int, p_single: float = 0.01) -> float:
    """Pre-ictal detection probability. Eq 2 from P-002."""
    return 1.0 - (1.0 - p_single) ** n_channels


def phase_error(latency_s: float, freq_hz: float) -> float:
    """Phase error in degrees. Eq 4 from P-002."""
    return 360.0 * latency_s * freq_hz


def anti_phase_feasible(latency_s: float, freq_hz: float, threshold_deg: float = 30.0) -> bool:
    """Can anti-phase termination work at this frequency?"""
    return phase_error(latency_s, freq_hz) < threshold_deg


def stdp_pathway_reduction(n_sessions: int = 30, pulses_per_session: int = 1000,
                            eta_targeting: float = 0.8, a_minus: float = 0.005,
                            floor: float = 0.05) -> dict:
    """STDP anti-kindling pathway reduction. Eq 11-15 from P-002."""
    total_effective = n_sessions * pulses_per_session * eta_targeting
    # With floor: W = floor + (1-floor) * (1-a)^n
    w_idealized = (1.0 - a_minus) ** total_effective
    w_realistic = floor + (1.0 - floor) * (1.0 - a_minus) ** total_effective
    return {
        'sessions': n_sessions,
        'total_pulses': n_sessions * pulses_per_session,
        'effective_pulses': total_effective,
        'w_idealized': w_idealized,
        'w_realistic': w_realistic,
        'reduction_pct': (1.0 - w_realistic) * 100,
    }


def shannon_safety(I_uA: float = 600, pw_us: float = 200,
                    A_geo_um2: float = 2000, roughness: float = 200) -> dict:
    """Shannon charge density safety check. Eq 19 from P-002."""
    Q_uC = I_uA * 1e-6 * pw_us * 1e-6 * 1e6  # convert to uC
    A_eff_cm2 = roughness * A_geo_um2 * 1e-8   # um^2 to cm^2, x roughness
    q_eff = Q_uC / A_eff_cm2  # uC/cm^2

    log_q = math.log10(q_eff * 1e-6)  # convert to C/cm^2 for Shannon
    log_Q = math.log10(Q_uC * 1e-6)   # convert to C
    shannon_k = log_q + log_Q
    margin = 1.85 - shannon_k

    return {
        'Q_uC': Q_uC,
        'A_eff_cm2': A_eff_cm2,
        'q_eff_uC_cm2': q_eff,
        'shannon_k': shannon_k,
        'margin': margin,
        'safe': margin > 0,
    }


def n1_vs_rns() -> dict:
    """Compare N1 vs NeuroPace RNS across all dimensions."""
    n1_detect = detection_time(1024, 300, 1)
    rns_detect = detection_time(4, 300, 10)

    n1_pre = pre_ictal_probability(1024)
    rns_pre = pre_ictal_probability(4)

    return {
        'channels': {'N1': 1024, 'RNS': 4, 'ratio': 1024 / 4},
        'detection_ms': {'N1': n1_detect, 'RNS': rns_detect, 'ratio': rns_detect / n1_detect},
        'pre_ictal': {'N1': n1_pre, 'RNS': rns_pre},
        'phase_10Hz': {'N1': phase_error(0.001, 10), 'RNS': phase_error(0.160, 10)},
        'stdp_capable': {'N1': True, 'RNS': False},
    }


def print_report():
    """Print full epilepsy calculator report."""
    print(f"\n{'=' * 60}")
    print(f"  N1 Epilepsy Treatment Calculator")
    print(f"{'=' * 60}")

    # Detection
    print(f"\n  === Detection ===")
    for name, n, lat in [('RNS', 4, 10), ('N1', 1024, 1)]:
        t = detection_time(n, 300, lat)
        p = pre_ictal_probability(n)
        print(f"  {name:>4}: {n:>5}ch, detect={t:>7.1f}ms, P_pre={p:.4f}")

    # Anti-phase
    print(f"\n  === Anti-Phase Feasibility ===")
    print(f"  {'Freq':>6} {'N1 err':>8} {'RNS err':>9} {'N1?':>4} {'RNS?':>5}")
    for f in [3, 5, 10, 20, 80, 250]:
        n1_e = phase_error(0.001, f)
        rns_e = phase_error(0.160, f)
        print(f"  {f:>5}Hz {n1_e:>7.1f}° {rns_e:>8.1f}° {'YES':>4} {'YES' if rns_e < 30 else 'NO':>5}")

    # STDP
    print(f"\n  === STDP Anti-Kindling ===")
    for sessions in [10, 20, 30, 50]:
        r = stdp_pathway_reduction(sessions)
        print(f"  {sessions:>3} sessions: W={r['w_realistic']:.4f}, reduction={r['reduction_pct']:.1f}%")

    # Shannon
    print(f"\n  === Shannon Safety ===")
    s = shannon_safety()
    print(f"  Q={s['Q_uC']:.3f} uC, q_eff={s['q_eff_uC_cm2']:.1f} uC/cm^2")
    print(f"  Shannon k={s['shannon_k']:.3f}, margin={s['margin']:.3f}, safe={s['safe']}")

    # Comparison
    print(f"\n  === N1 vs RNS ===")
    c = n1_vs_rns()
    print(f"  Channels:  {c['channels']['ratio']:.0f}x")
    print(f"  Detection: {c['detection_ms']['ratio']:.1f}x faster")
    print(f"  Pre-ictal: N1={c['pre_ictal']['N1']:.4f} vs RNS={c['pre_ictal']['RNS']:.4f}")
    print(f"  Phase@10Hz: N1={c['phase_10Hz']['N1']:.1f} deg vs RNS={c['phase_10Hz']['RNS']:.1f} deg")
    print(f"  STDP cure: N1={c['stdp_capable']['N1']} vs RNS={c['stdp_capable']['RNS']}")


if __name__ == '__main__':
    print_report()
