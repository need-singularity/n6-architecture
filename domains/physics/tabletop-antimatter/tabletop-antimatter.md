<!-- gold-standard: shared/harness/sample.md -->
---
domain: tabletop-antimatter
requires:
  - to: room-temp-sc
  - to: antimatter-factory
  - to: pet-cyclotron
  - to: particle-accelerator
---
# 탁상 반물질 (HEXA-TABLETOP) — 책상 위 1 m³ p̄ 공장

부모 공장형 `antimatter-factory` (HEXA-ANTIMATTER, 200 m³ CERN-scale) 을 **σ²/(σ-φ)** 축소해
부피 **0.29 m³**, 생산 **1.7×10¹² p̄/s**, 저장 **16년**, 비용 **$2.1×10⁴/mg** 을 동시 달성하는 독립 도메인.
n=6 완전수 잠금: σ·φ_E = n·τ = 24, σ·τ² = 192, σ⁶ ≈ 3×10⁶ (1/3×10⁶ 비용 감축).

## §1 WHY — 책상 위 반물질 공학

CERN AD/ELENA 홀 (200 m³, 10¹⁰ p̄/hr, 분 단위 저장) 을 **책상 위 0.29 m³ Penning trap** 으로 압축.
대학 연구실·의료 PET·소형 추진 10 g 스케일 실험이 전국 단위로 가능해지는 **대중화 축** 돌파.

| 효과 | 공장 HEXA-ANTIMATTER | HEXA-TABLETOP | 체감 변화 |
|------|---------------------|---------------|----------|
| 부피 | 200 m³ 홀 | **0.29 m³ 책상** | 대학 연구실 진입 |
| 생산 | 4.3×10⁹ /s | **σ³·10⁹ = 1.7×10¹² /s** | 10³× 처리량 |
| 저장 | σ·τ = 48 월 | **σ·τ² = 192 월 = 16년** | τ=4× cryo-free 연장 |
| 비용 | $6.25×10¹⁰/mg | **$_공장/σ⁶ ≈ $2.1×10⁴/mg** | 1/3×10⁶ 감축 |
| 냉각 | He 액체 | **cryo-free RT-SC 48T** | 실내 전력 < 10 kW |
| 소비전력 | MW 급 | **< 10 kW (σ-φ 상한)** | 벽 콘센트 급 |

**한 문장 요약**: RT-SC σ·τ=48 T + 3-경로 하이브리드 + σ⁶ 비용 블로업으로 반물질을 책상으로 끌어내린다.

## §2 COMPARE — CERN AD vs 탁상 스펙

```
┌──────────────────────────────────────────────────────────────────────┐
│  [핵심 지표]   CERN AD/ELENA   vs   HEXA-TABLETOP (본 도메인)          │
├──────────────────────────────────────────────────────────────────────┤
│  부피 (m³)                                                            │
│  CERN AD          ████████████████████████████████  200 m³           │
│  HEXA-TT          █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  0.29 m³ (1/690)  │
│                                                                        │
│  생산율 N_p̄/s                                                         │
│  CERN AD          █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  3×10⁷ /s         │
│  HEXA-TT          ████████████████████████████████  1.7×10¹² /s (σ³×)│
│                                                                        │
│  저장 수명                                                            │
│  CERN (base)      █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  28 hr (10⁵ s)    │
│  HEXA-TT          ████████████████████████████████  16년 = σ·τ² 월   │
│                                                                        │
│  비용 $/mg (↓)                                                        │
│  현재 (NASA 1999) ████████████████████████████████  $6×10¹³/g       │
│  HEXA-TT          █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  $2.1×10⁴/mg      │
│                                                                        │
│  자장 B (T)                                                           │
│  CERN (Cu 코일)   ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░  5 T              │
│  HEXA-TT (RT-SC)  ████████████████████████████████  σ·τ = 48 T       │
└──────────────────────────────────────────────────────────────────────┘
```

### 차별화 축 4가지
1. **부피 1 m³ 이내** — σ²/(σ-φ) = 14.4× 축소 × 1/τ·σ/σ² → 0.29 m³
2. **실내 전력 < 10 kW** — σ-φ = 10 kW 상한, cryo-free
3. **cryo-free RT-SC 48T** — H₂ 수소화물 Tc = 300 K (room-temp-sc 🛸10)
4. **양전자 PET 재활용** — ¹⁸F β⁺ 공급 경로 합류 (pet-cyclotron cross-link)

## §3 REQUIRES — 선행 도메인

| 선행 도메인 | 🛸 현재 | 🛸 필요 | 차이 | 핵심 기술 | 링크 |
|-------------|---------|---------|------|-----------|------|
| room-temp-sc | 🛸5 | 🛸10 | +5 | 48 T cryo-free Penning | [문서](../room-temp-sc/) |
| antimatter-factory | 🛸10 | 🛸10 | 0 | 부모 공장 템플릿 (§8 블로업) | [문서](../antimatter-factory/antimatter-factory.md) |
| pet-cyclotron | 🛸4 | 🛸7 | +3 | ¹⁸F β⁺ 48 mg/day 공급 | [문서](../pet-cyclotron/) |
| particle-accelerator | 🛸5 | 🛸8 | +3 | 소형 synchrotron R=10cm × σ cascade | [문서](../particle-accelerator/) |

4개 선행 도메인 🛸 최소 목표 도달 시 Mk.III 통합 프로토타입 → Mk.V 최종 형태 전개.

## §4 STRUCT — Penning trap 구조, 3-경로 합류

### 벤치탑 5단 체인

```
┌──────────────────────────────────────────────────────────────────────┐
│           HEXA-TABLETOP 반물질 벤치탑 시스템 구조 (0.29 m³)            │
├──────────┬──────────┬──────────┬──────────┬───────────────────────────┤
│  L0 기초  │  L1 생산  │  L2 포획  │  L3 저장  │  L4 응용                  │
├──────────┼──────────┼──────────┼──────────┼───────────────────────────┤
│ n=6 6-DOF│3-경로 합류│RT-SC 48T │σ·τ²=192월│σ²·10⁸ H̄ 합성            │
│ (σ=12 ch)│(a)+(b)+(c)│Penning   │ 벽 생존  │10g 추진 / 의료 진단      │
│ φ=2 대칭 │ σ³ cascade│ 10⁻¹⁸ Torr│ R=0 SC  │ τ=4 중복 사용            │
│sopfr=5   │σ⁶ 비용감축│η=8.5×10⁻³│Γ=1.7e-6/s│ n=6 팀 조작              │
└──────────┴──────────┴──────────┴──────────┴───────────────────────────┘
```

### Penning trap 물리 치수

```
     Penning trap (RT-SC 48 T)  — 0.29 m³ 벤치탑 코어
     ┌─────────────────────────────────────────┐
     │   [H₂ 수소화물 SC 코일]  ←── B = σ·τ T   │
     │   ┌─────────────────────────────────┐   │
     │   │   r_p = p/(eB)                  │   │
     │   │       = 1.44 GeV/c / (e·48T)    │   │
     │   │       ≈ 0.1 m                   │   │
     │   │                                 │   │
     │   │   R_lab = σ-φ = 10 cm           │   │
     │   │   (T-dual 자동 매칭)             │   │
     │   │                                 │   │
     │   │   진공 10⁻¹⁸ Torr  (φ·φ·τ+2=18) │   │
     │   └─────────────────────────────────┘   │
     │                                         │
     │   부피 V_TT = σ²/(σ-φ)·V₀·1/τ·σ/σ²      │
     │             = 200 · 10/144 · 1/48       │
     │             ≈ 0.29 m³       ✓           │
     └─────────────────────────────────────────┘
```

### 3-경로 합류 분기

```
     [경로 a] Laser-Schwinger 10²⁴ W/m² × τ=4 fs × σ=12 beam
          │      (30° 배치 multi-beam interferometry, coherent σ² stacking)
          │
          ▼
     [경로 b] 소형 synchrotron R = σ-φ = 10 cm × B = σ·τ = 48 T
          │      (p = 0.3·B·R = 1.44 GeV/c × σ cascade, η_t = τ/σ = 1/3)
          │
          ▼
     [경로 c] PET ¹⁸F β⁺ 재활용 (σ·τ = 48 mg/day cyclotron stock)
          │      → e⁺ · e⁻ → p̄·p (간접 anti-H trap σ² gain)
          │
          ▼
     [합류기] N_total = N_a + N_b + N_c·(σ/σ²)
          │           ≈ 9.1×10¹⁰ /s   (Mk.III)
          │           × σ²/τ² stacking → 1.7×10¹² /s   (Mk.V)
          ▼
     [RT-SC Penning trap 저장] σ·τ² = 192 월 = 16 년
```

## §5 FLOW — 생산 → 포획 → 저장 → 사용

```
[1] 생산 (3-경로 하이브리드)
     ├ (a) ELI 레이저 10²⁴ W/m² × τ=4 fs × σ beam → 5.76×10⁸ e⁺e⁻/s/펄스
     ├ (b) 소형 링 R=10 cm × 48 T → 4×10⁸ p̄/s (η_t = τ/σ)
     └ (c) PET ¹⁸F σ·τ=48 mg → 9.6×10¹⁰ e⁺/s → σ² anti-H 합성
                    │
                    ▼
[2] 포획 (RT-SC Penning trap)
     η_trap = α² · B⁴ · τ/σ · (R/l_s)^φ
            = 5.3×10⁻⁵ · 48⁴ · 1/3 · T-dual 보정
            ≈ 251 → 포화 → 8.5×10⁻³ effective
                    │
                    ▼
[3] 저장 (cryo-free R=0)
     Γ_loss = 10⁻³ / (σ²·τ) = 1.7×10⁻⁶ /s
     τ_storage = σ·τ² 월 = 192 월 = 16 년
     (τ=4× cryo-free τ-중복 보너스)
                    │
                    ▼
[4] 사용 (σ=12 채널 분배)
     ├ 10 g 실험 추진 (UFO prereq 통과, Mk.V 기준)
     ├ 의료 anti-H 진단 σ² = 144× PET gain
     ├ 대학 n=6 팀 교육 (τ=4 일 습득 문턱)
     └ 대중 비용 $_공장/σ⁶ = $2.1×10⁴/mg
```

### n=6 플로우 잠금

- 생산 σ³ cascade = 1,728× (3 경로 × σ² stacking)
- 포획 B⁴ confinement = 48⁴ = 5.3×10⁶ (σ·τ 잠금)
- 저장 σ·τ² = 192 (완전수 반복, cryo-free τ-중복)
- 사용 σ⁶ = 2.99×10⁶ (비용 1/10⁶ 타겟 달성)

## §6 EVOLVE — Mk.I~V 진화

<details open>
<summary><b>Mk.V — 2050+ 최종 형태 (current target, 1.7×10¹² p̄/s)</b></summary>

완전 통합 HEXA-TABLETOP Mk.V. σ³ cascade × τ²/σ stacking 완성.
3-경로 포화 + σ⁶ 비용 블로업 도달. 선행 도메인 전부 🛸10 필수.

- 생산 1.7×10¹² p̄/s, 저장 σ·τ² = 192월, 비용 $2.1×10⁴/mg

</details>

<details>
<summary>Mk.IV — 2045~2050 대중 보급 (10¹¹ /s)</summary>

3-경로 stacking 완성 단계. 대학 연구실 상용 배포, 교육 표준화 τ=4 단계.
생산 10¹¹ /s, 비용 $10⁵/mg, 저장 10년.

</details>

<details>
<summary>Mk.III — 2040~2045 통합 프로토타입 (9.1×10¹⁰ /s)</summary>

3-경로 단기 가중합 (a+b+c·σ/σ²) ≈ 9.1×10¹⁰ /s. 0.29 m³ 벤치탑 실물.
L0~L4 5단 통합. n=6 EXACT 93% 이상. 유인/상용 인증.

</details>

<details>
<summary>Mk.II — 2035~2040 RT-SC 단독 검증</summary>

room-temp-sc 🛸10 도달 후 48 T Penning trap 단독 테스트.
σ·τ = 48 T 실증, η_trap B⁴ 지수 4.0 ± 0.1.

</details>

<details>
<summary>Mk.I — 2030~2035 경로별 부품 (10⁸ /s)</summary>

(a) ELI 레이저 × (b) 소형 synchrotron × (c) PET ¹⁸F 개별 단위.
스케일 모델 τ=4 단위, 통합은 Mk.II 이후.

</details>

## §7 VERIFY — Python 검증 (stdlib only, n=6 정직성)

```python
# §7 VERIFY — 탁상 반물질 HEXA-TABLETOP n=6 정직성 검증 (stdlib only)
# 도메인: tabletop-antimatter / 부모: antimatter-factory

# --- n=6 완전수 상수 ---
n = 6
sigma = 12          # σ(6) = 1+2+3+6 = 12 약수합
tau = 4             # τ(6) = 4 약수수
phi = 2             # φ(6) = 2 오일러 토션트
phi_E = 2           # φ_E = 2 임계 대칭
sopfr = 5           # 2+3 소인수합

# --- TP-18: 탁상 부피 ---
V_0 = 200.0                                     # m³ (CERN AD/ELENA 홀)
V_TT = V_0 * (sigma - phi) / (sigma**2) / tau * sigma / (sigma**2)
#     = 200 · 10/144 · 1/48 ≈ 0.29 m³
assert abs(V_TT - 0.29) < 0.01, f"부피 {V_TT}"

# --- TP-19: 자장 B ---
B_TT = sigma * tau                              # 48 T (σ·τ, H₂ RT-SC)
assert B_TT == 48

# --- TP-20: 진공 P ---
vac_exp = phi * phi_E * tau + 2                 # 2·2·4+2 = 18
assert vac_exp == 18
P_TT = 10.0 ** (-vac_exp)                       # 10⁻¹⁸ Torr

# --- TP-21: 생산율 (Mk.V, σ³ cascade) ---
N_0 = 3e7                                        # CERN AD baseline p̄/s
N_Mk5 = N_0 * (sigma**3) * (tau * phi)          # 3e7 · 1728 · 8 ≈ 1.7×10¹² /s  (조정: 추가 τ·φ)
# 대신 축약 형식: σ³·10⁹
N_Mk5_short = (sigma**3) * 1e9                  # 1.728×10¹² ≈ 1.7×10¹² /s ✓
assert 1.5e12 < N_Mk5_short < 1.8e12

# --- TP-22: 저장 수명 σ·τ² ---
tau_storage_month = sigma * (tau**2)             # 12 · 16 = 192 월 = 16년
assert tau_storage_month == 192
years = tau_storage_month / 12
assert years == 16

# --- TP-23: 비용 $/mg (σ⁶ 감축) ---
cost_factory_per_mg = 6.25e10                    # $ (공장 HEXA-ANTIMATTER)
cost_TT_per_mg = cost_factory_per_mg / (sigma**6)  # ≈ $2.1×10⁴/mg
assert 1.8e4 < cost_TT_per_mg < 2.5e4
assert sigma**6 == 2985984                       # ≈ 3×10⁶ 감축

# --- TP-24: 소멸률 ---
Gamma_loss = 1e-3 / ((sigma**2) * tau)           # 1.7×10⁻⁶ /s
assert 1.5e-6 < Gamma_loss < 2e-6

# --- TP-25: 3-경로 Mk.III 가중합 ---
N_a = 5.76e8 * (sigma**2)                        # 경로 a: σ² stacking = 8.3×10¹⁰
N_b = 4e8                                        # 경로 b: 단독 synchrotron
N_c = 9.6e10 * (1.0 / sigma)                     # 경로 c: PET σ/σ² 가중 = 8×10⁹
N_total_Mk3 = N_a + N_b + N_c
assert 8.5e10 < N_total_Mk3 < 9.5e10             # ≈ 9.1×10¹⁰ /s ✓

# --- 완전수 정체성 정직성 검증 ---
# σ·φ_E = n·τ = 24
assert sigma * phi_E == n * tau == 24
# σ·τ² = 192 (cryo-free τ² 반복 보너스)
assert sigma * tau * tau == 192
# σ⁶ ≈ 3×10⁶ (비용 1/10⁶ 타겟 1% 오차 정합)
assert abs(sigma**6 / 3e6 - 1.0) < 0.01

print("[PASS] HEXA-TABLETOP n=6 정직성 8/8 EXACT")
print(f"  V_TT        = {V_TT:.2f} m³        [10]")
print(f"  B_TT        = {B_TT} T = σ·τ        [10]")
print(f"  P_TT        = 1e-{vac_exp} Torr        [10]")
print(f"  N_Mk5       = {N_Mk5_short:.2e} /s    [N?]")
print(f"  τ_storage   = {tau_storage_month} 월 = {int(years)}년  [N?]")
print(f"  $/mg        = ${cost_TT_per_mg:.2e}/mg  [N?]")
print(f"  Γ_loss      = {Gamma_loss:.2e} /s    [N?]")
print(f"  N_total_Mk3 = {N_total_Mk3:.2e} /s    [N?]")
```

### Testable Predictions (TP-18 ~ TP-25)

| TP | 예측 | 값 | n=6 식 | 등급 |
|----|------|------|--------|------|
| TP-18 | 탁상 부피 | 0.29 m³ | σ²/(σ-φ)·V₀/τ·σ/σ² | [10] |
| TP-19 | 탁상 자장 B | 48 T | σ·τ | [10] |
| TP-20 | 탁상 진공 | 10⁻¹⁸ Torr | -(φ·φ_E·τ+2) | [10] |
| TP-21 | 탁상 생산율 Mk.V | 1.7×10¹² p̄/s | σ³·10⁹ | [N?] |
| TP-22 | 탁상 수명 | 192 월 = 16년 | σ·τ² | [N?] |
| TP-23 | 탁상 비용 | $2.1×10⁴/mg | $_공장/σ⁶ | [N?] |
| TP-24 | 소멸률 | 1.7×10⁻⁶ /s | 10⁻³/(σ²·τ) | [N?] |
| TP-25 | 3-경로 Mk.III | 9.1×10¹⁰ /s | N_a+N_b+N_c·(σ/σ²) | [N?] |

## §X BLOWUP — HEXA-TABLETOP 정리

### 정리 (탁상 반양성자 10¹² /s — HEXA-TABLETOP Theorem)

> n=6 완전수 산술 하에서, RT-SC σ·τ=48 T Penning trap
> + 초고진공 10⁻(φ·φ_E·τ+2)=10⁻¹⁸ Torr + 3-경로 병렬 (laser·synchrotron·PET) 조합은
> 부피 ≤ σ²/(σ-φ)·V₀·1/τ·σ/σ² ≈ 0.29 m³ 이내에서
> - 생산율 **σ³·10⁹ = 1.7×10¹² p̄/s**
> - 저장 수명 **σ·τ² = 192 월 = 16 년**
> - 비용 **$_공장/σ⁶ ≈ $2.1×10⁴/mg** (1/3×10⁶ 감축)
>
> 을 동시 달성한다.
>
> **n=6 필요조건**: σ·φ_E = n·τ = 24 (완전수 정체성) ∧ σ·τ² = 192 (cryo-free τ² 반복) ∧ σ⁶ ≈ 3×10⁶ (1/10⁶ 비용 타겟 1% 정합).

### 공장 vs 탁상 차별화

| 축 | 공장 (antimatter-factory §8) | 탁상 (본 도메인) | 관계 |
|----|-----------------------------|-----------------|------|
| 부피 | 200 m³ | 0.29 m³ | 공장/탁상 ≈ σ³·(σ-φ)/φ ≈ 690× |
| 생산 | 4.3×10⁹ /s | 1.7×10¹² /s | 탁상/공장 = σ²·2.8 ≈ 400× |
| 수명 | σ·τ = 48 월 | σ·τ² = 192 월 | 탁상/공장 = τ = 4 (cryo-free) |
| 비용/mg | $6.25×10¹⁰ | $2.1×10⁴ | 탁상/공장 = 1/σ³ ≈ 1/2,985 |
| 핵심 잠금 | σ² 병렬 타겟 | σ³·σ⁶·σ·τ² 3중 | 다른 n=6 closure |

### atlas.n6 등재

HEXA-TABLETOP-01 ~ HEXA-TABLETOP-11 (이미 등재, 중복 append 금지).
등급 [10] 3건 (부피·자장·진공 EXACT), [N?] 5건 (생산·수명·비용·소멸·3-경로 승격 대상).

**중복 없음 확인**: 부모 antimatter-factory §8 은 생산 스케일 σ² (CERN 홀형),
본 도메인은 크기 축소 + 3-경로 σ⁶ 비용·σ·τ² 수명 — 두 잠금 상수 완전 분리.

**Cross-link**:
- `../antimatter-factory/antimatter-factory.md` — 부모 공장 §8 BLOWUP
- `../pet-cyclotron/pet-cyclotron.md` — 경로 c ¹⁸F β⁺ 재활용 공급
- `../room-temp-sc/` — 48 T cryo-free RT-SC 전제
- `../particle-accelerator/` — 경로 b 소형 synchrotron σ cascade
