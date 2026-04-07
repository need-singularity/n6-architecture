"""Shannon Charge Density Safety Calculator for neural stimulation electrodes."""
import math

def compute_safety(I_uA: float = 600, pw_us: float = 200,
                    A_geo_um2: float = 2000, roughness: float = 200,
                    k_limit: float = 1.85) -> dict:
    """Full Shannon safety analysis."""
    Q_C = I_uA * 1e-6 * pw_us * 1e-6
    Q_uC = Q_C * 1e6
    Q_nC = Q_C * 1e9
    A_geo_cm2 = A_geo_um2 * 1e-8
    A_eff_cm2 = roughness * A_geo_cm2
    q_geo = Q_uC / A_geo_cm2
    q_eff = Q_uC / A_eff_cm2
    log_q = math.log10(q_eff * 1e-6)
    log_Q = math.log10(Q_uC * 1e-6)
    k = log_q + log_Q
    margin = k_limit - k
    return {
        'I_uA': I_uA, 'pw_us': pw_us,
        'Q_nC': Q_nC, 'Q_uC': Q_uC,
        'A_geo_um2': A_geo_um2, 'A_geo_cm2': A_geo_cm2,
        'A_eff_cm2': A_eff_cm2, 'roughness': roughness,
        'q_geo_uC_cm2': q_geo, 'q_eff_uC_cm2': q_eff,
        'log_q': log_q, 'log_Q': log_Q,
        'k': k, 'k_limit': k_limit, 'margin': margin,
        'safe': margin > 0,
    }

def main():
    print(f"\n{'='*60}")
    print(f"  Shannon Charge Density Safety Calculator")
    print(f"{'='*60}")

    configs = [
        ('N1 max', 600, 200, 2000, 200),
        ('N1 typical', 300, 200, 2000, 200),
        ('N1 conservative', 100, 100, 2000, 200),
        ('RNS typical', 3000, 250, 12000, 50),  # macro electrode
        ('DBS typical', 3000, 90, 60000, 10),   # DBS lead
    ]
    print(f"\n  {'Config':<18} {'I(uA)':>7} {'pw(us)':>7} {'q_eff':>10} {'k':>6} {'margin':>7} {'Safe':>5}")
    print(f"  {'-'*18} {'-'*7} {'-'*7} {'-'*10} {'-'*6} {'-'*7} {'-'*5}")
    for name, I, pw, A, rough in configs:
        s = compute_safety(I, pw, A, rough)
        print(f"  {name:<18} {I:>7} {pw:>7} {s['q_eff_uC_cm2']:>8.1f} {s['k']:>6.2f} {s['margin']:>6.2f} {'YES' if s['safe'] else 'NO':>5}")

if __name__ == '__main__':
    main()
