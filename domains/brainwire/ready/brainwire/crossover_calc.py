"""MDD ↔ Panic Disorder Crossover Analysis Calculator.

Identifies mathematical intersections between depression and panic disorder models.

Core Equations:
  Eq X1: Shared circuit overlap — |MDD_circuits ∩ Panic_circuits| / |MDD ∪ Panic|
  Eq X2: Profile distance — d(MDD, Panic) = √Σ w_i(V_MDD_i - V_Panic_i)²
  Eq X3: Comorbidity transition — P(MDD→Panic) = σ(Σ shared_pathway_weights - θ)
  Eq X4: Shared STDP framework — both use W(n) = W_ceil-(W_ceil-W_0)×(1-a)^(n×η)
  Eq X5: Contraindication index — variables with opposite treatment directions

Findings:
  1. Shared subcortical targets: Amygdala, LC (via PFC/ACC)
  2. Both models share STDP potentiation framework (D2≡P2)
  3. PFC direction is OPPOSITE: MDD=suppress(1.5→0.8), Panic=activate(0.4→1.3)
  4. NE direction is SAME: both need NE reduction
  5. Comorbid treatment requires sequential protocol (not simultaneous)
"""
import math

from brainwire.depression_calc import (
    MDD_PATHOLOGY, MDD_TREATMENT, MDD_MAINTENANCE, MDD_BASELINE,
    MDD_CIRCUITS, MDD_COVERAGE,
)
from brainwire.panic_calc import (
    PANIC_PATHOLOGY, PANIC_TREATMENT, PANIC_MAINTENANCE, PANIC_BASELINE,
    PANIC_CIRCUITS, PANIC_COVERAGE,
)
from brainwire.variables import VAR_NAMES, TENSION_WEIGHTS


# ══════════════════════════════════════════════════════════════════════════
# Eq X1: Shared Circuit Overlap
# ══════════════════════════════════════════════════════════════════════════

def circuit_overlap() -> dict:
    """Jaccard similarity of subcortical target sets."""
    mdd_targets = set(MDD_COVERAGE.keys())
    panic_targets = set(PANIC_COVERAGE.keys())
    shared = mdd_targets & panic_targets
    union = mdd_targets | panic_targets
    jaccard = len(shared) / len(union) if union else 0
    return {
        'mdd_targets': mdd_targets,
        'panic_targets': panic_targets,
        'shared': shared,
        'mdd_only': mdd_targets - panic_targets,
        'panic_only': panic_targets - mdd_targets,
        'jaccard': jaccard,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq X2: Profile Distance (weighted Euclidean)
# ══════════════════════════════════════════════════════════════════════════

def profile_distance(profile_a: dict, profile_b: dict,
                     weights: dict | None = None) -> float:
    """d = √Σ w_i × (V_a_i - V_b_i)²."""
    if weights is None:
        weights = TENSION_WEIGHTS
    return math.sqrt(sum(weights.get(v, 1.0) * (profile_a[v] - profile_b[v]) ** 2
                         for v in VAR_NAMES))


def all_profile_distances() -> dict:
    """Compute distances between all profile pairs."""
    profiles = {
        'MDD_path': MDD_PATHOLOGY,
        'MDD_tx': MDD_TREATMENT,
        'Panic_path': PANIC_PATHOLOGY,
        'Panic_tx': PANIC_TREATMENT,
        'Baseline': MDD_BASELINE,
    }
    distances = {}
    keys = list(profiles.keys())
    for i, k1 in enumerate(keys):
        for k2 in keys[i + 1:]:
            d = profile_distance(profiles[k1], profiles[k2])
            distances[f"{k1}↔{k2}"] = d
    return distances


# ══════════════════════════════════════════════════════════════════════════
# Eq X3: Comorbidity Transition Probability
# ══════════════════════════════════════════════════════════════════════════

def comorbidity_transition(shared_weight: float = 0.0,
                           theta: float = 1.5) -> dict:
    """P(MDD→Panic) = σ(shared_pathway_weight - θ).

    Models probability that untreated MDD transitions to comorbid panic.
    Shared pathways (NE↑, GABA↓, 5HT↓) create vulnerability.
    """
    # Compute shared pathway weight from overlapping pathology
    shared_vars = {}
    for v in VAR_NAMES:
        mdd_dev = abs(MDD_PATHOLOGY[v] - 1.0)
        panic_dev = abs(PANIC_PATHOLOGY[v] - 1.0)
        # Same direction deviation = shared vulnerability
        mdd_dir = MDD_PATHOLOGY[v] - 1.0
        panic_dir = PANIC_PATHOLOGY[v] - 1.0
        if mdd_dir * panic_dir > 0:  # same direction
            shared_vars[v] = min(mdd_dev, panic_dev) * TENSION_WEIGHTS.get(v, 1.0)
        else:
            shared_vars[v] = 0.0

    total_shared = sum(shared_vars.values())
    z = total_shared - theta
    p_transition = 1.0 / (1.0 + math.exp(-z))

    return {
        'shared_vars': {k: v for k, v in shared_vars.items() if v > 0},
        'total_shared_weight': total_shared,
        'theta': theta,
        'p_transition': p_transition,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq X4: Shared STDP Framework Comparison
# ══════════════════════════════════════════════════════════════════════════

def stdp_comparison() -> dict:
    """Compare STDP parameters between MDD (D2) and Panic (P2)."""
    mdd_params = {'a_plus': 0.005, 'eta': 0.8, 'w0': 0.5, 'w_ceil': 1.0,
                  'pulses_per': 1000, 'sessions': 30, 'pathway': 'PFC→Raphe (excitatory)'}
    panic_params = {'a_plus': 0.004, 'eta': 0.8, 'w0': 0.4, 'w_ceil': 1.0,
                    'pulses_per': 800, 'sessions': 40, 'pathway': 'vmPFC→Amygdala (inhibitory)'}

    # Effective pulses to 90% recovery
    for params in [mdd_params, panic_params]:
        target_w = params['w0'] + 0.9 * (params['w_ceil'] - params['w0'])
        # W(n) = ceil - (ceil-w0)(1-a)^(n*eta) = target_w
        # (1-a)^(n*eta) = (ceil-target_w)/(ceil-w0)
        ratio = (params['w_ceil'] - target_w) / (params['w_ceil'] - params['w0'])
        if ratio > 0:
            n_eff = math.log(ratio) / math.log(1 - params['a_plus'])
            params['pulses_to_90pct'] = n_eff
            params['sessions_to_90pct'] = n_eff / (params['pulses_per'] * params['eta'])
        else:
            params['pulses_to_90pct'] = 0
            params['sessions_to_90pct'] = 0

    return {
        'shared_formula': 'W(n) = W_ceil - (W_ceil - W_0) × (1 - a+)^(n × η)',
        'mdd': mdd_params,
        'panic': panic_params,
        'differences': {
            'a_plus': 'MDD 0.005 > Panic 0.004 (inhibitory synapses potentiate slower)',
            'w0': 'MDD 0.5 > Panic 0.4 (panic has more severe pathway deficit)',
            'sessions': 'MDD 30 < Panic 40 (panic needs longer treatment)',
            'pulses': 'MDD 1000 > Panic 800 (inhibitory STDP shorter window)',
        },
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq X5: Contraindication Index (opposite treatment directions)
# ══════════════════════════════════════════════════════════════════════════

def contraindication_index() -> dict:
    """Variables where MDD and Panic treatment targets are in opposite directions.

    These prevent simultaneous treatment — must be sequenced.
    """
    contraindications = {}
    aligned = {}
    for v in VAR_NAMES:
        mdd_dir = MDD_TREATMENT[v] - 1.0  # >0: increase, <0: decrease
        panic_dir = PANIC_TREATMENT[v] - 1.0
        if mdd_dir * panic_dir < 0:  # opposite directions
            contraindications[v] = {
                'mdd_target': MDD_TREATMENT[v],
                'panic_target': PANIC_TREATMENT[v],
                'conflict': f"MDD={'↑' if mdd_dir > 0 else '↓'} vs Panic={'↑' if panic_dir > 0 else '↓'}",
            }
        elif abs(mdd_dir) > 0.05 and abs(panic_dir) > 0.05:
            aligned[v] = {
                'mdd_target': MDD_TREATMENT[v],
                'panic_target': PANIC_TREATMENT[v],
                'direction': '↑' if mdd_dir > 0 else '↓',
            }

    ci = len(contraindications) / len(VAR_NAMES)
    return {
        'contraindicated_vars': contraindications,
        'aligned_vars': aligned,
        'n_contraindicated': len(contraindications),
        'n_aligned': len(aligned),
        'contraindication_index': ci,
        'simultaneous_safe': ci < 0.25,
    }


# ══════════════════════════════════════════════════════════════════════════
# Comorbid Sequential Protocol
# ══════════════════════════════════════════════════════════════════════════

def comorbid_protocol() -> dict:
    """Recommended treatment sequence for comorbid MDD + Panic.

    Based on contraindication analysis: treat panic first (acute safety),
    then transition to MDD treatment once panic is stabilized.
    """
    ci = contraindication_index()
    return {
        'phase_1': {
            'target': 'Panic stabilization',
            'duration': '12 weeks (40 sessions)',
            'rationale': 'Acute panic attacks pose immediate safety risk',
            'key_vars': ['NE↓', 'GABA↑', 'PFC↑'],
            'contraindication_safe': True,
        },
        'phase_2': {
            'target': 'MDD treatment',
            'duration': '8 weeks (30 sessions)',
            'rationale': 'Once panic stabilized, PFC direction can be reversed',
            'key_vars': ['5HT↑', 'DA↑', 'PFC↓ (rumination suppression)'],
            'note': 'PFC direction REVERSES from Phase 1 — must wait for panic stability',
        },
        'phase_3': {
            'target': 'Maintenance (both conditions)',
            'duration': 'Ongoing',
            'shared_maintenance': {v: (MDD_MAINTENANCE[v] + PANIC_MAINTENANCE[v]) / 2
                                    for v in VAR_NAMES},
        },
        'total_sessions': 70,
        'contraindications': ci['contraindicated_vars'],
    }


# ══════════════════════════════════════════════════════════════════════════
# Report
# ══════════════════════════════════════════════════════════════════════════

def print_report():
    """Print full crossover analysis report."""
    print(f"\n{'=' * 70}")
    print(f"  MDD ↔ Panic Disorder Crossover Analysis")
    print(f"  Mathematical Intersection Finding")
    print(f"{'=' * 70}")

    # X1: Circuit overlap
    print(f"\n  === Eq X1: Circuit Overlap ===")
    co = circuit_overlap()
    print(f"  MDD targets:   {co['mdd_targets']}")
    print(f"  Panic targets: {co['panic_targets']}")
    print(f"  Shared:        {co['shared']}")
    print(f"  MDD only:      {co['mdd_only']}")
    print(f"  Panic only:    {co['panic_only']}")
    print(f"  Jaccard sim:   {co['jaccard']:.3f}")

    # X2: Profile distances
    print(f"\n  === Eq X2: Profile Distances ===")
    print(f"  d = √Σ w_i(V_a - V_b)²")
    dists = all_profile_distances()
    for pair, d in sorted(dists.items(), key=lambda x: x[1]):
        print(f"    {pair:<30} d={d:.3f}")

    # X3: Comorbidity
    print(f"\n  === Eq X3: Comorbidity Transition ===")
    ct = comorbidity_transition()
    print(f"  Shared vulnerability vars: {list(ct['shared_vars'].keys())}")
    print(f"  Total shared weight: {ct['total_shared_weight']:.3f}")
    print(f"  P(MDD→Panic): {ct['p_transition']:.4f}")

    # X4: STDP comparison
    print(f"\n  === Eq X4: Shared STDP Framework ===")
    sc = stdp_comparison()
    print(f"  Formula: {sc['shared_formula']}")
    print(f"  MDD:   a+={sc['mdd']['a_plus']}, w0={sc['mdd']['w0']}, "
          f"sessions_to_90%={sc['mdd']['sessions_to_90pct']:.1f}")
    print(f"  Panic: a+={sc['panic']['a_plus']}, w0={sc['panic']['w0']}, "
          f"sessions_to_90%={sc['panic']['sessions_to_90pct']:.1f}")
    for key, desc in sc['differences'].items():
        print(f"    {key}: {desc}")

    # X5: Contraindications
    print(f"\n  === Eq X5: Contraindication Index ===")
    ci = contraindication_index()
    print(f"  Contraindicated: {ci['n_contraindicated']}/12 variables")
    print(f"  Aligned:         {ci['n_aligned']}/12 variables")
    print(f"  CI index:        {ci['contraindication_index']:.3f}")
    print(f"  Simultaneous OK: {ci['simultaneous_safe']}")
    if ci['contraindicated_vars']:
        print(f"\n  Conflicts:")
        for v, info in ci['contraindicated_vars'].items():
            print(f"    {v:<12} {info['conflict']}  (MDD={info['mdd_target']:.1f}, Panic={info['panic_target']:.1f})")
    if ci['aligned_vars']:
        print(f"\n  Aligned:")
        for v, info in ci['aligned_vars'].items():
            print(f"    {v:<12} Both {info['direction']}  (MDD={info['mdd_target']:.1f}, Panic={info['panic_target']:.1f})")

    # Comorbid protocol
    print(f"\n  === Comorbid Sequential Protocol ===")
    cp = comorbid_protocol()
    for phase in ['phase_1', 'phase_2', 'phase_3']:
        p = cp[phase]
        print(f"\n  {phase.upper()}: {p['target']}")
        print(f"    Duration: {p['duration']}")
        if 'rationale' in p:
            print(f"    Rationale: {p['rationale']}")
        if 'key_vars' in p:
            print(f"    Key vars: {p['key_vars']}")
    print(f"\n  Total sessions: {cp['total_sessions']}")


if __name__ == '__main__':
    print_report()
