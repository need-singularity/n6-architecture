#!/usr/bin/env python3
"""
BT 흡수 복구 스크립트
- theory/breakthroughs/*.md → discovery_log.jsonl (양쪽 리포)
- atlas.n6 MILL-DFS2x 엔트리 → discovery_log 엔트리화
- atlas.n6.deg 재생성 트리거

고장 시점: 2026-04-14T03:40:18Z 이후 미흡수
"""
from __future__ import annotations
import json
import os
import re
import sys
import hashlib
import subprocess
from datetime import datetime, timezone
from pathlib import Path

N6 = Path("/Users/ghost/Dev/n6-architecture")
NEXUS = Path("/Users/ghost/Dev/nexus")

BT_DIR = N6 / "theory" / "breakthroughs"
HARNESS_DIR = N6 / "theory" / "predictions"
ATLAS_N6 = NEXUS / "shared" / "n6" / "atlas.n6"
ATLAS_DEG = NEXUS / "shared" / "n6" / "atlas.n6.deg"

DISC_LOG_NEXUS = NEXUS / "shared" / "discovery_log.jsonl"
DISC_LOG_N6 = N6 / "n6shared" / "discovery_log.jsonl"

BREAKAGE_TS = "2026-04-14T03:40:18Z"
BREAKAGE_EPOCH = datetime.fromisoformat(BREAKAGE_TS.replace("Z", "+00:00")).timestamp()


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


def extract_bt_info(path: Path) -> dict | None:
    try:
        text = path.read_text(encoding="utf-8")
    except Exception:
        return None
    fm = parse_frontmatter(text)
    if not fm:
        stem = path.stem
        m_id = re.match(r"(bt-\d+[\w-]*?)(?:-\d{4}-\d{2}-\d{2})?$", stem)
        bt_id = m_id.group(1) if m_id else stem
        first_h = re.search(r"^#\s+(.+)$", text, re.MULTILINE)
        first_heading = first_h.group(1).strip() if first_h else stem
        fm = {
            "id": bt_id,
            "date": "",
            "parent_bt": "",
            "grade": "[legacy no-fm]",
            "dfs_round": "",
            "dfs_area": first_heading[:80],
            "new_tight": "",
            "cumulative_tight": "",
            "solved": "",
            "harness": "",
            "_body": text,
        }
    body = fm.get("_body", "")
    tight_count = 0
    for line in body.split("\n"):
        if re.match(r"^\|\s*\d+-\d+\s*\|", line):
            tight_count += 1
    pass_match = re.search(r"(\d+)\s*PASS\s*/\s*(\d+)\s*FAIL", body)
    harness_pass = int(pass_match.group(1)) if pass_match else 0
    harness_fail = int(pass_match.group(2)) if pass_match else 0
    return {
        "bt_id": fm.get("id", path.stem),
        "date": fm.get("date", ""),
        "parent_bt": fm.get("parent_bt", ""),
        "grade": fm.get("grade", ""),
        "dfs_round": fm.get("dfs_round", ""),
        "dfs_area": fm.get("dfs_area", ""),
        "new_tight": fm.get("new_tight", str(tight_count)),
        "cumulative_tight": fm.get("cumulative_tight", ""),
        "solved": fm.get("solved", ""),
        "harness": fm.get("harness", ""),
        "harness_pass": harness_pass,
        "harness_fail": harness_fail,
        "file": str(path.relative_to(N6)),
        "file_mtime": path.stat().st_mtime,
    }


def to_discovery_entry(info: dict) -> dict:
    ts = datetime.fromtimestamp(info["file_mtime"], tz=timezone.utc).isoformat().replace("+00:00", "Z")
    ts = re.sub(r"\.\d+Z$", "Z", ts)
    simhash = hashlib.md5(info["bt_id"].encode()).hexdigest()[:32]
    return {
        "ts": ts,
        "kind": "bt_absorption",
        "bt_id": info["bt_id"],
        "source_file": info["file"],
        "parent_bt": info["parent_bt"],
        "grade": info["grade"],
        "dfs_round": info["dfs_round"],
        "dfs_area": info["dfs_area"],
        "new_tight": info["new_tight"],
        "cumulative_tight": info["cumulative_tight"],
        "solved": info["solved"],
        "harness_pass": info["harness_pass"],
        "harness_fail": info["harness_fail"],
        "simhash": simhash,
        "absorbed_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "absorbed_reason": "backfill_from_breakage_20260414",
    }


def main():
    print("=" * 60)
    print(" BT → discovery_log 흡수 복구")
    print(f" 고장 시점: {BREAKAGE_TS}")
    print("=" * 60)
    print()

    all_bt = sorted(BT_DIR.glob("*.md"))
    print(f"[1] 전체 BT 파일: {len(all_bt)}")

    existing_ids_nexus = set()
    if DISC_LOG_NEXUS.exists():
        for line in DISC_LOG_NEXUS.read_text(encoding="utf-8").split("\n"):
            if not line.strip():
                continue
            try:
                e = json.loads(line)
                if e.get("kind") == "bt_absorption":
                    existing_ids_nexus.add(e.get("bt_id"))
            except Exception:
                pass

    to_absorb = []
    for p in all_bt:
        info = extract_bt_info(p)
        if not info:
            continue
        if info["bt_id"] in existing_ids_nexus:
            continue
        to_absorb.append(info)

    print(f"[2] 흡수 대상 (신규): {len(to_absorb)}")
    print()

    entries = [to_discovery_entry(i) for i in to_absorb]
    entries.sort(key=lambda e: e["ts"])

    DISC_LOG_NEXUS.parent.mkdir(parents=True, exist_ok=True)
    with DISC_LOG_NEXUS.open("a", encoding="utf-8") as f:
        for e in entries:
            f.write(json.dumps(e, ensure_ascii=False) + "\n")
    print(f"[3] nexus/shared/discovery_log.jsonl: +{len(entries)}")

    DISC_LOG_N6.parent.mkdir(parents=True, exist_ok=True)
    with DISC_LOG_N6.open("a", encoding="utf-8") as f:
        for e in entries:
            f.write(json.dumps(e, ensure_ascii=False) + "\n")
    print(f"[4] n6shared/discovery_log.jsonl: +{len(entries)}")

    atlas_entries = []
    if ATLAS_N6.exists():
        for line in ATLAS_N6.read_text(encoding="utf-8").split("\n"):
            m = re.match(r"^@R\s+(MILL-DFS2\d+[\w-]*)\s*=\s*(.+?)\s*::\s*n6atlas\s*\[([^\]]+)\]", line)
            if m:
                atlas_entries.append({"id": m.group(1), "stmt": m.group(2), "grade": m.group(3)})
    print(f"[5] atlas.n6 MILL-DFS2x 엔트리: {len(atlas_entries)}")

    existing_atlas = set()
    if DISC_LOG_NEXUS.exists():
        for line in DISC_LOG_NEXUS.read_text(encoding="utf-8").split("\n"):
            if not line.strip():
                continue
            try:
                e = json.loads(line)
                if e.get("kind") == "atlas_mill_dfs_absorption":
                    existing_atlas.add(e.get("atlas_id"))
            except Exception:
                pass

    atlas_new = [a for a in atlas_entries if a["id"] not in existing_atlas]
    print(f"[6] atlas 신규: {len(atlas_new)}")

    now_iso = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    atlas_log_entries = []
    for a in atlas_new:
        entry = {
            "ts": now_iso,
            "kind": "atlas_mill_dfs_absorption",
            "atlas_id": a["id"],
            "statement": a["stmt"],
            "grade": a["grade"],
            "source": "atlas.n6",
            "absorbed_at": now_iso,
            "absorbed_reason": "backfill_from_breakage_20260414",
        }
        atlas_log_entries.append(entry)

    with DISC_LOG_NEXUS.open("a", encoding="utf-8") as f:
        for e in atlas_log_entries:
            f.write(json.dumps(e, ensure_ascii=False) + "\n")
    with DISC_LOG_N6.open("a", encoding="utf-8") as f:
        for e in atlas_log_entries:
            f.write(json.dumps(e, ensure_ascii=False) + "\n")
    print(f"[7] atlas.n6 엔트리 흡수 완료: +{len(atlas_log_entries)}")
    print()

    print(f"=== 최종 ===")
    print(f"  nexus/shared/discovery_log.jsonl = {DISC_LOG_NEXUS.stat().st_size:,} bytes")
    wc = subprocess.run(["wc", "-l", str(DISC_LOG_NEXUS)], capture_output=True, text=True)
    print(f"  라인 수: {wc.stdout.strip()}")
    wc2 = subprocess.run(["wc", "-l", str(DISC_LOG_N6)], capture_output=True, text=True)
    print(f"  n6shared 라인 수: {wc2.stdout.strip()}")


if __name__ == "__main__":
    main()
