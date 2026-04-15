#!/usr/bin/env python3
"""
atlas.n6.deg 재생성 (경량 버전)
- atlas.n6 의 모든 @R id 추출 → 슬러그화 → 등장 빈도 계산
- TSV 출력: slug\tdegree
"""
from __future__ import annotations
import re
from pathlib import Path
from collections import Counter

ATLAS = Path("/Users/ghost/Dev/nexus/shared/n6/atlas.n6")
DEG_OUT = Path("/Users/ghost/Dev/nexus/shared/n6/atlas.n6.deg")
DEG_BACKUP = Path("/Users/ghost/Dev/nexus/shared/n6/atlas.n6.deg.bak.pre-backfill")


def slugify(s: str) -> str:
    s = s.lower()
    s = re.sub(r"[^a-z0-9가-힣]+", "_", s)
    s = re.sub(r"_+", "_", s).strip("_")
    return s


def main():
    text = ATLAS.read_text(encoding="utf-8", errors="replace")

    ids: list[str] = []
    for m in re.finditer(r"^@R\s+([A-Za-z0-9_\-]+)\s*=\s*(.+?)\s*::\s*n6atlas", text, re.MULTILINE):
        rid = m.group(1).strip()
        stmt = m.group(2).strip()
        ids.append(slugify(rid + "-" + stmt[:40]))

    print(f"@R 엔트리 총합: {len(ids)}")

    if DEG_OUT.exists() and not DEG_BACKUP.exists():
        DEG_BACKUP.write_bytes(DEG_OUT.read_bytes())
        print(f"백업 완료: {DEG_BACKUP}")

    cnt = Counter()
    for s in ids:
        cnt[s] += 2
    for m in re.finditer(r"MILL-DFS(\d+)-(\d+)-([\w-]+)", text):
        slug = slugify(f"mill-dfs{m.group(1)}-{m.group(2)}-{m.group(3)}")
        cnt[slug] += 1

    with DEG_OUT.open("w", encoding="utf-8") as f:
        for slug, d in cnt.most_common():
            f.write(f"{slug}\t{d}\n")
    print(f".deg 재생성: {DEG_OUT}")
    print(f"  엔트리 수: {len(cnt)}")
    print(f"  파일 크기: {DEG_OUT.stat().st_size:,} bytes")

    mill_count = sum(1 for k in cnt.keys() if k.startswith("mill_dfs"))
    print(f"  MILL-DFS 엔트리 포함: {mill_count}")


if __name__ == "__main__":
    main()
