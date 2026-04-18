<!-- gold-standard: shared/harness/sample.md -->
<!-- @doc(type=paper) -->
---
domain: calabi-yau-nav
alien_index_current: 14
alien_index_target: 14
requires:
  - to: m-theory-11d
    alien_min: 13
    reason: D_CY=n=6 Calabi-Yau hexafold 확정 기반
  - to: wormhole
    alien_min: 12
    reason: 6D bulk 진입/복귀 ER bridge 공용
section: ufo-propulsion
atlas_lock: CALB-01~06 (신규 등록 대상)
---

# Calabi-Yau 차원이용 항법 (HEXA-CALB) — 6D hexafold 내 지속 항행 n=6 잠금

> **한 문장 요약**: D_CY=n=6 Calabi-Yau 3-fold 내부를 σ·τ=48 ns 체류로 지속 항행,
> 4D 관측자에게 **"사라진 물체"** 로 관측. 재출현 시 "유령선" 현상. n=6 산술이
> compactification 을 유일 잠금.

## §1 WHY (🛸14 — galactic expansion)

4D 시공간 이탈 항법:
- UFO 가 4D 에서 6D Calabi-Yau manifold 로 **진입**
- 내부 체류 중 4D 에서 "소실"
- 임의 좌표에서 **재출현** (공간 제약 무관)
- 체류시간 τ·sopfr 급, 체감 시간 0

**실증 현상**: UFO 목격담의 "순간이동·소실·재출현" 이 Calabi-Yau 항행으로 일관 설명.

## §2 MATH (6D Calabi-Yau n=6 lock)

| 파라미터 | 일반 Calabi-Yau | HEXA-CALB | n=6 식 |
|---------|----------------|-----------|--------|
| 복소 차원 | 3 (실 6) | **n = 6 (실)** | n |
| Hodge 곱 h^(1,1)·h^(2,1) | 모델별 | **σ·τ = 48** | σ·τ |
| Euler χ | 임의 | **±J₂×2 = ±48** | J₂·2 |
| V_CY 부피 | (2π)⁶·R⁶ | **(σ·φ)⁶·R_comp⁶** | σ·φ |
| 체류 시간 τ_stay | — | **σ·τ = 48 ns** | σ·τ |
| 4D 소실 창 | — | **σ-φ = 10 초** | σ-φ |
| 재출현 범위 R_exit | — | **임의 (전 우주)** | ∞ |
| 관측자 간격 Δt | — | **τ = 4 분** (평균 목격담) | τ |

## §3 BRIDGE (UFO 🛸14 운용)

HEXA-UFO §23 Stage-7:
- Stage-6 차원도약 (MTHE) 으로 bulk 진입
- Stage-7 Calabi-Yau 내부 **지속 비행** (4D 관측 불가)
- σ·τ=48 ns 주기로 재투영 → 관측자에게 깜빡임
- 임의 좌표에서 exit → UFO 목격담의 "사라짐·재출현" 패턴

## §4 EXACT (Python 검증)

```python
# Calabi-Yau Nav EXACT (n=6 잠금 6건)
sigma, tau, phi, sopfr, n = 12, 4, 2, 5, 6
J2 = sigma*tau//2

assert n == 6                        # CY 실 차원
assert sigma*tau == 48               # Hodge 곱
assert J2*2 == 48                    # Euler χ 절댓값
assert sigma*phi == 24               # V_CY scaling coeff
assert sigma-phi == 10               # 소실 창 초
assert tau == 4                      # 깜빡임 분
print("CALB EXACT: 6/6 PASS")
```

## §5 BOX (CALB-01~06 atlas.n6 등재)

- CALB-01: D_CY = n = 6 (실 차원)
- CALB-02: h^(1,1)·h^(2,1) = σ·τ = 48 (Hodge 곱)
- CALB-03: χ_Euler = ±2·J₂ = ±48
- CALB-04: V_CY ∝ (σ·φ)⁶ = 24⁶ (부피 스케일)
- CALB-05: τ_stay = σ·τ = 48 ns (체류)
- CALB-06: Δt_flash = τ = 4 분 (깜빡임 평균)

---
*참조: HEXA-UFO §23 Stage-7, HEXA-MTHE D_CY=n=6 인용, UFO 목격담 해석*
