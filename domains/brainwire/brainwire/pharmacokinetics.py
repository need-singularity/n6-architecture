"""THC Pharmacokinetic Time Model.

Models the temporal dynamics of each 12-variable during actual THC experience.
Different variables have different onset rates, peak times, and decay curves.

Based on:
- THC plasma kinetics: Tmax=8min (smoked), elimination t1/2=1.5h
- CB1 receptor occupancy: Hill equation with time-varying concentration
- Downstream cascade delays: DA fast (seconds), eCB slow (minutes), brainwave intermediate
"""
import math
from brainwire.variables import VAR_NAMES
from brainwire.profiles import load_profile


# Per-variable pharmacokinetic parameters
# (onset_tau_s, peak_time_s, decay_tau_s, relative_peak)
# onset_tau: time constant for rise (63% at t=tau)
# peak_time: when variable reaches maximum
# decay_tau: time constant for decline
# relative_peak: multiplier on target (some overshoot, some undershoot peak)
VAR_KINETICS = {
    'DA':        (30,   300,  2400,  1.15),   # Fast onset (reward), slow decay, slight overshoot
    'eCB':       (120,  900,  5400,  1.00),   # Slow onset (receptor cascade), very slow decay
    '5HT':       (60,   600,  3600,  1.05),   # Medium onset, medium decay
    'GABA':      (90,   720,  3000,  1.00),   # Medium-slow onset
    'NE':        (20,   180,  1800,  1.20),   # Very fast onset (LC suppression), overshoot then stabilize
    'Theta':     (45,   480,  2700,  1.10),   # Medium onset (entrainment builds)
    'Alpha':     (30,   360,  2400,  1.10),   # Fast suppression
    'Gamma':     (60,   600,  3600,  1.00),   # Medium onset
    'PFC':       (40,   420,  2700,  1.05),   # Fast suppression
    'Sensory':   (90,   720,  4200,  1.00),   # Slow build
    'Body':      (120,  900,  4800,  1.00),   # Slow build (somatic)
    'Coherence': (75,   600,  3600,  0.95),   # Medium onset, slight undershoot
}


def variable_at_time(var: str, t: float, target: float, baseline: float = 1.0) -> float:
    """Compute variable value at time t using pharmacokinetic model.

    Uses a gamma-function-like curve: fast rise, slow decay.
    v(t) = baseline + (target - baseline) × peak_factor × f(t)

    f(t) = (1 - exp(-t/onset_tau)) × exp(-(t-peak_time)²/(2×decay_tau²)) envelope
    Simplified: rise phase × sustained phase
    """
    onset_tau, peak_time, decay_tau, rel_peak = VAR_KINETICS[var]

    deviation = (target - baseline) * rel_peak

    # Rise phase: exponential approach
    if t <= 0:
        return baseline

    rise = 1.0 - math.exp(-t / onset_tau)

    # Decay phase: starts after peak
    if t > peak_time:
        decay = math.exp(-(t - peak_time) / decay_tau)
    else:
        decay = 1.0

    # Combined
    amplitude = rise * decay

    return baseline + deviation * amplitude


def simulate_thc_pharmacokinetics(concentration: str = 'strong',
                                    duration_s: float = 7200,
                                    dt: float = 10.0) -> dict:
    """Simulate THC pharmacokinetic time course for all 12 variables.

    Args:
        concentration: 'micro', 'light', 'medium', 'strong', 'intense'
        duration_s: total simulation time in seconds
        dt: time step in seconds
    """
    profile = load_profile('thc')

    # Concentration scaling
    scales = {'micro': 0.25, 'light': 0.50, 'medium': 0.75, 'strong': 1.00, 'intense': 1.15}
    scale = scales.get(concentration, 1.0)
    target = profile.scale(scale)

    timeline = []
    t = 0.0

    while t <= duration_s:
        state = {}
        for var in VAR_NAMES:
            state[var] = variable_at_time(var, t, target[var])

        # Compute which phase we're in
        avg_progress = sum(
            abs(state[v] - 1.0) / max(abs(target[v] - 1.0), 0.01)
            for v in VAR_NAMES
        ) / 12

        if avg_progress < 0.3:
            phase = 'onset'
        elif avg_progress > 0.8:
            phase = 'peak' if t < 1800 else 'plateau'
        else:
            phase = 'rising' if t < 900 else 'declining'

        timeline.append({
            't': t,
            'variables': state.copy(),
            'avg_progress': avg_progress,
            'phase': phase,
        })
        t += dt

    # Find peak times per variable
    peak_times = {}
    for var in VAR_NAMES:
        max_val = max(timeline, key=lambda d: abs(d['variables'][var] - 1.0))
        peak_times[var] = max_val['t']

    return {
        'concentration': concentration,
        'scale': scale,
        'target': target,
        'duration_s': duration_s,
        'timeline': timeline,
        'peak_times': peak_times,
    }


def generate_hardware_timing(pk_result: dict) -> dict:
    """Convert pharmacokinetic profile to hardware timing commands.

    For each variable, generate the temporal profile that hardware
    should follow to replicate the natural THC time course.
    """
    timeline = pk_result['timeline']
    target = pk_result['target']

    # For each timepoint, compute what fraction of target each variable should be
    hardware_timeline = []

    for point in timeline:
        hw_point = {'t': point['t'], 'fractions': {}}
        for var in VAR_NAMES:
            actual = point['variables'][var]
            tgt = target[var]
            # Fraction of full target reached
            if abs(tgt - 1.0) > 0.01:
                frac = (actual - 1.0) / (tgt - 1.0)
            else:
                frac = 1.0
            hw_point['fractions'][var] = max(0.0, min(1.5, frac))
        hardware_timeline.append(hw_point)

    return {
        'concentration': pk_result['concentration'],
        'hardware_timeline': hardware_timeline,
        'peak_times': pk_result['peak_times'],
    }


def print_pk_report(result: dict):
    """Print pharmacokinetic simulation report."""
    r = result
    print(f"\n{'='*70}")
    print(f"  THC Pharmacokinetic Model — {r['concentration']} ({r['scale']:.2f}x)")
    print(f"  Duration: {r['duration_s']/60:.0f} min")
    print(f"{'='*70}")

    # Peak times
    print(f"\n  Variable Peak Times (when each variable reaches maximum):")
    sorted_peaks = sorted(r['peak_times'].items(), key=lambda x: x[1])
    for var, peak_t in sorted_peaks:
        tgt = r['target'][var]
        direction = '↑' if tgt > 1.0 else '↓'
        print(f"    {var:<12} peak @ {peak_t/60:>5.1f}min  target={tgt:.1f}x{direction}")

    # Timeline samples
    tl = r['timeline']
    samples = [0, 1, 3, 5, 10, 15, 20, 30, 45, 60, 90, 120]
    print(f"\n  {'Time':>6}", end="")
    for var in ['DA', 'eCB', 'NE', 'Theta', 'Alpha', 'Gamma', 'Body']:
        print(f" {var:>6}", end="")
    print(f" {'Phase':<10}")

    for min_t in samples:
        t_s = min_t * 60
        # Find closest timepoint
        closest = min(tl, key=lambda d: abs(d['t'] - t_s))
        print(f"  {min_t:>4}m ", end="")
        for var in ['DA', 'eCB', 'NE', 'Theta', 'Alpha', 'Gamma', 'Body']:
            v = closest['variables'][var]
            print(f" {v:>5.2f}x", end="")
        print(f"  {closest['phase']}")


def main():
    import argparse
    parser = argparse.ArgumentParser(description='THC Pharmacokinetic Model')
    parser.add_argument('--level', default='strong',
                        choices=['micro', 'light', 'medium', 'strong', 'intense'])
    parser.add_argument('--duration', type=float, default=120, help='Duration in minutes')
    parser.add_argument('--all', action='store_true', help='All concentration levels')
    parser.add_argument('--hardware', action='store_true', help='Generate hardware timing')
    args = parser.parse_args()

    if args.all:
        for level in ['micro', 'light', 'medium', 'strong', 'intense']:
            result = simulate_thc_pharmacokinetics(level, args.duration * 60)
            print_pk_report(result)
    else:
        result = simulate_thc_pharmacokinetics(args.level, args.duration * 60)
        print_pk_report(result)

        if args.hardware:
            hw = generate_hardware_timing(result)
            print(f"\n  Hardware timing generated: {len(hw['hardware_timeline'])} timepoints")
            print(f"  Peak sequence: {' → '.join(v for v, _ in sorted(hw['peak_times'].items(), key=lambda x: x[1]))}")


if __name__ == '__main__':
    main()
