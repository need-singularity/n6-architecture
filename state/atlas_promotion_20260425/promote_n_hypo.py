#!/usr/bin/env python3
"""atlas.n6 [N?] hypothesis -> [N!] promotion analyzer (dry-run).

[N?] = CONJECTURE, [N!] = breakthrough (conjecture-class, unreviewed).
Promotion criterion: needs explicit |> verify_by script OR !!
breakthrough marker. Without those, hypothesis stays as conjecture.
"""
import re
import json
import sys
from pathlib import Path

ATLAS = Path.home() / "core/n6-architecture/atlas/atlas.n6"

HEADER_RE = re.compile(r'^(@[A-Z?])\s+(\S+?)(?:\s*=\s*(.+?))?\s*::\s*(\S+)\s+(\[[^\]]+\])\s*$')
VERIFIED_GRADES = {'[10*]', '[10**]', '[11*]', '[12*]', '[10*!]', '[5*]', '[9*]', '[8*]', '[6*]', '[7*]', '[0.10*]', '[0.9*]'}


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
                        'line': line_no, 'type': t, 'id': eid,
                        'expr': (expr or '').strip(),
                        'domain': domain, 'grade': grade,
                        'depends_on': [], 'verified_by': [],
                        'has_breakthrough': False, 'has_eq': False,
                        'has_converge': False, 'has_apply': False,
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
                    for d in re.split(r'[,\s]+', stripped[2:].strip()):
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


def main():
    entries, by_id = parse_atlas(ATLAS)
    nhypo = [e for e in entries if e['grade'] == '[N?]']
    print(f"# [N?] count: {len(nhypo)}", file=sys.stderr)

    promote = []
    hold = []
    for e in nhypo:
        if e['verified_by'] or e['has_breakthrough']:
            promote.append({
                'id': e['id'], 'line': e['line'], 'type': e['type'],
                'domain': e['domain'],
                'has_verify_script': bool(e['verified_by']),
                'has_breakthrough': e['has_breakthrough'],
                'verified_by': e['verified_by'][:1],
            })
        else:
            hold.append({
                'id': e['id'], 'line': e['line'], 'type': e['type'],
                'domain': e['domain'],
                'has_eq': e['has_eq'], 'has_converge': e['has_converge'],
                'has_apply': e['has_apply'],
                'n_deps': len(e['depends_on']),
            })
    out = {
        'baseline': '2026-04-25',
        'task': 'N-hypothesis promotion [N?] -> [N!]',
        'total_n_question': len(nhypo),
        'promote_eligible': len(promote),
        'hold_count': len(hold),
        'promote_entries': promote,
        'hold_sample': hold[:30],
    }
    print(json.dumps(out, indent=2, ensure_ascii=False))


if __name__ == '__main__':
    main()
