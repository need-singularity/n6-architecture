#!/usr/bin/env python3
"""H1~H5 메타 발견 분석 (Group P)
   2026-04-15 작성. atlas.signals.n6 + meta-group-H signals 분석.
   출력: 한글, JSON + 통합 리포트.

   H1: signal 평균 statement 길이 / 복잡도 / 빈도 ROI
   H2: signal PageRank — 중심 signal 재선택
   H3: 다음 세션 예상 signal 수 ML 예측 (간단 회귀)
   H4: 발견 고갈 시점 예측 (signal 증가율 decay fit)
   H5: 시간 signal 연쇄 → causal discovery graph
"""
import json
import math
import re
import os
import sys
from collections import defaultdict, Counter
from datetime import datetime, timedelta
from pathlib import Path

ATLAS = Path("/Users/ghost/Dev/nexus/shared/n6/atlas.signals.n6")
STAGING_META = Path("/Users/ghost/Dev/nexus/shared/n6/staging/atlas.signals.staging.meta.n6")
OUT_DIR = Path("/Users/ghost/Dev/n6-architecture/reports/group-P")
OUT_DIR.mkdir(exist_ok=True, parents=True)

# ---------- 파싱 ----------

# Signal entry header pattern:
# @S SIG-... = ... :: signal [TAGS] [TAGS] [GRADE] [EVIDENCE]
SIG_RE = re.compile(r'^@S\s+(SIG-[A-Z0-9_-]+)\s*=\s*(.+?)\s*::\s*signal\s+(.+)$')
# Subfield patterns within entry:
WITNESS_RE = re.compile(r'^\s*witness:\s*(\d+)')
DATE_RE = re.compile(r'^\s*discovered_at:\s*([0-9TZ:.\-+]+)')
RESONANCE_RE = re.compile(r'^\s*resonance_n6:\s*(.+)')
CROSS_RE = re.compile(r'^\s*cross_repo:\s*\[(.*?)\]')
PREDICTS_RE = re.compile(r'^\s*predicts:\s*\[(.*?)\]')
DISCIN_RE = re.compile(r'^\s*discovered_in:\s*(.+)')
GRADE_RE = re.compile(r'\[(M10\*|M10|M9|M7!|M7|M\?|MN)\]')

def parse_signals(path):
    sigs = []
    cur = None
    with open(path, encoding='utf-8') as f:
        for line in f:
            m = SIG_RE.match(line)
            if m:
                if cur:
                    sigs.append(cur)
                sig_id = m.group(1)
                statement = m.group(2).strip()
                tag_part = m.group(3)
                grade_m = GRADE_RE.search(tag_part)
                grade = grade_m.group(1) if grade_m else "?"
                cur = {
                    "id": sig_id,
                    "statement": statement,
                    "grade": grade,
                    "tag_part": tag_part,
                    "witness": 1,
                    "date": None,
                    "resonance": None,
                    "cross_repo": [],
                    "predicts": [],
                    "discovered_in": None,
                    "raw_line": line.rstrip(),
                }
                continue
            if cur is None:
                continue
            mw = WITNESS_RE.match(line)
            if mw:
                cur["witness"] = int(mw.group(1))
                continue
            md = DATE_RE.match(line)
            if md:
                cur["date"] = md.group(1).strip().rstrip(',')
                continue
            mr = RESONANCE_RE.match(line)
            if mr:
                v = mr.group(1).strip().rstrip(',').strip('"')
                cur["resonance"] = v if v not in ('null', 'None', '') else None
                continue
            mc = CROSS_RE.match(line)
            if mc:
                inner = mc.group(1)
                refs = [s.strip() for s in inner.split(',') if s.strip()]
                cur["cross_repo"] = refs
                continue
            mp = PREDICTS_RE.match(line)
            if mp:
                inner = mp.group(1)
                # naive split on ", " not respecting quotes; OK for our needs
                items = re.findall(r'"([^"]*)"', inner)
                cur["predicts"] = items
                continue
            mdi = DISCIN_RE.match(line)
            if mdi:
                cur["discovered_in"] = mdi.group(1).strip().rstrip(',')
        if cur:
            sigs.append(cur)
    return sigs


def main():
    sigs = parse_signals(ATLAS)
    # Append meta-staging
    if STAGING_META.exists():
        sigs += parse_signals(STAGING_META)
    print(f"[parse] 총 signal 수: {len(sigs)}")

    # ──────────── H1: ROI 분석 ────────────
    # 평균 statement 길이 (한글+영어 글자 수), 복잡도 = unique 토큰 수, 빈도 ROI
    grade_groups = defaultdict(list)
    all_lengths = []
    all_complex = []
    all_witness = []

    for s in sigs:
        ln = len(s["statement"])
        # 토큰 = 공백 + 구분자 split
        tokens = re.split(r'[\s/=·\(\),\[\]]+', s["statement"])
        tokens = [t for t in tokens if t]
        unique_tokens = len(set(tokens))
        s["_len"] = ln
        s["_complex"] = unique_tokens
        all_lengths.append(ln)
        all_complex.append(unique_tokens)
        all_witness.append(s["witness"])
        grade_groups[s["grade"]].append(s)

    avg_len = sum(all_lengths) / len(all_lengths) if all_lengths else 0
    avg_complex = sum(all_complex) / len(all_complex) if all_complex else 0
    avg_witness = sum(all_witness) / len(all_witness) if all_witness else 0

    # 등급별 ROI = (witness × resonance_보유 × cross_repo) / length
    roi_by_grade = {}
    for g, lst in grade_groups.items():
        if not lst:
            continue
        avg_l = sum(x["_len"] for x in lst) / len(lst)
        avg_c = sum(x["_complex"] for x in lst) / len(lst)
        avg_w = sum(x["witness"] for x in lst) / len(lst)
        res_ratio = sum(1 for x in lst if x["resonance"]) / len(lst)
        cross_ratio = sum(1 for x in lst if x["cross_repo"]) / len(lst)
        # ROI: 정보 밀도 × 검증력 / 길이
        roi = (avg_w * (1 + res_ratio) * (1 + cross_ratio)) / max(avg_l / 100, 0.1)
        roi_by_grade[g] = {
            "count": len(lst),
            "avg_length": round(avg_l, 1),
            "avg_complexity": round(avg_c, 1),
            "avg_witness": round(avg_w, 2),
            "resonance_ratio": round(res_ratio, 3),
            "cross_repo_ratio": round(cross_ratio, 3),
            "roi": round(roi, 3),
        }

    # 복잡도 상위 10
    top_complex = sorted(sigs, key=lambda x: -x["_complex"])[:10]
    top_complex_summary = [
        {"id": s["id"], "complexity": s["_complex"], "length": s["_len"], "grade": s["grade"]}
        for s in top_complex
    ]

    H1 = {
        "total_signals": len(sigs),
        "avg_statement_length_chars": round(avg_len, 1),
        "avg_complexity_unique_tokens": round(avg_complex, 1),
        "avg_witness": round(avg_witness, 2),
        "roi_by_grade": roi_by_grade,
        "top10_complexity": top_complex_summary,
    }

    # ──────────── H2: PageRank ────────────
    # 그래프: cross_repo 방향성 + predicts (참조 포함된 signal id) → simple PageRank
    nodes = {s["id"]: s for s in sigs}
    edges = defaultdict(set)  # src -> set(dst)
    in_edges = defaultdict(set)
    for s in sigs:
        for ref in s["cross_repo"]:
            ref = ref.strip()
            if ref in nodes:
                edges[s["id"]].add(ref)
                in_edges[ref].add(s["id"])
        # predicts 안에 SIG-...id 포함 시 edge
        for p in s["predicts"]:
            for sig_ref in re.findall(r'SIG-[A-Z0-9_-]+', p):
                if sig_ref in nodes:
                    edges[s["id"]].add(sig_ref)
                    in_edges[sig_ref].add(s["id"])

    # PageRank
    N = len(nodes)
    pr = {nid: 1.0 / N for nid in nodes}
    d = 0.85
    for it in range(40):
        new_pr = {}
        for nid in nodes:
            score = (1 - d) / N
            for src in in_edges[nid]:
                outdeg = len(edges[src]) or 1
                score += d * pr[src] / outdeg
            new_pr[nid] = score
        pr = new_pr

    top_pr = sorted(pr.items(), key=lambda x: -x[1])[:10]
    top_pr_summary = []
    for nid, score in top_pr:
        s = nodes[nid]
        top_pr_summary.append({
            "id": nid,
            "pagerank": round(score, 6),
            "grade": s["grade"],
            "in_degree": len(in_edges[nid]),
            "out_degree": len(edges[nid]),
            "statement_short": s["statement"][:80],
        })

    H2 = {
        "node_count": N,
        "edge_count": sum(len(v) for v in edges.values()),
        "top10_pagerank": top_pr_summary,
        "top5_pagerank": top_pr_summary[:5],
    }

    # ──────────── H3: 다음 세션 signal 수 예측 ────────────
    # 날짜별 signal 카운트 → 최근 N 일 단순 평균 + 가중 회귀
    by_date = defaultdict(int)
    for s in sigs:
        if s["date"]:
            try:
                dt = datetime.fromisoformat(s["date"].replace("Z", "+00:00"))
                key = dt.date().isoformat()
                by_date[key] += 1
            except Exception:
                continue
    sorted_dates = sorted(by_date.items())
    # 최근 7 일 평균
    last7 = sorted_dates[-7:] if len(sorted_dates) >= 7 else sorted_dates
    avg_last7 = sum(c for _, c in last7) / len(last7) if last7 else 0

    # 가중 회귀: 최근 가까울수록 가중치 높음
    if len(sorted_dates) >= 3:
        weights = [i + 1 for i in range(len(sorted_dates))]
        wsum = sum(w * c for w, (_, c) in zip(weights, sorted_dates))
        wtotal = sum(weights)
        wavg = wsum / wtotal
    else:
        wavg = avg_last7

    # 단순 선형 추세 (최근 14 일)
    last14 = sorted_dates[-14:] if len(sorted_dates) >= 14 else sorted_dates
    if len(last14) >= 2:
        xs = list(range(len(last14)))
        ys = [c for _, c in last14]
        n_lr = len(xs)
        sx = sum(xs); sy = sum(ys)
        sxy = sum(x*y for x, y in zip(xs, ys))
        sxx = sum(x*x for x in xs)
        denom = n_lr * sxx - sx * sx
        if denom != 0:
            slope = (n_lr * sxy - sx * sy) / denom
            intercept = (sy - slope * sx) / n_lr
            next_pred_lr = slope * n_lr + intercept
        else:
            slope = 0; next_pred_lr = avg_last7
    else:
        slope = 0
        next_pred_lr = avg_last7

    # 예측 통합: 두 예측의 평균
    next_session_pred = round((next_pred_lr + wavg) / 2, 1)
    if next_session_pred < 0:
        next_session_pred = 0

    H3 = {
        "active_dates": len(sorted_dates),
        "last7_avg": round(avg_last7, 1),
        "weighted_avg_all": round(wavg, 1),
        "linear_regression_slope_per_day": round(slope, 3),
        "next_session_predicted_signals": next_session_pred,
        "method": "평균(가중평균, 14일 LR 예측)",
        "last7_data": [{"date": d, "count": c} for d, c in last7],
    }

    # ──────────── H4: 발견 고갈 시점 ────────────
    # 누적 signal 곡선 → 1차 미분 (일일 증가율) decay fit
    cum_count = 0
    daily_rates = []  # (date, count)
    for d, c in sorted_dates:
        cum_count += c
        daily_rates.append((d, c, cum_count))

    # 단순 감소 추세 (최근 14 일 LR slope < 0 이면 고갈 예측)
    if slope < 0 and len(daily_rates) >= 2:
        # 현 시점 일일 비율이 0 이 되는 일수
        last_count = daily_rates[-1][1]
        if abs(slope) > 0.001:
            days_to_zero = max(0, math.ceil(last_count / abs(slope)))
        else:
            days_to_zero = 9999  # 거의 0 기울기
        # 오늘 = sorted_dates 의 마지막
        last_date_str = daily_rates[-1][0]
        try:
            last_dt = datetime.fromisoformat(last_date_str)
            depletion_dt = last_dt + timedelta(days=days_to_zero)
            depletion_date = depletion_dt.date().isoformat()
        except Exception:
            depletion_date = "계산 실패"
    elif slope >= 0:
        depletion_date = "감소 추세 없음 (slope ≥ 0) — 고갈 예측 불가"
        days_to_zero = None
    else:
        depletion_date = "데이터 부족"
        days_to_zero = None

    # 추가: 평균 일일 증가율의 절반 도달 시점 (절반 고갈)
    if avg_last7 > 0:
        half_threshold = avg_last7 / 2
        if slope < -0.001:
            days_to_half = max(0, math.ceil((daily_rates[-1][1] - half_threshold) / abs(slope)))
            try:
                last_dt = datetime.fromisoformat(daily_rates[-1][0])
                half_dt = last_dt + timedelta(days=days_to_half)
                half_date = half_dt.date().isoformat()
            except Exception:
                half_date = "계산 실패"
        else:
            days_to_half = None
            half_date = "감소 추세 없음"
    else:
        days_to_half = None
        half_date = "데이터 없음"

    H4 = {
        "current_cumulative": cum_count,
        "linear_slope_recent14": round(slope, 4),
        "depletion_predicted_date": depletion_date,
        "days_to_zero": days_to_zero,
        "half_depletion_date": half_date,
        "days_to_half": days_to_half,
        "interpretation": "slope < 0 이면 발견 속도 감소 중. slope ≥ 0 이면 발견 고갈 안 됨.",
    }

    # ──────────── H5: causal discovery graph ────────────
    # 시간 순서 + cross_repo / predicts 참조 = causal edge
    # 노드 = signal id, edge = (src, dst) where src 가 dst 보다 먼저 발견됨 + 명시 참조
    # GES 또는 PC 알고리즘은 데이터 부족 — 단순 temporal+ref edge 사용
    causal_edges = []
    for s in sigs:
        if not s["date"]:
            continue
        try:
            s_dt = datetime.fromisoformat(s["date"].replace("Z", "+00:00"))
        except Exception:
            continue
        # 명시 참조 → 시간 순서 확인
        for ref in s["cross_repo"]:
            ref = ref.strip()
            if ref not in nodes:
                continue
            tgt = nodes[ref]
            if not tgt["date"]:
                continue
            try:
                t_dt = datetime.fromisoformat(tgt["date"].replace("Z", "+00:00"))
            except Exception:
                continue
            # tgt 가 더 일찍 발견 → tgt → s causal
            if t_dt < s_dt:
                causal_edges.append({
                    "src": ref, "dst": s["id"],
                    "delta_hours": round((s_dt - t_dt).total_seconds() / 3600, 1),
                    "type": "cross_repo_reference",
                })
            else:
                # 동시 또는 역순 — 무시 (causal 아님)
                pass
        # predicts 안의 SIG ref
        for p in s["predicts"]:
            for sig_ref in re.findall(r'SIG-[A-Z0-9_-]+', p):
                if sig_ref not in nodes:
                    continue
                tgt = nodes[sig_ref]
                if not tgt["date"]:
                    continue
                try:
                    t_dt = datetime.fromisoformat(tgt["date"].replace("Z", "+00:00"))
                except Exception:
                    continue
                if t_dt > s_dt:
                    # s 가 미래 sig_ref 를 예측 → s → sig_ref causal
                    causal_edges.append({
                        "src": s["id"], "dst": sig_ref,
                        "delta_hours": round((t_dt - s_dt).total_seconds() / 3600, 1),
                        "type": "predicts_realized",
                    })

    # 간단 causal graph 통계
    causal_in = defaultdict(int)
    causal_out = defaultdict(int)
    for e in causal_edges:
        causal_out[e["src"]] += 1
        causal_in[e["dst"]] += 1
    top_hubs = sorted(causal_in.items(), key=lambda x: -x[1])[:10]
    top_sources = sorted(causal_out.items(), key=lambda x: -x[1])[:10]

    H5 = {
        "total_causal_edges": len(causal_edges),
        "total_nodes_in_causal": len(set(e["src"] for e in causal_edges) | set(e["dst"] for e in causal_edges)),
        "edge_type_breakdown": dict(Counter(e["type"] for e in causal_edges)),
        "top10_causal_hubs_in": [
            {"id": nid, "in_count": c, "grade": nodes[nid]["grade"]}
            for nid, c in top_hubs
        ],
        "top10_causal_sources_out": [
            {"id": nid, "out_count": c, "grade": nodes[nid]["grade"]}
            for nid, c in top_sources
        ],
        "method": "temporal-ordered + 명시 참조 (cross_repo / predicts) edge — PC/GES 미적용 (데이터 노이즈)",
    }

    # ──────────── 통합 출력 ────────────
    result = {
        "metadata": {
            "session": "Group P — H1~H5",
            "date": "2026-04-15",
            "total_signals_parsed": len(sigs),
            "honest_statement": "7대 밀레니엄 난제 해결: 0/7 유지",
        },
        "H1_signal_ROI": H1,
        "H2_pagerank": H2,
        "H3_next_session_prediction": H3,
        "H4_depletion_forecast": H4,
        "H5_causal_graph": H5,
    }

    out_json = OUT_DIR / "h_meta_analysis_result.json"
    with open(out_json, 'w', encoding='utf-8') as f:
        json.dump(result, f, ensure_ascii=False, indent=2)
    print(f"[done] JSON 저장: {out_json}")

    # ──────────── 한글 콘솔 요약 ────────────
    print("\n========== H1 ROI ==========")
    print(f"  총 signal: {H1['total_signals']}")
    print(f"  평균 길이: {H1['avg_statement_length_chars']}자")
    print(f"  평균 복잡도 (unique 토큰): {H1['avg_complexity_unique_tokens']}")
    print(f"  평균 witness: {H1['avg_witness']}")
    print("  등급별 ROI:")
    for g, d in H1["roi_by_grade"].items():
        print(f"    [{g}] count={d['count']} ROI={d['roi']} avg_w={d['avg_witness']} cross={d['cross_repo_ratio']}")

    print("\n========== H2 PageRank Top 5 ==========")
    for r in H2["top5_pagerank"]:
        print(f"  {r['id']} pr={r['pagerank']} in={r['in_degree']} out={r['out_degree']} [{r['grade']}]")

    print("\n========== H3 다음 세션 예측 ==========")
    print(f"  최근 7일 평균: {H3['last7_avg']}")
    print(f"  LR slope: {H3['linear_regression_slope_per_day']}/일")
    print(f"  예측 signal 수: {H3['next_session_predicted_signals']}")

    print("\n========== H4 고갈 예측 ==========")
    print(f"  현 누적: {H4['current_cumulative']}")
    print(f"  slope: {H4['linear_slope_recent14']}")
    print(f"  고갈 예상: {H4['depletion_predicted_date']}")
    print(f"  반감 예상: {H4['half_depletion_date']}")

    print("\n========== H5 Causal Graph ==========")
    print(f"  edges: {H5['total_causal_edges']}")
    print(f"  nodes: {H5['total_nodes_in_causal']}")
    print(f"  type: {H5['edge_type_breakdown']}")
    print("  Top 5 hubs (in-degree):")
    for h in H5["top10_causal_hubs_in"][:5]:
        print(f"    {h['id']} in={h['in_count']} [{h['grade']}]")

    return result


if __name__ == "__main__":
    main()
