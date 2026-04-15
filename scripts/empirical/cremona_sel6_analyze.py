#!/usr/bin/env python3
# Cremona allbsd 332k 타원곡선 → Sel_6(E) 통계 실측
# 용도: atlas MILL-PX-A9 (E[|Sel_n(E)|] = σ(n), squarefree n) 대상 n=6 empirical check
# 출처: https://github.com/JohnCremona/ecdata
# 정직: Python 사용 이유 = hexa 텍스트 파싱 및 대용량 처리 제약

import json
import sys
from pathlib import Path
from collections import Counter

ROOT = Path("/Users/ghost/Dev/n6-architecture")
DATA_DIR = ROOT / "data" / "cremona" / "allbsd"


def parse_allbsd_line(line: str) -> dict | None:
    """1 line of allbsd: conductor iso num ainvs rank torsion tam omega reg L sha
    정직: 일부 라인의 sha 값은 analytic (BSD conjectural), 라인에 따라 열 수 달라짐 가능."""
    parts = line.strip().split()
    if len(parts) < 8:
        return None
    try:
        conductor = int(parts[0])
        rank = int(parts[4])
        torsion_order = int(parts[5])
        # analytic sha — float or int. int(round(...)) 로 정규화.
        sha_raw = parts[-1]
        sha = int(round(float(sha_raw)))
        if sha < 1:
            sha = 1  # BSD 예상 sha=1 fallback (ALiE float jitter)
    except (ValueError, IndexError):
        return None
    return {"N": conductor, "r": rank, "t": torsion_order, "sha": sha}


def size_sel_2(r: int, t: int, sha: int) -> int:
    """|Sel_2(E)| 추정. 1차근사 (torsion 구조 Z/2×Z/2 무시)"""
    t_factor = 2 if t % 2 == 0 else 1
    # Sha 2-primary part: sha=4 ⟹ 4, sha=16 ⟹ 16, sha=64 ⟹ 64 (|Sha[2]| order)
    sha_2 = 1
    s = sha
    while s % 2 == 0:
        sha_2 *= 2
        s //= 2
    return (2 ** r) * t_factor * sha_2


def size_sel_3(r: int, t: int, sha: int) -> int:
    t_factor = 3 if t % 3 == 0 else 1
    sha_3 = 1
    s = sha
    while s % 3 == 0:
        sha_3 *= 3
        s //= 3
    return (3 ** r) * t_factor * sha_3


def main():
    shards = sorted(DATA_DIR.glob("allbsd.*"))
    if not shards:
        print("[ERR] data/cremona/allbsd/ 없음. 먼저 다운로드 필요.", file=sys.stderr)
        sys.exit(1)

    print(f"[GALO-PX-2] Cremona allbsd {len(shards)} shard 로드", file=sys.stderr)
    for s in shards:
        print(f"  · {s.name}", file=sys.stderr)

    curves = []
    skipped = 0
    for shard in shards:
        with shard.open("r", encoding="utf-8") as fh:
            for line in fh:
                c = parse_allbsd_line(line)
                if c is None:
                    skipped += 1
                else:
                    curves.append(c)

    print(f"[완료] {len(curves)} curve 파싱 / {skipped} skip", file=sys.stderr)

    # 통계
    rank_dist = Counter(c["r"] for c in curves)
    sha_dist = Counter(c["sha"] for c in curves)
    torsion_dist = Counter(c["t"] for c in curves)

    # Sel_2, Sel_3, Sel_6 계산
    sel2 = [size_sel_2(c["r"], c["t"], c["sha"]) for c in curves]
    sel3 = [size_sel_3(c["r"], c["t"], c["sha"]) for c in curves]
    sel6 = [s2 * s3 for s2, s3 in zip(sel2, sel3)]

    N = len(curves)
    mean_sel2 = sum(sel2) / N
    mean_sel3 = sum(sel3) / N
    mean_sel6 = sum(sel6) / N

    # 기대값: BKLPR E[|Sel_n|] = σ(n) for squarefree n
    # σ(2)=3, σ(3)=4, σ(6)=12
    print()
    print("=" * 72)
    print(f"[결과] Cremona allbsd N = {N}")
    print("=" * 72)
    print()
    print("rank 분포:")
    for r in sorted(rank_dist):
        pct = 100.0 * rank_dist[r] / N
        print(f"  rank={r}: {rank_dist[r]:>7} ({pct:.2f}%)")
    print()
    print("sha 분포 (상위 10):")
    for sha, cnt in sha_dist.most_common(10):
        print(f"  sha={sha}: {cnt:>7} ({100.0*cnt/N:.3f}%)")
    print()
    print("torsion order 분포 (상위 10):")
    for t, cnt in torsion_dist.most_common(10):
        print(f"  t={t}: {cnt:>7} ({100.0*cnt/N:.3f}%)")
    print()
    print("E[|Sel_n(E)|] vs BKLPR 예측:")
    print(f"  n=2: 실측 mean = {mean_sel2:.4f} | 예측 σ(2) = 3  | 비율 = {mean_sel2/3:.4f}")
    print(f"  n=3: 실측 mean = {mean_sel3:.4f} | 예측 σ(3) = 4  | 비율 = {mean_sel3/4:.4f}")
    print(f"  n=6: 실측 mean = {mean_sel6:.4f} | 예측 σ(6) = 12 | 비율 = {mean_sel6/12:.4f}")
    print()
    print("정직 주의:")
    print("  · Cremona 는 conductor 정렬, BKLPR 은 height (discriminant) 정렬. 정렬 차이 보정 없음.")
    print("  · |Sel_n| 계산은 1차근사 (torsion 구조 Z/2×Z/2 미분리, 완전 정밀값 아님).")
    print("  · sha 는 BSD conjectural value (analytic).")
    print("  · BKLPR (A3 가정) 미증명 — CONDITIONAL 해석.")
    print()

    # 출력 JSON 요약
    summary = {
        "source": "John Cremona ecdata (allbsd.00000-49999)",
        "N": N,
        "conductor_range": [1, 49999],
        "rank_distribution": dict(rank_dist),
        "sha_top10": sha_dist.most_common(10),
        "torsion_top10": torsion_dist.most_common(10),
        "empirical": {
            "mean_sel_2": mean_sel2,
            "mean_sel_3": mean_sel3,
            "mean_sel_6": mean_sel6,
        },
        "bklpr_prediction": {
            "E_sel_2": 3,
            "E_sel_3": 4,
            "E_sel_6": 12,
        },
        "ratio": {
            "sel_2": mean_sel2 / 3,
            "sel_3": mean_sel3 / 4,
            "sel_6": mean_sel6 / 12,
        },
        "caveats": [
            "Cremona conductor 정렬 vs BKLPR height 정렬 차이 미보정",
            "|Sel_n| 계산 1차근사 (torsion 구조 Z/2×Z/2 미분리)",
            "sha 는 BSD conjectural value (analytic)",
            "BKLPR (A3) 미증명 CONDITIONAL",
        ],
    }

    out_path = ROOT / "data" / "cremona" / "sel6_stats_332k.json"
    out_path.write_text(json.dumps(summary, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"[저장] {out_path}")


if __name__ == "__main__":
    main()
