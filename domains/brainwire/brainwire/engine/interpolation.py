import math
from brainwire.variables import VAR_NAMES

def linear_alpha(t_norm: float) -> float:
    return max(0.0, min(1.0, t_norm))

def sigmoid_alpha(t_norm: float) -> float:
    return 1.0 / (1.0 + math.exp(-12.0 * (t_norm - 0.5)))

def exponential_alpha(t_norm: float, tau: float = 0.2) -> float:
    return 1.0 - math.exp(-t_norm / tau)

def lerp_states(state_a: dict[str, float], state_b: dict[str, float], alpha: float) -> dict[str, float]:
    alpha = max(0.0, min(1.0, alpha))
    return {k: (1.0 - alpha) * state_a[k] + alpha * state_b[k] for k in VAR_NAMES}

def blend_states(states: list[dict[str, float]], weights: list[float]) -> dict[str, float]:
    if abs(sum(weights) - 1.0) > 0.01:
        raise ValueError(f"Weights must sum to 1.0, got {sum(weights):.3f}")
    result = {k: 0.0 for k in VAR_NAMES}
    for state, w in zip(states, weights):
        for k in VAR_NAMES:
            result[k] += w * state[k]
    return result

def envelope_value(t: float, onset_s: float, plateau_s: float, offset_s: float, curve: str = 'sigmoid') -> float:
    curve_fn = {'linear': linear_alpha, 'sigmoid': sigmoid_alpha, 'exponential': exponential_alpha}.get(curve, linear_alpha)
    if t < 0:
        return 0.0
    elif t < onset_s:
        return curve_fn(t / onset_s) if onset_s > 0 else 1.0
    elif t < onset_s + plateau_s:
        return 1.0
    elif t < onset_s + plateau_s + offset_s:
        elapsed = t - onset_s - plateau_s
        return 1.0 - curve_fn(elapsed / offset_s) if offset_s > 0 else 0.0
    else:
        return 0.0
