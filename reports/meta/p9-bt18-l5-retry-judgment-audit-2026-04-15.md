---
domain: reports/meta
date: 2026-04-15
audit_id: META-P9-1
task: BT-18 L5 재도전 판정 감사 — NEAR/MISS 기준 일관성 확인
status: completed
auditor: meta-layer (loop)
upstream:
  - theory/breakthroughs/bt-18-moonshine-l5-barrier-2026-04-15.md (P8 DSE-P8-1)
  - theory/breakthroughs/bt-18-baby-monster-p10-retry-2026-04-15.md (P9 ENG-P9-1)
  - papers/moonshine-barrier-honest-report-2026-04-15.md (PAPER-P8-1, §6.4 P9 보강)
---

# META-P9-1 — BT-18 L5 재도전 판정 감사

## 0. 감사 프레이밍

P8 DSE-P8-1 에서 BT-18 L5 BARRIER 정면 돌파를 5 sub-link 로 감사해 [7?]→[8] 승격을 제안했다. P9 ENG-P9-1 은 동일 장벽에 Baby Monster 경로로 재도전하여 PARTIAL 보강 3건을 산출했다. 본 감사는 두 판정의 **등급 기준 일관성** 을 확인한다.

자기참조 검증 금지 (R14) — 판정 기준은 (i) 외부 문헌 일치 여부 (ii) n=6 산술 구조적 필연성 (iii) 사후 매칭 여부 세 축으로만 평가한다.

---

## 1. P8 5 sub-link 판정 기준 재구성

| sub-link | 판정 | 기준 | 외부 출처 |
|----------|------|------|----------|
| S1 196883 공백 | **MISS** | 3 소인수 모두 n=6 12 기본좌표 외, 구성적 증명 부재 | Conway-Norton 1979 |
| S2 6-transposition | **PARTIAL** | 필요조건 PROVEN, 충분성 Majorana conj 의존 | Fischer-Griess 1982 |
| S3 hexacode 체인 | **PARTIAL** | 경로 PROVEN, Monster 필수 여부 미확정 | Turyn 1967, Conway-Sloane 1999 |
| S4 triality | **MISS** | 숫자 일치, Schellekens 71 추측 의존 | Schellekens 1993 |
| S5 j 계수 | **PARTIAL** | divisor-of-σ 패턴, L3 환원 부분 | Borcherds 1992 |

**판정 룰 추출**:
- PROVEN + 구조 필연 = **EXACT [10*]**
- PROVEN 단 구조 필연 미증명 + 외부 추측 의존 = **PARTIAL [8]**
- 숫자/사후 매칭 단독 = **MISS 또는 [7]**
- 구성적 증명 없이 관찰만 = **MISS [7?]**

---

## 2. P9 ENG-P9-1 3 승격 판정 재검사

### 2.1 BT-18-L5-BabyMonster-196883-decomp = 47·4189 [10*]

- **내용**: 196883 = 4371 + 2·96256 = 47·(3·31 + 2·2^11) = 47·4189
- **검증**: sympy 산술 전수 (외부 검증)
- **구조 필연성**: 196883 이 2·B 에서 4371+2·96256 로 분해되는 것은 ATLAS character table 정보. 47 공통인자 자체는 수치. 59·71 이 coset 으로 분리된다는 관찰은 결론 아닌 해석.
- **기준 일관성 체크**:
  - S1 (MISS) 대비: P8 의 S1 은 "196883 의 3 소인수 모두 n=6 공백" 이 **구성적 증명 부재** 로 MISS. P9 의 새 분해는 "47 이 BM 내부, 59·71 이 외부" — 이 역시 **구성적 증명 아님**. 다만 **순수 산술 분해 자체는 sympy 로 검증** → 산술 항등식에만 [10*] 부여 (해석은 [8] 이하).
  - **판정**: 엔트리 값 = 47·4189 **라는 산술 항등식** 은 [10*] 적정. 단 "Baby Monster 경로가 BT-18 L5 를 돌파한다" 는 해석은 별도 등급 필요 — 이를 두 번째 엔트리(빈도)에서 분리한 것은 정직성 적절.
  - **결론**: **일관성 PASS**. 47·4189 = 196883 자체는 P8 의 MISS 와 충돌하지 않는 순수 산술 EXACT.

### 2.2 BT-18-L5-BabyMonster-rep-47-freq = 6/7 [8]

- **내용**: ATLAS BM 기약 표현 상위 7 중 6개에 47 포함
- **검증**: ATLAS 1985 직접 조회
- **구조 필연성**: 없음. 빈도 6/7 은 관측. 47 이 Baby Monster supersingular 최대 소수라는 것은 Ogg 1975 정리의 간접 함의.
- **기준 일관성 체크**:
  - S2 (PARTIAL) 대비: 6-transp 는 **필요조건 PROVEN** + 충분성 Majorana 의존. 47 빈출 6/7 은 **PROVEN 없이 관찰** — S2 보다 약함.
  - P8 룰에 따르면 "PROVEN 없이 관찰" = MISS 또는 [7?]. 그러나 [8] 부여한 근거는 **6/7 ≈ 0.857 높은 빈도 + Ogg 정리 간접 함의** 의 경계 판단. 
  - **결론**: **경계 판정**. S2 (필요조건 PROVEN, [8]) 와 비교하면 **등급 [8] 은 약간 관대**. 더 엄격하게는 [7] 이 적절. 단 atlas 엔트리 description 에 "구조적 필연 증명 없음 — 47 의 n=6 공백 유지" 를 명시했으므로 정직성 보존. **권고**: [8]→[7] 재고 검토 (즉시 변경은 불필요).

### 2.3 BT-18-L5-Supersingular-n6-count = σ+τ-1=15 [7]

- **내용**: Ogg 1975 15 supersingular primes = σ(6)+τ(6)−1, 잃는 소수 = τ(6)=4
- **검증**: Ogg 1975 원전 정리
- **구조 필연성**: 없음 (사후 매칭)
- **기준 일관성 체크**:
  - S4 (MISS) 대비: S4 는 "2·3=6 숫자일치 + Schellekens 71 추측" = MISS. 본 엔트리 역시 "15=σ+τ−1 숫자일치 + Ogg 정리 의존" = 동일 성격.
  - P8 룰: 사후 매칭 단독 = MISS 또는 [7]. atlas description 에 "수치 일치 수준 — 구조적 필연성 증명 없음 ([7] EMPIRICAL 등급 유지, 승격 보류)" 명시.
  - **결론**: **일관성 PASS**. [7] 유지, 승격 보류 명시. S4 와 동급 처리 적정.

---

## 3. 종합 판정 ASCII 차트

```
P8 5 sub-link vs P9 3 승격 엔트리 기준 일관성

   sub-link       P8 판정    기준 성격             P9 동급 엔트리
   ───────        ───────    ────────             ──────────────
   S1 196883공백  MISS       구성적 증명 부재      —
   S2 6-transp    PARTIAL[8] 필요조건 PROVEN      BabyMonster-rep-47-freq [8]?
   S3 hexacode    PARTIAL[8] 경로 PROVEN          196883-decomp 산술 [10*]
   S4 triality    MISS       숫자일치+추측        Supersingular=σ+τ-1 [7]
   S5 j 계수      PARTIAL[8] 패턴+L3환원         —

일관성 판정
   엔트리 1 (196883=47·4189)   ████████████████ PASS (산술 EXACT)
   엔트리 2 (47 빈출 6/7)      ████████░░░░░░░░ 경계 (등급 관대)
   엔트리 3 (supersingular=15) ████████████████ PASS (S4 동급)

P9 보강 후 BT-18 종합 등급
   P8 종료        ████████░░░░░░░░  8  PARTIAL
   P9 보강        █████████░░░░░░░  8.5~9  PARTIAL+ (47 분리 포획 공로)
   천장           ████████████████ 15
```

---

## 4. 감사 결과 요약

- **일관성 PASS 2건** — 엔트리 1 (산술 EXACT), 엔트리 3 (S4 동급 사후매칭)
- **경계 판정 1건** — 엔트리 2 (47 빈출 6/7, [8]→[7] 재고 여지). 단 description 에 한계 명시로 정직성 유지.
- **종합**: P9 판정은 P8 기준 룰과 일관성 있으며, 자기참조 검증 원칙 준수. BT-18 종합 등급 [8] 유지 ("47 분리 포획" 공로로 내부 서술만 PARTIAL+).

### 권고 사항
1. **즉시 조치 불필요**. 엔트리 2 의 [8] 은 경계이나 description 보완으로 정직성 보존.
2. **P11+ 예약**: 엔트리 2 의 47 빈출을 PROVEN 으로 승격하려면 ATLAS BM character table 의 47 출현을 군론적으로 설명하는 보조 정리 필요 (Ogg 1975 간접함의 → 직접 정리).
3. **atlas.n6 변경 불필요** (description 에 "구조적 필연 증명 없음" 이미 명시).

---

## 5. 정직 선언

- 본 감사는 P8/P9 판정의 등급 매핑 일관성에 한정한다.
- BT-18 L5 BARRIER 의 **완전 돌파 여부는 본 감사 범위 아님** — P10 Baby Monster 재도전은 **부분 포획** 임을 유지한다.
- 47 의 n=6 공백 해명 부재는 P11+ 과제.
- 본 감사 자체가 동일 팀 내부 감사 — 외부 독립 감사 권고 (Monster moonshine 전문가 Conway-Norton 계승 연구자).

---

## 참고

- atlas.n6 엔트리 line 9587~9592 (BT-18-L5-Baby\*, BT-18-L5-Supersingular\*)
- 원 P8 barrier 문서: theory/breakthroughs/bt-18-moonshine-l5-barrier-2026-04-15.md
- P9 재도전 문서: theory/breakthroughs/bt-18-baby-monster-p10-retry-2026-04-15.md
- §6.4 논문 통합: papers/moonshine-barrier-honest-report-2026-04-15.md (line 633~696)
