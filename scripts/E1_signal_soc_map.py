#!/usr/bin/env python3
# E1_signal_soc_map.py — 385 signal self-organizing criticality (SOC) map
#
# 목적: atlas.signals.n6 의 신호 분포가 self-organizing criticality 패턴을 보이는지
#       (power-law tail, criticality threshold) 측정.
#
# 측정:
#   1. axis tag 빈도 분포 → log-log fit → slope α (Zipf-like)
#   2. domain tag 빈도 분포 → 동일
#   3. cross_repo edge 분포 → 동일
#   4. grade 분포 → 평형 vs 임계
#
# 사용:
#   python3 scripts/E1_signal_soc_map.py
#
# 산출:
#   reports/E1_signal_soc_map_20260415.md
#   reports/E1_signal_soc_data.json
#
# 정직: 본 스크립트는 단순 통계 기술. SOC 판정은 정성적 (power-law fit p-value 미계산).

import json
import math
import re
from collections import Counter
from pathlib import Path

NEXUS = Path.home() / "Dev" / "nexus"
N6_ROOT = Path.home() / "Dev" / "n6-architecture"
SIGNALS = NEXUS / "shared/n6/atlas.signals.n6"
OUT_MD = N6_ROOT / "reports/E1_signal_soc_map_20260415.md"
OUT_JSON = N6_ROOT / "reports/E1_signal_soc_data.json"


def load_signals(path: Path):
    """signal 라인 파싱 — 각 신호의 (id, axis_tags, domain_tags, grade) 추출."""
    if not path.exists():
        return []
    text = path.read_text(encoding="utf-8")
    lines = text.split("\n")
    signals = []
    sig_re = re.compile(r"^@S\s+(SIG-[A-Z0-9-]+)\s*=\s*.*?::\s*signal\s+\[([^\]]+)\]\s+\[([^\]]+)\]\s+\[([^\]]+)\]\s+\[([^\]]+)\]")
    for line in lines:
        m = sig_re.match(line)
        if not m:
            continue
        sid, repo_tags, domain_tags, grade, evid = m.groups()
        signals.append({
            "id": sid,
            "repo": [t.strip() for t in repo_tags.split(",")],
            "domains": [t.strip() for t in domain_tags.split(",")],
            "grade": grade.strip(),
            "evidence": evid.strip(),
        })
    return signals


def power_law_slope(counter, min_count=2):
    """Zipf-like log-log fit slope. 단순 OLS over (rank, count)."""
    items = sorted([c for c in counter.values() if c >= min_count], reverse=True)
    n = len(items)
    if n < 3:
        return None, n
    xs = [math.log(r + 1) for r in range(n)]
    ys = [math.log(c) for c in items]
    mean_x = sum(xs) / n
    mean_y = sum(ys) / n
    num = sum((xs[i] - mean_x) * (ys[i] - mean_y) for i in range(n))
    den = sum((xs[i] - mean_x) ** 2 for i in range(n))
    if den == 0:
        return None, n
    slope = num / den
    return slope, n


def main():
    signals = load_signals(SIGNALS)
    n_total = len(signals)

    # 분포 1: domain tag 빈도
    dom_counter = Counter()
    for s in signals:
        for d in s["domains"]:
            dom_counter[d] += 1

    # 분포 2: repo tag
    repo_counter = Counter()
    for s in signals:
        for r in s["repo"]:
            repo_counter[r] += 1

    # 분포 3: grade
    grade_counter = Counter(s["grade"] for s in signals)

    # 분포 4: id prefix (SIG-XXX-NNN 의 XXX) 식별 — 도메인 가족
    prefix_counter = Counter()
    pre_re = re.compile(r"^SIG-([A-Z0-9]+)-")
    for s in signals:
        m = pre_re.match(s["id"])
        if m:
            prefix_counter[m.group(1)] += 1

    # power-law fit
    slope_dom, n_dom = power_law_slope(dom_counter)
    slope_pre, n_pre = power_law_slope(prefix_counter)

    # criticality 임계 추정 — top-k 가 전체의 X% 차지
    top_dom = dom_counter.most_common(5)
    top_dom_share = sum(c for _, c in top_dom) / max(sum(dom_counter.values()), 1)

    top_pre = prefix_counter.most_common(5)
    top_pre_share = sum(c for _, c in top_pre) / max(sum(prefix_counter.values()), 1)

    # SOC 판정 (정성적)
    soc_signal_dom = "POWER-LAW" if slope_dom and -2.5 < slope_dom < -0.5 else "FLAT-or-UNCLEAR"
    soc_signal_pre = "POWER-LAW" if slope_pre and -2.5 < slope_pre < -0.5 else "FLAT-or-UNCLEAR"

    # JSON 출력
    out_data = {
        "ts": "2026-04-15",
        "n_total_signals": n_total,
        "domain_distribution": {
            "n_unique": len(dom_counter),
            "top_5": top_dom,
            "top_5_share": top_dom_share,
            "log_log_slope": slope_dom,
            "n_fit": n_dom,
            "soc_signal": soc_signal_dom,
        },
        "id_prefix_distribution": {
            "n_unique": len(prefix_counter),
            "top_5": top_pre,
            "top_5_share": top_pre_share,
            "log_log_slope": slope_pre,
            "n_fit": n_pre,
            "soc_signal": soc_signal_pre,
        },
        "grade_distribution": dict(grade_counter),
        "repo_distribution": dict(repo_counter),
    }
    OUT_JSON.parent.mkdir(parents=True, exist_ok=True)
    OUT_JSON.write_text(json.dumps(out_data, ensure_ascii=False, indent=2))

    # MD 리포트
    md = []
    md.append("# E1 Signal SOC Map — 2026-04-15")
    md.append("")
    md.append(f"> 입력: `{SIGNALS}`")
    md.append(f"> 신호 총 수: {n_total}")
    md.append("> 분석: 단순 log-log slope (Zipf-like). SOC 판정은 정성적.")
    md.append("> 7대 난제 해결 0/7 유지.")
    md.append("")
    md.append("## 1. Domain tag 분포")
    md.append("")
    md.append(f"- 고유 domain tag 수: {len(dom_counter)}")
    md.append(f"- log-log slope: {slope_dom}")
    md.append(f"- 적합 데이터 수: {n_dom}")
    md.append(f"- top 5 점유율: {top_dom_share:.2%}")
    md.append(f"- SOC 시그널: **{soc_signal_dom}**")
    md.append("")
    md.append("| rank | domain | count |")
    md.append("|------|--------|-------|")
    for i, (d, c) in enumerate(dom_counter.most_common(15), 1):
        md.append(f"| {i} | {d} | {c} |")
    md.append("")
    md.append("## 2. ID prefix 분포 (도메인 가족)")
    md.append("")
    md.append(f"- 고유 prefix 수: {len(prefix_counter)}")
    md.append(f"- log-log slope: {slope_pre}")
    md.append(f"- 적합 데이터 수: {n_pre}")
    md.append(f"- top 5 점유율: {top_pre_share:.2%}")
    md.append(f"- SOC 시그널: **{soc_signal_pre}**")
    md.append("")
    md.append("| rank | prefix | count |")
    md.append("|------|--------|-------|")
    for i, (p, c) in enumerate(prefix_counter.most_common(15), 1):
        md.append(f"| {i} | SIG-{p}-* | {c} |")
    md.append("")
    md.append("## 3. Grade 분포")
    md.append("")
    md.append("| grade | count | share |")
    md.append("|-------|-------|-------|")
    total_g = sum(grade_counter.values())
    for g, c in sorted(grade_counter.items(), key=lambda x: -x[1]):
        md.append(f"| {g} | {c} | {c/max(total_g,1):.2%} |")
    md.append("")
    md.append("## 4. Repo 분포")
    md.append("")
    md.append("| repo | count |")
    md.append("|------|-------|")
    for r, c in repo_counter.most_common():
        md.append(f"| {r} | {c} |")
    md.append("")
    md.append("## 5. SOC 해석 (정성적)")
    md.append("")
    md.append("- **POWER-LAW**: log-log slope ∈ [-2.5, -0.5] 면 (Zipf-near) 임계 분포 후보.")
    md.append("- **FLAT-or-UNCLEAR**: 평탄 분포 또는 적합 부족 — sub-critical 또는 noise.")
    md.append(f"- domain tag 분포 SOC: **{soc_signal_dom}**")
    md.append(f"- ID prefix 분포 SOC: **{soc_signal_pre}**")
    md.append("")
    md.append("## 6. 정직 한계")
    md.append("")
    md.append("- log-log fit 의 R² 미산출. p-value 미계산.")
    md.append("- SOC 판정은 slope 범위만으로 정성. KS-test 또는 Clauset 2009 power-law fit 미수행.")
    md.append("- 본 분석은 atlas.signals.n6 의 self-citation 효과를 보정하지 않음.")
    md.append("- 7대 난제 0/7 유지.")

    OUT_MD.write_text("\n".join(md))
    print(f"E1 SOC map: total={n_total}")
    print(f"  domain slope = {slope_dom}, SOC = {soc_signal_dom}")
    print(f"  prefix slope = {slope_pre}, SOC = {soc_signal_pre}")
    print(f"  top domain share = {top_dom_share:.2%}")
    print(f"  top prefix share = {top_pre_share:.2%}")
    print(f"  -> {OUT_MD}")
    print(f"  -> {OUT_JSON}")


if __name__ == "__main__":
    main()
