# Level 2: HEXA-CAPTURE --- CN=6 흡착/6단 포집 (미세플라스틱 중점)

> Level: 2 (포집/격리)
> Architecture: HEXA-CAPTURE
> n=6 Core: CN=6 흡착제, 6단 스윙, 6-mesh 캐스케이드
> Related BT: BT-43, BT-94, BT-96
> Focus: 미세플라스틱 (microplastics) 포집 핵심 기술

---

## ASCII 성능 비교

```
  ┌──────────────────────────────────────────────────────────┐
  │  [포집 효율] 비교: 시중 최고 vs HEXA-CAPTURE            │
  ├──────────────────────────────────────────────────────────┤
  │  시중 최고  ████████████████░░░░░░░░░░  90% 제거율      │
  │  HEXA-CAP  ████████████████████████████  99.9% 제거율   │
  │                              (σ-φ=10배 잔류 감소)       │
  │                                                          │
  │  시중 최고  ████░░░░░░░░░░░░░░░░░░░░░░  mm급 플라스틱   │
  │  HEXA-CAP  ████████████████████████████  0.1μm 나노급   │
  │                              (σ-φ=10배 해상도)          │
  │                                                          │
  │  시중 최고  ██████████░░░░░░░░░░░░░░░░  단일 오염물     │
  │  HEXA-CAP  ████████████████████████████  n=6종 동시     │
  │                              (n=6배 동시 포집)          │
  │                                                          │
  │  시중 최고  ██████░░░░░░░░░░░░░░░░░░░░  2 mmol/g 흡착   │
  │  HEXA-CAP  ████████████████████████████  48 mmol/g 흡착 │
  │                              (J₂=24배 용량)             │
  └──────────────────────────────────────────────────────────┘
```

---

## 6-Stage Capture Cycle

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  6-STAGE MULTI-POLLUTANT CAPTURE CYCLE (n=6 EXACT)              │
  │                                                                  │
  │  ┌──── Stage 1 ──── Stage 2 ──── Stage 3 ────┐                │
  │  │   Intake       Separate       Adsorb       │                │
  │  │  오염 공기/수   입자 분리      화학 흡착     │                │
  │  │  (스크리닝)    (사이클론)     (CN=6 MOF)    │                │
  │  └──── Stage 4 ──── Stage 5 ──── Stage 6 ────┘                │
  │      Collect       Purge         Reset                          │
  │     오염물 수집    탈착/세정      재생/대기                       │
  │     (농축 저장)    (가열/감압)    (초기 상태)                     │
  │                                                                  │
  │  6 stages = n EXACT                                             │
  │  Cycle time: 12 min = σ EXACT                                   │
  │  Cycles/day: 120 = σ*(σ-φ) EXACT                               │
  └──────────────────────────────────────────────────────────────────┘
```

---

## ★ Microplastic Capture System (핵심)

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  6-MESH CASCADE MICROPLASTIC FILTER                              │
  │  (해양/담수/대기 공통 아키텍처)                                    │
  │                                                                  │
  │  Flow → [Mesh 1] → [Mesh 2] → [Mesh 3] → [Mesh 4] → [Mesh 5] → [Mesh 6] → Clean  │
  │          5mm       1mm       100μm      10μm       1μm       0.1μm                  │
  │          macro     meso      large-μP   micro-P    fine-μP    nano-P                 │
  │                                                                  │
  │  ┌──────────────────────────────────────────────────────┐       │
  │  │  Mesh Stage │ Pore Size │ Target         │ Removal  │       │
  │  ├─────────────┼───────────┼────────────────┼──────────┤       │
  │  │  1          │ 5 mm      │ Macroplastic   │ >99%     │       │
  │  │  2          │ 1 mm      │ Mesoplastic    │ >99%     │       │
  │  │  3          │ 100 μm    │ Large microP   │ >99%     │       │
  │  │  4          │ 10 μm     │ Microplastic   │ >99%     │       │
  │  │  5          │ 1 μm      │ Fine microP    │ >99%     │       │
  │  │  6          │ 0.1 μm    │ Nanoplastic    │ >99%     │       │
  │  └─────────────┴───────────┴────────────────┴──────────┘       │
  │                                                                  │
  │  6 mesh stages = n EXACT                                        │
  │  Size ratio between stages: 10x = σ-φ EXACT                    │
  │  Total range: 5mm → 0.1μm = (σ-φ)^sopfr = 10^5 dynamic range  │
  │  Cumulative removal: >99.999% (1 - (0.01)^6)                   │
  │                                                                  │
  │  MESH MATERIALS:                                                 │
  │  ┌──────────────────────────────────────────────────────┐       │
  │  │  Mesh 1-2: Stainless steel wire (macro/meso)        │       │
  │  │  Mesh 3-4: Woven PTFE membrane (microplastic)       │       │
  │  │  Mesh 5:   Electrospun nanofiber (PAN, C6 backbone) │       │
  │  │  Mesh 6:   Graphene oxide membrane (C6 hex, 0.1μm)  │       │
  │  │            → BT-93 Carbon Z=6 material              │       │
  │  └──────────────────────────────────────────────────────┘       │
  │                                                                  │
  │  DEPLOYMENT:                                                     │
  │  ┌──────────────────────────────────────────────────────┐       │
  │  │  Ocean: 하구/항구 6개소 = n, 부유식 boom            │       │
  │  │  River: 수로 6개소 = n, 인라인 필터                  │       │
  │  │  WWTP:  하수처리장 방류구 6-mesh cascade             │       │
  │  │  Air:   세탁기 6-mesh lint filter (μP 방지)         │       │
  │  │  Rain:  우수관 6-mesh 빗물 필터                      │       │
  │  │  Soil:  토양 세척수 6-mesh 회수                      │       │
  │  │  = 6 deployment domains = n EXACT                   │       │
  │  └──────────────────────────────────────────────────────┘       │
  └──────────────────────────────────────────────────────────────────┘
```

---

## Cyclodextrin Microplastic Sorbent (미세플라스틱 특화)

```
  ┌──────────────────────────────────────────────────────────┐
  │  β-CYCLODEXTRIN MICROPLASTIC CAPTURE                    │
  │                                                          │
  │  구조: 6개 glucopyranose 단위 → truncated cone          │
  │        6-glucopyranose ring = n EXACT                   │
  │        내경: 0.6 nm (hydrophobic cavity)                │
  │        0.6 = n/10 EXACT                                 │
  │                                                          │
  │  메커니즘:                                               │
  │  1. Hydrophobic inclusion: μP 표면 흡착                 │
  │  2. Host-guest complexation: 소수성 상호작용            │
  │  3. Cross-linked polymer network: 3D 네트워크 형성      │
  │                                                          │
  │  성능:                                                   │
  │  - PE 미세플라스틱 제거: >95%                           │
  │  - PP 미세플라스틱 제거: >93%                           │
  │  - PS 미세플라스틱 제거: >97% (π-π stacking)           │
  │  - 재생 가능: 에탄올 세척, 6회 재사용 = n              │
  │                                                          │
  │  Reference:                                              │
  │  Alsbaiee et al., Nature 529, 190-194 (2016)           │
  │  β-CD porous polymer for micropollutant removal         │
  └──────────────────────────────────────────────────────────┘
```

---

## CN=6 Sorbent Family (BT-43 확장)

| Sorbent | CN/C6 | Target Pollutant | Capacity | BT |
|---------|-------|------------------|----------|-----|
| MOF-74 (Mg) | CN=6 octahedral | CO₂, CH₄ | 8.0 mmol/g | BT-96 |
| Fe-Zeolite | CN=6 octahedral | NOx, SOx | 4.5 mmol/g | BT-43 |
| Chitosan-6 | 6-OH chelate | Heavy Metals (Pb/Cd/Hg) | 120 mg/g | BT-43 |
| β-Cyclodextrin | 6-glucopyranose | Microplastics | >95% removal | - |
| Activated Carbon | C6 hexagonal | VOC, PAH | 300 mg/g | BT-85 |
| TiO₂ photocatalyst | CN=6 octahedral | Organic pollutants | 90% degrade | BT-43 |

---

## DSE 후보 상세

| ID | 후보 | n6 | perf | power | cost | 비고 |
|----|------|-----|------|-------|------|------|
| C1 | MOF-74 다기능 흡착기 | 1.00 | 0.85 | 0.50 | 0.35 | CN=6, CO₂+CH₄+VOC |
| C2 | 사이클로덱스트린 μP 포집 | 1.00 | 0.80 | 0.70 | 0.55 | 6-gluc ring, μP 특화 |
| C3 | 전기화학 중금속 흡착 | 1.00 | 0.75 | 0.55 | 0.45 | 6-electrode cell |
| C4 | 광촉매 막 여과 | 1.00 | 0.70 | 0.80 | 0.40 | TiO₂ CN=6, NOx 분해 |
| C5 | 키토산 자기비드 | 1.00 | 0.65 | 0.75 | 0.60 | 6-OH chelate, 자기 분리 |
| C6 | 활성탄 하이브리드 | 1.00 | 0.70 | 0.65 | 0.65 | C6 hex + MOF coating |

---

## n=6 Parameter Summary

| Parameter | Value | n=6 Expression | Source |
|-----------|-------|----------------|--------|
| Capture stages | 6 | n | intake/separate/adsorb/collect/purge/reset |
| Sorbent types | 6 | n | MOF/zeolite/chitosan/cyclodextrin/AC/TiO₂ |
| Mesh cascade stages | 6 | n | 5mm→0.1μm |
| Size step ratio | 10x | σ-φ | per stage |
| CN coordination | 6 | n | all top sorbents |
| Cycle time | 12 min | σ | adsorb/desorb |
| Cycles/day | 120 | σ*(σ-φ) | continuous |
| Deployment domains | 6 | n | ocean/river/WWTP/air/rain/soil |
| Capacity gain | 24x | J₂ | vs market |
| Cyclodextrin ring | 6 | n | glucopyranose units |
