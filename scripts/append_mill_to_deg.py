#!/usr/bin/env python3
"""
atlas.n6.deg 에 MILL-DFS2x 엔트리만 safe-append
원본 .deg (21264 줄) 건드리지 않고 신규 70개만 추가.
"""
from __future__ import annotations
import re
from pathlib import Path

ATLAS = Path("/Users/ghost/Dev/nexus/shared/n6/atlas.n6")
DEG = Path("/Users/ghost/Dev/nexus/shared/n6/atlas.n6.deg")


def slugify(s: str) -> str:
    s = s.lower()
    s = re.sub(r"[^a-z0-9]+", "_", s)
    s = re.sub(r"_+", "_", s).strip("_")
    return s


def main():
    existing_slugs: set[str] = set()
    with DEG.open("r", encoding="utf-8", errors="replace") as f:
        for line in f:
            parts = line.rstrip("\n").split("\t")
            if parts:
                existing_slugs.add(parts[0])
    print(f"기존 .deg 엔트리: {len(existing_slugs)}")

    text = ATLAS.read_text(encoding="utf-8", errors="replace")

    mill_ids = []
    for m in re.finditer(r"^@R\s+(MILL-DFS\d+[\w-]+)", text, re.MULTILINE):
        mill_ids.append(m.group(1))

    print(f"atlas.n6 MILL-DFS 엔트리: {len(mill_ids)}")

    new_count = 0
    with DEG.open("a", encoding="utf-8") as f:
        for mid in mill_ids:
            slug = slugify(mid)
            if slug in existing_slugs:
                continue
            f.write(f"{slug}\t3\n")
            existing_slugs.add(slug)
            new_count += 1

    print(f"신규 append: {new_count}")
    print(f"최종 .deg 엔트리: {len(existing_slugs)}")


if __name__ == "__main__":
    main()
