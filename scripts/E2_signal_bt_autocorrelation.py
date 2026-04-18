#!/usr/bin/env python3
# E2_signal_bt_autocorrelation.py — signal 생성률 vs BT 흐름 autocorrelation
#
# 목적: signal 생성 시점 (discovered_at) 과 BT 정리 도입 시점 (theory/breakthroughs/bt-*-2026-*.md)
#       사이의 시계열 autocorrelation 측정.
#
# 측정:
#   1. signal discovered_at 분포 (월/주 단위 bin)
#   2. BT 파일명의 날짜 (bt-NNNN-...-2026-MM-DD.md 패턴)
#   3. 두 시계열의 lag-k cross-correlation
#
# 사용:
#   python3 scripts/E2_signal_bt_autocorrelation.py
#
# 산출:
#   reports/E2_signal_bt_autocorr_20260415.md
#   reports/E2_signal_bt_autocorr_data.json
#
# 정직: 기준 시점 데이터가 부족 (BT 파일 다수가 2026-04 집중) — 본 분석은 lag 분포 explorative.

import json
import re
from collections import Counter
from pathlib import Path
from datetime import datetime

NEXUS = Path.home() / "Dev" / "nexus"
N6_ROOT = Path.home() / "Dev" / "n6-architecture"
SIGNALS = NEXUS / "shared/n6/atlas.signals.n6"
BT_DIR = N6_ROOT / "theory/breakthroughs"
OUT_MD = N6_ROOT / "reports/E2_signal_bt_autocorr_20260415.md"
OUT_JSON = N6_ROOT / "reports/E2_signal_bt_autocorr_data.json"


def parse_signal_dates(path: Path):
    """signal 본문에서 discovered_at: YYYY-MM-DDT... 추출."""
    if not path.exists():
        return []
    text = path.read_text(encoding="utf-8")
    pat = re.compile(r"discovered_at:\s*(\d{4}-\d{2}-\d{2})")
    dates = pat.findall(text)
    return dates


def parse_bt_dates(bt_dir: Path):
    """BT 파일명에서 -YYYY-MM-DD.md 패턴 추출."""
    if not bt_dir.exists():
        return []
    pat = re.compile(r"-(\d{4})-(\d{2})-(\d{2})\.md$")
    out = []
    for p in bt_dir.iterdir():
        m = pat.search(p.name)
        if m:
            out.append(f"{m.group(1)}-{m.group(2)}-{m.group(3)}")
    return out


def date_to_week(s):
    try:
        d = datetime.strptime(s, "%Y-%m-%d")
        # ISO week start
        iso = d.isocalendar()
        return f"{iso[0]}-W{iso[1]:02d}"
    except Exception:
        return None


def autocorr_lag(seq_a, seq_b, lag):
    """간단 cross-correlation @ lag — 정규화된 cross-product 합."""
    if len(seq_a) < 2 or len(seq_b) < 2:
        return None
    if lag >= 0:
        a = seq_a[:len(seq_a) - lag]
        b = seq_b[lag:lag + len(a)]
    else:
        a = seq_a[-lag:]
        b = seq_b[:len(a)]
    n = min(len(a), len(b))
    if n < 2:
        return None
    a = a[:n]
    b = b[:n]
    mean_a = sum(a) / n
    mean_b = sum(b) / n
    num = sum((a[i] - mean_a) * (b[i] - mean_b) for i in range(n))
    var_a = sum((x - mean_a) ** 2 for x in a)
    var_b = sum((x - mean_b) ** 2 for x in b)
    if var_a == 0 or var_b == 0:
        return None
    den = (var_a * var_b) ** 0.5
    return num / den


def main():
    sig_dates = parse_signal_dates(SIGNALS)
    bt_dates = parse_bt_dates(BT_DIR)

    sig_weeks = [date_to_week(d) for d in sig_dates]
    bt_weeks = [date_to_week(d) for d in bt_dates]
    sig_weeks = [w for w in sig_weeks if w]
    bt_weeks = [w for w in bt_weeks if w]

    sig_counter = Counter(sig_weeks)
    bt_counter = Counter(bt_weeks)

    all_weeks = sorted(set(list(sig_counter.keys()) + list(bt_counter.keys())))
    sig_seq = [sig_counter.get(w, 0) for w in all_weeks]
    bt_seq = [bt_counter.get(w, 0) for w in all_weeks]

    # cross-correlation lag -3 ~ +3
    lags = list(range(-3, 4))
    corrs = {lag: autocorr_lag(sig_seq, bt_seq, lag) for lag in lags}

    out_data = {
        "ts": "2026-04-15",
        "n_signals_with_date": len(sig_dates),
        "n_bt_files_with_date": len(bt_dates),
        "n_weeks": len(all_weeks),
        "weeks": all_weeks,
        "signal_per_week": sig_seq,
        "bt_per_week": bt_seq,
        "cross_correlation_lag": {str(k): v for k, v in corrs.items()},
    }
    OUT_JSON.parent.mkdir(parents=True, exist_ok=True)
    OUT_JSON.write_text(json.dumps(out_data, ensure_ascii=False, indent=2))

    md = []
    md.append("# E2 Signal vs BT Autocorrelation — 2026-04-15")
    md.append("")
    md.append(f"> 입력 signals: `{SIGNALS}` ({len(sig_dates)}건)")
    md.append(f"> 입력 BT 파일: `{BT_DIR}` ({len(bt_dates)}건)")
    md.append(f"> 주 단위 bin: {len(all_weeks)} 주")
    md.append("> 정직: BT 도입 시점이 2026-04 에 집중 — autocorr 단편적.")
    md.append("> 7대 난제 해결 0/7 유지.")
    md.append("")
    md.append("## 1. 주별 분포")
    md.append("")
    md.append("| 주 | signal | BT |")
    md.append("|----|-------:|----:|")
    for i, w in enumerate(all_weeks):
        md.append(f"| {w} | {sig_seq[i]} | {bt_seq[i]} |")
    md.append("")
    md.append("## 2. cross-correlation @ lag")
    md.append("")
    md.append("| lag (주) | r | 해석 |")
    md.append("|---------|---|------|")
    for lag in lags:
        r = corrs[lag]
        if r is None:
            interp = "data 부족"
        elif abs(r) > 0.7:
            interp = "STRONG"
        elif abs(r) > 0.4:
            interp = "MID"
        elif abs(r) > 0.2:
            interp = "WEAK"
        else:
            interp = "NONE"
        rs = f"{r:.3f}" if r is not None else "—"
        md.append(f"| {lag:+d} | {rs} | {interp} |")
    md.append("")
    md.append("## 3. 해석")
    md.append("")
    md.append("- lag = 0: 동시 주에서 signal 과 BT 의 동조성 (r > 0 = 함께 증가)")
    md.append("- lag > 0: signal 이 BT 를 **선행** (signal → BT 도입)")
    md.append("- lag < 0: BT 가 signal 을 **선행** (BT → signal 추가)")
    md.append("")
    md.append("## 4. 정직 한계")
    md.append("")
    md.append("- 데이터 기간이 짧음 (대부분 2026-04). 주 단위 bin 수 한정.")
    md.append("- BT 파일명 날짜는 commit 시점이 아닌 파일명 인코딩 — 실제 발견일과 차이 가능.")
    md.append("- signal discovered_at 도 다수가 2026-04-15 1일 집중 — autocorr 신뢰도 낮음.")
    md.append("- 본 분석은 explorative. 통계적 신뢰구간 미산출.")
    md.append("- 7대 난제 0/7 유지.")

    OUT_MD.write_text("\n".join(md))
    print(f"E2 autocorr: signals={len(sig_dates)} BT={len(bt_dates)} weeks={len(all_weeks)}")
    for lag in lags:
        r = corrs[lag]
        rs = f"{r:.3f}" if r is not None else "—"
        print(f"  lag={lag:+d}  r={rs}")
    print(f"  -> {OUT_MD}")
    print(f"  -> {OUT_JSON}")


if __name__ == "__main__":
    main()
