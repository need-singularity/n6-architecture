#!/usr/bin/env python3
"""
atlas.signals.n6 수동·반자동 흡수 도구

사용법:
  # 1) CLI 인자 — 단발 추가
  python3 scripts/absorb_to_signals.py add \
      --repo NX --domain SR,OURO \
      --grade M7! --evidence E1 \
      --statement "Ouroboros σ=0.1 PEAK" \
      --source ~/.claude-claude1/.../p-stoch-resonance.md

  # 2) 대화형
  python3 scripts/absorb_to_signals.py interactive

  # 3) BT 파일에서 일괄 추출 (반자동)
  python3 scripts/absorb_to_signals.py from-bt \
      --file theory/breakthroughs/bt-1420-*.md \
      --repo N6 --auto-tag

  # 4) 리스트 + 필터
  python3 scripts/absorb_to_signals.py list --repo NX --grade M7,M7!
  python3 scripts/absorb_to_signals.py list --domain SR,OURO
  python3 scripts/absorb_to_signals.py list --cross-only

  # 5) 통계
  python3 scripts/absorb_to_signals.py stats
  python3 scripts/absorb_to_signals.py gaps  # 도메인별 gap
"""
from __future__ import annotations
import argparse
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
SIGNALS_DIR = Path("/Users/ghost/Dev/nexus/n6/signals")

VALID_REPOS = {"NX", "N6", "AN", "CROSS"}
VALID_GRADES = {"M10*", "M10", "M9", "M7!", "M7", "M?", "MN"}
VALID_EVIDENCE = {"E1", "E2", "E3", "EC", "EP", "EF"}
MILLENNIUM_TAGS = {"7R", "7N", "7H", "7P", "7Y", "7S", "7B"}


def now_iso() -> str:
    s = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    return re.sub(r"\.\d+Z$", "Z", s)


def load_signals() -> list[dict]:
    """현재 atlas.signals.n6 파싱"""
    if not SSOT.exists():
        return []
    text = SSOT.read_text(encoding="utf-8", errors="replace")
    signals = []
    lines = text.split("\n")
    i = 0
    while i < len(lines):
        m = re.match(
            r"@S\s+(\S+)\s*=\s*(.+?)\s*::\s*signal\s*\[([^\]]+)\]\s*\[([^\]]+)\]\s*\[([^\]]+)\]\s*\[([^\]]+)\]",
            lines[i],
        )
        if m:
            sig = {
                "sig_id": m.group(1),
                "statement": m.group(2),
                "repo_tags": [t.strip() for t in m.group(3).split(",")],
                "domain_tags": [t.strip() for t in m.group(4).split(",")],
                "grade": m.group(5).strip(),
                "evidence": m.group(6).strip(),
                "witness": 1,
                "line_start": i,
            }
            # 다음 빈 줄 또는 다음 @S 까지가 이 signal 의 bulk
            j = i + 1
            while j < len(lines) and not lines[j].startswith("@S ") and not lines[j].startswith("# ─"):
                if "witness:" in lines[j]:
                    wm = re.search(r"witness:\s*(\d+)", lines[j])
                    if wm:
                        sig["witness"] = int(wm.group(1))
                j += 1
            sig["line_end"] = j
            signals.append(sig)
            i = j
        else:
            i += 1
    return signals


def next_sig_id(signals: list[dict], domain: str) -> str:
    existing = [s["sig_id"] for s in signals]
    prefix = f"SIG-{domain}-"
    used = [int(m.group(1)) for s in existing if (m := re.match(rf"{re.escape(prefix)}(\d+)", s))]
    nxt = max(used, default=0) + 1
    return f"{prefix}{nxt:03d}"


def validate(tags_repo: list[str], tags_domain: list[str], grade: str, evidence: str):
    for t in tags_repo:
        if t not in VALID_REPOS:
            raise ValueError(f"Invalid repo tag: {t}. Valid: {VALID_REPOS}")
    if grade not in VALID_GRADES:
        raise ValueError(f"Invalid grade: {grade}. Valid: {VALID_GRADES}")
    if evidence not in VALID_EVIDENCE:
        raise ValueError(f"Invalid evidence: {evidence}. Valid: {VALID_EVIDENCE}")
    if "CROSS" in tags_repo and len(tags_repo) < 2:
        raise ValueError("CROSS repo tag requires at least one other repo tag (NX/N6/AN)")


def format_signal(
    sig_id: str,
    statement: str,
    repo_tags: list[str],
    domain_tags: list[str],
    grade: str,
    evidence: str,
    context: str = "",
    refs: list[str] | None = None,
    cross: list[str] | None = None,
    cross_repo: list[str] | None = None,
    predicts: list[str] | None = None,
    witness: int = 1,
    resonance_n6: str | None = None,
    null_reason: str | None = None,
    retry_forbidden_until: str | None = None,
    discovered_in: str = "manual",
    discovered_at: str | None = None,
    source_file: str = "",
) -> str:
    lines = [
        f"@S {sig_id} = {statement} :: signal [{','.join(repo_tags)}] [{','.join(domain_tags)}] [{grade}] [{evidence}]"
    ]
    if context:
        lines.append(f'  "{context}"')
    if refs:
        lines.append(f"  refs: [{', '.join(refs)}]")
    if cross:
        lines.append(f"  cross: [{', '.join(cross)}]")
    if cross_repo:
        lines.append(f"  cross_repo: [{', '.join(cross_repo)}]")
    if predicts:
        preds_fmt = ', '.join(f'"{p}"' for p in predicts)
        lines.append(f"  predicts: [{preds_fmt}]")
    lines.append(f"  witness: {witness}")
    if resonance_n6:
        lines.append(f'  resonance_n6: "{resonance_n6}"')
    if null_reason:
        lines.append(f'  null_reason: "{null_reason}"')
    if retry_forbidden_until:
        lines.append(f'  retry_forbidden_until: "{retry_forbidden_until}"')
    lines.append(f"  discovered_in: {discovered_in}")
    lines.append(f"  discovered_at: {discovered_at or now_iso()}")
    if source_file:
        lines.append(f"  <- {source_file}")
    return "\n".join(lines) + "\n"


def cmd_add(args):
    signals = load_signals()
    repo_tags = [t.strip() for t in args.repo.split(",")]
    domain_tags = [t.strip() for t in args.domain.split(",")]
    validate(repo_tags, domain_tags, args.grade, args.evidence)

    primary_domain = domain_tags[0].upper()
    sig_id = args.sig_id or next_sig_id(signals, primary_domain)

    entry = format_signal(
        sig_id=sig_id,
        statement=args.statement,
        repo_tags=repo_tags,
        domain_tags=domain_tags,
        grade=args.grade,
        evidence=args.evidence,
        context=args.context or "",
        refs=args.refs.split(",") if args.refs else None,
        cross_repo=args.cross_repo.split(",") if args.cross_repo else None,
        witness=args.witness,
        resonance_n6=args.resonance_n6,
        null_reason=args.null_reason,
        retry_forbidden_until=args.retry_forbidden_until,
        discovered_in=args.discovered_in or "manual",
        source_file=args.source or "",
    )
    with SSOT.open("a", encoding="utf-8") as f:
        f.write("\n" + entry)
    print(f"✓ 추가: {sig_id}")
    print(f"  위치: {SSOT}")


def cmd_interactive(args):
    print("=== atlas.signals.n6 대화형 추가 ===")
    signals = load_signals()
    repo = input("repo tags (예: NX or CROSS,NX,AN): ").strip()
    domain = input("domain tags (예: SR,OURO): ").strip()
    statement = input("statement: ").strip()
    grade = input(f"grade {sorted(VALID_GRADES)}: ").strip()
    evidence = input(f"evidence {sorted(VALID_EVIDENCE)}: ").strip()
    context = input("context (선택): ").strip()
    source = input("source_file (필수): ").strip()
    refs = input("refs (,로 구분, 선택): ").strip()
    resonance = input("resonance_n6 (선택): ").strip()

    repo_tags = [t.strip() for t in repo.split(",")]
    domain_tags = [t.strip() for t in domain.split(",")]
    validate(repo_tags, domain_tags, grade, evidence)

    sig_id = next_sig_id(signals, domain_tags[0].upper())
    entry = format_signal(
        sig_id=sig_id,
        statement=statement,
        repo_tags=repo_tags,
        domain_tags=domain_tags,
        grade=grade,
        evidence=evidence,
        context=context,
        refs=refs.split(",") if refs else None,
        resonance_n6=resonance or None,
        source_file=source,
    )
    print("\n=== 미리보기 ===")
    print(entry)
    confirm = input("저장? [y/N]: ").strip().lower()
    if confirm == "y":
        with SSOT.open("a", encoding="utf-8") as f:
            f.write("\n" + entry)
        print(f"✓ 저장됨: {sig_id}")
    else:
        print("✗ 취소")


def cmd_list(args):
    signals = load_signals()
    filtered = signals
    if args.repo:
        tags = set(t.strip().upper() for t in args.repo.split(","))
        filtered = [s for s in filtered if tags & set(s["repo_tags"])]
    if args.domain:
        tags = set(t.strip().upper() for t in args.domain.split(","))
        filtered = [s for s in filtered if tags & set(s["domain_tags"])]
    if args.grade:
        grades = set(g.strip() for g in args.grade.split(","))
        filtered = [s for s in filtered if s["grade"] in grades]
    if args.cross_only:
        filtered = [s for s in filtered if "CROSS" in s["repo_tags"]]

    for s in filtered:
        repo_str = ",".join(s["repo_tags"])
        dom_str = ",".join(s["domain_tags"])
        print(f"  {s['sig_id']:20}  [{repo_str:12}] [{dom_str:20}] [{s['grade']:5}] w={s['witness']}  {s['statement'][:60]}")
    print(f"\n총 {len(filtered)} / 전체 {len(signals)}")


def cmd_stats(args):
    signals = load_signals()
    print(f"총 signals: {len(signals)}")
    print()
    print("리포별:")
    from collections import Counter
    repo_counts = Counter()
    for s in signals:
        for t in s["repo_tags"]:
            repo_counts[t] += 1
    for t, c in sorted(repo_counts.items(), key=lambda x: -x[1]):
        bar = "█" * c
        print(f"  [{t:6}] {c:3}  {bar}")
    print()
    print("도메인별:")
    dom_counts = Counter()
    for s in signals:
        for t in s["domain_tags"]:
            dom_counts[t] += 1
    for t, c in sorted(dom_counts.items(), key=lambda x: -x[1])[:20]:
        bar = "█" * c
        print(f"  [{t:6}] {c:3}  {bar}")
    print()
    print("등급별:")
    grade_counts = Counter(s["grade"] for s in signals)
    for g in ["M10*", "M10", "M9", "M7!", "M7", "M?", "MN"]:
        c = grade_counts.get(g, 0)
        bar = "█" * c
        print(f"  [{g:4}] {c:3}  {bar}")
    print()
    print("CROSS-repo resonance:")
    cross_sigs = [s for s in signals if "CROSS" in s["repo_tags"]]
    print(f"  {len(cross_sigs)} 건")
    for s in cross_sigs:
        print(f"    {s['sig_id']}  [{','.join(s['repo_tags'])}]  {s['statement'][:60]}")


def cmd_gaps(args):
    """도메인별 gap detection"""
    signals = load_signals()
    from collections import Counter
    dom_counts = Counter()
    for s in signals:
        for t in s["domain_tags"]:
            dom_counts[t] += 1

    print("=== 도메인별 signal 분포 (gap detection) ===\n")
    # 밀레니엄 난제
    print("밀레니엄 난제:")
    mill_tags = ["7R", "7N", "7H", "7P", "7Y", "7B"]
    max_mill = max((dom_counts.get(t, 0) for t in mill_tags), default=1)
    for t in mill_tags:
        c = dom_counts.get(t, 0)
        width = int(30 * c / max(max_mill, 1))
        bar = "█" * width if width else ""
        marker = " ←── SPARSE" if c < max_mill * 0.3 else ""
        print(f"  [{t:4}] {c:3}  {bar}{marker}")

    print("\nphenomenon/구조:")
    phen_tags = ["SR", "QRNG", "OURO", "64T", "CONS", "NEURAL", "PHYS", "HEXA", "BLOW", "ATLAS", "DFS"]
    max_phen = max((dom_counts.get(t, 0) for t in phen_tags), default=1)
    for t in phen_tags:
        c = dom_counts.get(t, 0)
        width = int(30 * c / max(max_phen, 1))
        bar = "█" * width if width else ""
        marker = " ←── SPARSE" if c < max_phen * 0.3 else ""
        print(f"  [{t:6}] {c:3}  {bar}{marker}")

    print("\n메타:")
    meta_tags = ["META", "UNIV", "GAP", "REPLAY", "NULL"]
    for t in meta_tags:
        c = dom_counts.get(t, 0)
        print(f"  [{t:6}] {c:3}")


def cmd_from_bt(args):
    """BT 파일에서 signal 일괄 추출"""
    path = Path(args.file)
    if not path.exists():
        print(f"ERR: {path} 없음")
        sys.exit(1)
    text = path.read_text(encoding="utf-8")
    signals = load_signals()

    m_fm = re.match(r"---\n(.*?)\n---\n", text, re.DOTALL)
    fm = {}
    if m_fm:
        for line in m_fm.group(1).split("\n"):
            if ":" in line:
                k, _, v = line.partition(":")
                fm[k.strip()] = v.strip().strip('"')

    body = text[m_fm.end():] if m_fm else text

    rows = re.findall(r"^\|\s*(\d+-\d+)\s*\|\s*(.+?)\s*\|", body, re.MULTILINE)
    print(f"=== {path.name} 에서 {len(rows)} 개 tight row 발견 ===\n")

    added = 0
    for tight_id, tight_stmt in rows:
        dfs_round = fm.get("dfs_round", "??")
        domain = "7R" if "ζ" in tight_stmt or "Riemann" in tight_stmt else \
                 "7P" if "Ramsey" in tight_stmt or "NP" in tight_stmt else \
                 "7H" if "kissing" in tight_stmt or "Hodge" in tight_stmt else \
                 "DFS"
        sig_id = next_sig_id(signals, domain)
        entry = format_signal(
            sig_id=sig_id,
            statement=f"DFS{dfs_round} [{tight_id}] {tight_stmt[:100]}",
            repo_tags=[args.repo or "N6"],
            domain_tags=[domain, "DFS"],
            grade="M7",
            evidence="E1",
            context=f"BT frontmatter: {fm.get('grade', '')} {fm.get('parent_bt', '')}",
            refs=[f"{path.relative_to(Path('/Users/ghost/Dev/n6-architecture'))}:{tight_id}"],
            discovered_in=f"n6/DFS-{dfs_round}",
            source_file=str(path.relative_to(Path('/Users/ghost/Dev/n6-architecture'))),
        )
        if args.dry_run:
            print(entry)
        else:
            with SSOT.open("a", encoding="utf-8") as f:
                f.write("\n" + entry)
            signals.append({"sig_id": sig_id, "domain_tags": [domain]})
        added += 1
    print(f"\n{'DRY RUN — ' if args.dry_run else ''}{added} signal 추출")


def main():
    p = argparse.ArgumentParser(prog="absorb_to_signals")
    sub = p.add_subparsers(dest="cmd", required=True)

    p_add = sub.add_parser("add")
    p_add.add_argument("--repo", required=True, help="NX,N6,AN,CROSS")
    p_add.add_argument("--domain", required=True, help="SR,QRNG,7R,…")
    p_add.add_argument("--grade", required=True)
    p_add.add_argument("--evidence", required=True)
    p_add.add_argument("--statement", required=True)
    p_add.add_argument("--source", required=True)
    p_add.add_argument("--context")
    p_add.add_argument("--refs")
    p_add.add_argument("--cross-repo", dest="cross_repo")
    p_add.add_argument("--witness", type=int, default=1)
    p_add.add_argument("--resonance-n6", dest="resonance_n6")
    p_add.add_argument("--null-reason", dest="null_reason")
    p_add.add_argument("--retry-forbidden-until", dest="retry_forbidden_until")
    p_add.add_argument("--discovered-in", dest="discovered_in")
    p_add.add_argument("--sig-id", dest="sig_id")
    p_add.set_defaults(func=cmd_add)

    p_int = sub.add_parser("interactive")
    p_int.set_defaults(func=cmd_interactive)

    p_list = sub.add_parser("list")
    p_list.add_argument("--repo")
    p_list.add_argument("--domain")
    p_list.add_argument("--grade")
    p_list.add_argument("--cross-only", action="store_true", dest="cross_only")
    p_list.set_defaults(func=cmd_list)

    p_stats = sub.add_parser("stats")
    p_stats.set_defaults(func=cmd_stats)

    p_gaps = sub.add_parser("gaps")
    p_gaps.set_defaults(func=cmd_gaps)

    p_bt = sub.add_parser("from-bt")
    p_bt.add_argument("--file", required=True)
    p_bt.add_argument("--repo")
    p_bt.add_argument("--dry-run", action="store_true", dest="dry_run")
    p_bt.set_defaults(func=cmd_from_bt)

    args = p.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
