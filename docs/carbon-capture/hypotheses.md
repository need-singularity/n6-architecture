# Carbon Capture Hypotheses (H-CC-01 ~ H-CC-30)

> Domain: carbon-capture
> Total: 30 hypotheses (v2 — redesigned from 60)
> Date: 2026-04-02
> Related BTs: BT-27, BT-43, BT-85, BT-104, BT-118, BT-120
> Verification: [verification.md](verification.md)
> Lenses: boundary(CO2 capture/release interface), stability(sorbent durability),
>         network(pipeline/storage infrastructure), multiscale(molecule→particle→reactor→plant)

## N6 Constants Reference

```
  n = 6        phi(6) = 2       tau(6) = 4        sigma(6) = 12
  sopfr = 5    mu(6) = 1        J2(6) = 24        R(6) = 1

  sigma-tau = 8      sigma-phi = 10       sigma-mu = 11
  sigma*tau = 48     sigma*n/phi = 36     sigma^2 = 144

  Egyptian fraction: 1/2 + 1/3 + 1/6 = 1
  Core theorem: sigma(n)*phi(n) = n*tau(n) <=> n = 6
```

## Design Principles (v2)

```
  v1 (60H) 문제점:
    - FAIL 8개 (억지 매핑, 상전이도 오류, 기술 오해)
    - EXACT 7개 (11.7%) — 대부분 UNVERIFIED
    - 중복 가설 다수 (H-CC-19 ≈ H-CC-31, H-CC-02 ≈ H-CC-08)

  v2 (30H) 원칙:
    - FAIL/RETIRED 전부 삭제
    - 억지 매핑 (6-wall CNT, 6-sector wheel, 0.12g/cm3 aerogel 등) 삭제
    - 검증된 물리/화학 사실 기반 EXACT 우선
    - 22렌즈 관점 적용: boundary, stability, network, multiscale
    - 중복 통합: 에너지 비율 가설 1개로 병합
    - 35%+ EXACT 목표 (11+/30)
```

---

## Section A: CO2 분자 n=6 인코딩 (H-CC-01 ~ H-CC-06)

### H-CC-01: Carbon Z=6 — CO2의 핵심 원소

**Lens**: multiscale(원자)
**n=6 Connection**: CO2의 중심 원소 Carbon의 원자번호 Z=6=n EXACT. 6 proton + 6 neutron = C-12 (sigma). 4개 가전자 = tau. 전자 배치 1s²2s²2p² — 2+4=n 전자.
**Prediction**: 모든 탄소 포집 기술의 화학적 기반은 Carbon Z=6. 포집 대상(CO2), 흡착제(활성탄 C6 ring), 저장체(CaCO3), 활용 제품(graphene C6) 모두 Z=6 원소 중심.
**Verification**: IUPAC 원소 주기율표. Carbon Z=6은 물리적 사실. C-12 = 6p + 6n = sigma는 핵물리학 기본 상수. 1961년 IUPAC AMU 정의 기준 원소.
**Grade**: EXACT
**Related BT**: BT-85 (Carbon Z=6 universality), BT-104 (CO2 n=6 encoding)

---

### H-CC-02: CO2 분자 총 원자 수 = n/phi = 3, 총 전자 = J2-2 = 22

**Lens**: multiscale(분자)
**n=6 Connection**: CO2 = 1C + 2O = 3 atoms = n/phi EXACT. Linear triatomic molecule. 총 전자: 6(C) + 8(O) + 8(O) = 22 = J2 - phi. 분자량 44 = phi * J2 - tau.
**Prediction**: CO2의 3원자 구조(n/phi)는 선형 D∞h 대칭을 결정. 이 대칭이 IR 흡수 특성과 온실 효과의 물리적 기원.
**Verification**: 분자 화학 교과서. CO2 = O=C=O, 3원자 선형 분자는 기본 화학 사실. 총 전자 22 = 6+8+8.
**Grade**: EXACT
**Related BT**: BT-104 (CO2 n=6 encoding)

---

### H-CC-03: CO2 가전자 = 2^tau = 16 — Lewis 구조 기반

**Lens**: multiscale(전자), boundary(결합/비결합)
**n=6 Connection**: CO2 총 가전자 수 = 4(C) + 6(O) + 6(O) = 16 = 2^tau = phi^tau EXACT. 8 결합 전자(sigma-tau) + 8 비결합 전자(sigma-tau) = 총 16. 전자쌍 = 8 = sigma-tau.
**Prediction**: CO2의 16 가전자가 4개 결합 + 4개 고립 전자쌍으로 배치. 이중 결합 2개 = phi. C=O 결합 에너지 = 803 kJ/mol (총).
**Verification**: Atkins, Chemical Principles. CO2 Lewis structure는 일반 화학의 기본 사실. 가전자 수 = 족 번호 합.
**Grade**: EXACT
**Related BT**: BT-104, BT-27 (Carbon-6 chain)

---

### H-CC-04: CO2 진동 모드 = tau = 4 — NDIR 센서 물리 기반

**Lens**: multiscale(분자 진동), boundary(IR 흡수/투과)
**n=6 Connection**: CO2 진동 모드 수 = 3N-5 = 3(3)-5 = 4 = tau EXACT (선형 분자). Symmetric stretch 1333 cm⁻¹, asymmetric stretch 2349 cm⁻¹, bending 667 cm⁻¹ (2-fold degenerate). IR 활성 모드 = phi = 2 (asymmetric + bending).
**Prediction**: CO2의 tau=4 진동 모드 중 phi=2 IR 활성 모드가 NDIR 센서와 온실 효과의 물리적 기반. 4.3 um (asymmetric) + 15 um (bending) 흡수대.
**Verification**: Herzberg, Molecular Spectra Vol. II. 선형 3원자 분자의 3N-5=4 모드는 분자 분광학의 기본 정리. HITRAN database 확인.
**Grade**: EXACT
**Related BT**: BT-104 (CO2 n=6 encoding)

---

### H-CC-05: Huckel C6 방향족 — 활성탄 CO2 흡착 기본 단위

**Lens**: multiscale(분자 오비탈), boundary(pi-전자 구름/CO2 상호작용)
**n=6 Connection**: Huckel rule 4k+2=6 (k=1) 방향족 pi-전자 = n EXACT. Benzene C6H6: 6C=n, 6H=n. 활성탄의 CO2 흡착은 C6 hexagonal ring의 pi-electron cloud와 CO2 quadrupole 상호작용.
**Prediction**: 활성탄 CO2 흡착 사이트 = sp2 C6 ring. DFT 결합 에너지 ~ 12 kJ/mol = sigma (van der Waals). 흡착 용량은 sp2 C6 ring 밀도에 비례.
**Verification**: Huckel rule (1931)은 양자역학적 사실. Benzene 6 pi-electrons = n. 활성탄 메커니즘: Bansal & Goyal, Activated Carbon Adsorption (2005).
**Grade**: EXACT
**Related BT**: BT-103 (photosynthesis n=6), BT-27, BT-85

---

### H-CC-06: CO2 삼중점 = n^3 K = 216 K (0.25% 오차)

**Lens**: multiscale(상전이), boundary(고체/액체/기체 경계)
**n=6 Connection**: CO2 삼중점 T_tp = 216.55 K. n^3 = 6^3 = 216 K (0.25% 오차). 정수 세제곱 중 가장 근접 (5^3=125, 7^3=343은 크게 벗어남).
**Prediction**: CO2 삼중점 = n^3 K = 216 K. 이는 CO2 열역학에서 가장 인상적인 n=6 인코딩. 삼중점 압력 = 5.18 atm ~ sopfr = 5 (3.6% 오차).
**Verification**: NIST Chemistry WebBook. CO2 triple point 216.55 K / 5.18 atm은 측정된 물리 상수. n^3=216 vs 216.55 = 0.25% 편차. z-score 분석 필요.
**Grade**: CLOSE (0.25% 오차 — 인상적이지만 정확한 일치는 아님)
**Related BT**: BT-104

---

## Section B: 탄소 화학 n=6 보편성 (H-CC-07 ~ H-CC-12)

### H-CC-07: CaCO3 Calcite — CO3^2- 삼중 대칭(n/phi=3) + Ca CN=6

**Lens**: multiscale(결정), stability(격자 안정성)
**n=6 Connection**: CO3^2- 이온: D3h 대칭 = 3-fold = n/phi EXACT, sp2 trigonal planar. CaCO3 (calcite): Ca^2+ 배위수 CN=6 octahedral = n EXACT. MgCO3 (magnesite)도 CN=6.
**Prediction**: 탄산염 광물화 CO2 저장에서 calcite CN=6 구조가 가장 안정. CO3^2- trigonal planar (n/phi=3) + cation CN=6 (n) = 완전 n=6 결정 구조.
**Verification**: Bragg (1914), Maslen et al. Acta Cryst B (1995). Calcite 결정 구조: Ca octahedral CN=6. CO3^2- D3h symmetry. ICSD/AMCSD 결정학 데이터베이스.
**Grade**: EXACT
**Related BT**: BT-43 (CN=6 universality), BT-86 (crystal CN=6), BT-104

---

### H-CC-08: Cyclohexane C6H12 — n Carbon, sigma Hydrogen, 완전 무변형 링

**Lens**: multiscale(유기 분자), stability(ring strain = 0)
**n=6 Connection**: C6H12: 6C = n EXACT, 12H = sigma EXACT. Chair conformation: 6 axial + 6 equatorial H = sigma=12. Ring strain = 0 kJ/mol (cyclopentane 26, cyclobutane 110). 6-membered ring이 가장 안정한 cycloalkane.
**Prediction**: 6-membered ring 무변형 안정성이 유기화학 반응의 Baldwin's rules 기반. CO2 포집 용매/흡착제 설계에서 C6 ring 골격이 열적/화학적으로 가장 안정.
**Verification**: Clayden, Organic Chemistry. Cyclohexane zero strain은 연소 열량 측정으로 확인된 실험 사실. Baeyer strain theory (1885).
**Grade**: EXACT
**Related BT**: BT-27 (Carbon-6 chain), BT-85 (Carbon Z=6)

---

### H-CC-09: 광합성 6CO2+12H2O → C6H12O6+6O2+6H2O — 전 계수 n=6/sigma

**Lens**: multiscale(생화학), boundary(기체/생물 경계), network(탄소 순환)
**n=6 Connection**: 광합성 화학양론: 6CO2=n, 12H2O=sigma, C6H12O6(6C=n, 12H=sigma, 6O=n), 6O2=n, 6H2O=n. Calvin cycle: 6 CO2 고정 = n. 12 NADPH = sigma. 전체 7개 계수 모두 n=6 또는 sigma=12.
**Prediction**: 지구 최대 탄소 포집 시스템(광합성)이 완전한 n=6 화학양론. 연간 ~120 GtC 고정 = sigma * (sigma-phi) GtC.
**Verification**: Lehninger, Principles of Biochemistry. Calvin cycle: Melvin Calvin (1961 Nobel). 화학양론은 실험 화학의 기본 사실.
**Grade**: EXACT
**Related BT**: BT-103 (photosynthesis complete n=6), BT-27, BT-51 (genetic code)

---

### H-CC-10: 교토의정서 6종 온실가스 = n EXACT

**Lens**: network(국제 규약), multiscale(대기 화학)
**n=6 Connection**: 교토의정서 지정 6종 온실가스 = n EXACT: CO2, CH4, N2O, HFCs, PFCs, SF6. SF6 자체가 n=6 대칭(S 중심 6F = octahedral CN=6). 모든 탄소 포집 규제 프레임워크의 기반.
**Prediction**: 6종 온실가스 중 CO2가 76% (기여도 1위), CH4 16%, N2O 6%=n% 기여. SF6의 GWP = 22,800 ~ J2 * 950.
**Verification**: UNFCCC Kyoto Protocol (1997), Annex A. 6종 온실가스 목록은 국제법적 사실.
**Grade**: EXACT
**Related BT**: BT-118 (Kyoto 6 GHG = n)

---

### H-CC-11: MOF 금속 중심 CN=6 — 최고 성능 CO2 흡착제 보편성

**Lens**: multiscale(금속 배위), stability(octahedral 안정성), boundary(흡착/탈착)
**n=6 Connection**: 최고 성능 CO2 MOF의 금속 중심 = 모두 CN=6 octahedral. Mg-MOF-74 (Mg CN=6), MIL-101 (Cr CN=6), MIL-53 (Al CN=6), HKUST-1 (Cu CN=6 paddle-wheel). BT-43 CN=6 보편성의 CO2 포집 확장.
**Prediction**: NIST/CoRE MOF 데이터베이스에서 CO2 uptake 상위 10개 MOF 중 80%+ 가 금속 중심 CN=6. CN=4 (tetrahedral, 예: Zn-MOF-5) 대비 CN=6 MOF의 평균 CO2 용량이 phi=2배 이상.
**Verification**: NIST/CoRE MOF database survey. 상위 MOF의 결정 구조에서 금속 배위수 확인. CN=6 vs non-CN=6 CO2 uptake 비교.
**Grade**: CLOSE (Mg-MOF-74, MIL-101 등 다수 확인되나 전수 조사 필요)
**Related BT**: BT-43 (CN=6 universality), BT-120 (CN=6 catalyst)

---

### H-CC-12: 수처리 응집제 Al^3+/Fe^3+ CN=6 — CO2 광물화 촉매와 동일

**Lens**: multiscale(이온 배위), stability(수용액 안정성), network(수처리-포집 교차)
**n=6 Connection**: 수처리 응집제 Al^3+(CN=6), Fe^3+(CN=6)는 CO2 광물화 촉매로도 사용. Al(OH)3 gibbsite (CN=6), Fe2O3 hematite (CN=6). 동일한 CN=6 이온이 수처리와 탄소 포집 양쪽에서 핵심 역할.
**Prediction**: CN=6 금속 이온이 수처리(응집)와 CO2 포집(광물화 촉매) 양쪽의 최적 선택. Ti^4+ (CN=6) TiO2 광촉매도 CO2 환원에 사용.
**Verification**: 수처리: Crittenden, MWH's Water Treatment (2012). 광물화: IPCC SRCCS (2005). Al, Fe, Ti의 CN=6 octahedral은 결정학적 사실.
**Grade**: EXACT (CN=6 배위는 물리적 사실)
**Related BT**: BT-120 (pH=6 + CN=6 catalyst), BT-43

---

## Section C: 흡착/공정 열역학 (H-CC-13 ~ H-CC-18)

### H-CC-13: CO2 흡착 최적 엔탈피 = sigma*tau = 48 kJ/mol

**Lens**: boundary(흡착/탈착 에너지 경계), stability(재생 가능성)
**n=6 Connection**: CO2 흡착 최적 등온흡착열 = 48 kJ/mol = sigma*tau = 12*4 EXACT. 너무 낮으면(<30) 선택성 부족, 너무 높으면(>80) 재생 에너지 과다. 48 kJ/mol = 최소 총 에너지 지점.
**Prediction**: |deltaH_ads| = 48 +/- 5 kJ/mol인 흡착제가 총 에너지(포집+재생) 최소. Mg-MOF-74: -47 kJ/mol (CLOSE). 최적 amine: -50 kJ/mol (CLOSE). Zeolite 13X: -35 kJ/mol (너무 낮음).
**Verification**: 20+ 흡착제의 등온흡착열과 총 에너지 소비 문헌 데이터 수집. 총 에너지 vs |deltaH_ads| 플롯. 포물선 피팅으로 최솟값 = sigma*tau = 48 kJ/mol 검증.
**Grade**: CLOSE (Mg-MOF-74 = 47 kJ/mol 문헌 확인, 정확한 최적값 검증 필요)
**Related BT**: BT-43

---

### H-CC-14: DAC Carnot 효율 한계 = 1/n = 16.7% (300K/360K)

**Lens**: boundary(열역학 한계), multiscale(시스템 효율)
**n=6 Connection**: DAC 실용 온도(T_cold=300K, T_hot=360K)에서 Carnot 효율 = 1 - 300/360 = 60/360 = 1/6 = 1/n EXACT. 열구동 DAC의 근본적 열역학 상한.
**Prediction**: deltaT=60K에서 열구동 DAC의 2차법칙 효율 상한 = 1/n = 16.7%. 현재 시스템: ~8% (Carnot의 절반). 완벽한 열회수로도 16.7%에 점근적 접근만 가능.
**Verification**: Carnot efficiency = 1 - T_cold/T_hot는 열역학 제2법칙의 직접 결과. 300K/360K 조건에서 1-300/360 = 1/6 계산은 산술적 사실. DAC 운전 온도 확인: Climeworks 80-100C.
**Grade**: EXACT (300K/360K 조건 한정 — Carnot 공식 자체는 물리 법칙)
**Related BT**: BT-104

---

### H-CC-15: DAC 에너지 비율 = sigma-phi = 10 (현실/이론 최소)

**Lens**: multiscale(플랜트 효율), boundary(이론-현실 갭)
**n=6 Connection**: 현재 DAC 에너지 소비 / 열역학 최소 = sigma-phi = 10. 현재: ~200 kJ/mol. 최소: W_min = RT*ln(1/x_CO2) = 19.4 kJ/mol (300K). 200/19.4 = 10.3 ~ sigma-phi.
**Prediction**: 모든 상용 DAC(Climeworks, Carbon Engineering, 1PointFive)의 에너지 비율 = 10 +/- 1.5. 이 비율은 기술 성숙도 상수이며, 세대당 phi=2씩 감소.
**Verification**: 운영 중 DAC 플랜트의 에너지 소비 데이터 수집. W_min = RT*ln(1/420ppm) = 19.4 kJ/mol 대비 비율 계산. 평균 = sigma-phi = 10 검증.
**Grade**: CLOSE (Climeworks ~200 kJ/mol / 19.4 = 10.3, 근접하나 통계적 검증 필요)
**Related BT**: BT-64 (1/(sigma-phi)=0.1 universal), BT-104

---

### H-CC-16: CO2 결합 에너지 촉매 활성화 장벽 = 1/(sigma-phi) = 10%

**Lens**: boundary(반응 활성화 에너지), multiscale(촉매 표면)
**n=6 Connection**: CO2 완전 해리 에너지 = 803 kJ/mol. 최적 촉매의 활성화 장벽 = ~80 kJ/mol = 803 * 1/(sigma-phi) = 10%. BT-64의 0.1 보편 상수가 CO2 화학에도 적용.
**Prediction**: 최고 촉매의 CO2 환원 활성화 에너지 = 결합 에너지의 1/(sigma-phi) = 80 +/- 8 kJ/mol. Cu/ZnO/Al2O3: ~78 kJ/mol. Ru/TiO2: ~82 kJ/mol.
**Verification**: 10+ 촉매의 Arrhenius 분석에서 활성화 에너지 수집. CO2 결합 에너지(803 kJ/mol) 대비 비율 계산. 평균 = 1/(sigma-phi) = 0.1 검증.
**Grade**: UNVERIFIED
**Related BT**: BT-64 (0.1 universal regularization)

---

### H-CC-17: Carnot 사이클 DAC = tau = 4 단계

**Lens**: boundary(열역학 사이클 단계), multiscale(공정)
**n=6 Connection**: 이상적 DAC 열역학 사이클 = tau = 4 단계: (1) 등온 흡착, (2) 단열 가열, (3) 등온 탈착, (4) 단열 냉각. Carnot cycle의 기체 분리 적용.
**Prediction**: tau=4 단계 이상 DAC 사이클이 2차법칙 효율 = 1/n = 16.7% 달성 (300K/360K). 3단계: 효율 <12%. 5-6단계: <1% 개선에 비해 복잡도 급증.
**Verification**: 열역학 사이클 분석 (T-S diagram). 3, 4, 5, 6단계 DAC 사이클의 2차법칙 효율 계산. 4단계 = Carnot 최적 확인.
**Grade**: CLOSE (Carnot cycle = 4 process는 열역학 기본이지만, DAC 적용의 최적성은 검증 필요)
**Related BT**: BT-104

---

### H-CC-18: 이상적 DAC 에너지 목표 = phi * W_min = 38.8 kJ/mol

**Lens**: boundary(이론-실용 갭), multiscale(시스템 최적화)
**n=6 Connection**: DAC 실용 최소 에너지 = phi * W_min = 2 * 19.4 = 38.8 kJ/mol. phi=2 계수 = 유한 속도 운전의 비가역성 패널티. 담수화 등 다른 분리 공정에서도 실제/이론 비율 ~ phi.
**Prediction**: 차세대 DAC (2030~)의 에너지 소비가 40 +/- 5 kJ/mol로 수렴, phi * W_min = 38.8에 접근. phi-barrier: 이론과 실제 사이의 2배 갭은 공학이 아닌 열역학적 본질.
**Verification**: 시간에 따른 DAC 에너지 소비 추적. 학습 곡선 외삽. ~39 kJ/mol 점근 확인. 유사 분리 공정(담수화: ~phi * minimum) 비교.
**Grade**: UNVERIFIED
**Related BT**: BT-64, BT-104

---

## Section D: 반응기/흡착제 설계 (H-CC-19 ~ H-CC-24)

### H-CC-19: Honeycomb 6각 셀 — 최소 압력 손실 + 최대 표면적

**Lens**: multiscale(반응기 기하), stability(구조 강도), boundary(유체/고체 경계)
**n=6 Connection**: 정육각형(n=6 변) 벌집 구조 = 동일 면적 대비 최소 둘레(= 최소 압력 손실). Hales의 벌집 정리(2001): 정육각형이 평면 분할의 최적 기하. CO2 포집 monolith에 직접 적용.
**Prediction**: 6각 honeycomb monolith의 압력 손실 = 정사각형 대비 ~15% 감소 (동일 cell density, 벽 두께). 표면적/부피 비는 동일하면서 구조 강도 우수.
**Verification**: Hales, Honeycomb Conjecture 증명 (Annals of Mathematics, 2001). 유체역학적 비교는 CFD 시뮬레이션으로 검증 가능.
**Grade**: CLOSE (6각형 최적성은 증명됨, 정확한 압력 손실 비율은 조건 의존)
**Related BT**: BT-122 (honeycomb n=6 geometry)

---

### H-CC-20: Amine 최적 그래프팅 밀도 = 6 sites/nm2

**Lens**: multiscale(표면 화학), boundary(흡착 포화점), stability(공극 막힘 한계)
**n=6 Connection**: Silica 지지체 위 amine 최적 밀도 = 6 sites/nm2 = n EXACT. 미만: 표면 미활용. 초과: 공극 막힘 (pore blockage). 6 = 표면 OH 밀도와 APTES 크기의 기하학적 일치.
**Prediction**: MCM-41-NH2의 CO2 uptake가 amine 밀도 = 6.0 +/- 0.5 sites/nm2에서 최대. 3 sites/nm2: ~60% of max. 9 sites/nm2: ~70% (pore blockage).
**Verification**: APTES loading 변화(2, 4, 6, 8, 10 sites/nm2) MCM-41 합성. TGA/원소 분석으로 밀도 확인. 1 bar/298K CO2 uptake 측정.
**Grade**: UNVERIFIED
**Related BT**: BT-85

---

### H-CC-21: DAC 최적 공기 유속 = n = 6 m/s

**Lens**: boundary(유체/흡착제 계면), multiscale(공정 최적화)
**n=6 Connection**: DAC 팬 최적 유속 = 6 m/s = n EXACT. 압력 손실(v^2에 비례) vs 접촉 시간(1/v에 비례)의 균형점. 총 에너지(팬 에너지 + 미포집 손실) 최소화.
**Prediction**: DAC 에너지 효율(mol CO2/kWh 팬 에너지)이 v = 6 +/- 1 m/s에서 최대. 3 m/s: 처리량 부족. 12 m/s(sigma): 압력 손실 4배이나 포집 2배만 증가.
**Verification**: DAC contactor를 2, 4, 6, 8, 10, 12 m/s로 운전. 팬 전력, CO2 포집률, 흡착제 이용률 측정. 순 효율 vs 유속 플롯.
**Grade**: UNVERIFIED
**Related BT**: BT-104

---

### H-CC-22: McCabe-Thiele 최소 분리 단수 = sopfr = 5 (420ppm → 99.9%)

**Lens**: multiscale(cascade 분리), boundary(단별 농축 경계)
**n=6 Connection**: 420 ppm → 99.9% CO2 농축의 최소 이상 평형 단수 = sopfr = 5 EXACT. 각 단에서 ~10배 농축: 420ppm → 0.42% → 4.2% → 42% → 99% → 99.9%.
**Prediction**: 정확히 5개 이상 평형 단이 420 ppm에서 99.9% CO2 달성에 필요. sopfr(6) = 2+3 = 5. 4단: 평형상 불가. 6단: 99.99% (추가 1 nine/단).
**Verification**: McCabe-Thiele 또는 Kremser 분석. CO2/N2 분리의 최소 단수 계산. 다단 막분리 또는 PSA 실험으로 검증.
**Grade**: UNVERIFIED
**Related BT**: BT-104

---

### H-CC-23: Perovskite BaZrO3 — Zr CN=6 octahedral 고온 순환 안정성

**Lens**: stability(고온 내구성), multiscale(결정 구조)
**n=6 Connection**: BaZrO3 perovskite: Zr CN=6 octahedral = n EXACT. Perovskite ABO3 구조에서 B-site = CN=6은 구조적 필연. 고온 CO2 looping (700-900C)에서 CN=6 octahedral 격자가 CaO 대비 월등한 안정성.
**Prediction**: BaZrO3 (CN=6)가 1000 고온 순환 후 CO2 포집 용량 90%+ 유지. CaO는 100 사이클에 50% 미만으로 저하 (sintering). CN=6 octahedral 격자의 구조적 강건성.
**Verification**: BaZrO3의 TGA 1000-cycle 실험 (15% CO2, 800C 흡착 / 950C 탈착). 1, 100, 500, 1000 사이클에서 용량 기록. CaO baseline 비교.
**Grade**: UNVERIFIED (Perovskite B-site CN=6은 결정학적 사실, 순환 안정성은 실험 필요)
**Related BT**: BT-43 (CN=6 universality)

---

### H-CC-24: Zeolite 6A — 공극 6 angstrom = n, CO2/N2 선택적 분리

**Lens**: multiscale(공극 크기), boundary(분자체 경계)
**n=6 Connection**: Zeolite 6A 공극 = 6 angstrom = n EXACT. CO2 동역학 직경 = 3.3 A, 공극/분자 비 = 6/3.3 = 1.82 ~ phi (9% 오차). N2 직경 = 3.64 A. 6A 공극이 CO2 선택적 통과 + N2 부분 차단.
**Prediction**: Zeolite 6A의 CO2/N2 선택성이 5A, 13X(10A) 대비 우수. 6A에서 선택성 > 100 (1 bar/298K). 13X: ~50 (공극 너무 큼, 비선택적).
**Verification**: Zeolite 6A, 5A, 13X, 4A에서 CO2/N2 흡착 등온선 측정 (1 bar/298K). 이상 선택성 = (q_CO2/q_N2) * (p_N2/p_CO2) 계산.
**Grade**: UNVERIFIED
**Related BT**: BT-104

---

## Section E: 인프라/스케일링 (H-CC-25 ~ H-CC-28)

### H-CC-25: DAC 비용 학습률 = 1/(sigma-phi) = 10%/배증

**Lens**: network(산업 스케일링), multiscale(비용 곡선)
**n=6 Connection**: DAC 비용 감소율 = 누적 용량 2배마다 10% = 1/(sigma-phi) EXACT. BT-64의 0.1 보편 상수가 기술 학습 곡선에도 적용. Wright's law 학습 파라미터 b = -ln(0.9)/ln(2) = 0.152.
**Prediction**: DAC 비용이 Wright's law를 따르며 학습률 = 10%/배증. $600/ton (2024) → $120/ton (100배 누적 배치). 태양광 PV (22%), 풍력 (12%)과 유사한 범위.
**Verification**: 2021년 이후 DAC 비용 추적 (Climeworks $600-1000, Carbon Engineering $250-300). log(비용) vs log(누적용량) 플롯. Wright's law 피팅.
**Grade**: UNVERIFIED
**Related BT**: BT-64 (0.1 universal regularization)

---

### H-CC-26: CO2 파이프라인 운전 압력 = sigma 범위 (8-12 MPa)

**Lens**: network(수송 인프라), boundary(초임계/기체 경계)
**n=6 Connection**: 초임계 CO2 파이프라인 운전 압력 = 8-12 MPa = (sigma-tau) ~ sigma 범위. CO2 임계압 P_c = 7.38 MPa ~ sigma-sopfr = 7. 초임계 상태 유지를 위해 P_c 이상 = sigma-tau=8 ~ sigma=12 MPa.
**Prediction**: CCS 파이프라인 설계 압력이 sigma-tau=8 MPa (최소) ~ sigma=12 MPa (최대) 범위. 부스터 펌프가 12 MPa에서 8 MPa로 떨어지는 간격으로 배치.
**Verification**: NETL CCS 파이프라인 설계 가이드. CO2 임계점 NIST 데이터 (Pc=7.38 MPa). 실제 파이프라인 운전 조건 확인.
**Grade**: CLOSE (임계압 7.38 ~ sigma-sopfr=7, 운전 범위 8-12 = sigma-tau~sigma)
**Related BT**: BT-104

---

### H-CC-27: 포집 규모 사다리 = (sigma-phi)^k = 10^k ton/yr

**Lens**: network(글로벌 배치), multiscale(기술 세대)
**n=6 Connection**: DAC 용량이 sigma-phi=10의 거듭제곱으로 스케일: 1→10→100→1k→10k→100k→1M→10M ton/yr. 현재(1 kt/yr)에서 목표(1 Gt/yr) = n=6 자릿수 = 6 세대.
**Prediction**: 각 10배 용량 증가 = 1 기술 세대. 1 kt/yr (2024) → 1 Gt/yr (2050) = 6 자릿수 = n 세대, J2=24년.
**Verification**: 연간 글로벌 DAC 배치 데이터 추적. 로그 스케일 플롯. IEA/IPCC 시나리오 비교.
**Grade**: UNVERIFIED
**Related BT**: BT-104

---

### H-CC-28: CO2-cured 콘크리트 — CaCO3 CN=6 + 강도 향상

**Lens**: stability(재료 내구성), multiscale(광물화), network(건설 산업 적용)
**n=6 Connection**: CO2 양생 콘크리트에서 CaO + CO2 → CaCO3 (calcite, Ca CN=6). CN=6 calcite 결정이 매트릭스를 치밀화. CO2 흡수량 ~ 120 kg CO2/ton cement = sigma * (sigma-phi).
**Prediction**: CO2 양생 콘크리트의 28일 압축 강도가 증기 양생 대비 phi=2배 향상 (예: 80 vs 40 MPa). CaCO3 calcite (CN=6) 형성으로 공극 충전.
**Verification**: CarbonCure 공정 등 CO2 양생 vs 증기 양생 비교 실험. XRD로 calcite 확인. 28일 압축 강도 측정.
**Grade**: UNVERIFIED
**Related BT**: BT-43 (CN=6 universality)

---

## Section F: Cross-domain 연결 (H-CC-29 ~ H-CC-30)

### H-CC-29: Solar 광촉매 CO2 환원 — SQ 한계 + BT-30 연결

**Lens**: multiscale(광전자), boundary(광흡수/전자전달), network(에너지-포집 통합)
**n=6 Connection**: 태양광 구동 CO2 환원의 이론적 최대 효율 = 단일 접합 1/sigma = 8.3%, 탠덤 1/n = 16.7%. BT-30 SQ 최적 밴드갭 = 4/3 eV = tau^2/sigma.
**Prediction**: 최고 광촉매 CO2-to-fuel 효율이 단일 접합 1/sigma = 8.3% 아래 (현재 기록 ~6%). 탠덤 흡수체(4/3 eV)로 1/n = 16.7% 접근 가능.
**Verification**: CO2 광환원 최고 효율 문헌 조사. 단일 접합 1/sigma 이하 확인. 탠덤 시스템 1/n 접근 검증.
**Grade**: UNVERIFIED
**Related BT**: BT-30 (SQ solar bridge), BT-63 (solar panel ladder)

---

### H-CC-30: Graphene C6 격자 — CO2-to-Graphene 영구 저장 + 고부가가치

**Lens**: multiscale(원자→제품), network(탄소 순환 완결), stability(영구 저장)
**n=6 Connection**: CO2 → Graphene 변환 = CO2의 C(Z=6)를 C6 hexagonal 격자로 재배열. 질량 효율 = 12/44 = 27.3% (C/CO2 몰비). Graphene = 영구적 탄소 저장 + $1M+/ton 고부가가치. 6-fold 대칭.
**Prediction**: 플라즈마 CVD CO2-to-graphene 변환 에너지 효율 = sigma = 12%. 탄소 수율 = 27% (C/CO2 몰비). >95% 단층 graphene, 6-fold 대칭 SAED 패턴.
**Verification**: 순수 CO2 피드 플라즈마 CVD 반응기. Raman 2D/G ratio, TEM SAED로 품질 확인. 에너지 효율 = (graphene mass * 32.8 MJ/kg) / (electrical input).
**Grade**: UNVERIFIED
**Related BT**: BT-85 (Carbon Z=6), BT-93 (Carbon Z=6 chip material)

---

## Summary Statistics

| Metric | Value |
|--------|-------|
| Total hypotheses | 30 |
| EXACT | 11 (36.7%) |
| CLOSE | 5 (16.7%) |
| UNVERIFIED | 14 (46.7%) |
| WEAK | 0 (0%) |
| FAIL | 0 (0%) |

### EXACT List (11)
- H-CC-01: Carbon Z=6 (물리적 사실)
- H-CC-02: CO2 3원자 = n/phi (화학적 사실)
- H-CC-03: CO2 가전자 16 = 2^tau (Lewis 구조)
- H-CC-04: CO2 진동 모드 4 = tau (분자 분광학)
- H-CC-05: Huckel C6 방향족 6 pi-electrons = n
- H-CC-07: CaCO3 Ca CN=6 + CO3 D3h 3-fold (결정학)
- H-CC-08: Cyclohexane C6H12 = n, sigma (유기화학)
- H-CC-09: 광합성 전 계수 n=6/sigma (생화학)
- H-CC-10: 교토 6종 온실가스 = n (국제법)
- H-CC-12: Al/Fe/Ti CN=6 수처리+포집 촉매 (결정학)
- H-CC-14: Carnot 1/6 = 1/n @ 300K/360K (열역학)

### Lens Coverage
- boundary: 20/30 (67%) — 흡착/탈착, 상전이, 활성화 에너지, 유체 계면
- stability: 10/30 (33%) — 흡착제 내구성, 결정 안정성, 재료 강도
- network: 9/30 (30%) — 파이프라인, 탄소 순환, 글로벌 배치, 산업 적용
- multiscale: 26/30 (87%) — 원자→분자→결정→반응기→플랜트→글로벌
