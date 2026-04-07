"""Cortical-to-Deep Brain Projection Map — 15 subcortical structures × 9 cortical sources."""

# Full projection matrix: cortical_source → {deep_target: f_project}
PROJECTIONS = {
    'PFC': {
        'VTA': 0.08, 'LC': 0.03, 'Raphe': 0.05, 'Hippocampus': 0.05,
        'Thalamus': 0.15, 'Amygdala': 0.10, 'Hypothalamus': 0.08,
        'Basal_Ganglia': 0.12, 'NAc': 0.10, 'PAG': 0.03,
        'SN': 0.05, 'STN': 0.08, 'Cerebellum': 0.08,
    },
    'ACC': {
        'VTA': 0.06, 'LC': 0.04, 'Raphe': 0.04, 'Hypothalamus': 0.06,
        'NAc': 0.08, 'PAG': 0.05, 'NTS': 0.02, 'PBN': 0.02,
    },
    'Insula': {
        'LC': 0.03, 'Raphe': 0.03, 'Amygdala': 0.12,
        'Hypothalamus': 0.10, 'PAG': 0.04, 'NTS': 0.05, 'PBN': 0.03,
    },
    'Motor': {
        'Thalamus': 0.20, 'Basal_Ganglia': 0.15, 'SN': 0.08, 'STN': 0.10,
        'Cerebellum': 0.12,  # Motor→Pons→Cerebellum (2-synapse, corticopontocerebellar)
    },
    'OFC': {'VTA': 0.06, 'NAc': 0.12},
    'Temporal': {'Hippocampus': 0.08, 'Amygdala': 0.15},
    'SMA': {'Basal_Ganglia': 0.10, 'SN': 0.06},
    'EC': {'Hippocampus': 0.40},
    'V1': {'Thalamus': 0.10},
}

ALL_DEEP = ['VTA','LC','Raphe','Hippocampus','Thalamus','Amygdala','Hypothalamus',
            'Basal_Ganglia','NAc','PAG','SN','STN','Cerebellum','NTS','PBN']

DEPTHS = {
    'Amygdala': 35, 'Hippocampus': 40, 'Basal_Ganglia': 45, 'Thalamus': 50,
    'NAc': 50, 'Hypothalamus': 55, 'STN': 55, 'SN': 65, 'Cerebellum': 70,
    'VTA': 75, 'PBN': 80, 'PAG': 85, 'LC': 90, 'Raphe': 90, 'NTS': 95,
}

def get_all_sources(target: str) -> dict[str, float]:
    """Get all cortical sources projecting to a deep target."""
    sources = {}
    for src, targets in PROJECTIONS.items():
        if target in targets:
            sources[src] = targets[target]
    return sources

def get_all_targets(source: str) -> dict[str, float]:
    """Get all deep targets reachable from a cortical source."""
    return PROJECTIONS.get(source, {})

def coverage_check() -> dict:
    """Check which deep structures are covered."""
    covered = set()
    for src, targets in PROJECTIONS.items():
        covered.update(targets.keys())
    uncovered = set(ALL_DEEP) - covered
    return {'covered': len(covered), 'total': len(ALL_DEEP), 'uncovered': uncovered}

def max_coefficient_per_target() -> dict:
    """For each deep target, find the strongest cortical projection."""
    result = {}
    for target in ALL_DEEP:
        sources = get_all_sources(target)
        if sources:
            best_src = max(sources, key=sources.get)
            result[target] = {'source': best_src, 'f_project': sources[best_src], 'n_sources': len(sources)}
        else:
            result[target] = {'source': None, 'f_project': 0, 'n_sources': 0}
    return result

def main():
    print(f"\n{'='*70}")
    print(f"  Cortical → Deep Brain Projection Map")
    print(f"{'='*70}")

    # Full matrix
    sources = sorted(PROJECTIONS.keys())
    print(f"\n  {'Target':<16} {'Depth':>5}", end="")
    for s in sources:
        print(f" {s:>6}", end="")
    print(f" {'Best':>6} {'#src':>4}")
    print(f"  {'-'*16} {'-'*5}", end="")
    for _ in sources:
        print(f" {'-'*6}", end="")
    print(f" {'-'*6} {'-'*4}")

    for target in ALL_DEEP:
        depth = DEPTHS.get(target, 0)
        print(f"  {target:<16} {depth:>4}mm", end="")
        all_src = get_all_sources(target)
        for s in sources:
            f = all_src.get(s, 0)
            if f > 0:
                print(f" {f:>5.2f}", end="")
            else:
                print(f"     .", end="")
        best = max(all_src.values()) if all_src else 0
        print(f" {best:>5.2f} {len(all_src):>4}")

    # Coverage
    cov = coverage_check()
    print(f"\n  Coverage: {cov['covered']}/{cov['total']}")
    if cov['uncovered']:
        print(f"  Uncovered: {cov['uncovered']}")
    else:
        print(f"  All structures covered (Theorem 6 verified)")

if __name__ == '__main__':
    main()
