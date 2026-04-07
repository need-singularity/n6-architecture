# H-CERN-5: Topological Barcode of Dimuon Mass Spectrum

> **Thesis**: CMS 디뮤온 불변 질량 스펙트럼에 persistent homology(H0)를 적용하면,
> 바코드의 birth-death 패턴이 입자 세대(generation) 구조를 드러내고,
> topological_optics.py의 β₀(ε) 위상 전이가 QCD 에너지 스케일 전환점에 대응한다.

**Golden Zone 의존성**: 있음 (R-spectrum 위상구조 해석 기반)

## 1. Background

topological_optics.py는 R-spectrum을 1D point cloud로 취급하여:
- H0 barcode: connected components의 birth/death
- β₀(ε): filtration parameter ε에서의 Betti number
- Topological transitions: β₀가 급변하는 critical ε 값

디뮤온 질량 스펙트럼은 자연스러운 1D point cloud:
각 이벤트의 불변 질량 = 1차원 데이터 포인트

## 2. Core Hypothesis

```
  디뮤온 Mass Spectrum           Topological Optics
  ──────────────────            ────────────────────
  공명 피크 (cluster)            H0 birth event
  피크 간 valley                 H0 death event
  persistence (death-birth)      공명의 "구조적 강도"
  β₀ transition ε_c              에너지 스케일 전환
  long-lived bar                 견고한 물리적 공명
  short-lived bar                통계적 fluctuation
```

### Generation Structure Prediction

```
  Generation 1: ρ/ω — φ — J/ψ         (0.7 - 3.1 GeV)
  Generation 2: ψ(2S) — Υ             (3.7 - 9.5 GeV)
  Generation 3: Z                      (91.2 GeV)

  β₀(ε) 예측:
  β₀ |
   6 +  ****
   5 +      *
   4 +       ***
   3 +          ****                    ← 3 = σ(6)/τ(6) = 세대 수
   2 +              ********
   1 +                      *****************************
   0 +──────────────────────────────────────────→ ε (GeV)
       0.01  0.1    1     10    100
```

## 3. Methodology

### Step 1: Mass Spectrum → Point Cloud

```
  1. CMS 디뮤온 100k events에서 불변 질량 추출
  2. 히스토그램 피크 위치 = 1D point cloud
     X = {0.770, 1.020, 3.097, 3.686, 9.460, 91.188}
  3. 선택적: log(M) 변환으로 스케일 압축
     X_log = {-0.114, 0.020, 1.130, 1.305, 2.247, 4.513}
```

### Step 2: Persistent Homology 계산

```bash
# topological_optics.py 직접 사용 (피크 위치를 R-spectrum 대신 입력)
# 또는 순수 PH 계산:

from scipy.spatial.distance import pdist
from ripser import ripser  # 또는 순수 Python gap analysis

peaks_log = [-0.114, 0.020, 1.130, 1.305, 2.247, 4.513]
gaps = sorted([peaks_log[i+1] - peaks_log[i] for i in range(len(peaks_log)-1)])
# gaps = [0.134, 0.175, 0.942, 1.110, 2.266]
```

### Step 3: β₀(ε) Sweep

```
  ε = 0:      β₀ = 6 (모든 피크 분리)
  ε = 0.134:  β₀ = 5 (ρ/ω — φ merge)
  ε = 0.175:  β₀ = 4 (J/ψ — ψ(2S) merge)
  ε = 0.942:  β₀ = 3 (Gen1 — Gen2 merge)  ← σ/τ = 3!
  ε = 1.110:  β₀ = 2 (Υ absorb)
  ε = 2.266:  β₀ = 1 (Z absorb → single component)
```

## 4. Key Predictions

| # | Prediction | Criterion | n=6 Connection |
|---|---|---|---|
| 1 | β₀=3 plateau 존재 | plateau width > 0.1 (log scale) | σ/τ = 3 세대 |
| 2 | longest bar = Z boson | persistence > 2.0 | σ(6)-τ(6)=8 스케일 |
| 3 | Gen1 내부 2개 sub-bar | ρ/ω-φ, J/ψ gaps | φ(6)=2 sub-generations |
| 4 | total persistence ∝ σ(6) | ΣL_i ≈ 12k for some k | σ(6)=12 보존 |
| 5 | critical ε ratio = ln(4/3) | ε_c2/ε_c1 ≈ 0.288 | Golden Zone width |

## 5. ASCII: Barcode Diagram (log-mass scale)

```
  Component    Birth    Death    Persistence
  ─────────    ─────    ─────    ───────────
  ρ/ω         -0.114    0.020    0.134       ■■
  φ(1020)      0.020    1.130    1.110       ■■■■■■■■■■■■■■■■■
  J/ψ          1.130    1.305    0.175       ■■■
  ψ(2S)        1.305    2.247    0.942       ■■■■■■■■■■■■■■
  Υ(1S)        2.247    4.513    2.266       ■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  Z            4.513      ∞        ∞         ■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■→

  가장 긴 bar: Z (trivially — 가장 고립됨)
  물리적으로 의미있는 가장 긴 bar: Υ→Z gap (persistence=2.266)
  가장 짧은 bar: ρ/ω→φ (persistence=0.134) — 가장 가까운 이웃
```

## 6. Verification Commands

```bash
# R-spectrum topological lens를 CERN 피크에 적용
python3 calc/topological_optics.py --telescope --n 6 --epsilon-range 0.001-5 --steps 100

# 디뮤온 데이터 직접 PH 분석 (새 스크립트 필요)
# verify_cern_topology.py 작성 → 피크 위치 PH + β₀ sweep
```

## 7. Limitations

- 피크 6개 → n=6: small sample에서의 위상 구조는 불안정
- log 변환 여부에 따라 바코드 구조 변경
- β₀=3이 "세대"를 의미한다는 해석은 post-hoc
- 위상적 구조는 질량 분포의 산물이지, R-spectrum과의 직접 연결 미확립

## 8. Next Steps

1. PDG 전체 입자(84개)로 확장한 PH 분석
2. Bootstrap 리샘플링으로 바코드 안정성 검증
3. 무작위 mass spectrum과의 Texas Sharpshooter 비교
4. R-spectrum β₀(ε)와 CERN β₀(ε)의 상관관계 정량화
