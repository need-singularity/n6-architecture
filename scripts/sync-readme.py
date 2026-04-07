#!/usr/bin/env python3
"""README SSOT 동기화 — shared/readme-data.json → README.md 마커 기반 자동 생성.

Usage:
    python3 scripts/sync-readme.py           # 동기화 실행
    python3 scripts/sync-readme.py --check   # 변경 필요 여부만 확인 (CI용)
    python3 scripts/sync-readme.py --dry-run # 변경 내용 미리보기

데이터 원본: shared/readme-data.json (유일한 SSOT)
마커 형식:   <!-- AUTO:SECTION:START --> ... <!-- AUTO:SECTION:END -->
직접 편집:   마커 밖의 내용은 건드리지 않음

마커 종류:
  AUTO:BADGE           — DSE/NEXUS-6 뱃지
  AUTO:STATS           — 통계 블록
  AUTO:ALIEN_INDEX     — 외계인 지수 테이블
  AUTO:ROADMAP         — 로드맵 테이블
  AUTO:REFERENCE       — 참조 테이블
  AUTO:SUMMARY_<id>    — 도메인별 요약 blockquote
  AUTO:FOOTER_<id>     — 도메인별 풋터 링크
"""

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA_PATH = ROOT / "shared" / "readme-data.json"
README_PATH = ROOT / "README.md"

MARKER_RE = re.compile(
    r"(<!-- AUTO:(\w+):START -->)\n.*?\n(<!-- AUTO:\2:END -->)",
    re.DOTALL,
)


def load_data() -> dict:
    with open(DATA_PATH) as f:
        return json.load(f)


def _find_domain(data: dict, domain_id: str) -> dict:
    for d in data["domains"]:
        if d["id"] == domain_id:
            return d
    return None


# ─── Global generators ───


def gen_badge(data: dict) -> str:
    s = data["stats"]
    return "\n".join([
        f'[![DSE](https://img.shields.io/badge/DSE-{s["dse_domains"]}%20domains-blue.svg)](docs/dse-map.toml)',
        f'[![NEXUS-6](https://img.shields.io/badge/NEXUS--6-{s["nexus_tests"]}%20tests-green.svg)](tools/nexus/)',
    ])


def gen_stats(data: dict) -> str:
    s = data["stats"]
    return (
        "```\n"
        f'  AI techniques:    {s["ai_techniques"]}\n'
        f'  DSE domains:      {s["dse_domains"]}\n'
        f'  DSE paths:        {s["dse_paths"]}\n'
        f'  NEXUS-6 tests:    {s["nexus_tests"]}\n'
        "```"
    )


def _fmt_pct(val) -> str:
    if val is None:
        return ""
    if isinstance(val, str):
        return f"{val}%"
    if isinstance(val, (int, float)):
        # 100.0 → "100%", 82.2 → "82.2%"
        return f"{val:g}%"
    return str(val)


def _fmt_industry(d: dict) -> str:
    parts = []
    if d.get("industry_pct") is not None:
        parts.append(f'{d["industry_pct"]:g}%' if isinstance(d["industry_pct"], (int, float)) else str(d["industry_pct"]) + "%")
    if d.get("industry_detail"):
        parts.append(f'({d["industry_detail"]})')
    return " ".join(parts) if parts else "—"


def _fmt_experiment(d: dict) -> str:
    parts = []
    if d.get("experiment_pct") is not None:
        parts.append(f'{d["experiment_pct"]:g}%' if isinstance(d["experiment_pct"], (int, float)) else str(d["experiment_pct"]) + "%")
    if d.get("experiment_detail"):
        parts.append(d["experiment_detail"])
    return " ".join(parts) if parts else "—"


def gen_alien_index(data: dict) -> str:
    header = (
        "| 섹션 | 🛸 구현 | 천장확인 | BT검증 | 산업검증 | 실험검증 | TP | 발견 |\n"
        "|------|:------:|:------:|:------:|:-------:|:-------:|:--:|:----:|"
    )
    rows = []
    for d in data["domains"]:
        anchor = d["anchor"]
        section = f'[{d["emoji"]} {d["name"]}](#{anchor})'
        alien = f'🛸{d["alien"]}'
        ceiling = "✅" if d["ceiling"] else "❌"
        bt = _fmt_pct(d["bt_exact_pct"]) or "—"
        industry = _fmt_industry(d)
        experiment = _fmt_experiment(d)
        tp = str(d["tp"])
        disc = str(d["discoveries"])
        rows.append(f"| {section} | {alien} | {ceiling} | {bt} | {industry} | {experiment} | {tp} | {disc} |")
    return header + "\n" + "\n".join(rows)


def _impact_stars(n: int) -> str:
    return "★" * n + "☆" * (6 - n) if n <= 6 else "★" * n


def gen_roadmap(data: dict) -> str:
    header = (
        "| 순위 | 실현 | 도메인 | 영향력 | Tier | DSE |\n"
        "|:---:|:---:|--------|:---:|:---:|:---:|"
    )
    rows = []
    for r in data["roadmap"]:
        rows.append(
            f'| {r["rank"]} | {r["year"]} | **{r["name"]}** '
            f'| {_impact_stars(r["impact"])} | {r["tier"]} | {r["dse"]} |'
        )
    return header + "\n" + "\n".join(rows)


def gen_reference(data: dict) -> str:
    s = data["stats"]
    return (
        "| 항목 | 링크 |\n"
        "|------|------|\n"
        "| **n=6 상수표** | σ=12, τ=4, φ=2, sopfr=5, J₂=24, σ-τ=8, 1/(σ-φ)=0.1 |\n"
        f'| **{s["bt_count"]} Breakthrough Theorems** | [docs/breakthrough-theorems.md](docs/breakthrough-theorems.md) |\n'
        f'| **{s["atlas_constants"]} Atlas Constants** | [docs/atlas-constants.md](docs/atlas-constants.md) |\n'
        f'| **{s["testable_predictions"]} Testable Predictions** | [docs/testable-predictions.md](docs/testable-predictions.md) |\n'
        "| **DSE Map** | [docs/dse-map.toml](docs/dse-map.toml) |\n"
        f'| **{s["dse_toml"]} DSE Domains** | [docs/dse-domains.md](docs/dse-domains.md) |\n'
        "| **Cross-Domain Resonance** | [docs/cross-domain-resonance-2026-03-31.md](docs/cross-domain-resonance-2026-03-31.md) |\n"
        "| **Core Theorem Proof** | [docs/theorem-r1-uniqueness.md](docs/theorem-r1-uniqueness.md) |\n"
        f'| **{s["calculators"]} Calculators** | [docs/calculator-registry.md](docs/calculator-registry.md) |\n'
        "| **Universal DSE** | `tools/universal-dse/` — TOML 1개로 즉시 DSE |"
    )


# ─── Per-domain generators ───


def gen_summary(data: dict, domain_id: str) -> str:
    """도메인 요약 blockquote 생성. 모든 수치를 JSON에서 가져옴."""
    d = _find_domain(data, domain_id)
    if d is None:
        return f"> (unknown domain: {domain_id})"

    parts = [f'**🛸{d["alien"]}**']

    if d["ceiling"]:
        parts.append("✅")

    # BT
    bt_str = f'BT {d["bt_count"]}개' if d.get("bt_count") else ""
    if d.get("bt_exact_pct") is not None:
        pct = d["bt_exact_pct"]
        pct_s = f"{pct:g}" if isinstance(pct, (int, float)) else str(pct)
        bt_str += f" {pct_s}%EXACT" if bt_str else f"BT {pct_s}%EXACT"
    if bt_str:
        parts.append(bt_str)

    # DSE
    if d.get("dse_count"):
        parts.append(f'DSE {d["dse_count"]}')

    # Industry
    ind = _fmt_industry(d)
    if ind != "—":
        parts.append(f"산업{ind}")

    # Experiment
    exp = _fmt_experiment(d)
    if exp != "—":
        parts.append(f"실험{exp}")

    # Physical limits
    if d.get("physical_limits"):
        parts.append(f'물리한계{d["physical_limits"]}')

    # TP
    parts.append(f'TP{d["tp"]}')

    # Discoveries
    if d.get("discoveries") and str(d["discoveries"]) != "0":
        parts.append(f'발견{d["discoveries"]}')

    # Extras
    for ex in d.get("extras", []):
        parts.append(ex)

    return "> " + " | ".join(parts)


def gen_footer(data: dict, domain_id: str) -> str:
    """도메인 풋터 링크 생성."""
    d = _find_domain(data, domain_id)
    if d is None:
        return f"> (unknown domain: {domain_id})"

    parts = []

    # Doc links
    doc_links = []
    for doc in d.get("docs", []):
        name = doc.rstrip("/")
        doc_links.append(f"[{doc}](docs/{doc})")
    if doc_links:
        parts.append("도메인: " + " · ".join(doc_links))

    # Tool links
    tool_links = [f"`{t}`" for t in d.get("tools", [])]
    if tool_links:
        parts.append("도구: " + " · ".join(tool_links))

    # Footer extra
    if d.get("footer_extra"):
        parts.append(d["footer_extra"])

    return "> " + " · ".join(parts)


# ─── Dispatch ───

GLOBAL_GENERATORS = {
    "BADGE": gen_badge,
    "STATS": gen_stats,
    "ALIEN_INDEX": gen_alien_index,
    "ROADMAP": gen_roadmap,
    "REFERENCE": gen_reference,
}


def resolve_generator(section: str, data: dict) -> str:
    """섹션 이름에서 적절한 생성기를 찾아 실행."""
    if section in GLOBAL_GENERATORS:
        return GLOBAL_GENERATORS[section](data)

    if section.startswith("SUMMARY_"):
        domain_id = section[8:]  # len("SUMMARY_") == 8
        return gen_summary(data, domain_id)

    if section.startswith("FOOTER_"):
        domain_id = section[7:]  # len("FOOTER_") == 7
        return gen_footer(data, domain_id)

    return None  # unknown → skip


def sync(readme_text: str, data: dict) -> str:
    def replacer(m):
        start_tag = m.group(1)
        section = m.group(2)
        end_tag = m.group(3)
        content = resolve_generator(section, data)
        if content is None:
            return m.group(0)
        return f"{start_tag}\n{content}\n{end_tag}"

    return MARKER_RE.sub(replacer, readme_text)


def _list_markers(text: str) -> list:
    return [m[1] for m in MARKER_RE.findall(text)]


def main():
    check_only = "--check" in sys.argv
    dry_run = "--dry-run" in sys.argv

    data = load_data()
    old_text = README_PATH.read_text()
    new_text = sync(old_text, data)

    if old_text == new_text:
        print("✅ README.md is up to date with readme-data.json")
        return 0

    if check_only:
        print("❌ README.md is out of sync with readme-data.json")
        print("   Run: python3 scripts/sync-readme.py")
        return 1

    if dry_run:
        markers = _list_markers(old_text)
        old_by_section = {}
        for m in MARKER_RE.finditer(old_text):
            old_by_section[m.group(2)] = m.group(0)
        for m in MARKER_RE.finditer(new_text):
            sec = m.group(2)
            if old_by_section.get(sec) != m.group(0):
                print(f"  → AUTO:{sec} changed")
        return 0

    README_PATH.write_text(new_text)
    markers = _list_markers(new_text)
    print(f"✅ README.md synced from {DATA_PATH.name} ({len(markers)} markers)")
    for sec in markers:
        print(f"  ✓ AUTO:{sec}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
