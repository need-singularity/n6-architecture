#!/usr/bin/env python3
# NUM-PX-3: arXiv 서베이 — 6 millennium BT 최근 논문 수집
# 소스: arXiv Atom API (http://export.arxiv.org/api/query)
# 출력: data/arxiv/millennium_survey_6bt.json

import json
import re
import sys
import time
import urllib.parse
import urllib.request
from pathlib import Path
from datetime import datetime

ROOT = Path("/Users/ghost/Dev/n6-architecture")
OUT = ROOT / "data" / "arxiv" / "millennium_survey_6bt.json"

# 6 BT 별 arXiv search query
BT_QUERIES = {
    "BT-541 RH": {
        "desc": "Riemann Hypothesis — zeta zeros, L-function analogs",
        "query": 'cat:math.NT AND (abs:"Riemann hypothesis" OR abs:"zeta zeros" OR abs:"critical line")',
    },
    "BT-542 P vs NP": {
        "desc": "P vs NP — circuit complexity, meta-complexity, barriers",
        "query": 'cat:cs.CC AND (abs:"P vs NP" OR abs:"circuit lower bound" OR abs:"meta-complexity")',
    },
    "BT-543 Yang-Mills": {
        "desc": "Yang-Mills mass gap — QCD, lattice gauge, rigorous construction",
        "query": 'cat:math-ph AND (abs:"Yang-Mills" OR abs:"mass gap" OR abs:"gauge theory construction")',
    },
    "BT-544 Navier-Stokes": {
        "desc": "Navier-Stokes regularity — blowup, weak solutions, Onsager",
        "query": 'cat:math.AP AND (abs:"Navier-Stokes" AND (abs:"regularity" OR abs:"blowup" OR abs:"Onsager"))',
    },
    "BT-545 Hodge": {
        "desc": "Hodge conjecture — algebraic cycles, Tate, Abel-Jacobi",
        "query": 'cat:math.AG AND (abs:"Hodge conjecture" OR abs:"Tate conjecture" OR abs:"algebraic cycles")',
    },
    "BT-546 BSD": {
        "desc": "BSD conjecture — Selmer groups, elliptic curves, Iwasawa",
        "query": 'cat:math.NT AND (abs:"BSD conjecture" OR abs:"Birch Swinnerton-Dyer" OR abs:"Selmer group" OR abs:"Iwasawa invariant")',
    },
}


def strip_tag(s):
    return re.sub(r"<[^>]+>", "", s).strip()


def parse_entries(raw: str) -> list:
    """Atom feed 파싱. 각 entry 는 <entry>...</entry>."""
    entries = []
    for m in re.finditer(r"<entry>(.*?)</entry>", raw, re.DOTALL):
        body = m.group(1)
        t = re.search(r"<title>(.*?)</title>", body, re.DOTALL)
        a = re.search(r"<published>(.*?)</published>", body)
        aid_m = re.search(r"<id>http://arxiv\.org/abs/(.*?)</id>", body)
        sum_m = re.search(r"<summary>(.*?)</summary>", body, re.DOTALL)
        authors = re.findall(r"<author>\s*<name>(.*?)</name>", body)
        if t and a and aid_m:
            entries.append({
                "id": aid_m.group(1).strip(),
                "title": re.sub(r"\s+", " ", strip_tag(t.group(1))),
                "authors": [au.strip() for au in authors][:6],
                "published": a.group(1).strip(),
                "summary": re.sub(r"\s+", " ", strip_tag(sum_m.group(1))[:400]) if sum_m else "",
            })
    return entries


def fetch_bt(bt_name: str, query: str, max_per_page=30) -> list:
    params = {
        "search_query": query,
        "start": 0,
        "max_results": max_per_page,
        "sortBy": "submittedDate",
        "sortOrder": "descending",
    }
    url = "http://export.arxiv.org/api/query?" + urllib.parse.urlencode(params)
    try:
        with urllib.request.urlopen(url, timeout=30) as r:
            raw = r.read().decode("utf-8")
    except Exception as e:
        print(f"[{bt_name}] FAIL: {e}", file=sys.stderr)
        return []
    entries = parse_entries(raw)
    print(f"[{bt_name}] {len(entries)} papers 수집", file=sys.stderr)
    return entries


def main():
    all_results = {}
    for bt, cfg in BT_QUERIES.items():
        print(f"\n[수집] {bt} — {cfg['desc']}", file=sys.stderr)
        entries = fetch_bt(bt, cfg["query"], max_per_page=30)
        all_results[bt] = {
            "description": cfg["desc"],
            "query": cfg["query"],
            "n_papers": len(entries),
            "papers": entries,
        }
        time.sleep(3)  # arXiv rate limit etiquette (3초 권장)

    # 요약
    print()
    print("=" * 70)
    print("[arXiv 서베이 요약]")
    print("=" * 70)
    total = 0
    for bt, data in all_results.items():
        n = data["n_papers"]
        total += n
        print(f"  {bt}: {n} papers")
        if data["papers"]:
            latest = data["papers"][0]
            print(f"    최신: {latest['published'][:10]} — {latest['title'][:70]}")
    print(f"\n총 {total} papers 수집")

    # 2024-2026 필터
    recent = {}
    for bt, data in all_results.items():
        recent_papers = [p for p in data["papers"] if p["published"] >= "2024-01-01"]
        recent[bt] = {
            "description": data["description"],
            "n_papers_total": data["n_papers"],
            "n_papers_2024_plus": len(recent_papers),
            "papers": recent_papers,
        }

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps({
        "source": "arXiv Atom API (http://export.arxiv.org/api/query)",
        "fetched_at": datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ"),
        "query_per_bt": {bt: cfg["query"] for bt, cfg in BT_QUERIES.items()},
        "all_papers": all_results,
        "recent_2024_plus": recent,
    }, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"\n[저장] {OUT}")


if __name__ == "__main__":
    main()
