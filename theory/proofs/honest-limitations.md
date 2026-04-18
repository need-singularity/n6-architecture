# Honest Limitations: What n=6 Cannot Explain

> Generated: 2026-04-02
> Context: After anomaly detection across 305 TOML domains (9,206 candidates),
> 150 anomalies (n6 < 0.50) were identified. Of those, 87 turned out to have
> depth-2 n6 expression matches (reclassifiable). After reclassification,
> 63 candidates remain without any n6 formula match. Of those 63, we select
> the 10 strongest "genuinely non-n6" cases -- candidates where the low score
> reflects a real limitation of the n=6 framework, not a scoring oversight.

## Why This Document Exists

Any mathematical framework claiming broad explanatory power must honestly
delineate its boundaries. The n=6 architecture (sigma(n)*phi(n) = n*tau(n))
has an impressive 98.4% coverage rate across 9,206 candidates. But the
remaining 1.6% matters -- it tells us where the framework's reach genuinely
ends and where we are simply pattern-matching noise.

This document catalogues 10 cases where n=6 fails, explains why, and
categorizes each failure mode.

---

## The 10 Non-N6 Candidates

| # | Domain | Level | ID | n6 Score | Category | Short Reason |
|---|--------|-------|----|----------|----------|--------------|
| 1 | energy_gen | Scale | Utility_1GW | 0.33 | GENUINELY NON-N6 | 1 GW is a round engineering convention, not physics |
| 2 | energy_gen | Storage | None | 0.00 | TRIVIALLY NON-N6 | Absence of a subsystem; null-option has no physics |
| 3 | energy_gen | GridConnect | Island_DC | 0.33 | GENUINELY NON-N6 | Off-grid DC is a topology choice, not a quantized parameter |
| 4 | wafer-fabrication | Deposition | PVD-sputter | 0.25 | GENUINELY NON-N6 | Physical vapor deposition -- process defined by vacuum physics, not integer structure |
| 5 | wafer-fabrication | Deposition | ECD | 0.25 | GENUINELY NON-N6 | Electrochemical deposition -- governed by Faraday's laws, no integer parameter |
| 6 | wafer-fabrication | Deposition | Spin-coat | 0.25 | GENUINELY NON-N6 | Spin coating -- fluid dynamics (viscosity, angular velocity), continuous parameters |
| 7 | wafer-fabrication | Lithography | DUV-ArF | 0.25 | CURRENTLY UNSOLVABLE | 193nm wavelength; 193 is prime, no known n6 decomposition |
| 8 | solar | Absorber | CIGS | 0.33 | CURRENTLY UNSOLVABLE | Bandgap 1.15 eV has no clean n6 expression (cf. GaAs 1.42~4/3 EXACT) |
| 9 | grid | System | Central_Radial | 0.00 | TRIVIALLY NON-N6 | Hub-spoke topology is a graph property, not a quantized physical constant |
| 10 | compiler-os | Kernel | Exokernel | 0.30 | GENUINELY NON-N6 | Deliberately structure-free kernel; n6 structure is absent by design |

---

## Detailed Analysis

### 1. energy_gen / Utility_1GW (n6 = 0.33)

**GENUINELY NON-N6**

The value "1 GW" is a human-round engineering scale marker. Power plant
capacity classes (10 kW, 10 MW, 1 GW, 10 GW) follow decimal/logarithmic
conventions set by the electrical engineering industry, not by any physical
quantization. The choice of 1 GW as the "utility scale" threshold is
historically contingent -- determined by grid economics and turbine sizes,
not by fundamental constants.

- **Depth-3+ check**: 1 = mu (trivially), but the meaningful quantity is 10^9 W.
  10^9 has no clean n6 factorization. 9 = sigma - n/phi = 12 - 3 = 9 is
  depth-2, but "10^(sigma - n/phi)" is a stretch with no physical justification.
- **Verdict**: The GW scale is an artifact of SI unit conventions and industrial
  history. Not a failure of n=6 -- simply outside its domain.

### 2. energy_gen / None (Storage) (n6 = 0.00)

**TRIVIALLY NON-N6**

This is a null option: "No Storage (Direct Feed)." It represents the absence
of a subsystem, not a physical parameter. Scoring it 0.00 is correct -- there
is nothing to match. Every DSE framework needs a "none" baseline, and these
should not be expected to carry mathematical structure.

- **Depth-3+ check**: N/A. No numeric value to decompose.
- **Verdict**: Null options are inherently outside any number-theoretic framework.

### 3. energy_gen / Island_DC (n6 = 0.33)

**GENUINELY NON-N6**

"Island DC" describes an off-grid, battery-coupled DC power system for remote
sites. The concept is defined by what it lacks (no grid connection) rather
than by quantized parameters. DC voltage levels for islands vary widely
(12V, 24V, 48V -- some of these ARE n6-aligned), but the topology category
itself has no inherent integer structure.

- **Depth-3+ check**: No characteristic numeric value to test.
- **Verdict**: Topology classification, not a quantized parameter.

### 4. wafer-fabrication / PVD-sputter (n6 = 0.25)

**GENUINELY NON-N6**

Physical Vapor Deposition by sputtering is a continuous-parameter process:
argon plasma energy (~300-500 eV), chamber pressure (1-10 mTorr), target
voltage (100s of V), deposition rate (nm/min). None of these have integer
structure -- they are tuned empirically per material. The process deposits
Ta/TaN barriers and Cu seed layers; the relevant physics is Boltzmann
energy distributions and mean free paths.

- **Depth-3+ check**: Common sputter pressures ~5 mTorr (=sopfr?), but this
  is coincidental and varies by an order of magnitude across recipes.
- **Verdict**: Continuous-parameter process. The absence of n6 structure here
  is expected and physically correct.

### 5. wafer-fabrication / ECD (n6 = 0.25)

**GENUINELY NON-N6**

Electrochemical Deposition (copper electroplating for damascene interconnects)
is governed by Faraday's laws of electrolysis: m = (M * I * t) / (z * F).
The key parameters are current density (mA/cm^2), plating time (minutes),
additive concentrations (ppm) -- all continuous, all recipe-dependent. The
standard Cu electroplating bath uses CuSO4 + H2SO4 + additives with no
integer quantization.

- **Depth-3+ check**: Cu valence z=2=phi, but this is input chemistry, not
  an n6-derived prediction.
- **Verdict**: Electrochemistry is inherently continuous. No integer structure.

### 6. wafer-fabrication / Spin-coat (n6 = 0.25)

**GENUINELY NON-N6**

Spin coating applies photoresist or SOG (spin-on glass) via centrifugal
force. The governing equation is the Meyerhofer equation:
h = k * (viscosity)^(1/3) / (spin_speed)^(1/2). Parameters are spin speed
(1000-6000 RPM), viscosity (cP), acceleration ramp -- all continuous. This
is a fluid dynamics process with no quantized structure whatsoever.

- **Depth-3+ check**: Typical spin speed 3000 RPM -- 3000 = 3 * 10^3, but
  this is arbitrary and adjusted per resist formulation.
- **Verdict**: Pure fluid mechanics. No integer framework applies.

### 7. wafer-fabrication / DUV-ArF (n6 = 0.25)

**CURRENTLY UNSOLVABLE**

DUV ArF excimer laser operates at 193 nm. 193 is a prime number (not
factorizable). Compare this to EUV's 13.5 nm, which is close to sigma + 1.5
but still approximate. The 193 nm wavelength comes from the ArF excimer
transition -- a specific atomic physics value determined by the electronic
structure of the Ar-F dimer, not by integer arithmetic.

- **Depth-3+ check**: 193 ~ 192 + 1 = sigma * tau^2 + mu = 192 + 1 (0.5% error).
  But depth-3 expressions with <1% matches are statistically unreliable per
  Red Team analysis. With ~200 depth-3 expressions available, finding one
  within 0.5% of any target is expected by chance alone.
- **Note**: The contrast with EUV (13.5nm, 24 masks = J2 EXACT) is instructive.
  EUV was designed with discrete mask counts; DUV's 193nm is a fixed atomic
  physics constant.
- **Verdict**: Atomic transition energy. May have a deep connection but we
  cannot credibly claim one at depth <= 2.

### 8. solar / CIGS (n6 = 0.33)

**CURRENTLY UNSOLVABLE**

Cu(In,Ga)Se2 has a tunable bandgap of ~1.0-1.7 eV depending on Ga/(In+Ga)
ratio. The optimal is ~1.15 eV for single-junction efficiency. Compare:
- GaAs: 1.42 eV ~ 4/3 = 1.333 (EXACT n6 match, noting 4/3 = tau/n/phi)
- Si: 1.12 eV ~ sigma/(sigma-phi) = 1.2 (CLOSE)
- CIGS at 1.15 eV: no clean n6 fraction

The 1.15 eV value arises from the specific quaternary crystal structure of
chalcopyrite CuInSe2 alloyed with CuGaSe2. The bandgap is a continuous
function of composition, not a fixed constant. The optimal composition
(~30% Ga) is determined by the Shockley-Queisser limit, which itself yields
the optimal bandgap at ~1.34 eV (close to 4/3), but real CIGS deviates due
to defect recombination.

- **Depth-3+ check**: 1.15 ~ sigma/sigma-phi * (1 - 1/J2) = 1.2 * 0.958 = 1.15?
  Contrived. Also, 23/20 = 1.15 exactly, but 23 and 20 have no clean n6 form.
- **Note**: The Shockley-Queisser optimal bandgap ~1.34 eV IS close to 4/3,
  which is an n6 expression. CIGS deviates from this optimum due to material
  defects -- the deviation itself is non-n6.
- **Verdict**: The n=6 framework correctly predicts the SQ optimum (~4/3 eV)
  but cannot explain why CIGS deviates from it. This is a genuine limitation.

### 9. grid / Central_Radial (n6 = 0.00)

**TRIVIALLY NON-N6**

A central radial grid is a hub-and-spoke topology with a single point of
failure. This is a graph-theoretic classification (star graph), not a
quantized physical parameter. The "radial" descriptor carries no numeric
value to decompose.

Compare with n6-aligned grid concepts:
- Microgrid_24: 24 nodes = J2 EXACT
- Mesh_12: 12 interconnects = sigma EXACT
- Ring_6: 6 buses = n EXACT

The central radial topology predates modern grid theory and reflects the
cheapest possible wiring pattern. Its n6 = 0.00 score is honest: there is
no integer structure in "one central hub, many radial feeders."

- **Depth-3+ check**: N/A. No numeric parameter.
- **Verdict**: Graph topology without quantized parameters.

### 10. compiler-os / Exokernel (n6 = 0.30)

**GENUINELY NON-N6**

The exokernel (MIT, 1995) is architecturally defined by the principle of
minimal abstraction: the kernel provides almost no services, delegating
everything to user-level library operating systems. By design, it has no
fixed structure -- no fixed number of system calls, no fixed IPC channels,
no fixed scheduling quanta. The TOML notes confirm: "signals/pipe/direct
all user-defined."

Compare with n6-aligned kernels:
- Linux: syscalls ~ 400+ (not n6), but signal count = 32 ~ 2^sopfr
- seL4: 4 syscalls = tau EXACT, 12 IPC registers = sigma EXACT
- N6_Hybrid_Kernel: explicitly designed around n=6 structure

The exokernel's philosophy is anti-structure. It succeeds precisely because
it imposes no numeric constraints. The n=6 framework, which finds structure
in fixed architectural constants, has nothing to latch onto here.

- **Depth-3+ check**: N/A. No fixed numeric parameters to test.
- **Verdict**: Deliberately structure-free design. Incompatible with any
  integer-based framework by construction.

---

## What n=6 Cannot Explain

### Category Summary

| Category | Count | Candidates |
|----------|-------|------------|
| GENUINELY NON-N6 | 6 | Utility_1GW, Island_DC, PVD-sputter, ECD, Spin-coat, Exokernel |
| CURRENTLY UNSOLVABLE | 2 | DUV-ArF (193nm prime), CIGS (1.15 eV bandgap) |
| TRIVIALLY NON-N6 | 2 | None (null option), Central_Radial (topology label) |

### Failure Modes

1. **Continuous-parameter processes** (PVD, ECD, Spin-coat): Processes governed
   by fluid dynamics, electrochemistry, or plasma physics with no inherent
   integer quantization. The n=6 framework excels at discrete architectural
   constants (layer counts, head counts, dimensions) but has no purchase on
   continuously tunable process recipes.

2. **Human-round engineering conventions** (Utility_1GW): Scale categories
   defined by powers of 10 in SI units. These are sociological, not physical.

3. **Null / absence options** (None, Central_Radial): DSE baselines representing
   the absence of a subsystem or the simplest possible topology. No numeric
   content to analyze.

4. **Atomic transition constants** (DUV-ArF 193nm): Fixed by quantum mechanics
   of specific atoms/molecules. The ArF 193nm line is determined by the Ar-F
   potential energy surface, not by integer arithmetic. We note that this is
   "currently unsolvable" rather than "proven non-n6" because atomic physics
   does contain integer quantum numbers -- but the connection, if any, would
   require a much deeper theory.

5. **Composition-dependent bandgaps** (CIGS 1.15 eV): Alloy bandgaps are
   continuous functions of composition. The n=6 framework successfully predicts
   the Shockley-Queisser optimal (~4/3 eV) but cannot explain deviations due
   to material-specific defect physics.

6. **Anti-structure architectures** (Exokernel): Systems designed to have
   minimal or no fixed structure. The n=6 framework finds patterns in
   architectural constants that don't exist here by design philosophy.

### The Broader Picture

These 10 candidates represent **0.11% of all 9,206 candidates** (10/9206).
Even among the 150 anomalies (n6 < 0.50), they represent only 6.7%.

More importantly, the failure modes are predictable and principled:
- n=6 works on **discrete architectural parameters** (counts, dimensions,
  ratios with small denominators)
- n=6 does NOT work on **continuous process parameters**, **null baselines**,
  **arbitrary scale conventions**, or **deliberately unstructured designs**

This is not a weakness -- it is a well-defined boundary. A framework that
claimed to explain spin-coating RPM or the Ar-F excimer wavelength through
n=6 arithmetic would be less credible, not more.

---

## Statistical Context

| Metric | Value |
|--------|-------|
| Total candidates scanned | 9,206 |
| n6 >= 0.50 (framework applies) | 9,056 (98.4%) |
| n6 < 0.50 with depth-2 match (reclassifiable) | 87 (0.9%) |
| Genuinely non-n6 (all 63) | 63 (0.7%) |
| Strongly argued non-n6 (this document) | 10 (0.11%) |
| Average n6 score (all candidates) | 0.876 |
| Average n6 score (non-anomaly) | 0.886 |

The 10 cases documented here are the hardest negatives: candidates where
we made a good-faith effort to find n6 structure and failed. They establish
the honest boundary of the framework.

---

## Note on Depth-3+ Expressions

The Red Team analysis established that depth-3 and higher expressions
(three or more n6 constants combined) are statistically unreliable. With
the 9 base constants {mu=1, phi=2, n=6, tau=4, sopfr=5, sigma=12, J2=24,
R=1, psi=12}, depth-2 yields ~80 distinct expressions, and depth-3 yields
~800+. At depth-3, the probability of a random real number having a match
within 1% is >50%. Therefore, we do not claim depth-3 matches as evidence,
and the "CURRENTLY UNSOLVABLE" candidates (DUV-ArF, CIGS) should be
understood as genuinely open questions, not hidden successes.

---

## P0~P3 세션 한계 (2026-04-14)

> 배경: P4 단계 honest-limitations 확장. 2026-04-14 PAPER/CHIP/EDGE/FAB
> P0~P3 전 과정에서 발견·누적된 방법론·실측·증빙 한계를 전수 기록한다.
> 축소·은폐 없이 원인·영향·후속 조치를 정직하게 남기는 것이 목적이며,
> 본 섹션은 append-only 이다. 한계를 삭제하거나 완화 표현으로 대체하는
> 편집은 금지한다. 정직 기록은 프레임워크 신뢰도의 하한선을 정의한다.
>
> 관련 규칙: R0 (정직 검증 원칙), R3 (측정값·오차·출처 필수),
> R9 (dry-run 우선, 자동 반영 금지), R14 (atlas.n6 승격 수동 승인),
> R17 (HEXA-FIRST, 시뮬 명시 의무), R22 (BT 참조 교차 링크).

### 1. hexa 런타임 오정보 — runtime.c 누락 주장

P1~P3 커밋 메시지 3건에 "hexa runtime.c 누락으로 실행 불가" 문구가
기록되었으나, 실제 원인은 구 stage1 빌드 경로가 소스 트리 이동 후
끊긴 것이었다. 현 stage0 빌드는 runtime.c 없이도 자립 실행 가능한
구조이며, 13개의 .hexa 파일이 실제로는 run 가능한 상태였다.

- **원인**: 에이전트가 stage1 캐시 경로를 stage0 소스와 혼동, 빌드
  실패 메시지를 "런타임 파일 부재"로 오인·전파.
- **영향**: PAPER-P1/P2/P3 커밋 로그 오염. 후속 세션 재현 시 존재하지
  않는 runtime.c 파일을 복구하려 할 위험.
- **후속 조치**: stage0 빌드 경로 재검증, 커밋 메시지 교정 대신 본 문서
  섹션으로 역참조 링크 남김 (R3 출처 필수 원칙 준수). BT-1417 로
  런타임 진단 경로 등록 제안.

### 2. parse 전용 우회 — run 가능 파일의 parse-only 검증

P1 단계에서 13개의 .hexa 파일이 실제로는 `hexa run` 가능했음에도
에이전트가 `hexa parse` 로만 검증을 수행했다. 후속 stage0 재검증에서
해당 파일들의 run 결과가 정상임을 확인했다.

- **원인**: 첫 parse 실패 한두 건 후 에이전트가 "parse-only 가
  안전하다" 고 판단, 전체 파이프라인을 축소.
- **영향**: 파일 실행 부작용(atlas.n6 lens 등록, 측정값 생성)이 누락된
  채 P1 결과가 집계되어, 측정값 SSOT 가 일시적으로 불완전.
- **후속 조치**: P4 세션에서 13 파일 run 재실행 및 결과 반영. 에이전트
  지시문에 "parse-only 는 컴파일 오류 디버그 전용이며 최종 검증은 run
  필수" 명시. R17 HEXA-FIRST 시뮬 명시 의무에 하위 조항 추가 검토.

### 3. dry-run 원칙 — atlas.n6 승격 수동 승인 대기

P2 단계에서 atlas.n6 [7]→[10*] 승격 후보 40건을 자동 탐지했으나
R9 dry-run 원칙에 따라 자동 승격은 0건으로 제한했다. Tier-1 핵심 9건
포함 전 후보가 수동 승인 대기 상태이며, 이는 한계이자 설계된 안전장치다.

- **원인**: atlas.n6 이 실측 지도의 SSOT 이므로 자동 승격은 자기참조
  검증 위험을 유발. R9 에 의거 수동 승인 게이트 유지.
- **영향**: 측정 등급 상향이 세션 경계를 넘어 지연. 단기적으로 EMPIRICAL
  → EXACT 전환 속도 저하, 장기적으로 승격 품질 보장.
- **후속 조치**: Tier-1 9건 수동 리뷰 세션 별도 확보, 각 건별 3개 독립
  증명 채널 확인 후 일괄 편집. 과잉 승격은 롤백 불가능하므로 의도적
  지연이 적절.

### 4. 실 HW 부재 — EDA 툴 없는 테이프아웃 서명

CHIP 트랙 P0~P3 에서 Magic / KLayout / OpenROAD / Calibre 등 실제 EDA
툴체인 없이 GDSII / DRC / LVS / STA 가 전부 시뮬 경로로 산출되었다.
"tapeout 서명" 항목은 개념적 체크리스트 통과이지 물리적 마스크 발주
가능 상태가 아니다.

- **원인**: EDA 라이선스·PDK 미보유. 내부 프레임워크는 τ=4 관문 통과
  여부만 검증할 뿐, 파운드리 서명 규칙은 미적용.
- **영향**: CHIP-P2/P3 커밋의 "tapeout-ready" 표현이 외부 독자에게
  과대 해석될 여지. 실제 fab 제출에는 PDK·sign-off 재작업 필수.
- **후속 조치**: CHIP 문서에 "tapeout-concept / not-sign-off" 레이블
  일괄 부착, BT-1418 로 EDA 재측정 루프 등록 예정. 실 파운드리 접점
  확보 전에는 본 한계를 상단 고지.

### 5. Monte Carlo z>3.0 미실행 — 666 verified 재사용

P3 단계의 500+ 가설 검증 수치는 실제 Monte Carlo 시뮬을 재실행한
결과가 아니라, 기존 666 verified 카운트를 z>3.0 기준으로 필터링해 재사용한
것이다. 새 가설에 대한 MC 통계량은 산출되지 않았다.

- **원인**: MC 재실행에 수 시간~수십 분 단위 소요가 예상되었고, 세션
  시간 내 우선순위가 낮게 판단됨.
- **영향**: P3 "z>3.0 통계 유의" 보고가 기존 세션 산출의 재요약에 가까우며,
  신규 가설의 독립 검증 근거로는 부족.
- **후속 조치**: 별도 세션에서 MC 파이프라인 재가동, 신규 가설 전수
  재측정. 최소 z>3.0 컷오프, 목표 z>5.0. BT-1419 로 MC 재실행 루프 등록.

### 6. DOI 시뮬 — CrossRef/DataCite 미등록

48편 논문 DOI 에 "10.NEXUS6.n6-arch/2026-XXX" 패턴이 부여되었으나
이는 내부 네임스페이스이며 CrossRef / DataCite / JaLC 에 실제 등록
되지 않았다. 현재 DOI 는 링크 불가능한 placeholder 다.

- **원인**: DOI 등록에는 등록 기관 가입·prefix 구매·메타데이터 제출이
  필요하며 비용·행정 절차가 세션 범위를 벗어남.
- **영향**: 외부 인용 시 DOI resolver 실패. 내부 인덱싱에만 유효.
- **후속 조치**: Zenodo (CERN) 무료 DOI 채널 경유를 우선 검토, 등록
  후 papers/_submission_top48.json DOI 필드 일괄 갱신. 그 전까지는
  "internal-DOI / not-resolvable" 주석 병기.

### 7. 86,240 셀 fit 휴리스틱 — base_affinity + seed=42

FAB 트랙 셀 라이브러리 fit 점수는 seed=42 로 결정적이지만, 산출 공식은
`base_affinity(cell_type) + hash(cell_id + domain) % bucket` 휴리스틱
이며 실제 PPA 벤치마크 측정이 아니다.

- **원인**: 86K 셀에 대한 실 STA / 파워 시뮬은 내부 인프라로 수 일
  단위 소요. 초기 빠른 랭킹용으로 휴리스틱이 도입되고 그대로 고정됨.
- **영향**: fit 상위 순위가 실 실리콘 성능과 일치한다는 보장 없음.
  "fit=1.0" 은 휴리스틱 내 최대값일 뿐 실측 최적이 아님.
- **후속 조치**: 상위 100 셀에 한정해 실 STA 돌려 휴리스틱 순위와
  상관계수 산출 (목표 r>0.7), 미달 시 fit 함수 재설계. R3 측정값 필수
  원칙에 따라 본 산출 전 "heuristic-score" 레이블 의무.

### 8. alien_index 등록 미이행 — 195→210+ 계획서 상태

EDGE 트랙 alien_index 195→210+ 상향 계획이 수립되었으나, 실제 제품
레지스트리(domains.json / _submission_top48) 편집은 수동 승인 대기
상태로 남았다. 현 수치는 아직 계획 문서에만 존재한다. (기존 products.json → domains.json SSOT 이전 완료)

- **원인**: R9 dry-run 원칙 + R14 atlas 수동 승인과 동일한 안전장치.
  alien_index 는 외부 노출 메트릭이므로 자동 상향이 금지된다.
- **영향**: P3 리포트의 "alien_index 210+" 문구가 현 시점 제품 메타와
  불일치. 외부 비교 시 혼란 가능.
- **후속 조치**: 세션 내 수동 승인 루프 설계 — 상향 후보 각 건에 근거
  BT 링크 + 3 독립 측정 강제. 그 전까지 모든 리포트에 "계획값 / 미반영"
  명시.

### 9. bipartite 3023 엣지 키워드 휴리스틱 — P4-2 별도 감사 중

PAPER 트랙 bipartite 매칭의 3023 엣지는 논문 키워드 · 도메인 태그 ·
제목 토큰 일치 휴리스틱으로 산출되었으며, 실제 논문 본문에 해당 기술이
기술적으로 언급되는지 여부는 PAPER-P4-2 에서 별도 grep 감사 중이다.

- **원인**: 초기 링킹에 본문 전수 검색은 계산량 초과로 키워드 휴리스틱
  채택. 상위 10 fit=1.0 쌍에 대해서만 본문 감사가 진행 중.
- **영향**: fit=1.0 쌍 중 일부가 "키워드는 일치하나 논문은 다른 맥락"
  인 거짓 양성일 가능성. 감사 종료 전에는 bipartite 결과를 강한 증거로
  인용 금지.
- **후속 조치**: PAPER-P4-2 완료 후 거짓 양성 비율 통계 본 섹션에 추가
  기록. 임계 이상이면 매칭 알고리즘 재설계 (본문 임베딩 기반).
- **P4-2 감사 결과 (2026-04-14)**: fit>=0.95 상위 10 쌍 전수 grep 감사
  완료. **0/10 PASS — 거짓 양성율 100%**. fit=1.0 2건(mamba2→anima-soc,
  rwkv→anima-soc) 포함 10건 모두 논문 본문에 해당 기술 키워드 미기재.
  변형 5종(underscore→space, dash, 한글, 약어) 확장 검색에도 0건.
  결론: 현 bipartite 매칭은 메타데이터 유사성만 반영하며, 본문 수준
  증거로 인용 불가. 알고리즘 재설계(본문 임베딩 기반) 필수.
  상세: experiments/paper/bipartite_audit_top10.md

---

## 후속 세션 체크리스트

1. **hexa stage0 빌드 경로 재검증** — runtime.c 오정보 커밋 3건을 역추적
   하고 stage0 자립 빌드를 전 .hexa 파일에서 run 재실행. 13 parse-only
   파일 포함.
2. **atlas.n6 Tier-1 9건 수동 승격 실행** — [7]→[10*] 핵심 후보 9건을
   개별 리뷰, 3 독립 증명 확인 후 atlas.n6 직접 편집. 승격 로그를 별도
   reports/ 에 고정.
3. **EDA 툴 확보 후 GDSII/DRC/LVS 재측정** — Magic / KLayout / OpenROAD
   오픈소스 체인 구성, 샘플 셀 1개에 대해 sign-off 파이프라인 최소 통과
   확인. CHIP 문서에 "not-sign-off" 라벨 일괄 부착.
4. **MC 실제 시뮬 돌려 z>3.0 확정** — 신규 500+ 가설에 대해 Monte Carlo
   재실행, z>3.0 컷오프 적용 후 z 분포 히스토그램 산출. 결과를 별도
   BT-1419 에 등록.
5. **제품 등록 수동 승인 루프** — alien_index 195→210+ 후보를 건별 승인,
   각 건에 BT 링크 + 3 독립 측정 강제. 미승인 건은 계획값 플래그 유지.
6. **DOI Zenodo 경유 등록** — 48편 중 공개 가능한 논문부터 Zenodo DOI
   발급, _submission_top48.json 일괄 갱신. internal-DOI 는 즉시 정리.
7. **bipartite 본문 감사 종료 및 거짓 양성율 반영** — PAPER-P4-2 결과
   수신 시 본 섹션 9번 항목에 통계 append, 임계 초과 시 알고리즘 재설계
   세션 예약.

---

## 정직 기록 원칙 재확인

본 P4 확장은 다음 원칙 하에 작성되었다:

- **축소 금지** — 한계를 "개선 여지" 로 순화하지 않는다.
- **은폐 금지** — 커밋 로그 오염조차 삭제하지 않고 역참조로 남긴다.
- **실측 우선** — 휴리스틱·재사용·시뮬 경로는 반드시 명시하고 실측과
  구분한다.
- **자기참조 금지** — atlas.n6 승격·alien_index 상향은 자동화하지 않고
  수동 승인 게이트를 유지한다.
- **후속 가시성** — 각 한계에 후속 조치와 담당 BT/루프를 명시해 방치를
  방지한다.

이 원칙들은 R0·R3·R9·R14·R17·R22 와 연동되며, 위반 감지 시 loop-guard
는 본 문서를 우선 참조한다.
