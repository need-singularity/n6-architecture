import argparse
from brainwire.profiles import load_profile, list_profiles
from brainwire.engine.transfer import TransferEngine
from brainwire.engine.tension import compute_tension, compute_match
from brainwire.hardware.configs import TIER_CONFIGS, get_tier_params
from brainwire.variables import VAR_NAMES

def run_benchmark(state: str, tier: int = 3) -> dict:
    profile = load_profile(state)
    params = get_tier_params(tier)
    engine = TransferEngine()
    variables = engine.compute(params)
    match_raw = compute_match(variables, profile.target)
    match = {k: max(0.0, v) for k, v in match_raw.items()}
    tension = compute_tension(variables, target=profile.target)
    avg_match = sum(match.values()) / 12
    over_100 = sum(1 for v in match.values() if v >= 100)
    return {
        'profile_name': profile.name, 'state': state, 'tier': tier,
        'cost': TIER_CONFIGS[tier]['cost'], 'variables': variables,
        'match': match, 'avg_match': avg_match, 'over_100_count': over_100,
        'tension': tension, 'target': profile.target,
    }

def compare_states(states: list[str], tier: int = 3) -> list[dict]:
    return [run_benchmark(s, tier) for s in states]

def run_tier_comparison(state: str) -> list[dict]:
    return [run_benchmark(state, tier=t) for t in sorted(TIER_CONFIGS.keys())]

def print_benchmark(result: dict):
    r = result
    print(f"\n{'='*70}")
    print(f"  {r['profile_name']}  |  Tier {r['tier']} (${r['cost']:,})  |  Avg {r['avg_match']:.1f}%  |  {r['over_100_count']}/12 >= 100%")
    print(f"{'='*70}")
    print(f"  {'Var':<12} {'Target':>7} {'Actual':>7} {'Match':>7} {'Bar'}")
    print(f"  {'-'*12} {'-'*7} {'-'*7} {'-'*7} {'-'*25}")
    for k in VAR_NAMES:
        tgt = r['target'][k]
        act = r['variables'][k]
        pct = r['match'][k]
        bar_len = int(min(pct, 150) / 150 * 20)
        bar = '#' * bar_len + '.' * (20 - bar_len)
        ok = '+' if pct >= 100 else ''
        print(f"  {k:<12} {tgt:>6.1f}x {act:>6.2f}x {pct:>6.1f}% {bar} {ok}")
    t = r['tension']
    print(f"\n  Tension: T={t['T_total']:.2f}/{t['T_total_target']:.2f}  dir={t['direction_sim']:.1f}%  mag={t['magnitude_match']:.1f}%  match={t['tension_match']:.1f}%")

def print_comparison_matrix(results: list[dict]):
    print(f"\n{'='*80}")
    print(f"  {'Variable':<12}", end="")
    for r in results:
        print(f" {r['state']:>8}", end="")
    print()
    print(f"  {'-'*12}", end="")
    for _ in results:
        print(f" {'-'*8}", end="")
    print()
    for k in VAR_NAMES:
        print(f"  {k:<12}", end="")
        for r in results:
            pct = r['match'][k]
            print(f" {pct:>7.0f}%", end="")
        print()
    print(f"  {'AVERAGE':<12}", end="")
    for r in results:
        print(f" {r['avg_match']:>7.1f}%", end="")
    print()

def main():
    parser = argparse.ArgumentParser(description='BrainWire Multi-State Benchmark')
    sub = parser.add_subparsers(dest='command')
    p_bench = sub.add_parser('bench')
    p_bench.add_argument('state')
    p_bench.add_argument('--tier', type=int, default=3, choices=[1, 2, 3, 4, 5])
    p_compare = sub.add_parser('compare')
    p_compare.add_argument('states', nargs='+')
    p_compare.add_argument('--tier', type=int, default=3)
    p_tiers = sub.add_parser('tiers')
    p_tiers.add_argument('state')
    p_all = sub.add_parser('all')
    args = parser.parse_args()
    if args.command == 'bench':
        print_benchmark(run_benchmark(args.state, args.tier))
    elif args.command == 'compare':
        results = compare_states(args.states, args.tier)
        for r in results:
            print_benchmark(r)
        print_comparison_matrix(results)
    elif args.command == 'tiers':
        results = run_tier_comparison(args.state)
        for r in results:
            print_benchmark(r)
    elif args.command == 'all':
        for state in list_profiles():
            results = run_tier_comparison(state)
            for r in results:
                print_benchmark(r)
    else:
        parser.print_help()

if __name__ == '__main__':
    main()
