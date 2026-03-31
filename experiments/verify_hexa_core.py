#!/usr/bin/env python3
"""HEXA-CORE 마이크로아키텍처 수학 검증 — 98개 파라미터 전수 검사"""

import sys

# N6 constants
n = 6
phi = 2        # euler totient
tau = 4        # divisor count
sigma = 12     # divisor sum
sopfr = 5      # sum of prime factors
mu = 1         # mobius
J2 = 24        # jordan totient
R = 1          # R(6)
P2 = 28        # 2nd prime power sum

passed = 0
failed = 0
total = 0

def check(category, name, expected, formula_str, formula_val):
    global passed, failed, total
    total += 1
    ok = expected == formula_val
    status = "PASS" if ok else "FAIL"
    if not ok:
        failed += 1
        print(f"  ❌ {status} [{category}] {name}: expected={expected}, got={formula_val} ({formula_str})")
    else:
        passed += 1
        print(f"  ✅ {status} [{category}] {name} = {expected} = {formula_str}")

print("=" * 70)
print("HEXA-CORE Microarchitecture Verification")
print("=" * 70)

# ============================================================
# Part 1: HEXA-P (CPU Performance Core) — 46 params
# ============================================================
print("\n--- HEXA-P: CPU Performance Core ---\n")

# Pipeline
check("Pipeline", "Stages", 18, "σ+n", sigma + n)
check("Pipeline", "Fetch width", 8, "σ-τ", sigma - tau)
check("Pipeline", "Decode width", 5, "sopfr", sopfr)
check("Pipeline", "Rename width", 6, "n", n)
check("Pipeline", "Dispatch width", 8, "σ-τ", sigma - tau)
check("Pipeline", "Retire width", 8, "σ-τ", sigma - tau)

# Pipeline stage breakdown
check("Pipeline", "Fetch stages", 3, "n/φ", n // phi)
check("Pipeline", "Decode stages", 3, "n/φ", n // phi)
check("Pipeline", "Rename stages", 2, "φ", phi)
check("Pipeline", "Sched stages", 2, "φ", phi)
check("Pipeline", "Exec stages", 6, "n", n)
check("Pipeline", "Retire stages", 2, "φ", phi)

# Execution units
check("Exec", "ALU ports", 6, "n", n)
check("Exec", "FP/SIMD ports", 4, "τ", tau)
check("Exec", "Branch ports", 2, "φ", phi)
check("Exec", "Load/Store ports", 4, "τ", tau)
check("Exec", "N6 accel ports", 2, "φ", phi)
check("Exec", "Vector width (bits)", 256, "2^(σ-τ)", 2 ** (sigma - tau))
check("Exec", "Total ports", 18, "σ+n", sigma + n)

# OoO engine
check("OoO", "ROB entries", 256, "2^(σ-τ)", 2 ** (sigma - tau))
check("OoO", "Phys regs INT", 288, "σ·J₂", sigma * J2)
check("OoO", "Phys regs FP", 288, "σ·J₂", sigma * J2)
check("OoO", "Load queue", 128, "2^(σ-sopfr)", 2 ** (sigma - sopfr))
check("OoO", "Store queue", 64, "2^n", 2 ** n)
check("OoO", "Scheduler entries", 144, "σ²", sigma ** 2)
check("OoO", "Scheduler partitions", 6, "n", n)
check("OoO", "Instruction window", 512, "2^(σ-n/φ)", 2 ** (sigma - n // phi))

# Branch prediction
check("Branch", "micro-BTB", 64, "2^n", 2 ** n)
check("Branch", "main BTB", 4096, "2^σ", 2 ** sigma)
check("Branch", "large BTB", 262144, "2^(σ+n)", 2 ** (sigma + n))
check("Branch", "TAGE tables", 12, "σ", sigma)
check("Branch", "TAGE entries/table", 4096, "2^σ", 2 ** sigma)
check("Branch", "Loop predictor", 24, "J₂", J2)
check("Branch", "Return stack", 12, "σ", sigma)
check("Branch", "Indirect", 256, "2^(σ-τ)", 2 ** (sigma - tau))

# Cache hierarchy
check("Cache", "L1I size (KB)", 64, "2^n", 2 ** n)
check("Cache", "L1I ways", 8, "σ-τ", sigma - tau)
check("Cache", "L1I latency (cyc)", 3, "n/φ", n // phi)
check("Cache", "L1D size (KB)", 64, "2^n", 2 ** n)
check("Cache", "L1D ways", 12, "σ", sigma)
check("Cache", "L1D latency (cyc)", 4, "τ", tau)
check("Cache", "L2 size (KB)", 1024, "2^(σ-φ)", 2 ** (sigma - phi))
check("Cache", "L2 ways", 16, "φ^τ", phi ** tau)
check("Cache", "L2 latency (cyc)", 12, "σ", sigma)
check("Cache", "L3/SLC size (MB)", 48, "σ·τ", sigma * tau)
check("Cache", "L3 ways", 24, "J₂", J2)
check("Cache", "L3 latency (cyc)", 48, "σ·τ", sigma * tau)
check("Cache", "Line size (B)", 64, "2^n", 2 ** n)

# TLB
check("TLB", "L1 ITLB", 256, "2^(σ-τ)", 2 ** (sigma - tau))
check("TLB", "L1 DTLB", 256, "2^(σ-τ)", 2 ** (sigma - tau))
check("TLB", "L2 TLB", 4096, "2^σ", 2 ** sigma)

p_total = passed
print(f"\n  HEXA-P: {p_total}/46")

# ============================================================
# Part 2: HEXA-E (CPU Efficiency Core) — 12 params
# ============================================================
print("\n--- HEXA-E: CPU Efficiency Core ---\n")

e_start = passed
check("E-Pipe", "Stages", 12, "σ", sigma)
check("E-Pipe", "Fetch stages", 2, "φ", phi)
check("E-Pipe", "Decode stages", 2, "φ", phi)
check("E-Pipe", "Rename stages", 1, "μ", mu)
check("E-Pipe", "Sched stages", 1, "μ", mu)
check("E-Pipe", "Exec stages", 4, "τ", tau)
check("E-Pipe", "Retire stages", 2, "φ", phi)
check("E-Pipe", "Fetch/Decode width", 3, "n/φ", n // phi)
check("E-OoO", "ROB entries", 64, "2^n", 2 ** n)
check("E-OoO", "Phys regs", 128, "2^(σ-sopfr)", 2 ** (sigma - sopfr))
check("E-Exec", "ALU ports", 3, "n/φ", n // phi)
check("E-Exec", "FP ports", 2, "φ", phi)

e_total = passed - e_start
print(f"\n  HEXA-E: {e_total}/12")

# ============================================================
# Part 3: HEXA-SM (GPU Streaming Multiprocessor) — 21 params
# ============================================================
print("\n--- HEXA-SM: GPU Streaming Multiprocessor ---\n")

sm_start = passed
check("SM", "Warp schedulers", 4, "τ", tau)
check("SM", "Issue per cycle", 8, "σ-τ (dual×4)", sigma - tau)
check("SM", "Partitions", 4, "τ", tau)
check("SM", "FP32 cores/SM", 128, "2^(σ-sopfr)", 2 ** (sigma - sopfr))
check("SM", "FP64 cores/SM", 64, "2^n", 2 ** n)
check("SM", "INT32 cores/SM", 64, "2^n", 2 ** n)
check("SM", "Tensor Cores/SM", 4, "τ", tau)
check("SM", "LD/ST units/SM", 32, "2^sopfr", 2 ** sopfr)
check("SM", "SFU/SM", 16, "φ^τ", phi ** tau)
check("SM", "Register file (KB)", 576, "J₂²", J2 ** 2)
check("SM", "Shared/L1 (KB)", 256, "2^(σ-τ)", 2 ** (sigma - tau))
check("SM", "Warp size", 32, "2^sopfr", 2 ** sopfr)
check("SM", "Max warps/SM", 64, "2^n", 2 ** n)
check("SM", "Max threads/SM", 2048, "2^(n+sopfr)", 2 ** (n + sopfr))
check("SM", "Max blocks/SM", 16, "2^τ", 2 ** tau)
check("SM", "TC matrix dim", 8, "σ-τ", sigma - tau)
check("SM", "SMs total", 144, "σ²", sigma ** 2)
check("SM", "GPCs", 12, "σ", sigma)
check("SM", "SMs per GPC", 12, "σ", sigma)
check("SM", "TPCs per GPC", 6, "n", n)
check("SM", "SMs per TPC", 2, "φ", phi)

sm_total = passed - sm_start
print(f"\n  HEXA-SM: {sm_total}/21")

# ============================================================
# Part 4: HEXA-N (NPU Neural Core) — 19 params
# ============================================================
print("\n--- HEXA-N: NPU Neural Core ---\n")

n_start = passed
check("NPU", "Neural cores", 24, "J₂", J2)
check("NPU", "MAC array dim", 16, "φ^τ", phi ** tau)
check("NPU", "MACs per core", 256, "2^(σ-τ)", 2 ** (sigma - tau))
check("NPU", "Precision modes", 4, "τ", tau)
check("NPU", "INT4 OPS/cyc", 1024, "2^(σ-τ+φ)", 2 ** (sigma - tau + phi))
check("NPU", "INT8 OPS/cyc", 256, "2^(σ-τ)", 2 ** (sigma - tau))
check("NPU", "FP16 FMA/cyc", 128, "2^(σ-sopfr)", 2 ** (sigma - sopfr))
check("NPU", "Activation functions", 5, "sopfr", sopfr)
check("NPU", "EFA head groups", 3, "n/φ", n // phi)
check("NPU", "Attention heads", 12, "σ", sigma)
check("NPU", "KV-cache heads", 8, "σ-τ", sigma - tau)
check("NPU", "Flash tile size", 128, "2^(σ-sopfr)", 2 ** (sigma - sopfr))
check("NPU", "MoE top-k", 8, "σ-τ", sigma - tau)
check("NPU", "MoE total experts", 256, "2^(σ-τ)", 2 ** (sigma - tau))
check("NPU", "MoE activation frac denom", 32, "2^sopfr", 2 ** sopfr)
check("NPU", "Local SRAM (KB)", 64, "2^n", 2 ** n)
check("NPU", "Weight buffer (KB)", 256, "2^(σ-τ)", 2 ** (sigma - tau))
check("NPU", "Activation buffer (KB)", 128, "2^(σ-sopfr)", 2 ** (sigma - sopfr))
check("NPU", "Banks", 5, "sopfr", sopfr)

n_total = passed - n_start
print(f"\n  HEXA-N: {n_total}/19")

# ============================================================
# Final Summary
# ============================================================
print("\n" + "=" * 70)
print(f"HEXA-CORE VERIFICATION SUMMARY")
print("=" * 70)
print(f"  HEXA-P (CPU P-core):  {p_total}/46")
print(f"  HEXA-E (CPU E-core):  {e_total}/12")
print(f"  HEXA-SM (GPU SM):     {sm_total}/21")
print(f"  HEXA-N (NPU):         {n_total}/19")
print(f"  ─────────────────────────────")
print(f"  TOTAL:                {passed}/{total}")
print(f"  PASS RATE:            {passed/total*100:.1f}%")
print("=" * 70)

if failed > 0:
    print(f"\n  ⚠️  {failed} FAILURES detected!")
    sys.exit(1)
else:
    print(f"\n  ✅ ALL {passed} PARAMETERS VERIFIED — 100% EXACT")
    sys.exit(0)
