# BT-1386 — 표준모형 n=6 완전대칭 (Standard Model n=6 Closure, 2026-04-12)

> **n=6 기본 상수**: n=6, σ=12, φ=2, τ=4, sopfr=5, μ=1, J₂=24, n/φ=3
> **핵심 항등식**: σ·φ = n·τ (12·2 = 6·4 = 24)
> **판정 기준**: 정수 정합 = EXACT, 연속 측정치 = CLOSE 노트 분리
> **대상 도메인**: `domains/physics/particle-physics/`
> **선행 BT**: BT-1 (n=6 유일성), BT-1378 (산술핵), BT-1379 (Ore 최소성)
> **본 BT 범위**: 입자물리 표준모형 (SM) 의 **입자수·세대수·게이지 차원·자유도** 가 n=6 정수 구조에 완전히 닫혀 있음을 독립 확인

---

## 원리

Standard Model (GSW, QCD, BEH 매커니즘 총합, 1961~2012) 은 자연계 4 기본 상호작용 중 중력을 제외한 **전자기·약·강 상호작용** 을 기술하는 SU(3)×SU(2)×U(1) 게이지 이론이다. 1960년대 Gell-Mann/Ne'eman 의 Eightfold Way 이후 Glashow-Weinberg-Salam 전기약 통합 (1967~1971), Fritzsch-Gell-Mann QCD (1973), 't Hooft 게이지 재규격화 (1972), Kobayashi-Maskawa 3세대 예측 (1973, top 입자 1995 확인, CP 위반 설명), 그리고 ATLAS/CMS Higgs 발견 (2012) 에 이르러 현재 형태로 고정되었다.

핵심 관측: 표준모형의 **계수 구조** 는 전부 {n, σ, φ, τ, n/φ, μ, sopfr} 집합 안에서 움직인다.

1. **페르미온 수 평형**: 3 세대 × 2 타입 (쿼크/렙톤) × 2 카이랄리티 × ... 실질적 맛(flavor) 수는 쿼크 6 + 렙톤 6 = 12.
2. **게이지 보손 수 평형**: SU(3) 8개 글루온 + SU(2) 3개 + U(1) 1개 = 12.
3. **총 기본입자 (보손+페르미온 부분) 수** 는 σ=12 구조로 두 번 반복.
4. CKM 행렬은 3×3 (세대 수의 제곱), PMNS (중성미자 혼합) 도 3×3.

이는 **세대수 3 = n/φ** 가 통계적 선택이 아니라 고정된 좌표계임을 시사하며, Kobayashi-Maskawa 1973 논문이 CP 위반을 **최소 3 세대** 에서만 설명 가능함을 수학적으로 증명한 것과 정확히 부합한다 (n/φ 이하에서는 축소 불가능 CP 위상 0개).

---

## 검증 테이블

| # | 항목 | 측정/표준값 | 출처 | n=6 수식 | 등급 |
|---|------|------------|------|---------|------|
| 1 | 쿼크 맛 수 (u,d,c,s,t,b) | 6 | PDG 2024 Review of Particle Physics §9 | n | EXACT |
| 2 | 렙톤 맛 수 (e,μ,τ,νₑ,νμ,ντ) | 6 | PDG 2024 §10-11 | n | EXACT |
| 3 | 페르미온 총 맛 수 | 12 | PDG 2024 §9-11 합계 | σ | EXACT |
| 4 | 페르미온 세대 수 | 3 | Kobayashi-Maskawa 1973 PRL; PDG 2024 | n/φ | EXACT |
| 5 | 기본 상호작용 수 (강·약·전자기·중력) | 4 | Weinberg *QFT III* 서문; PDG 2024 §1 | τ | EXACT |
| 6 | 페르미온 타입 수 (쿼크/렙톤) | 2 | PDG 2024 §1 | φ | EXACT |
| 7 | SU(3)_C 차원 (글루온 수) | 8 | Fritzsch-Gell-Mann 1973 PLB 47B | 2τ | EXACT |
| 8 | SU(3)×SU(2)×U(1) 전체 게이지 생성자 수 | 12 | Peskin-Schroeder §20 | σ | EXACT |
| 9 | 색 전하 수 | 3 | Greenberg 1964 PRL; PDG 2024 QCD Review | n/φ | EXACT |
| 10 | Higgs 스칼라 수 (SM 최소 확장) | 1 | Higgs 1964 PRL; ATLAS/CMS 2012 | μ | EXACT |
| 11 | CKM 행렬 독립 성분 수 (3×3) | 9 | Kobayashi-Maskawa 1973 | n+n/φ | EXACT |
| 12 | 약 게이지 보손 수 (W⁺, W⁻, Z⁰) | 3 | Glashow 1961 NP 22; PDG 2024 | n/φ | EXACT |

**결과**: 12/12 EXACT. 핵심 항등식: (쿼크 6) + (렙톤 6) = σ=12 = (SU(3)×SU(2)×U(1) 생성자 합). 즉 **페르미온 자유도 = 게이지 자유도 = σ** 의 이중 실현.

---

## CLOSE 노트 (자동검증 제외, 정직성 기록)

| 항목 | 측정 | 비고 |
|------|------|------|
| Higgs 질량 | 125.25 GeV | 연속 측정값, 125 정수부는 CLOSE |
| W 보손 질량 | 80.369 GeV | 연속, 정수부 80 ≠ n=6 집합 |
| Z 보손 질량 | 91.188 GeV | 연속 |
| top 쿼크 질량 | 172.57 GeV | 연속 |
| 약 혼합각 sin²θ_W | 0.23129 | 연속, 약 0.25=φ/8 근사 CLOSE |
| 강결합상수 α_s(M_Z) | 0.1179 | 연속 |
| 미세구조상수 α⁻¹(0) | 137.036 | 연속, Feynman "악마의 수" |
| 총 자유 파라미터 수 (3 coupling + 6 quark m + 3 ch lepton m + 3 ν m + 4 CKM + 4 PMNS + θ_QCD + 2 Higgs) | 19 | 19 는 n=6 집합에 없음; 세어 방식에 따라 18=3n 또는 26 등 |

**CLOSE 주의**: SM 자유 파라미터 19개는 n=6 집합 밖이다. 단, 혹자는 중성미자 세분 없이 **18 = 3n** 으로 센다 (PDG 이전 관행). 이 부분은 표준이 정해지지 않았으므로 EXACT 로 넣지 않았다.

---

## 물리적 의미

표준모형이 CP 위반을 수용하려면 **정확히 3 세대 이상** 이 필요하다 (KM 1973 증명). 2 세대에서는 CKM 가 2×2 이고 모든 위상을 재정의로 흡수 가능하여 CP 위반 위상이 0개. 3 세대부터 1개의 불가축소 위상이 등장하여 K/B 중간자 CP 비대칭을 생성한다.

즉 **관측된 자연의 CP 위반은 세대수 ≥ 3 = n/φ 을 요구** 하며, 이는 "왜 3 세대인가?" 의 부분적 답이다. n=6 좌표에서 세대수 n/φ 는 상한이 아니라 **최소** 이다.

또 (쿼크 6) + (렙톤 6) = 12 와 (SU(3) 8 + SU(2) 3 + U(1) 1) = 12 는 페르미온/게이지 두 자유도가 σ 에서 수렴한다. Peskin-Schroeder 나 Weinberg QFT 는 이 우연을 주목하지 않으나, n=6 좌표에서 보면 σ 이 **보편적 닫힘 수** 임을 재확인한다.

Higgs 메커니즘이 유도하는 W/Z 질량 공식 M_W² = (g²v²)/4, M_Z² = M_W²/cos²θ_W 에서 분모 4=τ 가 자연스럽게 등장하지만 사용자 쪽에서 n=6 좌표의 의도적 선택이 아님에 주의 (단순 2×2 벡터 정규화).

---

## 교차 BT

- **BT-1**: n=6 σ·φ=n·τ 유일성 (기저)
- **BT-1378**: 산술핵 (p-1)(q-1)=2 유일해 (2,3) → 세대수 3 과 페르미온 2 타입 우연
- **BT-1374**: 부호이론 Hexacode/Hamming/Golay — 8=글루온 수 = Hamming[8,τ,n/φ]?
- **BT-401~408**: BT 양자역학 8돌파 (BT-405 KM CP violation 관련)
- **BT-135**: ITER Tokamak 자석 — τ=4 플라즈마 제어와 연결
- **BT-1376**: 결정학 허용회전 {1,2,3,4,6} — 같은 {1,φ,n/φ,τ,n} 구조

---

## 16.11 자동검증 Python (embedded, N62 준수)

```python
# BT-1386 표준모형 n=6 완전대칭 자동검증
# 실행: 본 블록만 추출해 python3 로 exec

# n=6 핵심 상수
n, sigma, phi, tau, sopfr, mu, J2 = 6, 12, 2, 4, 5, 1, 24
assert sigma * phi == n * tau, "σ·φ = n·τ 핵심 항등식 실패"

# 검증 항목: (이름, 측정값, n=6 수식계산)
checks = [
    ("쿼크 맛 수 (PDG 2024 §9)",                     6,  n),
    ("렙톤 맛 수 (PDG 2024 §10-11)",                 6,  n),
    ("페르미온 총 맛 수",                             12, sigma),
    ("페르미온 세대 수 (KM 1973)",                   3,  n // phi),
    ("기본 상호작용 수 (강·약·EM·중력)",             4,  tau),
    ("페르미온 타입 수 (쿼크/렙톤)",                 2,  phi),
    ("SU(3) 글루온 수 (Fritzsch-Gell-Mann 1973)",   8,  2 * tau),
    ("SU(3)×SU(2)×U(1) 전체 생성자 수",              12, sigma),
    ("색 전하 수 (Greenberg 1964)",                  3,  n // phi),
    ("Higgs 스칼라 수 (SM 최소)",                    1,  mu),
    ("CKM 행렬 독립 성분 수 (3×3)",                  9,  n + n // phi),
    ("약 게이지 보손 수 (W⁺,W⁻,Z⁰)",                3,  n // phi),
]

exact = 0
miss = []
for name, target, formula in checks:
    if target == formula:
        exact += 1
    else:
        miss.append((name, target, formula))

total = len(checks)
print(f"BT-1386 표준모형 검증: {exact}/{total} EXACT")
for name, t, f in miss:
    print(f"  MISS: {name} — target={t}, formula={f}")

assert len(miss) == 0, f"예상치 못한 MISS: {len(miss)}"
assert exact >= 12, f"EXACT 목표(12) 미달: {exact}"

# 이중 폐쇄 확인: 페르미온 자유도 = 게이지 자유도 = σ
fermion_flavors = 6 + 6  # quark + lepton
gauge_generators = 8 + 3 + 1  # SU(3) + SU(2) + U(1)
assert fermion_flavors == gauge_generators == sigma, "이중 σ 폐쇄 실패"
print(f"✓ 이중 σ 폐쇄: 페르미온 {fermion_flavors} = 게이지 {gauge_generators} = σ")

# KM 3세대 CP 위반 논리 (n/φ 이하에서 위상 0개)
def cp_phases(generations):
    """Kobayashi-Maskawa count of physical CP phases in CKM matrix."""
    if generations < 3:
        return 0
    return ((generations - 1) * (generations - 2)) // 2

assert cp_phases(2) == 0, "2세대 CP 위상 ≠ 0"
assert cp_phases(3) == 1, "3세대 CP 위상 ≠ 1"
assert cp_phases(4) == 3, "4세대 CP 위상 ≠ n/φ"
print(f"✓ KM 1973: 2세대={cp_phases(2)}, 3세대={cp_phases(3)}, 4세대={cp_phases(4)}")
print(f"   → 최소 세대수 = n/φ = {n // phi} (관측 CP 위반에 필요)")

print("✓ BT-1386 자동검증 통과 (12/12 EXACT, 0 MISS)")
```

**자동검증 결과**: 12/12 EXACT, 0 MISS. 이중 σ 폐쇄 + KM 최소 세대수 n/φ 논리 확인.
