#!/usr/bin/env python3
"""
fusion_auto_append.py — A1 + A2

fusion_log.jsonl 을 파싱해서 같은 (a, b, c) 조합이 2회 이상 등장하면
atlas.signals.n6 에 [M?] 또는 [M7!] 승격 entry 를 append 한다.

규칙 (spec atlas.signals.n6.spec.md v0.2):
  - 새 sig_id: SIG-FUSION-NNN (domain: FUSION)
  - statement: "fusion of a+b+c (witness=N)"
  - 첫 발견 (witness=1): 등록 안함 (너무 약함)
  - 2회+ (witness>=2): [M?]
  - 3회+ (witness>=3): [M7!]
  - 이미 동일 조합 entry 있으면 witness 증분 + 승격만

사용법:
  /usr/bin/python3 scripts/fusion_auto_append.py --dry-run
  /usr/bin/python3 scripts/fusion_auto_append.py --commit
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
FUSION_LOG = Path("/Users/ghost/Dev/nexus/n6/signals/fusion_log.jsonl")
BACKUP = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6.bak.pre-fusion")


def now_iso() -> str:
    s = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    return re.sub(r"\.\d+Z$", "Z", s)


def load_fusion_log() -> list[dict]:
    if not FUSION_LOG.exists():
        print(f"ERR: fusion_log.jsonl 없음: {FUSION_LOG}", file=sys.stderr)
        sys.exit(1)
    rows = []
    for ln in FUSION_LOG.read_text(encoding="utf-8", errors="replace").splitlines():
        ln = ln.strip()
        if not ln:
            continue
        try:
            rows.append(json.loads(ln))
        except json.JSONDecodeError:
            continue
    return rows


def normalize_triple(a: str, b: str, c: str) -> tuple[str, str, str]:
    """공백·plcaceholder 제거 후 정렬 (순서 무관)"""
    cleaned = []
    for t in (a, b, c):
        t = (t or "").strip()
        if not t or t in {"───", "—", "-", ""}:
            continue
        cleaned.append(t)
    cleaned.sort()
    while len(cleaned) < 3:
        cleaned.append("")
    return tuple(cleaned[:3])  # type: ignore


def parse_existing_fusion_sigs(text: str) -> dict[tuple[str, str, str], dict]:
    """기존 SIG-FUSION-NNN 엔트리 파싱 (triple → {sig_id, witness, line_start, line_end})"""
    result: dict[tuple[str, str, str], dict] = {}
    lines = text.split("\n")
    i = 0
    n_lines = len(lines)
    while i < n_lines:
        m = re.match(r"^@S\s+(SIG-FUSION-\d+)\s*=\s*fusion of (.+?)\s*\(witness=\d+\)", lines[i])
        if m:
            sig_id = m.group(1)
            members = m.group(2).strip()
            parts = [p.strip() for p in members.split("+") if p.strip()]
            triple = normalize_triple(*(parts + ["", "", ""])[:3])
            j = i + 1
            witness = 1
            while j < n_lines and not lines[j].startswith("@S ") and not lines[j].startswith("# ─"):
                wm = re.search(r"^\s*witness:\s*(\d+)", lines[j])
                if wm:
                    witness = int(wm.group(1))
                j += 1
            result[triple] = {
                "sig_id": sig_id,
                "witness": witness,
                "line_start": i,
                "line_end": j,
            }
            i = j
        else:
            i += 1
    return result


def next_fusion_id(text: str) -> int:
    nums = [int(m.group(1)) for m in re.finditer(r"^@S\s+SIG-FUSION-(\d+)", text, re.MULTILINE)]
    return (max(nums) + 1) if nums else 1


def grade_for(witness: int) -> str:
    if witness >= 3:
        return "M7!"
    if witness >= 2:
        return "M?"
    return "M?"  # 호출부에서 이 경우는 skip 처리


def format_new_entry(sig_id: str, triple: tuple[str, str, str], witness: int, sources: list[str]) -> str:
    members = [t for t in triple if t]
    statement = "fusion of " + "+".join(members) + f" (witness={witness})"
    grade = grade_for(witness)
    src_note = ", ".join(sources[:3]) if sources else "fusion_log.jsonl"
    return (
        f"@S {sig_id} = {statement} :: signal [CROSS,NX] [META,UNIV] [{grade}] [E2]\n"
        f'  "fusion_log.jsonl 에서 {witness}회 재등장한 signal 조합. 메타-공명 자동 탐지."\n'
        f"  refs: [fusion_log.jsonl, {src_note}]\n"
        f"  cross_repo: [{', '.join(members)}]\n"
        f"  witness: {witness}\n"
        f"  resonance_n6: null\n"
        f"  discovered_in: nexus/fusion_auto_append\n"
        f"  discovered_at: {now_iso()}\n"
        f"  <- /Users/ghost/Dev/nexus/n6/signals/fusion_log.jsonl\n"
    )


def update_witness_in_lines(lines: list[str], entry: dict, new_witness: int) -> None:
    start = entry["line_start"]
    end = entry["line_end"]
    new_grade = grade_for(new_witness)
    head = lines[start]
    head = re.sub(r"\(witness=\d+\)", f"(witness={new_witness})", head)
    head = re.sub(r"\[M\?\]|\[M7!\]", f"[{new_grade}]", head, count=1)
    lines[start] = head
    for k in range(start + 1, end):
        if re.match(r"^\s*witness:\s*\d+", lines[k]):
            lines[k] = re.sub(r"witness:\s*\d+", f"witness: {new_witness}", lines[k])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--commit", action="store_true", help="실제 쓰기 (기본 dry-run)")
    args = ap.parse_args()
    dry_run = not args.commit

    rows = load_fusion_log()
    print(f"fusion_log 엔트리: {len(rows)}")

    triples: Counter = Counter()
    sources: dict[tuple[str, str, str], list[str]] = {}
    for r in rows:
        a = r.get("a", "")
        b = r.get("b", "")
        c = r.get("c", "")
        mode = r.get("mode", "")
        fusion_id = r.get("fusion_id", "")
        triple = normalize_triple(a, b, c)
        triples[triple] += 1
        sources.setdefault(triple, []).append(f"{mode}:{fusion_id}")

    print(f"고유 조합: {len(triples)}  (2회+ = {sum(1 for v in triples.values() if v >= 2)})")

    if not SSOT.exists():
        print(f"ERR: SSOT 없음: {SSOT}", file=sys.stderr)
        sys.exit(1)

    original = SSOT.read_text(encoding="utf-8", errors="replace")
    existing = parse_existing_fusion_sigs(original)

    lines = original.split("\n")
    new_appends: list[str] = []
    updated: list[tuple[str, int, int]] = []  # (sig_id, old_w, new_w)
    appended_ids: list[str] = []

    next_n = next_fusion_id(original)
    for triple, cnt in sorted(triples.items(), key=lambda x: -x[1]):
        if cnt < 2:
            continue  # 1회 는 약한 신호, skip
        if triple in existing:
            old_w = existing[triple]["witness"]
            new_w = max(old_w, cnt)
            if new_w > old_w:
                update_witness_in_lines(lines, existing[triple], new_w)
                updated.append((existing[triple]["sig_id"], old_w, new_w))
        else:
            sig_id = f"SIG-FUSION-{next_n:03d}"
            next_n += 1
            entry = format_new_entry(sig_id, triple, cnt, sources.get(triple, []))
            new_appends.append(entry)
            appended_ids.append(sig_id)

    print(f"\n신규 append 대상: {len(new_appends)}")
    for sid in appended_ids:
        print(f"  + {sid}")
    print(f"witness 증분 대상: {len(updated)}")
    for sid, ow, nw in updated:
        print(f"  ~ {sid}: {ow} -> {nw}")

    if dry_run:
        print("\n[DRY RUN] --commit 지정 시 실제 반영")
        return

    if not updated and not new_appends:
        print("변경 없음. 종료.")
        return

    if not BACKUP.exists():
        BACKUP.write_bytes(SSOT.read_bytes())
        print(f"\n백업: {BACKUP}")

    # in-place update 먼저 반영
    updated_text = "\n".join(lines)

    # append 블록
    if new_appends:
        updated_text = updated_text.rstrip("\n") + "\n\n"
        updated_text += f"# ─── [FUSION] fusion_auto_append ({now_iso()[:10]}) ───\n\n"
        updated_text += "\n".join(new_appends)

    SSOT.write_text(updated_text, encoding="utf-8")
    print(f"\n반영 완료: {SSOT}")
    print(f"  신규: {len(new_appends)}  증분: {len(updated)}")


if __name__ == "__main__":
    main()
