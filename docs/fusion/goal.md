# N6 핵융합 — 궁극 아키텍처 DSE 후보군 정의

**체인: 방식(Scheme) → 소재(Material) → 코어(Core) → 장치(Device) → 시스템(System)**

---

## N6 Constants Reference

```
  n=6  φ(6)=2  τ(6)=4  σ(6)=12  sopfr(6)=5
  μ(6)=1  J₂(6)=24  R(6)=1  λ(6)=2
  σ-τ=8  σ-φ=10  σ-μ=11  σ·τ=48  n/φ=3
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

---

## Level 1 — 방식 (Scheme) [6종]

| ID | 방식 | Q상한 | TRL | LCOE_est($/MWh) | n6 핵심 연관 |
|----|------|-------|-----|-----------------|-------------|
| S1 | Tokamak | 10+ | 7 | 60 | PF=6=n, CS=6=n, a=2=φ, A≈3=n/φ, Q=10=sopfr×φ |
| S2 | Stellarator | 5+ | 5 | 80 | W7-X periods=5=sopfr, coils complex |
| S3 | ICF (Laser) | 1.5+ | 4 | 200 | NIF 192=φ·σ(σ-τ) beams |
| S4 | FRC (Field-Reversed) | 2+ | 3 | 100 | compact, TAE C-2W |
| S5 | Mirror | 1+ | 3 | 150 | simple geometry |
| S6 | Z-pinch | 0.1+ | 2 | 300 | Zap Energy pulsed |

## Level 2 — 소재 (Material) [48 조합 = 4×4×3]

### 초전도체 [4종]
| ID | 초전도체 | Tc(K) | B_max(T) | 비용등급 | n6 연관 |
|----|---------|-------|---------|---------|--------|
| SC1 | LTS-NbTi | 9 | 10 | 1 | Tc≈σ-n/φ? |
| SC2 | LTS-Nb3Sn | 18 | 24 | 2 | B_max=J₂=24 |
| SC3 | HTS-REBCO | 92 | 45 | 4 | 현 SPARC/ARC 선택 |
| SC4 | HTS-BSCCO | 108 | 35 | 3 | Bi-2223 |

### 블랭킷 [4종]
| ID | 블랭킷 | TBR | 냉각재 | 비용등급 | n6 연관 |
|----|--------|-----|--------|---------|--------|
| BL1 | Li-ceramic | 1.05 | He | 2 | Li-6 증식=n |
| BL2 | PbLi-eutectic | 1.15 | PbLi self | 3 | 공융 460°C |
| BL3 | FLiBe-molten | 1.10 | FLiBe | 4 | 2LiF-BeF₂ |
| BL4 | He-cooled-pebble | 1.08 | He | 2 | HCPB ITER TBM |

### 구조재 [3종]
| ID | 구조재 | 내방사선(dpa) | 운전온도(°C) | 비용등급 |
|----|--------|-------------|------------|---------|
| ST1 | RAFM-steel | 80 | 550 | 1 |
| ST2 | V-alloy | 150 | 700 | 3 |
| ST3 | SiC-SiC | 200 | 1000 | 4 |

## Level 3 — 코어 (Core) [48 조합 = 4×3×4]

### 가열 방식 [4종]
| ID | 가열 | 주파수/에너지 | 효율 | n6 연관 |
|----|------|-------------|------|--------|
| H1 | NBI | 120keV=σ×10 | 40% | KSTAR 8MW=σ-τ |
| H2 | ICRH | 40-80MHz | 60% | KSTAR 6MW=n |
| H3 | ECRH | 170GHz | 70% | KSTAR 1MW=μ |
| H4 | LHCD | 5GHz=sopfr | 65% | 전류구동 전문 |

### 가둠 방식 [3종]
| ID | 가둠 | B_T 범위(T) | 비용 |
|----|------|-----------|------|
| C1 | SC-coil (초전도) | 5-20 | 높음 |
| C2 | Normal-Cu | 2-8 | 낮음 |
| C3 | Permanent+SC hybrid | 3-12 | 중간 |

### 연료 [4종]
| ID | 연료 | Q_value(MeV) | 반응단면적(keV) | n6 연관 |
|----|------|-------------|---------------|--------|
| F1 | D-T | 17.6 | 10-100 | D=φ, T=n/φ, sum=sopfr |
| F2 | D-D | 3.65 | 100+ | D=φ, sum=τ |
| F3 | D-He3 | 18.3 | 200+ | aneutronic |
| F4 | p-B11 | 8.7 | 500+ | B=11=σ-μ |

## Level 4 — 장치 (Device) [180 조합 = 4×5×3×3]

### 코일 배치 [4종]
| ID | 코일수 | n6 표현 |
|----|--------|--------|
| TF1 | 6 | n |
| TF2 | 12 | σ |
| TF3 | 16 | 2^τ |
| TF4 | 18 | σ+n |

### 기하 (Aspect Ratio A) [5종]
| ID | A | n6 표현 | 대표 장치 |
|----|---|--------|----------|
| A1 | 2.5 | sopfr/φ | compact |
| A2 | 3.0 | n/φ | ARC/SPARC |
| A3 | 3.1 | ITER 실제 | ITER |
| A4 | 4.0 | τ | mid-size |
| A5 | 5.0 | sopfr | W7-X |

### 자기장 등급 B_T [3종]
| ID | B_T(T) | n6 표현 |
|----|--------|--------|
| B1 | 5 | sopfr |
| B2 | 12 | σ |
| B3 | 20 | J₂-τ |

### Q 목표 [3종]
| ID | Q | n6 표현 | 의미 |
|----|---|--------|------|
| Q1 | 2 | φ | breakeven |
| Q2 | 10 | sopfr×φ | ITER 목표 |
| Q3 | 1000 | ∞ (점화) | 자기유지 |

## Level 5 — 시스템 (System) [27 조합 = 3×3×3]

### 발전 방식 [3종]
| ID | 발전 | 효율 | 성숙도 |
|----|------|------|--------|
| PW1 | Rankine (증기) | 33%=1/(n/φ) | 높음 |
| PW2 | Brayton (가스) | 45% | 중간 |
| PW3 | Direct-conversion | 60% | 낮음 |

### TBR 전략 [3종]
| ID | TBR 방식 | TBR 값 | 비용 |
|----|---------|--------|------|
| TR1 | Li6-ceramic-breeder | 1.05 | 중간 |
| TR2 | PbLi-self-cooled | 1.15 | 높음 |
| TR3 | Dual-coolant-DCLL | 1.20=σ/(σ-φ) | 높음 |

### 전력망 [3종]
| ID | 전력망 | 주파수/전압 | n6 연관 |
|----|--------|-----------|--------|
| G1 | AC-50Hz | 50Hz=sopfr×(σ-φ) | BT-62 |
| G2 | AC-60Hz | 60Hz=σ×sopfr | BT-62 |
| G3 | HVDC | ±500kV | BT-68 |

---

## 전수 조합 수

```
  6 × (4×4×3) × (4×3×4) × (4×5×3×3) × (3×3×3)
= 6 × 48 × 48 × 180 × 27
= 67,184,640 조합

→ Rust DSE 필수 (>10K 기준)
```

## 평가 5축

| 축 | 설명 | 범위 | 가중치 |
|----|------|------|--------|
| n6_EXACT | 이산 파라미터 n=6 매칭 비율 | 0-100% | 35% |
| Q_gain | 에너지 이득 | 0-1000 | 25% |
| TRL | 기술 성숙도 | 1-9 | 20% |
| LCOE | 균등화 발전비용 ($/MWh) | 300→10 (역수) | 12% |
| T_comm | 상용화 시점 (빠를수록 좋음) | 2050→2030 | 8% |

---

## DSE 실행

```
  도구: tools/fusion-dse/main.rs (Rust)
  빌드: ~/.cargo/bin/rustc tools/fusion-dse/main.rs -o tools/fusion-dse/fusion-dse
  출력: Pareto frontier + 최적 경로 + 방식별 분석
```
