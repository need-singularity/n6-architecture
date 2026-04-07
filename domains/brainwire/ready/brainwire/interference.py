"""Multi-Device Electromagnetic Interference Model.

Models interactions when multiple stimulation devices operate simultaneously.
Key effects: current density summation, frequency beating, path interference.
"""
import math
from dataclasses import dataclass
from brainwire.variables import VAR_NAMES
from brainwire.hardware.configs import get_tier_params


@dataclass
class ElectrodePosition:
    """Standard 10-20 EEG positions mapped to scalp coordinates (simplified 2D)."""
    name: str
    x: float  # lateral (left=-1, right=+1)
    y: float  # anterior-posterior (front=+1, back=-1)

# Key electrode positions (simplified 2D projection)
POSITIONS = {
    'F3': ElectrodePosition('F3', -0.3, 0.6),   # left DLPFC
    'F4': ElectrodePosition('F4', 0.3, 0.6),    # right DLPFC
    'Fz': ElectrodePosition('Fz', 0.0, 0.7),    # midline frontal
    'Cz': ElectrodePosition('Cz', 0.0, 0.0),    # vertex
    'C3': ElectrodePosition('C3', -0.5, 0.0),   # left motor/somatosensory
    'C4': ElectrodePosition('C4', 0.5, 0.0),    # right motor/somatosensory
    'Oz': ElectrodePosition('Oz', 0.0, -0.8),   # visual cortex
    'O1': ElectrodePosition('O1', -0.3, -0.8),  # left visual
    'O2': ElectrodePosition('O2', 0.3, -0.8),   # right visual
    'ear_L': ElectrodePosition('ear_L', -0.9, 0.1),  # taVNS left ear
    'ear_R': ElectrodePosition('ear_R', 0.9, 0.1),   # taVNS right ear
    'S1': ElectrodePosition('S1', -0.4, -0.1),  # somatosensory
}

# Device electrode placements
DEVICE_ELECTRODES = {
    'tDCS_anode_F3': ('F3', 'Fz', 2.0),       # anode F3, return Fz, max 2mA
    'tDCS_cathode_Fz': ('Fz', 'F3', 2.0),     # cathode Fz
    'tDCS_cathode_F4': ('F4', 'Cz', 2.0),     # cathode F4
    'tDCS_anode_V1': ('Oz', 'Cz', 2.0),       # anode V1
    'tDCS_anode_S1': ('C3', 'C4', 2.0),       # anode S1
    'taVNS': ('ear_L', 'ear_R', 0.5),         # ear clips
    'tACS_6Hz': ('Fz', 'Oz', 2.0),            # theta tACS
    'tACS_40Hz': ('Cz', 'Oz', 2.0),           # gamma tACS
}


def distance(a: ElectrodePosition, b: ElectrodePosition) -> float:
    """Euclidean distance between two electrode positions."""
    return math.sqrt((a.x - b.x)**2 + (a.y - b.y)**2)


def current_density_at_point(x: float, y: float,
                              active_devices: dict[str, float]) -> float:
    """Estimate total current density at a scalp point from all active devices.

    Uses simplified 1/r^2 falloff model (far-field approximation).
    Returns total current density in mA/cm^2 (approximate).
    """
    total_density = 0.0
    point = ElectrodePosition('point', x, y)

    for device_key, current_mA in active_devices.items():
        if current_mA <= 0 or device_key not in DEVICE_ELECTRODES:
            continue

        anode_name, cathode_name, max_current = DEVICE_ELECTRODES[device_key]
        anode = POSITIONS.get(anode_name)
        cathode = POSITIONS.get(cathode_name)
        if not anode or not cathode:
            continue

        # Current limited to max
        I = min(abs(current_mA), max_current)

        # Simplified current density: I / (4π × r²) from each electrode
        # Assuming 1cm² electrode area, scalp conductivity ~0.33 S/m
        r_anode = max(distance(point, anode), 0.1)
        r_cathode = max(distance(point, cathode), 0.1)

        # Contribution from this device (both electrodes contribute)
        density = I / (4 * math.pi * r_anode**2) + I / (4 * math.pi * r_cathode**2)
        total_density += density

    return total_density


def compute_interference_map(active_devices: dict[str, float],
                              resolution: int = 20) -> dict:
    """Compute 2D current density map across the scalp.

    Returns grid of current densities and safety analysis.
    """
    grid = []
    max_density = 0.0
    max_point = (0, 0)

    for iy in range(resolution):
        row = []
        for ix in range(resolution):
            x = -1.0 + 2.0 * ix / (resolution - 1)
            y = -1.0 + 2.0 * iy / (resolution - 1)

            # Only compute within unit circle (head shape)
            if x*x + y*y > 1.0:
                row.append(0.0)
                continue

            density = current_density_at_point(x, y, active_devices)
            row.append(density)

            if density > max_density:
                max_density = density
                max_point = (x, y)

        grid.append(row)

    # Safety: FDA limit is ~2.0 A/m² = 0.2 mA/cm²
    safety_limit = 0.2  # mA/cm²
    exceeds_limit = max_density > safety_limit

    return {
        'grid': grid,
        'max_density': max_density,
        'max_point': max_point,
        'safety_limit': safety_limit,
        'exceeds_limit': exceeds_limit,
        'resolution': resolution,
    }


def frequency_interference(frequencies: list[float]) -> dict:
    """Analyze frequency interference between simultaneous AC stimulations.

    When tACS runs at 6Hz and 40Hz simultaneously:
    - Beat frequency = |f1 - f2| = 34Hz
    - Sum frequency = f1 + f2 = 46Hz
    - These create unintended stimulation at new frequencies
    """
    beats = []
    sums = []

    for i in range(len(frequencies)):
        for j in range(i + 1, len(frequencies)):
            f1, f2 = frequencies[i], frequencies[j]
            beat = abs(f1 - f2)
            fsum = f1 + f2
            beats.append({'f1': f1, 'f2': f2, 'beat': beat})
            sums.append({'f1': f1, 'f2': f2, 'sum': fsum})

    # Check if any beat frequency falls in a brain band
    brain_bands = {
        'delta': (0.5, 4), 'theta': (4, 8), 'alpha': (8, 13),
        'beta': (13, 30), 'gamma': (30, 100),
    }

    unintended = []
    for b in beats:
        for band, (lo, hi) in brain_bands.items():
            if lo <= b['beat'] <= hi:
                unintended.append({
                    'type': 'beat', 'freq': b['beat'], 'band': band,
                    'source': f"{b['f1']}Hz + {b['f2']}Hz"
                })
    for s in sums:
        for band, (lo, hi) in brain_bands.items():
            if lo <= s['sum'] <= hi:
                unintended.append({
                    'type': 'sum', 'freq': s['sum'], 'band': band,
                    'source': f"{s['f1']}Hz + {s['f2']}Hz"
                })

    return {
        'beats': beats,
        'sums': sums,
        'unintended_bands': unintended,
        'n_frequencies': len(frequencies),
        'n_interactions': len(beats) + len(sums),
    }


def analyze_tier_interference(tier: int) -> dict:
    """Full interference analysis for a tier's default configuration."""
    params = get_tier_params(tier)

    # Extract active devices with currents
    active = {}
    for key, val in params.items():
        if val > 0:
            # Map param keys to electrode configs
            if 'tDCS_anode' in key and 'mA' in key:
                active['tDCS_anode_F3'] = val
            elif 'cathode_Fz' in key:
                active['tDCS_cathode_Fz'] = val
            elif 'cathode_F4' in key:
                active['tDCS_cathode_F4'] = val
            elif 'anode_V1' in key:
                active['tDCS_anode_V1'] = val
            elif 'anode_S1' in key:
                active['tDCS_anode_S1'] = val
            elif 'VNS' in key:
                active['taVNS'] = val
            elif 'tACS_6Hz' in key:
                active['tACS_6Hz'] = val
            elif 'tACS_40Hz' in key:
                active['tACS_40Hz'] = val

    # Current density map
    density_map = compute_interference_map(active)

    # Frequency interference
    frequencies = []
    if params.get('tACS_6Hz_mA', 0) > 0:
        frequencies.append(6.0)
    if params.get('tACS_10Hz_mA', 0) > 0:
        frequencies.append(10.0)
    if params.get('tACS_40Hz_mA', 0) > 0:
        frequencies.append(40.0)
    if params.get('entrainment_LED_40Hz', 0) > 0:
        frequencies.append(40.0)
    if params.get('entrainment_binaural_6Hz', 0) > 0:
        frequencies.append(6.0)
    # Deduplicate
    frequencies = sorted(set(frequencies))

    freq_analysis = frequency_interference(frequencies)

    # Correction factor: how much the interference model differs from additive
    # Simplified: at high density points, actual effect may be 1.1-1.5x of additive model
    overlap_factor = 1.0 + 0.1 * len(active)  # rough heuristic

    return {
        'tier': tier,
        'active_devices': active,
        'density_map': density_map,
        'freq_analysis': freq_analysis,
        'overlap_factor': overlap_factor,
        'recommendation': (
            'SAFE' if not density_map['exceeds_limit'] and len(freq_analysis['unintended_bands']) == 0
            else 'CAUTION' if not density_map['exceeds_limit']
            else 'REDUCE_CURRENT'
        ),
    }


def print_interference_report(analysis: dict):
    """Print interference analysis report."""
    a = analysis
    d = a['density_map']
    f = a['freq_analysis']

    print(f"\n{'='*70}")
    print(f"  Multi-Device Interference Analysis — Tier {a['tier']}")
    print(f"{'='*70}")

    print(f"\n  Active devices: {len(a['active_devices'])}")
    for dev, current in a['active_devices'].items():
        print(f"    {dev}: {current:.2f} mA")

    print(f"\n  Current Density:")
    print(f"    Max density:    {d['max_density']:.4f} mA/cm²")
    print(f"    Safety limit:   {d['safety_limit']:.4f} mA/cm²")
    print(f"    Location:       ({d['max_point'][0]:.2f}, {d['max_point'][1]:.2f})")
    print(f"    Exceeds limit:  {'YES' if d['exceeds_limit'] else 'NO'}")

    print(f"\n  Frequency Interference:")
    print(f"    Active frequencies: {f['n_frequencies']}")
    print(f"    Interactions: {f['n_interactions']}")
    if f['unintended_bands']:
        print(f"    UNINTENDED band stimulation:")
        for u in f['unintended_bands']:
            print(f"      {u['source']} → {u['type']} {u['freq']:.1f}Hz ({u['band']})")
    else:
        print(f"    No unintended band stimulation")

    print(f"\n  Overlap factor: {a['overlap_factor']:.2f}x (additive model correction)")
    print(f"  Recommendation: {a['recommendation']}")
    print(f"{'='*70}")


def main():
    import argparse
    parser = argparse.ArgumentParser(description='Interference Analysis')
    parser.add_argument('--tier', type=int, default=3)
    parser.add_argument('--all', action='store_true')
    args = parser.parse_args()

    if args.all:
        for tier in [1, 2, 3, 4]:
            a = analyze_tier_interference(tier)
            print_interference_report(a)
    else:
        a = analyze_tier_interference(args.tier)
        print_interference_report(a)


if __name__ == '__main__':
    main()
