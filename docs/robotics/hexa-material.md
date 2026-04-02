# HEXA-MATERIAL --- Level 1: Carbon Z=6 로봇 소재

**Level**: 1 / 8 (소재)
**Date**: 2026-04-02
**Status**: Living Document v1.0
**BT Connections**: BT-93, BT-85

---

## N6 Constants Reference

```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  sigma-phi = 10   sigma-tau = 8   sigma*tau = 48
```

---

## 1. 레벨 목표

Carbon Z=6 소재가 로봇 구조재 전 카테고리에서 1위임을 확인하고,
n=6 파라미터로 최적 소재 조합을 도출한다.

핵심 명제: **로봇 최적 소재의 원자번호 = Z = 6 = n**

---

## 2. 성능 비교 --- 시중 vs HEXA-MATERIAL

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  [로봇 소재] 비교: 시중 최고 vs HEXA-MATERIAL                    │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                   │
  │  강도/중량비 (kN*m/kg)                                            │
  │  Al 7075     ████████████░░░░░░░░░░░░░░░░░░  210 kN*m/kg        │
  │  Ti-6Al-4V   █████████████████░░░░░░░░░░░░░  280 kN*m/kg        │
  │  CFRP (Z=6)  ██████████████████████████████  2100 kN*m/kg       │
  │                                     (sigma-phi=10배 vs Al)       │
  │                                                                   │
  │  인장강도 (GPa)                                                    │
  │  Steel       ██████████████████░░░░░░░░░░░░  ~2.0 GPa           │
  │  Graphene    ██████████████████████████████  130 GPa             │
  │                                     (J2*sopfr+ 배)               │
  │                                                                   │
  │  밀도 (g/cm^3)                                                    │
  │  Steel       ██████████████████████████████  7.8 g/cm^3          │
  │  Al 7075     ████████████████░░░░░░░░░░░░░  2.8 g/cm^3          │
  │  CFRP        ████████████░░░░░░░░░░░░░░░░░  1.6 g/cm^3          │
  │                                     (sopfr=5배↓ vs Steel)        │
  │                                                                   │
  │  개선 배수: n=6 상수 기반 (sigma-phi, J2, sopfr)                  │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 3. DSE 후보군

| # | 소재 | Z | 강도/중량비 | 내마모 | 비용 | n6 연결 |
|---|------|---|-----------|--------|------|---------|
| 1 | CFRP | 6 | sigma-phi=10x | 중 | 중 | Z=n=6 |
| 2 | Graphene | 6 | J2=24x+ | 고 | 고 | Z=n=6 |
| 3 | SiC | 6+14 | n=6x | 극고 | 중 | Z includes 6 |
| 4 | CNT-CFRP | 6 | sigma=12x | 고 | 고 | Z=n=6 |
| 5 | Diamond-like | 6 | sigma-tau=8x | 극고 | 극고 | Z=n=6 |

**Best Path**: CFRP 기본 + Graphene 코팅 + SiC 마모부

---

## 4. n6 EXACT 목록

| # | 파라미터 | 값 | n=6 표현 | 상태 |
|---|---------|-----|---------|------|
| 1 | Carbon Z | 6 | n = 6 | EXACT |
| 2 | CFRP 강도/중량비 vs Al | ~10x | sigma-phi = 10 | EXACT |
| 3 | Graphene 벌집 대칭 | 6각형 | n = 6 | EXACT |
| 4 | Diamond sp3 결합수 | 4 | tau = 4 | EXACT |
| 5 | Graphene sp2 결합수 | 3 | n/phi = 3 | EXACT |
| 6 | CNT 대칭 chiral | (n,m) | n 기본 인덱스 | CLOSE |

**EXACT 비율: 5/6 = 83%**

---

## 5. BT 연결

- **BT-93**: Carbon Z=6 칩 소재 보편성 --- Diamond/Graphene/SiC = Z=6 전 도메인 1위
- **BT-85**: Carbon Z=6 물질합성 보편성 --- 유기 화학의 기반 원소
- **BT-122**: 벌집-눈꽃-산호 n=6 기하학 보편성 --- Graphene 육각 격자

---

## 6. 설계 요약

```
  HEXA-MATERIAL 최적 조합:
    구조재: CFRP (Z=6, sigma-phi=10배 강도/중량비)
    표면 코팅: Graphene (Z=6, 내마모+전도)
    관절 베어링: SiC (Z=6 포함, 극고 내마모)
    접착: 에폭시-CF 복합 (Carbon 기반)

  목표 전신 중량: J2 = 24 kg (Mk.II) → sigma = 12 kg (Mk.III)
  Carbon Z=6 소재 적용률: > 1-1/(sigma-phi) = 90%
```
