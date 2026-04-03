#!/usr/bin/env python3
"""n6-architecture growth scanner — JSON stdout, no Claude CLI dependency.

Called by infinite_growth.sh or nexus6_hook --mode growth-scan.
Output: {"opportunities": [...], "growth_delta": N} JSON to stdout.

Scans:
  - Incomplete DSE domains (dse-map.toml entries without results)
  - Unverified breakthrough theorems (docs/breakthrough-theorems.md)
  - Missing testable predictions verification
  - Domains without extreme-hypotheses.md
  - Technique files without experiments
"""
import glob
import json
import os
import re
import sys
import time

_THIS_DIR = os.path.dirname(os.path.abspath(__file__))
_ROOT = os.path.normpath(os.path.join(_THIS_DIR, '..'))
_DOCS = os.path.join(_ROOT, 'docs')
_TECHNIQUES = os.path.join(_ROOT, 'techniques')
_EXPERIMENTS = os.path.join(_ROOT, 'experiments')
_DSE_MAP = os.path.join(_DOCS, 'dse-map.toml')
_BT_FILE = os.path.join(_DOCS, 'breakthrough-theorems.md')
_TP_FILE = os.path.join(_DOCS, 'testable-predictions.md')
_GROWTH_STATE = os.path.join(_THIS_DIR, 'growth_state.json')
_NEXUS_SHARED = os.path.join(os.environ.get('HOME', ''), 'Dev', 'nexus6', 'shared')
_GROWTH_BUS = os.path.join(_NEXUS_SHARED, 'growth_bus.jsonl')
_GROWTH_REGISTRY = os.path.join(_NEXUS_SHARED, 'growth-registry.json')


# ── Scanners ──────────────────────────────────────────


def scan_incomplete_dse():
    """Find DSE domains in dse-map.toml that lack results."""
    if not os.path.exists(_DSE_MAP):
        return []
    results = []
    try:
        with open(_DSE_MAP) as f:
            content = f.read()
        # Simple TOML parser — find [section] blocks and check dse = "done"/"wip"/"none"
        current_section = None
        section_data = {}
        for line in content.split('\n'):
            line = line.strip()
            # Section header
            m = re.match(r'^\[([a-zA-Z0-9_-]+)\]$', line)
            if m:
                if current_section and current_section != 'meta':
                    section_data[current_section] = section_data.get(current_section, {})
                current_section = m.group(1)
                section_data.setdefault(current_section, {})
                continue
            if current_section and '=' in line:
                key, _, val = line.partition('=')
                key = key.strip()
                val = val.strip().strip('"').strip("'")
                section_data.setdefault(current_section, {})[key] = val

        incomplete = []
        wip = []
        for name, data in section_data.items():
            if name == 'meta':
                continue
            status = data.get('dse', 'none')
            if status == 'none':
                incomplete.append(name)
            elif status == 'wip':
                wip.append(name)

        if incomplete:
            results.append({
                'type': 'DSE_INCOMPLETE',
                'priority': 'HIGH',
                'description': '%d DSE domains have no results: %s' % (
                    len(incomplete), ', '.join(sorted(incomplete)[:5])),
                'count': len(incomplete),
                'items': sorted(incomplete)[:10],
                'growth_value': len(incomplete),
            })
        if wip:
            results.append({
                'type': 'DSE_WIP',
                'priority': 'MEDIUM',
                'description': '%d DSE domains work-in-progress: %s' % (
                    len(wip), ', '.join(sorted(wip)[:5])),
                'count': len(wip),
                'items': sorted(wip)[:10],
                'growth_value': len(wip),
            })
    except Exception as e:
        results.append({
            'type': 'DSE_SCAN_ERROR',
            'priority': 'LOW',
            'description': 'DSE scan error: %s' % str(e)[:80],
        })
    return results


def scan_unverified_bts():
    """Find breakthrough theorems without verification status."""
    if not os.path.exists(_BT_FILE):
        return []
    results = []
    try:
        with open(_BT_FILE) as f:
            content = f.read()
        # Count BT entries
        bt_ids = re.findall(r'\*\*BT-(\d+)\*\*', content)
        total_bts = len(set(bt_ids))

        # Look for verification markers
        verified_markers = len(re.findall(
            r'(?:verified|EXACT|proved|confirmed)', content, re.IGNORECASE))

        # Check for verification files in docs subdirectories
        verification_files = glob.glob(os.path.join(_DOCS, '*/verification.md'))
        verified_domains = len(verification_files)

        # Heuristic: each BT should have at least some verification
        unverified_est = max(0, total_bts - verified_markers // 3)

        if unverified_est > 10:
            results.append({
                'type': 'BT_UNVERIFIED',
                'priority': 'HIGH',
                'description': '~%d/%d BTs may need deeper verification (%d domain verification files)' % (
                    unverified_est, total_bts, verified_domains),
                'count': unverified_est,
                'total': total_bts,
                'growth_value': min(unverified_est, 20),
            })

        # Check max BT number for gap detection
        if bt_ids:
            max_bt = max(int(x) for x in bt_ids)
            gaps = [i for i in range(1, max_bt + 1) if str(i) not in bt_ids]
            if gaps:
                results.append({
                    'type': 'BT_GAPS',
                    'priority': 'LOW',
                    'description': '%d BT ID gaps detected (max BT-%d, %d entries)' % (
                        len(gaps), max_bt, total_bts),
                    'count': len(gaps),
                })
    except Exception as e:
        results.append({
            'type': 'BT_SCAN_ERROR',
            'priority': 'LOW',
            'description': 'BT scan error: %s' % str(e)[:80],
        })
    return results


def scan_testable_predictions():
    """Check testable predictions verification progress."""
    if not os.path.exists(_TP_FILE):
        return [{'type': 'TP_MISSING', 'priority': 'HIGH',
                 'description': 'testable-predictions.md not found'}]
    results = []
    try:
        with open(_TP_FILE) as f:
            content = f.read()
        # Count predictions and their verification status
        tier_counts = {}
        for m in re.finditer(r'Tier\s+(\d+)', content):
            t = m.group(1)
            tier_counts[t] = tier_counts.get(t, 0) + 1

        total_predictions = len(re.findall(r'(?:TP-\d+|\|\s*\d+\s*\|)', content))
        verified = len(re.findall(r'(?:verified|confirmed|PASS)', content, re.IGNORECASE))
        pending = max(0, total_predictions - verified)

        if pending > 5:
            results.append({
                'type': 'TP_PENDING',
                'priority': 'MEDIUM',
                'description': '%d/%d testable predictions unverified' % (
                    pending, total_predictions),
                'count': pending,
                'total': total_predictions,
                'growth_value': min(pending, 10),
            })
    except Exception as e:
        results.append({
            'type': 'TP_SCAN_ERROR',
            'priority': 'LOW',
            'description': 'TP scan error: %s' % str(e)[:80],
        })
    return results


def scan_missing_extreme_hypotheses():
    """Find domain directories without extreme-hypotheses.md."""
    results = []
    try:
        # Get all domain directories under docs/
        domain_dirs = [d for d in os.listdir(_DOCS)
                       if os.path.isdir(os.path.join(_DOCS, d))]
        missing = []
        for d in domain_dirs:
            extreme_path = os.path.join(_DOCS, d, 'extreme-hypotheses.md')
            if not os.path.exists(extreme_path):
                missing.append(d)

        if missing:
            results.append({
                'type': 'EXTREME_MISSING',
                'priority': 'MEDIUM',
                'description': '%d domains without extreme-hypotheses.md: %s' % (
                    len(missing), ', '.join(sorted(missing)[:5])),
                'count': len(missing),
                'items': sorted(missing)[:15],
                'growth_value': len(missing),
            })
    except Exception as e:
        results.append({
            'type': 'EXTREME_SCAN_ERROR',
            'priority': 'LOW',
            'description': 'Extreme hypotheses scan error: %s' % str(e)[:80],
        })
    return results


def scan_technique_experiments():
    """Find technique .py files without corresponding experiments."""
    results = []
    try:
        techniques = [os.path.splitext(os.path.basename(f))[0]
                      for f in glob.glob(os.path.join(_TECHNIQUES, '*.py'))
                      if not f.endswith('__init__.py')]
        experiments = set()
        for f in glob.glob(os.path.join(_EXPERIMENTS, '*.py')):
            experiments.add(os.path.basename(f).lower())

        uncovered = []
        for t in techniques:
            # Check if any experiment references this technique
            found = False
            t_lower = t.lower()
            for exp in experiments:
                if t_lower in exp or t_lower.replace('_', '') in exp.replace('_', ''):
                    found = True
                    break
            if not found:
                uncovered.append(t)

        if uncovered:
            results.append({
                'type': 'TECHNIQUE_NO_EXPERIMENT',
                'priority': 'LOW',
                'description': '%d techniques without experiments: %s' % (
                    len(uncovered), ', '.join(sorted(uncovered)[:5])),
                'count': len(uncovered),
                'items': sorted(uncovered)[:10],
                'growth_value': len(uncovered),
            })
    except Exception as e:
        results.append({
            'type': 'TECHNIQUE_SCAN_ERROR',
            'priority': 'LOW',
            'description': 'Technique scan error: %s' % str(e)[:80],
        })
    return results


def scan_cross_dse_opportunities():
    """Find domains that are done but not yet cross-DSE linked."""
    if not os.path.exists(_DSE_MAP):
        return []
    results = []
    try:
        with open(_DSE_MAP) as f:
            content = f.read()
        # Count done domains and cross-DSE connections
        done_domains = len(re.findall(r'dse\s*=\s*"done"', content))
        cross_dse_entries = len(re.findall(r'cross_dse\s*=', content))
        unlinked = max(0, done_domains - cross_dse_entries)

        if unlinked > 0:
            results.append({
                'type': 'CROSS_DSE_GAP',
                'priority': 'MEDIUM',
                'description': '%d done domains without cross-DSE links' % unlinked,
                'count': unlinked,
                'growth_value': unlinked * 2,
            })
    except Exception:
        pass
    return results


def scan_documentation_completeness():
    """Check for domains with hypotheses but no goal.md or verification.md."""
    results = []
    try:
        domain_dirs = [d for d in os.listdir(_DOCS)
                       if os.path.isdir(os.path.join(_DOCS, d))]
        missing_goal = []
        missing_verify = []
        for d in domain_dirs:
            dpath = os.path.join(_DOCS, d)
            has_hyp = os.path.exists(os.path.join(dpath, 'hypotheses.md'))
            has_goal = os.path.exists(os.path.join(dpath, 'goal.md'))
            has_verify = os.path.exists(os.path.join(dpath, 'verification.md'))
            if has_hyp and not has_goal:
                missing_goal.append(d)
            if has_hyp and not has_verify:
                missing_verify.append(d)

        if missing_goal:
            results.append({
                'type': 'DOC_MISSING_GOAL',
                'priority': 'LOW',
                'description': '%d domains with hypotheses but no goal.md: %s' % (
                    len(missing_goal), ', '.join(sorted(missing_goal)[:5])),
                'count': len(missing_goal),
                'items': sorted(missing_goal)[:10],
            })
        if missing_verify:
            results.append({
                'type': 'DOC_MISSING_VERIFY',
                'priority': 'MEDIUM',
                'description': '%d domains with hypotheses but no verification.md: %s' % (
                    len(missing_verify), ', '.join(sorted(missing_verify)[:5])),
                'count': len(missing_verify),
                'items': sorted(missing_verify)[:10],
                'growth_value': len(missing_verify),
            })
    except Exception:
        pass
    return results


# ── Growth State + NEXUS-6 Sync ──────────────────────


def sync_nexus6_growth(opportunities, growth_delta):
    """Write to NEXUS-6 growth bus + registry for cross-repo sync."""
    nexus_bonus = 0

    # 1) Write to growth bus (append JSONL)
    try:
        entry = {
            'ts': time.strftime('%Y-%m-%dT%H:%M:%S'),
            'repo': 'n6-architecture',
            'opportunities': len(opportunities),
            'growth_delta': growth_delta,
            'types': list(set(o.get('type', '') for o in opportunities)),
        }
        with open(_GROWTH_BUS, 'a') as f:
            f.write(json.dumps(entry, ensure_ascii=False) + '\n')
    except Exception:
        pass

    # 2) Update growth registry
    try:
        registry = {}
        if os.path.exists(_GROWTH_REGISTRY):
            with open(_GROWTH_REGISTRY) as f:
                registry = json.load(f)

        dse_count = sum(1 for o in opportunities if 'DSE' in o.get('type', ''))
        bt_count = sum(1 for o in opportunities if 'BT' in o.get('type', ''))
        tp_count = sum(1 for o in opportunities if 'TP' in o.get('type', ''))

        registry['n6-architecture'] = {
            'last_scan': time.strftime('%Y-%m-%dT%H:%M:%S'),
            'opportunities': len(opportunities),
            'dse_gaps': dse_count,
            'bt_unverified': bt_count,
            'tp_pending': tp_count,
            'growth_delta': growth_delta,
        }

        with open(_GROWTH_REGISTRY, 'w') as f:
            json.dump(registry, f, indent=2, ensure_ascii=False)
    except Exception:
        pass

    # 3) Cross-repo resonance bonus
    try:
        if os.path.exists(_GROWTH_REGISTRY):
            with open(_GROWTH_REGISTRY) as f:
                registry = json.load(f)
            active_repos = [k for k in registry
                           if isinstance(registry[k], dict) and 'last_scan' in registry[k]]
            if len(active_repos) >= 3:
                nexus_bonus += 1  # Cross-repo resonance
    except Exception:
        pass

    return nexus_bonus


def update_growth_state(opportunities):
    """Update growth_state.json with scan results + NEXUS-6 sync."""
    if not os.path.exists(_GROWTH_STATE):
        return 0
    try:
        with open(_GROWTH_STATE) as f:
            growth = json.load(f)

        # Calculate growth delta from opportunities
        growth_delta = 0
        for o in opportunities:
            gv = o.get('growth_value', 0)
            if gv > 0:
                growth_delta += max(1, gv // 5)
            elif o.get('priority') == 'HIGH':
                growth_delta += 2
            elif o.get('priority') == 'MEDIUM':
                growth_delta += 1

        # NEXUS-6 sync bonus
        nexus_bonus = sync_nexus6_growth(opportunities, growth_delta)
        growth_delta += nexus_bonus

        if growth_delta > 0:
            prev_count = growth.get('scan_count', 0)
            growth['scan_count'] = prev_count + 1
            growth['total_growth'] = growth.get('total_growth', 0) + growth_delta
            growth['last_scan'] = time.strftime('%Y-%m-%dT%H:%M:%S')
            growth['last_delta'] = growth_delta
            growth['nexus6_bonus'] = nexus_bonus
            growth['opportunity_count'] = len(opportunities)

            # Track high-water marks
            hwm = growth.get('high_water', {})
            for o in opportunities:
                t = o.get('type', '')
                c = o.get('count', 0)
                if c > hwm.get(t, 0):
                    hwm[t] = c
            growth['high_water'] = hwm

            with open(_GROWTH_STATE, 'w') as f:
                json.dump(growth, f, indent=2, ensure_ascii=False)

        return growth_delta
    except Exception:
        return 0


# ── Main ─────────────────────────────────────────────


def main():
    all_opps = []
    scanners = [
        scan_incomplete_dse,
        scan_unverified_bts,
        scan_testable_predictions,
        scan_missing_extreme_hypotheses,
        scan_technique_experiments,
        scan_cross_dse_opportunities,
        scan_documentation_completeness,
    ]
    for scanner in scanners:
        try:
            all_opps.extend(scanner())
        except Exception:
            pass

    # Sort by priority
    prio_order = {'HIGH': 0, 'MEDIUM': 1, 'LOW': 2}
    all_opps.sort(key=lambda x: prio_order.get(x.get('priority', 'LOW'), 9))

    # Update growth state + NEXUS-6 sync
    growth_delta = update_growth_state(all_opps)

    print(json.dumps({
        'opportunities': all_opps,
        'growth_delta': growth_delta,
    }, ensure_ascii=False))


if __name__ == '__main__':
    main()
