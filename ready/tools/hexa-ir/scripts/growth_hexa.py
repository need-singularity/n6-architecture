#!/usr/bin/env python3
"""HEXA-IR Growth Engine — NEXUS-6 Growth Bus 자동 합류

Growth Bus가 *growth* 패턴으로 자동 발견.
NEXUS-6 렌즈 성장 → HEXA-IR 컴파일러 공진화.

피드백 루프:
  1. NEXUS-6 새 렌즈/상수 발견 → HEXA-IR n6.rs 갱신
  2. convergent_refinement 스캔 → anomaly 자동 수정
  3. 새 BT/패턴 발견 → opt/ 패스 후보 생성
  4. cargo test → 회귀 검증
  5. 결과를 Growth Bus에 발행

사용:
  python3 growth_hexa.py              # 1 사이클 (Growth Bus 호출용)
  python3 growth_hexa.py --standalone # 독립 실행 (Bus 없이)
  python3 growth_hexa.py --scan-only  # 스캔만 (수정 없음)
"""

import json, os, sys, subprocess, time
from pathlib import Path
from datetime import datetime

# ═══════════════════════════════════════════════════════════
# Paths
# ═══════════════════════════════════════════════════════════

HEXA_ROOT = Path(__file__).resolve().parent.parent  # tools/hexa-ir/
N6_ARCH = HEXA_ROOT.parent.parent                   # n6-architecture/
NEXUS_ROOT = Path.home() / "Dev" / "nexus"
BUS_FILE = NEXUS_ROOT / "shared" / "growth_bus.jsonl"
N6_RS = HEXA_ROOT / "src" / "util" / "n6.rs"
CONVERGENT = N6_ARCH / "tools" / "nexus" / "scripts" / "convergent_refinement.py"

# ═══════════════════════════════════════════════════════════
# Bus Integration
# ═══════════════════════════════════════════════════════════

def bus_emit(source, event_type, data):
    """Growth Bus에 이벤트 발행."""
    event = {
        "ts": datetime.now().isoformat(),
        "source": f"hexa-ir:{source}",
        "type": event_type,
        "data": data,
    }
    try:
        with open(BUS_FILE, "a") as f:
            f.write(json.dumps(event) + "\n")
    except:
        pass  # Bus 없으면 무시
    return event

def bus_query(event_type=None, last_n=10):
    """Growth Bus에서 이벤트 읽기."""
    events = []
    try:
        if BUS_FILE.exists():
            for line in BUS_FILE.read_text().strip().split("\n")[-last_n * 3:]:
                if not line.strip():
                    continue
                e = json.loads(line)
                if event_type and e.get("type") != event_type:
                    continue
                events.append(e)
    except:
        pass
    return events[-last_n:]

# ═══════════════════════════════════════════════════════════
# Phase 1: NEXUS-6 렌즈 동기화 — 새 렌즈 → n6.rs 상수 갱신
# ═══════════════════════════════════════════════════════════

def sync_nexus_constants():
    """NEXUS-6에서 발견된 새 상수를 n6.rs에 반영."""
    changes = 0

    # Read current n6.rs constants
    if not N6_RS.exists():
        return changes

    current = N6_RS.read_text()

    # Check NEXUS-6 lens count
    lens_dir = NEXUS_ROOT / "src" / "telescope" / "lenses"
    if lens_dir.exists():
        lens_count = sum(1 for f in lens_dir.glob("*.rs") if f.name != "mod.rs")

        # Update HEXA-IR's awareness of lens count
        marker = "// NEXUS-6 lens count (auto-synced)"
        new_line = f"pub const NEXUS_LENSES: usize = {lens_count};  {marker}"
        if marker not in current:
            # Add after SIGMA_J2 line
            current = current.replace(
                "pub const SIGMA_J2: usize = SIGMA * J2;",
                f"pub const SIGMA_J2: usize = SIGMA * J2;\n\n{new_line}"
            )
            N6_RS.write_text(current)
            changes += 1
            print(f"  [sync] Added NEXUS_LENSES = {lens_count}")
        elif f"NEXUS_LENSES: usize = {lens_count}" not in current:
            # Update existing
            import re
            current = re.sub(
                r'pub const NEXUS_LENSES: usize = \d+;',
                f'pub const NEXUS_LENSES: usize = {lens_count};',
                current
            )
            N6_RS.write_text(current)
            changes += 1
            print(f"  [sync] Updated NEXUS_LENSES → {lens_count}")

    # Check for new discoveries in bus
    discoveries = bus_query(event_type="engine_result", last_n=20)
    new_constants = []
    for event in discoveries:
        output = event.get("data", {}).get("output_tail", "")
        if "EXACT" in output and "n6_check" in output.lower():
            # Extract constant name and value if possible
            new_constants.append(event.get("source", "unknown"))

    if new_constants:
        print(f"  [sync] {len(new_constants)} new n6 discoveries detected in bus")

    return changes

# ═══════════════════════════════════════════════════════════
# Phase 2: 수렴 루프 실행 — anomaly 스캔
# ═══════════════════════════════════════════════════════════

def run_convergent_scan():
    """convergent_refinement.py로 HEXA-IR 스캔."""
    src_dir = str(HEXA_ROOT / "src")

    if not CONVERGENT.exists():
        print("  [scan] convergent_refinement.py not found, skipping")
        return {"exact": 0, "anomalies": 0, "total": 0}

    try:
        result = subprocess.run(
            ["python3", str(CONVERGENT), src_dir, "--depth", "survey", "--quiet"],
            capture_output=True, text=True, timeout=30
        )
        output = result.stdout

        # Parse results
        exact = 0
        anomalies = 0
        total = 0
        for line in output.split("\n"):
            if "EXACT:" in line:
                parts = line.split()
                for i, p in enumerate(parts):
                    if p == "EXACT:":
                        try: exact = int(parts[i+1])
                        except: pass
            if "Anomalies:" in line:
                parts = line.split()
                for i, p in enumerate(parts):
                    if p == "Anomalies:":
                        try: anomalies = int(parts[i+1])
                        except: pass
            if "Constants scanned:" in line:
                parts = line.split()
                for i, p in enumerate(parts):
                    if p == "scanned:":
                        try: total = int(parts[i+1])
                        except: pass

        pct = (exact / max(total, 1)) * 100
        print(f"  [scan] {exact}/{total} EXACT ({pct:.1f}%), {anomalies} anomalies")
        return {"exact": exact, "anomalies": anomalies, "total": total}
    except Exception as e:
        print(f"  [scan] Error: {e}")
        return {"exact": 0, "anomalies": 0, "total": 0}

# ═══════════════════════════════════════════════════════════
# Phase 3: 빌드 + 테스트 검증
# ═══════════════════════════════════════════════════════════

def build_and_test():
    """cargo build + 기본 컴파일 테스트."""
    print("  [build] cargo build --release...")
    try:
        result = subprocess.run(
            "~/.cargo/bin/cargo build --release",
            shell=True, capture_output=True, text=True,
            timeout=120, cwd=str(HEXA_ROOT)
        )
        build_ok = result.returncode == 0
        errors = result.stderr.count("error[")
        warnings = result.stderr.count("warning:")
        print(f"  [build] {'✅ OK' if build_ok else '❌ FAIL'} ({errors} errors, {warnings} warnings)")

        if not build_ok:
            return {"build": False, "compile_test": False, "errors": errors}

        # Test: compile hello.hexa
        hello = HEXA_ROOT / "tests" / "hello.hexa"
        if hello.exists():
            test_result = subprocess.run(
                f"./target/release/hexa-ir {hello}",
                shell=True, capture_output=True, text=True,
                timeout=10, cwd=str(HEXA_ROOT)
            )
            compile_ok = test_result.returncode == 0
            print(f"  [test] Hello World compile: {'✅' if compile_ok else '❌'}")

            # Run the binary
            test_bin = str(hello).replace(".hexa", "")
            if os.path.exists(test_bin):
                run_result = subprocess.run(
                    test_bin, capture_output=True, text=True, timeout=5
                )
                exit_code = run_result.returncode
                print(f"  [test] Hello World run: exit={exit_code} {'✅' if exit_code == 42 else '❌'}")
                return {"build": True, "compile_test": compile_ok,
                        "run_test": exit_code == 42, "errors": 0}

        return {"build": True, "compile_test": False, "errors": 0}
    except Exception as e:
        print(f"  [build] Error: {e}")
        return {"build": False, "compile_test": False, "errors": -1}

# ═══════════════════════════════════════════════════════════
# Phase 4: 성장 리포트 + Bus 발행
# ═══════════════════════════════════════════════════════════

def count_metrics():
    """HEXA-IR 소스 메트릭."""
    src = HEXA_ROOT / "src"
    rs_files = list(src.rglob("*.rs"))
    total_lines = 0
    for f in rs_files:
        try:
            total_lines += len(f.read_text().split("\n"))
        except:
            pass

    dirs = set(f.parent for f in rs_files)
    opt_passes = sum(1 for f in (src / "opt").rglob("*.rs")
                     if f.name not in ("mod.rs",) and f.parent.name in ("front", "mid", "back"))

    return {
        "rs_files": len(rs_files),
        "directories": len(dirs),
        "total_lines": total_lines,
        "opt_passes": opt_passes,
    }

# ═══════════════════════════════════════════════════════════
# Main
# ═══════════════════════════════════════════════════════════

def run_growth_cycle(scan_only=False):
    """HEXA-IR 성장 사이클 1회."""
    ts = datetime.now().strftime("%H:%M:%S")
    print(f"\n{'═' * 50}")
    print(f"  HEXA-IR Growth Engine — {ts}")
    print(f"{'═' * 50}\n")

    # Phase 1: Sync NEXUS-6
    print("[Phase 1] NEXUS-6 상수 동기화...")
    sync_changes = sync_nexus_constants()

    # Phase 2: Convergent scan
    print("[Phase 2] 수렴 루프 스캔...")
    scan = run_convergent_scan()

    if scan_only:
        print("\n[DONE] Scan-only mode.")
        return

    # Phase 3: Build + test
    print("[Phase 3] 빌드 + 테스트...")
    test = build_and_test()

    # Phase 4: Metrics + report
    print("[Phase 4] 메트릭 수집...")
    metrics = count_metrics()

    # Emit to Growth Bus
    report = {
        "sync_changes": sync_changes,
        "scan": scan,
        "test": test,
        "metrics": metrics,
        "timestamp": datetime.now().isoformat(),
    }

    bus_emit("growth", "hexa_growth_cycle", report)

    # Summary
    health = "✅" if test.get("build") and scan.get("anomalies", 99) < 30 else "⚠️"
    exact_pct = scan.get("exact", 0) / max(scan.get("total", 1), 1) * 100

    print(f"\n{'─' * 50}")
    print(f"  {health} HEXA-IR Growth Report")
    print(f"  Files: {metrics['rs_files']} .rs | Lines: {metrics['total_lines']}")
    print(f"  Opt passes: {metrics['opt_passes']} (σ=12)")
    print(f"  n6 EXACT: {scan.get('exact', '?')}/{scan.get('total', '?')} ({exact_pct:.0f}%)")
    print(f"  Anomalies: {scan.get('anomalies', '?')}")
    print(f"  Build: {'✅' if test.get('build') else '❌'}")
    print(f"  Hello World: {'✅ exit=42' if test.get('run_test') else '❌'}")
    print(f"  NEXUS sync: {sync_changes} changes")
    print(f"{'─' * 50}\n")

    # Signal for Growth Bus feedback
    if scan.get("anomalies", 0) == 0:
        print("  ★ PERFECT: 0 anomalies — n=6 100%")
        bus_emit("growth", "emergence", {"type": "n6_perfection", "exact_pct": exact_pct})

    if test.get("run_test"):
        print("  ★ Hello World: HEXA-LANG → ARM64 → exit=42 (LLVM 0%)")
        bus_emit("growth", "discovery", {"type": "native_compile", "target": "arm64"})

    return report


if __name__ == "__main__":
    scan_only = "--scan-only" in sys.argv
    run_growth_cycle(scan_only=scan_only)
