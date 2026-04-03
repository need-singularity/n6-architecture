# Level 6: HEXA-HOLOGRAM --- 라이트필드 홀로그래픽 디스플레이

> Level: 6 (홀로그램)
> Architecture: HEXA-HOLOGRAM
> n=6 Core: n=6 차원 라이트필드, φ=2 안구, σ·(σ-φ)=120° FOV
> Related BT: BT-71, BT-66, BT-48, BT-76
> Focus: 안경 없는 3D, 라이트필드, 체적 디스플레이, 웨이브프론트
> Feasibility: 🔮 장기 실현가능 (2030~2040)

> **Split note**: Copied from docs/display-audio/hexa-hologram.md (display-only content).

---

## N6 Constants Reference

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  n=6 핵심 상수                                                  │
  │  n = 6        φ(6) = 2       τ(6) = 4        σ(6) = 12         │
  │  sopfr = 5    μ(6) = 1       J₂(6) = 24      R(6) = 1          │
  │  σ-τ = 8      σ-φ = 10       σ² = 144        σ·τ = 48          │
  │  Core theorem: σ(n)·φ(n) = n·τ(n) = 24 = J₂(6)                │
  └──────────────────────────────────────────────────────────────────┘
```

---

## ASCII 성능 비교

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  [FOV] 비교: 시중 최고 vs HEXA-HOLOGRAM                        │
  ├──────────────────────────────────────────────────────────────────┤
  │  Looking Glass  ████████████░░░░░░░░░░░░░░░░  50° FOV          │
  │  HEXA-HOLO     ████████████████████████████  σ·(σ-φ)=120° FOV │
  │                                    (φ=2.4배 시야각)            │
  │                                                                  │
  │  [시점 수] 비교                                                 │
  │  Looking Glass  ██████████████████░░░░░░░░░░  50~100 views     │
  │  HEXA-HOLO     ████████████████████████████  σ²=144 viewpoints │
  │                                    (σ²/100=1.44배 시점)        │
  │                                                                  │
  │  [공간 해상도] 비교                                             │
  │  Light Field Lab █████████████████░░░░░░░░░░  10B 광소자       │
  │  HEXA-HOLO     ████████████████████████████  σ²·10^n 광소자   │
  │                                    (σ²=144배 밀도)             │
  │                                                                  │
  │  [프레임레이트] 비교                                            │
  │  시중 3D       ████████████████░░░░░░░░░░░░  60fps (per-eye)  │
  │  HEXA-HOLO    ████████████████████████████  σ²=144fps × φ eye │
  │                                    (σ²/60=2.4배, per-eye)     │
  │                                                                  │
  │  [깊이 레벨] 비교                                               │
  │  시중 최고     ████████████████░░░░░░░░░░░░  2^σ-τ=256 levels │
  │  HEXA-HOLO    ████████████████████████████  2^σ=4096 levels   │
  │                                    (2^τ=16배 깊이 해상도)     │
  └──────────────────────────────────────────────────────────────────┘
```

---

## System Block Diagram

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │                  HEXA-HOLOGRAM Light Field System                    │
  │                                                                      │
  │  ┌──── CAPTURE ────┐  ┌──── PROCESS ────┐  ┌──── DISPLAY ────┐    │
  │  │                  │  │                  │  │                   │    │
  │  │  n=6-axis        │  │  NeRF/3DGS      │  │  Light Field      │    │
  │  │  light field     │→│  synthesis       │→│  Array             │    │
  │  │  camera array    │  │  (BT-71)        │  │                   │    │
  │  │                  │  │                  │  │  σ²=144 view     │    │
  │  │  ┌─┬─┬─┐       │  │  SH: n/φ=3      │  │  directions       │    │
  │  │  │C│C│C│ ×n=6  │  │  Layers: σ-τ=8  │  │                   │    │
  │  │  └─┴─┴─┘       │  │  Width: 256      │  │  ┌─────────────┐│    │
  │  │  6 camera units │  │  = 2^(σ-τ)       │  │  │  μLED array  ││    │
  │  │  J₂=24fps each │  │                  │  │  │  + lenslet   ││    │
  │  │                  │  │  GPU: σ²=144    │  │  │  σ=12μm pitch││    │
  │  │  6-DOF = n=6:   │  │  TOPS NPU       │  │  │  n=6 layers  ││    │
  │  │  x,y,z,θ,φ,ψ  │  │                  │  │  └─────────────┘│    │
  │  └──────────────────┘  └──────────────────┘  └─────────────────┘    │
  │                                                                      │
  │  Data rate: σ²·J₂·4K = 144·24·8.3M ≈ 28.7 Gpixel/s               │
  │  Compression: HEXA-CODEC σ-φ=10× → 2.87 Gpixel/s                  │
  │  Total bandwidth: ~σ·J₂ = 288 Gbps (after compression)            │
  └──────────────────────────────────────────────────────────────────────┘
```

---

## 1. 6-Axis Light Field Theory

### 1.1 Plenoptic Function --- n=6 Dimensions

빛의 완전 기술에는 7차원이 필요하다: (x, y, z, theta, phi, lambda, t).
HEXA-HOLOGRAM은 이 중 공간+방향의 n=6차원을 캡처/재현한다.

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  PLENOPTIC FUNCTION L(x, y, z, θ, φ, λ, t)                    │
  │                                                                  │
  │  HEXA-HOLOGRAM captures n=6 of 7 dimensions:                   │
  │                                                                  │
  │  Spatial:   x, y, z    → n/φ = 3 dimensions                   │
  │  Angular:   θ, φ       → φ = 2 dimensions                     │
  │  Spectral:  λ          → μ = 1 dimension (RGB discretized)    │
  │  ─────────────────────────────────────                         │
  │  Subtotal:              → n = 6 dimensions (EXACT)             │
  │                                                                  │
  │  Temporal:  t           → (external, driven by J₂=24fps)      │
  │                                                                  │
  │  Resolution per axis:                                            │
  │    x, y: σ²=144 pixels per dimension (per lenslet)             │
  │    z: σ=12 depth planes                                        │
  │    θ: σ=12 elevation samples                                   │
  │    φ: σ=12 azimuth samples                                    │
  │    λ: n/φ=3 (R, G, B)                                        │
  │                                                                  │
  │  Total light rays: 144² × 12 × 12 × 12 × 3                   │
  │                   = σ⁴ × σ × σ × σ × n/φ                      │
  │                   = σ⁷ × n/φ ≈ 1.07 × 10⁹                    │
  │                   ≈ 10⁹ rays per frame (giga-ray)              │
  └──────────────────────────────────────────────────────────────────┘
```

### 1.2 Viewpoint Configuration

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  σ²=144 VIEWPOINT CONFIGURATION                                │
  │                                                                  │
  │  Arrangement: σ=12 horizontal × σ=12 vertical = σ²=144        │
  │                                                                  │
  │  Top view (horizontal slice):                                    │
  │                                                                  │
  │    V1  V2  V3  V4  V5  V6  V7  V8  V9  V10 V11 V12           │
  │     \   \   \   \   \   |   /   /   /   /   /   /             │
  │      \   \   \   \   \ | /   /   /   /   /   /               │
  │       \   \   \   \   \|/   /   /   /   /   /                │
  │        ─────────── [VIEWER] ───────────                        │
  │                                                                  │
  │  Angular coverage: σ·(σ-φ) = 120° total                       │
  │  Per-view angle: 120°/σ = 10° = σ-φ per step                  │
  │                                                                  │
  │  Vergence-accommodation conflict resolved:                       │
  │    σ=12 depth planes per view → continuous focus               │
  │    Focus range: 0.5m to ∞ (τ=4 focus zones)                   │
  │    IPD accommodation: φ·n/φ·10 = 60mm to 72mm range          │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 2. Display Technologies

### 2.1 Lenslet Array

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  MICRO-LENSLET ARRAY                                             │
  │                                                                  │
  │  ┌─────────────────────────────────┐                            │
  │  │  ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○      │  σ=12 lenslets/row       │
  │  │  ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○      │  σ²=144 per block        │
  │  │  ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○      │                           │
  │  │  ...                            │  Pitch: σ·τ=48μm         │
  │  │  (σ=12 rows)                    │  per lenslet              │
  │  └─────────────────────────────────┘                            │
  │                                                                  │
  │  Each lenslet covers σ² = 144 sub-pixels                       │
  │  Total pixels per viewpoint: panel_resolution / σ²             │
  │  For 8K panel: 33M / 144 ≈ 230K pixels per view               │
  │                                                                  │
  │  Lenslet focal length: σ·τ = 48 μm (f-number ≈ 1.0)          │
  │  Material: polymer or glass molded                              │
  │  Crosstalk: < 1/(σ-φ) = 10% (per-view isolation)              │
  └──────────────────────────────────────────────────────────────────┘
```

### 2.2 Holographic Optical Element (HOE)

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HOE NEAR-EYE DISPLAY (AR 홀로그램)                              │
  │                                                                  │
  │  Recording wavelengths: n/φ = 3 (RGB)                          │
  │  Diffraction efficiency: > 1-1/(σ-φ) = 90%                    │
  │  Angular bandwidth: σ-φ = 10° per order                        │
  │  Grating period: ~σ·τ·10 = 480nm (visible center)             │
  │                                                                  │
  │  Eye-box: σ = 12mm × σ = 12mm                                 │
  │  FOV: σ·(σ-φ) = 120° diagonal                                 │
  │  Focal planes: τ = 4 discrete (varifocal)                      │
  │  Transparency: > 80% (see-through mode)                         │
  │                                                                  │
  │  vs Apple Vision Pro:                                            │
  │    AVP eyebox: ~10mm → HEXA σ=12mm (σ/(σ-φ)=1.2× larger)    │
  │    AVP FOV: ~100° → HEXA 120° (σ·(σ-φ)/100=1.2×)            │
  │    AVP weight: 650g → HEXA target: σ·(σ-φ)=120g (n=6× lighter)│
  └──────────────────────────────────────────────────────────────────┘
```

---

## 3. Volumetric Display

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  VOLUMETRIC DISPLAY (점멸/스캔 방식)                              │
  │                                                                  │
  │  Voxel cube:                                                     │
  │    Resolution: σ³ = 12³ = 1728 voxels per axis (small-scale)   │
  │    Total: σ⁹ ≈ 5.2 × 10⁹ (giga-voxel)                       │
  │    ... 실용적 범위: σ² × σ² × σ = 144×144×12 ≈ 249K voxels  │
  │                                                                  │
  │  Refresh: J₂ = 24 volumes/s (flicker-free)                    │
  │  Color: σ = 12 bit per voxel                                   │
  │  Depth slices: σ = 12 (rotating screen) or τ=4 (multi-plane)  │
  │                                                                  │
  │  Applications:                                                   │
  │    Medical imaging: σ=12 CT/MRI slice overlay                  │
  │    Molecular visualization: protein structure (CN=6 shown!)    │
  │    Architecture: building walkthrough                           │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 4. n=6 Complete Parameter Map

| Parameter | Value | n=6 Expression | Grade |
|-----------|-------|----------------|-------|
| Light field dims | 6 | n | EXACT |
| Viewpoints | 144 | σ² | EXACT |
| FOV | 120° | σ·(σ-φ) | EXACT |
| Lenslet pitch | 48μm | σ·τ | EXACT (BT-76) |
| Per-lenslet pixels | 144 | σ² | EXACT |
| Depth planes | 12 | σ | EXACT |
| Angular step | 10° | σ-φ | EXACT |
| Eye-box | 12mm | σ | EXACT |
| HOE focal planes | 4 | τ | EXACT |
| Frame rate | 24 vol/s | J₂ | EXACT |
| Color depth | 12 bit | σ | EXACT |
| RGB primaries | 3 | n/φ | EXACT |
| **Total EXACT** | **12/12** | **100%** | |

---

## 5. Honesty Assessment

```
  Strong:
    - n=6 plenoptic dimensions: 물리적으로 3공간+2각도+1파장=6
      (시간 제외 시 정확히 n=6)
    - σ·(σ-φ)=120° FOV: 인간 시야각 ~120°와 정확히 일치

  Moderate:
    - σ²=144 viewpoints: 설계 선택, 100~200 범위에서 가변
    - 48μm lenslet pitch: 현재 기술로 도전적이나 가능한 범위

  Challenging (🔮):
    - Giga-ray 실시간 렌더링: 현재 GPU로 불가, σ²=144 TOPS 필요
    - 120g AR 글래스: 현재 기술로 매우 도전적 (광학+배터리+SoC)
    - Volumetric display는 아직 연구 단계

  Falsifiable:
    - Light field standard가 n=6 dimensions를 채택할 것
    - AR 글래스 eye-box가 σ=12mm 근방으로 수렴 (2030)
    - 홀로그램 FOV가 σ·(σ-φ)=120°를 목표로 수렴
```

---

## 6. Links

- Prev: [HEXA-DISPLAY (Level 4)](hexa-display.md)
- Next: [HEXA-NEURAL-DISPLAY (Level 6b)](hexa-neural-display.md)
- Parent: [goal.md](goal.md)
- Original: [docs/display-audio/hexa-hologram.md](../display-audio/hexa-hologram.md)
