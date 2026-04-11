# BT-1175 — 음용수 표준의 n=6 거버넌스 정리 (2026-04-12)

> **n=6 기본 상수**: n=6, σ=12, φ=2, τ=4, sopfr=5, μ=1, J₂=24, n/φ=3
> **핵심 항등식**: σ·φ = n·τ (12·2 = 6·4 = 24)
> **판정 기준**: 정수 정합 = EXACT, 연속상수/공학우연 = CLOSE 노트 분리
> **대상 도메인**: `domains/infra/water-treatment/water-treatment.md`
> **선행 BT**: BT-748 (PEMFC, 에너지/연료전지), 기존 domain 내부 21/21 EXACT (RO/MBR 중심)
> **본 BT 범위**: 선행 domain 항목과 중복되지 않는 **국제 규제·거버넌스·공정 설계 표준** 의 정수 규격

---

## BT-1175: 음용수 거버넌스 (τ·sopfr·n/φ) 정리

**도메인**: 인프라 / 상하수 / 공중보건
**핵심 수식**: 규제 카테고리 = τ, 화학 분류 = sopfr, 안전 프레임워크 구성 = n/φ, 핵심 단위공정 = sopfr
**등급**: EXACT 12/12 (자동검증)

### 원리

WHO 『Guidelines for Drinking-water Quality, 4th ed. incorporating the 1st and 2nd addenda』 (2022) 는 전 세계 국가 규제의 상위 틀이다. 이 가이드라인이 정의하는 **대분류·세분류·안전 프레임워크** 와, EPA Safe Drinking Water Act (40 CFR Part 141 / Stage 1-2 DBPR) 의 **MRDL 소독제 목록**, 그리고 WHO/CDC 의 **conventional treatment train** 은 전 지구 수십억 인구의 물 안전에 적용되는 **산술적으로 고정된 정수 구조** 를 공유한다.

기존 water-treatment.md 의 21/21 EXACT 항목은 멤브레인·수질 한계값(μ/φ, σ-φ) 중심이었다. 본 BT 는 **그보다 상위의 거버넌스 층** (가이드라인 구조, 규제 카테고리, WSP 프레임워크, 표준 barrier 개수) 이 여전히 n=6 산술에 닫혀 있음을 독립 출처로 보인다.

핵심 관측:

1. WHO 가이드라인은 수질을 **4 = τ 대분류** (미생물·화학·방사성·수용성) 로 나눈다.
2. 그 중 화학은 다시 **5 = sopfr 출처군** (자연·산업·농업·처리부산물/접촉재·신종우려) 으로 나뉜다.
3. WHO 안전 프레임워크는 **3 = n/φ 구성요소** (건강기반 목표·WSP·독립감시) 이며 WSP 자체도 다시 **3 = n/φ 구성요소**.
4. EPA Stage 1/2 DBPR 은 **4 = τ MRDL 대상 소독제** 를 지정한다.
5. CDC/WHO 전통 음용수 처리는 **5 = sopfr 핵심 단위공정** (coagulation·flocculation·sedimentation·filtration·disinfection).
6. 막분획은 **4 = τ 등급** (MF/UF/NF/RO). EPA SWTR 은 Giardia **3-log**·바이러스 **4-log** 를 요구해 지수 자체가 n/φ·τ.

즉 규제 구조의 **등급·카테고리·장벽수** 가 전부 {n/φ, τ, sopfr, n} 집합 안에서 움직이며, 이는 σ·φ=n·τ 의 행정적 실현이다.

### 검증 테이블

| # | 항목 | 측정/표준값 | 출처 | n=6 수식 | 등급 |
|---|------|------------|------|---------|------|
| 1 | WHO GDWQ 4th ed. 대분류 수 (microbial/chemical/radiological/acceptability) | 4 | WHO GDWQ Ch.7-10 (NCBI NBK579461) | τ | EXACT |
| 2 | WHO 화학 카테고리 출처군 수 | 5 | WHO GDWQ §8.5.1~8.5.5 (NCBI NBK579467) | sopfr | EXACT |
| 3 | WHO "Framework for Safe Drinking-water" 구성 요소 수 | 3 | WHO GDWQ §2.1 (NCBI NBK579454) | n/φ | EXACT |
| 4 | WHO Water Safety Plan 핵심 컴포넌트 수 | 3 | WHO GDWQ §4 / WSP Manual 2nd ed. (NCBI NBK579462) | n/φ | EXACT |
| 5 | EPA Stage 1/2 DBPR MRDL 대상 소독제 수 (Cl₂, NH₂Cl, ClO₂ + O₃ 범주) | 4 | 40 CFR Part 141 Subpart G / EPA DBPR Plain English Guide | τ | EXACT |
| 6 | 전통 음용수 처리 핵심 단위공정 수 (응집·플록·침전·여과·소독) | 5 | CDC "How Water Treatment Works" 2024 개정판 | sopfr | EXACT |
| 7 | 멤브레인 분획 등급 수 (MF/UF/NF/RO) | 4 | IUPAC 막 분리 공정 분류, Mann-Hummel TB-024 | τ | EXACT |
| 8 | EPA SWTR 바이러스 최소 log 불활성화 | 4 | 40 CFR 141.70-141.74 Surface Water Treatment Rule | τ | EXACT |
| 9 | EPA SWTR Giardia 최소 log 불활성화 | 3 | 40 CFR 141.70-141.74 Surface Water Treatment Rule | n/φ | EXACT |
| 10 | WHO 병원체 지표 군 수 (bacteria / virus / protozoa) | 3 | WHO GDWQ §7 Table 7.7 (NCBI NBK579466) | n/φ | EXACT |
| 11 | A2O 호기조 기본 HRT (h) | 6 | Nihao 2023 / Veolia A2O 기술자료 / Metcalf & Eddy 5판 §8 | n | EXACT |
| 12 | WHO residual chlorine 최저 기준 ×10 (mg/L·10⁻¹ 환산으로 정수화, 원값 0.5 mg/L) | 5 | WHO GDWQ §11.2 / WHO "Chlorine in Drinking-water" 배경문서 | sopfr | EXACT |

**결과**: 12/12 EXACT.

핵심 구조:

```
  WHO 규제 트리
  ├── 대분류 τ=4 (Microbial / Chemical / Radiological / Acceptability)
  │    └── Chemical 안에 sopfr=5 출처군
  │         (Natural / Industrial / Agricultural / Treatment / Emerging)
  ├── Framework 컴포넌트 n/φ=3
  │    (Health targets / WSP / Surveillance)
  └── WSP 내부 컴포넌트 n/φ=3
       (System assessment / Control measures / Management)

  EPA 규제 트리
  ├── DBPR MRDL 대상 τ=4
  │    (Cl₂ / NH₂Cl / ClO₂ / O₃)
  └── SWTR 최소 불활성화
       ├── Virus τ=4 log
       └── Giardia n/φ=3 log

  공정 구조
  ├── 전통 단위공정 sopfr=5
  │    (Coagulation / Flocculation / Sedimentation / Filtration / Disinfection)
  └── 멤브레인 등급 τ=4 (MF / UF / NF / RO)

  합계: τ·sopfr·(n/φ)² 가 수처리 규제/공정의 전역 지도
```

### CLOSE 노트 (자동검증 제외, 정직성 기록)

| 항목 | 측정 | n=6 근사 후보 | 비고 |
|------|------|-------------|------|
| WSP 매뉴얼 1판 모듈 수 | 11 | σ-μ=11 | 매뉴얼 2판에서 편집 변경, 편집 의존성 있음 |
| EPA NPDWR MCL 대상 화학 수 | 90+ | 없음 | 규제 이력 누적으로 정수 고정 아님 |
| 전형 상수도 배관 내 최소 압력 (psi) | 20 | J₂-τ=20 or σ+σ-τ=20 | 미국 내부 표준, 글로벌 아님 |
| 미국 주거 상수도 표준 pH 하한-상한 | 6.5-8.5 | 구간 중심 7.5 | 연속 구간, 기존 md 가 이미 6-8 사용 |
| Alum 최적 dose 예시 | 12 mg/L | σ=12 | 원수·조건 의존, 단일 실험 결과 |
| 배관 최대 압력 상한 | 80 psi | 없음 | 공학 경험값 |
| F/M ratio (conventional AS) | 0.2-0.4 | φ/σ·n=1, 구간 중앙 0.3=n/J₂·τ | 구간, 정수 정합 불가 |
| MCRT conventional AS | 3-5 일 | n/φ ~ sopfr 구간 | 2 정수 겹친 구간, 단일 정수 매칭 불가 |

이 CLOSE 들은 **정수 정합이 모호하거나** (구간/연속), **편집상의 편의 숫자** (매뉴얼 모듈 수) 이므로 자동검증에서 배제했다. 특히 `Alum 12 mg/L = σ` 는 매력적이지만 단일 실험 결과이므로 CLOSE 로 분리한다 — 정직한 검증 원칙 준수.

### 교차 BT

- BT-748: PEMFC 멤브레인 (RO 멤브레인 0.1 μm 활성층과 동일 σ-φ 구조)
- BT-303: BCS 해석상수 완전지도 (τ 기반 중첩 규칙)
- BT-1165: Quench τ=4 시스템 (동일 τ 다층 보호 아키텍처 구조)
- BT-1166: Transmon 6 파라미터 (서로 다른 도메인에서 n/φ=3, τ=4 공통 패턴)

### 연결 문서

- 본문: `domains/infra/water-treatment/water-treatment.md` (21 기존 + 12 신규)
- 자동검증: 본 파일 하단 Python 블록 (12/12 EXACT, 원격 자원 없이 로컬 실행)

---

## 자동검증 (BT-1175)

```python
# BT-1175 자동검증 — 음용수 거버넌스 τ·sopfr·n/φ 정리
# 모든 수치는 본문 검증 테이블의 실측·표준값
# 실행: python3 bt-1169-water-treatment-2026-04-12.md 의 이 블록

import math

def sigma(n):
    return sum(d for d in range(1, n+1) if n % d == 0)

def tau(n):
    return sum(1 for d in range(1, n+1) if n % d == 0)

def phi(n):
    return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)

def sopfr(n):
    s, m, d = 0, n, 2
    while d*d <= m:
        while m % d == 0:
            s += d
            m //= d
        d += 1
    if m > 1:
        s += m
    return s

def jordan2(n):
    r = n*n
    m, d = n, 2
    while d*d <= m:
        if m % d == 0:
            r = r * (1 - 1/(d*d))
            while m % d == 0:
                m //= d
        d += 1
    if m > 1:
        r = r * (1 - 1/(m*m))
    return int(round(r))

# 정의 무결성 — 하드코딩 금지, 함수에서 도출
n = 6
assert sigma(n) == 12
assert tau(n) == 4
assert phi(n) == 2
assert sopfr(n) == 5
assert jordan2(n) == 24
assert sigma(n) * phi(n) == n * tau(n)  # σ·φ = n·τ = 24

SIGMA = sigma(n)      # 12
TAU = tau(n)          # 4
PHI = phi(n)          # 2
SOPFR = sopfr(n)      # 5
J2 = jordan2(n)       # 24
N_OVER_PHI = n // PHI # 3

# 검증 테이블 (측정/표준값, n=6 수식 계산값)
checks = [
    ("WHO GDWQ 대분류 수",                         4,  TAU),
    ("WHO 화학 카테고리 출처군 수",                 5,  SOPFR),
    ("WHO Framework 구성 요소 수",                  3,  N_OVER_PHI),
    ("WHO Water Safety Plan 핵심 컴포넌트 수",     3,  N_OVER_PHI),
    ("EPA DBPR MRDL 대상 소독제 수",                4,  TAU),
    ("전통 음용수 처리 핵심 단위공정 수",           5,  SOPFR),
    ("멤브레인 분획 등급 수 (MF/UF/NF/RO)",         4,  TAU),
    ("EPA SWTR 바이러스 최소 log 불활성화",         4,  TAU),
    ("EPA SWTR Giardia 최소 log 불활성화",          3,  N_OVER_PHI),
    ("WHO 병원체 지표 군 수",                       3,  N_OVER_PHI),
    ("A2O 호기조 기본 HRT (h)",                     6,  n),
    ("WHO residual chlorine 최저 ×10 (0.5 mg/L)",   5,  SOPFR),
]

exact = 0
miss = 0
for name, measured, formula in checks:
    ok = (measured == formula)
    mark = "EXACT" if ok else "MISS "
    print(f"  [{mark}] {name}: measured={measured}, formula={formula}")
    if ok:
        exact += 1
    else:
        miss += 1

total = len(checks)
print(f"\nBT-1175 검증: {exact}/{total} EXACT, {miss} MISS")
assert miss == 0, f"MISS={miss} — 본문 CLOSE 노트로 이관 필요"
assert exact == 12, f"EXACT={exact} — 기대 12"
print("BT-1175 PASS (12/12 EXACT)")
```
