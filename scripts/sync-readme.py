#!/usr/bin/env python3
"""README SSOT 동기화 — shared/readme-data.json → README.md 마커 기반 자동 생성.

Usage:
    python3 scripts/sync-readme.py           # 동기화 실행
    python3 scripts/sync-readme.py --check   # 변경 필요 여부만 확인 (CI용)
    python3 scripts/sync-readme.py --dry-run # 변경 내용 미리보기

데이터 원본: shared/readme-data.json (유일한 SSOT)
마커 형식:   <!-- AUTO:SECTION:START --> ... <!-- AUTO:SECTION:END -->
직접 편집:   마커 밖의 내용은 건드리지 않음
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


def gen_badge(data: dict) -> str:
    s = data["stats"]
    lines = [
        f'[![DSE](https://img.shields.io/badge/DSE-{s["dse_domains"]}%20domains-blue.svg)](docs/dse-map.toml)',
        f'[![NEXUS-6](https://img.shields.io/badge/NEXUS--6-{s["nexus6_tests"]}%20tests-green.svg)](tools/nexus6/)',
    ]
    return "\n".join(lines)


def gen_stats(data: dict) -> str:
    s = data["stats"]
    return (
        "```\n"
        f'  AI techniques:    {s["ai_techniques"]}\n'
        f'  DSE domains:      {s["dse_domains"]}\n'
        f'  DSE paths:        {s["dse_paths"]}\n'
        f'  NEXUS-6 tests:    {s["nexus6_tests"]}\n'
        "```"
    )


def gen_alien_index(data: dict) -> str:
    header = (
        "| 섹션 | 🛸 구현 | 천장확인 | BT검증 | 산업검증 | 실험검증 | TP | 발견 |\n"
        "|------|:------:|:------:|:------:|:-------:|:-------:|:--:|:----:|"
    )
    rows = []
    for d in data["domains"]:
        if d["id"] == "safety":
            continue  # 아직 외계인 지수 테이블에 넣기엔 이름
        anchor = d["anchor"]
        section = f'[{d["emoji"]} {d["name"]}](#{anchor})'
        alien = f'🛸{d["alien"]}'
        ceiling = "✅" if d["ceiling"] else "❌"
        bt = f'{d["bt_exact_pct"]}%' if d["bt_exact_pct"] is not None else "—"

        # industry
        ind_parts = []
        if d["industry_pct"] is not None:
            ind_parts.append(f'{d["industry_pct"]}%')
        if d["industry_detail"]:
            ind_parts.append(f'({d["industry_detail"]})')
        industry = " ".join(ind_parts) if ind_parts else "—"

        # experiment
        exp_parts = []
        if d["experiment_pct"] is not None:
            exp_parts.append(f'{d["experiment_pct"]}%')
        if d["experiment_detail"]:
            exp_parts.append(d["experiment_detail"])
        experiment = " ".join(exp_parts) if exp_parts else "—"

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
        rank = str(r["rank"])
        year = str(r["year"])
        name = f'**{r["name"]}**'
        impact = _impact_stars(r["impact"])
        tier = str(r["tier"])
        dse = str(r["dse"])
        rows.append(f"| {rank} | {year} | {name} | {impact} | {tier} | {dse} |")
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


GENERATORS = {
    "BADGE": gen_badge,
    "STATS": gen_stats,
    "ALIEN_INDEX": gen_alien_index,
    "ROADMAP": gen_roadmap,
    "REFERENCE": gen_reference,
}


def sync(readme_text: str, data: dict) -> str:
    def replacer(m):
        start_tag = m.group(1)
        section = m.group(2)
        end_tag = m.group(3)
        gen = GENERATORS.get(section)
        if gen is None:
            return m.group(0)  # 알 수 없는 섹션은 건드리지 않음
        content = gen(data)
        return f"{start_tag}\n{content}\n{end_tag}"

    return MARKER_RE.sub(replacer, readme_text)


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
        # 변경된 마커 섹션만 표시
        old_sections = {m[1]: m[0] for m in MARKER_RE.findall(old_text)}
        new_sections = {m[1]: m[0] for m in MARKER_RE.findall(new_text)}
        for section in GENERATORS:
            if old_sections.get(section) != new_sections.get(section):
                print(f"  → AUTO:{section} changed")
        return 0

    README_PATH.write_text(new_text)
    print(f"✅ README.md synced from {DATA_PATH.name}")

    # 마커별 상태
    for section in GENERATORS:
        marker = f"<!-- AUTO:{section}:START -->"
        if marker in old_text:
            print(f"  ✓ AUTO:{section}")
        else:
            print(f"  ⚠ AUTO:{section} — marker not found in README.md (add it manually)")

    return 0


if __name__ == "__main__":
    sys.exit(main())
