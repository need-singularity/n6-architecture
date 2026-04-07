"""Profile-specific hardware parameter optimizer with coordinate descent.

Solves the overshoot problem: generic Tier configs are THC-optimized and produce
massive overshoot on individual variables (e.g. 5HT 206%, NE 165%) which tanks
the tension_match score.  This module finds per-profile params that maximize
tension_match = direction_sim * magnitude_match / 100.
"""

from brainwire.profiles import load_profile, list_profiles
from brainwire.engine.transfer import TransferEngine
from brainwire.engine.tension import compute_tension, compute_match
from brainwire.hardware.configs import get_tier_params
from brainwire.variables import VAR_NAMES


def optimize_for_profile(state: str, tier: int = 4, max_iters: int = 100) -> dict:
    """Find optimal hardware params for a specific consciousness state profile."""
    profile = load_profile(state)
    engine = TransferEngine()
    base_params = get_tier_params(tier)

    # Objective: maximize tension_match (direction × magnitude)
    def objective(params):
        variables = engine.compute(params)
        t = compute_tension(variables, target=profile.target)
        return t['tension_match']

    # Coordinate descent: try adjusting each param
    best_params = base_params.copy()
    best_score = objective(best_params)

    iteration = 0
    for iteration in range(max_iters):
        improved = False
        for param in list(best_params.keys()):
            current_val = best_params[param]
            # Try decreasing (key insight: less can be more!)
            for delta in [-0.1, -0.2, -0.05, 0.05, 0.1]:
                new_val = max(0.0, current_val + delta)
                trial = best_params.copy()
                trial[param] = new_val
                score = objective(trial)
                if score > best_score:
                    best_score = score
                    best_params[param] = new_val
                    improved = True
                    break
        if not improved:
            break

    # Compute final results
    variables = engine.compute(best_params)
    match = compute_match(variables, profile.target)
    tension = compute_tension(variables, target=profile.target)

    return {
        'state': state, 'tier': tier,
        'params': best_params, 'variables': variables,
        'match': match, 'avg_match': sum(match.values()) / 12,
        'tension': tension, 'tension_match': tension['tension_match'],
        'iterations': iteration + 1,
    }


def optimize_all(tier: int = 4) -> dict[str, dict]:
    """Optimize params for all profiles."""
    results = {}
    for state in list_profiles():
        results[state] = optimize_for_profile(state, tier)
    return results


def print_optimization_report(results: dict[str, dict]):
    """Print comparison: generic vs optimized."""
    engine = TransferEngine()
    print(f"\n{'='*80}")
    print(f"  Profile-Specific Optimization Report")
    print(f"{'='*80}")
    print(f"\n  {'State':<15} {'Generic':>10} {'Optimized':>10} "
          f"{'Δ':>8} {'GenTM':>8} {'OptTM':>8} {'ΔTM':>8}")
    print(f"  {'-'*15} {'-'*10} {'-'*10} {'-'*8} {'-'*8} {'-'*8} {'-'*8}")

    for state, r in results.items():
        # Generic
        profile = load_profile(state)
        gen_vars = engine.compute(get_tier_params(r['tier']))
        gen_match = compute_match(gen_vars, profile.target)
        gen_avg = sum(gen_match.values()) / 12
        gen_tension = compute_tension(gen_vars, target=profile.target)
        gen_tm = gen_tension['tension_match']

        opt_avg = r['avg_match']
        opt_tm = r['tension_match']

        print(f"  {state:<15} {gen_avg:>9.1f}% {opt_avg:>9.1f}% "
              f"{opt_avg-gen_avg:>+7.1f}% "
              f"{gen_tm:>7.1f}% {opt_tm:>7.1f}% {opt_tm-gen_tm:>+7.1f}%")

    print()


def main():
    import argparse
    parser = argparse.ArgumentParser(description='BrainWire Profile Optimizer')
    parser.add_argument('--state', default=None, help='Optimize specific state')
    parser.add_argument('--tier', type=int, default=4, choices=[1, 2, 3, 4, 5])
    parser.add_argument('--iters', type=int, default=100)
    args = parser.parse_args()

    if args.state:
        result = optimize_for_profile(args.state, args.tier, args.iters)
        print(f"\n  Optimized {args.state} @ Tier {args.tier}")
        print(f"  Tension match: {result['tension_match']:.1f}%")
        print(f"  Avg match: {result['avg_match']:.1f}%")
        print(f"  Iterations: {result['iterations']}")
        print(f"\n  Per-variable:")
        profile = load_profile(args.state)
        for k in VAR_NAMES:
            tgt = profile.target[k]
            act = result['variables'][k]
            pct = result['match'][k]
            print(f"    {k:<12} target={tgt:.1f}x  actual={act:.2f}x  match={pct:.1f}%")
    else:
        results = optimize_all(args.tier)
        print_optimization_report(results)


if __name__ == '__main__':
    main()
