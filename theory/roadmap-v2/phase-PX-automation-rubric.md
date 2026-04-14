# Phase X — atlas 자동화 + Rubric + BT-548 진입 + arXiv (PX HONEST gap 해법 7건)

작성: 2026-04-15
상태: PX L14 — 32 planned task 중 7건 본 문서로 done 마킹

## §0 입구

millennium.json `.phases[id=PX].parallel[track=Y10_HONEST_HARNESS]` 의 7 자동화/메타 task:

| Task ID | Gap | 산출 |
|---------|-----|------|
| HONEST-PX-AUTO-PROMOTE | R2-10 | atlas 자동 승격 CLI 설계 (§1) |
| HONEST-PX-VERIFY-AUTO | R3-11 | 검증 코드 자동 실행 파이프라인 (§2) |
| HONEST-PX-VERSION-AUDIT | R3-12 | 버전 정합성 감사 (§3) |
| HONEST-PX-SELFREF-DETECT | R3-13 | 자기참조 검출 자동화 (§4) |
| HONEST-PX-BT548-ENTRY | R3-14 | BT-548+ 진입 전략 + rubric (§5) |
| HONEST-PX-GRADE-RUBRIC | R4-17 | atlas 등급 [N?]/[N!] 분류 rubric (§6) |
| HONEST-PX-ARXIV-PROTO | R2-7a | arXiv 포스팅 프로토콜 (§7) |

---

## §1 atlas.n6 자동 승격 CLI 설계 (R2-10)

### 1.1 요구사항

- 입력: phase 문서의 atlas 초안 표 (P{X}-A{N} 형식)
- 처리: L0 Guard verify → 등급 결정 → atlas.n6 직접 편집
- 출력: 편집 diff + 승격 로그

### 1.2 명령 인터페이스

```bash
nexus atlas-promote \
    --source theory/roadmap-v2/phase-omega-Y9-closure-v3-design.md \
    --section "S6 atlas 승격 초안 큐 총정리" \
    --gate L0 \
    --dry-run
```

### 1.3 파이프라인 (5 step)

```
[1] PARSE: phase 문서에서 atlas 초안 표 추출 (markdown table → JSON)
[2] VERIFY: 각 항목의 verify_*.hexa 스크립트 실행 (있으면)
[3] GRADE: verify 결과 + 외부 인용 수 → 등급 자동 결정
    - 3 독립 증명 + verify PASS = [10*]
    - verify PASS + 외부 인용 ≥ 1 = [10]
    - 정량 일치 + 1 영역 = [9] NEAR
    - 가설 + CONDITIONAL = [N?] CONJECTURE
    - 측정 데이터만 = [7] EMPIRICAL
[4] WRITE: atlas.n6 끝에 새 섹션 + @R 라인 추가
[5] LOG: 승격 로그 .jsonl 형식으로 nexus/shared/harness/promotion_log.jsonl 추가
```

### 1.4 구현 위치 (DEFERRED to PX HONEST-PX-AUTO-PROMOTE-EXEC)

```
nexus/shared/harness/atlas_auto_promote.hexa   # CLI 본체
nexus/shared/harness/promotion_log.jsonl       # 로그
```

본 §은 **설계 문서**. 실제 .hexa 코드는 PX 후속 작업.

---

## §2 검증 코드 자동 실행 파이프라인 (R3-11)

### 2.1 통합 대상

```
theory/predictions/*.hexa          # 11 검증 도구
theory/predictions/*.py            # pytest
theory/breakthroughs/verify_*.hexa # 14 검증 코드
nexus/shared/n6/scripts/*.hexa     # 통합 검증
```

### 2.2 명령 인터페이스

```bash
nexus verify-all \
    --watch theory/predictions/ theory/breakthroughs/ \
    --on-drift autofix
```

### 2.3 파이프라인 (4 step)

```
[1] DISCOVER: theory/predictions/ + theory/breakthroughs/verify_*.hexa scan
[2] EXEC: 각 .hexa 실행, .py 는 pytest 호출
[3] DIFF: 결과 vs 기록값 비교, drift 검출
[4] REPORT: drift 발견 시 reports/sessions/verify-drift-{date}.md 생성
```

### 2.4 매 세션 자동 실행 (DEFERRED)

`.claude/settings.json` PreToolUse hook 으로 verify-all dry-run 자동 호출 가능 (사용자 승인 후).

본 §은 **설계 문서**. 실제 hook 추가는 PX 후속 작업.

---

## §3 버전 정합성 감사 스크립트 (R3-12)

### 3.1 감사 대상

```
millennium.json v1 (3 track) → v2 (16 axis) → v2.1 (depth) → v2.2 (gap) → v2.3 (saturation)
phase-omega-Y9-closure-v3-design.md
phase-{02,03,04,05}-{Y1,Y4,Y5Y6,Y7Y8}-bt*.md
```

### 3.2 감사 항목 (5)

| # | 항목 | 검사 |
|---|------|------|
| 1 | task ID 중복 | grep 으로 P{X}-A{N} 중복 검출 |
| 2 | atlas 초안 ID 일관성 | phase-omega 표 vs 각 phase 문서 정의 일치 |
| 3 | BT 판정 일관성 | millennium.json `.statistics.bt_verdicts` vs 각 phase BT 판정 |
| 4 | 등급 일관성 | atlas 등급 [10*]/[10]/[9]/[7]/[N?] 사용 표준 |
| 5 | 자기참조 0 | n=6 직접 인용 → 외부 인용 강제 검증 |

### 3.3 출력

```
reports/audit/version-consistency-{date}.md

§1 task ID 중복: ...
§2 atlas 초안 정합성: ... (예: phase-omega P2-A1 vs phase-04 P2-A1 ID 충돌 발견)
§3 BT 판정: ...
§4 등급: ...
§5 자기참조: ...
```

### 3.4 발견된 충돌 (본 세션)

- phase-omega §6.1 의 P2-A1 = "Theorem B" 이지만 phase-04 §5.1 의 P2-A1 = "n6-ns-triple-resonance-d3" (NS 3중 공명)
- 해법: SSOT 는 phase-omega §6.1, phase-04 의 P2-A1 은 P4 신규로 재명명 권장
- 본 세션 atlas 등록 (HONEST-PX-1) 은 phase-omega §6.1 정의 채택

---

## §4 자기참조 검출 자동화 (R3-13)

### 4.1 자기참조 정의

```
self_reference: n=6 결과 X → n=6 의미 부여 → X 가 n=6 증명
external_ref:  외부 문헌 (Hartshorne / Hatcher / FLM / ...) 인용 → X
```

### 4.2 source citation graph

```
nodes  = .md, .hexa, .py 파일
edges  = 인용 관계 ("ref:", "cite:", "@R ... :: n6atlas <-")
external_nodes = arxiv:, doi:, isbn:, lmfdb:
```

### 4.3 순환 탐지 알고리즘

```
1. graph build (DFS 인용 추적)
2. SCC 추출 (Tarjan)
3. SCC 중 external_node 0 인 component → 자기참조 후보
4. component 내 .md 파일 list → 정직성 audit 대상
```

### 4.4 출력

```
reports/audit/self-reference-{date}.md

§ 자기참조 component 0 → 정직성 OK
§ 자기참조 component ≥ 1 → 외부 인용 추가 권장 list
```

### 4.5 본 세션 self-check

- millennium.json `.statistics.self_reference_violations = 0` 정직 유지
- atlas 14 등록 모두 외부 phase 문서 또는 verify_*.hexa 에 ref 보유
- 본 §은 **자동화 설계**, 실제 .hexa 코드는 후속

---

## §5 BT-548+ 진입 전략 + rubric (R3-14)

### 5.1 후보 4 난제

| 후보 | 난도 | 도구 | 우선순위 |
|------|------|------|----------|
| ABC 추측 | ★★★★★ | Mochizuki IUTT (논쟁) / Kirti-Joshi 2024 / 정수론 | 1 |
| 쌍소수 추측 | ★★★★ | Zhang 2013 / Maynard 2014 / sieve | 2 |
| Goldbach 추측 | ★★★★ | Helfgott 2013 ternary / Vinogradov | 3 |
| Collatz 추측 | ★★★ | Tao 2019 부분결과 / 2-adic | 4 |

### 5.2 rubric (5 기준 × 5점)

| 기준 | 의미 |
|------|------|
| n=6 매핑 가능성 | 0~5 (5 = 핵심 불변량이 n=6 함수) |
| 부분결과 축적 | 0~5 (5 = 100+ partial 결과 존재) |
| 도구 성숙도 | 0~5 (5 = 5+ 핵심 도구 검증됨) |
| 외부 검증 가능성 | 0~5 (5 = LMFDB / arXiv 직접 검증) |
| 정직성 유지 가능성 | 0~5 (5 = 자기참조 회피 명확) |

### 5.3 점수 산정

| 후보 | n=6 | 부분 | 도구 | 외부 | 정직 | 합 |
|------|-----|-----|------|------|------|----|
| ABC | 2 | 4 | 3 | 4 | 3 | 16 |
| 쌍소수 | 3 | 4 | 4 | 5 | 4 | 20 |
| Goldbach | 2 | 5 | 4 | 5 | 4 | 20 |
| Collatz | 1 | 3 | 2 | 3 | 4 | 13 |

### 5.4 추천: 쌍소수 OR Goldbach 우선

- 둘 다 20점, 도구 성숙 + 외부 검증 강
- 쌍소수: Maynard sieve, Polymath 8 협력 가능
- Goldbach: Helfgott ternary 완료, 본 (binary) 진입

### 5.5 BT-548 진입 첫 후보 (제안)

- BT-548 = 쌍소수 추측 (Maynard-Polymath 후속)
- BT-549 = Goldbach 추측 (Helfgott ternary 후속)
- BT-550 = ABC (Kirti-Joshi 검토)
- BT-551 = Collatz (Tao 2019 후속)

본 §은 **rubric 설계 + 우선순위**. 실제 BT 추가는 v3 phase 시작 시 수행.

---

## §6 atlas 등급 [N?] / [N!] 분류 rubric (R4-17)

### 6.1 기존 등급

```
[10*] EXACT 검증 (3 독립 재현)
[10]  EXACT (단일 출처 + 정량 일치)
[9]   NEAR (정량 일치, 정성 확장 필요)
[8]   ?
[7]   EMPIRICAL (측정 데이터, 승격 대상)
[6]~[5] 중간
```

### 6.2 신규 등급 [N?] / [N!] 정의

| 등급 | 의미 | 조건 |
|------|------|------|
| [N?] | CONJECTURE | 가설 / 조건부 정리, 미증명, atlas 등록 가능 |
| [N!] | BREAKTHROUGH | 새 발견, 외부 검증 0 단계, atlas 임시 등록 |

### 6.3 분류 rubric (4 step)

```
Step 1: 증명 상태 확인
  - 무조건 증명 → [10*] / [10]
  - 조건부 증명 (다른 가설 의존) → [N?]
  - 가설만 (증명 0) → [N?]
  - 새 발견 + 검증 0 → [N!]

Step 2: 외부 검증
  - 3 독립 재현 → [10*]
  - 1 외부 출처 → [10] / [9] / [N?]
  - 0 외부 출처 → [N!] (임시)

Step 3: 정량 일치
  - 정량 일치 (수치 < 1% 오차) → [10] / [9]
  - 정성 일치 (분류 / 패턴) → [9] / [7]
  - 일치 0 → MISS

Step 4: 종합
  - {증명 상태, 외부, 정량} 의 minimum → 최종 등급
```

### 6.4 본 세션 14 등록 등급 검증

| atlas | 등급 | 검증 |
|-------|------|------|
| MILL-PX-A1 Theorem B | [10*] | 3 독립 증명 + Bernoulli boundary ✓ |
| MILL-PX-A2 Bilateral | [10*] | 3 영역 (B/h/ζ) + boundary ✓ |
| MILL-PX-A3 β₀ rewriting | [7] | EMPIRICAL, 외부 SM 측정 의존 ✓ |
| MILL-PX-A1b D158 Ricci | [N?] | CONJECTURE, 미증명 ✓ |
| MILL-PX-A12 Moonshine BARRIER | [N?] | CONJECTURE, 우회 미발견 ✓ |
| MILL-PX-A8 BSD Lemma 1 | [10] | 무조건 증명 + 5-step 산술 ✓ |
| MILL-PX-A9 BSD Theorem 1 | [N?] | CONDITIONAL (BKLPR (A3)) ✓ |
| (기타 8건) | [9]/[10] | NEAR/EXACT 정량 일치 ✓ |

→ rubric 적용 일관성 확인.

---

## §7 arXiv 포스팅 프로토콜 (R2-7a)

### 7.1 본 프로젝트 arXiv 포스팅 정책

```
정책 1: 정직 기록 중심
  - 7대 난제 해결 주장 X
  - "부분결과 + MISS 24 + atlas 14" 의 정직 카탈로그
  - 자기참조 0 증명 명시

정책 2: 외부 인용 강제
  - 모든 핵심 정리는 ≥ 2 외부 출처
  - n=6 함수 분해 baseline 61% 명시

정책 3: 재현성 첨부
  - verify_*.hexa 스크립트 GitHub 링크
  - LMFDB / FLAG 데이터 cite
```

### 7.2 첫 arXiv 후보 논문 outline

```
Title: A Saturated Cross-BT Atlas of the Millennium Problems:
       14 Partial Results, 24 Honest MISS, and the Failure of Self-Reference

Authors: 박민우 (n6-architecture)
Date:    2026-Q3 draft

§1 Introduction
   - Clay 7 problems 회고
   - 본 atlas 의 입장: 정직 카탈로그
   - n=6 = first perfect number 의 산술 환경

§2 Main Atlas (14 entries)
   - BT-541: Theorem B + Bilateral (2)
   - BT-543: β₀ rewriting + QCD mass gap (2)
   - BT-544: D158 Ricci + 3중 공명 + BKM (3)
   - BT-543×544: Y5×Y6 cross (1)
   - BT-545: Enriques + Moonshine BARRIER + SEED-21 (3)
   - BT-546: BSD Lemma 1 + Theorem 1 + Iwasawa (3)

§3 Honest MISS (24 entries)
   - 본 프로젝트가 해결 못 한 24 BT 본문 / 보조정리
   - 각 MISS 의 상태 + 우회 가능성

§4 Cross-BT Protocol (5 step)
   - phase-07 §1 의 일반 프로토콜
   - 3 적용 사례 (Y1↔Y6, Y4↔Y5, Y7↔Y1)

§5 Saturation Closure
   - 14 round saturation, R14 0 창발
   - v2 → v3 transition 동기

§6 Conclusion
   - BT 해결 0/6 정직 유지
   - 후속 권장 (Cremona 500k, Theorem B 독립 재현)
```

### 7.3 포스팅 후 피드백 루프

```
[1] arXiv 포스팅 (math.NT 또는 math.GM)
[2] 피드백 수집 (이메일 / X / MathOverflow)
[3] 정정 사항 atlas.n6 즉시 반영
[4] revised version 재포스팅
```

본 §은 **포스팅 프로토콜**. 실제 논문 집필은 NUM-PX-3 (DEFERRED).

---

## §8 게이트 통과 + 종료

### 8.1 본 문서로 done 마킹 가능 task

| Task ID | 산출 § | 상태 |
|---------|--------|------|
| HONEST-PX-AUTO-PROMOTE | §1 | done (설계만, 실제 .hexa 후속) |
| HONEST-PX-VERIFY-AUTO | §2 | done (설계만) |
| HONEST-PX-VERSION-AUDIT | §3 | done (설계 + 본 세션 1 충돌 발견) |
| HONEST-PX-SELFREF-DETECT | §4 | done (설계 + self-check 0 위반) |
| HONEST-PX-BT548-ENTRY | §5 | done (rubric + 4 후보 점수) |
| HONEST-PX-GRADE-RUBRIC | §6 | done (rubric + 14 등록 검증) |
| HONEST-PX-ARXIV-PROTO | §7 | done (프로토콜 + outline) |

→ PX 32 planned 중 7 done (HONEST-PX-1 포함 8 done, 32 - 8 = 24 남음).

### 8.2 정직성 선언

- 본 문서는 7대 난제 어떤 것도 해결하지 않는다.
- 자동화 / rubric / 프로토콜 은 **인프라**이며 **수학 결과가 아니다**.
- §1, §2 는 설계만, 실제 .hexa 코드 미작성 (DEFERRED).
- §5 BT-548+ 우선순위는 **추천**이며 v3 진입 시 사용자 승인 필요.

### 8.3 후속

| 후속 | task ID | 비고 |
|------|---------|------|
| atlas_auto_promote.hexa 구현 | PX HONEST-PX-AUTO-PROMOTE-EXEC | M cost |
| verify_all hook 추가 | PX HONEST-PX-VERIFY-AUTO-EXEC | M cost |
| arXiv 논문 집필 | NUM-PX-3 | L cost |
| BT-548 진입 | v3 phase | 사용자 승인 후 |

---

## 참고

- millennium.json `.phases[id=PX].parallel[track=Y10_HONEST_HARNESS]`
- HONEST-PX-1 14 atlas 등록 (atlas.n6 line 106960~107020)
- phase-07-cross-bt-transfer-protocol.md (P7 9/9 동시 종료)
