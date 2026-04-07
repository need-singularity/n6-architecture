import argparse
from brainwire.profiles import load_profile, list_profiles
from brainwire.engine.transfer import TransferEngine
from brainwire.engine.tension import compute_tension, compute_match
from brainwire.engine.interpolation import blend_states
from brainwire.hardware.configs import TIER_CONFIGS, get_tier_params
from brainwire.variables import VAR_NAMES

def multi_state_sensitivity(state: str, tier: int = 3, step: float = 0.1) -> list[dict]:
    profile = load_profile(state)
    engine = TransferEngine()
    base_params = get_tier_params(tier)
    base_vars = engine.compute(base_params)
    base_match = compute_match(base_vars, profile.target)
    base_avg = sum(base_match.values()) / 12
    results = []
    for param, val in base_params.items():
        test_params = base_params.copy()
        test_params[param] = val + step
        test_vars = engine.compute(test_params)
        test_match = compute_match(test_vars, profile.target)
        test_avg = sum(test_match.values()) / 12
        delta = test_avg - base_avg
        changed = [k for k in VAR_NAMES if abs(test_match[k] - base_match[k]) > 0.1]
        results.append({'param': param, 'delta': delta, 'changed': changed, 'current': val})
    results.sort(key=lambda x: -abs(x['delta']))
    return results

def state_gap_analysis(state: str, tier: int = 3) -> list[dict]:
    profile = load_profile(state)
    engine = TransferEngine()
    params = get_tier_params(tier)
    variables = engine.compute(params)
    match = compute_match(variables, profile.target)
    gaps = []
    for k in VAR_NAMES:
        if match[k] < 100:
            gaps.append({'var': k, 'match_pct': match[k], 'target': profile.target[k], 'actual': variables[k], 'deficit': 100 - match[k]})
    gaps.sort(key=lambda x: x['match_pct'])
    return gaps

def blend_command(states: list[str], weights: list[float], tier: int = 3) -> dict:
    profiles = [load_profile(s) for s in states]
    targets = [p.target for p in profiles]
    blended_target = blend_states(targets, weights)
    engine = TransferEngine()
    params = get_tier_params(tier)
    variables = engine.compute(params)
    match = compute_match(variables, blended_target)
    tension = compute_tension(variables, target=blended_target)
    return {'target': blended_target, 'variables': variables, 'match': match, 'avg_match': sum(match.values()) / 12, 'tension': tension, 'blend': dict(zip(states, weights))}

def main():
    parser = argparse.ArgumentParser(description='BrainWire Extended Calculator')
    sub = parser.add_subparsers(dest='command')
    p_sens = sub.add_parser('sensitivity')
    p_sens.add_argument('state')
    p_sens.add_argument('--tier', type=int, default=3)
    p_gap = sub.add_parser('gap')
    p_gap.add_argument('state')
    p_gap.add_argument('--tier', type=int, default=3)
    p_blend = sub.add_parser('blend')
    p_blend.add_argument('--states', nargs='+', required=True)
    p_blend.add_argument('--weights', nargs='+', type=float, required=True)
    p_blend.add_argument('--tier', type=int, default=3)
    args = parser.parse_args()
    if args.command == 'sensitivity':
        results = multi_state_sensitivity(args.state, args.tier)
        print(f"\n  Sensitivity: {args.state} @ Tier {args.tier}\n")
        for r in results[:10]:
            arrow = '+' if r['delta'] > 0 else '-'
            print(f"  {r['param']:<30} {arrow}{abs(r['delta']):.2f}%  [{', '.join(r['changed'][:3])}]")
    elif args.command == 'gap':
        gaps = state_gap_analysis(args.state, args.tier)
        if not gaps:
            print(f"\n  {args.state} @ Tier {args.tier}: all variables >= 100%")
        else:
            print(f"\n  Gaps: {args.state} @ Tier {args.tier} ({len(gaps)} vars below 100%)\n")
            for g in gaps:
                print(f"  {g['var']:<12} {g['match_pct']:>5.1f}%  (target={g['target']:.1f}x actual={g['actual']:.2f}x)")
    elif args.command == 'blend':
        result = blend_command(args.states, args.weights, args.tier)
        print(f"\n  Blend: {result['blend']}  @ Tier {args.tier}")
        print(f"  Avg match: {result['avg_match']:.1f}%\n")
        for k in VAR_NAMES:
            print(f"  {k:<12} target={result['target'][k]:.2f}x  actual={result['variables'][k]:.2f}x  match={result['match'][k]:.1f}%")
    else:
        parser.print_help()

if __name__ == '__main__':
    main()
