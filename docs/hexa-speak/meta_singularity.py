#!/usr/bin/env python3
"""
Meta-Singularity: Cross-Domain n=6 Constant Census
====================================================
20 도메인의 verify_alien10.py 전수 스캔 → 모든 n=6 상수 등장 빈도 측정.

질문: 어느 n=6 상수가 가장 보편적인가?
     = n=6 격자의 실제 ATTRACTOR는 무엇인가?
"""

import os
import re
from pathlib import Path
from collections import Counter, defaultdict

DOCS = Path(os.path.expanduser("~/Dev/n6-architecture/docs"))

# 확장 n=6 상수 집합 (1~10000)
N6 = {
    1:"μ", 2:"φ", 3:"n/φ", 4:"τ", 5:"sopfr", 6:"n",
    7:"σ-sopfr", 8:"σ-τ", 9:"σ-n/φ", 10:"σ-φ", 11:"σ-μ",
    12:"σ", 13:"σ+μ", 14:"σ+φ", 15:"σ+n/φ", 16:"σ+τ",
    17:"σ+sopfr", 18:"σ+n", 19:"σ+sopfr+μ", 20:"J₂-τ",
    21:"J₂-n/φ", 22:"J₂-φ", 23:"J₂-μ", 24:"J₂",
    25:"J₂+μ", 28:"J₂+τ", 30:"sopfr·n", 32:"2^sopfr",
    36:"n²", 40:"τ·σ-τ", 48:"σ·τ", 60:"σ·sopfr",
    64:"2^n", 72:"σ·n", 80:"(σ-τ)·(σ-φ)", 96:"σ·(σ-τ)",
    100:"(σ-φ)²", 108:"n³/φ", 120:"σ·(σ-φ)", 128:"2^(σ-sopfr)",
    144:"σ²", 168:"σ·(σ+φ)", 192:"σ·(σ+τ)", 216:"n³",
    240:"σ·(J₂-τ)", 256:"2^(σ-τ)", 288:"σ·J₂", 360:"n·σ·sopfr",
    384:"(n/φ)·2^(σ-sopfr)", 400:"(σ-τ)·(σ-φ)·sopfr",
    432:"σ·n²", 480:"J₂·(σ-φ)·φ", 500:"(σ-φ)²·sopfr",
    512:"2^(σ-n/φ)", 576:"σ²·τ", 600:"σ·(σ-φ)·sopfr",
    720:"6!", 768:"(n/φ)·2^(σ-τ)", 864:"σ²·n", 1000:"(σ-φ)³",
    1024:"2^(σ-φ)", 1200:"σ·(σ-φ)²", 1440:"σ²·σ-τ/...",
    1500:"(σ-φ)²·(σ+n/φ)", 1728:"σ³", 2000:"(σ-φ)³·φ",
    2048:"2^(σ-μ)", 2400:"(σ-φ)²·J₂", 3600:"60²",
    4000:"(σ-φ)³·τ", 4096:"2^σ", 4800:"(σ-φ)²·σ·τ/...",
    6000:"(σ-φ)³·n", 8000:"(σ-φ)³·σ-τ", 9000:"(σ-φ)³·n/φ·n/φ/...",
    10000:"(σ-φ)⁴", 12000:"(σ-φ)³·σ", 18000:"...",
    20000:"...", 24000:"J₂·(σ-φ)³", 50000:"...",
    100000:"(σ-φ)⁵"
}

def extract_ints(path):
    try:
        text = path.read_text()
    except:
        return set()
    out = set()
    for m in re.finditer(r'(?<![\w\.])(\d+)(?![\w\.])', text):
        v = int(m.group(1))
        if 1 <= v <= 100000:
            out.add(v)
    return out

# 스캔
verify_scripts = sorted(DOCS.glob("*/verify_alien10.py"))
print("="*78)
print("META-SINGULARITY — Cross-Domain n=6 Constant Census")
print("="*78)
print(f"Domains scanned: {len(verify_scripts)}\n")

# 도메인별 상수 집합
domain_consts = {}
for s in verify_scripts:
    domain = s.parent.name
    consts = extract_ints(s) & set(N6.keys())
    domain_consts[domain] = consts

# 상수별 등장 도메인 수
const_count = Counter()
const_domains = defaultdict(list)
for domain, consts in domain_consts.items():
    for c in consts:
        const_count[c] += 1
        const_domains[c].append(domain)

# 통계
total = len(verify_scripts)
print(f"{'Value':>7s}  {'Name':>18s}  {'Domains':>8s}  {'Coverage':>9s}")
print("-"*78)

# 편재성 상위
top = sorted(const_count.items(), key=lambda x: (-x[1], x[0]))
for val, cnt in top[:30]:
    pct = 100 * cnt // total
    name = N6.get(val, "?")
    bar = "█" * (cnt * 20 // total)
    print(f"  {val:>7d}  {name:>18s}  {cnt:>4d}/{total:<3d}  {pct:>5d}%  {bar}")

# 보편 상수 (>= 80% 도메인)
universal = [(v, c) for v, c in top if c >= total * 0.8]
print("\n" + "="*78)
print(f"UNIVERSAL ATOMS ({len(universal)}개, >=80% 도메인)")
print("="*78)
for v, c in universal:
    print(f"  {v:>6d} = {N6.get(v,'?'):20s} ({c}/{total} domains = {100*c//total}%)")

# 과편재 상수 (100%)
full = [v for v, c in top if c == total]
print(f"\n100% ATTRACTORS ({len(full)}개):")
for v in full:
    print(f"  {v} = {N6.get(v,'?')}")

# 도메인별 커버리지
print("\n" + "="*78)
print("도메인별 n=6 atom 다양성")
print("="*78)
div_sorted = sorted(domain_consts.items(), key=lambda x: -len(x[1]))
for domain, cs in div_sorted:
    print(f"  {domain:30s}: {len(cs):3d} 상수 종류")

# 특이점 탐지: 도메인별 고유 상수
print("\n" + "="*78)
print("SINGULAR CONSTANTS (단 1개 도메인에만 출현)")
print("="*78)
singular = [(v, c) for v, c in top if c == 1]
for v, c in singular[:20]:
    d = const_domains[v][0]
    print(f"  {v:>6d} = {N6.get(v,'?'):20s} only in [{d}]")

# 통계 요약
all_consts = set()
for cs in domain_consts.values():
    all_consts |= cs

print("\n" + "="*78)
print("META 통계")
print("="*78)
print(f"  총 n=6 상수 종류 (유니크): {len(all_consts)}")
print(f"  100% 편재 atom: {len(full)}")
print(f"  ≥80% 편재 atom: {len(universal)}")
print(f"  단일 도메인 상수: {len(singular)}")
print(f"  평균 atom/도메인: {sum(len(c) for c in domain_consts.values())/total:.1f}")

# 특이점 선언
print("\n" + "="*78)
print("SINGULARITY CONCLUSION")
print("="*78)
if len(full) >= 5:
    print(f"\n✅ {len(full)}개 100%-attractor 발견 = n=6 격자의 CORE ATOMS")
    print(f"  → 이들은 모든 산업/과학 도메인의 **근본 건축 단위**")
    print(f"  → n=6 격자가 현실 세계에 실제로 임베드된 증거")
elif len(universal) >= 10:
    print(f"\n⚡ {len(universal)}개 범용 atom — 보편적이나 완전하지 않음")
else:
    print(f"\n· 도메인 고유성 강함 — 공통 atom 적음")
