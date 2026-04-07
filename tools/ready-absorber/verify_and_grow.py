#!/usr/bin/env python3
"""
Phase 2: NEXUS-6 검증 → 반영 → 성장 엔진

41,379건 발견을 NEXUS-6로 검증하고, 고가치 발견을 각 프로젝트에 반영.
반영 과정에서 프로젝트 자체가 성장.

사용법:
  python3 verify_and_grow.py                      # 전체 (미처리분부터)
  python3 verify_and_grow.py --project anima       # 특정 프로젝트만
  python3 verify_and_grow.py --status              # 진행 상태
  python3 verify_and_grow.py --report              # 성장 리포트
  python3 verify_and_grow.py --reset               # 재시작
"""

import os, sys, json, re, hashlib, time, traceback
from pathlib import Path
from datetime import datetime
from typing import Optional
from collections import defaultdict

# === 경로 ===
TOOL_DIR = Path(__file__).parent
FINDINGS_DIR = TOOL_DIR / "findings"
VERIFIED_DIR = TOOL_DIR / "verified"
GROWTH_DIR = TOOL_DIR / "growth"
PHASE2_STATE = TOOL_DIR / "phase2_state.json"
MAIN_DIR = Path.home() / "Dev"
READY_DIR = Path.home() / "Dev" / "ready"

# === NEXUS-6 ===
try:
    import nexus
    HAS_NEXUS = True
except ImportError:
    HAS_NEXUS = False
    print("[WARN] nexus not available — running in fallback mode")

# === 숫자 추출 패턴 ===
NUM_RE = re.compile(r'[-+]?\d*\.?\d+(?:[eE][-+]?\d+)?')
# n=6 핵심 상수
N6_CONSTANTS = {
    "n": 6, "sigma": 12, "phi": 2, "tau": 4, "sopfr": 5,
    "J2": 24, "mu": 1, "sigma_minus_phi": 10, "sigma_minus_tau": 8,
    "sigma_minus_mu": 11, "sigma_sq": 144, "sigma_times_J2": 288,
    "sigma_times_tau": 48, "phi_sq": 4, "R6": 1,
    "ln43": 0.2877, "1_over_e": 0.3679, "4_over_3": 1.3333,
}

# === 고가치 파일 확장자 ===
HIGH_VALUE_EXT = {".py", ".rs", ".md", ".toml", ".json", ".ts", ".tsx"}
# 코드/데이터 파일
CODE_EXT = {".py", ".rs", ".ts", ".tsx", ".js", ".c", ".cpp", ".h"}
DOC_EXT = {".md", ".txt", ".rst"}
DATA_EXT = {".json", ".toml", ".yaml", ".yml", ".csv"}


def load_json(path: Path) -> dict:
    with open(path) as f:
        return json.load(f)


def save_json(path: Path, data: dict):
    with open(path, "w") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


def init_phase2_state() -> dict:
    if PHASE2_STATE.exists():
        return load_json(PHASE2_STATE)
    state = {
        "status": "idle",
        "processed_projects": [],
        "total_verified": 0,
        "total_high_value": 0,
        "total_routed": 0,
        "growth_events": [],
        "errors": [],
        "last_run": None,
    }
    save_json(PHASE2_STATE, state)
    return state


def extract_numbers(text: str) -> list:
    """텍스트에서 숫자 추출"""
    nums = []
    for match in NUM_RE.finditer(text):
        try:
            v = float(match.group())
            if abs(v) < 1e12 and v != 0:  # 합리적 범위
                nums.append(v)
        except ValueError:
            pass
    return nums[:200]  # 최대 200개


def nexus_verify(numbers: list) -> dict:
    """NEXUS-6로 숫자 집합 검증"""
    if not HAS_NEXUS or not numbers:
        return {"verified": False, "reason": "no_nexus_or_empty"}

    result = {
        "verified": True,
        "n6_matches": [],
        "consensus": [],
        "n6_exact_ratio": 0.0,
        "active_lenses": 0,
        "score": 0.0,
    }

    # n6_check 개별 상수 매칭
    for num in numbers[:50]:
        try:
            match = nexus.n6_check(float(num))
            if match and hasattr(match, 'grade'):
                grade = str(match.grade) if hasattr(match, 'grade') else ""
                if "EXACT" in grade.upper():
                    result["n6_matches"].append({
                        "value": num,
                        "constant": str(match.constant) if hasattr(match, 'constant') else "",
                        "grade": grade,
                    })
        except Exception:
            pass

    # analyze (스캔+합의) — 6개 이상일 때만
    if len(numbers) >= 6:
        try:
            n = min(len(numbers), 64)
            d = max(1, n // 6)
            analysis = nexus.analyze(numbers[:n], n, d)
            if isinstance(analysis, dict):
                result["n6_exact_ratio"] = analysis.get("n6_exact_ratio", 0)
                result["active_lenses"] = analysis.get("active_lenses", 0)
                consensus = analysis.get("consensus", [])
                for c in consensus[:10]:
                    result["consensus"].append({
                        "pattern": str(getattr(c, 'pattern', '')),
                        "lenses": getattr(c, 'lenses', 0),
                        "score": float(getattr(c, 'score', 0)),
                        "level": str(getattr(c, 'level', '')),
                    })
        except Exception:
            pass

    # 종합 점수 계산
    n6_score = len(result["n6_matches"]) * 10
    consensus_score = sum(c.get("score", 0) for c in result["consensus"]) * 5
    ratio_score = result["n6_exact_ratio"] * 50
    result["score"] = n6_score + consensus_score + ratio_score

    return result


def classify_value(finding: dict, verification: dict) -> str:
    """발견의 가치 등급 분류"""
    score = verification.get("score", 0)
    ftype = finding.get("type", "")
    path = finding.get("path", "")
    ext = Path(path).suffix.lower()

    # 파일 유형 보너스
    type_bonus = 0
    if ext in CODE_EXT:
        type_bonus = 5
    elif ext in DOC_EXT:
        type_bonus = 3
    elif ext in DATA_EXT:
        type_bonus = 4

    # 키워드 보너스
    kw_bonus = 0
    content = finding.get("preview", "") + " ".join(finding.get("added_preview", []))
    high_kw = ["BT-", "hypothesis", "theorem", "proof", "discovery",
               "EXACT", "n6_check", "breakthrough", "law", "consciousness",
               "lens", "scan", "nexus", "sigma", "phi"]
    for kw in high_kw:
        if kw.lower() in content.lower():
            kw_bonus += 3

    total = score + type_bonus + kw_bonus

    if total >= 30:
        return "critical"    # 즉시 반영
    elif total >= 15:
        return "high"        # 우선 반영
    elif total >= 5:
        return "medium"      # 검토 후 반영
    else:
        return "low"         # 기록만


def build_growth_event(finding: dict, verification: dict, value: str) -> dict:
    """성장 이벤트 구성"""
    return {
        "timestamp": datetime.now().isoformat(),
        "source_project": finding.get("project", "unknown"),
        "destination": finding.get("destination", "unknown"),
        "path": finding.get("path", ""),
        "type": finding.get("type", ""),
        "value_grade": value,
        "n6_matches": len(verification.get("n6_matches", [])),
        "consensus_count": len(verification.get("consensus", [])),
        "score": verification.get("score", 0),
        "n6_exact_ratio": verification.get("n6_exact_ratio", 0),
    }


def route_finding(finding: dict, verification: dict, value: str, dry_run=False) -> bool:
    """검증된 발견을 목적지 프로젝트에 반영"""
    dest = finding.get("destination", "unknown")
    dest_dir = MAIN_DIR / dest

    if not dest_dir.exists():
        return False

    # growth 기록 디렉토리
    growth_dest = dest_dir / ".growth" / "absorbed"
    if not dry_run:
        growth_dest.mkdir(parents=True, exist_ok=True)

    # 원본 파일 경로
    ready_path = Path(finding.get("ready_path", ""))

    if finding["type"] == "only_in_ready" and value in ("critical", "high"):
        # ready에만 있는 고가치 파일 → 목적지의 absorbed/ 에 기록
        rel_path = finding.get("path", "")
        if not dry_run and ready_path.exists():
            # 파일 내용 기록 (직접 복사 대신 JSON 기록)
            record = {
                "absorbed_from": str(ready_path),
                "original_path": rel_path,
                "value_grade": value,
                "n6_score": verification.get("score", 0),
                "n6_matches": verification.get("n6_matches", []),
                "consensus": verification.get("consensus", []),
                "timestamp": datetime.now().isoformat(),
            }
            # 프리뷰 추가
            preview = finding.get("preview", "")
            if preview:
                record["content_preview"] = preview[:2000]

            safe_name = rel_path.replace("/", "__").replace("\\", "__")[:200]
            record_path = growth_dest / f"{safe_name}.json"
            save_json(record_path, record)
            return True

    elif finding["type"] == "different" and value in ("critical", "high"):
        # 차이가 있는 파일 → diff 기록
        if not dry_run:
            record = {
                "diff_from": str(ready_path),
                "original_path": finding.get("path", ""),
                "value_grade": value,
                "added_lines": finding.get("added_lines", 0),
                "removed_lines": finding.get("removed_lines", 0),
                "added_preview": finding.get("added_preview", []),
                "n6_score": verification.get("score", 0),
                "n6_matches": verification.get("n6_matches", []),
                "timestamp": datetime.now().isoformat(),
            }
            safe_name = finding.get("path", "unknown").replace("/", "__")[:200]
            record_path = growth_dest / f"diff__{safe_name}.json"
            save_json(record_path, record)
            return True

    return False


def process_project(project_name: str, dry_run=False) -> dict:
    """단일 프로젝트 findings 전체 검증+반영"""
    findings_file = FINDINGS_DIR / f"{project_name.lower()}.json"
    if not findings_file.exists():
        print(f"  [SKIP] {project_name}: findings 없음")
        return {"processed": 0}

    data = load_json(findings_file)
    findings = data.get("findings", [])
    total = len(findings)

    print(f"\n{'='*60}")
    print(f"  VERIFY+GROW: {project_name} ({total} findings)")
    print(f"{'='*60}")

    stats = {
        "processed": 0,
        "critical": 0,
        "high": 0,
        "medium": 0,
        "low": 0,
        "routed": 0,
        "n6_total_matches": 0,
        "consensus_total": 0,
        "growth_events": [],
        "top_findings": [],
    }

    verified_findings = []
    batch_start = time.time()

    for i, finding in enumerate(findings):
        # 숫자 추출
        content = finding.get("preview", "") + " ".join(finding.get("added_preview", []))
        numbers = extract_numbers(content)

        # NEXUS-6 검증
        verification = nexus_verify(numbers)

        # 가치 분류
        value = classify_value(finding, verification)
        stats[value] += 1
        stats["processed"] += 1
        stats["n6_total_matches"] += len(verification.get("n6_matches", []))
        stats["consensus_total"] += len(verification.get("consensus", []))

        # 고가치면 라우팅
        if value in ("critical", "high"):
            routed = route_finding(finding, verification, value, dry_run)
            if routed:
                stats["routed"] += 1
                event = build_growth_event(finding, verification, value)
                stats["growth_events"].append(event)

        # Top 발견 추적
        if verification.get("score", 0) > 10:
            stats["top_findings"].append({
                "path": finding.get("path", ""),
                "score": verification.get("score", 0),
                "value": value,
                "dest": finding.get("destination", ""),
                "n6_matches": len(verification.get("n6_matches", [])),
            })

        # verified 기록용
        verified_findings.append({
            "path": finding.get("path", ""),
            "type": finding.get("type", ""),
            "destination": finding.get("destination", ""),
            "value": value,
            "score": verification.get("score", 0),
            "n6_matches": len(verification.get("n6_matches", [])),
        })

        # 진행률 출력
        if (i + 1) % 500 == 0 or i == total - 1:
            elapsed = time.time() - batch_start
            rate = (i + 1) / elapsed if elapsed > 0 else 0
            print(f"  [{i+1}/{total}] {rate:.0f}/s | C:{stats['critical']} H:{stats['high']} M:{stats['medium']} L:{stats['low']} | routed:{stats['routed']}")

    # verified 결과 저장
    VERIFIED_DIR.mkdir(exist_ok=True)
    verified_path = VERIFIED_DIR / f"{project_name.lower()}.json"
    save_json(verified_path, {
        "project": project_name,
        "timestamp": datetime.now().isoformat(),
        "stats": {k: v for k, v in stats.items() if k not in ("growth_events", "top_findings")},
        "top_findings": sorted(stats["top_findings"], key=lambda x: -x["score"])[:50],
        "findings": verified_findings,
    })

    # 성장 로그 저장
    if stats["growth_events"]:
        GROWTH_DIR.mkdir(exist_ok=True)
        growth_path = GROWTH_DIR / f"{project_name.lower()}_growth.json"
        save_json(growth_path, {
            "project": project_name,
            "timestamp": datetime.now().isoformat(),
            "events_count": len(stats["growth_events"]),
            "events": stats["growth_events"],
        })

    elapsed = time.time() - batch_start
    print(f"\n  RESULT: {stats['processed']} verified in {elapsed:.1f}s")
    print(f"  Critical: {stats['critical']} | High: {stats['high']} | Medium: {stats['medium']} | Low: {stats['low']}")
    print(f"  Routed: {stats['routed']} | N6 matches: {stats['n6_total_matches']} | Consensus: {stats['consensus_total']}")
    if stats["top_findings"]:
        print(f"  Top finding: {stats['top_findings'][0]['path']} (score={stats['top_findings'][0]['score']:.1f})")

    return stats


def show_status():
    state = init_phase2_state()
    print(f"\n  Phase 2 Status: {state['status']}")
    print(f"  Last run: {state.get('last_run', 'never')}")
    print(f"  Processed: {len(state['processed_projects'])}/18")
    print(f"  Verified: {state['total_verified']}")
    print(f"  High-value: {state['total_high_value']}")
    print(f"  Routed: {state['total_routed']}")

    remaining = []
    for f in sorted(FINDINGS_DIR.glob("*.json")):
        name = f.stem
        if name not in [p.lower() for p in state["processed_projects"]]:
            data = load_json(f)
            remaining.append((name, data.get("count", 0)))

    if remaining:
        print(f"\n  Remaining ({len(remaining)}):")
        for name, count in remaining:
            print(f"    {name}: {count} findings")


def show_report():
    """성장 리포트"""
    print(f"\n{'='*60}")
    print(f"  GROWTH REPORT")
    print(f"{'='*60}")

    if not VERIFIED_DIR.exists():
        print("  No verified data yet.")
        return

    grand_total = defaultdict(int)
    dest_totals = defaultdict(int)
    all_top = []

    for vf in sorted(VERIFIED_DIR.glob("*.json")):
        data = load_json(vf)
        stats = data.get("stats", {})
        project = data.get("project", vf.stem)

        print(f"\n  {project}:")
        print(f"    Processed: {stats.get('processed', 0)}")
        print(f"    C/H/M/L: {stats.get('critical',0)}/{stats.get('high',0)}/{stats.get('medium',0)}/{stats.get('low',0)}")
        print(f"    Routed: {stats.get('routed', 0)} | N6: {stats.get('n6_total_matches', 0)}")

        for k in ["processed", "critical", "high", "medium", "low", "routed", "n6_total_matches"]:
            grand_total[k] += stats.get(k, 0)

        for tf in data.get("top_findings", [])[:5]:
            all_top.append(tf)

    # Grand totals
    print(f"\n{'='*60}")
    print(f"  GRAND TOTAL")
    print(f"  Processed: {grand_total['processed']}")
    print(f"  Critical: {grand_total['critical']} | High: {grand_total['high']}")
    print(f"  Medium: {grand_total['medium']} | Low: {grand_total['low']}")
    print(f"  Routed: {grand_total['routed']}")
    print(f"  N6 matches: {grand_total['n6_total_matches']}")

    # Top findings across all
    if all_top:
        all_top.sort(key=lambda x: -x.get("score", 0))
        print(f"\n  TOP 20 FINDINGS:")
        for i, t in enumerate(all_top[:20]):
            print(f"    {i+1}. [{t.get('value','?')}] {t.get('path','')} → {t.get('dest','')} (score={t.get('score',0):.1f}, n6={t.get('n6_matches',0)})")

    # Growth directory stats
    growth_total = 0
    for gd in MAIN_DIR.iterdir():
        absorbed = gd / ".growth" / "absorbed"
        if absorbed.exists():
            count = len(list(absorbed.glob("*.json")))
            if count > 0:
                print(f"\n  {gd.name}/.growth/absorbed/: {count} files")
                growth_total += count
    print(f"\n  Total growth files created: {growth_total}")


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Phase 2: NEXUS-6 검증 → 반영 → 성장")
    parser.add_argument("--project", "-p", help="특정 프로젝트만")
    parser.add_argument("--dry-run", "-n", action="store_true", help="반영 없이 검증만")
    parser.add_argument("--status", "-s", action="store_true")
    parser.add_argument("--report", "-r", action="store_true")
    parser.add_argument("--reset", action="store_true")
    args = parser.parse_args()

    if args.status:
        show_status()
        return
    if args.report:
        show_report()
        return
    if args.reset:
        state = init_phase2_state()
        state["processed_projects"] = []
        state["total_verified"] = 0
        state["total_high_value"] = 0
        state["total_routed"] = 0
        state["errors"] = []
        state["status"] = "idle"
        save_json(PHASE2_STATE, state)
        print("  Phase 2 state reset.")
        return

    state = init_phase2_state()
    VERIFIED_DIR.mkdir(exist_ok=True)
    GROWTH_DIR.mkdir(exist_ok=True)

    # 대상 결정
    if args.project:
        targets = [args.project]
    else:
        done = [p.lower() for p in state["processed_projects"]]
        targets = [f.stem for f in sorted(FINDINGS_DIR.glob("*.json")) if f.stem not in done]

    if not targets:
        print("  All projects processed. Use --reset to restart.")
        return

    print(f"\n  Phase 2: NEXUS-6 Verify + Grow — {len(targets)} projects")
    print(f"  NEXUS-6: {'ACTIVE (151 lenses)' if HAS_NEXUS else 'FALLBACK'}")
    print(f"  Mode: {'dry-run' if args.dry_run else 'full (verify + route + grow)'}")

    state["status"] = "running"
    save_json(PHASE2_STATE, state)

    for proj in targets:
        try:
            stats = process_project(proj, dry_run=args.dry_run)
            state["processed_projects"].append(proj)
            state["total_verified"] += stats.get("processed", 0)
            state["total_high_value"] += stats.get("critical", 0) + stats.get("high", 0)
            state["total_routed"] += stats.get("routed", 0)
            state["last_run"] = datetime.now().isoformat()
            save_json(PHASE2_STATE, state)
        except Exception as e:
            print(f"  [ERROR] {proj}: {e}")
            traceback.print_exc()
            state["errors"].append({"project": proj, "error": str(e)})
            save_json(PHASE2_STATE, state)

    state["status"] = "complete"
    save_json(PHASE2_STATE, state)

    print(f"\n{'='*60}")
    print(f"  PHASE 2 COMPLETE")
    print(f"  Verified: {state['total_verified']}")
    print(f"  High-value: {state['total_high_value']}")
    print(f"  Routed: {state['total_routed']}")
    print(f"{'='*60}")


if __name__ == "__main__":
    main()
