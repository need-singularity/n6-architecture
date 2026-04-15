#!/usr/bin/env python3
# LMFDB Cremona DB 타원곡선 실측 러너 — GALO-PX-2 / HONEST-PX-AUTO-EMPIRICAL
# 용도: BT-546 BSD atlas MILL-PX-A9 (E[|Sel_n(E)|] = σ(n) squarefree n) 실측 검증
# 정직: Python 선택 이유 = hexa HTTP 미지원. theory/ 외부 인프라 한정 사용.
# 출력: data/cremona/cremona_sample_N.json (N = 수집량)

import json
import sys
import time
from pathlib import Path
import urllib.request
import urllib.parse

LMFDB_BASE = "https://www.lmfdb.org/api/ec_curvedata"
PAGE_SIZE = 1000  # LMFDB max per call
FIELDS = ["lmfdb_label", "ainvs", "conductor", "rank", "sha",
          "torsion", "torsion_structure", "analytic_rank",
          "num_bad_primes", "absD"]
SLEEP = 0.3  # 예의상 레이트 리밋


def fetch_page(offset: int, limit: int) -> list:
    """LMFDB ec_curvedata 한 페이지 fetch"""
    params = {
        "_format": "json",
        "_fields": ",".join(FIELDS),
        "_limit": limit,
        "_offset": offset,
    }
    url = f"{LMFDB_BASE}/?{urllib.parse.urlencode(params)}"
    with urllib.request.urlopen(url, timeout=30) as resp:
        raw = resp.read().decode("utf-8")
    d = json.loads(raw)
    return d.get("data", [])


def compute_sel_size_2(rank: int, torsion_structure: list, sha: int | None) -> int | None:
    """|Sel_2(E)| 근사: 2^(rank + t_2 + s_2) 단 sha 2-part 근사"""
    if sha is None:
        return None
    t_2 = sum(1 for ts in torsion_structure if ts % 2 == 0)
    # Sha[2] 추정: sha 가 2-parti (짝수) 만큼
    s_2 = 0
    sha_tmp = sha
    while sha_tmp % 4 == 0:  # Sha[2] 는 일반적으로 sha 의 제곱근에 관계
        s_2 += 1
        sha_tmp //= 4
    if sha_tmp % 2 == 0:
        s_2 += 1
    return 2 ** (rank + t_2 + s_2)


def compute_sel_size_3(rank: int, torsion_structure: list, sha: int | None) -> int | None:
    """|Sel_3(E)| 근사"""
    if sha is None:
        return None
    t_3 = sum(1 for ts in torsion_structure if ts % 3 == 0)
    s_3 = 0
    sha_tmp = sha
    while sha_tmp % 9 == 0:
        s_3 += 1
        sha_tmp //= 9
    if sha_tmp % 3 == 0:
        s_3 += 1
    return 3 ** (rank + t_3 + s_3)


def main():
    n_target = int(sys.argv[1]) if len(sys.argv) > 1 else 50000
    # exec() 시 __file__ 없을 수 있으므로 fallback
    try:
        base = Path(__file__).resolve().parent.parent.parent
    except NameError:
        base = Path("/Users/ghost/Dev/n6-architecture")
    out_path = base / "data" / "cremona" / f"cremona_sample_{n_target}.json"
    out_path.parent.mkdir(parents=True, exist_ok=True)

    print(f"[GALO-PX-2] LMFDB ec_curvedata 수집 시작 → {n_target} 건", file=sys.stderr)
    print(f"[출력] {out_path}", file=sys.stderr)

    rows = []
    offset = 0
    t0 = time.time()

    while len(rows) < n_target:
        need = min(PAGE_SIZE, n_target - len(rows))
        try:
            page = fetch_page(offset, need)
        except Exception as e:
            print(f"[페이지 {offset} 실패] {e} — 5초 재시도", file=sys.stderr)
            time.sleep(5)
            try:
                page = fetch_page(offset, need)
            except Exception as e2:
                print(f"[최종 실패] offset={offset}: {e2}", file=sys.stderr)
                break
        if not page:
            print(f"[offset {offset}] 빈 페이지 → 종료", file=sys.stderr)
            break
        rows.extend(page)
        offset += len(page)
        elapsed = time.time() - t0
        rate = len(rows) / elapsed if elapsed > 0 else 0
        print(f"  진행 {len(rows)}/{n_target} ({100*len(rows)/n_target:.1f}%) — {rate:.0f}건/초", file=sys.stderr)
        time.sleep(SLEEP)

    # Sel_2, Sel_3, Sel_6 계산 부가
    enriched = []
    miss_sha = 0
    for r in rows:
        sha = r.get("sha")
        ts = r.get("torsion_structure", [])
        rk = r.get("rank")
        if rk is None or sha is None:
            miss_sha += 1
            continue
        s2 = compute_sel_size_2(rk, ts, sha)
        s3 = compute_sel_size_3(rk, ts, sha)
        if s2 is None or s3 is None:
            miss_sha += 1
            continue
        s6 = s2 * s3  # CRT: |Sel_6| = |Sel_2|·|Sel_3| (gcd(2,3)=1, MILL-PX-A8 PROVEN)
        enriched.append({
            "label": r.get("lmfdb_label"),
            "conductor": r.get("conductor"),
            "rank": rk,
            "sha": sha,
            "torsion": r.get("torsion", 1),
            "torsion_structure": ts,
            "sel_2": s2,
            "sel_3": s3,
            "sel_6": s6,
        })

    out_path.write_text(json.dumps({
        "source": "LMFDB ec_curvedata API",
        "fetched_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "n_target": n_target,
        "n_fetched": len(rows),
        "n_enriched": len(enriched),
        "n_missed": miss_sha,
        "fields": FIELDS,
        "sel_formula": "|Sel_p(E)| ≈ p^(rank + t_p + s_p) (근사, 정확값은 Sha 구조에 따라 다름)",
        "crt_note": "|Sel_6| = |Sel_2|·|Sel_3| (MILL-PX-A8 PROVEN, gcd=1)",
        "data": enriched,
    }, indent=2), encoding="utf-8")

    elapsed = time.time() - t0
    print(f"[완료] {len(enriched)} enriched / {len(rows)} fetched / {miss_sha} missed — {elapsed:.1f}초", file=sys.stderr)
    print(f"[저장] {out_path}")


if __name__ == "__main__":
    main()
