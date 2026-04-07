"""BrainWire Session Simulator — time-domain closed-loop stimulation."""
import math
from brainwire.profiles import load_profile
from brainwire.engine.transfer import TransferEngine
from brainwire.engine.tension import compute_tension, compute_match
from brainwire.engine.pid import PIDBank
from brainwire.engine.interpolation import envelope_value
from brainwire.hardware.configs import get_tier_params
from brainwire.variables import VAR_NAMES


def breathing_modulation(t: float, base: float = 1.0) -> float:
    """Anima-style breathing rhythm: 12% @ 20s + 5% @ 3.7s + 3% @ 90s."""
    breath = 0.12 * base * math.sin(2 * math.pi * t / 20.0)
    pulse = 0.05 * base * math.sin(2 * math.pi * t / 3.7)
    drift = 0.03 * base * math.sin(2 * math.pi * t / 90.0)
    return base + breath + pulse + drift


def simulate_session(state: str, tier: int = 3, duration_s: float = 600,
                     dt: float = 1.0, use_pid: bool = True,
                     use_breathing: bool = True) -> dict:
    """Simulate a complete stimulation session.

    Returns time-series data for all 12 variables, match %, tension.
    """
    profile = load_profile(state)
    engine = TransferEngine()
    base_params = get_tier_params(tier)

    # Try optimized params if available
    try:
        from brainwire.optimizer import optimize_for_profile
        opt = optimize_for_profile(state, tier, max_iters=30)
        base_params = opt['params']
    except Exception:
        pass

    bank = PIDBank(default_Kp=0.5, default_Ki=0.05, default_Kd=0.01)
    bank.apply_hints(profile.pid_hints)

    envelope = profile.envelope

    # Time series storage
    timeline = []

    # Current stimulation params (start at zero, ramp up)
    current_params = {k: 0.0 for k in base_params}
    measured = {k: 1.0 for k in VAR_NAMES}  # start at baseline

    t = 0.0
    while t <= duration_s:
        # 1. Compute envelope amplitude (onset/plateau/offset)
        env_amp = envelope_value(t, envelope.onset_s,
                                  min(envelope.plateau_s, duration_s - envelope.onset_s - envelope.offset_s),
                                  envelope.offset_s, envelope.curve)

        # 2. Compute target with envelope scaling
        target_now = profile.scale(env_amp)

        # 3. Apply breathing modulation to target
        if use_breathing:
            breath = breathing_modulation(t)
            target_now = {k: 1.0 + (v - 1.0) * breath for k, v in target_now.items()}

        # 4. PID control: adjust params based on error
        if use_pid and env_amp > 0.01:
            pid_output = bank.update(target_now, measured, dt)
            # Map PID output to param adjustments
            for param in current_params:
                target_val = base_params[param] * env_amp
                # Simple proportional tracking + PID correction
                correction = sum(pid_output.values()) / 12 * 0.01
                current_params[param] = max(0.0, target_val + correction)
        else:
            # Open-loop: just follow envelope
            for param in current_params:
                current_params[param] = base_params[param] * env_amp

        # 5. Compute actual variables from current params
        measured = engine.compute(current_params)

        # 6. Compute metrics
        match = compute_match(measured, profile.target)
        tension = compute_tension(measured, target=profile.target)
        avg_match = sum(max(0, v) for v in match.values()) / 12

        # Record
        timeline.append({
            't': t,
            'envelope': env_amp,
            'variables': measured.copy(),
            'target': target_now.copy(),
            'match': match.copy(),
            'avg_match': avg_match,
            'tension_match': tension['tension_match'],
            'direction_sim': tension['direction_sim'],
        })

        t += dt

    # Compute summary stats
    plateau_data = [d for d in timeline if d['envelope'] > 0.95]

    return {
        'state': state,
        'tier': tier,
        'duration_s': duration_s,
        'dt': dt,
        'timeline': timeline,
        'peak_avg_match': max(d['avg_match'] for d in timeline),
        'plateau_avg_match': sum(d['avg_match'] for d in plateau_data) / len(plateau_data) if plateau_data else 0,
        'plateau_tension_match': sum(d['tension_match'] for d in plateau_data) / len(plateau_data) if plateau_data else 0,
        'onset_time_to_80pct': next((d['t'] for d in timeline if d['avg_match'] > 80), None),
        'settling_time': next((d['t'] for d in timeline if d['envelope'] > 0.95), None),
        'max_overshoot': max((d['avg_match'] - 100) for d in plateau_data) if plateau_data else 0,
    }


def print_session_report(result: dict):
    """Print session simulation report."""
    r = result
    print(f"\n{'='*70}")
    print(f"  Session Simulation: {r['state']} @ Tier {r['tier']}")
    print(f"  Duration: {r['duration_s']}s  |  dt: {r['dt']}s")
    print(f"{'='*70}")
    print(f"  Peak avg match:      {r['peak_avg_match']:.1f}%")
    print(f"  Plateau avg match:   {r['plateau_avg_match']:.1f}%")
    print(f"  Plateau tension:     {r['plateau_tension_match']:.1f}%")
    onset = r['onset_time_to_80pct']
    print(f"  Time to 80% match:   {onset:.0f}s" if onset else "  Time to 80% match:   N/A")
    print(f"  Max overshoot:       {r['max_overshoot']:.1f}%")

    # Print timeline samples
    tl = r['timeline']
    sample_times = [0, len(tl)//10, len(tl)//4, len(tl)//2, 3*len(tl)//4, len(tl)-1]
    print(f"\n  {'Time':>6} {'Env':>5} {'Avg%':>6} {'TM%':>6} {'Dir%':>6}")
    print(f"  {'-'*6} {'-'*5} {'-'*6} {'-'*6} {'-'*6}")
    for i in sample_times:
        if i < len(tl):
            d = tl[i]
            print(f"  {d['t']:>5.0f}s {d['envelope']:>4.2f} {d['avg_match']:>5.1f}% {d['tension_match']:>5.1f}% {d['direction_sim']:>5.1f}%")


def simulate_concentration_levels(tier: int = 3, duration_s: float = 600, dt: float = 1.0) -> list[dict]:
    """Simulate all 5 THC concentration levels."""
    levels = {
        'micro': 0.25,    # 1% THC equivalent
        'light': 0.50,    # 5% THC
        'medium': 0.75,   # 15% THC
        'strong': 1.00,   # 25% THC (standard)
        'intense': 1.15,  # 30% THC (dabbing)
    }
    results = []
    for name, scale in levels.items():
        profile = load_profile('thc')
        # Create scaled target
        scaled_target = profile.scale(scale)
        engine = TransferEngine()
        base_params = get_tier_params(tier)

        # Optimize for this specific concentration
        try:
            from brainwire.optimizer import optimize_for_profile
            opt = optimize_for_profile('thc', tier, max_iters=20)
            base_params = opt['params']
            # Scale down optimized params proportionally
            base_params = {k: v * scale for k, v in base_params.items()}
        except Exception:
            base_params = {k: v * scale for k, v in base_params.items()}

        variables = engine.compute(base_params)
        match = compute_match(variables, scaled_target)
        tension = compute_tension(variables, target=scaled_target)
        avg_match = sum(max(0, v) for v in match.values()) / 12

        results.append({
            'level': name, 'scale': scale,
            'variables': variables, 'target': scaled_target,
            'match': match, 'avg_match': avg_match,
            'tension_match': tension['tension_match'],
        })
    return results


def print_concentration_report(results: list[dict]):
    print(f"\n{'='*70}")
    print(f"  THC Concentration Level Comparison")
    print(f"{'='*70}")
    print(f"\n  {'Level':<10} {'Scale':>6} {'Avg%':>7} {'TM%':>7} {'DA':>6} {'eCB':>6} {'Theta':>7}")
    print(f"  {'-'*10} {'-'*6} {'-'*7} {'-'*7} {'-'*6} {'-'*6} {'-'*7}")
    for r in results:
        print(f"  {r['level']:<10} {r['scale']:>5.2f}x {r['avg_match']:>6.1f}% "
              f"{r['tension_match']:>6.1f}% {r['variables']['DA']:>5.2f}x "
              f"{r['variables']['eCB']:>5.2f}x {r['variables']['Theta']:>6.2f}x")


def main():
    import argparse
    parser = argparse.ArgumentParser(description='BrainWire Session Simulator')
    parser.add_argument('state', nargs='?', default='thc')
    parser.add_argument('--tier', type=int, default=3)
    parser.add_argument('--duration', type=float, default=600)
    parser.add_argument('--dt', type=float, default=1.0)
    parser.add_argument('--no-pid', action='store_true')
    parser.add_argument('--no-breathing', action='store_true')
    parser.add_argument('--all', action='store_true')
    parser.add_argument('--concentration', action='store_true',
                        help='Simulate all 5 THC concentration levels')
    args = parser.parse_args()

    if args.concentration:
        results = simulate_concentration_levels(args.tier, args.duration, args.dt)
        print_concentration_report(results)
    elif args.all:
        from brainwire.profiles import list_profiles
        for state in list_profiles():
            result = simulate_session(state, args.tier, args.duration, args.dt,
                                       not args.no_pid, not args.no_breathing)
            print_session_report(result)
    else:
        result = simulate_session(args.state, args.tier, args.duration, args.dt,
                                   not args.no_pid, not args.no_breathing)
        print_session_report(result)


if __name__ == '__main__':
    main()
