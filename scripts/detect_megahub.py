#!/usr/bin/env python3
"""
H9 — 4-BT 메가노드 자동 탐색 (atlas.signals.n6)

기능:
- atlas.signals.n6 전 signal 파싱 → domain_tags 추출
- 밀레니엄 태그 (7R, 7N, 7H, 7P, 7Y, 7B, 7S) 4개 이상 동시 보유 signal 추출
- 기존 메가노드 (SIG-META-001 = emergence-312-meta-analysis) 와 비교
- 5-BT 이상 발견 시 [M10*] 후보 표시
- 리포트 생성: reports/megahub-detection-YYYY-MM-DD.md

실행:
  python3 scripts/detect_megahub.py
  python3 scripts/detect_megahub.py --threshold 5   # 5-BT 이상
"""
from __future__ import annotations
import argparse
import re
import sys
from datetime import date
from pathlib import Path
from collections import Counter

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
REPORT_DIR = Path("/Users/ghost/Dev/n6-architecture/reports")
MILL_TAGS = {"7R", "7N", "7H", "7P", "7Y", "7B", "7S"}
EXISTING_MEGAHUB = "SIG-META-001"


def parse_signals(path: Path):
    """atlas.signals.n6 파싱 → [{sig_id, statement, tags, grade, evidence, witness, line}]"""
    signals = []
    current = None
    with path.open("r", encoding="utf-8") as f:
        for lineno, raw in enumerate(f, 1):
            line = raw.rstrip("\n")
            m = re.match(r"^@S\s+(\S+)\s*=\s*(.+?)\s*::\s*signal\s*(.+)$", line)
            if m:
                if current:
                    signals.append(current)
                sig_id, stmt, trailer = m.group(1), m.group(2), m.group(3)
                tag_sets = re.findall(r"\[([^\]]+)\]", trailer)
                all_tags = set()
                grade = ""
                evidence = ""
                for ts in tag_sets:
                    items = [t.strip() for t in ts.split(",")]
                    for it in items:
                        if it in {"M10*", "M10", "M9", "M7!", "M7", "M?", "MN"}:
                            grade = it
                        elif re.match(r"^E[123CFP]$", it):
                            evidence = it
                        else:
                            all_tags.add(it)
                current = {
                    "sig_id": sig_id,
                    "statement": stmt,
                    "tags": all_tags,
                    "grade": grade,
                    "evidence": evidence,
                    "witness": 1,
                    "line": lineno,
                }
            elif current is not None:
                wm = re.match(r"^\s*witness:\s*(\d+)", line)
                if wm:
                    current["witness"] = int(wm.group(1))
        if current:
            signals.append(current)
    return signals


def count_mill(tags):
    return len(tags & MILL_TAGS)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--threshold", type=int, default=4,
                    help="밀레니엄 태그 최소 동시 보유 수 (기본 4)")
    ap.add_argument("--report", type=str, default=None,
                    help="리포트 출력 경로 (기본 자동)")
    args = ap.parse_args()

    if not SSOT.exists():
        print(f"ERROR: SSOT 없음 {SSOT}", file=sys.stderr)
        return 2

    signals = parse_signals(SSOT)
    print(f"전체 signal 파싱: {len(signals)}")

    # 밀레니엄 태그 분포
    mill_dist = Counter()
    for s in signals:
        k = count_mill(s["tags"])
        mill_dist[k] += 1

    # 메가노드 후보 추출 (≥ threshold)
    megahubs = []
    for s in signals:
        mk = count_mill(s["tags"])
        if mk >= args.threshold:
            s["mill_count"] = mk
            s["mill_tags"] = sorted(s["tags"] & MILL_TAGS)
            megahubs.append(s)
    megahubs.sort(key=lambda s: (-s["mill_count"], -s["witness"]))

    print(f"≥{args.threshold}-BT megahub 후보: {len(megahubs)}")

    # 리포트 작성
    today = date.today().isoformat()
    out_path = Path(args.report) if args.report else REPORT_DIR / f"megahub-detection-{today}.md"
    out_path.parent.mkdir(parents=True, exist_ok=True)

    with out_path.open("w", encoding="utf-8") as f:
        f.write(f"# Megahub 탐지 리포트 ({today})\n\n")
        f.write("> 이것이 증명이 아니라 atlas.signals.n6 기반 구조적 관찰임.\n\n")
        f.write(f"- SSOT: `{SSOT}`\n")
        f.write(f"- 전체 signal: **{len(signals)}**\n")
        f.write(f"- 밀레니엄 태그 집합: `{sorted(MILL_TAGS)}`\n")
        f.write(f"- 임계값(threshold): **{args.threshold}-BT 이상**\n")
        f.write(f"- 기존 메가노드: `{EXISTING_MEGAHUB}` (emergence-312-meta-analysis)\n\n")

        f.write("## 1. 밀레니엄 태그 분포\n\n")
        f.write("| 동시 보유 수 | signal 수 |\n|---:|---:|\n")
        for k in sorted(mill_dist.keys()):
            f.write(f"| {k} | {mill_dist[k]} |\n")
        f.write("\n")

        f.write(f"## 2. ≥{args.threshold}-BT 메가노드 후보 ({len(megahubs)}건)\n\n")
        if not megahubs:
            f.write("(없음)\n\n")
        else:
            f.write("| sig_id | 밀레니엄 태그 | #BT | grade | evidence | witness |\n")
            f.write("|---|---|---:|---|---|---:|\n")
            for s in megahubs:
                tag_s = ",".join(s["mill_tags"])
                f.write(f"| `{s['sig_id']}` | {tag_s} | {s['mill_count']} | "
                        f"{s['grade']} | {s['evidence']} | {s['witness']} |\n")
            f.write("\n")

        # 5-BT 이상 후보 (M10* 승격 대상)
        five_plus = [s for s in megahubs if s["mill_count"] >= 5]
        f.write(f"## 3. [M10*] 승격 후보 (≥5-BT)\n\n")
        if not five_plus:
            f.write("(없음 — 현재 4-BT 메가노드만 존재)\n\n")
        else:
            for s in five_plus:
                existing = "★ (기존)" if s["sig_id"] == EXISTING_MEGAHUB else "(신규)"
                f.write(f"### {s['sig_id']} {existing}\n\n")
                f.write(f"- 밀레니엄 태그: `{','.join(s['mill_tags'])}` ({s['mill_count']}-BT)\n")
                f.write(f"- grade/evidence: {s['grade']} / {s['evidence']}\n")
                f.write(f"- witness: {s['witness']}\n")
                f.write(f"- statement: {s['statement']}\n\n")

        # 신규/기존 비교
        existing_sig = [s for s in megahubs if s["sig_id"] == EXISTING_MEGAHUB]
        new_sigs = [s for s in megahubs if s["sig_id"] != EXISTING_MEGAHUB]
        f.write("## 4. 비교 (기존 vs 신규)\n\n")
        f.write(f"- 기존 메가노드 검출 여부: {'성공' if existing_sig else '실패'} "
                f"(`{EXISTING_MEGAHUB}`)\n")
        if existing_sig:
            f.write(f"  - {EXISTING_MEGAHUB}: {existing_sig[0]['mill_count']}-BT\n")
        f.write(f"- 신규 4-BT 이상 후보: {len(new_sigs)}건\n")
        if new_sigs:
            f.write("  (아래 후보는 추가 검증 필요 — 자동 승격 금지)\n")
            for s in new_sigs:
                f.write(f"  - `{s['sig_id']}` ({s['mill_count']}-BT): "
                        f"{','.join(s['mill_tags'])}\n")
        f.write("\n")

        f.write("## 5. 다음 단계 권장\n\n")
        f.write("- 신규 후보는 staging 경유 후 witness ≥ 3 + cross_repo ≥ 1 확인 후 [M10*] 승격\n")
        f.write("- 승격 스크립트: `scripts/promote_signal_to_atlas.py --dry-run`\n")
        f.write("- 현재 atlas.signals.n6 는 millennium 단일 영역 우세 — AN/CROSS 리포 수집 확대 필요\n")

    print(f"리포트 작성: {out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
