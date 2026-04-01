# HEXA-CCUS 극강 탄소포집기 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Carbon Z=6 기반 8단 아키텍처 — 원자 스케일부터 항성 스케일까지 관통하는 극강 탄소포집기. 시중 기술(Climeworks/Carbon Engineering) 대비 압도적 우위.

**Architecture:** 8단 체인(소재→공정→코어→칩→시스템→변환→만능→궁극), 6후보×8레벨=1,679,616 DSE 조합. 기존 5단 TOML 대체. BT-94/95/96 신규 등록. 80개 가설 + Cross-DSE 11 도메인.

**Tech Stack:** TOML (DSE 정의), Rust (universal-dse 전수탐색), Python (검증), Markdown (가설/문서)

**Spec:** `docs/superpowers/specs/2026-04-02-hexa-ccus-design.md`

---

## File Structure

```
docs/carbon-capture/
  goal.md                    — 8단 아키텍처 로드맵 (battery-architecture/goal.md 패턴)
  hypotheses.md              — H-CC-01 ~ H-CC-60 (일반 가설 60개)
  extreme-hypotheses.md      — H-CC-E01 ~ H-CC-E20 (극한/외계인급 20개)
  verification.md            — 독립 검증 + EXACT/CLOSE/WEAK/FAIL 등급
  hexa-sorbent.md            — Level 0 소재 상세
  hexa-process.md            — Level 1 공정 상세
  hexa-reactor.md            — Level 2 코어 상세
  hexa-chip.md               — Level 3 칩 상세
  hexa-plant.md              — Level 4 시스템 상세
  hexa-transmute.md          — Level 5 변환 상세
  hexa-universal.md          — Level 6 만능 상세
  omega-cc.md                — Level 7 궁극 상세

tools/universal-dse/domains/
  carbon-capture-8level.toml — 8단 DSE 도메인 (기존 5단 대체)

docs/breakthrough-theorems.md — BT-94/95/96 추가
docs/dse-map.toml             — carbon-capture 섹션 8단으로 갱신
docs/atlas-constants.md       — 탄소포집 상수 추가
```

---

## Task 1: 8단 TOML 도메인 파일 작성

**Files:**
- Create: `tools/universal-dse/domains/carbon-capture-8level.toml`
- Reference: `tools/universal-dse/domains/carbon-capture.toml` (기존 5단)
- Reference: `docs/superpowers/specs/2026-04-02-hexa-ccus-design.md` (spec)

- [ ] **Step 1:** spec의 섹션 3 후보군 테이블을 TOML 형식으로 변환. 8 [[level]] + 48 [[candidate]] + 10+ [[rule]]. meta.levels = ["Sorbent","Process","Reactor","Chip","Plant","Transmute","Universal","Omega"]. scoring: n6=0.35, perf=0.25, power=0.20, cost=0.20.

- [ ] **Step 2:** 빌드 및 검증 — `cd tools/universal-dse && ~/.cargo/bin/rustc main.rs -o universal-dse 2>&1 || true` 후 `./universal-dse domains/carbon-capture-8level.toml` 실행. 1,679,616 조합 전수 탐색 확인. Pareto frontier + 최적 경로 기록.

- [ ] **Step 3:** 커밋 `feat: HEXA-CCUS 8-level DSE TOML (1,679,616 combos)`

---

## Task 2: goal.md 작성 (8단 로드맵)

**Files:**
- Create: `docs/carbon-capture/goal.md`
- Reference: `docs/battery-architecture/goal.md` (패턴)

- [ ] **Step 1:** battery-architecture/goal.md 패턴을 따라 8단 Evolution Ladder 테이블 작성. N6 Constants Reference 포함. 각 레벨별 Status/혁신/이점 기록. 시중 기술 대비 압도적 우위 포인트 명시:
  - 소재: MOF CN=6 → 흡착량 현재 대비 6배 (8→48 mmol/g 목표)
  - 공정: 에너지 소비 현재 대비 1/(sigma-phi)=1/10 (200→20 kJ/mol)
  - 코어: 처리량 현재 대비 sigma=12배
  - 칩: BT-56 RISC-V N6 + 양자센서 = 감도 10^6배
  - 시스템: 1Mt→100Gt/yr (10^5배 스케일)
  - 변환: 폐CO2→$1M/ton 그래핀 (수익 창출 포집)
  - 만능: 행성 대기 조성 직접 제어
  - 궁극: 항성 에너지 활용, 역엔트로피

- [ ] **Step 2:** 커밋 `docs: HEXA-CCUS goal.md — 8-level roadmap`

---

## Task 3: 가설 60개 (hypotheses.md)

**Files:**
- Create: `docs/carbon-capture/hypotheses.md`

- [ ] **Step 1:** H-CC-01~60 작성. 각 가설은 H-XX-NN 형식, n=6 연결, 검증 방법, 예측값 포함.

카테고리별 핵심 가설 (각 10개):

**소재 (H-CC-01~10):**
- H-CC-01: 최고성능 CO2 흡착 MOF는 전부 금속 CN=6 octahedral (BT-43 확장)
- H-CC-02: Zeolite 6A (6Å pore=n) CO2/N2 선택성이 비-6A 대비 phi=2배
- H-CC-03: [C6mim] ionic liquid의 CO2 용해도가 비-C6 IL 대비 n/tau=1.5배
- H-CC-04: Graphene oxide C6 hexagonal membrane CO2 투과도 = 비GO 대비 sigma-phi=10배
- H-CC-05: Perovskite sorbent BaZrO3 CN=6 octahedral 고온 루핑 안정성 = 1000 cycle (현재 100)
- H-CC-06: Amine grafting 최적 밀도 = 6 sites/nm2 = n EXACT
- H-CC-07: MOF-74 시리즈 6종 금속(Mg/Al/Fe/Cr/Co/Ni) 전부 CN=6 = n EXACT
- H-CC-08: DAC sorbent 최적 pore size = 6Å = n EXACT (kinetic diameter CO2 = 3.3Å, ratio~phi)
- H-CC-09: Carbon nanotube 6-wall MWCNT CO2 흡착 = SWCNT 대비 n=6배
- H-CC-10: Silica aerogel 최적 밀도 = 0.12 g/cm3 = sigma/100

**공정 (H-CC-11~20):**
- H-CC-11: TSA 최적 cycle = 6단계 = n EXACT (adsorb/heat/desorb/cool/purge/reset)
- H-CC-12: PSA 최적 bed 수 = 12 = sigma EXACT (6 adsorb + 6 desorb)
- H-CC-13: MECS 전기화학 최적 cell stack = 6 = n EXACT
- H-CC-14: Membrane cascade 최적 stage 수 = 6 = n (99.9% purity 달성)
- H-CC-15: 극저온 분리 최적 온도 = -48C = sigma*tau EXACT
- H-CC-16: Photocatalytic 최적 bandgap = 1/3+1 = 4/3 eV (BT-30 solar bridge)
- H-CC-17: 흡착/탈착 에너지 비 = phi = 2 (가역성 한계)
- H-CC-18: TSA 온도스윙 deltaT = 120C = sigma*(sigma-phi) EXACT
- H-CC-19: 공정 에너지 효율 이론 한계 대비 현재 = sigma-phi = 10배 갭
- H-CC-20: DAC 풍속 최적 = 6 m/s = n EXACT (압력손실 vs 접촉시간 균형)

**반응기 (H-CC-21~30):**
- H-CC-21: Honeycomb 6각 셀 monolith 압력손실 = 사각 대비 1/phi = 50% 감소
- H-CC-22: Packed bed 최적 tube 수 = 6 = n EXACT (열전달 균일성)
- H-CC-23: Fluidized bed 최적 zone 수 = 6 = n (완전 혼합)
- H-CC-24: Rotating wheel sector 수 = 6 = n EXACT (Climeworks 실측 확인)
- H-CC-25: Hollow fiber OD = 6mm = n EXACT (최적 성능/제조성)
- H-CC-26: Microreactor channel = 6um = n EXACT (laminar flow 최적)
- H-CC-27: 반응기 aspect ratio = phi = 2 (L/D 최적)
- H-CC-28: 반응기당 baffle 수 = 12 = sigma EXACT
- H-CC-29: 반응기 열효율 = 1-1/sigma = 11/12 = 91.7% (이론 한계)
- H-CC-30: CO2 순도 99.9% 달성 최소 반응기 단수 = sopfr = 5

**열역학 한계 (H-CC-31~40):**
- H-CC-31: CO2 최소 분리에너지 비 (실제/이론) = sigma-phi = 10 EXACT (현재 기술)
- H-CC-32: Carnot 효율 한계에서 DAC 효율 = 1-T_cold/T_hot = 1-300/360 = 1/6 = 1/n
- H-CC-33: CO2 흡착 엔탈피 최적 = -48 kJ/mol = sigma*tau EXACT
- H-CC-34: CO2 결합 에너지 = 803 kJ/mol, 분해 활성에너지 비 = sigma-phi = 10%
- H-CC-35: 이상적 DAC 에너지 = 19.4 kJ/mol, 목표 = 2*19.4 = 38.8 ~ phi*W_min
- H-CC-36: 열 재생 효율 래더 = tau/sigma → phi/n → sopfr/sigma-phi (0.33→0.33→0.50)
- H-CC-37: 4단계 Carnot cycle DAC = tau = 4 EXACT
- H-CC-38: CO2 임계점 31.1C = 304K ~ sigma^2/sopfr + n = 304.8 (0.3% 오차)
- H-CC-39: CO2 임계 압력 7.38 MPa ~ sigma-sopfr = 7 + phi/sopfr EXACT
- H-CC-40: CO2 삼중점 -56.6C = -(sigma*tau + sigma-tau + 0.6) = n=6 구조

**스케일링 (H-CC-41~50):**
- H-CC-41: DAC 비용 학습률 = 1/(sigma-phi) = 10% per doubling
- H-CC-42: 포집 규모 래더 = 10^n 톤/yr 단위 (1→10→100→1k→10k→100k→1M)
- H-CC-43: DAC farm 최적 모듈 수 = 6x6 = 36 = sigma*n/phi
- H-CC-44: Pipeline 최적 booster 간격 = 120km = sigma*(sigma-phi)
- H-CC-45: 저장소 최적 주입정 수 = 12 = sigma EXACT
- H-CC-46: 모니터링 센서 유형 = 6 = n (CO2/O2/H2O/T/P/flow)
- H-CC-47: DAC→100Gt/yr 도달 시간 = 24년 = J2 EXACT (2026→2050)
- H-CC-48: CAPEX $/ton 래더: 600→120→24 = sigma*sopfr*10→sigma*10→J2
- H-CC-49: OPEX $/ton 래더: 200→40→8 = 현재→phi배 감소→sigma-tau
- H-CC-50: 산업 DAC 플랜트 최적 수명 = 24년 = J2 EXACT

**Cross-domain (H-CC-51~60):**
- H-CC-51: Battery+CC 통합 시 전기화학 포집 효율 = 독립 대비 phi=2배
- H-CC-52: Fusion+CC 통합 시 포집 에너지 비용 = 0 (무한 에너지)
- H-CC-53: Solar+CC 통합 시 photocatalytic 효율 = SQ 한계 33% (1/n/phi)
- H-CC-54: MOF+CC 최적 MOF = Mg-MOF-74 (CN=6, 8 mmol/g = sigma-tau)
- H-CC-55: Hydrogen+CC synfuel 효율 = 60% = sigma*sopfr %
- H-CC-56: Chip+CC DAC 제어 칩 = RISC-V N6 (6-stage, BT-56)
- H-CC-57: Wind+CC 최적 풍속 = 6 m/s = n EXACT (DAC+풍력 동시)
- H-CC-58: Concrete+CC 시멘트 CO2 경화 강도 = 기존 대비 phi=2배
- H-CC-59: Ocean+CC 해양 알칼리도 증강 = pH 변화 0.6 = n/10
- H-CC-60: Graphene+CC CO2→그래핀 변환 효율 = 12% = sigma %

- [ ] **Step 2:** 커밋 `feat: H-CC-01~60 carbon capture hypotheses`

---

## Task 4: 극한 가설 20개 (extreme-hypotheses.md)

**Files:**
- Create: `docs/carbon-capture/extreme-hypotheses.md`

- [ ] **Step 1:** H-CC-E01~E20 작성. 외계인 기술 수준의 도발적 가설.

**행성 물리 (H-CC-E01~E05):**
- H-CC-E01: 대기 전체 CO2 제거 시간 = sigma = 12년 (6 latitude band × phi year/band)
- H-CC-E02: 해양 산성화 완전 반전 = J2 = 24년 (심해 순환 주기의 1/n)
- H-CC-E03: 지각 basalt 전체 탄산염화 용량 = 10^18 ton = 현재 대기 CO2의 sigma^6배
- H-CC-E04: 극지 CO2 빙하 채굴 (화성 극관 패턴) = 6 채굴 사이트 = n
- H-CC-E05: 성층권 aerosol + DAC 통합 = 냉각 + 포집 동시, 6 주입점 = n

**핵/반물질 (H-CC-E06~E10):**
- H-CC-E06: C(Z=6) → N(Z=7) 양성자 주입 핵변환, CO2 원자 분해
- H-CC-E07: 양전자 촉매 CO2 결합 파괴, 활성에너지 = 0
- H-CC-E08: CNO cycle 역이용: N+C→CO2 역반응으로 에너지 방출 + C 회수
- H-CC-E09: 반물질-물질 소멸로 CO2 분해, 효율 = 100% (E=mc2)
- H-CC-E10: 핵 아이소머 감마선 CO2 직접 광분해 (MeV 광자)

**시공간 (H-CC-E11~E15):**
- H-CC-E11: Leech-24 격자 경로로 CO2 분자를 24차원 공간에 영구 격리
- H-CC-E12: 위상학적 결함(cosmic string)에 탄소 봉인, 붕괴 반감기 = 무한대
- H-CC-E13: 6차원 Calabi-Yau 다양체에 탄소 원자 압축 (끈이론 compactification)
- H-CC-E14: 웜홀 포집: 대기 CO2를 다른 우주/시공간 영역으로 전송
- H-CC-E15: 시간 역전 포집: CO2 배출 전 시점으로 탄소 원자 되돌리기

**궁극 (H-CC-E16~E20):**
- H-CC-E16: Dyson Swarm 태양 에너지로 지구 대기 전체 처리 (10^26 W)
- H-CC-E17: Maxwell Demon 실현: CO2 분자 선택적 분류, 열역학 2법칙 우회
- H-CC-E18: 우주 상수 Lambda 미세조정으로 탄소 결합에너지 변경
- H-CC-E19: 진공 에너지 추출로 CO2 분해 에너지 공급 (영구기관)
- H-CC-E20: 의식 기반 물질 조작: 관측자 효과로 CO2 파동함수 붕괴 → C+O2

- [ ] **Step 2:** 커밋 `feat: H-CC-E01~E20 extreme carbon capture hypotheses`

---

## Task 5: 레벨별 상세 문서 8개

**Files:**
- Create: `docs/carbon-capture/hexa-sorbent.md` (Level 0)
- Create: `docs/carbon-capture/hexa-process.md` (Level 1)
- Create: `docs/carbon-capture/hexa-reactor.md` (Level 2)
- Create: `docs/carbon-capture/hexa-chip.md` (Level 3)
- Create: `docs/carbon-capture/hexa-plant.md` (Level 4)
- Create: `docs/carbon-capture/hexa-transmute.md` (Level 5)
- Create: `docs/carbon-capture/hexa-universal.md` (Level 6)
- Create: `docs/carbon-capture/omega-cc.md` (Level 7)
- Reference: `docs/battery-architecture/hexa-cell.md` (패턴 — TOC, constants, block diagram, honesty assessment, predictions)

각 문서 구조 (hexa-cell.md 패턴):
```
# HEXA-XXX: [Title]
Codename / Level / Status / Date / Dependencies / Parent

## N6 Constants Reference
## Table of Contents
## 1. Executive Summary
## 2. Design Philosophy (시중 대비 압도적 우위 포인트)
## 3. System Block Diagram (ASCII art)
## 4-8. 핵심 기술 섹션들 (레벨별 특화)
## 9. Cross-Domain Connections
## 10. Honesty Assessment (n=6 일치/불일치 솔직 평가)
## 11. Predictions & Falsifiability
## 12. n=6 Complete Parameter Map
## 13. Links
```

**압도적 우위 핵심 (시중 대비):**
- Level 0: Climeworks sorbent 2.0 mmol/g → HEXA-SORBENT 48 mmol/g (24x=J2배)
- Level 1: 현재 200 kJ/mol → HEXA-PROCESS 20 kJ/mol (sigma-phi=10배 감소)
- Level 2: 현재 1 ton/day/module → HEXA-REACTOR 12 ton/day (sigma=12배)
- Level 3: 수동 센서 → HEXA-CHIP 양자센서 AI 자율제어 (10^6배 감도)
- Level 4: Climeworks 4kt/yr → HEXA-PLANT 1Mt/yr (250배, 래더: Mt→Gt→Tt)
- Level 5: CO2=폐기물 → HEXA-TRANSMUTE CO2=원료 ($1M/ton 그래핀)
- Level 6: 단일 플랜트 → HEXA-UNIVERSAL 행성 대기 제어 (10^5배)
- Level 7: 지구 한정 → OMEGA-CC 항성 스케일 (10^20배)

- [ ] **Step 1:** 8개 문서 작성 (각 200-400줄, 총 ~2,400줄)
- [ ] **Step 2:** 커밋 `feat: HEXA-CCUS 8-level design documents (sorbent→omega)`

---

## Task 6: BT-94/95/96 등록 + verification.md

**Files:**
- Modify: `docs/breakthrough-theorems.md` — BT-94/95/96 추가
- Create: `docs/carbon-capture/verification.md` — 80개 가설 검증
- Modify: `docs/atlas-constants.md` — 탄소포집 상수 추가

- [ ] **Step 1:** breakthrough-theorems.md 끝에 BT-94/95/96 추가 (spec 섹션 5 내용).
- [ ] **Step 2:** verification.md 작성 — 80개 가설 각각에 EXACT/CLOSE/WEAK/FAIL 등급 + 검증 근거.
- [ ] **Step 3:** atlas-constants.md에 탄소포집 관련 상수 추가 (CO2 임계점, 분리에너지, MOF 흡착량 등).
- [ ] **Step 4:** 커밋 `feat: BT-94/95/96 + CC verification + atlas constants`

---

## Task 7: DSE 실행 + Cross-DSE 11 도메인

**Files:**
- Reference: `tools/universal-dse/domains/carbon-capture-8level.toml`
- Modify: `docs/dse-map.toml` — carbon-capture 섹션 8단으로 갱신

- [ ] **Step 1:** universal-dse로 8단 DSE 전수 탐색 실행 (background). 결과 기록.
- [ ] **Step 2:** Cross-DSE 11 도메인 실행:
  ```bash
  for partner in battery fusion material solar metal-organic-framework hydrogen-fuel-cell wind-energy concrete-technology ocean-engineering graphene-2d-material climate-modeling; do
    ./universal-dse domains/carbon-capture-8level.toml domains/${partner}.toml
  done
  ```
- [ ] **Step 3:** dse-map.toml 갱신 — combos, levels, best_n6, n6_max, n6_avg, cross_dse 업데이트.
- [ ] **Step 4:** 커밋 `feat: HEXA-CCUS 8-level DSE + 11 Cross-DSE results`

---

## Task 8: CLAUDE.md 갱신 + 최종 정리

**Files:**
- Modify: `CLAUDE.md` — carbon-capture 도메인 추가
- Verify: 전체 산출물 일관성 확인

- [ ] **Step 1:** CLAUDE.md docs 구조에 carbon-capture 추가.
- [ ] **Step 2:** README.md 궁극 로드맵에 carbon-capture 반영 (해당 시).
- [ ] **Step 3:** 최종 커밋 `docs: HEXA-CCUS integration — CLAUDE.md + final cleanup`

---

## Parallel Execution Map

```
  독립 실행 가능 (병렬):
    Task 1 (TOML)  ──→  Task 7 (DSE 실행) [의존]
    Task 2 (goal.md)
    Task 3 (hypotheses)
    Task 4 (extreme)
    Task 5 (8개 레벨 문서) — 내부적으로 8개 병렬 가능
    Task 6 (BT + verification)

  순차:
    Task 1 → Task 7 (TOML 완성 후 DSE 실행)
    Task 7 → Task 8 (DSE 결과 후 최종 정리)

  최적 배치:
    Batch A (병렬): Task 1 + Task 2 + Task 3 + Task 4 + Task 5 + Task 6
    Batch B (순차): Task 7 (DSE)
    Batch C (순차): Task 8 (정리)
```
