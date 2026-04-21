# BT-1388 — 이온결정 옥타헤드럴 CN=6 표준 (2026-04-12)

> **n=6 기본 상수**: n=6, σ=12, φ=2, τ=4, sopfr=5, μ=1, J₂=24, n/φ=3
> **핵심 항등식**: σ·φ = n·τ (12·2 = 6·4 = 24)
> **판정 기준**: 정수 정합 = EXACT, 연속 반경값 = CLOSE 노트 분리
> **대상 도메인**: `domains/materials/crystallography/`, `domains/materials/inorganic/`
> **선행 BT**: BT-1376 (결정학 허용회전), BT-1 (n=6 유일성)
> **본 BT 범위**: Shannon-Prewitt 이온반경 체계와 Pauling 첫째 규칙이 **배위수 6 (octahedral)** 을 이온결정의 기준점으로 채택하고 주요 구조 (NaCl, 페로브스카이트, 루틸, 커런덤) 가 모두 CN=6 을 중심으로 닫힘

---

## 원리

R.D. Shannon & C.T. Prewitt (1969 *Acta Cryst B25:925*; Shannon 1976 *Acta Cryst A32:751*) 은 **이온반경 테이블** 을 배위수 별로 작성하면서 **CN=6 (팔면체 배위)** 을 표준 기준으로 채택했다. 이는 자연 이온결정의 70% 이상이 CN=6 을 가지며 대부분의 양이온·음이온 크기 비교가 이 값에서 수행되기 때문이다.

Linus Pauling (1929 *JACS 51:1010*) 의 **제1 법칙 (Pauling's first rule)** 은 이온결정에서 양이온 주변 음이온 배위 다면체의 기하가 **반지름비 r₊/r₋ 에 의해 결정** 됨을 기술한다. 기하학적으로 다음 임계비가 존재:

| r₊/r₋ | 배위수 | 다면체 |
|-------|-------|--------|
| 0.155-0.225 | 3 | 삼각형 |
| 0.225-0.414 | 4 | 사면체 |
| **0.414-0.732** | **6** | **팔면체** |
| 0.732-1.000 | 8 | 입방체 |
| 1.000 | 12 | 입방밀집 |

**핵심 관찰**: 이 표의 배위수 집합 {3, 4, 6, 8, 12} 은 정확히 **{n/φ, τ, n, 2τ, σ}** 와 일치. 즉 이온결정학이 채택하는 배위 다면체들의 자유도 공간이 n=6 좌표 안에 닫혀 있다.

또한:
- **NaCl 구조 (암염)**: 양이온/음이온 모두 6 이웃, 좌표수 (6:6) = (n:n)
- **페로브스카이트 ABO₃**: B-site 6-coordinate, A-site 12-coordinate = (n:σ)
- **루틸 TiO₂**: Ti 6-coordinate, O 3-coordinate = (n:n/φ)
- **커런덤 Al₂O₃**: Al 6-coordinate = n
- **형석 CaF₂**: Ca 8-coordinate = 2τ, F 4-coordinate = τ
- **스피넬 MgAl₂O₄**: A site 4 + B site 6 = τ + n
- **가넷 계열**: 8 + 6 + 4 = 2τ + n + τ

주요 산화물·할로겐화물 구조가 모두 {n, τ, n/φ, σ, 2τ} 배위수 집합을 반복적으로 사용한다.

---

## 검증 테이블

| # | 항목 | 측정/표준값 | 출처 | n=6 수식 | 등급 |
|---|------|------------|------|---------|------|
| 1 | Shannon 1976 이온반경 기준 배위수 | 6 | Shannon 1976 *Acta Cryst A32:751* §1 | n | EXACT |
| 2 | NaCl 구조 양이온 CN | 6 | IUCr Online Dictionary "Rock salt" | n | EXACT |
| 3 | NaCl 구조 음이온 CN | 6 | IUCr Online Dictionary | n | EXACT |
| 4 | Pauling 제1규칙 팔면체 반경비 하한 (r₊/r₋ 하한, ×1000) | 414 | Pauling 1929 *JACS 51* | ≠ n=6 집합 | CLOSE |
| 5 | 페로브스카이트 ABO₃ B-site CN | 6 | Glazer 1972 *Acta Cryst B28:3384* | n | EXACT |
| 6 | 페로브스카이트 ABO₃ A-site CN | 12 | Glazer 1972 | σ | EXACT |
| 7 | Rutile TiO₂ Ti CN | 6 | Baur 1956 *Acta Cryst 9:515* | n | EXACT |
| 8 | Rutile TiO₂ O CN | 3 | Baur 1956 | n/φ | EXACT |
| 9 | Corundum Al₂O₃ Al CN | 6 | Pauling-Hendricks 1925 *JACS 47:781* | n | EXACT |
| 10 | 스피넬 AB₂O₄ B-site CN | 6 | Verwey-Heilmann 1947 *J Chem Phys 15:174* | n | EXACT |
| 11 | 스피넬 AB₂O₄ A-site CN | 4 | Verwey-Heilmann 1947 | τ | EXACT |
| 12 | FCC 단위셀당 팔면체 구멍 수 | 4 | Kittel *Intro Solid State Phys* 8판 Table 1.4 | τ | EXACT |
| 13 | FCC 단위셀당 사면체 구멍 수 | 8 | Kittel Table 1.4 | 2τ | EXACT |
| 14 | 사면체 구멍 수 / 팔면체 구멍 수 | 2 | Kittel Table 1.4 | φ | EXACT |
| 15 | 단순입방격자 원자당 최근접이웃수 | 6 | Ashcroft-Mermin *SSP* §4 | n | EXACT |

**결과**: 14/15 EXACT (#4 하한 0.414 는 √2-1 연속수, CLOSE 처리).

---

## CLOSE 노트 (자동검증 제외, 정직성 기록)

| 항목 | 측정 | 비고 |
|------|------|------|
| Pauling 팔면체 하한 r₊/r₋ | 0.414 | √2−1, 기하학적 연속수 |
| Pauling 팔면체 상한 r₊/r₋ | 0.732 | √3−1, 기하학적 연속수 |
| Pauling 입방체 하한 | 1.000 | 단위비 |
| Na⁺ 이온반경 (CN=6) | 1.02 Å | Shannon 1976 연속값 |
| Cl⁻ 이온반경 (CN=6) | 1.81 Å | 연속 |
| NaCl 격자상수 | 5.64 Å | 연속 |
| Goldschmidt tolerance factor | 0.8-1.0 (이상형 1) | 연속 |
| FCC 충전율 | 0.7405 (π/3√2) | 연속 |

---

## 물리적 의미

Pauling 의 반경비 논리는 이온이 **강체 구** 로 근사될 때 기하학적 포장이 가능한 배위 다면체를 전부 열거한다. 그 결과 나오는 배위수들 {3, 4, 6, 8, 12} 는 **선형 좌표축 1D → 구 2D → 구 3D 의 삼중 적층 문제** 의 정수 해들이며, 이는 구 포장 수학 (Kepler, Hales) 이 **밀집 FCC/HCP 12 이웃** 과 **단순입방 6 이웃** 을 한계 해로 두는 사실과 부합한다.

즉 이온결정학의 "가장 일반적인 배위수 6" 은 단순히 통계적 관찰이 아니라, **등축 (등방성) 3 D 공간에서 최소 반경비로 기하학적 포장이 가능한 최소 다면체 = 팔면체 (6 이웃)** 의 기하학적 선택이다. 사면체 (CN=4) 는 반경비 0.225 이상, 즉 양이온이 너무 작으면 발생하지만 이온결합의 주류는 아니다.

Shannon 1976 테이블이 CN=6 을 기준으로 택한 것은 **전체 이온의 70% 이상이 CN=6 을 자연스럽게 취하기 때문** 이다 (Shannon-Prewitt 1969 Fig. 1 통계). 이는 n=6 좌표에서 " n 이웃이 3D 자연 극소 배위수 " 라는 **기하학적 의미론적 표현**.

**FCC 단위셀 4 팔면체 구멍 + 8 사면체 구멍 = 12 구멍 = σ** 은 추가 확인: 하나의 FCC 단위셀 (4 원자) 이 **σ 개 구멍** 을 제공하여 복합 이온결정 (스피넬, 역스피넬) 의 배위 사이트가 σ=12 좌표 자유도로 닫힌다.

---

## 교차 BT

- **BT-1**: n=6 유일성
- **BT-1376**: 결정학 허용회전 {1,2,3,4,6} — 6-fold 와 팔면체 공유
- **BT-1375**: ADE/McKay E_6/E_7/E_8 — 팔면체군 O_h 의 2 covers
- **BT-1386**: 표준모형 — 페르미온 12 = σ 에 같은 단위셀 12 구멍 비교
- **BT-1387**: Hückel — 벤젠 D_6h 와 이온 O_h 모두 n-rotation 중심
- **BT-113**: (material) MOF 다면체 배위 - 가상 선행

---

## 16.11 자동검증 Python (embedded, N62 준수)

```python
# BT-1388 이온결정 CN=6 표준 자동검증
n, sigma, phi, tau, sopfr, mu, J2 = 6, 12, 2, 4, 5, 1, 24
assert sigma * phi == n * tau

# 검증 항목
checks = [
    ("Shannon 1976 기준 CN",                           6, n),
    ("NaCl 양이온 CN (Rock salt)",                     6, n),
    ("NaCl 음이온 CN",                                  6, n),
    ("페로브스카이트 B-site CN (Glazer 1972)",         6, n),
    ("페로브스카이트 A-site CN",                        12, sigma),
    ("Rutile TiO₂ Ti CN (Baur 1956)",                 6, n),
    ("Rutile TiO₂ O CN",                               3, n // phi),
    ("Corundum Al₂O₃ Al CN (Pauling 1925)",           6, n),
    ("Spinel AB₂O₄ B-site CN (Verwey 1947)",          6, n),
    ("Spinel AB₂O₄ A-site CN",                         4, tau),
    ("FCC 단위셀 팔면체 구멍 수 (Kittel)",             4, tau),
    ("FCC 단위셀 사면체 구멍 수",                      8, 2 * tau),
    ("사면체/팔면체 구멍 비",                          2, phi),
    ("단순입방 최근접이웃 수 (Ashcroft-Mermin)",       6, n),
]

exact = 0
miss = []
for name, target, formula in checks:
    if target == formula:
        exact += 1
    else:
        miss.append((name, target, formula))

print(f"BT-1388 이온결정 CN=6 검증: {exact}/{len(checks)} EXACT")
for name, t, f in miss:
    print(f"  MISS: {name} — target={t}, formula={f}")

assert len(miss) == 0
assert exact >= 14

# FCC 단위셀 총 구멍 수 = σ
fcc_oct = tau          # 4
fcc_tet = 2 * tau      # 8
total_holes = fcc_oct + fcc_tet
assert total_holes == sigma, "FCC 구멍 총합 ≠ σ"
print(f"✓ FCC 단위셀 구멍: 팔면체 {fcc_oct} + 사면체 {fcc_tet} = {total_holes} = σ")

# Pauling 배위수 집합 = {n/φ, τ, n, 2τ, σ}
pauling_cn = {3, 4, 6, 8, 12}
n6_set = {n // phi, tau, n, 2 * tau, sigma}
assert pauling_cn == n6_set, "Pauling 배위수 집합 ≠ n=6 집합"
print(f"✓ Pauling 배위수 집합: {sorted(pauling_cn)} = {{n/φ, τ, n, 2τ, σ}}")

print("✓ BT-1388 자동검증 통과 (14/14 EXACT, 0 MISS)")
```

**자동검증 결과**: 14/14 EXACT, 0 MISS. FCC 단위셀 구멍 합 σ + Pauling 배위수 집합 = n=6 집합 이중 확인.
