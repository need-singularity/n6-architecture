"""N1 Implant Placement Optimizer — computes J(l) score for candidate locations."""
import math

# 10 candidate cortical locations
LOCATIONS = {
    'F3':  {'name': 'Left DLPFC', 'ba': 'BA46', 'deep_targets': ['VTA','LC','Raphe','Hippocampus','Thalamus','Amygdala','Hypothalamus','Basal_Ganglia','NAc','PAG','SN','STN','Cerebellum'], 'g_control': {'D': True, 'P': True, 'I': True}},
    'F4':  {'name': 'Right DLPFC', 'ba': 'BA46', 'deep_targets': ['VTA','LC','Raphe','Hippocampus','Thalamus','Amygdala','Hypothalamus','Basal_Ganglia','NAc','PAG'], 'g_control': {'D': False, 'P': True, 'I': True}},
    'Fz':  {'name': 'Medial PFC', 'ba': 'BA10', 'deep_targets': ['VTA','Raphe','Hypothalamus','NAc','PAG'], 'g_control': {'D': False, 'P': True, 'I': True}},
    'C3':  {'name': 'Left Motor', 'ba': 'BA4', 'deep_targets': ['Thalamus','Basal_Ganglia','SN','STN'], 'g_control': {'D': False, 'P': False, 'I': False}},
    'C4':  {'name': 'Right Motor', 'ba': 'BA4', 'deep_targets': ['Thalamus','Basal_Ganglia','SN','STN'], 'g_control': {'D': False, 'P': False, 'I': False}},
    'P3':  {'name': 'Left Parietal', 'ba': 'BA7', 'deep_targets': ['Thalamus','Hippocampus'], 'g_control': {'D': True, 'P': False, 'I': False}},
    'P4':  {'name': 'Right Parietal', 'ba': 'BA7', 'deep_targets': ['Thalamus','Hippocampus'], 'g_control': {'D': True, 'P': False, 'I': False}},
    'Oz':  {'name': 'Occipital', 'ba': 'BA17', 'deep_targets': ['Thalamus'], 'g_control': {'D': False, 'P': False, 'I': False}},
    'Insula': {'name': 'Anterior Insula', 'ba': 'BA13', 'deep_targets': ['LC','Raphe','Amygdala','Hypothalamus','PAG','NTS','PBN'], 'g_control': {'D': False, 'P': False, 'I': False}},
    'ACC': {'name': 'Anterior Cingulate', 'ba': 'BA24', 'deep_targets': ['VTA','LC','Raphe','Hypothalamus','NAc','PAG','NTS','PBN'], 'g_control': {'D': False, 'P': True, 'I': False}},
}

ALL_DEEP = ['VTA','LC','Raphe','Hippocampus','Thalamus','Amygdala','Hypothalamus','Basal_Ganglia','NAc','PAG','SN','STN','Cerebellum','NTS','PBN']

def compute_j(loc: str, w_deep=0.4, w_cortical=0.3, w_g=0.3) -> dict:
    """Compute placement score J(l). From P-001 Theorem 3."""
    info = LOCATIONS[loc]
    deep_score = len(info['deep_targets']) / len(ALL_DEEP)
    cortical_score = 0.85 if 'PFC' in info['name'] or 'DLPFC' in info['name'] else 0.5
    g_score = sum(info['g_control'].values()) / 3.0
    j = w_deep * deep_score + w_cortical * cortical_score + w_g * g_score
    return {'location': loc, 'name': info['name'], 'J': j, 'deep': deep_score, 'cortical': cortical_score, 'g': g_score, 'n_deep': len(info['deep_targets'])}

def rank_all() -> list[dict]:
    results = [compute_j(loc) for loc in LOCATIONS]
    return sorted(results, key=lambda x: -x['J'])

def minimum_covering_set() -> dict:
    """Find minimum set of locations to cover all 15 deep structures."""
    uncovered = set(ALL_DEEP)
    selected = []
    available = dict(LOCATIONS)
    while uncovered and available:
        best = max(available.items(), key=lambda x: len(set(x[1]['deep_targets']) & uncovered))
        name, info = best
        covered = set(info['deep_targets']) & uncovered
        if not covered:
            break  # No location can cover remaining structures
        selected.append((name, covered))
        uncovered -= covered
        del available[name]
    return {'locations': selected, 'total': len(selected), 'uncovered': uncovered}

def main():
    print(f"\n{'='*60}")
    print(f"  N1 Implant Placement Optimizer")
    print(f"{'='*60}")
    ranked = rank_all()
    print(f"\n  {'Rank':>4} {'Location':<10} {'Name':<20} {'J':>6} {'Deep':>5} {'G':>5}")
    print(f"  {'-'*4} {'-'*10} {'-'*20} {'-'*6} {'-'*5} {'-'*5}")
    for i, r in enumerate(ranked, 1):
        print(f"  {i:>4} {r['location']:<10} {r['name']:<20} {r['J']:>5.3f} {r['n_deep']:>4}/15 {r['g']:>4.2f}")

    print(f"\n  Minimum covering set:")
    cover = minimum_covering_set()
    for loc, covered in cover['locations']:
        print(f"    {loc}: +{len(covered)} ({', '.join(sorted(covered)[:5])}...)")
    print(f"    Total: {cover['total']} locations for 15/15 structures")

if __name__ == '__main__':
    main()
