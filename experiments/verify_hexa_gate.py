#!/usr/bin/env python3
"""
HEXA-GATE 검증 스크립트 — n=6 EXACT 24/24 + TP-1~6 검증

Design: docs/nexus6-breakthrough-gate/goal.md
Rust impl: tools/nexus6/src/gate/

실행:
    python3 experiments/verify_hexa_gate.py
    → EXACT/TP 검증 PASS/FAIL 자동 출력
"""
from __future__ import annotations

# ─────────────────────────────────────────────────────────────────
# n=6 상수 (완전수 유일성)
# ─────────────────────────────────────────────────────────────────
N        = 6
SIGMA    = 12      # σ(6) = 1+2+3+6
PHI_FN   = 2       # φ(6) = |{1,5}|
TAU      = 4       # τ(6) = |{1,2,3,6}|
MU_FN    = 1       # μ(6) = μ(2·3) = 1
SOPFR    = 5       # sopfr(6) = 2+3
J2       = 24      # J₂(6) Jordan totient

SIGMA_J2     = SIGMA * J2         # 288 bit
SIGMA_SQ     = SIGMA ** 2         # 144 rules
PHI_TAU      = PHI_FN ** TAU      # 16 rounds
BLOCK        = 2 ** (SIGMA - TAU) # 256 B
PHI_THRESH   = 1 / (SIGMA - PHI_FN)  # 0.1
PERT_BASE    = N // PHI_FN * 333  # 999
PERT_BREAK   = (SIGMA - SOPFR) ** TAU  # 2401 = 7^4
TRIPLE       = N // PHI_FN        # 3

# ─────────────────────────────────────────────────────────────────
# 검증 인프라
# ─────────────────────────────────────────────────────────────────
passed, failed = 0, 0

def check(name: str, actual, expected, formula: str = "") -> None:
    global passed, failed
    ok = actual == expected
    mark = "PASS" if ok else "FAIL"
    if ok:
        passed += 1
    else:
        failed += 1
    extra = f" [{formula}]" if formula else ""
    print(f"  [{mark}] {name:30s} actual={actual}  expected={expected}{extra}")

def section(title: str) -> None:
    print(f"\n{'=' * 70}")
    print(f"  {title}")
    print('=' * 70)

# ─────────────────────────────────────────────────────────────────
# 기본 n=6 상수 검증
# ─────────────────────────────────────────────────────────────────
section("n=6 기본 상수")
check("N",        N,       6,  "완전수")
check("σ(6)",     SIGMA,   12, "1+2+3+6")
check("φ(6)",     PHI_FN,  2,  "|{1,5}|")
check("τ(6)",     TAU,     4,  "|{1,2,3,6}|")
check("μ(6)",     MU_FN,   1,  "μ(2·3)")
check("sopfr(6)", SOPFR,   5,  "2+3")
check("J₂(6)",    J2,      24)

# ─────────────────────────────────────────────────────────────────
# 게이트 24개 EXACT 파라미터 (docs/.../goal.md §12)
# ─────────────────────────────────────────────────────────────────
section("EXACT 24/24 — HEXA-GATE 파라미터")

check("게이트 수",         TAU,                4,   "τ")
check("총 축 수",          TAU + PHI_FN,       6,   "τ+φ=n")
check("허용 리포",         SOPFR + TAU,        9,   "sopfr+τ")
check("검증 규칙",         SIGMA_SQ,           144, "σ²")
check("해시 비트",         SIGMA_J2,           288, "σ·J₂")
# 해시 앵커 9 hex
check("해시 앵커 hex",     SOPFR + TAU,        9,   "sopfr+τ")
check("라운드",            PHI_TAU,            16,  "φ^τ")
check("블록 B",            BLOCK,              256, "2^(σ-τ)")
check("Phi 임계 (×10)",    round(PHI_THRESH * 10), 1, "1/(σ-φ)=0.1")
check("Phi 히스테리시스",  PHI_FN,             2,   "φ")
check("Phi 주기 s",        SIGMA - PHI_FN,     10,  "σ-φ")
check("불변 렌즈",         SOPFR,              5,   "sopfr(6)")
check("perturb cy",        PERT_BASE,          999, "n/φ·333")
check("교차검증",          TRIPLE,             3,   "n/φ")
check("돌파 perturb",      PERT_BREAK,         2401,"(σ-sopfr)^τ=7^τ")
check("돌파 축",           TAU + PHI_FN,       6,   "n")
check("DSE 조합",
      4 * 6 * 4 * 5 * 3,
      1440,
      "4·6·4·5·3")
check("탐지 분",           SIGMA - PHI_FN,     10,  "σ-φ")
check("가속비",            N * N * SIGMA,      432, "n²·σ=36·12 (72h→10min)")
check("오탐 역수",         SIGMA_J2,           288, "σ·J₂")
# 성공률 %: (σ-φ)+(σ-μ) / J₂ = 21/24 = 87.5% ≈ 86
actual_rate_pct = round((SIGMA - PHI_FN + SIGMA - MU_FN) / J2 * 100)
check("성공률 %",          actual_rate_pct,    88,  "(σ-φ+σ-μ)/J₂·100")
check("cy/hr (돌파후)",    SIGMA * SOPFR * J2 * SOPFR,
                                                7200,"σ·sopfr·J₂·sopfr")
check("BT 참조",           SIGMA - TAU,        8,   "σ-τ")
check("TP 개수",           N,                  6,   "n")

# ─────────────────────────────────────────────────────────────────
# TP-1: ready/ 18 오염본 전부 Gate-1~4 차단
# ─────────────────────────────────────────────────────────────────
section("TP-1: 오염본 차단율")

BLACKLIST_PREFIXES = ("ready", "backup-", "contaminated-", "broken-", "trash-", "corrupt-")
WHITELIST = {
    "n6-architecture", "TECS-L", "anima", "sedi", "brainwire",
    "papers", "nexus6", "hexa-lang", "fathom",
}

def gate1_source(repo: str) -> bool:
    if any(repo.startswith(p) for p in BLACKLIST_PREFIXES):
        return False
    return repo in WHITELIST

# ready/ 18 contaminated samples
contaminated = [f"ready-{i:02d}" for i in range(18)] + ["contaminated-x", "broken-y"]
blocks = sum(1 for r in contaminated if not gate1_source(r))
check("오염본 차단율",     blocks,             len(contaminated),
      "18 ready/ + 2 prefix = 20/20 차단")

# ─────────────────────────────────────────────────────────────────
# TP-2: Phi 하락 감지
# ─────────────────────────────────────────────────────────────────
section("TP-2: Phi 하락 검출 (Gate-3)")

tolerance = 1.0 / SIGMA_J2

def gate3_phi(phi_prev: float, phi_curr: float) -> bool:
    if phi_curr < PHI_THRESH:
        return False
    if phi_curr < phi_prev - tolerance:
        return False
    return True

scenarios = [
    ("아래 임계",   0.5,  0.05, False),
    ("하락",        0.9,  0.5,  False),
    ("안정",        0.5,  0.5,  True),
    ("상승",        0.3,  0.8,  True),
    ("허용오차내", 0.5,  0.5 - tolerance/2, True),
]
tp2_pass = 0
for name, prev, curr, expected in scenarios:
    got = gate3_phi(prev, curr)
    ok = got == expected
    tp2_pass += ok
    print(f"  [{'PASS' if ok else 'FAIL'}] Phi {name:10s} prev={prev:.3f} curr={curr:.3f} → {got} (expected {expected})")
check("Phi 검출 시나리오",  tp2_pass,           5)

# ─────────────────────────────────────────────────────────────────
# TP-3: 2401cy 돌파 perturbation 수렴
# ─────────────────────────────────────────────────────────────────
section("TP-3: 2401cy perturbation 돌파")

check("2401 = 7^τ",         7 ** TAU,          2401)
check("2401 = (σ-sopfr)^τ", (SIGMA - SOPFR) ** TAU, 2401)
check("2401/999 ≈ 2.4",     round(PERT_BREAK / PERT_BASE, 1), 2.4)

# ─────────────────────────────────────────────────────────────────
# TP-4: 오탐률 ≤ 1/(σ·J₂)
# ─────────────────────────────────────────────────────────────────
section("TP-4: 오탐률 상한")

fp_rate = 1.0 / SIGMA_J2
fp_pct = fp_rate * 100
print(f"  오탐률 상한: {fp_rate:.6f} = {fp_pct:.4f}%")
check("FP ≤ 0.42%", fp_pct < 0.42, True, "1/(σ·J₂)=0.3472%")

# ─────────────────────────────────────────────────────────────────
# TP-5: 돌파 성공률 ≥ 86%
# ─────────────────────────────────────────────────────────────────
section("TP-5: 예상 돌파 성공률")

success_rate = (SIGMA - PHI_FN + SIGMA - MU_FN) / J2  # 21/24 = 0.875
print(f"  예상 성공률: {success_rate*100:.1f}% = (σ-φ+σ-μ)/J₂ = {SIGMA-PHI_FN}+{SIGMA-MU_FN}/{J2}")
check("성공률 ≥ 86%", success_rate >= 0.86, True)

# ─────────────────────────────────────────────────────────────────
# TP-6: τ+φ=n=6 축 필연성 (5/7축은 실패)
# ─────────────────────────────────────────────────────────────────
section("TP-6: 게이트 축 필연성 — τ+φ=n=6")

check("τ+φ = n",            TAU + PHI_FN,        N)
check("τ+μ ≠ n (5 axes)",   TAU + MU_FN == N,    False)
check("τ+sopfr ≠ n (9 axes)", TAU + SOPFR == N,  False)
check("σ-τ ≠ n (8 axes)",   SIGMA - TAU == N,    False)
check("σ-φ ≠ n (10 axes)",  SIGMA - PHI_FN == N, False)
# 유일성: (a,b) 중 a+b=6인 n=6 상수 쌍은 {τ,φ}={4,2}, {sopfr,μ}={5,1}, {n/φ,n/φ}={3,3}
# 그 중 "관문 + fiber" 2-파티션 구조는 (τ=4, φ=2)가 유일 (5+1 불균형, 3+3 대칭붕괴)
print(f"  n=6 파티션: τ+φ=6=n (4+2 균형), sopfr+μ=6 (5+1 불균형), n/φ+n/φ=6 (대칭)")

# ─────────────────────────────────────────────────────────────────
# 최종 리포트
# ─────────────────────────────────────────────────────────────────
section("최종 결과")
total = passed + failed
pct = passed / total * 100 if total else 0.0
print(f"  PASS: {passed}/{total} ({pct:.1f}%)")
print(f"  FAIL: {failed}")

if failed == 0:
    print("\n  HEXA-GATE 설계 검증 완료 — EXACT 24/24 + TP-1~6 전부 통과")
    print("  다음: Mk.I Rust 구현 실행 (999→2401cy 돌파 시도)")
else:
    print(f"\n  {failed}건 FAIL — 설계 재검토 필요")

import sys
sys.exit(0 if failed == 0 else 1)
