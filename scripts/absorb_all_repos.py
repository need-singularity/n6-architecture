#!/usr/bin/env python3
"""
3-리포 통합 흡수: n6-architecture + nexus + anima
→ nexus/shared/discovery_log.jsonl (SSOT) + n6shared 미러

흡수 범위:
  n6-architecture/theory/breakthroughs/*.md       (bt_absorption)
  nexus/shared/n6/atlas.n6 MILL-DFS2x             (atlas_mill_absorption)
  nexus/shared/docs/** *.md                       (nexus_docs_absorption)
  anima/anima-*/docs/*.md                         (anima_docs_absorption)
  anima/data/laws_from_atlas.jsonl                (anima_law_absorption)
  anima/*/checkpoints/summary.json                (anima_checkpoint_absorption)

고장 시점: 2026-04-14T03:40:18Z
"""
from __future__ import annotations
import json
import re
import hashlib
from datetime import datetime, timezone
from pathlib import Path

N6 = Path("/Users/ghost/Dev/n6-architecture")
NEXUS = Path("/Users/ghost/Dev/nexus")
ANIMA = Path("/Users/ghost/Dev/anima")

SINK_NEXUS = NEXUS / "shared" / "discovery_log.jsonl"
SINK_N6 = N6 / "n6shared" / "discovery_log.jsonl"

NOW = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
NOW = re.sub(r"\.\d+Z$", "Z", NOW)


def parse_frontmatter(text: str) -> dict:
    m = re.match(r"---\n(.*?)\n---\n(.*)", text, re.DOTALL)
    if not m:
        return {}
    fm_block = m.group(1)
    body = m.group(2)
    fm = {}
    for line in fm_block.split("\n"):
        if ":" not in line:
            continue
        k, _, v = line.partition(":")
        fm[k.strip()] = v.strip().strip('"').strip("'")
    fm["_body"] = body
    return fm


def file_ts(path: Path) -> str:
    ts = datetime.fromtimestamp(path.stat().st_mtime, tz=timezone.utc).isoformat().replace("+00:00", "Z")
    return re.sub(r"\.\d+Z$", "Z", ts)


def load_existing_ids() -> dict[str, set[str]]:
    result = {"bt_absorption": set(), "atlas_mill_dfs_absorption": set(),
              "nexus_docs_absorption": set(), "anima_docs_absorption": set(),
              "anima_law_absorption": set(), "anima_checkpoint_absorption": set()}
    if not SINK_NEXUS.exists():
        return result
    with SINK_NEXUS.open("r", encoding="utf-8") as f:
        for line in f:
            if not line.strip():
                continue
            try:
                e = json.loads(line)
            except Exception:
                continue
            kind = e.get("kind", "")
            if kind in result:
                key = e.get("bt_id") or e.get("atlas_id") or e.get("source_file") or e.get("law_id") or e.get("checkpoint_path")
                if key:
                    result[kind].add(key)
    return result


def absorb_n6_bt(existing: set[str]) -> list[dict]:
    entries = []
    for p in sorted((N6 / "theory" / "breakthroughs").glob("*.md")):
        try:
            text = p.read_text(encoding="utf-8")
        except Exception:
            continue
        fm = parse_frontmatter(text)
        if fm:
            bt_id = fm.get("id", p.stem)
            grade = fm.get("grade", "")
            parent = fm.get("parent_bt", "")
            new_tight = fm.get("new_tight", "")
            cum_tight = fm.get("cumulative_tight", "")
            solved = fm.get("solved", "")
            dfs_round = fm.get("dfs_round", "")
            body = fm.get("_body", "")
        else:
            stem = p.stem
            m = re.match(r"(bt-\d+[\w-]*?)(?:-\d{4}-\d{2}-\d{2})?$", stem)
            bt_id = m.group(1) if m else stem
            h = re.search(r"^#\s+(.+)$", text, re.MULTILINE)
            grade = "[legacy no-fm]"
            parent = ""
            new_tight = cum_tight = solved = dfs_round = ""
            body = text
        if bt_id in existing:
            continue
        pass_m = re.search(r"(\d+)\s*PASS\s*/\s*(\d+)\s*FAIL", body)
        hp = int(pass_m.group(1)) if pass_m else 0
        hf = int(pass_m.group(2)) if pass_m else 0
        entries.append({
            "ts": file_ts(p),
            "kind": "bt_absorption",
            "bt_id": bt_id,
            "source_file": str(p.relative_to(N6)),
            "source_repo": "n6-architecture",
            "parent_bt": parent,
            "grade": grade,
            "dfs_round": dfs_round,
            "new_tight": new_tight,
            "cumulative_tight": cum_tight,
            "solved": solved,
            "harness_pass": hp,
            "harness_fail": hf,
            "simhash": hashlib.md5(bt_id.encode()).hexdigest()[:32],
            "absorbed_at": NOW,
            "absorbed_reason": "backfill_multirepo_20260415",
        })
    return entries


def absorb_nexus_atlas_mill(existing: set[str]) -> list[dict]:
    atlas = NEXUS / "shared" / "n6" / "atlas.n6"
    if not atlas.exists():
        return []
    text = atlas.read_text(encoding="utf-8", errors="replace")
    entries = []
    for m in re.finditer(r"^@R\s+(MILL-DFS\d+[\w-]+)\s*=\s*(.+?)\s*::\s*n6atlas\s*\[([^\]]+)\]", text, re.MULTILINE):
        aid = m.group(1)
        if aid in existing:
            continue
        entries.append({
            "ts": NOW,
            "kind": "atlas_mill_dfs_absorption",
            "atlas_id": aid,
            "source_repo": "nexus",
            "source_file": "shared/n6/atlas.n6",
            "statement": m.group(2).strip(),
            "grade": m.group(3),
            "absorbed_at": NOW,
            "absorbed_reason": "backfill_multirepo_20260415",
        })
    return entries


def absorb_nexus_docs(existing: set[str]) -> list[dict]:
    entries = []
    doc_dirs = [NEXUS / "shared" / "docs", NEXUS / "shared" / "papers", NEXUS / "shared" / "roadmaps"]
    cutoff = datetime.fromisoformat("2026-04-08T00:00:00+00:00").timestamp()
    for d in doc_dirs:
        if not d.exists():
            continue
        for p in d.rglob("*.md"):
            if ".claude" in str(p) or "node_modules" in str(p):
                continue
            try:
                if p.stat().st_mtime < cutoff:
                    continue
            except Exception:
                continue
            rel = str(p.relative_to(NEXUS))
            if rel in existing:
                continue
            try:
                text = p.read_text(encoding="utf-8", errors="replace")
            except Exception:
                continue
            h = re.search(r"^#\s+(.+)$", text, re.MULTILINE)
            title = h.group(1).strip()[:120] if h else p.stem
            entries.append({
                "ts": file_ts(p),
                "kind": "nexus_docs_absorption",
                "source_repo": "nexus",
                "source_file": rel,
                "title": title,
                "size_bytes": p.stat().st_size,
                "simhash": hashlib.md5(rel.encode()).hexdigest()[:32],
                "absorbed_at": NOW,
                "absorbed_reason": "backfill_multirepo_20260415",
            })
    return entries


def absorb_anima_docs(existing: set[str]) -> list[dict]:
    entries = []
    doc_dirs = [
        ANIMA / "anima-physics" / "docs",
        ANIMA / "anima-agent" / "docs",
        ANIMA / "anima-core" / "docs",
        ANIMA / "anima-eeg" / "docs",
        ANIMA / "docs",
    ]
    cutoff = datetime.fromisoformat("2026-04-08T00:00:00+00:00").timestamp()
    for d in doc_dirs:
        if not d.exists():
            continue
        for p in d.rglob("*.md"):
            if ".claude" in str(p) or "node_modules" in str(p) or "ready/" in str(p):
                continue
            try:
                if p.stat().st_mtime < cutoff:
                    continue
            except Exception:
                continue
            rel = str(p.relative_to(ANIMA))
            if rel in existing:
                continue
            try:
                text = p.read_text(encoding="utf-8", errors="replace")
            except Exception:
                continue
            h = re.search(r"^#\s+(.+)$", text, re.MULTILINE)
            title = h.group(1).strip()[:120] if h else p.stem
            entries.append({
                "ts": file_ts(p),
                "kind": "anima_docs_absorption",
                "source_repo": "anima",
                "source_file": rel,
                "title": title,
                "size_bytes": p.stat().st_size,
                "simhash": hashlib.md5(rel.encode()).hexdigest()[:32],
                "absorbed_at": NOW,
                "absorbed_reason": "backfill_multirepo_20260415",
            })
    return entries


def absorb_anima_laws(existing: set[str]) -> list[dict]:
    path = ANIMA / "data" / "laws_from_atlas.jsonl"
    if not path.exists():
        return []
    entries = []
    for idx, line in enumerate(path.read_text(encoding="utf-8").split("\n")):
        if not line.strip():
            continue
        try:
            law = json.loads(line)
        except Exception:
            continue
        law_id = f"anima_law_{law.get('seed', 'unk')}_{idx}"
        if law_id in existing:
            continue
        entries.append({
            "ts": NOW,
            "kind": "anima_law_absorption",
            "law_id": law_id,
            "source_repo": "anima",
            "source_file": "data/laws_from_atlas.jsonl",
            "direction": law.get("direction", ""),
            "seed": law.get("seed", ""),
            "result": law.get("result", ""),
            "status": law.get("status", ""),
            "law_ts": law.get("ts", ""),
            "absorbed_at": NOW,
            "absorbed_reason": "backfill_multirepo_20260415",
        })
    return entries


def absorb_anima_checkpoints(existing: set[str]) -> list[dict]:
    entries = []
    main_cp = ANIMA / "ready" / "models" / "animalm" / "checkpoints" / "summary.json"
    if not main_cp.exists():
        main_cp = ANIMA / "anima-agent" / "checkpoints" / "summary.json"
    if not main_cp.exists():
        return []
    rel = str(main_cp.relative_to(ANIMA))
    if rel in existing:
        return []
    try:
        summary = json.loads(main_cp.read_text(encoding="utf-8"))
    except Exception:
        return []
    entries.append({
        "ts": file_ts(main_cp),
        "kind": "anima_checkpoint_absorption",
        "checkpoint_path": rel,
        "source_repo": "anima",
        "summary_keys": list(summary.keys())[:20] if isinstance(summary, dict) else [],
        "summary_preview": json.dumps(summary)[:400],
        "absorbed_at": NOW,
        "absorbed_reason": "backfill_multirepo_20260415",
    })
    return entries


def main():
    print("=" * 62)
    print(" 3-리포 통합 흡수 (n6-architecture + nexus + anima)")
    print("=" * 62)
    print()

    existing = load_existing_ids()
    for k, s in existing.items():
        print(f"  기존 {k}: {len(s)}")
    print()

    all_new: list[dict] = []
    for name, fn in [
        ("n6 BT", lambda: absorb_n6_bt(existing["bt_absorption"])),
        ("nexus atlas MILL", lambda: absorb_nexus_atlas_mill(existing["atlas_mill_dfs_absorption"])),
        ("nexus docs", lambda: absorb_nexus_docs(existing["nexus_docs_absorption"])),
        ("anima docs", lambda: absorb_anima_docs(existing["anima_docs_absorption"])),
        ("anima laws", lambda: absorb_anima_laws(existing["anima_law_absorption"])),
        ("anima checkpoints", lambda: absorb_anima_checkpoints(existing["anima_checkpoint_absorption"])),
    ]:
        batch = fn()
        print(f"  [+{len(batch)}] {name}")
        all_new.extend(batch)
    print()

    all_new.sort(key=lambda e: e["ts"])
    print(f"총 신규 엔트리: {len(all_new)}")

    if not all_new:
        print("신규 없음 — 완료")
        return

    SINK_NEXUS.parent.mkdir(parents=True, exist_ok=True)
    with SINK_NEXUS.open("a", encoding="utf-8") as f:
        for e in all_new:
            f.write(json.dumps(e, ensure_ascii=False) + "\n")
    print(f"  → {SINK_NEXUS}")

    if SINK_N6.is_symlink() and SINK_N6.resolve() == SINK_NEXUS.resolve():
        print(f"  ↪ {SINK_N6} (symlink → 기록 생략)")
    else:
        SINK_N6.parent.mkdir(parents=True, exist_ok=True)
        with SINK_N6.open("a", encoding="utf-8") as f:
            for e in all_new:
                f.write(json.dumps(e, ensure_ascii=False) + "\n")
        print(f"  → {SINK_N6}")
    print()
    import subprocess
    wc1 = subprocess.run(["wc", "-l", str(SINK_NEXUS)], capture_output=True, text=True).stdout.strip()
    wc2 = subprocess.run(["wc", "-l", str(SINK_N6)], capture_output=True, text=True).stdout.strip()
    print(f"최종:")
    print(f"  nexus/shared/discovery_log.jsonl  = {wc1}")
    print(f"  n6shared/discovery_log.jsonl     = {wc2}")


if __name__ == "__main__":
    main()
