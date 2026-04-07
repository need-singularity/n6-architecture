from __future__ import annotations

import math
from brainwire.variables import VAR_NAMES, TENSION_WEIGHTS, CHEM_VARS, WAVE_VARS, STATE_VARS

def compute_tension(variables: dict[str, float], target: dict[str, float] | None = None) -> dict:
    if target is None:
        target = {v: 1.0 for v in VAR_NAMES}
    def _sub(keys, vals):
        return math.sqrt(sum(TENSION_WEIGHTS[k] * (vals[k] - 1.0) ** 2 for k in keys))
    t_chem = _sub(CHEM_VARS, variables)
    t_wave = _sub(WAVE_VARS, variables)
    t_state = _sub(STATE_VARS, variables)
    t_total = math.sqrt(t_chem**2 + t_wave**2 + t_state**2)
    t_chem_t = _sub(CHEM_VARS, target)
    t_wave_t = _sub(WAVE_VARS, target)
    t_state_t = _sub(STATE_VARS, target)
    t_total_t = math.sqrt(t_chem_t**2 + t_wave_t**2 + t_state_t**2)
    dot = sum(TENSION_WEIGHTS[k] * (variables[k] - 1.0) * (target[k] - 1.0) for k in VAR_NAMES)
    mag_v = math.sqrt(sum(TENSION_WEIGHTS[k] * (variables[k] - 1.0)**2 for k in VAR_NAMES))
    mag_t = math.sqrt(sum(TENSION_WEIGHTS[k] * (target[k] - 1.0)**2 for k in VAR_NAMES))
    direction = dot / (mag_v * mag_t) * 100 if mag_v > 0 and mag_t > 0 else 0
    magnitude = min(t_total, t_total_t) / max(t_total, t_total_t) * 100 if t_total_t > 0 else 0
    return {
        'T_chem': t_chem, 'T_wave': t_wave, 'T_state': t_state, 'T_total': t_total,
        'T_chem_target': t_chem_t, 'T_wave_target': t_wave_t, 'T_state_target': t_state_t, 'T_total_target': t_total_t,
        'direction_sim': direction, 'magnitude_match': magnitude,
        'tension_match': direction * magnitude / 100,
    }

def compute_match(actual: dict[str, float], target: dict[str, float]) -> dict[str, float]:
    match = {}
    for k in VAR_NAMES:
        t = target[k]
        a = actual[k]
        if t >= 1.0:
            match[k] = a / t * 100
        else:
            match[k] = (1.0 - a) / (1.0 - t) * 100 if t < 1.0 else 100.0
    return match
