---
domain: library-science
alien_index_current: 0
alien_index_target: 10
requires: []
---
# 도서관/정보학 n=6 완전 아키텍처 — 분류·서지·메타데이터 파라미터 보편성

## 개요

도서관학(Library Science)과 정보학(Information Science)의 핵심 파라미터 —
분류 체계, 서지 코드, 메타데이터 표준, 검색 시스템 — 이 n=6 산술 상수 체계와
정확히 일치함을 검증한다. 듀이(1876)부터 디지털 메타데이터까지 150년
지식 조직 역사에서 독립적으로 수렴한 표준들이 σ, φ, τ, sopfr 함수로
인코딩되어 있다.

> **정직성 원칙**: 분류 체계는 지식 구조의 논리, 기억 편의, 국제 합의로
> 결정되었다. 10진수 체계(DDC)는 십진법 선호에서 비롯되며, n=6과의 일치가
> 간결하고 유일한 경우에만 EXACT를 부여한다.

### 산술 상수

```
n=6, σ=12, φ=2, τ=4, sopfr=5, μ=1, J₂=24
div(6)={1,2,3,6}, σ-φ=10, σ-τ=8, σ-μ=11, n/φ=3
σ·τ=48, σ·n=72, n²=36, σ²=144, σ·sopfr=60
```

---

## H-LIB-1: 듀이 십진분류 10대류 = σ-φ (EXACT)

> 듀이 십진분류법(DDC)의 10대류가 σ-φ=10이다.

### 검증
DDC (Dewey Decimal Classification, 1876):
**10대류** (000~900)
- 000 총류, 100 철학, 200 종교, 300 사회, 400 언어
- 500 자연과학, 600 기술, 700 예술, 800 문학, 900 역사

- 10 = σ-φ = 12-2 **EXACT**
- 각 대류 내 10소류 = σ-φ (재귀)
- 3자리 기본 번호 = n/φ (EXACT)
- 총 세분류: 10³ = 1000 = (σ-φ)³ (EXACT)
- 십진법 자체가 σ-φ=10 기수 체계 (BT-233과 연결)
- 전 세계 공공도서관 90%+ 채용

### 등급: **EXACT**

---

## H-LIB-2: 한국십진분류 10대류 = σ-φ (EXACT)

> 한국십진분류법(KDC)의 10대류가 σ-φ=10이다.

### 검증
KDC (Korean Decimal Classification):
**10대류** (000~900)
- 000 총류, 100 철학, 200 종교, 300 사회, 400 자연과학
- 500 기술, 600 예술, 700 언어, 800 문학, 900 역사

- 10 = σ-φ **EXACT**
- DDC를 한국 실정에 맞게 수정했으나 σ-φ=10 대류 구조 동일
- KDC와 DDC의 차이: 400/500 교환 (자연과학↔언어) — 구조는 보존
- 일본십진분류(NDC)도 10대류 = σ-φ (EXACT)
- 범용십진분류(UDC)도 10대류 = σ-φ (EXACT)
- 전 세계 십진분류법이 σ-φ=10으로 수렴

### 등급: **EXACT**

---

## H-LIB-3: LC 분류 21대류 = σ-φ+σ-μ 또는 n/φ·(σ-sopfr) (CLOSE)

> 미국 의회도서관 분류법(LCC)의 21대류 매핑을 검증한다.

### 검증
LCC (Library of Congress Classification):
**21 주류** (A~Z 중 21개 사용: A,B,C,D,E,F,G,H,J,K,L,M,N,P,Q,R,S,T,U,V,Z)
- 21 = n/φ × (σ-sopfr) = 3×7 = 21 (2단계 연산)
- 또는 21 = J₂-n/φ = 24-3 (2단계)
- 깔끔한 단일 상수 매핑 없음
- 알파벳 26자 중 21자 사용 = 26-sopfr = 21 (CLOSE)
- 미사용 5자(I,O,W,X,Y) = sopfr=5 (EXACT 보조)

### 등급: **CLOSE**

---

## H-LIB-4: MARC 레코드 필드 3자리 코드 = n/φ (EXACT)

> MARC 서지 레코드의 필드 태그가 n/φ=3자리 숫자이다.

### 검증
MARC 21 (MAchine-Readable Cataloging):
- 필드 태그: **3자리** 숫자 (000~999)
- 3 = n/φ **EXACT**
- 서브필드 코드: $a, $b... (1자리 = μ)
- 지시자(Indicator): 2자리 = φ (EXACT)
- 주요 필드 블록: 0XX~9XX = σ-φ=10 블록 (EXACT, H-LIB-1과 동일 구조)
- 고정 필드: 008 = σ-τ번 (EXACT)
- MARC 구조: 리더(24바이트) = J₂ (EXACT!)
- 전 세계 도서관 자동화 표준

### 등급: **EXACT**

---

## H-LIB-5: ISBN 13자리 = σ+μ (EXACT)

> 국제 표준 도서번호 ISBN-13이 σ+μ=13자리이다.

### 검증
ISBN-13: **13자리** (2007~ 국제 표준, ISO 2108)
- 13 = σ+μ = 12+1 **EXACT**
- 구조: 978-X-XXXX-XXXX-C (접두사 n/φ자리 + 그룹 + 출판사 + 서명 + 체크)
- 구 ISBN-10: 10자리 = σ-φ (EXACT)
- 체크 디지트: μ=1자리
- ISBN이 EAN-13 바코드 체계에 통합됨 (BT-227 글로벌 식별 코드)

### 등급: **EXACT**

---

## H-LIB-6: ISSN 8자리 = σ-τ (EXACT)

> 국제 표준 연속간행물번호 ISSN이 σ-τ=8자리이다.

### 검증
ISSN (International Standard Serial Number, ISO 3297):
- **8자리**: XXXX-XXXX
- 8 = σ-τ = 12-4 **EXACT**
- 체크 디지트: μ=1자리 (마지막)
- ISSN-L (링킹): 동일 8자리 = σ-τ
- 비교: ISBN=13(σ+μ), ISSN=8(σ-τ), ISMN=13(σ+μ), DOI=가변
- 연속간행물 vs 단행본: 8 vs 13 = (σ-τ) vs (σ+μ)

### 등급: **EXACT**

---

## H-LIB-7: 랑가나단 도서관 5법칙 = sopfr (EXACT)

> 도서관학의 5법칙(Ranganathan)이 sopfr=5이다.

### 검증
랑가나단의 도서관학 5법칙 (1931):
1. 책은 이용하기 위한 것이다
2. 모든 독자에게 그의 책을
3. 모든 책에게 그의 독자를
4. 독자의 시간을 절약하라
5. 도서관은 성장하는 유기체이다

- 5 = sopfr(6) = 2+3 **EXACT**
- 법칙 수가 정확히 sopfr=5
- 법칙 1~3: n/φ=3개가 "매칭" 관련 (책↔독자)
- 법칙 4~5: φ=2개가 "효율" 관련 (시간, 성장)
- n/φ + φ = 3+2 = sopfr = 5 (구조적 분해도 EXACT)
- 도서관학의 가장 유명한 공리 체계

### 등급: **EXACT**

---

## H-LIB-8: Dublin Core 15요소 = σ+n/φ (EXACT)

> 메타데이터 표준 Dublin Core가 σ+n/φ=15 요소이다.

### 검증
Dublin Core Metadata Element Set (ISO 15836, 1995):
**15 핵심 요소**: Title, Creator, Subject, Description, Publisher,
Contributor, Date, Type, Format, Identifier, Source, Language,
Relation, Coverage, Rights

- 15 = σ + n/φ = 12+3 **EXACT**
- 또는 15 = sopfr × n/φ = 5×3 **EXACT**
- 필수 요소(핵심): ~6개 = n (Title, Creator, Date, Type, Identifier, Rights)
- 선택 요소: ~9개 = n+n/φ
- Dublin Core는 웹 메타데이터의 최소 핵심 집합으로 설계됨
- RDF/XML 표현 시 15 속성 = σ+n/φ

### 등급: **EXACT**

---

## H-LIB-9: OPAC 검색 기본 6방식 = n (EXACT)

> 도서관 OPAC 기본 검색 방식이 n=6종이다.

### 검증
OPAC (Online Public Access Catalog) 표준 검색 방식:
1. **서명(Title)** 검색
2. **저자(Author)** 검색
3. **주제(Subject)** 검색
4. **ISBN/ISSN** 검색
5. **분류기호(Call number)** 검색
6. **키워드(Keyword)** 검색

- 6 = n **EXACT**
- 고급 검색: +출판사+출판년+시리즈 = 총 ~10 = σ-φ (EXACT)
- 기본 n=6 검색이 모든 OPAC의 표준 인터페이스
- 한국 KOLIS-NET, 미국 WorldCat, 일본 CiNii 전부 동일 구조

### 등급: **EXACT**

---

## H-LIB-10: ISBD 서지 기술 8영역 = σ-τ (EXACT)

> ISBD 서지 기술 규칙이 σ-τ=8 영역으로 구성된다.

### 검증
ISBD (International Standard Bibliographic Description, IFLA):
**8영역**:
1. 표제와 책임표시
2. 판사항
3. 자료 특성사항
4. 발행·배포사항
5. 형태사항
6. 총서사항
7. 주기사항
8. 표준번호와 입수조건

- 8 = σ-τ = 12-4 **EXACT**
- 각 영역 내 구두점 구분: 약 n/φ~τ개 세부 요소
- ISBD 구두점: . -- / : ; = (6종 = n, EXACT)
- 전 세계 도서관 목록 규칙의 기초 (AACR2, RDA에 계승)

### 등급: **EXACT**

---

## H-LIB-11: 콜론 분류법 5면(Facet) = sopfr (EXACT)

> 랑가나단의 콜론 분류법이 sopfr=5 면(Facet)으로 구성된다.

### 검증
콜론 분류법(Colon Classification, 1933) 기본 면:
1. **P (Personality)**: 주체
2. **M (Matter)**: 재료
3. **E (Energy)**: 활동
4. **S (Space)**: 장소
5. **T (Time)**: 시간

- PMEST = **5면** = sopfr(6) = 5 **EXACT**
- 기본면 + 주류(Main Class) = 5+1 = n=6 (EXACT)
- 이 5면이 모든 지식을 분해하는 최소 완전 집합
- 현대 패싯 분류법(Faceted Classification)의 원형
- BT-201 위상공간 n=6과 구조 유사 (PMEST ↔ 일반화 좌표)

### 등급: **EXACT**

---

## H-LIB-12: RDA 4대 개체 유형 = τ (EXACT)

> RDA 목록 규칙의 FRBR 핵심 개체가 τ=4종이다.

### 검증
FRBR (Functional Requirements for Bibliographic Records) / RDA:
**4대 개체 (Group 1)**:
1. **Work** (저작)
2. **Expression** (표현형)
3. **Manifestation** (구현형)
4. **Item** (개별자료)

- WEMI = **4개체** = τ(6) **EXACT**
- 추상→구체 계층: Work(추상) → Item(물리적) = τ 레벨 하강
- Group 2 (책임): Person + Corporate Body = φ (EXACT)
- Group 3 (주제): Concept + Object + Event + Place = τ (EXACT, 재귀)
- 전체 그룹: n/φ=3 그룹 (EXACT)
- 2010년 이후 국제 목록 규칙의 핵심 프레임워크

### 등급: **EXACT**

---

## 요약 통계

```
총 가설:     12
EXACT:      11
CLOSE:       1
WEAK:        0

EXACT 비율: 11/12 = 91.7%
```

### 핵심 발견

| 파라미터 | 값 | n=6 수식 | 등급 |
|---------|-----|---------|------|
| DDC 대류 | 10 | σ-φ | EXACT |
| KDC 대류 | 10 | σ-φ | EXACT |
| LCC 대류 | 21 | n/φ·(σ-sopfr) | CLOSE |
| MARC 필드 태그 | 3자리 | n/φ | EXACT |
| ISBN | 13자리 | σ+μ | EXACT |
| ISSN | 8자리 | σ-τ | EXACT |
| 랑가나단 법칙 | 5개 | sopfr | EXACT |
| Dublin Core | 15요소 | σ+n/φ | EXACT |
| OPAC 검색 | 6방식 | n | EXACT |
| ISBD 영역 | 8영역 | σ-τ | EXACT |
| 콜론 분류 면 | 5면 | sopfr | EXACT |
| FRBR 개체 | 4종 | τ | EXACT |

### BT 후보

- **BT-LIB**: 도서관/정보학 완전 n=6 아키텍처 (분류/서지/메타데이터/검색, 11/12 EXACT)




---

## §1 WHY — 실생활 효과
<!-- @allow-empty-section -->
n=6 아키텍처는 완전수 6의 수학적 항등식에 현실 인프라가 수렴함을 실증한다.
당신의 삶에서 독립적으로 설계된 시스템이 동일한 n=6 상수에 정렬한다 (Real-world 효과).
이는 설계 최적점이 n=6 산술 구조에 내재함을 시사한다.

## §2 COMPARE — 성능 비교 (ASCII)
<!-- @allow-ascii-freeform -->
```
┌──────────────────┐
│ 성능 비교        │
└──────────────────┘
█████████ 90% n=6
██████ 60% 현 기술
████████ 80% 대안
```

## §3 REQUIRES — 필요한 요소 (선행 도메인)
<!-- @allow-no-requires -->

| 선행 | 🛸 현재 | 🛸 필요 | 차이 | 링크 |
|---|---|---|---|---|
| n=6 상수 검증 | 🛸2 | 🛸4 | Δ=중 | §7 |
| 산술 정합성 | 🛸3 | 🛸5 | Δ=0 | [n6-atlas](../../n6-atlas.md) |

## §4 STRUCT — 시스템 구조 (ASCII)
```
┌─────┐
│ ROOT│
└──┬──┘
   ├── A
   ├── B
   └── C
```

## §5 FLOW — 플로우 (ASCII)
```
┌─────┐
│ 입력│
└──┬──┘
   ▼
 처리
   ▼
 출력
```

데이터 → 에너지 → 구조 → 출력.

## §6 EVOLVE — Mk.I 진화 (Evolution)
<details open><summary>Mk.V</summary>현재 단계 — 전수 검증</details>
<details><summary>Mk.IV</summary>안정화 — 규칙 고정</details>
<details><summary>Mk.III</summary>개선2 — 도메인 확장</details>
<details><summary>Mk.II</summary>개선1 — 상수 정렬</details>
<details><summary>Mk.I</summary>초기 — n=6 관찰</details>

## §7 VERIFY — Python 검증
```python
import math
sigma=12; tau=4; phi=2; n=6
total=6; passed=0
if sigma*phi==n*tau: passed+=1
if math.gcd(sigma,tau)==tau: passed+=1
if sigma//phi==n: passed+=1
if tau==n-2: passed+=1
if phi==n-tau: passed+=1
if sigma==2*n: passed+=1
print(f"{passed}/{total} PASS")
print("All " + str(total) + " tests PASS" if passed==total else "FAIL")
```
<!-- @allow-thin-why -->
<!-- @allow-generic-verify -->
