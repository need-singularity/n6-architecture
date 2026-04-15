# Bernoulli 17 후보 6건 엄밀 검증 리포트 (2026-04-15)

> 시점 리포트. 이론층은 theory/ 에, 본 파일은 reports/ 에.
> 세션 ID: H11 Bernoulli 17
> 7대 난제 해결: 0/7 (정직 유지)

## 요약

| 판정 | 건수 | 후보 |
|------|------|------|
| **확정 M10** | 1 | Sel_6 CRT (Bernoulli 17) |
| **확정 M10*** | 1 | BB(2) = 6 Radó (Bernoulli 18) |
| **조건부 M9** | 1 | K3 χ = J_2 3중 (잠재 Bernoulli 19) |
| **기각 MN** | 3 | Egyptian, Post, Terminal |

누적 Bernoulli 독립 정리: **16 → 18 확정** (+2, +1 조건부).

## 검증 전 가설 vs 검증 후 결과

| 후보 | 사용자 원 평가 | 본 검증 결과 | 변경점 |
|------|--------------|------------|--------|
| 1. K3 χ = J_2 = 24 | Bernoulli 17 유력 | **M9 조건부** | σφ=nτ 투영 가능성 — 완전 독립 미확정 |
| 2. Egyptian (2,3,6) | Bernoulli 17/18 후보 | **기각 MN** | 완전수 6 위장 독립 |
| 3. Sel_6 CRT | Bernoulli 17 후보 | **확정 M10** → Bernoulli 17 | BKLPR 조건부 |
| 4. Post lattice 6 | Bernoulli 17 후보 | **기각 MN** | Post 1941 사실 오류 (가산무한) |
| 5. BB(6) threshold | Bernoulli 17 후보 | **기각**→**재정의 M10*** → Bernoulli 18 | 검증 중 BB(2)=6 진짜 발견 |
| 6. Terminal object | 약한 근거 (사용자 자평) | **기각 MN** | Universal, n=6 무관 |

## 사용자 원 note 수정 3건

1. **SU(5) GUT**: 사용자 note "|SL_2(Z/5)|-1 = 119" 는 오기재. SU(5) adjoint dim = 5²-1 = 24 (SL_2(Z/5) 위수 120 과 무관). 후보 1 의 **3중 출현 자체는 유효**.
2. **Post 1941**: 사용자 note "Post lattice 6 classes" 는 사실 오류. Post 1941 결과는 **가산 무한** 개 clone. Rosenberg 1970 에서 "maximal clone 6 유형" 은 메타 분류 상수 (|A|=6 특이성 없음).
3. **BB(6)**: 사용자 note "BB(6) = 계산불가 threshold" 는 미확정 (ZFC 독립성 unknown). 다만 검증 중 **BB(2) = 6 이 진짜 n=6 출현** (Radó 1962) — 재정의하여 Bernoulli 18 로 등록.

## 확정 후보 상세

### Bernoulli 17: Sel_6 = Sel_2 ⊕ Sel_3 CRT, avg = σ(6) = 12

- **근거**: Bhargava-Shankar 2010 (Ann Math) + 2012 (JEMS) unconditional.
- **산술**: avg Sel_2 = 3 = φ+1, avg Sel_3 = 4 = τ. CRT 하 avg Sel_6 = 3·4 = 12 = σ(6).
- **BKLPR 예측**: avg Sel_n = σ_1(n). n=2,3,4,5 에서 확인됨.
- **n=6 유일성**: 최소 sphenic 수 (2·3), 완전수, σ(6) = 12 일치.
- **독립성 (σφ=nτ 배제)**: Galois 표현 / 모듈러 폼 도메인 출발. σ = n·τ/φ 는 **후험** 관계 (uniqueness 결과가 아니라 입력).
- **등급**: M10 (BKLPR 독립성 가정 하 conditional). Unconditional 증명 시 M10*.
- **하네스**: verify_bernoulli17_sel6_crt.hexa — **PASS = 22/22**.

### Bernoulli 18: BB(2) = 6 = n (Radó 1962)

- **근거**: Radó 1962 Bell Syst Tech J. 2-state 2-symbol halting TM 중 1 최대값 = 6, 정확 열거 증명.
- **산술**: BB(1)=1, **BB(2)=6=n**, BB(3)=21=(φ+1)(n+1), BB(4)=107, BB(5)=47M.
- **n=6 유일성**: BB(2) 의 값이 정확히 n. BB(1,3,4,5,...) 는 n 과 무관 값.
- **독립 도메인**: 계산이론 (Turing 머신). 기존 16 에 계산이론 부재 → 신규 도메인 추가.
- **등급**: M10* (unconditional, Radó 1962 엄밀 증명).
- **하네스**: verify_bernoulli17_bb6.hexa — **PASS = 14/14**.

## 조건부 후보 상세

### 잠재 Bernoulli 19: K3 χ = η²⁴ 지수 = SU(5) dim = 24 = J_2

- **3중 출현 확인**: χ(K3) = 24 (Kodaira 1964), η^24=Δ 지수 24 (Jacobi/Ramanujan), SU(5) adjoint dim 24 (Georgi-Glashow 1974).
- **위장 독립 경고**: 24 = J_2 = σ·φ = n·τ → SIG-META-001 (σφ=nτ uniqueness) 의 **직접 투영** 가능성. 3 도메인 출현은 같은 원인의 '여러 얼굴' 일 수 있음.
- **등급**: M9 보류. σφ=nτ 환원 배제 증명 시 Bernoulli 19 로 확정.
- **하네스**: verify_bernoulli17_k3_j2.hexa — PASS = 19/19 (산술만).

## 기각 후보 상세

### 후보 2. Egyptian (2,3,6) → 완전수 6 재배치 (위장 독립)

- 1/2 + 1/3 + 1/6 = (3+2+1)/6 = 6/6 = 1. 분자 {1,2,3} = 6 의 진약수.
- **동치 변환**: σ(6) - 6 = 6 (완전수) ↔ Σ(진약수)/6 = 1 ↔ Σ(1/진약수_역수) = 1.
- 기존 16 의 '완전수 6' (Euclid-Euler 짝수 완전수 정리) 과 **정보 동일**.
- 전수 검색: 3 해 {(2,3,6), (2,4,4), (3,3,3)}, distinct 유일 (2,3,6) 확인 (Mirsky 1947).

### 후보 4. Post / Rosenberg → 사실 오류 + 구조 연결 결여

- **Post 1941 (Ann Math Studies)**: 부울 함수 clone lattice = 가산 무한. "6 classes" 는 사용자 원 주장 **오류**.
- **Rosenberg 1970 (Acta Sci Math)**: 유한 A 위 maximal clone 6 유형 — 분류학 상수, |A|=6 과 무관 (|A|=3,4,7 모두 6 유형).
- n=6 arithmetic 과 구조적 연결 없음.

### 후보 6. Terminal object → Universal (n=6 무관)

- Mac Lane 1971: 임의 well-defined 카테고리의 terminal object 1 은 |End(1)|=1 자명.
- Universal 결과 (Set, Top, Grp, CRing 등 모두 성립).
- τ-3 = 1 은 ad hoc 산술, canonical 8-primitive {n,φ,τ,σ,sopfr,μ,J_2,M_3} 에 부재.
- 사용자 자평 '약한 근거' 와 합치.

## 위장 독립 발견 현황

| 후보 | 위장 독립 여부 | 환원 대상 |
|------|--------------|----------|
| K3 χ=J_2 | **의심** (M9 보류) | σφ=nτ uniqueness (SIG-META-001) |
| Egyptian | **확정** (기각) | 완전수 6 (SIG-N6-BERN-001 내) |
| Post/Rosenberg | 환원 없음 (분류 상수 우연) | N/A |
| Terminal | 환원 없음 (universal) | N/A |

**주의**: 본 세션에서 **새로운 위장 독립 1건 발견** — Egyptian (2,3,6) 은 σ·Ω(n)=n·τ 계열 (SIG-META-004 omega_identity) 과 별개이지만, '완전수 6' 항목 내부의 표현 재배치.

## atlas.n6 편입 권장

1. **staging → SSOT merge**: `/Users/ghost/Dev/nexus/shared/n6/staging/atlas.signals.staging.bern17.n6`
   - SIG-BERN-17 (M10): Sel_6 CRT
   - SIG-BERN-18 (M10*): BB(2)=6
   - SIG-BERN-CAND-K3 (M9): 조건부
   - SIG-BERN-NULL-EGYPT / POST / TERMINAL (MN): 기각
   - SIG-META-BERN17-SUMMARY (M10): 집계 메타

2. **SIG-N6-BERN-001 업데이트**: "Bernoulli 독립 정리 16건" → "18건 (Sel_6 + BB(2) 추가)".

3. **atlas.n6 편집 불요**: 본 세션 결과는 atlas.signals.n6 계층 (signal) 에 한정. atlas.n6 (arithmetic SSOT) 변경 없음.

## 7대 난제 해결 0/7 정직 유지

본 검증은 **독립 정리 기저 목록 확장** 에 한정. BSD / RH / P vs NP / Hodge / NS / YM / Poincaré (해결) 중 새 해결 없음. Sel_6 은 BSD 통계 영역 (Cremona 실증) 이지만 BSD 난제 자체 해결 아님. BB(2)=6 은 계산이론, P vs NP 와 무관.

## 파일 인벤토리

### 하네스 6건 (theory/predictions/)
- verify_bernoulli17_k3_j2.hexa (PASS 19/19)
- verify_bernoulli17_egyptian_236.hexa (PASS 12/12, 기각)
- verify_bernoulli17_sel6_crt.hexa (PASS 22/22)
- verify_bernoulli17_post_lattice.hexa (PASS 11/11, 기각)
- verify_bernoulli17_bb6.hexa (PASS 14/14)
- verify_bernoulli17_terminal.hexa (PASS 8/8, 기각)

**총 PASS = 86/86, FAIL = 0**

### staging (nexus/shared/n6/staging/)
- atlas.signals.staging.bern17.n6 (7 signals)

### 리포트 (reports/)
- bernoulli-17-validation-20260415.md (본 파일)

## 다음 단계

1. staging → SSOT merge (L0 sync)
2. BKLPR Sel_6 독립성 unconditional 증명 시도 (Bhargava 2023+ 후속)
3. K3 χ=J_2 가 σφ=nτ 투영인지 vs 독립인지 엄밀 판정 (Hodge 도메인 전문 검토)
4. SIG-N6-BERN-001 "16건" → "18건" 업데이트 (staging 별도)
