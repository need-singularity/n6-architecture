#!/usr/bin/env python3
"""
sync_products_readme.py — config/products.json -> README.md 자동 생성

SSOT 원칙: products.json이 원본, README.md의 마커 구간을 자동 치환.

마커 구간:
  - <!-- AUTO:ALIEN_INDEX:START --> ~ <!-- AUTO:ALIEN_INDEX:END -->
  - <!-- AUTO:SUMMARY_<id>:START --> ~ <!-- AUTO:SUMMARY_<id>:END -->
  - <!-- AUTO:FOOTER_<id>:START --> ~ <!-- AUTO:FOOTER_<id>:END -->
  - <!-- AUTO:ROADMAP:START --> ~ <!-- AUTO:ROADMAP:END -->

제품 테이블은 SUMMARY END ~ FOOTER START 사이에서
기존 마커 테이블(| 🛸 |로 시작하는 행들)만 교체.

Usage:
  python3 scripts/sync_products_readme.py [--dry-run]
"""

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PRODUCTS_JSON = ROOT / "config" / "products.json"
README_MD = ROOT / "README.md"


def load_products():
    with open(PRODUCTS_JSON, "r", encoding="utf-8") as f:
        return json.load(f)


def format_links(links):
    """링크 배열을 '[label](path) · [label2](path2)' 형식으로."""
    parts = []
    for lnk in links:
        parts.append(f"[{lnk['label']}]({lnk['path']})")
    return " · ".join(parts)


def gen_alien_index(data):
    """외계인 지수 테이블 생성."""
    lines = []
    lines.append("| 섹션 | 🛸 구현 | 천장확인 | BT검증 | 산업검증 | 실험검증 | TP | 발견 |")
    lines.append("|------|:------:|:------:|:------:|:-------:|:-------:|:--:|:----:|")

    # Use alien_index_order if available, otherwise sections order
    sec_map = {s["id"]: s for s in data["sections"]}
    order = data.get("_meta", {}).get("alien_index_order", [s["id"] for s in data["sections"]])

    for sid in order:
        sec = sec_map.get(sid)
        if not sec:
            continue

        # Use alien_index_row if provided (exact match to README)
        if sec.get("alien_index_row"):
            lines.append(sec["alien_index_row"])
            continue

        icon = sec["icon"]
        title = sec["title"]
        ai = sec["alien_index"]
        ceil = "✅" if sec["ceiling"] else "❌"

        # BT
        if sec.get("bt_exact_pct_display"):
            bt_str = sec["bt_exact_pct_display"]
        elif sec.get("bt_exact_pct") is not None:
            bt_str = f"{sec['bt_exact_pct']}%"
        else:
            bt_str = "—"

        # Industry
        if sec.get("industry_pct") is not None:
            ind_str = f"{sec['industry_pct']}%"
            if sec.get("industry_detail"):
                ind_str += f" ({sec['industry_detail']})"
        elif sec.get("industry_detail"):
            ind_str = f"({sec['industry_detail']})"
        else:
            ind_str = "—"

        # Experiment
        if sec.get("experiment_pct") is not None:
            exp_str = f"{sec['experiment_pct']}%"
            if sec.get("experiment_detail"):
                exp_str += f" {sec['experiment_detail']}"
        elif sec.get("experiment_detail"):
            exp_str = sec["experiment_detail"]
        else:
            exp_str = "—"

        # TP / Discovery
        tp_str = str(sec["tp_count"]) if sec["tp_count"] is not None else "—"
        disc_str = str(sec["discovery_count"]) if sec["discovery_count"] is not None else "—"

        anchor = sec.get("anchor", "")
        sec_link = f"[{icon} {title}](#{anchor})"

        lines.append(
            f"| {sec_link} | 🛸{ai} | {ceil} | {bt_str} | {ind_str} | {exp_str} | {tp_str} | {disc_str} |"
        )

    return "\n".join(lines)


def gen_products_table(sec):
    """제품 테이블 생성."""
    lines = []
    lines.append("| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |")
    lines.append("|:--:|:--:|:---:|---------|------|------|")

    for prod in sec["products"]:
        ufo = str(prod["ufo"]) if prod["ufo"] is not None else "—"
        ceil = " ✅" if prod.get("ceiling") else ""
        ver = prod.get("ver", "")
        name = f"**{prod['name']}**"
        desc = prod.get("description", "")
        links_str = format_links(prod.get("links", []))
        lines.append(f"| {ufo} |{ceil} | {ver} | {name} | {desc} | {links_str} |")

    return "\n".join(lines)


def gen_roadmap(data):
    """로드맵 테이블 생성."""
    lines = []
    lines.append("| 순위 | 실현 | 도메인 | 영향력 | Tier | DSE |")
    lines.append("|:---:|:---:|--------|:---:|:---:|:---:|")

    for item in data["roadmap"]:
        rank = str(item["rank"])
        realize = str(item["realize"])
        name = f"**{item['name']}**"
        stars = "★" * item["impact"] + "☆" * (6 - item["impact"])
        tier = str(item["tier"])
        dse = item.get("dse_info", "—")
        lines.append(f"| {rank} | {realize} | {name} | {stars} | {tier} | {dse} |")

    return "\n".join(lines)


def replace_marker(text, marker_name, new_content):
    """Replace content between <!-- AUTO:MARKER:START --> and <!-- AUTO:MARKER:END -->."""
    pattern = re.compile(
        rf"(<!-- AUTO:{re.escape(marker_name)}:START -->)\n.*?\n(<!-- AUTO:{re.escape(marker_name)}:END -->)",
        re.DOTALL,
    )
    replacement = rf"\1\n{new_content}\n\2"
    new_text, count = pattern.subn(replacement, text)
    return new_text, count


def replace_products_table(text, section_id, products_content):
    """
    Replace ONLY the product table between SUMMARY_END and FOOTER_START.
    Preserves everything else (like <details> blocks).

    Strategy: find the table (starts with '| 🛸 |' header) and replace it.
    """
    summary_end_marker = f"<!-- AUTO:SUMMARY_{section_id}:END -->"
    footer_start_marker = f"<!-- AUTO:FOOTER_{section_id}:START -->"

    idx_sum_end = text.find(summary_end_marker)
    idx_foot_start = text.find(footer_start_marker)

    if idx_sum_end < 0 or idx_foot_start < 0:
        return text

    after_summary = idx_sum_end + len(summary_end_marker)
    between = text[after_summary:idx_foot_start]

    # Find the product table within this region.
    # The table starts with a line containing '| 🛸 |' and ends when lines
    # no longer start with '|'.
    between_lines = between.split("\n")
    table_start = None
    table_end = None

    in_table = False
    for i, line in enumerate(between_lines):
        stripped = line.strip()
        if table_start is None:
            # Look for the table header
            if stripped.startswith("| 🛸 |") or stripped.startswith("| \U0001f6f8"):
                table_start = i
                in_table = True
        elif in_table:
            if stripped.startswith("|"):
                continue  # still in table
            else:
                # First non-table line (blank or otherwise)
                table_end = i
                in_table = False
                break

    if table_start is None:
        # No existing table found, insert after first blank line
        new_between = f"\n\n{products_content}\n\n"
        text = text[:after_summary] + new_between + text[idx_foot_start:]
        return text

    if table_end is None:
        # Table extends to very end -- but preserve trailing blank lines
        # Find last non-blank line from end
        table_end = len(between_lines)
        while table_end > table_start and not between_lines[table_end - 1].strip():
            table_end -= 1

    pre_table = between_lines[:table_start]
    post_table = between_lines[table_end:]

    # Reconstruct: pre-table content + new table + post-table content
    new_between_lines = pre_table + products_content.split("\n") + post_table
    new_between = "\n".join(new_between_lines)

    text = text[:after_summary] + new_between + text[idx_foot_start:]
    return text


def main():
    dry_run = "--dry-run" in sys.argv

    data = load_products()
    readme_text = README_MD.read_text(encoding="utf-8")
    original = readme_text

    # 1. Alien Index table
    alien_idx_content = gen_alien_index(data)
    readme_text, n = replace_marker(readme_text, "ALIEN_INDEX", alien_idx_content)
    if n:
        print(f"  [OK] ALIEN_INDEX 치환 ({n}곳)")

    # 2. Each section: SUMMARY + products table + FOOTER
    for sec in data["sections"]:
        sid = sec["id"]

        # SUMMARY: use raw text if provided, otherwise generate
        if sec.get("summary_raw"):
            summary = sec["summary_raw"]
        else:
            summary = _gen_summary_fallback(sec)
        readme_text, _ = replace_marker(readme_text, f"SUMMARY_{sid}", summary)

        # Products table
        products_table = gen_products_table(sec)
        readme_text = replace_products_table(readme_text, sid, products_table)

        # FOOTER: use raw text if provided, otherwise generate
        if sec.get("footer_raw"):
            footer = sec["footer_raw"]
        else:
            footer = _gen_footer_fallback(sec)
        readme_text, _ = replace_marker(readme_text, f"FOOTER_{sid}", footer)

        print(f"  [OK] {sid}: SUMMARY + 제품테이블 + FOOTER 치환")

    # 3. Roadmap
    roadmap_content = gen_roadmap(data)
    readme_text, n = replace_marker(readme_text, "ROADMAP", roadmap_content)
    if n:
        print(f"  [OK] ROADMAP 치환 ({n}곳)")

    # 4. Write
    if dry_run:
        print("\n[DRY-RUN] README.md 변경 없음")
        if readme_text != original:
            old_lines = original.splitlines()
            new_lines = readme_text.splitlines()
            print(f"  변경: {len(old_lines)}줄 -> {len(new_lines)}줄")
        else:
            print("  변경 없음")
    else:
        README_MD.write_text(readme_text, encoding="utf-8")
        if readme_text != original:
            old_lines = original.splitlines()
            new_lines = readme_text.splitlines()
            print(f"\n[DONE] README.md 갱신 완료 ({len(old_lines)}줄 -> {len(new_lines)}줄)")
        else:
            print("\n[DONE] 변경 없음 (이미 최신)")


def _gen_summary_fallback(sec):
    """Fallback summary generator when summary_raw is not set."""
    parts = [f"**🛸{sec['alien_index']}**"]
    if sec.get("ceiling"):
        parts.append("✅")
    bt_count = sec.get("bt_count")
    pct = sec.get("bt_exact_pct_display") or (f"{sec['bt_exact_pct']}%" if sec.get("bt_exact_pct") is not None else None)
    if bt_count and pct:
        parts.append(f"BT {bt_count}개 {pct}EXACT")
    if sec.get("summary_extra"):
        parts.append(sec["summary_extra"])
    if sec.get("industry_pct") is not None:
        ind = f"산업{sec['industry_pct']}%"
        if sec.get("industry_detail"):
            ind += f" ({sec['industry_detail']})"
        parts.append(ind)
    if sec.get("experiment_pct") is not None:
        exp = f"실험{sec['experiment_pct']}%"
        if sec.get("experiment_detail"):
            exp += f" {sec['experiment_detail']}"
        parts.append(exp)
    if sec.get("physics_limit_count"):
        parts.append(f"물리한계{sec['physics_limit_count']}")
    if sec.get("tp_count") is not None:
        parts.append(f"TP{sec['tp_count']}")
    if sec.get("discovery_count") and sec["discovery_count"] != 0:
        parts.append(f"발견{sec['discovery_count']}")
    if sec.get("mk_count"):
        parts.append("Mk.V")
    return "> " + " | ".join(parts)


def _gen_footer_fallback(sec):
    """Fallback footer generator when footer_raw is not set."""
    parts = [f"[{d}/](docs/{d}/)" for d in sec.get("domains", [])]
    result = "> 도메인: " + " · ".join(parts)
    tools = sec.get("tools", [])
    if tools:
        result += " · 도구: " + " · ".join(f"`{t}`" for t in tools)
    return result


if __name__ == "__main__":
    main()
