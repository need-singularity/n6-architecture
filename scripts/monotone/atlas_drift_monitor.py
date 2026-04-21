#!/usr/bin/env python3
# MONOTONE-PX-1: atlas.n6 등급 단조성 감시 CLI
# 규칙: atlas 엔트리의 등급은 시간에 따라 단조 비감소. [10*] → [9] 같은 하향은 drift 의심.
# 사용: python3 atlas_drift_monitor.py [--update-snapshot]
#   (--update-snapshot 없으면 체크만, 있으면 현재 상태를 새 snapshot 으로 기록)
# 출력: JSON report — drift 발견 시 exit code 1

import argparse
import json
import re
import sys
from pathlib import Path

ATLAS_PATH = Path("/Users/ghost/Dev/nexus/n6/atlas.n6")
SNAPSHOT_PATH = Path("/Users/ghost/Dev/n6-architecture/data/atlas_grade_snapshot.json")
REPORT_PATH = Path("/Users/ghost/Dev/n6-architecture/reports/atlas_drift_latest.json")

# 등급 순위 (높을수록 강한 주장)
GRADE_RANK = {
    "N!": 11,       # breakthrough (최상위)
    "10*": 10,      # EXACT 검증
    "10": 9,        # EXACT
    "9": 8,         # NEAR
    "8": 7,
    "7": 6,         # EMPIRICAL (승격 대상)
    "6": 5,
    "5": 4,
    "N?": 2,        # CONJECTURE
}


def parse_atlas_entries(atlas_text: str) -> dict:
    """@R ID = expr :: axis [grade] 패턴 파싱"""
    entries = {}
    pattern = re.compile(r"^@[RX]\s+([\w\-]+)\s*=\s*.+?::\s*\S+\s*\[([^\]]+)\]", re.MULTILINE)
    for m in pattern.finditer(atlas_text):
        entry_id = m.group(1)
        grade = m.group(2).strip()
        entries[entry_id] = grade
    return entries


def grade_strength(g: str) -> int:
    """등급 → 정수 강도 (비교용)"""
    return GRADE_RANK.get(g, 0)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--update-snapshot", action="store_true",
                    help="현재 상태를 새 snapshot 으로 기록 (drift 검사 후)")
    ap.add_argument("--atlas", default=str(ATLAS_PATH), help="atlas.n6 경로")
    ap.add_argument("--snapshot", default=str(SNAPSHOT_PATH), help="snapshot 경로")
    args = ap.parse_args()

    atlas_path = Path(args.atlas)
    snap_path = Path(args.snapshot)

    if not atlas_path.exists():
        print(f"[ERR] atlas 없음: {atlas_path}", file=sys.stderr)
        sys.exit(2)

    atlas_text = atlas_path.read_text(encoding="utf-8")
    current = parse_atlas_entries(atlas_text)
    print(f"[스캔] {atlas_path}", file=sys.stderr)
    print(f"  N entries = {len(current)}", file=sys.stderr)

    # 등급 분포
    dist = {}
    for g in current.values():
        dist[g] = dist.get(g, 0) + 1
    print(f"  등급 분포: {dict(sorted(dist.items(), key=lambda x: -grade_strength(x[0])))}", file=sys.stderr)

    # snapshot 비교
    drifts = []
    missing = []
    new_entries = []
    if snap_path.exists():
        prev = json.loads(snap_path.read_text(encoding="utf-8"))
        prev_entries = prev.get("entries", {})
        print(f"[비교] 이전 snapshot: {prev.get('timestamp', 'unknown')}, N={len(prev_entries)}", file=sys.stderr)

        for eid, prev_g in prev_entries.items():
            if eid not in current:
                missing.append({"id": eid, "prev_grade": prev_g})
                continue
            cur_g = current[eid]
            if cur_g != prev_g:
                prev_str = grade_strength(prev_g)
                cur_str = grade_strength(cur_g)
                if cur_str < prev_str:
                    drifts.append({
                        "id": eid,
                        "prev_grade": prev_g,
                        "cur_grade": cur_g,
                        "prev_strength": prev_str,
                        "cur_strength": cur_str,
                        "delta": cur_str - prev_str,
                    })
                # 상승은 OK (기록만)
        for eid in current:
            if eid not in prev_entries:
                new_entries.append({"id": eid, "grade": current[eid]})
    else:
        print(f"[초기] snapshot 없음 — 최초 기록", file=sys.stderr)

    # 리포트
    report = {
        "timestamp": __import__("time").strftime("%Y-%m-%dT%H:%M:%SZ", __import__("time").gmtime()),
        "atlas_path": str(atlas_path),
        "n_entries_current": len(current),
        "grade_distribution": dist,
        "drifts_detected": drifts,
        "entries_missing": missing,
        "new_entries_count": len(new_entries),
        "verdict": "DRIFT_DETECTED" if drifts else "MONOTONE_OK",
    }
    REPORT_PATH.parent.mkdir(parents=True, exist_ok=True)
    REPORT_PATH.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"[리포트] {REPORT_PATH}", file=sys.stderr)

    print()
    print("=" * 60)
    if drifts:
        print(f"[DRIFT 발견] {len(drifts)} 엔트리 등급 하향")
        print("=" * 60)
        for d in drifts[:20]:
            print(f"  ⚠ {d['id']}: [{d['prev_grade']}] → [{d['cur_grade']}] (Δstrength={d['delta']})")
        if len(drifts) > 20:
            print(f"  ... 외 {len(drifts) - 20} 건")
    else:
        print(f"[MONOTONE OK] 등급 하향 없음 ({len(current)} entries)")
    print("=" * 60)

    if missing:
        print(f"[삭제 엔트리] {len(missing)} (정보용)")
        for m in missing[:5]:
            print(f"  - {m['id']}: [{m['prev_grade']}]")
    if new_entries:
        print(f"[신규 엔트리] {len(new_entries)} (정보용)")
        for n in new_entries[:5]:
            print(f"  + {n['id']}: [{n['grade']}]")

    # snapshot 업데이트
    if args.update_snapshot:
        snap_path.parent.mkdir(parents=True, exist_ok=True)
        snap_data = {
            "timestamp": report["timestamp"],
            "atlas_path": str(atlas_path),
            "entries": current,
            "grade_distribution": dist,
        }
        snap_path.write_text(json.dumps(snap_data, indent=2, ensure_ascii=False), encoding="utf-8")
        print(f"\n[snapshot 갱신] {snap_path} ({len(current)} entries)")

    sys.exit(1 if drifts else 0)


if __name__ == "__main__":
    main()
