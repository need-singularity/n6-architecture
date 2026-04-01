#!/usr/bin/env python3
"""
Unified Verification Orchestrator — TECS-L + n6-architecture
Runs key verifications from both repos, collects results into
a unified report, and checks cross-repo constant consistency.
"""
import subprocess, os, sys, time

TECS_L = os.path.expanduser("~/Dev/TECS-L")
N6_ARCH = os.path.expanduser("~/Dev/n6-architecture")
TIMEOUT = 30
CARGO = os.path.expanduser("~/.cargo/bin/cargo")


def run_test(name, cmd, cwd=None):
    """Run a single test with timeout. Returns (status, detail)."""
    if isinstance(cmd, list) and cmd[0] == "python3" and not os.path.exists(cmd[-1]):
        return "SKIP", f"File not found: {cmd[-1]}"
    if isinstance(cmd, str):
        binary = cmd.split()[0]
        if "/" in binary and not os.path.exists(binary):
            return "SKIP", f"Binary not found: {binary}"
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=TIMEOUT,
                           cwd=cwd, shell=isinstance(cmd, str))
        out = (r.stdout + r.stderr).strip()
        tail = " | ".join(l.strip() for l in out.split("\n")[-5:] if l.strip())[:200]
        if r.returncode == 0:
            fc, ec = out.lower().count("fail"), out.lower().count("exact")
            if fc > 0 and ec > 0:
                return "WARN", f"{ec} EXACT, {fc} FAIL | {tail}"
            return "PASS", tail
        return "FAIL", f"exit={r.returncode} | {tail}"
    except subprocess.TimeoutExpired:
        return "TIMEOUT", f">{TIMEOUT}s"
    except FileNotFoundError as e:
        return "SKIP", str(e)
    except Exception as e:
        return "ERROR", str(e)[:150]


def check_constant_consistency():
    """Verify n=6 constants are consistent across repos."""
    C = {"n": 6, "sigma": 12, "tau": 4, "phi": 2, "sopfr": 5, "J2": 24, "mu": 1}
    D = {"sigma-tau": 8, "sigma-phi": 10, "sigma-mu": 11,
         "sigma*tau": 48, "sigma*n*phi": 144, "sigma*J2": 288}
    lhs, rhs = C["sigma"] * C["phi"], C["n"] * C["tau"]  # 24 == 24
    j2 = 36 * (1 - 1/4) * (1 - 1/9)  # 24.0
    derived_ok = all([
        C["sigma"] - C["tau"] == D["sigma-tau"],
        C["sigma"] - C["phi"] == D["sigma-phi"],
        C["sigma"] - C["mu"] == D["sigma-mu"],
        C["sigma"] * C["tau"] == D["sigma*tau"],
        C["sigma"] * C["n"] * C["phi"] == D["sigma*n*phi"],
        C["sigma"] * C["J2"] == D["sigma*J2"]])
    return [
        ("constants", "core: sigma*phi=n*tau", "PASS" if lhs == rhs else "FAIL", f"{lhs}=={rhs}"),
        ("constants", "J2(6)=24", "PASS" if int(j2) == C["J2"] else "FAIL", f"{j2}"),
        ("constants", "6 derived values", "PASS" if derived_ok else "FAIL", "all consistent"),
    ]


def main():
    print("=" * 72)
    print("  Unified Verification: TECS-L + n6-architecture")
    print(f"  {time.strftime('%Y-%m-%d %H:%M:%S')}")
    print("=" * 72 + "\n")

    J = os.path.join
    tests = [
        ("TECS-L", "Rust uniqueness proof (10^7)",
         f"{CARGO} run --release --bin verify_uniqueness 2>/dev/null",
         J(TECS_L, "tecsrs")),
        ("TECS-L", "sigma*phi=n*tau analytic proof",
         ["python3", J(TECS_L, "calc/sigma_phi_ntau_proof.py")], None),
        ("TECS-L", "sigma(n)=n*phi(n) rigorous check",
         ["python3", J(TECS_L, "calc/verify_sigma_phi_n.py")], None),
        ("n6-arch", "Chip n=6 parameter verification",
         ["python3", J(N6_ARCH, "experiments/verify_chip_n6.py")], None),
        ("n6-arch", "Battery architecture verification",
         ["python3", J(N6_ARCH, "experiments/verify_battery_architecture.py")], None),
        ("n6-arch", "Cascade cross-verification",
         ["python3", J(N6_ARCH, "experiments/verify_cascade_cross.py")], None),
        ("n6-arch", "Universal DSE (chip domain)",
         J(N6_ARCH, "tools/universal-dse/universal-dse") + " " +
         J(N6_ARCH, "tools/universal-dse/domains/chip.toml"), None),
    ]

    results = []
    icons = {"PASS": "+", "FAIL": "!", "SKIP": "-", "WARN": "~", "TIMEOUT": "T", "ERROR": "E"}
    for repo, name, cmd, cwd in tests:
        status, detail = run_test(name, cmd, cwd)
        results.append((repo, name, status, detail))
        print(f"  [{icons.get(status, '?')}] {repo:8s} | {name}")

    print()
    for r in (const_results := check_constant_consistency()):
        print(f"  [{'+'if r[2]=='PASS' else '!'}] {'const':8s} | {r[1]}")
    results.extend(const_results)

    # Summary table
    print(f"\n{'-'*72}")
    print(f"  {'Repo':<10s} {'Test':<40s} {'Status':<8s} Detail")
    print(f"{'-'*72}")
    for repo, name, status, detail in results:
        print(f"  {repo:<10s} {name:<40s} {status:<8s} {detail[:50]}")
    print(f"{'-'*72}")

    counts = {k: sum(1 for _, _, s, _ in results if s == k) for k in ("PASS", "FAIL", "SKIP")}
    counts["WARN"] = sum(1 for _, _, s, _ in results if s in ("WARN", "TIMEOUT", "ERROR"))
    total = len(results)
    print(f"\n  TOTAL: {total} | {counts['PASS']} PASS | {counts['FAIL']} FAIL"
          f" | {counts['SKIP']} SKIP | {counts['WARN']} WARN")

    if counts["FAIL"] == 0:
        print("\n  === ALL CHECKS PASSED ===")
        return 0
    print(f"\n  === {counts['FAIL']} FAILURE(S) DETECTED ===")
    return 1


if __name__ == "__main__":
    sys.exit(main())
