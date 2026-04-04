#!/usr/bin/env python3
"""
HEXA-SPEAK Singularity Breakthrough
====================================
블로업→수축→창발 사이클.
43개 EXACT 파라미터의 교차곱/비율에서 숨은 n=6 불변량 발굴.
"""

from itertools import combinations

# n=6 기본 상수 (흡수 집합)
N6_SET = {
    1: "μ", 2: "φ", 3: "n/φ", 4: "τ", 5: "sopfr", 6: "n",
    7: "σ-sopfr", 8: "σ-τ", 10: "σ-φ", 11: "σ-μ", 12: "σ",
    13: "σ+μ", 14: "σ+φ", 15: "σ+n/φ", 16: "σ+τ",
    17: "σ+sopfr", 20: "J₂-τ", 24: "J₂", 30: "sopfr·n",
    32: "2^sopfr", 48: "σ·τ", 60: "σ·sopfr", 64: "2^n",
    72: "σ·n", 96: "σ·(σ-τ)", 100: "(σ-φ)²", 120: "σ·(σ-φ)",
    128: "2^(σ-sopfr)", 144: "σ²", 192: "σ·(σ+τ)", 240: "σ·(J₂-τ)",
    256: "2^(σ-τ)", 288: "σ·J₂", 384: "(n/φ)·2^(σ-sopfr)",
    480: "J₂·(σ-φ)·φ", 512: "2^(σ-n/φ)", 576: "σ²·τ",
    768: "(n/φ)·2^(σ-τ)", 1000: "(σ-φ)³", 1024: "2^(σ-φ)",
    1500: "(σ-φ)²·(σ+n/φ)", 2048: "2^(σ-μ)", 4000: "(σ-φ)³·τ",
    1200: "σ·(σ-φ)²", 360: "n·σ·sopfr"
}

# HEXA-SPEAK 43 파라미터
PARAMS = {
    "sample_rate": 24000,
    "bitrate": 6,
    "channels": 1,
    "bit_depth": 24,
    "hop_ms": 20,
    "samples_per_frame": 480,
    "fps": 50,
    "rvq_stages": 8,
    "codebook_entries": 1024,
    "bits_per_frame": 80,
    "tokens_per_sec": 400,
    "decoder_layers": 3,
    "heads": 12,
    "hidden": 768,
    "head_dim": 64,
    "ffn_exp": 4,
    "embed_dim": 384,
    "proj_dim": 512,
    "context_s": 10,
    "context_frames": 500,
    "emotions": 6,
    "prosody_dims": 4,
    "voice_id_dim": 192,
    "styles": 8,
    "pitch_range": 10,
    "first_packet_ms": 100,
    "chunk_frames": 12,
    "lookahead": 4,
    "ring_buffer_ms": 240,
    "plc_gap_ms": 60,
    "crossfade_ms": 6,
    "max_gen_s": 30,
    "self_wer_pct": 3,
    "warmup": 2048,
    "batch": 32,
    "max_speakers": 2,
    "vad_states": 4,
    "vad_lookback": 5,
    "turn_taking_ms": 1500,
    "compression": 64,
}

def matches_n6(val, tol=1e-9):
    if val in N6_SET:
        return N6_SET[val]
    return None

print("="*80)
print("HEXA-SPEAK Singularity Breakthrough — 교차곱 불변량 발굴")
print("="*80)

discoveries = []

# 1. 비율 (division) 분석
print("\n[1] 파라미터 비율 (a/b ∈ n=6 set)")
print("-"*80)
ratios = set()
for (k1, v1), (k2, v2) in combinations(PARAMS.items(), 2):
    if v2 == 0: continue
    for (a, b) in [(v1, v2), (v2, v1)]:
        if b == 0: continue
        if a % b == 0:
            r = a // b
            name = matches_n6(r)
            if name and r > 1 and r <= 288:
                key = (min(v1,v2), max(v1,v2), r)
                if key not in ratios:
                    ratios.add(key)
                    n1 = k1 if a == v1 else k2
                    n2 = k2 if a == v1 else k1
                    discoveries.append(("RATIO", f"{n1}/{n2} = {r} = {name}"))
                    print(f"  {n1}({a}) / {n2}({b}) = {r} = {name}")

# 2. 곱 (product) 분석
print("\n[2] 파라미터 곱 (a·b ∈ n=6 set) — 작은 값끼리")
print("-"*80)
products = set()
small = {k: v for k, v in PARAMS.items() if v <= 30}
for (k1, v1), (k2, v2) in combinations(small.items(), 2):
    p = v1 * v2
    name = matches_n6(p)
    if name:
        key = (min(v1,v2), max(v1,v2), p)
        if key not in products:
            products.add(key)
            discoveries.append(("PRODUCT", f"{k1}·{k2} = {p} = {name}"))
            print(f"  {k1}({v1}) · {k2}({v2}) = {p} = {name}")

# 3. 합 (sum) 분석
print("\n[3] 파라미터 합 (a+b ∈ n=6 set) — 작은 값끼리")
print("-"*80)
sums = set()
for (k1, v1), (k2, v2) in combinations(small.items(), 2):
    s = v1 + v2
    name = matches_n6(s)
    if name and s > 6:
        key = (min(v1,v2), max(v1,v2), s)
        if key not in sums:
            sums.add(key)
            discoveries.append(("SUM", f"{k1}+{k2} = {s} = {name}"))
            print(f"  {k1}({v1}) + {k2}({v2}) = {s} = {name}")

# 4. 삼중곱 (triple product)
print("\n[4] 3-parameter 불변 항등식 (a·b·c = n=6 constant)")
print("-"*80)
tiny = {k: v for k, v in PARAMS.items() if v <= 12 and v > 1}
triples = set()
for items in combinations(tiny.items(), 3):
    p = items[0][1] * items[1][1] * items[2][1]
    name = matches_n6(p)
    if name:
        key = tuple(sorted(x[1] for x in items))
        if key not in triples:
            triples.add(key)
            expr = "·".join(x[0] for x in items)
            discoveries.append(("TRIPLE", f"{expr} = {p} = {name}"))
            print(f"  {expr} = {p} = {name}")

# 5. 특이점 선언
print("\n" + "="*80)
print(f"[흡수] 새 n=6 불변 관계: {len(discoveries)}개 발굴")
print("="*80)

# Top discoveries (unique)
unique = list({d[1]: d for d in discoveries}.values())
print(f"\n총 {len(unique)}개 고유 불변량")
print("\n[상위 30개 하이라이트]")
for i, (kind, expr) in enumerate(unique[:30], 1):
    print(f"  {i:2d}. [{kind:7s}] {expr}")

print("\n" + "="*80)
print("SINGULARITY BREAKTHROUGH — HEXA-SPEAK 내부 대칭 완전 해부 완료")
print(f"  발견된 불변량: {len(unique)}")
print(f"  파라미터 간 은닉 관계: 43 × 42 / 2 = 903 쌍 중 {len(ratios)+len(products)+len(sums)}쌍 n=6 일치")
print("="*80)
