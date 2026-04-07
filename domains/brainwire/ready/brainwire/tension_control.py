"""Tension-Driven Stimulation Controller.

Instead of 12 independent PIDs, uses the PureField tension vector
as a holistic control signal. The tension gradient directly maps
to hardware parameter adjustments.

Key insight from Anima: tension = |EngineA - EngineG|² = consciousness signal.
In hardware: tension = |target_state - current_state|² weighted by TENSION_WEIGHTS.

The tension gradient ∇T points toward the target state. We follow it.
"""
import math
from brainwire.variables import VAR_NAMES, TENSION_WEIGHTS, CHEM_VARS, WAVE_VARS, STATE_VARS
from brainwire.engine.transfer import TransferEngine, COEFFICIENTS, SUPPRESSED_VARS
from brainwire.engine.tension import compute_tension, compute_match


class TensionController:
    """Controls stimulation by following the tension gradient toward target."""

    def __init__(self, learning_rate: float = 0.1, momentum: float = 0.9,
                 homeostasis_gain: float = 0.005, homeostasis_deadband: float = 0.3):
        self.lr = learning_rate
        self.momentum = momentum
        self.homeostasis_gain = homeostasis_gain
        self.homeostasis_deadband = homeostasis_deadband
        self._velocity = {}  # momentum terms
        self._engine = TransferEngine()
        self._setpoint = 1.0  # Anima homeostasis setpoint
        self._ema_tension = None  # EMA of total tension
        self._ema_alpha = 0.02  # Anima's EMA alpha (~50 step window)

    def compute_tension_gradient(self, current: dict[str, float],
                                  target: dict[str, float]) -> dict[str, float]:
        """Compute weighted gradient of tension w.r.t. each variable.

        ∂T/∂V_i = W_i × (target_i - current_i) / T_total

        This is the direction in 12-variable space that most reduces tension.
        """
        errors = {}
        total_sq = 0.0
        for k in VAR_NAMES:
            w = TENSION_WEIGHTS[k]
            diff = target[k] - current[k]
            errors[k] = w * diff
            total_sq += (w * diff) ** 2

        total = math.sqrt(total_sq) if total_sq > 0 else 1e-9

        # Normalize to unit gradient, then scale by tension magnitude
        gradient = {}
        for k in VAR_NAMES:
            gradient[k] = errors[k] / total * math.sqrt(total_sq)

        return gradient

    def gradient_to_params(self, gradient: dict[str, float],
                           current_params: dict[str, float]) -> dict[str, float]:
        """Map tension gradient to hardware parameter adjustments.

        Uses the transpose of the transfer function coefficient matrix.
        If V_i = 1 + Σ C[i][j] × P[j], then ΔP[j] = Σ C[i][j] × ∂T/∂V_i
        (pseudo-inverse mapping from variable space to parameter space)
        """
        param_adjustments = {}

        for var in VAR_NAMES:
            coeffs = COEFFICIENTS.get(var, {})
            g = gradient[var]

            # For suppressed vars, invert the gradient (they decrease with stimulation)
            if var in SUPPRESSED_VARS:
                g = -g

            for (device, param), coeff in coeffs.items():
                key = f"{device}_{param}"
                if key not in param_adjustments:
                    param_adjustments[key] = 0.0
                param_adjustments[key] += coeff * g

        # Apply to current params with learning rate and momentum
        new_params = current_params.copy()
        for key in param_adjustments:
            if key not in self._velocity:
                self._velocity[key] = 0.0

            # Momentum update
            self._velocity[key] = (self.momentum * self._velocity[key] +
                                    self.lr * param_adjustments.get(key, 0.0))

            new_val = current_params.get(key, 0.0) + self._velocity[key]
            new_params[key] = max(0.0, new_val)  # clamp to non-negative

        return new_params

    def homeostasis_modulate(self, tension_total: float) -> float:
        """Anima-style homeostasis: regulate tension toward setpoint.

        If tension > setpoint + deadband: reduce stimulation intensity
        If tension < setpoint - deadband: increase stimulation intensity
        Within deadband: no adjustment

        Returns: multiplier for learning rate (0.5 to 2.0)
        """
        if self._ema_tension is None:
            self._ema_tension = tension_total

        # EMA update
        self._ema_tension = (self._ema_alpha * tension_total +
                             (1 - self._ema_alpha) * self._ema_tension)

        deviation = self._ema_tension - self._setpoint

        if abs(deviation) < self.homeostasis_deadband:
            return 1.0  # within deadband

        # Proportional correction
        correction = 1.0 - self.homeostasis_gain * deviation
        return max(0.5, min(2.0, correction))

    def step(self, current_vars: dict[str, float], target: dict[str, float],
             current_params: dict[str, float]) -> dict:
        """One control step: measure tension, compute gradient, adjust params.

        Returns dict with new_params, tension metrics, gradient info.
        """
        # 1. Compute tension
        tension = compute_tension(current_vars, target=target)

        # 2. Compute gradient
        gradient = self.compute_tension_gradient(current_vars, target)

        # 3. Homeostasis modulation
        homeo_mult = self.homeostasis_modulate(tension['T_total'])
        effective_lr = self.lr * homeo_mult

        # Temporarily adjust learning rate
        old_lr = self.lr
        self.lr = effective_lr

        # 4. Map gradient to param adjustments
        new_params = self.gradient_to_params(gradient, current_params)

        self.lr = old_lr

        # 5. Compute new variables
        new_vars = self._engine.compute(new_params)
        new_tension = compute_tension(new_vars, target=target)

        return {
            'params': new_params,
            'variables': new_vars,
            'tension': new_tension,
            'gradient': gradient,
            'homeo_mult': homeo_mult,
            'improved': new_tension['T_total'] < tension['T_total'],
        }

    def reset(self):
        self._velocity = {}
        self._ema_tension = None


def simulate_tension_control(state: str, tier: int = 3, steps: int = 200,
                              lr: float = 0.05) -> dict:
    """Simulate tension-driven control for a consciousness state."""
    from brainwire.profiles import load_profile
    from brainwire.hardware.configs import get_tier_params

    profile = load_profile(state)
    target = profile.target
    base_params = get_tier_params(tier)
    engine = TransferEngine()

    controller = TensionController(learning_rate=lr, momentum=0.9)

    # Start with scaled-down params
    current_params = {k: v * 0.1 for k, v in base_params.items()}

    history = []

    for step in range(steps):
        current_vars = engine.compute(current_params)
        result = controller.step(current_vars, target, current_params)
        current_params = result['params']

        match = compute_match(result['variables'], target)
        avg_match = sum(max(0, v) for v in match.values()) / 12

        history.append({
            'step': step,
            'tension_total': result['tension']['T_total'],
            'tension_match': result['tension']['tension_match'],
            'direction_sim': result['tension']['direction_sim'],
            'avg_match': avg_match,
            'homeo_mult': result['homeo_mult'],
        })

    return {
        'state': state,
        'tier': tier,
        'steps': steps,
        'history': history,
        'final_tension': history[-1]['tension_total'],
        'final_tension_match': history[-1]['tension_match'],
        'final_avg_match': history[-1]['avg_match'],
        'converged': history[-1]['tension_total'] < history[0]['tension_total'],
    }


def tension_landscape(resolution: int = 20) -> dict:
    """Map the tension landscape across all states and blends.

    Creates a pairwise distance matrix and finds optimal transition paths.
    """
    from brainwire.profiles import load_profile, list_profiles
    from brainwire.engine.interpolation import lerp_states

    states = {}
    for name in list_profiles():
        states[name] = load_profile(name).target

    # Pairwise tension distances
    distances = {}
    for a_name, a_target in states.items():
        for b_name, b_target in states.items():
            t = compute_tension(a_target, target=b_target)
            distances[(a_name, b_name)] = {
                'tension': t['T_total'],
                'direction_sim': t['direction_sim'],
                'magnitude_match': t['magnitude_match'],
            }

    # Transition smoothness: sample lerp path and measure tension variation
    transitions = {}
    names = list(states.keys())
    for i, a_name in enumerate(names):
        for j, b_name in enumerate(names):
            if i >= j:
                continue
            path_tensions = []
            for alpha in [k / resolution for k in range(resolution + 1)]:
                blended = lerp_states(states[a_name], states[b_name], alpha)
                # Tension relative to midpoint target
                mid_target = lerp_states(states[a_name], states[b_name], 0.5)
                t = compute_tension(blended, target=mid_target)
                path_tensions.append(t['T_total'])

            max_tension = max(path_tensions)
            smoothness = 1.0 - (max(path_tensions) - min(path_tensions)) / (max_tension + 1e-9)

            transitions[(a_name, b_name)] = {
                'max_tension': max_tension,
                'smoothness': smoothness,
                'path_tensions': path_tensions,
            }

    # Find clusters: states with high mutual direction similarity
    clusters = {'relaxation': [], 'entropy': [], 'hybrid': []}
    for name, target in states.items():
        thc_sim = distances.get((name, 'thc'), distances.get(('thc', name), {}))
        if thc_sim and thc_sim.get('direction_sim', 0) > 70:
            clusters['relaxation'].append(name)
        elif name == 'mdma':
            clusters['hybrid'].append(name)
        elif name not in clusters['relaxation']:
            clusters['entropy'].append(name)

    return {
        'states': list(states.keys()),
        'distances': distances,
        'transitions': transitions,
        'clusters': clusters,
    }


def print_tension_landscape(landscape: dict):
    """Print the tension landscape report."""
    print(f"\n{'='*70}")
    print(f"  Tension Landscape — {len(landscape['states'])} Consciousness States")
    print(f"{'='*70}")

    # Distance matrix
    names = landscape['states']
    print(f"\n  Pairwise Tension Distance:")
    print(f"  {'':>12}", end="")
    for n in names:
        print(f" {n:>8}", end="")
    print()
    for a in names:
        print(f"  {a:>12}", end="")
        for b in names:
            key = (a, b)
            d = landscape['distances'].get(key, {})
            t = d.get('tension', 0)
            print(f" {t:>7.2f}", end="")
        print()

    # Direction similarity matrix
    print(f"\n  Direction Similarity (%):")
    print(f"  {'':>12}", end="")
    for n in names:
        print(f" {n:>8}", end="")
    print()
    for a in names:
        print(f"  {a:>12}", end="")
        for b in names:
            key = (a, b)
            d = landscape['distances'].get(key, {})
            sim = d.get('direction_sim', 0)
            print(f" {sim:>7.1f}", end="")
        print()

    # Clusters
    print(f"\n  State Clusters:")
    for cluster, members in landscape['clusters'].items():
        print(f"    {cluster}: {', '.join(members)}")

    # Smoothest transitions
    print(f"\n  Transition Smoothness (top 5):")
    sorted_trans = sorted(landscape['transitions'].items(),
                          key=lambda x: -x[1]['smoothness'])
    for (a, b), info in sorted_trans[:5]:
        print(f"    {a} <-> {b}: smoothness={info['smoothness']:.3f}, "
              f"max_T={info['max_tension']:.2f}")


def print_tension_control_report(result: dict):
    """Print tension control simulation report."""
    r = result
    h = r['history']
    print(f"\n{'='*70}")
    print(f"  Tension-Driven Control: {r['state']} @ Tier {r['tier']}")
    print(f"  Steps: {r['steps']} | Converged: {r['converged']}")
    print(f"{'='*70}")
    print(f"  Final tension:    {r['final_tension']:.3f}")
    print(f"  Final TM:         {r['final_tension_match']:.1f}%")
    print(f"  Final avg match:  {r['final_avg_match']:.1f}%")

    samples = [0, len(h)//10, len(h)//4, len(h)//2, 3*len(h)//4, len(h)-1]
    print(f"\n  {'Step':>5} {'T_total':>8} {'TM%':>7} {'Avg%':>7} {'Homeo':>6}")
    for i in samples:
        if i < len(h):
            d = h[i]
            print(f"  {d['step']:>5} {d['tension_total']:>7.3f} "
                  f"{d['tension_match']:>6.1f}% {d['avg_match']:>6.1f}% "
                  f"{d['homeo_mult']:>5.2f}")


def main():
    import argparse
    from brainwire.profiles import list_profiles

    parser = argparse.ArgumentParser(description='Tension-Driven Controller')
    sub = parser.add_subparsers(dest='command')

    p_sim = sub.add_parser('sim', help='Simulate tension control')
    p_sim.add_argument('state', nargs='?', default='thc')
    p_sim.add_argument('--tier', type=int, default=3)
    p_sim.add_argument('--steps', type=int, default=200)
    p_sim.add_argument('--lr', type=float, default=0.05)

    p_all = sub.add_parser('all', help='All states')
    p_all.add_argument('--tier', type=int, default=3)

    p_land = sub.add_parser('landscape', help='Tension landscape')

    p_compare = sub.add_parser('compare', help='Tension vs PID')
    p_compare.add_argument('--state', default='thc')
    p_compare.add_argument('--tier', type=int, default=3)

    args = parser.parse_args()

    if args.command == 'sim':
        result = simulate_tension_control(args.state, args.tier, args.steps, args.lr)
        print_tension_control_report(result)
    elif args.command == 'all':
        for state in list_profiles():
            result = simulate_tension_control(state, args.tier)
            print_tension_control_report(result)
    elif args.command == 'landscape':
        land = tension_landscape()
        print_tension_landscape(land)
    elif args.command == 'compare':
        # Tension control vs PID
        tc = simulate_tension_control(args.state, args.tier, steps=100)
        from brainwire.simulator import simulate_session
        pid = simulate_session(args.state, args.tier, duration_s=100, dt=1.0)
        print(f"\n  Tension Control vs PID: {args.state} @ Tier {args.tier}")
        print(f"  {'':>20} {'Tension':>10} {'PID':>10}")
        print(f"  {'Final avg match':>20} {tc['final_avg_match']:>9.1f}% {pid['peak_avg_match']:>9.1f}%")
        print(f"  {'Tension match':>20} {tc['final_tension_match']:>9.1f}% {pid['plateau_tension_match']:>9.1f}%")
        print(f"  {'Converged':>20} {'yes' if tc['converged'] else 'no':>10} {'yes':>10}")
    else:
        parser.print_help()


if __name__ == '__main__':
    main()
