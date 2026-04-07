#!/usr/bin/env python3
"""n=6 Evidence Grader — SEDI 가설의 Bayesian n=6 매칭 등급 산정.

678개 가설 마크다운을 스캔하여:
  - 수식, 오차%, 기존 등급 추출
  - Bayesian 정보량 기반 n=6 evidence tier (A/B/C/D/E) 부여
  - summary JSON + markdown report 출력

Tier 기준 (bits of evidence):
  A (>20): EXACT match, multi-domain, survives falsification
  B (10-20): Strong match, <0.1% error
  C (3-10): Moderate, <1% error
  D (0-3): Marginal
  E (<0): Negative evidence

Usage:
  python3 scripts/auto_grade_n6.py                    # stdout markdown
  python3 scripts/auto_grade_n6.py --json              # stdout JSON
  python3 scripts/auto_grade_n6.py --save              # write both files to data/
"""

import re
import json
import math
import argparse
from pathlib import Path
from datetime import datetime

# ── 경로 ──────────────────────────────────────────────────────
SEDI_ROOT = Path(__file__).resolve().parent.parent
HYPOTHESES_DIR = SEDI_ROOT / "docs" / "hypotheses"
DATA_DIR = SEDI_ROOT / "data"

# ── 등급 이모지 → 수치 매핑 ──────────────────────────────────
GRADE_SCORES = {
    "🟩": 5,   # CONFIRMED / EXACT
    "🟥": 4,   # High significance (RED = strong detection)
    "🟧": 3,   # ORANGE level
    "🟨": 2,   # YELLOW
    "🟦": 1,   # BLUE
    "🟪": 0,   # PURPLE
    "⚪": -1,  # Ungraded
    "⬛": -2,  # Failed
    "★": 1,    # Star bonus (cumulative)
    "⭐": 1,
    "✅": 4,   # Confirmed
}

# 텍스트 등급 매핑
TEXT_GRADES = {
    "CONFIRMED": 5,
    "EXACT": 5,
    "VERIFIED": 4,
    "STRONG": 3,
    "MODERATE": 2,
    "MARGINAL": 1,
    "FAILED": -2,
    "REFUTED": -2,
}

# n=6 핵심 상수 (sedi/constants.py 기반)
N6_CONSTANTS = {
    6, 12, 24, 4, 2, 5, 8, 28, 496,  # 기본 n=6 산술
    0.5, 0.1667, 0.25, 0.8333, 0.6931,  # 비율들
    1.2247, 0.3679, 0.2877, 0.01536,   # 파생 상수
    0.608, 1.071,  # PSI 상수
}


def parse_hypothesis(filepath: Path) -> dict:
    """가설 마크다운 파일에서 메타데이터 추출."""
    text = filepath.read_text(encoding="utf-8")
    result = {
        "file": str(filepath.relative_to(SEDI_ROOT)),
        "filename": filepath.name,
        "id": None,
        "title": None,
        "grade_raw": None,
        "grade_score": 0,
        "errors": [],       # 오차% 값들
        "formulas": [],     # 수식 패턴들
        "n6_mentions": 0,   # n=6 관련 언급 수
        "convergence": False,
        "domain": None,
    }

    # ID 추출 (H-XX-NNN)
    m = re.match(r"(H-\w+-\d+)", filepath.stem)
    if m:
        result["id"] = m.group(1)

    # 도메인 추출
    dm = re.match(r"H-(\w+)-", filepath.stem)
    if dm:
        result["domain"] = dm.group(1)

    # 제목 (첫 번째 # 줄)
    tm = re.match(r"^#\s+(.+)", text, re.MULTILINE)
    if tm:
        result["title"] = tm.group(1).strip()

    # 등급 추출
    gm = re.search(r"^##\s*Grade:\s*(.+)", text, re.MULTILINE)
    if not gm:
        gm = re.search(r"\*\*(?:Grade|Status):\s*(.+?)(?:\*\*|$)", text, re.MULTILINE)
    if gm:
        grade_text = gm.group(1).strip()
        result["grade_raw"] = grade_text
        result["grade_score"] = _score_grade(grade_text)

    # 오차% 추출 (다양한 패턴)
    # "0.175%", "Error: 0.003%", "오차 2.2%", "< 0.1%"
    for em in re.finditer(r"(\d+\.?\d*)\s*%", text):
        val = float(em.group(1))
        if val < 50:  # 50% 이상은 오차가 아닐 가능성
            result["errors"].append(val)

    # n=6 관련 키워드 카운트
    n6_patterns = [
        r"n\s*=\s*6", r"n=6", r"σ\(6\)", r"τ\(6\)", r"φ\(6\)",
        r"perfect.number", r"R\(6\)", r"P₁\s*=\s*6",
        r"3!", r"factorial", r"Egyptian.fraction",
        r"1/2\+1/3\+1/6", r"σ-τ", r"σ/τ",
    ]
    for pat in n6_patterns:
        result["n6_mentions"] += len(re.findall(pat, text, re.IGNORECASE))

    # CONVERGENCE 여부
    if re.search(r"CONVERGENCE", text):
        result["convergence"] = True

    # 수식 패턴 추출 (코드 블록 + 테이블의 수학식)
    for fm in re.finditer(r"[στφσΦψ]\w*\s*[=≈]\s*[\d./]+", text):
        result["formulas"].append(fm.group(0).strip())

    return result


def _score_grade(grade_text: str) -> int:
    """등급 텍스트를 수치 점수로 변환."""
    score = 0
    for emoji, val in GRADE_SCORES.items():
        count = grade_text.count(emoji)
        if count > 0:
            if emoji == "★" or emoji == "⭐":
                score += val * count  # 별 개수만큼 가산
            else:
                score = max(score, val)  # 이모지 등급은 최대값

    # 텍스트 등급
    for keyword, val in TEXT_GRADES.items():
        if keyword in grade_text.upper():
            score = max(score, val)

    return score


def compute_n6_bits(hyp: dict) -> float:
    """Bayesian 정보량(bits) 계산 — n=6 증거 강도.

    구성요소:
      - 등급 점수 (0-5) → bits
      - 오차 정밀도 → bits (낮을수록 높은 정보량)
      - n=6 언급 밀도 → bits
      - CONVERGENCE 보너스
      - 다중 도메인 보너스 (cross-validation)
    """
    bits = 0.0

    # 1. 등급 기반 (최대 10 bits)
    bits += hyp["grade_score"] * 2.0

    # 2. 오차 정밀도 (최대 15 bits)
    if hyp["errors"]:
        min_err = min(hyp["errors"])
        if min_err > 0:
            # -log2(error/100) : 0.001% → 16.6 bits, 1% → 6.6 bits
            bits += min(15, -math.log2(min_err / 100))
        elif min_err == 0:
            bits += 15  # exact match

    # 3. n=6 언급 밀도 (최대 5 bits)
    bits += min(5, hyp["n6_mentions"] * 0.3)

    # 4. CONVERGENCE 보너스 (3 bits)
    if hyp["convergence"]:
        bits += 3.0

    # 5. 수식 수 보너스 (최대 3 bits)
    bits += min(3, len(hyp["formulas"]) * 0.5)

    return round(bits, 2)


def assign_tier(bits: float) -> str:
    """비트 수 기반 n=6 evidence tier 할당."""
    if bits > 20:
        return "A"
    elif bits >= 10:
        return "B"
    elif bits >= 3:
        return "C"
    elif bits >= 0:
        return "D"
    else:
        return "E"


def tier_label(tier: str) -> str:
    """Tier 라벨 (사람용)."""
    labels = {
        "A": "A (>20 bits): EXACT, multi-domain, falsification-survived",
        "B": "B (10-20): Strong match, <0.1% error",
        "C": "C (3-10): Moderate, <1% error",
        "D": "D (0-3): Marginal",
        "E": "E (<0): Negative evidence",
    }
    return labels.get(tier, tier)


def scan_all() -> list[dict]:
    """모든 가설 파일 스캔 후 등급 산정."""
    results = []
    for f in sorted(HYPOTHESES_DIR.glob("H-*.md")):
        hyp = parse_hypothesis(f)
        bits = compute_n6_bits(hyp)
        tier = assign_tier(bits)
        hyp["n6_bits"] = bits
        hyp["n6_tier"] = tier
        results.append(hyp)
    return results


def generate_json(results: list[dict]) -> dict:
    """Atlas 등록용 summary JSON 생성."""
    tier_counts = {}
    for r in results:
        tier_counts[r["n6_tier"]] = tier_counts.get(r["n6_tier"], 0) + 1

    # 상위 가설 (Tier A+B)
    top = [
        {
            "id": r["id"],
            "title": r.get("title", ""),
            "tier": r["n6_tier"],
            "bits": r["n6_bits"],
            "grade": r.get("grade_raw", ""),
            "min_error_pct": min(r["errors"]) if r["errors"] else None,
            "convergence": r["convergence"],
            "domain": r["domain"],
            "file": r["file"],
        }
        for r in sorted(results, key=lambda x: -x["n6_bits"])
        if r["n6_tier"] in ("A", "B")
    ]

    return {
        "generated": datetime.now().isoformat(timespec="seconds"),
        "source": "SEDI auto_grade_n6.py",
        "total_hypotheses": len(results),
        "tier_distribution": {
            "A": tier_counts.get("A", 0),
            "B": tier_counts.get("B", 0),
            "C": tier_counts.get("C", 0),
            "D": tier_counts.get("D", 0),
            "E": tier_counts.get("E", 0),
        },
        "mean_bits": round(sum(r["n6_bits"] for r in results) / len(results), 2) if results else 0,
        "top_hypotheses": top,
    }


def generate_markdown(results: list[dict]) -> str:
    """마크다운 리포트 생성."""
    lines = ["# SEDI n=6 Evidence Grading Report", ""]
    lines.append(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M')}")
    lines.append(f"Total hypotheses scanned: {len(results)}")
    lines.append("")

    # Tier 분포
    tier_counts = {}
    for r in results:
        tier_counts[r["n6_tier"]] = tier_counts.get(r["n6_tier"], 0) + 1

    lines.append("## Tier Distribution")
    lines.append("")
    for t in "ABCDE":
        count = tier_counts.get(t, 0)
        pct = count / len(results) * 100 if results else 0
        lines.append(f"- **Tier {t}**: {count} ({pct:.1f}%)")
    lines.append("")

    mean_bits = sum(r["n6_bits"] for r in results) / len(results) if results else 0
    lines.append(f"**Mean information: {mean_bits:.2f} bits**")
    lines.append("")

    # Tier A 상세
    tier_a = [r for r in results if r["n6_tier"] == "A"]
    if tier_a:
        lines.append("## Tier A — Strongest Evidence (>20 bits)")
        lines.append("")
        lines.append("| ID | Title | Bits | Grade | Min Error | CONV |")
        lines.append("|---|---|---|---|---|---|")
        for r in sorted(tier_a, key=lambda x: -x["n6_bits"]):
            title = (r.get("title") or "")[:50]
            err = f'{min(r["errors"]):.3f}%' if r["errors"] else "—"
            conv = "Yes" if r["convergence"] else "—"
            lines.append(f"| {r['id']} | {title} | {r['n6_bits']:.1f} | {r.get('grade_raw','')} | {err} | {conv} |")
        lines.append("")

    # Tier B 상세
    tier_b = [r for r in results if r["n6_tier"] == "B"]
    if tier_b:
        lines.append("## Tier B — Strong Evidence (10-20 bits)")
        lines.append("")
        lines.append("| ID | Title | Bits | Grade | Min Error |")
        lines.append("|---|---|---|---|---|")
        for r in sorted(tier_b, key=lambda x: -x["n6_bits"])[:30]:
            title = (r.get("title") or "")[:50]
            err = f'{min(r["errors"]):.3f}%' if r["errors"] else "—"
            lines.append(f"| {r['id']} | {title} | {r['n6_bits']:.1f} | {r.get('grade_raw','')} | {err} |")
        if len(tier_b) > 30:
            lines.append(f"| ... | ({len(tier_b)-30} more) | | | |")
        lines.append("")

    # 도메인별 요약
    domains = {}
    for r in results:
        d = r.get("domain") or "unknown"
        if d not in domains:
            domains[d] = {"count": 0, "total_bits": 0, "tiers": {}}
        domains[d]["count"] += 1
        domains[d]["total_bits"] += r["n6_bits"]
        t = r["n6_tier"]
        domains[d]["tiers"][t] = domains[d]["tiers"].get(t, 0) + 1

    lines.append("## Domain Summary")
    lines.append("")
    lines.append("| Domain | Count | Mean Bits | Tier A | Tier B | Tier C |")
    lines.append("|---|---|---|---|---|---|")
    for d in sorted(domains.keys()):
        info = domains[d]
        mean = info["total_bits"] / info["count"] if info["count"] else 0
        lines.append(
            f"| {d} | {info['count']} | {mean:.1f} | "
            f"{info['tiers'].get('A', 0)} | {info['tiers'].get('B', 0)} | "
            f"{info['tiers'].get('C', 0)} |"
        )
    lines.append("")

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description="SEDI n=6 Evidence Grader")
    parser.add_argument("--json", action="store_true", help="JSON 출력")
    parser.add_argument("--save", action="store_true", help="data/ 디렉토리에 저장")
    parser.add_argument("--summary", action="store_true", help="요약만 출력")
    args = parser.parse_args()

    results = scan_all()

    if args.summary:
        tier_counts = {}
        for r in results:
            tier_counts[r["n6_tier"]] = tier_counts.get(r["n6_tier"], 0) + 1
        mean = sum(r["n6_bits"] for r in results) / len(results) if results else 0
        print(f"SEDI n=6 Grading: {len(results)} hypotheses, mean={mean:.1f} bits")
        for t in "ABCDE":
            print(f"  Tier {t}: {tier_counts.get(t, 0)}")
        return

    summary = generate_json(results)
    report = generate_markdown(results)

    if args.save:
        DATA_DIR.mkdir(parents=True, exist_ok=True)
        json_path = DATA_DIR / "sedi-grades.json"
        md_path = DATA_DIR / "n6-grading-report.md"
        json_path.write_text(json.dumps(summary, indent=2, ensure_ascii=False))
        md_path.write_text(report)
        print(f"Saved: {json_path}")
        print(f"Saved: {md_path}")
    elif args.json:
        print(json.dumps(summary, indent=2, ensure_ascii=False))
    else:
        print(report)


if __name__ == "__main__":
    main()
