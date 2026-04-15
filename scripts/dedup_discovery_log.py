#!/usr/bin/env python3
"""
discovery_log.jsonl 중복 제거
- absorbed_reason="backfill_*" 엔트리 중복 최초 1회만 보존
- 기존 legacy consensus_candidate 등은 그대로 유지
"""
from __future__ import annotations
import json
from pathlib import Path

SINK = Path("/Users/ghost/Dev/nexus/shared/discovery_log.jsonl")
BAK = Path("/Users/ghost/Dev/nexus/shared/discovery_log.jsonl.bak.dedup")


def entry_key(e: dict) -> str:
    kind = e.get("kind", "")
    if kind == "bt_absorption":
        return f"bt::{e.get('bt_id', '')}"
    if kind == "atlas_mill_dfs_absorption":
        return f"atlas::{e.get('atlas_id', '')}"
    if kind == "nexus_docs_absorption":
        return f"nd::{e.get('source_file', '')}"
    if kind == "anima_docs_absorption":
        return f"ad::{e.get('source_file', '')}"
    if kind == "anima_law_absorption":
        return f"al::{e.get('law_id', '')}"
    if kind == "anima_checkpoint_absorption":
        return f"ac::{e.get('checkpoint_path', '')}"
    return ""


def main():
    SINK.rename(BAK)
    seen: set[str] = set()
    kept = 0
    dropped = 0
    legacy = 0
    with BAK.open("r", encoding="utf-8") as fin, SINK.open("w", encoding="utf-8") as fout:
        for line in fin:
            s = line.rstrip("\n")
            if not s:
                continue
            try:
                e = json.loads(s)
            except Exception:
                fout.write(line)
                legacy += 1
                continue
            k = entry_key(e)
            if not k:
                fout.write(line)
                legacy += 1
                continue
            if k in seen:
                dropped += 1
                continue
            seen.add(k)
            fout.write(line)
            kept += 1
    print(f"  legacy (dedup skip): {legacy}")
    print(f"  kept (first occurrence): {kept}")
    print(f"  dropped (duplicates): {dropped}")
    print(f"  최종 라인 수: {kept + legacy}")
    print(f"  백업: {BAK}")


if __name__ == "__main__":
    main()
