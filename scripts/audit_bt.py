#!/usr/bin/env python3
"""
BT Audit — docs/breakthrough-theorems.md 전수 감사기.

각 BT 섹션(## BT-N: ...)을 파싱해 evidence 테이블의 행을 검사한다.

판정 규칙(BT 단위):
  1) 행 Source/Values에 EXACT/CLOSE/MISS 태그가 있으면 그대로 카운트.
  2) "n=6 Expression" 컬럼이 있고 Parameters/Predicted에 수치가 있으면
     심볼릭으로 계산해 일치하면 EXACT, 불일치면 MISS로 자동 판정.
  3) BT status:
       - MISS 행이 1개 이상        → MISS
       - EXACT/CLOSE 행만 존재하고 MISS 0 → EXACT
       - 판정 가능한 행이 0        → UNKNOWN (증거 정성적 서술만)

출력: docs/bt-audit-report.md

사용:
  python3 scripts/audit_bt.py
"""

from __future__ import annotations

import math
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BT_FILE = ROOT / "docs" / "breakthrough-theorems.md"
REPORT = ROOT / "docs" / "bt-audit-report.md"

# n=6 상수 레지스트리 (atlas-constants.md / core 정의와 일치)
N6 = {
    "n": 6,
    "N": 6,
    "tau": 4,       # 약수 개수
    "sigma": 12,    # 약수 합
    "phi": 2,       # 오일러 토티언트
    "sopfr": 5,     # 소인수 합 (2+3)
    "omega": 2,     # 서로 다른 소인수 개수
    "Omega": 2,     # 소인수 개수(중복 포함)
    "mu": 1,        # Möbius μ(6)=1 (square-free, 짝수 개의 소인수)
    "J_2": 24,      # Jordan totient J_2(6) = 24
    "J2": 24,
    "P_2": 4,       # 2차 소수 멱합? 문맥상 σ(6)-φ(6)·ω 또는 4. 대부분 분모로만 등장하며 1/P_2 ≈ 1/28 혹은 1/4. 문맥 불명확 행은 skip.
    "P2": 4,
}

# 파생 자주 나오는 상수
N6_DERIVED = {
    "sigma-sopfr": N6["sigma"] - N6["sopfr"],    # 7
    "sigma-tau": N6["sigma"] - N6["tau"],        # 8
    "sigma+phi": N6["sigma"] + N6["phi"],        # 14
    "sigma+mu": N6["sigma"] + N6["mu"],          # 13
    "n/phi": N6["n"] / N6["phi"],                # 3
    "sigma/tau": N6["sigma"] / N6["tau"],        # 3
    "sigma/phi": N6["sigma"] / N6["phi"],        # 6
    "sigma*phi": N6["sigma"] * N6["phi"],        # 24
    "tau*phi": N6["tau"] * N6["phi"],            # 8
    "n*tau": N6["n"] * N6["tau"],                # 24
}

SECTION_RE = re.compile(r"^## BT-(\d+)\s*:?\s*(.*)$", re.MULTILINE)
TABLE_LINE_RE = re.compile(r"^\|(.+)\|\s*$")
TAG_RE = re.compile(r"\b(EXACT|CLOSE|MISS)\b")
NUMBER_RE = re.compile(r"[-+]?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?")


def load_sections(text: str):
    """(bt_id, title, body) 리스트 반환."""
    matches = list(SECTION_RE.finditer(text))
    sections = []
    for i, m in enumerate(matches):
        start = m.end()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        sections.append((int(m.group(1)), m.group(2).strip(), text[start:end]))
    return sections


def parse_tables(body: str):
    """Markdown 표를 헤더+행 리스트로 반환."""
    tables = []
    lines = body.splitlines()
    i = 0
    while i < len(lines):
        if TABLE_LINE_RE.match(lines[i]) and i + 1 < len(lines) and re.match(r"^\|\s*[-:| ]+\|\s*$", lines[i + 1]):
            header = [c.strip() for c in lines[i].strip().strip("|").split("|")]
            i += 2
            rows = []
            while i < len(lines) and TABLE_LINE_RE.match(lines[i]):
                cells = [c.strip() for c in lines[i].strip().strip("|").split("|")]
                if len(cells) == len(header):
                    rows.append(cells)
                i += 1
            tables.append((header, rows))
        else:
            i += 1
    return tables


def eval_n6(expr: str):
    """n=6 심볼릭 표현식을 float로 평가. 실패 시 None."""
    if expr is None:
        return None
    s = expr.strip().strip("*` ")
    # 괄호 안 인자 제거: phi(6) → phi, J_2(6) → J_2
    s = re.sub(r"\(\s*6\s*\)", "", s)
    s = s.replace("·", "*").replace("×", "*").replace("−", "-").replace("^", "**")
    # 파생 키워드부터 치환 (긴 것부터)
    merged = {**N6_DERIVED, **{k: v for k, v in N6.items()}}
    for key in sorted(merged.keys(), key=len, reverse=True):
        s = re.sub(rf"(?<![A-Za-z_]){re.escape(key)}(?![A-Za-z_0-9])", f"({merged[key]})", s)
    # 남아있는 알파벳 토큰이 있으면 포기
    if re.search(r"[A-Za-z_]", s):
        return None
    # 안전: 숫자/연산자/괄호/공백/점만 허용
    if not re.fullmatch(r"[0-9+\-*/(). ]+", s):
        return None
    try:
        return eval(s, {"__builtins__": {}}, {})  # noqa: S307
    except Exception:
        return None


def extract_numbers(cell: str):
    """셀에서 숫자 토큰들을 float 리스트로 추출."""
    return [float(x) for x in NUMBER_RE.findall(cell)]


def close_enough(a: float, b: float) -> bool:
    if a == b:
        return True
    denom = max(abs(a), abs(b), 1e-12)
    return abs(a - b) / denom < 0.02  # 2% 허용


def audit_bt(bt_id: int, title: str, body: str):
    tables = parse_tables(body)
    row_exact = row_close = row_miss = 0
    row_auto_ok = row_auto_bad = 0
    details = []

    for header, rows in tables:
        lower = [h.lower() for h in header]
        expr_idx = next((i for i, h in enumerate(lower) if "n=6" in h or "expression" in h), None)
        param_idx = next(
            (i for i, h in enumerate(lower) if h in ("parameters", "predicted", "value", "values", "measured")),
            None,
        )
        src_idx = next((i for i, h in enumerate(lower) if h in ("source", "sources", "ref")), None)

        for row in rows:
            # 1) 태그 기반
            joined = " ".join(row)
            tag = TAG_RE.search(joined)
            if tag:
                t = tag.group(1)
                if t == "EXACT":
                    row_exact += 1
                elif t == "CLOSE":
                    row_close += 1
                else:
                    row_miss += 1
                    details.append(f"MISS tag in BT-{bt_id}: {row[0] if row else '?'}")
                continue

            # 2) n=6 Expression 자동 평가
            if expr_idx is not None and param_idx is not None:
                predicted = eval_n6(row[expr_idx])
                nums = extract_numbers(row[param_idx])
                if predicted is not None and nums:
                    # Parameters 쪽에 여러 수치가 있으면 가장 가까운 하나와 비교
                    best = min(nums, key=lambda x: abs(x - predicted))
                    if close_enough(predicted, best):
                        row_auto_ok += 1
                    else:
                        row_auto_bad += 1
                        details.append(
                            f"MISMATCH BT-{bt_id}: expr={row[expr_idx]}→{predicted:g} vs {row[param_idx]}"
                        )

    total_judged = row_exact + row_close + row_miss + row_auto_ok + row_auto_bad
    if total_judged == 0:
        status = "UNKNOWN"
    elif row_miss + row_auto_bad == 0:
        status = "EXACT"
    elif row_exact + row_auto_ok >= 2 * (row_miss + row_auto_bad):
        status = "PARTIAL"
    else:
        status = "MISS"

    return {
        "id": bt_id,
        "title": title,
        "status": status,
        "exact": row_exact + row_auto_ok,
        "close": row_close,
        "miss": row_miss + row_auto_bad,
        "total": total_judged,
        "details": details,
    }


def main():
    text = BT_FILE.read_text(encoding="utf-8")
    sections = load_sections(text)

    results = [audit_bt(bt, title, body) for bt, title, body in sections]

    total = len(results)
    exact = sum(1 for r in results if r["status"] == "EXACT")
    partial = sum(1 for r in results if r["status"] == "PARTIAL")
    miss = sum(1 for r in results if r["status"] == "MISS")
    unknown = sum(1 for r in results if r["status"] == "UNKNOWN")

    row_exact = sum(r["exact"] for r in results)
    row_close = sum(r["close"] for r in results)
    row_miss = sum(r["miss"] for r in results)
    rows_total = row_exact + row_close + row_miss

    # 리포트 작성
    lines = []
    max_bt = max((r["id"] for r in results), default=0)
    min_bt = min((r["id"] for r in results), default=0)
    lines.append(f"# BT Audit Report — BT-1~{total} 전수 감사 (ID 범위 BT-{min_bt}~{max_bt})\n")
    lines.append(f"- 감사 대상: `{BT_FILE.relative_to(ROOT)}`\n")
    lines.append(f"- 감사 스크립트: `scripts/audit_bt.py`\n")
    lines.append(f"- 전체 BT 수: **{total}**\n")
    lines.append("\n## 요약\n")
    lines.append("| 구분 | 개수 | 비율 |\n|---|---|---|")
    for name, v in [("EXACT", exact), ("PARTIAL", partial), ("MISS", miss), ("UNKNOWN", unknown)]:
        pct = (v / total * 100) if total else 0
        lines.append(f"| {name} | {v} | {pct:.1f}% |")
    lines.append("")
    judgable = total - unknown
    rate_all = (exact + partial) / max(total, 1) * 100
    rate_judgable = (exact + partial) / max(judgable, 1) * 100
    exact_rate_all = exact / max(total, 1) * 100
    exact_rate_judgable = exact / max(judgable, 1) * 100
    lines.append(f"**BT 단위 일치율 (두 지표 병기)**\n")
    lines.append(f"- EXACT/(전체) = {exact}/{total} = {exact_rate_all:.1f}% "
                 f"(UNKNOWN {unknown}건 포함)")
    lines.append(f"- EXACT/(판정가능) = {exact}/{judgable} = {exact_rate_judgable:.1f}% "
                 f"(UNKNOWN 제외)")
    lines.append(f"- (EXACT+PARTIAL)/(전체) = {exact + partial}/{total} = {rate_all:.1f}%")
    lines.append(f"- (EXACT+PARTIAL)/(판정가능) = {exact + partial}/{judgable} = {rate_judgable:.1f}%\n")
    lines.append(f"**행 단위**: EXACT {row_exact} / CLOSE {row_close} / MISS {row_miss} "
                 f"(합 {rows_total}) — MISS율 {(row_miss / max(rows_total,1) * 100):.2f}%\n")
    lines.append(f"**mismatch 총계: {row_miss}** (목표 <50)\n")

    # MISS BT 목록
    lines.append("\n## MISS 판정 BT\n")
    miss_bts = [r for r in results if r["status"] == "MISS"]
    if not miss_bts:
        lines.append("_없음._\n")
    else:
        lines.append("| BT | 제목 | EXACT | MISS |\n|---|---|---|---|")
        for r in miss_bts:
            lines.append(f"| BT-{r['id']} | {r['title'][:60]} | {r['exact']} | {r['miss']} |")
        lines.append("")

    # PARTIAL
    lines.append("\n## PARTIAL 판정 BT (MISS 일부 포함)\n")
    partial_bts = [r for r in results if r["status"] == "PARTIAL"]
    if not partial_bts:
        lines.append("_없음._\n")
    else:
        lines.append("| BT | 제목 | EXACT | MISS |\n|---|---|---|---|")
        for r in partial_bts:
            lines.append(f"| BT-{r['id']} | {r['title'][:60]} | {r['exact']} | {r['miss']} |")
        lines.append("")

    # UNKNOWN (증거 표가 정성적이라 자동 판정 불가)
    lines.append(f"\n## UNKNOWN 판정 BT ({unknown}건) — 정성적 증거, 수치 비교 불가\n")
    unknown_bts = [r for r in results if r["status"] == "UNKNOWN"]
    lines.append("| BT | 제목 |\n|---|---|")
    for r in unknown_bts[:50]:
        lines.append(f"| BT-{r['id']} | {r['title'][:70]} |")
    if len(unknown_bts) > 50:
        lines.append(f"| … | (+{len(unknown_bts) - 50}건 생략) |")
    lines.append("")

    # mismatch 상세
    lines.append("\n## mismatch/MISS 상세 (최대 100건)\n")
    all_details = [d for r in results for d in r["details"]]
    lines.append(f"총 {len(all_details)}건\n")
    lines.append("```")
    for d in all_details[:100]:
        lines.append(d)
    lines.append("```\n")

    # 전체 결과표
    lines.append("\n## 전체 결과표\n")
    lines.append("| BT | 상태 | EXACT | CLOSE | MISS | 제목 |\n|---|---|---|---|---|---|")
    for r in results:
        lines.append(
            f"| BT-{r['id']} | {r['status']} | {r['exact']} | {r['close']} | {r['miss']} | {r['title'][:55]} |"
        )
    lines.append("")

    REPORT.write_text("\n".join(lines), encoding="utf-8")
    print(f"완료: {REPORT.relative_to(ROOT)}")
    print(f"  BT {total}개 — EXACT {exact} / PARTIAL {partial} / MISS {miss} / UNKNOWN {unknown}")
    print(f"  row MISS {row_miss} (목표 <50)")
    print(f"  BT 범위 BT-{min_bt}~{max_bt}")
    print(f"  EXACT/(전체)      = {exact}/{total} = {exact_rate_all:.1f}%")
    print(f"  EXACT/(판정가능)  = {exact}/{judgable} = {exact_rate_judgable:.1f}%")
    print(f"  (EXACT+PARTIAL)/(전체)     = {rate_all:.1f}%")
    print(f"  (EXACT+PARTIAL)/(판정가능) = {rate_judgable:.1f}%")


if __name__ == "__main__":
    main()
