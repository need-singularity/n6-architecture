"""STDP Synaptic Pathway Simulator — models pathway weakening/strengthening over sessions."""
import math

def stdp_weight(n_pulses: int, a: float = 0.005, eta: float = 0.8,
                floor: float = 0.05, w0: float = 1.0) -> float:
    """Compute synaptic weight after n effective pulses. Eq 11-15 from P-002."""
    effective = n_pulses * eta
    w = floor + (w0 - floor) * (1.0 - a) ** effective
    return w

def simulate_sessions(n_sessions: int = 50, pulses_per: int = 1000,
                       a: float = 0.005, eta: float = 0.8, floor: float = 0.05) -> list[dict]:
    """Simulate STDP weight trajectory over sessions."""
    trajectory = []
    for s in range(n_sessions + 1):
        total_pulses = s * pulses_per
        w = stdp_weight(total_pulses, a, eta, floor)
        reduction = (1.0 - w) * 100
        trajectory.append({'session': s, 'pulses': total_pulses, 'weight': w, 'reduction_pct': reduction})
    return trajectory

def time_to_threshold(threshold: float = 0.1, pulses_per: int = 1000,
                       a: float = 0.005, eta: float = 0.8, floor: float = 0.05) -> int:
    """How many sessions to reduce weight below threshold?"""
    for s in range(1, 1000):
        w = stdp_weight(s * pulses_per, a, eta, floor)
        if w <= threshold:
            return s
    return -1

def main():
    print(f"\n{'='*60}")
    print(f"  STDP Pathway Weakening Simulator")
    print(f"{'='*60}")

    traj = simulate_sessions(50)
    print(f"\n  {'Session':>7} {'Pulses':>8} {'Weight':>8} {'Reduction':>10}")
    print(f"  {'-'*7} {'-'*8} {'-'*8} {'-'*10}")
    for t in traj:
        if t['session'] % 5 == 0:
            bar = '#' * int(t['weight'] * 20) + '.' * (20 - int(t['weight'] * 20))
            print(f"  {t['session']:>7} {t['pulses']:>8} {t['weight']:>7.4f} {t['reduction_pct']:>9.1f}% {bar}")

    for thresh in [0.5, 0.2, 0.1]:
        s = time_to_threshold(thresh)
        print(f"\n  Sessions to reach W<{thresh}: {s}")

if __name__ == '__main__':
    main()
