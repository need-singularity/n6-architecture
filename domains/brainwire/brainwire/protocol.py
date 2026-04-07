"""Hardware Stimulation Protocol Generator.

Converts optimized parameter sets into:
1. Human-readable step-by-step protocol (printable instructions)
2. Machine-readable JSON sequence (for future automation)
3. Safety checklist
4. Session timeline with exact parameters at each phase
"""
import json
import math
from dataclasses import dataclass, field
from brainwire.profiles import load_profile
from brainwire.engine.interpolation import envelope_value
from brainwire.hardware.configs import TIER_CONFIGS, get_tier_params
from brainwire.hardware.safety import DEVICE_HARD_LIMITS
from brainwire.variables import VAR_NAMES


@dataclass
class ProtocolStep:
    time_s: float
    phase: str  # 'prep', 'ramp_up', 'plateau', 'ramp_down', 'cooldown'
    devices: dict[str, dict[str, float]]  # device -> {param: value}
    notes: str = ""


def params_to_devices(params: dict[str, float]) -> dict[str, dict[str, float]]:
    """Convert flat param dict to device-grouped dict."""
    devices = {}
    for key, val in params.items():
        if val <= 0:
            continue
        parts = key.split('_', 1)
        if len(parts) == 2:
            device, param = parts[0], parts[1]
        else:
            device, param = key, 'intensity'
        # Normalize device names
        device_map = {
            'tDCS': 'tDCS', 'tACS': 'tACS', 'TMS': 'TMS',
            'taVNS': 'taVNS', 'TENS': 'TENS', 'tFUS': 'tFUS',
            'GVS': 'GVS', 'mTI': 'mTI', 'tSCS': 'tSCS',
            'tRNS': 'tRNS', 'HD-tDCS': 'HD-tDCS',
            'entrainment': 'Entrainment',
        }
        device = device_map.get(device, device)
        if device not in devices:
            devices[device] = {}
        devices[device][param] = round(val, 3)
    return devices


def generate_protocol(state: str, tier: int = 3,
                       session_min: int = 30,
                       use_optimizer: bool = True) -> dict:
    """Generate a complete stimulation protocol.

    Returns dict with: steps, safety_checklist, timeline, summary
    """
    profile = load_profile(state)

    # Get params (optimized if requested)
    if use_optimizer:
        try:
            from brainwire.optimizer import optimize_for_profile
            opt = optimize_for_profile(state, tier, max_iters=30)
            target_params = opt['params']
        except Exception:
            target_params = get_tier_params(tier)
    else:
        target_params = get_tier_params(tier)

    envelope = profile.envelope
    session_s = session_min * 60

    # Scale envelope to fit session
    onset_s = min(envelope.onset_s, session_s * 0.2)
    offset_s = min(envelope.offset_s, session_s * 0.15)
    plateau_s = session_s - onset_s - offset_s

    # Generate timeline
    steps = []

    # Phase 0: Preparation (t < 0)
    steps.append(ProtocolStep(
        time_s=-120, phase='prep',
        devices={},
        notes="Electrode placement, impedance check, baseline EEG (2min)"
    ))

    # Phase 1: Ramp up
    ramp_points = [0, onset_s * 0.25, onset_s * 0.5, onset_s * 0.75, onset_s]
    for t in ramp_points:
        amp = envelope_value(t, onset_s, plateau_s, offset_s, envelope.curve)
        scaled = {k: v * amp for k, v in target_params.items()}
        devices = params_to_devices(scaled)
        steps.append(ProtocolStep(
            time_s=t, phase='ramp_up',
            devices=devices,
            notes=f"Ramp {amp*100:.0f}% intensity"
        ))

    # Phase 2: Plateau (sample every 5 min)
    t = onset_s
    while t < onset_s + plateau_s:
        amp = envelope_value(t, onset_s, plateau_s, offset_s, envelope.curve)
        scaled = {k: v * amp for k, v in target_params.items()}
        devices = params_to_devices(scaled)
        steps.append(ProtocolStep(
            time_s=t, phase='plateau',
            devices=devices,
            notes=f"Plateau — maintain {amp*100:.0f}% intensity"
        ))
        t += 300  # 5 min intervals

    # Phase 3: Ramp down
    ramp_down_start = onset_s + plateau_s
    ramp_points = [0, offset_s * 0.25, offset_s * 0.5, offset_s * 0.75, offset_s]
    for dt in ramp_points:
        t = ramp_down_start + dt
        amp = envelope_value(t, onset_s, plateau_s, offset_s, envelope.curve)
        scaled = {k: v * amp for k, v in target_params.items()}
        devices = params_to_devices(scaled)
        steps.append(ProtocolStep(
            time_s=t, phase='ramp_down',
            devices=devices,
            notes=f"Ramp down {amp*100:.0f}% intensity"
        ))

    # Phase 4: Cooldown
    steps.append(ProtocolStep(
        time_s=session_s + 60, phase='cooldown',
        devices={},
        notes="All devices OFF. Rest 5 min. Post-session EEG if available."
    ))

    # Safety checklist
    safety = generate_safety_checklist(state, tier, target_params)

    # Active devices summary
    active_devices = params_to_devices(target_params)

    # Cost
    cost = TIER_CONFIGS.get(tier, {}).get('cost', 0)

    return {
        'state': state,
        'tier': tier,
        'session_min': session_min,
        'onset_s': onset_s,
        'plateau_s': plateau_s,
        'offset_s': offset_s,
        'steps': steps,
        'safety': safety,
        'active_devices': active_devices,
        'cost': cost,
        'target_params': target_params,
    }


def generate_safety_checklist(state: str, tier: int,
                               params: dict[str, float]) -> list[str]:
    """Generate pre-session safety checklist."""
    profile = load_profile(state)
    checks = [
        "[ ] Skin clean and dry at electrode sites",
        "[ ] Electrode gel/saline applied",
        "[ ] Impedance < 5 kOhm at all sites",
        "[ ] Emergency stop button within reach",
        "[ ] Timer set for session duration",
        f"[ ] Max session: {profile.safety.max_session_min} min",
        "[ ] No contraindications (epilepsy, cardiac implant, pregnancy)",
        "[ ] No alcohol/caffeine in last 2 hours",
        "[ ] Hydrated (water available)",
    ]

    if state == 'dmt':
        checks.extend([
            "[ ] FIRST SESSION: Sensory limited to 3.0x",
            "[ ] Voice anchor prepared (someone to talk to)",
            "[ ] EEG epileptiform monitoring active",
        ])

    if state == 'mdma':
        checks.extend([
            "[ ] Temperature monitoring available",
            "[ ] Hydration reminder set (30min intervals)",
        ])

    if profile.safety.emergency_vars:
        vars_str = ', '.join(profile.safety.emergency_vars)
        checks.append(f"[ ] Emergency vars monitored: {vars_str}")

    # Device-specific checks
    devices = params_to_devices(params)
    if 'tDCS' in devices:
        checks.append("[ ] tDCS electrodes: anode RED, cathode BLACK")
    if 'TMS' in devices:
        checks.append("[ ] TMS coil positioned, earplugs in")
    if 'tFUS' in devices:
        checks.append("[ ] tFUS coupling gel applied, focus calibrated")

    return checks


def print_protocol(protocol: dict):
    """Print human-readable protocol."""
    p = protocol
    print(f"\n{'='*70}")
    print(f"  STIMULATION PROTOCOL: {p['state'].upper()}")
    print(f"  Tier {p['tier']} (${p['cost']:,}) | {p['session_min']} min session")
    print(f"  Onset: {p['onset_s']:.0f}s | Plateau: {p['plateau_s']:.0f}s | Offset: {p['offset_s']:.0f}s")
    print(f"{'='*70}")

    # Active devices
    print(f"\n  ACTIVE DEVICES:")
    for device, params in p['active_devices'].items():
        param_str = ', '.join(f"{k}={v}" for k, v in params.items())
        print(f"    {device}: {param_str}")

    # Safety checklist
    print(f"\n  PRE-SESSION CHECKLIST:")
    for check in p['safety']:
        print(f"    {check}")

    # Timeline
    print(f"\n  TIMELINE:")
    print(f"  {'Time':>8} {'Phase':<12} {'Notes'}")
    print(f"  {'-'*8} {'-'*12} {'-'*40}")
    for step in p['steps']:
        t = step.time_s
        if t < 0:
            time_str = f"{t:.0f}s"
        elif t < 60:
            time_str = f"{t:.0f}s"
        else:
            time_str = f"{t/60:.1f}m"
        print(f"  {time_str:>8} {step.phase:<12} {step.notes}")

    print(f"\n{'='*70}")


def export_json(protocol: dict, path: str):
    """Export protocol as machine-readable JSON."""
    data = {
        'state': protocol['state'],
        'tier': protocol['tier'],
        'session_min': protocol['session_min'],
        'timing': {
            'onset_s': protocol['onset_s'],
            'plateau_s': protocol['plateau_s'],
            'offset_s': protocol['offset_s'],
        },
        'active_devices': protocol['active_devices'],
        'safety_checklist': protocol['safety'],
        'timeline': [
            {
                'time_s': s.time_s,
                'phase': s.phase,
                'devices': s.devices,
                'notes': s.notes,
            }
            for s in protocol['steps']
        ],
    }
    with open(path, 'w') as f:
        json.dump(data, f, indent=2)
    return path


def generate_pk_protocol(tier: int = 3, session_min: int = 30,
                          concentration: str = 'strong') -> dict:
    """Generate PK-driven THC protocol — time-varying params following pharmacokinetics.

    Unlike static protocols, this varies EACH device independently
    to match the natural THC temporal profile:
    NE(3m) → DA(5m) → Alpha(6m) → Theta(8m) → 5HT(10m) → eCB(15m) → Body(15m)
    """
    from brainwire.pharmacokinetics import simulate_thc_pharmacokinetics, generate_hardware_timing
    from brainwire.engine.transfer import COEFFICIENTS, SUPPRESSED_VARS

    profile = load_profile('thc')
    pk = simulate_thc_pharmacokinetics(concentration, session_min * 60, dt=30)
    hw_timing = generate_hardware_timing(pk)

    # Get base params
    try:
        from brainwire.optimizer import optimize_for_profile
        opt = optimize_for_profile('thc', tier, max_iters=20)
        base_params = opt['params']
    except Exception:
        base_params = get_tier_params(tier)

    # Build variable→param mapping (which params affect which vars)
    var_to_params = {}
    for var in VAR_NAMES:
        coeffs = COEFFICIENTS.get(var, {})
        var_to_params[var] = []
        for (device, param), coeff in coeffs.items():
            key = f"{device}_{param}"
            if key in base_params or param in base_params:
                var_to_params[var].append(key if key in base_params else param)

    steps = []
    steps.append(ProtocolStep(
        time_s=-120, phase='prep',
        devices={},
        notes="PK-mode: electrode placement, impedance check, baseline EEG"
    ))

    for point in hw_timing['hardware_timeline']:
        t = point['t']
        fracs = point['fractions']

        # For each param, compute its intensity based on the variables it affects
        scaled_params = {}
        for param_key, base_val in base_params.items():
            # Find which variables this param primarily affects
            max_frac = 0.0
            for var, param_list in var_to_params.items():
                if param_key in param_list:
                    max_frac = max(max_frac, fracs.get(var, 0.0))
            if max_frac == 0.0:
                max_frac = sum(fracs.values()) / 12  # average fraction as fallback
            scaled_params[param_key] = base_val * min(max_frac, 1.2)

        devices = params_to_devices(scaled_params)

        # Determine dominant variable at this timepoint
        dominant = max(fracs.items(), key=lambda x: abs(x[1]))
        phase = 'pk_onset' if t < 300 else 'pk_peak' if t < 1200 else 'pk_decline'

        steps.append(ProtocolStep(
            time_s=t, phase=phase,
            devices=devices,
            notes=f"PK t={t/60:.1f}m dominant={dominant[0]}({dominant[1]:.0%})"
        ))

    steps.append(ProtocolStep(
        time_s=session_min * 60 + 60, phase='cooldown',
        devices={},
        notes="PK session complete. All OFF. Journal."
    ))

    safety = generate_safety_checklist('thc', tier, base_params)
    safety.insert(0, "[ ] PK MODE: params vary over time (not constant)")

    return {
        'state': 'thc',
        'tier': tier,
        'session_min': session_min,
        'mode': 'pharmacokinetic',
        'concentration': concentration,
        'onset_s': 0,
        'plateau_s': session_min * 60,
        'offset_s': 0,
        'steps': steps,
        'safety': safety,
        'active_devices': params_to_devices(base_params),
        'cost': TIER_CONFIGS.get(tier, {}).get('cost', 0),
        'target_params': base_params,
        'peak_sequence': sorted(pk['peak_times'].items(), key=lambda x: x[1]),
    }


def print_pk_protocol(protocol: dict):
    """Print PK protocol with peak sequence."""
    print_protocol(protocol)
    if 'peak_sequence' in protocol:
        print(f"\n  PHARMACOKINETIC PEAK SEQUENCE:")
        for var, peak_t in protocol['peak_sequence']:
            print(f"    {peak_t/60:>5.1f}m  {var}")


def main():
    import argparse
    parser = argparse.ArgumentParser(description='Protocol Generator')
    parser.add_argument('state', nargs='?', default='thc')
    parser.add_argument('--tier', type=int, default=3)
    parser.add_argument('--duration', type=int, default=30, help='Session minutes')
    parser.add_argument('--export', type=str, help='Export JSON path')
    parser.add_argument('--all', action='store_true')
    parser.add_argument('--pk', action='store_true', help='PK-driven protocol (THC only)')
    parser.add_argument('--concentration', default='strong',
                        choices=['micro', 'light', 'medium', 'strong', 'intense'])
    args = parser.parse_args()

    if args.pk:
        p = generate_pk_protocol(args.tier, args.duration, args.concentration)
        print_pk_protocol(p)
    elif args.all:
        from brainwire.profiles import list_profiles
        for state in list_profiles():
            p = generate_protocol(state, args.tier, args.duration)
            print_protocol(p)
    else:
        p = generate_protocol(args.state, args.tier, args.duration)
        print_protocol(p)
        if args.export:
            path = export_json(p, args.export)
            print(f"\n  Exported to: {path}")


if __name__ == '__main__':
    main()
