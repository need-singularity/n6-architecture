#!/usr/bin/env python3
"""atlas.n6 [10] -> [10*] promotion analyzer (dry-run, v3 fixpoint).

Adds transitive promotion: if an entry would be PROMOTE under v2 rules
once its currently-[10] deps are themselves promoted to [10*], promote
both. Iterate to fixpoint.

Conservative additional rule:
  T5  expr nonempty + all atlas deps verified-OR-pending-promote +
      type ∈ {@C, @R, @F, @L} + line content includes a numeric
      derivation (header expr matches r'.*=.*[0-9].*' or contains
      operators like '*', '+', '/', '-', '^') AND no cross-doc dep.

The aim: promote chains where the equation is in the header itself
(e.g. `L3-water = ...numeric...` with all deps in atlas, all verified),
which currently land in H4b/H2.
"""
import re
import json
import sys
from pathlib import Path

ATLAS = Path.home() / "core/n6-architecture/atlas/atlas.n6"

HEADER_RE = re.compile(r'^(@[A-Z?])\s+(\S+?)(?:\s*=\s*(.+?))?\s*::\s*(\S+)\s+(\[[^\]]+\])\s*$')

VERIFIED_GRADES = {'[10*]', '[10**]', '[11*]', '[12*]', '[10*!]', '[5*]', '[9*]', '[8*]', '[6*]', '[7*]', '[0.10*]', '[0.9*]'}
NUMERIC_OP_RE = re.compile(r'[0-9]|[+\-*/^=≈]')

DEFINITIONAL_TYPES = {'@C', '@R', '@F', '@L', '@S'}

def parse_atlas(path):
    entries = []
    by_id = {}
    cur = None
    line_no = 0
    with open(path, 'r', encoding='utf-8') as f:
        for raw in f:
            line_no += 1
            line = raw.rstrip('\n')
            stripped = line.strip()
            if not stripped or stripped.startswith('#'):
                continue
            is_header = (not line.startswith(' ')) and (not line.startswith('\t')) and stripped.startswith('@')
            if is_header:
                m = HEADER_RE.match(stripped)
                if m:
                    if cur is not None:
                        entries.append(cur)
                    t, eid, expr, domain, grade = m.groups()
                    cur = {
                        'line': line_no,
                        'type': t,
                        'id': eid,
                        'expr': (expr or '').strip(),
                        'domain': domain,
                        'grade': grade,
                        'depends_on': [],
                        'verified_by': [],
                        'has_breakthrough': False,
                        'has_eq': False,
                        'has_converge': False,
                        'has_apply': False,
                    }
                    by_id[eid] = cur
                else:
                    if cur is not None:
                        entries.append(cur)
                    cur = None
            else:
                if cur is None:
                    continue
                if stripped.startswith('<-'):
                    deps = stripped[2:].strip()
                    for d in re.split(r'[,\s]+', deps):
                        d = d.strip()
                        if d:
                            cur['depends_on'].append(d)
                elif stripped.startswith('|>'):
                    cur['verified_by'].append(stripped[2:].strip())
                elif stripped.startswith('!!'):
                    cur['has_breakthrough'] = True
                elif stripped.startswith('=='):
                    cur['has_eq'] = True
                elif stripped.startswith('~>'):
                    cur['has_converge'] = True
                elif stripped.startswith('=>'):
                    cur['has_apply'] = True
        if cur is not None:
            entries.append(cur)
    return entries, by_id

def is_verified(grade):
    return grade in VERIFIED_GRADES

def looks_like_doc_ref(dep):
    return ('/' in dep) or ('.md' in dep) or dep.startswith('§') or dep.startswith('see-')

def expr_is_numeric_derivation(expr):
    if not expr:
        return False
    e = expr.strip()
    el = e.lower()
    # Reject placeholder/no-mapping strings
    if el in ('misc', '?', '...'):
        return False
    # Reject "no mapping / cannot approximate" declarations
    NO_MAP_MARKERS = ['단순 매핑 없음', '직접 매핑 없음', '단순매핑불가',
                       '근사 불가', 'no mapping', 'cannot approximate']
    for m in NO_MAP_MARKERS:
        if m in e:
            return False
    # has at least one digit OR operator OR known math symbol
    return bool(NUMERIC_OP_RE.search(e))

def classify(entry, by_id, promote_pending):
    """promote_pending = set of ids slated for promotion in this fixpoint round."""
    deps = entry['depends_on']
    has_verify_script = bool(entry['verified_by'])
    has_breakthrough = entry['has_breakthrough']
    has_eq = entry['has_eq']
    has_converge = entry['has_converge']
    has_apply = entry['has_apply']
    has_expr = bool(entry['expr'])

    dep_doc_refs = []
    dep_missing = []
    dep_atlas = []
    for d in deps:
        if d in by_id:
            dep_atlas.append(d)
        elif looks_like_doc_ref(d):
            dep_doc_refs.append(d)
        else:
            dep_missing.append(d)

    def dep_ok(d):
        g = by_id[d]['grade']
        return is_verified(g) or d in promote_pending
    n_atlas_ok = sum(1 for d in dep_atlas if dep_ok(d))
    all_atlas_ok = (n_atlas_ok == len(dep_atlas))
    has_continuation = has_eq or has_converge or has_apply or has_breakthrough or has_verify_script

    evidence = {
        'id': entry['id'],
        'line': entry['line'],
        'type': entry['type'],
        'domain': entry['domain'],
        'expr': entry['expr'][:80] if has_expr else '',
        'n_deps_total': len(deps),
        'n_atlas_deps': len(dep_atlas),
        'n_atlas_deps_ok': n_atlas_ok,
        'n_doc_refs': len(dep_doc_refs),
        'n_missing': len(dep_missing),
        'has_verify_script': has_verify_script,
        'has_eq': has_eq,
        'has_apply': has_apply,
        'has_converge': has_converge,
        'has_breakthrough': has_breakthrough,
    }

    # HOLD reasons (any one disqualifies)
    if dep_doc_refs:
        return 'HOLD', f'H1 cross-doc dep ({len(dep_doc_refs)})', evidence
    if dep_missing:
        return 'HOLD', f'H1b unknown dep ({dep_missing[:1]})', evidence
    if not all_atlas_ok:
        return 'HOLD', 'H2 unverified atlas dep', evidence

    # PROMOTE tiers
    if has_verify_script:
        return 'PROMOTE', 'T1 |> verify script', evidence
    if has_breakthrough:
        return 'PROMOTE', 'T4 !! breakthrough', evidence
    if has_continuation and has_expr:
        return 'PROMOTE', 'T2 expr + continuation (==/~>/=>)', evidence
    if not has_continuation and has_expr and len(dep_atlas) > 0 and entry['type'] in DEFINITIONAL_TYPES and expr_is_numeric_derivation(entry['expr']):
        return 'PROMOTE', 'T5 definitional header-expr + verified deps', evidence
    if not has_continuation and has_expr and len(dep_atlas) == 0 and entry['type'] in DEFINITIONAL_TYPES and expr_is_numeric_derivation(entry['expr']):
        # @C/@R/@F/@L axiom-like with explicit numeric value, no deps — still hold (axiom needs external)
        return 'HOLD', 'H4c definitional axiom (no deps, manual review)', evidence

    if len(deps) == 0 and not has_expr and not has_continuation:
        return 'HOLD', 'H3 pure axiom', evidence
    if len(deps) == 0 and has_expr and entry['type'] in ('@P', '@X', '@?'):
        return 'HOLD', 'H4 P/X/? primitive claim (manual)', evidence
    if len(deps) == 0 and has_expr:
        return 'HOLD', 'H4b expr only, no continuation', evidence
    if len(deps) > 0 and not has_continuation and not has_expr:
        return 'HOLD', 'H5 deps verified, no expr/continuation', evidence
    return 'HOLD', 'H? unclassified', evidence


def main():
    print(f"# Loading {ATLAS} ...", file=sys.stderr)
    entries, by_id = parse_atlas(ATLAS)
    print(f"# Total entries parsed: {len(entries)}", file=sys.stderr)

    grade10 = [e for e in entries if e['grade'] == '[10]']
    print(f"# [10] candidates: {len(grade10)}", file=sys.stderr)

    # Fixpoint
    promote_pending = set()
    promote_evidence = {}
    rounds = 0
    while True:
        rounds += 1
        added = 0
        for e in grade10:
            if e['id'] in promote_pending:
                continue
            verdict, reason, ev = classify(e, by_id, promote_pending)
            if verdict == 'PROMOTE':
                promote_pending.add(e['id'])
                ev['reason'] = reason
                ev['round'] = rounds
                promote_evidence[e['id']] = ev
                added += 1
        print(f"# Round {rounds}: +{added} promotions, total={len(promote_pending)}", file=sys.stderr)
        if added == 0:
            break

    # Final hold list
    hold_list = []
    for e in grade10:
        if e['id'] in promote_pending:
            continue
        verdict, reason, ev = classify(e, by_id, promote_pending)
        ev['reason'] = reason
        hold_list.append(ev)

    promote_list = list(promote_evidence.values())

    out = {
        'baseline': '2026-04-25',
        'analyzer_version': 'v3-fixpoint',
        'analyzer_path': '/tmp/atlas_analysis/promote_v3.py',
        'atlas_path': str(ATLAS),
        'fixpoint_rounds': rounds,
        'total_entries': len(entries),
        'grade10_total': len(grade10),
        'promote_eligible': len(promote_list),
        'hold_count': len(hold_list),
        'promote_by_reason': {},
        'promote_by_round': {},
        'hold_by_reason': {},
        'promote_entries': promote_list,
        'hold_entries_by_reason': {},
    }
    for p in promote_list:
        out['promote_by_reason'][p['reason']] = out['promote_by_reason'].get(p['reason'], 0) + 1
        rk = f"round_{p['round']}"
        out['promote_by_round'][rk] = out['promote_by_round'].get(rk, 0) + 1
    for h in hold_list:
        out['hold_by_reason'][h['reason']] = out['hold_by_reason'].get(h['reason'], 0) + 1
        bucket = h['reason'].split(' ')[0]
        out['hold_entries_by_reason'].setdefault(bucket, []).append(h)
    capped = {}
    for k, v in out['hold_entries_by_reason'].items():
        capped[k] = {'count': len(v), 'sample': v[:30]}
    out['hold_entries_by_reason'] = capped

    print(json.dumps(out, indent=2, ensure_ascii=False))


if __name__ == '__main__':
    main()
