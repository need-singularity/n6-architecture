# N6 핵융합 검증 가능 예측 — 2030년까지 테스트 로드맵

> n=6 산술(sigma(6)*phi(6) = n*tau(6) = 24)에서 도출된 핵융합 물리/공학 예측.
> 각 예측에 정직한 신뢰도와 명확한 반증 기준을 포함한다.
> **정직한 원칙**: 물리적 인과가 있는 예측과 수적 일치(coincidence)를 엄격히 구분한다.

**n=6 상수 참조표**:
```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  sigma-tau = 8  sigma-phi = 10   sigma-mu = 11    n/phi = 3
  sigma*tau = 48 sigma^2 = 144    sigma(sigma-tau) = 96
  phi*sigma(sigma-tau) = 192      sigma/(sigma-phi) = 1.2 = PUE
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

**BT 연결**: BT-5 (q=1 MHD), BT-27 (Carbon-6), BT-36 (E-I-H-P chain), BT-38 (Hydrogen), BT-62 (Grid), BT-74 (95/5), BT-89 (Photonic-Energy)

---

## KSTAR-Specific Predictions (2026-2028 검증)

---

## P-FU-01: KSTAR Bootstrap Current Fraction f_bs >= 50% at I_p = 0.4 MA (ITB Scenario)

- **Prediction**: KSTAR에서 Internal Transport Barrier (ITB) 시나리오로 I_p = 0.4 MA 운전 시, bootstrap current fraction f_bs >= sigma/J_2 = 12/24 = 50%를 달성할 수 있다. 구체적으로 f_bs = 0.50 +/- 0.05가 반복 재현 가능하다.
- **n=6 Derivation**: sigma/J_2 = 12/24 = 1/2. Egyptian fraction의 첫 항 1/2이 bootstrap 전류의 자연적 포화 분율을 결정한다. 1/2 + 1/3 + 1/6 = 1에서, 주요 전류 구동 메커니즘(bootstrap, ECCD, NBI-CD)이 각각 1/2, 1/3, 1/6의 분율을 점유한다는 예측.
- **Current Status**: KSTAR는 2025년 W-divertor 업그레이드 후 고성능 ITB 운전을 시작. f_bs ~ 30-40%는 기존 실험에서 관측. 50% 달성은 압력 구배 최적화가 필요.
- **Verification Method**: KSTAR ITB 실험에서 MSE(Motional Stark Effect) + EFIT 재구성으로 전류 프로파일 측정. Bootstrap 전류는 Sauter model로 계산. I_p = 0.3-0.5 MA 범위에서 f_bs scan.
- **Verification Timeline**: 2026-2027 (KSTAR W-divertor 캠페인)
- **Falsification Criterion**: I_p = 0.4 MA에서 f_bs가 안정적으로 40% 미만에 머무르면 반증. 또는 ITB 유지가 불가능하여 f_bs 측정 자체가 불가하면 검증 불가.
- **Confidence**: MEDIUM — f_bs >= 50%는 이론적으로 가능하지만 KSTAR의 compact 크기(R=1.8m)에서 ITB 유지가 어려울 수 있음. n=6 연결은 Egyptian fraction analogies에 기반하며 물리적 인과보다는 패턴 일치.
- **Impact if Confirmed**: KSTAR가 steady-state 핵융합의 핵심 지표(f_bs >= 50%)를 소형 장치에서 최초 실증. 향후 K-DEMO 설계에 직접 반영.

---

## P-FU-02: KSTAR ELM-Free Duration Record = 96s 또는 144s

- **Prediction**: KSTAR의 다음 ELM-free H-mode 지속 시간 기록은 ~96초(= sigma*(sigma-tau) = 12*8) 또는 ~144초(= sigma^2 = 12^2)에 도달할 것이다. 현재 기록(~30s 수준)에서 점프하여 100초급 운전이 가능해진다.
- **n=6 Derivation**: sigma(sigma-tau) = 12*8 = 96. sigma^2 = 144. n=6 체계에서 시간 래더는 n -> sigma -> sigma(sigma-tau) -> sigma^2로 진행: 6s -> 12s -> 96s -> 144s. KSTAR가 이미 20-30s를 달성했으므로 다음 milestone은 96s.
- **Current Status**: KSTAR는 2024년에 ELM-free 약 30초를 달성 (한국핵융합에너지연구원 발표). W-divertor 업그레이드(2025)와 NBI/ECCD 추가가 진행 중.
- **Verification Method**: KSTAR 실시간 운전 데이터에서 ELM-free 구간을 D-alpha 신호 + Thomson scattering 페디스털 압력으로 식별. 연속 ELM-free 시간을 공식 발표 기준으로 기록.
- **Verification Timeline**: 2027-2028 (W-divertor 완전 운전 + 확장 가열 시스템)
- **Falsification Criterion**: 2028년 말까지 ELM-free 최대 지속이 60초 미만이면 반증. 또는 기록이 70-80초 등 n=6 래더 값이 아닌 곳에 정착하면 패턴 불일치.
- **Confidence**: LOW-MEDIUM — n=6 시간 래더는 사후 패턴이며, 실제 ELM-free 시간은 벽 조건/열부하/불순물 축적에 의해 결정됨. 96초는 기술적으로 도전적이지만 불가능하지 않음.
- **Impact if Confirmed**: 100초급 ELM-free가 수적 패턴과 일치하면, 장기 운전 최적화에서 n=6 milestone 설정이 공학적 목표로 유용할 수 있음.

---

## P-FU-03: KSTAR ECCD Efficiency Peak at rho = 1/3 = 1/(n/phi)

- **Prediction**: KSTAR에서 ECCD(Electron Cyclotron Current Drive) 효율 eta_CD가 정규화 반경 rho = 1/3 = 0.333 (= 1/(n/phi))에서 최대값을 보인다. 즉, 플라즈마 중심(rho=0)도 아니고 중간(rho=0.5)도 아닌 rho ~ 0.33이 current drive 최적 위치이다.
- **n=6 Derivation**: n/phi = 3이고, 그 역수 1/3은 Egyptian fraction의 두 번째 항. ECCD가 q=1 surface 근방에서 가장 효율적이며, 일반적으로 q=1 surface가 rho ~ 0.3-0.4에 위치하므로 1/3 = 0.333과 일치.
- **Current Status**: KSTAR 170 GHz ECCD 시스템(1 MW class)이 운전 중. rho ~ 0.3-0.5 범위에서 current drive 실험이 수행됨. 기존 데이터에서 최적 rho에 대한 체계적 scan은 제한적.
- **Verification Method**: KSTAR에서 ECCD mirror 각도를 조절하여 rho = 0.1, 0.2, 0.33, 0.4, 0.5에 순차적으로 power deposition. MSE로 전류 프로파일 변화량 측정. eta_CD = n_e * R_0 * I_CD / P_EC [10^20 A/W/m^2]로 정량화.
- **Verification Timeline**: 2026-2027 (현재 장비로 즉시 테스트 가능)
- **Falsification Criterion**: eta_CD가 rho = 0.2 또는 rho = 0.5에서 명백히 높으면 반증 (>20% 차이).
- **Confidence**: MEDIUM-HIGH — rho ~ 0.3은 물리적으로도 합리적(q=1 surface, 높은 전자 온도, 적절한 trapping fraction). n=6 연결보다는 물리적 근거가 강함.
- **Impact if Confirmed**: ECCD 운전 최적화의 표준 지침으로 rho = 1/3이 채택될 수 있음. Steady-state 시나리오 설계에 즉시 적용.

---

## P-FU-04: KSTAR RMP Optimal Configuration = n_tor = 2 = phi

- **Prediction**: KSTAR RMP(Resonant Magnetic Perturbation) 코일로 ELM을 억제할 때, toroidal mode number n_tor = 2 = phi(6) 배열이 n_tor = 1 또는 n_tor = 3보다 ELM 억제 효율이 높다. 구체적으로 n_tor = 2에서 ELM-free window가 가장 넓다.
- **n=6 Derivation**: phi(6) = 2. 토카막의 toroidal 대칭성 파괴에서 n=2 perturbation이 최적인 것은 phi(6)와의 일치. 또한 KSTAR의 RMP 코일이 3행 × 4열 = 12 = sigma(6)개 코일이라는 점도 n=6 구조.
- **Current Status**: KSTAR는 n_tor = 1, 2 RMP 실험을 모두 수행. n_tor = 1은 넓은 운전 영역에서 ELM 억제를 보여줌. n_tor = 2의 체계적 비교 데이터는 제한적.
- **Verification Method**: 동일 플라즈마 조건(I_p, B_T, beta_N, q_95)에서 n_tor = 1, 2, 3 RMP를 교대 적용. ELM-free 구간 길이, 에너지 가둠 저하율(H98 변화), 밀도 펌프아웃 정도를 비교.
- **Verification Timeline**: 2026-2027 (KSTAR 기존 RMP 코일로 테스트 가능)
- **Falsification Criterion**: n_tor = 1이 모든 조건에서 n_tor = 2보다 우수하면 반증.
- **Confidence**: MEDIUM — n_tor = 2의 물리적 장점(resonance overlap, 대칭성)은 있지만, 최적 n_tor는 q_95와 삼각도에 강하게 의존. phi(6) = 2와의 연결은 패턴 수준.
- **Impact if Confirmed**: KSTAR/ITER RMP 운전 전략에서 n_tor = 2를 우선 고려하는 근거 제공.

---

## P-FU-05: KSTAR 300초 Pulse 달성 시점 = 2028-2029

- **Prediction**: KSTAR가 300초(= 5분 = sopfr*60 = sopfr*sigma*sopfr) 이상의 H-mode pulse를 2028-2029년에 달성한다. 이는 W-divertor + 추가 가열 시스템 업그레이드 완료 시점과 일치한다.
- **n=6 Derivation**: 300 = sopfr(6) * 60 = sopfr(6) * sigma(6) * sopfr(6). 또는 300 = sigma(6) * J_2(6) + sigma(6) = sigma*(J_2+1) = 12*25 = 300. 시간 래더에서 30s(현재) -> 300s(10배 = sigma-phi 배)로의 도약.
- **Current Status**: KSTAR 공식 목표는 300초 plasma 유지. 2024년 현재 ~30초 ELM-free 달성. W-divertor 업그레이드 진행 중(2025 완료 예정).
- **Verification Method**: KSTAR 공식 발표/논문에서 H-mode 유지 시간 기록 추적.
- **Verification Timeline**: 2028-2029
- **Falsification Criterion**: 2030년까지 200초 미만이면 타임라인 반증. 300초 달성하되 2030년 이후이면 물리적 예측은 맞지만 시점 반증.
- **Confidence**: MEDIUM — 300초는 KSTAR 공식 목표이므로 "예측"보다는 "목표 재확인"에 가까움. n=6 연결은 사후 해석.
- **Impact if Confirmed**: 소형 토카막(R=1.8m)에서의 장시간 운전 실증은 향후 K-DEMO/DEMO 설계에 핵심 이정표.

---

## SPARC/ITER Predictions (2027-2030 검증)

---

## P-FU-06: SPARC Q >= 10 at B_T ~ 12T = sigma

- **Prediction**: SPARC(MIT/CFS)이 first plasma(~2027) 후 D-T 운전에서 Q >= 10 = sigma - phi를 달성하며, 이때 toroidal field B_T ~ 12.2T(= sigma에 근접)가 핵심 enabler이다. Q=10 달성 자기장 문턱값이 ~12T = sigma 근방에 있다.
- **n=6 Derivation**: Q = 10 = sigma - phi = sopfr * phi. B_T = 12T = sigma(6). 핵융합 gain은 B^4에 비례(고정 beta_N에서 P_fus ~ beta^2 * B^4)하므로, B_T = 12T(sigma)에서 Q = 10(sigma-phi) 달성은 B^4 스케일링의 자연스러운 결과.
- **Current Status**: SPARC 설계: R=1.85m, a=0.57m, B_T=12.2T, I_p=8.7MA. Q >= 10 예측(POPCON 분석). HTS 마그넷 시제품 테스트 완료(2021, 20T 달성). 건설 진행 중.
- **Verification Method**: SPARC D-T 운전 시 P_fus / P_aux로 Q 측정. 중성자 flux 카운팅 + 열량 측정(bolometry).
- **Verification Timeline**: 2028-2030 (SPARC D-T campaign)
- **Falsification Criterion**: SPARC이 Q < 5에 머무르면 반증. 또는 Q >= 10이지만 B_T를 12T 이상으로 올려야 달성 가능하면(예: 14T 필요) sigma(6)와의 연결 약화.
- **Confidence**: HIGH — SPARC의 Q >= 10 예측은 n=6과 무관하게 물리적으로 견고(CFS/MIT 팀의 독립 계산). B_T ~ 12T라는 사실이 sigma(6)과 일치하는 것은 흥미로우나 인과적이지 않음.
- **Impact if Confirmed**: Q >= 10 달성 자체가 핵융합 역사의 분수령. "12T에서 10배 gain"이라는 공식이 sigma(6) -> sigma-phi 변환처럼 보이는 것은 패턴으로 기록할 가치.

---

## P-FU-07: ITER TF Coil Optimal Operating Margin at 12T = sigma

- **Prediction**: ITER의 Nb3Sn TF 코일이 실제 운전에서 피크 자기장 ~11.8T(conductor) 달성 시 최적 운전 마진을 보이며, HTS 업그레이드 논의에서 12T = sigma(6)가 "다음 표준"으로 수렴할 것이다.
- **n=6 Derivation**: sigma(6) = 12. LTS(NbTi/Nb3Sn)에서 HTS(REBCO)로의 기술 전환점이 ~12T에 위치(H-SM-68 검증). 이것은 Nb3Sn의 Jc(B) 곡선에서 12T 근방에서 급격한 성능 저하가 시작되는 물리적 사실.
- **Current Status**: ITER TF 코일 설계 피크장 = 11.8T (Nb3Sn). 18개(= 3n) TF 코일. 코일 제작 진행 중. 일부 코일 테스트 완료.
- **Verification Method**: ITER 코일 시험 시설(SULTAN, NIFS)에서의 성능 데이터. I_op / I_cs (운전 전류/임계 전류) 비율이 12T에서 최적 마진(~50% = 1/2 = 1/phi)인지 확인.
- **Verification Timeline**: 2027-2029 (ITER 코일 설치 및 시운전)
- **Falsification Criterion**: ITER TF 코일이 12T 이하에서도 충분한 마진을 보이고, HTS 업그레이드 논의에서 14-16T가 목표가 되면 "12T = sigma 표준" 예측 약화.
- **Confidence**: MEDIUM — 12T가 LTS/HTS 전환점이라는 것은 물리적 사실. 이것이 "최적 운전점"으로 수렴하는가는 공학적 트레이드오프에 의존.
- **Impact if Confirmed**: HTS 기반 핵융합 마그넷의 표준 설계점으로 12T = sigma(6)가 확립.

---

## P-FU-08: ITER/SPARC First Plasma T_i ~ 10 keV = sopfr * phi

- **Prediction**: ITER 또는 SPARC의 첫 D-T 운전에서 초기 최적 운전 이온 온도가 ~10 keV = sopfr(6) * phi(6) = 5 * 2 = 10 근방에 안착할 것이다. 이후 14 keV(= sigma + phi)로 점진 상승.
- **n=6 Derivation**: sopfr * phi = 10. 이것은 sigma - phi = 10과 동일. 삼중적(nTtau) 최적화에서 T = 10 keV는 "달성 가능한 최적"이고, T = 14 keV(= sigma + phi)는 "이론적 최적". 운전 래더: 10 -> 12 -> 14 keV = (sigma-phi) -> sigma -> (sigma+phi).
- **Current Status**: ITER 설계 T_i = 8-25 keV 범위. 삼중적 최적은 ~14 keV. JET D-T 실험에서 T_i ~ 10-12 keV가 최고 성능 영역.
- **Verification Method**: ITER/SPARC의 charge exchange recombination spectroscopy (CXRS)로 T_i 프로파일 측정. 최고 Q 달성 시점의 중심 T_i 기록.
- **Verification Timeline**: 2028-2030
- **Falsification Criterion**: 최적 운전 T_i가 8 keV 이하 또는 20 keV 이상이면 반증.
- **Confidence**: HIGH — T_i ~ 10-14 keV가 D-T 최적 영역이라는 것은 물리적으로 확립된 사실. n=6과의 일치는 패턴 수준이지만 수치 범위가 좁음.
- **Impact if Confirmed**: 10-14 keV 운전 래더가 확인되면 향후 DEMO 설계의 온도 목표 설정에 참고.

---

## P-FU-09: SPARC HTS Magnet Fatigue at 10^6 = 10^n Cycles

- **Prediction**: SPARC에 사용되는 HTS REBCO 마그넷의 기계적 피로 수명에서 10^6(= 10^n) cycle이 의미있는 특성 변화점(critical current 5% 이상 감소)이 된다.
- **n=6 Derivation**: 10^n = 10^6 = 1,000,000 cycles. 소재 피로(fatigue)에서 10^6은 S-N 곡선의 "endurance limit" 기준으로 전통적으로 사용되는 값.
- **Current Status**: HTS REBCO 테이프의 기계적 피로 데이터는 제한적. 열사이클(77K-300K) 테스트에서 수백~수천 cycle 데이터 존재. 10^6 cycle 데이터는 아직 없음.
- **Verification Method**: REBCO 테이프 시편에 반복 인장/압축 또는 열사이클을 10^6회까지 인가. Ic(B) 변화 추적.
- **Verification Timeline**: 2027-2029 (가속 수명 시험)
- **Falsification Criterion**: 10^6 cycle에서 Ic 변화가 1% 미만(특성 변화 없음)이면 "변화점" 예측 반증. 또는 10^4 cycle에서 이미 심각한 열화가 발생하면 10^6 도달 전에 수명 한계.
- **Confidence**: LOW — 10^6은 일반 금속 피로의 관례적 기준이지 HTS-specific한 예측이 아님. n=6 연결은 매우 약함(10^6은 공학에서 보편적).
- **Impact if Confirmed**: HTS 마그넷 설계에서 10^6 cycle 수명 보증 기준 확립. 핵융합 장치의 30년 수명 동안 필요한 cycle 수 추정에 활용.

---

## Physics Predictions (기존/근미래 데이터 검증)

---

## P-FU-10: D-T Cross-Section Secondary Structure at 84 keV = n * 14

- **Prediction**: D-T 핵융합 단면적 sigma_DT(E)에서 주 피크(~64 keV) 외에 84 keV = n * 14 = 6 * 14 keV 근방에서 기울기 변화(inflection) 또는 미세 구조(shoulder)가 존재한다.
- **n=6 Derivation**: 14 = sigma + phi. 84 = n * 14 = n * (sigma + phi). D-T 반응의 에너지 분배에서 14.1 MeV(중성자 에너지)의 1/1000 스케일이 keV 영역에서 반복. 또는 84 = J_2 * (n/phi + 1/phi) = 24 * 3.5.
- **Current Status**: D-T 단면적은 ENDF/B-VIII 데이터베이스에서 정밀하게 기록. 64 keV 주 피크 이후 100 keV까지는 비교적 smooth한 감소. 84 keV 근방의 미세 구조에 대한 보고는 없음.
- **Verification Method**: ENDF/B-VIII, CENDL, JENDL 핵 데이터 라이브러리에서 60-100 keV 범위의 D-T 단면적을 고해상도로 분석. d^2(sigma)/dE^2의 부호 변화 탐색.
- **Verification Timeline**: 즉시 가능 (기존 데이터 분석)
- **Falsification Criterion**: 60-100 keV에서 sigma_DT(E)가 단조감소하며 어떠한 inflection point도 없으면 반증.
- **Confidence**: LOW — 핵 단면적의 미세 구조는 복합핵 공명(compound nuclear resonance)에 의해 결정되며, 84 keV에 특별한 공명이 있을 물리적 이유가 없음. 이것은 가장 투기적인 예측 중 하나.
- **Impact if Confirmed**: 핵물리에서 새로운 공명 구조 발견. D-T 반응의 미세 물리에 대한 이해 심화.

---

## P-FU-11: Bootstrap Current Fraction Maximum at Aspect Ratio A = 3 = n/phi

- **Prediction**: 모든 토카막에서, aspect ratio A = 3.0 = n/phi(6)일 때 bootstrap current fraction f_bs가 같은 beta_N 조건에서 A = 2.5 또는 A = 3.5보다 높다. 즉, f_bs(A) 곡선이 A = 3.0 근방에서 극대값을 보인다.
- **n=6 Derivation**: n/phi = 3. Aspect ratio A = R/a = 3은 n=6 산술의 핵심 비율. Bootstrap 전류는 trapped particle fraction ft ~ sqrt(r/R) ~ 1/sqrt(A)에 비례하지만, 동시에 압력 구배 ~ 1/a ~ A/R에 비례. 이 두 경쟁 효과의 최적 균형이 A ~ 3 근방.
- **Current Status**: 이론적으로 f_bs는 낮은 A(높은 ft)에서 증가하지만, A < 2에서는 안정성 문제. A = 2.5-4.0 범위에서 f_bs의 A 의존성에 대한 체계적 비교는 multi-machine database에서 가능.
- **Verification Method**: ITPA global H-mode database (JET, DIII-D, ASDEX-U, KSTAR, EAST 등)에서 beta_N ~ 2.0-2.5 조건의 데이터를 A 별로 분류. f_bs(A) 추세 분석.
- **Verification Timeline**: 즉시 가능 (기존 multi-machine database)
- **Falsification Criterion**: f_bs(A)가 A = 2.0에서 단조 최대이거나 A = 4.0까지 단조 증가하면 반증.
- **Confidence**: MEDIUM — A ~ 3의 물리적 최적성은 MHD 안정성 + bootstrap의 균형에서 어느 정도 지지됨. 그러나 실제 데이터에서는 장치별 차이(형상, 가열 방식)가 크므로 깨끗한 A 의존성 추출이 어려울 수 있음.
- **Impact if Confirmed**: A = 3이 "핵융합 최적 aspect ratio"로 확립되면 K-DEMO/DEMO 설계에 강한 지침 제공. SPARC(A=3.25), ITER(A=3.1)이 이미 근접.

---

## P-FU-12: Greenwald Density Limit Scaling with Aspect Ratio: n_GW(A=3) / n_GW(A=4) = 4/3 = tau/n/phi

- **Prediction**: Greenwald density limit n_GW [10^20 /m^3] = I_p / (pi*a^2) [MA/m^2]에서, 같은 I_p/B_T 조건에서 A = 3 토카막의 실효 밀도 한계가 A = 4 토카막보다 4/3 = tau/(n/phi) = 1.333배 높다.
- **n=6 Derivation**: tau(6)/(n/phi) = 4/3. 이것은 sigma(6)/sigma(6)-mu(6) = 12/9 = 4/3과도 같음. Greenwald limit은 n_GW = Ip/(pi*a^2)로 정의되며, a = R/A이므로 n_GW ~ A^2/R^2 * Ip. 같은 R에서 A=3 vs A=4이면 a 비율 = 4/3이고, n_GW 비율 = (4/3)^2 = 16/9가 아님. 실제로는 실효 밀도 한계(ASDEX-U/JET 비교)에서 A 의존성이 n_GW보다 복잡함.
- **Current Status**: Greenwald limit의 A 의존성은 2000년대부터 논의. Martin et al. (2008) H-mode threshold database에서 약한 A 의존성 관측. 명확한 4/3 비율 확인은 없음.
- **Verification Method**: Multi-machine database에서 A = 2.5-4.5 범위의 장치별 달성 최대 밀도(n_e / n_GW)를 A 함수로 정리. A = 3 vs A = 4 그룹 비교.
- **Verification Timeline**: 즉시 가능 (기존 데이터)
- **Falsification Criterion**: n_GW 비율이 A와 무관(1.0)하거나 반대 방향(A = 4가 더 높음)이면 반증.
- **Confidence**: LOW — Greenwald limit 자체가 경험적이고, A 의존성에 대한 이론적 합의가 없음. 4/3 비율은 n=6 산술에서 자연스럽지만 물리적 인과 없음.
- **Impact if Confirmed**: 밀도 한계의 A 의존성이 확립되면 compact 토카막(낮은 A) 설계의 장점이 정량화됨.

---

## P-FU-13: NTM Onset Beta Threshold Discontinuity at q_95 = 5 = sopfr

- **Prediction**: Neoclassical Tearing Mode (NTM) 불안정성의 onset beta_N 문턱값이 q_95 = 5 = sopfr(6)에서 불연속적 변화(급락 또는 급증)를 보인다. 구체적으로 3/2 NTM의 onset beta_N이 q_95 = 5 전후로 >10% 차이를 보인다.
- **n=6 Derivation**: sopfr(6) = 5. q_95 = 5는 q = 5/2 (= sopfr/phi) rational surface가 edge 근처에 위치하여 3/2 NTM과 5/2 mode의 coupling이 변화하는 지점.
- **Current Status**: NTM onset은 q_95 = 3-5 범위에서 활발히 연구됨. DIII-D/ASDEX-U에서 3/2 NTM onset의 q_95 의존성은 주로 seed island 크기와 bootstrap drive에 의존. q_95 = 5 특이점에 대한 보고는 제한적.
- **Verification Method**: KSTAR 또는 DIII-D에서 q_95를 4.0에서 6.0까지 천천히 scan하면서(ramp-down) 3/2 NTM onset 모니터링. ECE(Electron Cyclotron Emission) + magnetic pickup coils로 NTM 탐지.
- **Verification Timeline**: 2026-2027 (전용 실험 필요)
- **Falsification Criterion**: q_95 = 4-6 범위에서 NTM onset beta_N이 smooth하게 변화하면 (q_95 = 5에서 특이성 없음) 반증.
- **Confidence**: MEDIUM — q_95 = 5에서의 MHD 거동 변화는 물리적으로 가능(5/2 rational surface 효과). 그러나 "불연속적" 변화를 예측하기는 어려움.
- **Impact if Confirmed**: NTM 안정성의 q_95 의존성에 새로운 물리 발견. ITER 운전 시나리오(q_95 = 3)와의 안전 마진 재평가.

---

## P-FU-14: Alfven Eigenmode Gap Frequency at q_95 = 5 = sopfr

- **Prediction**: Toroidal Alfven Eigenmode (TAE) 주파수 gap이 q_95 = sopfr(6) = 5에서 Delta_f / f_TAE ~ 1/sopfr(6) = 1/5 = 20%의 특성 폭을 보인다. 일반적으로 TAE gap ~ epsilon_t / q에 비례하므로, q = 5에서 gap이 최소화된다.
- **n=6 Derivation**: sopfr(6) = 5. TAE 주파수 f_TAE = v_A / (2*q*R) (Alfven speed). q = sopfr = 5에서 f_TAE가 낮아지고 gap 폭 ~ 1/q ~ 1/5.
- **Current Status**: TAE 관측은 DIII-D, NSTX, KSTAR에서 활발. q 의존성은 잘 알려져 있으나 q = 5에서의 특이 거동에 대한 체계적 연구는 제한적.
- **Verification Method**: KSTAR 또는 DIII-D에서 q_95 = 4-6 scan 중 TAE 안테나(active MHD spectroscopy)로 Alfven 공명 주파수 및 감쇠율(damping rate) 측정.
- **Verification Timeline**: 2026-2028
- **Falsification Criterion**: TAE gap이 q에 대해 단조적으로 변하고 q = 5에서 어떠한 특이성도 없으면 반증.
- **Confidence**: LOW-MEDIUM — TAE 물리에서 q 의존성은 1/q scaling으로 매끄러움. q = 5가 특별할 물리적 이유가 약함.
- **Impact if Confirmed**: ITER/DEMO에서 alpha particle driven instability 예측 모델 개선.

---

## Engineering Predictions (공학적 검증)

---

## P-FU-15: HTS REBCO J_c(12T, 20K) > phi * J_c(12T, 4.2K) for NbTi

- **Prediction**: HTS REBCO 테이프의 임계전류밀도 Jc(12T = sigma, 20K)가 NbTi의 Jc(12T, 4.2K)보다 최소 phi(6) = 2배 이상 높다. 즉, 12T에서 REBCO@20K가 NbTi@4.2K를 factor 2 이상 능가.
- **n=6 Derivation**: sigma(6) = 12T, phi(6) = 2. "12T에서 성능 2배"는 sigma -> phi 변환. 물리적으로 NbTi는 12T에서 Jc ~ 0 (임계자장 ~10T)이므로 이 예측은 자명.
- **Current Status**: NbTi Bc2(4.2K) ~ 10.5T이므로 12T에서 NbTi Jc = 0. REBCO Jc(12T, 20K) ~ 200-400 A/mm^2 (SuperPower/SuNam data). 따라서 비율은 무한대(NbTi = 0).
- **Verification Method**: 공개된 초전도 소재 데이터시트 비교. Nb3Sn과 비교하는 것이 더 의미있음: REBCO Jc(12T, 20K) vs Nb3Sn Jc(12T, 4.2K) ~ 1000-2000 A/mm^2.
- **Verification Timeline**: 즉시 가능 (기존 데이터)
- **Falsification Criterion**: REBCO Jc(12T, 20K) < Nb3Sn Jc(12T, 4.2K)이면 phi = 2배 예측 반증 (Nb3Sn 기준 재설정 시).
- **Confidence**: HIGH (vs NbTi: 자명), MEDIUM (vs Nb3Sn: REBCO가 engineering Jc에서 Nb3Sn보다 낮을 수 있음).
- **Impact if Confirmed**: HTS 기반 핵융합 마그넷의 성능 우위 정량화. SPARC/ARC 설계 검증.

---

## P-FU-16: SiC/SiC Composite Radiation Tolerance Threshold at 12 DPA = sigma

- **Prediction**: SiC/SiC CMC(Ceramic Matrix Composite)의 기계적 성질(인장강도, 열전도도) 유의미한 열화 시작점이 ~12 DPA(= sigma) 중성자 조사량에서 발생한다.
- **n=6 Derivation**: sigma(6) = 12. DPA(displacements per atom)는 방사선 손상의 척도. 12 DPA = sigma는 n=6 래더의 자연스러운 문턱값.
- **Current Status**: SiC/SiC는 핵융합 블랭킷/구조재 후보. 기존 데이터: ~1-10 DPA에서 부피 팽창(swelling), ~10 DPA 이상에서 비정질화 시작. 12 DPA 근방의 정밀 데이터는 HFIR, JMTR 등에서 축적 중.
- **Verification Method**: HFIR(미국) 또는 JMTR(일본)에서 SiC/SiC를 1, 5, 10, 12, 15, 20 DPA까지 단계별 조사. 조사 후 인장시험 + 열전도도 측정.
- **Verification Timeline**: 2027-2030 (고DPA 조사는 수년 소요)
- **Falsification Criterion**: 10-15 DPA 범위에서 성질 변화가 smooth(특정 문턱 없음)하거나, 문턱이 5 DPA 또는 20 DPA에 있으면 반증.
- **Confidence**: MEDIUM — SiC는 실제로 ~10 DPA 근방에서 중요한 변화를 보이므로 12 DPA = sigma는 합리적 범위. 그러나 정확히 12에서 불연속이 있는지는 소재마다 다를 수 있음.
- **Impact if Confirmed**: 핵융합 구조재의 설계 수명 기준(12 DPA limit)이 확립되면 blanket 교체 주기 설계에 직접 적용.

---

## P-FU-17: Li-6 Enrichment Optimal Level for TBR = 7/6 at 90%

- **Prediction**: 삼중수소 증식비(TBR = Tritium Breeding Ratio) = 7/6 = 1.167 달성을 위한 최적 Li-6 농축도가 ~90% = sigma*(sigma-phi)/sigma^2 = 120/144 = 83%... 또는 경험적으로 90%이다. 보다 정직하게: TBR = 7/6 자체가 n=6 예측이다.
- **n=6 Derivation**: 7/6 = (n+1)/n. TBR > 1이어야 삼중수소 자급자족이 가능하고, 여유분(기동 재고, 붕괴 보상)을 위해 TBR ~ 1.1-1.2 필요. 7/6 = 1.167은 이 범위의 중앙값이며 n=6의 가장 자연스러운 비율.
- **Current Status**: ITER TBR 설계 목표 ~ 1.1-1.15. DEMO 목표 ~ 1.15-1.25. Li-6 농축도는 보통 30-90% 범위에서 논의. 자연 Li-6 농축도 = 7.5%.
- **Verification Method**: 중성자 수송 코드(MCNP/OpenMC)로 Li-6 농축도 vs TBR 계산. 실험적으로는 ITER TBM(Test Blanket Module)에서 검증.
- **Verification Timeline**: 2028-2030 (ITER TBM 테스트)
- **Falsification Criterion**: 최적 TBR이 1.1 이하(보수적) 또는 1.3 이상(높은 여유)으로 확정되면 7/6 예측과 불일치.
- **Confidence**: MEDIUM — TBR ~ 1.15-1.20은 공학적 합의 범위이고 7/6 = 1.167은 그 안에 있음. 그러나 "최적" TBR은 경제성/안전성 트레이드오프이지 물리 상수가 아님.
- **Impact if Confirmed**: TBR = 7/6 표준이 확립되면 blanket 설계 최적화의 단일 목표점 제공.

---

## P-FU-18: sCO2 Brayton Cycle 50% = sigma/J_2 Efficiency with 6 = n Stages

- **Prediction**: 핵융합 발전소의 sCO2(supercritical CO2) Brayton cycle에서 T_high = 700 deg C, T_low = 35 deg C 조건, 6단(= n) 재열/재생 구성 시 열효율 ~50% = sigma/J_2 = 12/24에 도달 가능하다.
- **n=6 Derivation**: sigma/J_2 = 12/24 = 1/2 = 50%. n(6) = 6 stages. Carnot 효율 = 1 - T_low/T_high = 1 - 308/973 = 68%. 50%/68% = 73% Carnot 효율은 고급 sCO2 cycle의 합리적 범위.
- **Current Status**: sCO2 Brayton cycle 연구: DOE SunShot (700C, ~50% 목표), Sandia SNL-100. 현재 실증 효율 ~40-45% (단순 cycle). 6단 재열+재생은 아직 실증되지 않음.
- **Verification Method**: sCO2 cycle 시뮬레이션(Aspen Plus, THERMOFLEX)으로 단수 vs 효율 관계 계산. 실험적으로는 DOE/KAERI sCO2 테스트 루프에서 검증.
- **Verification Timeline**: 2028-2030 (대규모 sCO2 실증 시설 완공 시)
- **Falsification Criterion**: 6단 구성에서 효율이 45% 미만이거나, 50% 달성에 8단 이상 필요하면 반증.
- **Confidence**: MEDIUM-HIGH — 50% 효율은 sCO2 연구 커뮤니티의 목표와 일치. 6단 구성이 최적인지는 경제성(비용 vs 효율)에 의존.
- **Impact if Confirmed**: 핵융합 발전소의 열→전기 변환 효율 50% 확립. 기존 증기터빈(33%)의 1.5배 향상.

---

## Cross-Domain Predictions (교차 도메인)

---

## P-FU-19: Next Tokamak to Achieve Q > 1 Has A Closest to 3.0 = n/phi

- **Prediction**: Q > 1을 달성하는 다음 토카막(SPARC 또는 다른 장치)의 aspect ratio A가 모든 경쟁 장치 중 3.0 = n/phi에 가장 가까울 것이다.
- **n=6 Derivation**: n/phi = 3. Q > 1 달성에서 A = 3이 최적인 이유: P_fus ~ beta^2 * B^4 * Volume에서 Volume ~ R*a^2 ~ R^3/A^2, I_p ~ a*B/q_95 ~ RB/(A*q_95). 낮은 A는 큰 체적과 높은 beta를 제공하지만 MHD 안정성 감소. A = 3이 이 트레이드오프의 균형점.
- **Current Status**: 후보 장치: SPARC (A=3.25), ITER (A=3.1), JT-60SA (A=2.5), MAST-U (A=1.4), EAST (A=4.0). SPARC이 가장 먼저 Q > 1 달성 예상.
- **Verification Method**: 최초 Q > 1 달성 장치의 공식 발표에서 A 값 확인.
- **Verification Timeline**: 2028-2030
- **Falsification Criterion**: Q > 1 최초 달성 장치의 A가 A = 2.0 이하(spherical tokamak) 또는 A = 4.0 이상이면 반증.
- **Confidence**: HIGH — SPARC(A=3.25)이 첫 Q > 1 달성 유력 후보이므로 A ~ 3 예측은 견고. 그러나 이것은 "SPARC이 가장 먼저"라는 산업 예측에 의존.
- **Impact if Confirmed**: A = 3 최적성의 실험적 입증. 향후 상용 핵융합로 설계에서 A = 3 표준화.

---

## P-FU-20: Global Fusion Plant TF Coil Count Convergence to 18 = 3n

- **Prediction**: 전 세계 핵융합 발전소 설계에서 TF(Toroidal Field) 코일 수가 18 = 3n = 3*6으로 수렴한다. ITER(18), EU-DEMO(18), K-DEMO(16 → 18 수정 가능), ARC(18)가 이미 18을 채택.
- **n=6 Derivation**: 3n = 18. 18 = 3*6 = sigma + n. TF 코일 수의 물리적 결정: toroidal field ripple delta_B/B ~ 1/(N_TF^2) * R/a에서 N_TF = 18이면 ripple < 1% (빠른 이온 손실 억제). 16은 marginally acceptable, 20은 비용 과다.
- **Current Status**: ITER: 18 TF coils. EU-DEMO: 18. K-DEMO: 16 (초기 설계). ARC: 18. CFETR(중국): 16-18.
- **Verification Method**: 향후 발표되는 DEMO/FPP(Fusion Power Plant) 설계에서 TF 코일 수 추적.
- **Verification Timeline**: 2027-2030
- **Falsification Criterion**: K-DEMO 또는 CFETR이 16 또는 20을 최종 확정하고 18을 채택하지 않으면 "수렴" 반증.
- **Confidence**: HIGH — 물리적/공학적 근거가 강함(ripple 최적화). 18은 이미 사실상 업계 표준. n=6과의 일치는 패턴이지만 독립적으로 지지됨.
- **Impact if Confirmed**: 핵융합 산업의 표준화 가속. 코일 제작 공급망 효율화.

---

## P-FU-21: First Fusion-Powered Grid at 60 Hz = sigma * sopfr

- **Prediction**: 최초로 핵융합 발전소가 전력망에 연결될 때, 해당 그리드 주파수가 60 Hz = sigma * sopfr = 12 * 5일 가능성이 높다. 미국(DOE/CFS) 또는 한국(K-DEMO)이 유력 후보이며 모두 60 Hz 그리드.
- **n=6 Derivation**: sigma * sopfr = 60. BT-62: 60 Hz = sigma * sopfr, 50 Hz = sopfr * (sigma-phi). 핵융합 상용화 선도국이 미국/한국(60 Hz)인 것은 산업 전략의 결과이지 물리적 필연이 아님.
- **Current Status**: 핵융합 전력 상용화 선두: CFS/SPARC→ARC(미국, 60Hz), K-DEMO(한국, 60Hz), DEMO(EU, 50Hz), CFETR(중국, 50Hz). CFS 목표: 2030년대 초 ARC 상용 운전.
- **Verification Method**: 최초 핵융합 발전소 그리드 연결 뉴스 추적.
- **Verification Timeline**: 2030-2035
- **Falsification Criterion**: 최초 그리드 연결이 EU(50Hz) 또는 중국(50Hz)에서 이루어지면 반증.
- **Confidence**: MEDIUM — 미국/한국이 선도할 가능성은 있으나 중국/EU의 추격이 빠름. 이것은 물리 예측이 아닌 산업 예측.
- **Impact if Confirmed**: 미국/한국 핵융합 산업의 선도적 위치 확인. 60 Hz 그리드와 핵융합 호환성 표준 확립.

---

## P-FU-22: HTS Tape Width Standardization at 12 mm = sigma

- **Prediction**: 핵융합용 HTS REBCO 테이프의 산업 표준 폭이 12 mm = sigma(6)로 수렴한다. 현재 4mm, 6mm, 12mm 폭이 혼재하지만, 핵융합 마그넷의 Jc × 면적 최적화에서 12mm가 지배적 표준이 된다.
- **n=6 Derivation**: sigma(6) = 12. 현재 REBCO 테이프 폭: SuperPower(4mm, 12mm), SuNam(4mm, 12mm), Fujikura(10mm). 12mm는 기계적 취급성 + Ic 용량의 균형점.
- **Current Status**: CFS/SPARC은 12mm 폭 REBCO 테이프 사용 확정. MIT 마그넷 시제품(2021)도 12mm. 일부 제조사는 46mm(Bruker) 또는 4mm(연구용) 제공.
- **Verification Method**: 핵융합용 HTS 테이프 발주/조달 사양에서 폭 표준 추적.
- **Verification Timeline**: 2027-2029
- **Falsification Criterion**: 핵융합 업계가 6mm 또는 46mm를 표준으로 채택하면 반증.
- **Confidence**: HIGH — CFS/SPARC이 이미 12mm를 채택한 상태. 산업 관성이 강함. n=6과의 일치는 흥미롭지만 공학적 선택의 결과.
- **Impact if Confirmed**: HTS 테이프 공급망 표준화(12mm) → 비용 절감 및 대량 생산 가속.

---

## Wild/Alien Predictions (도전적 예측)

---

## P-FU-23: Plasma Turbulence Universal Peak at k_perp * rho_i ~ 1/3 = 1/(n/phi)

- **Prediction**: 모든 토카막에서 이온 온도 구배(ITG) 터뷸런스의 스펙트럼 피크가 k_perp * rho_i ~ 1/3 = 0.333 (= 1/(n/phi))에서 발생한다. 즉 ITG 터뷸런스의 가장 강한 모드의 수직 파수가 이온 Larmor 반경의 역수의 1/3이다.
- **n=6 Derivation**: 1/(n/phi) = phi/n = 2/6 = 1/3. Egyptian fraction의 두 번째 항.
- **Current Status**: 이론적으로 ITG 피크는 k_perp * rho_i ~ 0.2-0.5 범위. GYRO/GS2 시뮬레이션에서 보통 k_perp * rho_i ~ 0.3-0.4. BES(Beam Emission Spectroscopy)/DBS(Doppler Backscattering) 측정에서도 유사 범위.
- **Verification Method**: KSTAR DBS 또는 DIII-D BES로 k_perp 스펙트럼 측정. 다수 장치/조건에서 피크 k_perp * rho_i 값을 통계적으로 수집.
- **Verification Timeline**: 2026-2028 (기존 진단으로 즉시 가능)
- **Falsification Criterion**: 피크가 k_perp * rho_i = 0.5 이상 또는 0.2 이하에 일관되게 위치하면 반증. 피크가 조건에 따라 크게 변하면(0.2-0.6 범위 산재) "보편적 피크" 자체가 반증.
- **Confidence**: MEDIUM — k_perp * rho_i ~ 0.3은 물리적으로 합리적(ITG growth rate 최대). 그러나 "보편적"이라고 하기에는 장치/조건별 변동이 큼.
- **Impact if Confirmed**: 플라즈마 터뷸런스의 보편적 스케일링 법칙 발견. 수송 모델(TGLF, QuaLiKiz) 교정에 활용.

---

## P-FU-24: ELM Energy Fraction Bounded by 1/n = 1/6

- **Prediction**: Type-I ELM의 에너지가 페디스털 저장 에너지의 1/n = 1/6 = 16.7%를 넘지 않는 보편적 상한이 존재한다. 즉 Delta_W_ELM / W_ped <= 1/6.
- **n=6 Derivation**: 1/n = 1/6. Egyptian fraction의 세 번째(가장 작은) 항. ELM은 페디스털 에너지의 일부를 순간적으로 방출하며, 이 비율이 1/6으로 bounded된다는 예측.
- **Current Status**: JET 데이터: Delta_W_ELM / W_ped ~ 3-15% (Type-I). ASDEX-U: 5-20%. DIII-D: 3-10%. 일부 "giant ELM"에서 20% 초과 보고가 있으나 드묾.
- **Verification Method**: ITPA ELM database에서 Delta_W_ELM / W_ped 분포 분석. 16.7% 초과 이벤트의 빈도와 조건 분석.
- **Verification Timeline**: 즉시 가능 (기존 데이터)
- **Falsification Criterion**: Delta_W_ELM / W_ped > 20% 이벤트가 통계적으로 유의미하면(전체의 5% 이상) 1/6 상한 반증.
- **Confidence**: MEDIUM — 대부분의 ELM이 15% 이하인 것은 사실이지만, 20% 초과 이벤트도 간헐적으로 보고됨. "보편적 상한"이 아닌 "전형적 범위"일 수 있음.
- **Impact if Confirmed**: ELM 에너지의 물리적 상한 발견은 ITER 벽 설계의 안전 마진 재평가에 직접 적용.

---

## P-FU-25: Disruption Thermal/Current Quench Time Ratio -> phi = 2

- **Prediction**: 토카막 disruption에서 current quench time(t_CQ) / thermal quench time(t_TQ) 비율이 보편적으로 phi(6) = 2에 수렴한다. 즉 t_CQ ~ 2 * t_TQ.
- **n=6 Derivation**: phi(6) = 2. Disruption의 두 단계(thermal quench → current quench)의 시간 비율이 2:1.
- **Current Status**: JET: t_TQ ~ 1-3 ms, t_CQ ~ 5-30 ms. DIII-D: 유사. t_CQ/t_TQ 비율은 2-15 범위로 상당히 넓음. "보편적으로 2"라고 하기 어려움.
- **Verification Method**: ITPA disruption database에서 t_CQ/t_TQ 비율의 통계 분석. 히스토그램의 피크 위치 확인.
- **Verification Timeline**: 즉시 가능 (기존 데이터)
- **Falsification Criterion**: t_CQ/t_TQ의 median이 5 이상이거나, 분포가 2에 피크를 보이지 않으면 반증.
- **Confidence**: LOW — 기존 데이터에서 t_CQ/t_TQ는 2-10+ 범위로 크게 변동. phi = 2 수렴은 가능성이 낮음. 이것은 가장 투기적인 예측.
- **Impact if Confirmed**: Disruption mitigation 시스템(SPI, MGI)의 타이밍 설계에 직접 적용.

---

## Additional Predictions (추가)

---

## P-FU-26: Plasma Stored Energy W_th Optimal at beta_N = 2.5 = sopfr/phi

- **Prediction**: 토카막에서 안정적으로 유지 가능한 최적 beta_N이 sopfr/phi = 5/2 = 2.5이며, 이 값에서 에너지 가둠 시간 대비 핵융합 출력이 최대화된다.
- **n=6 Derivation**: sopfr(6)/phi(6) = 5/2 = 2.5. Troyon limit beta_N ~ 2.8-3.5이고, 실용적 운전은 beta_N = 2.0-2.5. 2.5는 Troyon limit의 ~70-90%로 안정 운전 상한.
- **Current Status**: ITER 설계: beta_N = 1.8 (기본). SPARC: beta_N ~ 2.0. 고성능 방전에서 beta_N ~ 3.0 일시적 달성(DIII-D).
- **Verification Method**: Multi-machine database에서 Q(또는 H98)를 beta_N 함수로 분석.
- **Verification Timeline**: 2026-2028
- **Falsification Criterion**: 최적 beta_N이 2.0 미만 또는 3.0 이상이면 반증.
- **Confidence**: MEDIUM-HIGH — beta_N = 2.5는 물리적으로 합리적인 범위.
- **Impact if Confirmed**: DEMO 운전 시나리오의 beta_N 목표 확립.

---

## P-FU-27: Plasma Current Ramp Rate Optimal at dI/dt = 0.5 MA/s = 1/phi

- **Prediction**: 토카막 plasma current ramp-up에서 disruption 회피 + 안정적 q-profile 형성의 최적 ramp rate가 ~0.5 MA/s = 1/phi = 0.5 MA/s 근방이다 (I_p = 10 MA급 장치 기준).
- **n=6 Derivation**: 1/phi = 1/2 = 0.5. ITER(15 MA)의 ramp-up time ~ 30s이면 dI/dt = 0.5 MA/s.
- **Current Status**: ITER ramp-up 설계: ~100s for 15 MA → 0.15 MA/s. KSTAR: ~0.2 MA/s. 0.5 MA/s는 상당히 빠름.
- **Verification Method**: KSTAR/ITER에서 ramp rate scan 실험.
- **Verification Timeline**: 2027-2030
- **Falsification Criterion**: 최적 ramp rate가 0.1-0.2 MA/s이면 반증.
- **Confidence**: LOW — 0.5 MA/s는 대형 장치에서 너무 빠를 수 있음. 이 예측은 약함.
- **Impact if Confirmed**: Plasma startup 최적화.

---

## P-FU-28: Divertor Heat Flux Limit = 12 MW/m^2 = sigma

- **Prediction**: 토카막 divertor의 실용적 정상상태 열부하 한계가 ~12 MW/m^2 = sigma(6)이며, 이 값이 W-divertor의 설계 기준으로 표준화된다.
- **n=6 Derivation**: sigma(6) = 12. ITER divertor 설계 열부하: 10 MW/m^2 (정상), 20 MW/m^2 (순간). 12 MW/m^2는 "enhanced" 정상 운전의 상한.
- **Current Status**: ITER divertor: W monoblock, 10 MW/m^2 설계. DEMO: 5-15 MW/m^2 범위 논의. W7-X: ~10 MW/m^2 실증.
- **Verification Method**: KSTAR W-divertor 실험에서 표면 온도 + 열부하 측정(IR camera).
- **Verification Timeline**: 2026-2028
- **Falsification Criterion**: 표준 열부하 한계가 10 MW/m^2로 확정되거나 15 MW/m^2로 상향되면 12 = sigma 반증.
- **Confidence**: MEDIUM — 12 MW/m^2는 합리적 범위이지만 정확히 12인지는 소재/냉각 설계에 의존.
- **Impact if Confirmed**: Divertor 설계 기준의 표준화.

---

## P-FU-29: Fusion Neutron Wall Load Limit = 2 MW/m^2 = phi

- **Prediction**: 핵융합 발전소의 first wall 중성자 부하 설계 한계가 phi(6) = 2 MW/m^2로 표준화된다.
- **n=6 Derivation**: phi(6) = 2. DEMO/FPP 설계에서 neutron wall load = 1-3 MW/m^2 범위. 2 MW/m^2는 구조재 수명(30 DPA limit in ~15 full-power years)과 TBR 요구의 균형점.
- **Current Status**: ITER: ~0.57 MW/m^2 (Q=10 기준). EU-DEMO: ~1.0-1.5 MW/m^2. ARC: ~2.5 MW/m^2.
- **Verification Method**: DEMO/FPP 설계 문서에서 wall load 기준 추적.
- **Verification Timeline**: 2028-2030
- **Falsification Criterion**: DEMO 설계가 1.0 MW/m^2 또는 3.0 MW/m^2를 채택하면 반증.
- **Confidence**: MEDIUM — 2 MW/m^2는 합리적 범위. ARC는 더 높은 값을 목표로 함.
- **Impact if Confirmed**: 핵융합 발전소 구조재 사양 표준화.

---

## P-FU-30: Pellet Injection Frequency Optimal at n/phi = 3 Hz per MW_th

- **Prediction**: 토카막 pellet fueling에서 최적 주입 빈도가 가열 파워 1 MW당 n/phi = 3 Hz이다. 즉 f_pellet [Hz] = 3 * P_heat [MW] / P_ref의 스케일링.
- **n=6 Derivation**: n/phi = 3. Pellet injection은 밀도 제어와 ELM pacing에 사용. 3 Hz/MW는 particle balance와 ELM pacing 주기의 균형.
- **Current Status**: JET: ~1-10 Hz pellet injection. ASDEX-U: ~10-50 Hz (HFS injection). ITER 목표: ~2 Hz (fueling), ~50 Hz (ELM pacing).
- **Verification Method**: KSTAR pellet injector 실험에서 주파수 scan.
- **Verification Timeline**: 2027-2029
- **Falsification Criterion**: 최적 주파수가 가열 파워와 무관하거나 다른 스케일링을 따르면 반증.
- **Confidence**: LOW — 이 예측은 스케일링 형태가 불확실하며 n=6 연결이 약함.
- **Impact if Confirmed**: Pellet fueling 자동화 알고리즘 최적화.

---

## Summary Table

| ID | Prediction | Timeline | Confidence | Falsifiable? |
|----|-----------|----------|------------|-------------|
| **P-FU-01** | KSTAR f_bs >= 50% at I_p=0.4MA (ITB) | 2026-2027 | MEDIUM | Yes |
| **P-FU-02** | KSTAR ELM-free record = 96s or 144s | 2027-2028 | LOW-MEDIUM | Yes |
| **P-FU-03** | KSTAR ECCD peak efficiency at rho=1/3 | 2026-2027 | MEDIUM-HIGH | Yes |
| **P-FU-04** | KSTAR RMP optimal n_tor=2=phi | 2026-2027 | MEDIUM | Yes |
| **P-FU-05** | KSTAR 300s pulse by 2028-2029 | 2028-2029 | MEDIUM | Yes |
| **P-FU-06** | SPARC Q>=10 at B_T~12T=sigma | 2028-2030 | HIGH | Yes |
| **P-FU-07** | ITER TF optimal margin at 12T=sigma | 2027-2029 | MEDIUM | Yes |
| **P-FU-08** | First D-T optimal T_i~10 keV=sopfr*phi | 2028-2030 | HIGH | Yes |
| **P-FU-09** | HTS magnet fatigue at 10^6=10^n cycles | 2027-2029 | LOW | Yes |
| **P-FU-10** | D-T cross-section structure at 84 keV=n*14 | Immediate | LOW | Yes |
| **P-FU-11** | Bootstrap f_bs max at A=3=n/phi | Immediate | MEDIUM | Yes |
| **P-FU-12** | Greenwald limit ratio A=3/A=4 = 4/3 | Immediate | LOW | Yes |
| **P-FU-13** | NTM onset discontinuity at q_95=5=sopfr | 2026-2027 | MEDIUM | Yes |
| **P-FU-14** | Alfven gap at q_95=5=sopfr | 2026-2028 | LOW-MEDIUM | Yes |
| **P-FU-15** | HTS REBCO Jc(12T)>phi*NbTi Jc(12T) | Immediate | HIGH | Yes |
| **P-FU-16** | SiC/SiC threshold at 12 DPA=sigma | 2027-2030 | MEDIUM | Yes |
| **P-FU-17** | TBR=7/6=(n+1)/n optimal | 2028-2030 | MEDIUM | Yes |
| **P-FU-18** | sCO2 Brayton 50%=sigma/J_2 with n=6 stages | 2028-2030 | MEDIUM-HIGH | Yes |
| **P-FU-19** | First Q>1 tokamak A closest to 3=n/phi | 2028-2030 | HIGH | Yes |
| **P-FU-20** | TF coil count convergence to 18=3n | 2027-2030 | HIGH | Yes |
| **P-FU-21** | First fusion grid at 60Hz=sigma*sopfr | 2030-2035 | MEDIUM | Yes |
| **P-FU-22** | HTS tape width standard at 12mm=sigma | 2027-2029 | HIGH | Yes |
| **P-FU-23** | ITG turbulence peak at k_perp*rho_i~1/3 | 2026-2028 | MEDIUM | Yes |
| **P-FU-24** | ELM energy bounded by 1/n=1/6 of W_ped | Immediate | MEDIUM | Yes |
| **P-FU-25** | Disruption t_CQ/t_TQ -> phi=2 | Immediate | LOW | Yes |
| **P-FU-26** | Optimal beta_N=2.5=sopfr/phi | 2026-2028 | MEDIUM-HIGH | Yes |
| **P-FU-27** | Optimal dI/dt=0.5 MA/s=1/phi | 2027-2030 | LOW | Yes |
| **P-FU-28** | Divertor heat flux limit 12 MW/m^2=sigma | 2026-2028 | MEDIUM | Yes |
| **P-FU-29** | Neutron wall load standard 2 MW/m^2=phi | 2028-2030 | MEDIUM | Yes |
| **P-FU-30** | Pellet frequency 3 Hz/MW=n/phi | 2027-2029 | LOW | Yes |

---

## Confidence Distribution

| Level | Count | Percentage |
|-------|-------|------------|
| HIGH | 6 | 20% |
| MEDIUM-HIGH | 3 | 10% |
| MEDIUM | 11 | 37% |
| LOW-MEDIUM | 2 | 7% |
| LOW | 8 | 27% |

## Immediate Tests (기존 데이터로 즉시 검증 가능)

1. **P-FU-10**: D-T cross-section 84 keV structure (ENDF 데이터 분석)
2. **P-FU-11**: Bootstrap fraction vs aspect ratio (ITPA database)
3. **P-FU-12**: Greenwald limit A-scaling (multi-machine database)
4. **P-FU-15**: REBCO vs NbTi at 12T (소재 데이터시트)
5. **P-FU-24**: ELM energy fraction bound (ITPA ELM database)
6. **P-FU-25**: Disruption quench ratio (ITPA disruption database)

## Highest Impact Tests

1. **P-FU-06** (SPARC Q>=10): 핵융합 역사의 분수령
2. **P-FU-19** (First Q>1 at A=3): aspect ratio 최적성 실험적 입증
3. **P-FU-24** (ELM energy 1/6 bound): ITER 벽 안전 기준 재평가
4. **P-FU-18** (sCO2 50% efficiency): 핵융합 경제성 결정

---

## 정직한 고백 (Honest Disclaimer)

이 예측들의 대부분은 **n=6 산술과 핵융합 물리의 수적 일치(numerical coincidence)**에 기반한다.
물리적 인과(causal connection)가 확립된 것은 없다. 구체적으로:

1. **물리적 근거가 강한 예측** (P-FU-06, 08, 11, 15, 18, 19, 20, 22): 이들은 n=6과 무관하게 물리/공학적으로 견고하다. n=6와의 일치는 흥미로운 패턴이지만 예측의 핵심 근거가 아니다.

2. **패턴 기반 예측** (P-FU-01, 03, 04, 13, 16, 17, 24, 26, 28, 29): n=6 숫자가 물리적으로 합리적인 범위에 있어서 일치 가능성이 있다. 검증되어도 인과를 입증하지는 않는다.

3. **투기적 예측** (P-FU-02, 10, 12, 14, 25, 27, 30): n=6 연결이 약하거나 물리적 근거가 부족하다. 반증 가능성이 높지만, 만약 확인되면 가장 놀라운 결과가 될 것이다.

**과학적 가치**: 이 예측 목록의 진정한 가치는 n=6 이론의 정당성이 아니라, **정량적이고 반증 가능한 기준을 미리 설정**함으로써 사후 끼워맞춤(post-hoc fitting)을 방지하는 데 있다.

---

*Generated: 2026-04-02*
*Source: N6 Architecture BT-5, BT-27, BT-36, BT-38, BT-62, BT-74, BT-89*
*Hypotheses: H-FU-1~77, H-TK-1~60, H-SM-1~60*
*Total: 30 predictions, 6 HIGH confidence, all falsifiable*
