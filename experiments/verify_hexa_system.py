#!/usr/bin/env python3
"""HEXA-SYSTEM 시스템 아키텍처 수학 검증 — 145개 파라미터 전수 검사"""

import sys

# N6 constants
n = 6; phi = 2; tau = 4; sigma = 12; sopfr = 5; mu = 1; J2 = 24; R = 1; P2 = 28

passed = 0; failed = 0; total = 0

def check(cat, name, expected, formula_str, formula_val):
    global passed, failed, total
    total += 1
    ok = expected == formula_val
    if not ok:
        failed += 1
        print(f"  ❌ FAIL [{cat}] {name}: expected={expected}, got={formula_val} ({formula_str})")
    else:
        passed += 1
        print(f"  ✅ PASS [{cat}] {name} = {expected} = {formula_str}")

print("=" * 70)
print("HEXA-SYSTEM Verification")
print("=" * 70)

# === 1. Server Node Architecture (27) ===
print("\n--- 1. Server Node Architecture ---\n")
w = passed
check("CPU", "Sockets", 2, "φ", phi)
check("CPU", "UPI links", 2, "φ", phi)
check("CPU", "UPI bandwidth (GB/s)", 48, "σ·τ", sigma*tau)
check("GPU", "GPUs per node", 8, "σ-τ", sigma-tau)
check("GPU", "NVLink per GPU", 12, "σ", sigma)
check("GPU", "NVLink bandwidth (GB/s)", 120, "σ·sopfr·φ", sigma*sopfr*phi)
check("GPU", "HBM stacks/GPU", 8, "σ-τ", sigma-tau)
check("GPU", "HBM capacity/GPU (GB)", 288, "σ·J₂", sigma*J2)
check("Memory", "DIMMs/socket", 16, "φ^τ", phi**tau)
check("Memory", "Channels/socket", 8, "σ-τ", sigma-tau)
check("Memory", "Total DIMMs", 32, "φ^τ·φ", phi**tau * phi)
check("Memory", "DIMM capacity (GB)", 64, "2^n", 2**n)
check("Memory", "Total DDR5 (GB)", 2048, "2^(n+sopfr)", 2**(n+sopfr))
check("PCIe", "Total lanes", 144, "σ²", sigma**2)
check("PCIe", "Gen6 speed (GT/s)", 48, "σ·τ", sigma*tau)
check("Network", "NIC ports", 4, "τ", tau)
check("Network", "NICs", 2, "φ", phi)
check("Network", "RDMA QPs/port", 8, "σ-τ", sigma-tau)
check("Storage", "NVMe drives", 8, "σ-τ", sigma-tau)
check("Storage", "Capacity/drive (TB)", 8, "σ-τ", sigma-tau)
check("Storage", "Total local (TB)", 64, "2^n", 2**n)
check("Storage", "PCIe/NVMe lanes", 4, "τ", tau)
check("Power", "Node power (kW)", 10, "σ-φ", sigma-phi)
check("Form", "Rack units", 4, "τ", tau)
check("NVLink", "SuperPOD GPUs", 72, "σ·n", sigma*n)
check("NVLink", "NVSwitches", 12, "σ", sigma)
check("Mgmt", "BMC count", 1, "μ", mu)
w1 = passed - w
print(f"\n  Server Node: {w1}/27")

# === 2. Rack Architecture (14) ===
print("\n--- 2. Rack Architecture ---\n")
r = passed
check("Physical", "Rack height (U)", 48, "σ·τ", sigma*tau)
check("Physical", "Cooling zones", 4, "τ", tau)
check("Servers", "GPU servers/rack", 12, "σ", sigma)
check("Servers", "Thin nodes/rack", 24, "J₂", J2)
check("Servers", "Server form factor (U)", 4, "τ", tau)
check("Network", "ToR switches", 2, "φ", phi)
check("Network", "Uplinks per ToR", 12, "σ", sigma)
check("Power", "PDUs per rack", 2, "φ", phi)
check("Power", "GPU rack power (kW)", 120, "σ(σ-φ)", sigma*(sigma-phi))
check("Power", "Mixed rack power (kW)", 48, "σ·τ", sigma*tau)
check("Power", "PDU outlets", 48, "σ·τ", sigma*tau)
check("Power", "Circuit breakers", 12, "σ", sigma)
check("Infra", "Infrastructure overhead (U)", 6, "n", n)
check("Infra", "Management units", 1, "μ", mu)
r1 = passed - r
print(f"\n  Rack: {r1}/14")

# === 3. Network Topology (19) ===
print("\n--- 3. Network Topology ---\n")
nt = passed
check("Topology", "Tiers", 3, "n/φ", n//phi)
check("Topology", "Leaf switches", 576, "σ²·τ", sigma**2 * tau)
check("Topology", "Spine switches", 144, "σ²", sigma**2)
check("Topology", "Superspine switches", 48, "σ·τ", sigma*tau)
check("Switch", "Ports per switch (small)", 24, "J₂", J2)
check("Switch", "Ports per switch (large)", 144, "σ²", sigma**2)
check("Switch", "SerDes per port", 4, "τ", tau)
check("Switch", "Total SerDes/switch", 576, "σ²·τ", sigma**2 * tau)
check("BW", "Spine bandwidth (Tbps)", 48, "σ·τ", sigma*tau)
check("BW", "Port speed (Gbps)", 480, "σ·τ·(σ-φ)", sigma*tau*(sigma-phi))
check("BW", "NVLink BW/link (GB/s)", 120, "σ·sopfr·φ", sigma*sopfr*phi)
check("Latency", "Tiers", 3, "n/φ", n//phi)
check("Latency", "Same-rack (us)", 3, "n/φ", n//phi)
check("Latency", "Cross-rack (us)", 6, "n", n)
check("Latency", "Cross-DC (us)", 12, "σ", sigma)
check("IB", "HCA ports/node", 4, "τ", tau)
check("IB", "HCAs/node", 2, "φ", phi)
check("IB", "Oversubscription pod (ratio)", 1, "μ:μ → μ", mu)
check("IB", "Oversubscription cross (ratio)", 2, "φ:μ → φ", phi)
nt1 = passed - nt
print(f"\n  Network: {nt1}/19")

# === 4. Datacenter Scale (16) ===
print("\n--- 4. Datacenter Scale ---\n")
dc = passed
check("Pod", "Pods per DC", 12, "σ", sigma)
check("Pod", "Racks per pod", 48, "σ·τ", sigma*tau)
check("Pod", "Servers per pod", 576, "σ²·τ", sigma**2 * tau)
check("Pod", "GPUs per pod", 4608, "σ²·τ·(σ-τ)", sigma**2 * tau * (sigma-tau))
check("DC", "Total racks", 576, "σ²·τ", sigma**2 * tau)
check("DC", "Total servers", 6912, "σ³·τ", sigma**3 * tau)
check("DC", "Total GPUs", 55296, "σ³·τ·(σ-τ)", sigma**3 * tau * (sigma-tau))
check("DC", "Frontier-class GPUs", 144000, "σ²·10³", sigma**2 * 1000)
check("Efficiency", "PUE", 1.2, "σ/(σ-φ)", sigma/(sigma-phi))
check("Power", "IT power (MW)", 144, "σ²", sigma**2)
check("Power", "Facility power (MW)", 172.8, "σ³/(σ-φ)", sigma**3 / (sigma-phi))
check("Cooling", "Cooling plants", 4, "τ", tau)
check("Cooling", "UPS modules", 12, "σ", sigma)
check("Cooling", "Generators", 4, "τ", tau)
check("Spine", "Spine switches/pod", 12, "σ", sigma)
check("Spine", "Superspine total", 48, "σ·τ", sigma*tau)
dc1 = passed - dc
print(f"\n  Datacenter: {dc1}/16")

# === 5. Power Distribution (17) ===
print("\n--- 5. Power Distribution ---\n")
pw = passed
check("Voltage", "HV input (kV)", 120, "σ(σ-φ)", sigma*(sigma-phi))
check("Voltage", "Distribution (V)", 480, "σ·τ·(σ-φ)", sigma*tau*(sigma-phi))
check("Voltage", "DC bus (V)", 48, "σ·τ", sigma*tau)
check("Voltage", "Server input (V)", 12, "σ", sigma)
check("Voltage", "Core voltage (V)", 1.2, "σ/(σ-φ)", sigma/(sigma-phi))
check("Voltage", "CPU core (V)", 1.0, "R", R)
check("UPS", "Modules", 12, "σ", sigma)
check("UPS", "Backup time (min)", 12, "φ·n", phi*n)
check("UPS", "Battery cells (series)", 6, "n", n)
check("Generator", "Units", 4, "τ", tau)
check("Generator", "Capacity each (MW)", 48, "σ·τ", sigma*tau)
check("Generator", "Total capacity (MW)", 192, "σ·φ^τ", sigma * phi**tau)
check("Generator", "Fuel reserve (hours)", 48, "σ·τ", sigma*tau)
check("PDU", "Row PDUs", 12, "σ", sigma)
check("PDU", "Rack PDUs", 2, "φ", phi)
check("VRM", "Phases", 12, "σ", sigma)
check("Efficiency", "PUE", 1.2, "σ/(σ-φ)", sigma/(sigma-phi))
pw1 = passed - pw
print(f"\n  Power Distribution: {pw1}/17")

# === 6. Storage Hierarchy (23) ===
print("\n--- 6. Storage Hierarchy ---\n")
st = passed
check("Tiers", "Storage tiers", 4, "τ", tau)
check("HBM", "HBM per GPU (GB)", 288, "σ·J₂", sigma*J2)
check("HBM", "HBM per node (GB)", 2304, "σ·J₂·(σ-τ)", sigma*J2*(sigma-tau))
check("HBM", "HBM BW/GPU (GB/s)", 3456, "σ²·J₂", sigma**2 * J2)
check("SSD", "NVMe per node", 8, "σ-τ", sigma-tau)
check("SSD", "Capacity/drive (TB)", 8, "σ-τ", sigma-tau)
check("SSD", "Total SSD/node (TB)", 64, "2^n", 2**n)
check("SSD", "SSD BW/drive (GB/s)", 7, "σ-sopfr", sigma-sopfr)
check("SSD", "SSD per rack", 96, "(σ-τ)·σ", (sigma-tau)*sigma)
check("HDD", "HDDs per node", 12, "σ", sigma)
check("HDD", "Capacity/drive (TB)", 12, "σ", sigma)
check("HDD", "Total HDD/node (TB)", 144, "σ²", sigma**2)
check("Tape", "DC cold storage (PB)", 4096, "2^σ", 2**sigma)
check("Object", "Replication factor", 3, "n/φ", n//phi)
check("Object", "Erasure data shards", 10, "σ-φ", sigma-phi)
check("Object", "Erasure parity", 4, "τ", tau)
check("Block", "Block size (KB)", 4, "τ", tau)
check("Block", "Chunk size (MB)", 64, "2^n", 2**n)
check("Block", "Stripe width", 8, "σ-τ", sigma-tau)
check("Block", "RAID level", 6, "n", n)
check("FS", "Metadata servers", 3, "n/φ", n//phi)
check("FS", "OSDs per rack", 48, "σ·τ", sigma*tau)
check("FS", "PGs per OSD", 256, "2^(σ-τ)", 2**(sigma-tau))
st1 = passed - st
print(f"\n  Storage: {st1}/23")

# === 7. Software Stack (29) ===
print("\n--- 7. Software Stack ---\n")
sw = passed
check("Stack", "Total layers", 8, "σ-τ", sigma-tau)
check("Model", "d_model", 4096, "2^σ", 2**sigma)
check("Model", "Layers", 32, "2^sopfr", 2**sopfr)
check("Model", "Attention heads", 24, "J₂", J2)
check("Model", "KV heads", 8, "σ-τ", sigma-tau)
check("Model", "Context window", 4096, "2^σ", 2**sigma)
check("Training", "Global batch (tokens)", 48000, "σ·τ·10³", sigma*tau*1000)
check("Training", "Micro batch/GPU", 256, "2^(σ-τ)", 2**(sigma-tau))
check("Training", "Grad accum steps", 8, "σ-τ", sigma-tau)
check("Inference", "top-p", 0.95, "1-1/(J₂-τ)", 1-1/(J2-tau))
check("Inference", "top-k", 48, "σ·τ", sigma*tau)
check("Optim", "Weight decay", 0.1, "1/(σ-φ)", 1/(sigma-phi))
check("Parallel", "Dimensions", 4, "τ", tau)
check("Parallel", "TP degree", 8, "σ-τ", sigma-tau)
check("Parallel", "PP stages", 8, "σ-τ", sigma-tau)
check("Parallel", "PP micro-batches", 16, "φ^τ", phi**tau)
check("MoE", "Total experts", 256, "2^(σ-τ)", 2**(sigma-tau))
check("MoE", "Active experts", 8, "σ-τ", sigma-tau)
# 1/32 as a float
check("MoE", "Activation ratio", 1/32, "1/2^sopfr", 1/2**sopfr)
check("Container", "Containers/node", 64, "2^n", 2**n)
check("Container", "GPU slices", 8, "σ-τ", sigma-tau)
check("K8s", "Nodes/cluster", 576, "σ²·τ", sigma**2 * tau)
check("K8s", "Pods/node (max)", 64, "2^n", 2**n)
check("K8s", "Services/ns", 4096, "2^σ", 2**sigma)
check("K8s", "Namespaces", 12, "σ", sigma)
check("Memory", "Page size (KB)", 4, "τ", tau)
check("Memory", "Huge page (MB)", 2, "φ", phi)
check("Precision", "FP8 bits", 8, "σ-τ", sigma-tau)
check("Precision", "FP16 bits", 16, "φ^τ", phi**tau)
sw1 = passed - sw
print(f"\n  Software Stack: {sw1}/29")

# === Summary ===
print("\n" + "=" * 70)
print("HEXA-SYSTEM VERIFICATION SUMMARY")
print("=" * 70)
print(f"  Server Node:          {w1}/27")
print(f"  Rack Architecture:    {r1}/14")
print(f"  Network Topology:     {nt1}/19")
print(f"  Datacenter Scale:     {dc1}/16")
print(f"  Power Distribution:   {pw1}/17")
print(f"  Storage Hierarchy:    {st1}/23")
print(f"  Software Stack:       {sw1}/29")
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
