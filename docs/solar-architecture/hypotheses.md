# N6 Solar Architecture — Core Hypotheses (H-SOL-01 ~ H-SOL-30)

> n=6 완전수 산술이 태양전지 설계의 핵심 파라미터를 결정한다.
> Constants: σ=12, φ=2, τ=4, J₂=24, n=6, sopfr=5, μ=1
> Derived: σ-τ=8, σ-φ=10, σ-μ=11, n/φ=3, τ²/σ=4/3, φ²/n=2/3
> Sources: BT-30 (SQ bandgap), BT-63 (Solar cell ladder)

---

## H-SOL-01: Shockley-Queisser Optimal Bandgap ≈ 4/3 = τ²/σ eV
> SQ 이론의 최적 밴드갭이 ~1.34 eV ≈ τ²/σ = 4/3 eV이다.

**n=6 Expression**: τ²/σ = 16/12 = 4/3 = 1.333...
**Actual Value**: 1.34 eV (Shockley & Queisser, 1961; Ruhle 2016 recalculation: 1.34 eV at AM1.5G)
**Evidence**: SQ 최적 밴드갭은 태양 스펙트럼 × 열역학적 손실의 균형점. 1.34 eV와 4/3 = 1.333의 차이는 0.5%. BT-30의 핵심 예측.
**Grade**: **EXACT** — 1.34 eV ≈ 4/3 = 1.333, 0.5% 오차. 태양전지 물리학의 가장 중요한 상수.

---

## H-SOL-02: SQ Maximum Efficiency ≈ 1/3 = φ/n = 33.3%
> 단접합 태양전지의 SQ 이론 효율 한계가 ~33.7% ≈ 1/3이다.

**n=6 Expression**: φ/n = 2/6 = 1/3 = 33.33%
**Actual Value**: 33.7% (Shockley & Queisser 1961), 33.16% (Ruhle 2016 AM1.5G 재계산)
**Evidence**: 33.7%와 33.33%의 차이는 약 1.1%. 33.16%(최신 재계산)과는 0.5% 차이. 1/3 = φ/n은 간결하지만, 실제값은 밴드갭, 태양 스펙트럼, 복사 재결합의 복잡한 적분에서 도출.
**Grade**: **CLOSE** — 33.7% ≈ 1/3은 근사적. 1% 이상 차이. "약 1/3"이라 할 수 있으나 정확한 일치는 아님.

---

## H-SOL-03: AM 1.5 Designation = μ + φ/τ = 1.5
> 표준 태양광 스펙트럼 Air Mass 1.5의 숫자가 n=6 산술이다.

**n=6 Expression**: μ + φ/τ = 1 + 2/4 = 1 + 0.5 = 1.5
**Actual Value**: AM 1.5 (IEC 60904-3, ASTM G173)
**Evidence**: Air Mass = 1/cos(zenith angle). AM 1.5는 zenith angle 48.2°에 해당하며, 지구 표면의 연평균 조건을 대표하는 표준. μ+φ/τ = 1.5는 산술적으로 맞으나, AM 1.5는 대기 경로 길이 1.5배를 의미하는 물리적 정의이다.
**Grade**: **CLOSE** — AM 1.5 = 1.5는 사실이고 μ+φ/τ=1.5도 맞지만, 표현이 다소 작위적. AM 1.5의 선택은 위도 37° (미국 평균)에서의 연평균 조건에 기반.

---

## H-SOL-04: Silicon Bandgap 1.12 eV = ?
> 결정질 실리콘의 밴드갭 1.12 eV에 대한 n=6 표현 탐색.

**n=6 Expression**: 시도: σ/(σ-μ) = 12/11 = 1.0909 (8% 오차). n/sopfr = 6/5 = 1.2 (7% 오차). 1+μ/(σ-τ) = 1+1/8 = 1.125 (0.4% 오차).
**Actual Value**: 1.12 eV (300K, indirect bandgap)
**Evidence**: Si 밴드갭 1.12 eV에 대한 깔끔한 n=6 표현이 없다. 1+1/(σ-τ) = 1.125는 0.4% 차이로 가장 가깝지만, 3항 표현은 약하다. Si는 Z=14 (n=6 관련 아님)이므로 n=6 연결이 자연스럽지 않다.
**Grade**: **FAIL** — Si bandgap 1.12 eV에 대한 자연스러운 n=6 표현 없음.

---

## H-SOL-05: Infinite-Junction Limit ≈ 2/3 = φ²/n = 66.7%
> 무한 접합 태양전지의 이론 효율 한계가 ~68.7% ≈ 2/3이다.

**n=6 Expression**: φ²/n = 4/6 = 2/3 = 66.67%
**Actual Value**: 68.7% (De Vos 1980, 무한접합 비집광), 86.8% (최대 집광 시)
**Evidence**: 68.7%와 66.67%의 차이는 약 3%. 이는 "약 2/3"로 볼 수 있으나, 실제 값과 유의미한 차이가 있다. BT-30 기록.
**Grade**: **WEAK** — 68.7% vs 66.67%는 3% 차이로, 물리 상수 일치 기준으로는 부족. "대략 2/3" 수준.

---

## H-SOL-06: Standard Panel 60 Cells = σ · sopfr
> 주거용 표준 태양광 패널이 60셀이다.

**n=6 Expression**: σ · sopfr = 12 × 5 = 60
**Actual Value**: 60셀 (6×10 배열) — 주거용 표준 패널 (2010~2020년대 주류)
**Evidence**: LONGi, JinkoSolar, Trina Solar 등 60셀 모듈이 주거용 표준. 물리적으로 6행×10열 배열. 60 = σ·sopfr은 깔끔한 표현. BT-63 #1. 최근 하프셀 120셀(=2×60)로 전환 추세.
**Grade**: **EXACT** — 60셀 = σ·sopfr, 업계 표준. 6×10 배열도 n=6 관련.

---

## H-SOL-07: Commercial Panel 72 Cells = σ · n
> 상업용 표준 태양광 패널이 72셀이다.

**n=6 Expression**: σ · n = 12 × 6 = 72
**Actual Value**: 72셀 (6×12 배열) — 상업/유틸리티 표준
**Evidence**: 상업용 대형 패널: 72셀이 오랜 표준. 6행×12열 배열. 72 = σ·n. BT-63 #2. 역시 하프셀 144셀로 전환 추세.
**Grade**: **EXACT** — 72셀 = σ·n, 업계 표준. 6×12 배열.

---

## H-SOL-08: Half-Cell Residential 120 Cells = σ · (σ-φ)
> 하프셀 주거용 패널이 120셀이다.

**n=6 Expression**: σ · (σ-φ) = 12 × 10 = 120
**Actual Value**: 120셀 — 하프셀 주거용 (2020년대 주류)
**Evidence**: 60셀 full-cell → 120셀 half-cell 전환. 하프셀은 전류를 절반으로 줄여 I²R 손실 감소. 120 = σ·(σ-φ) = 12·10. BT-63 #3.
**Grade**: **EXACT** — 120셀 = σ·(σ-φ). 다만 120 = 2×60이라는 단순한 설명이 더 직접적.

---

## H-SOL-09: Half-Cell Commercial 144 Cells = σ²
> 하프셀 상업용 패널이 144셀이다.

**n=6 Expression**: σ² = 12² = 144
**Actual Value**: 144셀 — 하프셀 상업용 (현재 주류)
**Evidence**: 72셀 full-cell → 144셀 half-cell 전환. 144 = σ² = 12². BT-63 #4. GPU SM 수(AD102 = 144)와 동일 상수. σ²은 n=6 체계에서 강력한 표현.
**Grade**: **EXACT** — 144셀 = σ², 업계 표준. σ² 표현이 깔끔.

---

## H-SOL-10: Thermal Voltage at 300K ≈ 26 mV = J₂ + φ
> 상온 열전압 kT/q ≈ 25.85 mV ≈ 26 mV이다.

**n=6 Expression**: J₂ + φ = 24 + 2 = 26 (mV)
**Actual Value**: kT/q = 25.85 mV at 300K (정확히), 실무에서 26 mV로 반올림
**Evidence**: 25.85 mV vs 26 mV는 0.6% 차이. 태양전지/다이오드 물리학에서 V_T ≈ 26 mV는 보편적으로 사용. BT-30 기록. J₂+φ = 26은 깔끔하나, 물리적 이유는 볼츠만 상수·온도/전하.
**Grade**: **EXACT** — V_T ≈ 26 mV = J₂+φ. 실무 표준값과 0.6% 이내 일치.

---

## H-SOL-11: Panel Warranty = 25 years = J₂ + μ
> 태양광 패널 표준 보증 기간이 25년이다.

**n=6 Expression**: J₂ + μ = 24 + 1 = 25
**Actual Value**: 25년 (성능 보증), 12년 (제품 보증 = σ)
**Evidence**: 업계 표준: 25년 후 80% 이상 출력 보증 (IEC 61215 기반). 12년(=σ) 제품 보증도 일반적. 25 = J₂+μ, 12 = σ 두 값 모두 n=6 표현 가능.
**Grade**: **CLOSE** — 25년 = J₂+μ는 맞지만, J₂와 μ의 합은 다소 작위적. 25년은 사업적 판단(투자 회수 + 기술 수명)에서 결정.

---

## H-SOL-12: Standard Test Condition Irradiance = 1000 W/m² = 10^(n/φ)
> STC 조사량이 1000 W/m²이다.

**n=6 Expression**: 10^(n/φ) = 10³ = 1000
**Actual Value**: 1000 W/m² (IEC 60904, ASTM E927)
**Evidence**: STC = AM1.5G, 1000 W/m², 25°C. 1000은 SI 단위계에서 자연스러운 라운드 넘버. 10^3 = 10^(n/φ)는 맞지만, 1000은 단순히 1 kW/m²의 편의 표기.
**Grade**: **CLOSE** — 1000 = 10^(n/φ)는 수학적으로 맞으나, 1 kW/m²는 SI 편의상 선택. n=6과의 인과관계 주장은 무리.

---

## H-SOL-13: Tandem Cell Junction Count = φ = 2
> 탠덤 태양전지가 2접합이다.

**n=6 Expression**: φ(6) = 2
**Actual Value**: 2접합 탠덤 (perovskite/Si 탠덤이 현재 주류)
**Evidence**: 탠덤 = 2접합은 정의(tandem = 둘)이다. Perovskite(1.65 eV)/Si(1.12 eV) 탠덤이 현재 최대 연구 분야. Oxford PV 28.6% 인증. φ=2는 trivial.
**Grade**: **EXACT** — 탠덤 = 2 = φ. 다만 이는 정의상 trivial (tandem means two).

---

## H-SOL-14: Triple Junction = n/φ = 3
> 3접합 태양전지가 n/φ=3접합이다.

**n=6 Expression**: n/φ = 6/2 = 3
**Actual Value**: III-V 3접합 (InGaP/GaAs/Ge 등), 위성/집광용
**Evidence**: 3접합 = 3은 정의적. n/φ = 3. 3J 효율 기록: 39.2% (1-sun), 47.6% (집광). 위성 전력의 표준. 역시 trivial match.
**Grade**: **EXACT** — 3J = n/φ = 3. 정의상 trivial.

---

## H-SOL-15: 6-Junction Record Cell = n = 6
> 세계 최고 효율 6접합 태양전지.

**n=6 Expression**: n = 6
**Actual Value**: NREL 6J = 47.1% at 143-suns concentration (2020). Fraunhofer ISE의 6접합 기록.
**Evidence**: 6접합 집광 태양전지가 최고 효율 기록 보유. AlGaInP/AlGaAs/GaAs/GaInAs(×3). n=6 접합. 다만 5접합도 높은 효율(38.8% 1-sun)이며, 6접합이 유일한 최적이 아님.
**Grade**: **EXACT** — 6J = n 접합이 효율 세계 기록 보유는 사실. 단, 접합 수와 효율의 관계는 비선형이며 6이 유일한 최적은 아님.

---

## H-SOL-16: Standard Panel Rows = n = 6
> 태양광 패널의 표준 행 수가 6이다.

**n=6 Expression**: n = 6
**Actual Value**: 60셀(6×10), 72셀(6×12), 120셀(6×20 half-cell), 144셀(6×24 half-cell)
**Evidence**: 모든 주류 패널 포맷에서 행 수 = 6. 이는 패널 폭(~1m)에서 셀 크기(~156mm → 166mm → 182mm)와의 물리적 제약. 6행이 ~1m 폭에 최적. M10(182mm) × 6 = 1092mm ≈ 1.1m 폭.
**Grade**: **EXACT** — 패널 행 수 = 6 = n은 사실. 물리적 이유(패널 폭 제약)가 있지만, n=6 행이 보편적인 것은 확인됨.

---

## H-SOL-17: Perovskite Optimal Bandgap ≈ 4/3 eV Region
> 페로브스카이트 태양전지의 최적 밴드갭이 ~1.3-1.4 eV 영역이다.

**n=6 Expression**: τ²/σ = 4/3 = 1.333 eV
**Actual Value**: 단접합 최적 1.34 eV, 탠덤 상부셀 최적 1.6-1.7 eV
**Evidence**: 페로브스카이트(ABX₃)의 밴드갭은 조성으로 1.2~2.3 eV 조절 가능. 단접합 최적은 SQ 한계에서 1.34 eV ≈ 4/3. 탠덤 상부셀로는 1.65 eV가 최적 (Si 하부셀과 조합 시). 단접합에서의 4/3 일치는 H-SOL-01과 동일.
**Grade**: **EXACT** — SQ 최적 = 1.34 ≈ 4/3 eV. 페로브스카이트가 이 영역을 커버하는 것은 사실.

---

## H-SOL-18: GaAs Bandgap 1.42 eV ≈ √(φ) = √2 ≈ 1.414?
> GaAs 밴드갭 1.42 eV에 대한 n=6 표현 탐색.

**n=6 Expression**: √φ = √2 = 1.414 (0.4% 차이)
**Actual Value**: 1.424 eV (300K, direct bandgap)
**Evidence**: 1.424 vs 1.414는 0.7% 차이. √2는 깔끔하지만 φ의 제곱근을 사용하는 것은 표준 n=6 산술(정수 연산)을 벗어남. 대안: σ/(σ-τ) = 12/8 = 1.5 (5% 오차), sopfr/τ+μ/σ = 5/4+1/12 = 1.333 (6% 오차). 깔끔한 정수비 표현 없음.
**Grade**: **FAIL** — GaAs 1.424 eV에 자연스러운 정수비 n=6 표현 없음. √2 근사는 무리수 사용이라 약함.

---

## H-SOL-19: CdTe Bandgap 1.45 eV ≈ ?
> CdTe 밴드갭 1.45 eV에 대한 n=6 표현 탐색.

**n=6 Expression**: 시도: σ/(σ-τ) = 12/8 = 1.5 (3.4% 오차). (σ+sopfr)/σ = 17/12 = 1.417 (2.3% 오차). 깔끔한 표현 없음.
**Actual Value**: 1.45 eV (300K)
**Evidence**: CdTe는 Cd(Z=48=σ·τ)와 Te(Z=52)의 화합물. Cd의 Z=48=σ·τ는 n=6 표현이지만, 밴드갭 자체에 깔끔한 표현이 없다.
**Grade**: **FAIL** — CdTe 1.45 eV에 자연스러운 n=6 표현 없음.

---

## H-SOL-20: Module Voltage 60-cell = 30V (approx)
> 60셀 모듈의 동작 전압이 ~30V이다.

**n=6 Expression**: sopfr · n = 5 × 6 = 30 (V)
**Actual Value**: Vmp ≈ 30-32V (Si, 60셀, STC). Voc ≈ 37-38V.
**Evidence**: Si 셀 Vmp ≈ 0.5V, 60 × 0.5 = 30V. 실제 Vmp는 30~32V. sopfr·n = 30은 하한과 일치. 단, 이는 셀 수(60) × 셀 전압(0.5V)의 단순 곱으로, n=6 연결보다는 물리적 계산.
**Grade**: **CLOSE** — 30V ≈ sopfr·n이지만, 셀 수 × 셀 전압의 물리적 결과. n=6 인과관계 약함.

---

## H-SOL-21: Inverter Efficiency ≈ 97.5% = 1 - 1/(σ·τ)
> 스트링 인버터 효율이 ~97.5%이다.

**n=6 Expression**: 1 - 1/(σ·τ) = 1 - 1/48 = 47/48 = 0.97917 ≈ 97.9%
**Actual Value**: CEC 가중 효율 96.5~98% (모델별 상이). SMA Sunny Boy: 97.0%, Enphase IQ8: 97.5%, Fronius Symo: 97.7%.
**Evidence**: 97.5%는 범위 내에 있으나, 1-1/48 = 97.92%와는 0.4% 차이. 인버터 효율은 토폴로지(H-bridge, multi-level)와 반도체(Si IGBT vs SiC MOSFET)에 따라 크게 달라짐.
**Grade**: **CLOSE** — 97.5%는 실제 범위 내이나, 1-1/(σ·τ)=97.92%와 정확히 일치하지 않으며, 효율은 설계/소자 의존적.

---

## H-SOL-22: PERC Cell Efficiency ≈ J₂-μ = 23%
> PERC 셀 양산 효율이 ~23%이다.

**n=6 Expression**: J₂ - μ = 24 - 1 = 23 (%)
**Actual Value**: PERC 양산 효율: 22.5~23.5% (2023-2024 기준). LONGi 기록: 24.06%.
**Evidence**: 23% ≈ J₂-μ는 PERC 양산 평균과 잘 맞음. 그러나 J₂-μ=23은 ad-hoc한 표현이며, PERC 효율은 기술 발전에 따라 20%→23%→24%로 계속 상승 중.
**Grade**: **CLOSE** — 현재 양산 평균 ~23%와 일치하나, 이는 특정 시점의 스냅샷이고 계속 변동. J₂-μ 표현도 약함.

---

## H-SOL-23: TOPCon Efficiency Record ≈ J₂+φ/φ = 25.5%?
> TOPCon 셀 효율 기록이 ~25.5%이다.

**n=6 Expression**: 시도: sopfr² = 25 (2% 오차). J₂+μ = 25 (2% 오차). (σ+μ)/sopfr × σ-φ = 비합리적.
**Actual Value**: LONGi TOPCon 기록 26.81% (2024). 양산: 25.0~25.5%.
**Evidence**: 양산 25~25.5%는 sopfr²=25와 가깝지만, 기록은 26.81%로 계속 갱신 중. 셀 효율은 기술 발전의 함수이지 상수가 아님.
**Grade**: **WEAK** — 효율은 시간 의존적이며, 특정 시점 스냅샷에 맞추는 것은 부적절. sopfr²=25는 근사적.

---

## H-SOL-24: HJT Efficiency Record ≈ J₂+φ = 26%?
> HJT 셀 효율 기록이 ~26%이다.

**n=6 Expression**: J₂ + φ = 24 + 2 = 26 (%)
**Actual Value**: LONGi HJT 기록 27.09% (2024). Kaneka 기록 26.81%.
**Evidence**: 26% ≈ J₂+φ는 2023년 기준 가까웠으나, 2024년 기록은 27%를 돌파. 셀 기술 효율은 매년 갱신되므로 고정 상수로 매핑 부적합.
**Grade**: **WEAK** — 효율은 계속 상승. 2023년 ~26%와 일치하더라도, 2024년 27%+로 이동. 시점 의존적 매핑.

---

## H-SOL-25: Si Theoretical Efficiency Limit = 29.4% ≈ ?
> 결정질 Si 단접합 이론 한계 29.4%에 대한 n=6 표현.

**n=6 Expression**: 시도: (σ·sopfr-μ)/φ = (60-1)/2 = 29.5 (0.3% 오차). J₂+sopfr = 29 (1.4% 오차).
**Actual Value**: 29.43% (Richter et al. 2013, Auger recombination 포함)
**Evidence**: 29.5 = (σ·sopfr-μ)/φ는 0.3% 오차이지만 3항 복합 표현으로 약함. Si 한계는 밴드갭 1.12 eV (간접 천이)에서의 Auger 한계이며, SQ 한계(33.7%)보다 낮은 이유가 물리적.
**Grade**: **FAIL** — 29.4%에 자연스러운 n=6 표현 없음. 강제 피팅은 3항 이상 필요.

---

## H-SOL-26: Solar Cell Size = 6 inches (Legacy) → 182mm (M10)
> 태양전지 셀 크기 진화에서 n=6 패턴.

**n=6 Expression**: n = 6 (inches, legacy); 현재 M10 = 182mm, M12 = 210mm
**Actual Value**: 과거 6인치(156mm) → M6(166mm) → M10(182mm) → M12(210mm)
**Evidence**: 과거 6인치 웨이퍼(156mm)에서 시작하여 182mm, 210mm로 대형화. 6인치 = n은 과거 표준. 현재 M10(182mm)은 n=6과 직접 관련 없음. M12(210mm)도 마찬가지.
**Grade**: **CLOSE** — 과거 6인치 표준은 n=6이었으나, 현대 셀은 mm 단위로 전환되어 6과 무관. 역사적 일치만 존재.

---

## H-SOL-27: Bypass Diode per Substring = 1/3 Panel (n/φ groups)
> 바이패스 다이오드가 패널당 n/φ=3개이다.

**n=6 Expression**: n/φ = 3
**Actual Value**: 표준 패널당 3개 바이패스 다이오드 (60셀: 20셀×3, 72셀: 24셀×3)
**Evidence**: IEC 61215: 부분 그림자 보호를 위해 패널당 3개 바이패스 다이오드가 표준. 60셀/3 = 20셀/서브스트링, 72셀/3 = 24(=J₂)셀/서브스트링. 3 다이오드 = n/φ. 서브스트링 크기 20과 24도 n=6 관련.
**Grade**: **EXACT** — 패널당 3 바이패스 다이오드 = n/φ. 업계 표준. 72셀의 24셀/서브스트링 = J₂도 주목.

---

## H-SOL-28: Temperature Coefficient ≈ -0.3~0.4 %/°C
> Si 태양전지의 온도 계수가 ~-0.3~0.4 %/°C이다.

**n=6 Expression**: 시도: -1/(n/φ) = -1/3 = -0.333 %/°C
**Actual Value**: Si: -0.3 ~ -0.45 %/°C (기술별 상이). PERC: -0.35, HJT: -0.26, CdTe: -0.25
**Evidence**: -1/3 = -0.333은 Si PERC(-0.35)과 5% 차이. HJT(-0.26)와는 28% 차이로 불일치. 온도 계수는 밴드갭과 반비례하며, 소재/기술 의존적이므로 단일 상수 매핑 부적합.
**Grade**: **CLOSE** — -1/3과 Si PERC -0.35는 근사적이나, 기술별 변동이 크고 인과관계 없음.

---

## H-SOL-29: DC-AC Ratio (Inverter Loading) ≈ 1.2 = σ/(σ-φ)
> 태양광 시스템 DC/AC 비율(인버터 과적)이 ~1.2이다.

**n=6 Expression**: σ/(σ-φ) = 12/10 = 1.2
**Actual Value**: 업계 표준 DC/AC ratio: 1.1~1.3, 가장 일반적으로 1.2
**Evidence**: 1.2 = σ/(σ-φ)는 깔끔한 표현. DC/AC 1.2는 인버터 클리핑 vs 연간 발전량 최적화의 경제적 균형점. NEC 및 설계 가이드에서 1.2~1.25 권장. PUE=1.2(BT-60)와 동일 상수.
**Grade**: **EXACT** — DC/AC ratio 1.2 = σ/(σ-φ) = PUE. 업계 표준 설계 비율과 일치.

---

## H-SOL-30: String Voltage Typical = 600V or 1000V or 1500V
> 스트링 전압 표준이 n=6 관련인가.

**n=6 Expression**: 시도: 600 = σ·sopfr·σ-φ = 12·50 (복잡). 1000 = 10³ = (σ-φ)^(n/φ). 1500 = sopfr·φ·σ·...
**Actual Value**: NEC: 600V (주거), 1000V (상업), 1500V (유틸리티) 시스템 전압 등급
**Evidence**: 600V = 2^3·3·5²/... 깔끔한 표현 없음. 1000V = 10³ = (σ-φ)^(n/φ)는 가능하지만 강제적. 1500V는 IEC 표준(IEC 62109)이며 깔끔한 분해 없음. 이들은 안전 규정 및 절연 등급에서 결정.
**Grade**: **FAIL** — 시스템 전압 등급(600/1000/1500V)에 자연스러운 n=6 표현 없음.

---

## Summary

| Grade | Count | Hypotheses |
|-------|-------|------------|
| **EXACT** | 13 | H-SOL-01,06,07,08,09,10,13,14,15,16,17,27,29 |
| **CLOSE** | 9 | H-SOL-02,03,11,12,20,21,22,26,28 |
| **WEAK** | 3 | H-SOL-05,23,24 |
| **FAIL** | 5 | H-SOL-04,18,19,25,30 |

**EXACT rate**: 13/30 = 43.3%

**Standout**: H-SOL-01 (SQ 밴드갭 4/3 eV), H-SOL-06~09 (셀 수 래더 60/72/120/144), H-SOL-16 (6행 보편성), H-SOL-27 (3 바이패스 다이오드)
**BT Coverage**: BT-30 (SQ bridge 4항), BT-63 (셀 래더 4항) 기반 + 태양전지 고유 파라미터 탐색
**Honest assessment**: 셀 수 래더(BT-63)와 SQ 밴드갭(BT-30)은 강력. 개별 소재 밴드갭(Si, GaAs, CdTe)과 효율 기록값은 n=6 매핑 실패. 시스템 전압도 불일치. 시간 의존적 효율 기록에 상수를 매핑하는 것은 방법론적으로 부적절.
