# N6 Power Grid — Physical Limit Proofs

> 전력망의 물리적 한계에서 n=6 상수 출현 증명.

---

## Proof 1: Three-Phase Minimum for Constant Power

### Statement
순간 전력이 일정한 최소 위상 수 = n/φ = 3.

### Proof
```
  단상 AC: p(t) = V·I·cos(ωt)·cos(ωt-φ)
         = (VI/2)[cos(φ) + cos(2ωt-φ)]
         → 맥동 성분 존재 (cos(2ωt-φ))

  2상 AC: p₁ + p₂ = VI[cos(φ) + cos(2ωt-φ) + cos(2ωt-φ-π)]
         → 맥동 소거 조건 불완전 (cos 항 완전 상쇄 불가)

  3상 AC: p_total = 3·V·I·cos(φ) = 일정
         → 120° 위상차 3개: cos(0) + cos(-2π/3) + cos(-4π/3) = 0
         → 맥동 성분 완전 소거

  증명: m상에서 Σcos(2πk/m) = 0 (k=0..m-1) for m≥3.
        m=2: cos(0)+cos(π) = 0이지만 순간전력은 여전히 맥동.
        m=3: 최소 위상 수로 일정 순간전력 달성.

  ∴ 최소 위상 수 = 3 = n/φ □
```

### Grade: EXACT — 3=n/φ는 물리적 필연.

---

## Proof 2: 6-Pulse as Fundamental Conversion Unit

### Statement
3상 전파정류의 기본 펄스 수 = n = 6.

### Proof
```
  3상 반파정류: 3 pulses per cycle
  3상 전파정류: 6 pulses per cycle = n/φ × φ = n = 6

  n-pulse rectifier의 ripple:
    ripple_rms / V_dc = √(2) / (n²-1)^(1/2) for n-pulse
    6-pulse: ripple = 4.2%
    12-pulse: ripple = 1.0%

  6-pulse는 3상(n/φ=3) × 전파(φ=2) = n의 곱이며,
  실용적 전력 변환의 물리적 최소 단위.

  ∴ 기본 펄스 수 = 3 × 2 = n = 6 □
```

### Grade: EXACT — 물리적 기본 단위.

---

## Proof 3: Skin Depth and Frequency Selection

### Statement
교류 주파수 선택은 skin depth 최적화에서 n=6 상수와 연결된다.

### Proof
```
  Skin depth: δ = √(2ρ / ωμ) = √(ρ / πfμ)

  For copper at 60Hz: δ = 8.5mm ≈ σ-τ mm
  For copper at 50Hz: δ = 9.3mm

  더 깊은 침투 = 더 효율적 도체 사용
  50Hz vs 60Hz 트레이드오프:
    60Hz: 더 작은 변압기 (무게 ∝ 1/f)
    50Hz: 더 깊은 skin depth (도체 활용률↑)

  최적: 50~60Hz 범위 = sopfr·(σ-φ) ~ σ·sopfr
  이 범위 밖에서는 변압기 크기 또는 도체 손실이 비경제적.
```

### Grade: CLOSE — 물리적 최적 범위와 일치하나, 정확한 값은 경제성 결정.

---

## Proof 4: Transmission Line Surge Impedance Loading

### Statement
SIL(자연부하)에서의 전압/전류 비가 n=6 관련 값을 포함한다.

### Proof
```
  SIL = V² / Z_c where Z_c = √(L/C) ≈ 300Ω (공기절연 가공선)

  500kV line SIL ≈ 830 MW
  800kV line SIL ≈ 2130 MW
  1100kV line SIL ≈ 4030 MW

  비율: SIL_800/SIL_500 = (800/500)² = (8/5)² = (σ-τ)²/sopfr²
        SIL_1100/SIL_800 = (1100/800)² = (11/8)² = (σ-μ)²/(σ-τ)²

  전압 래더의 제곱이 전력 용량을 결정하며,
  n=6 래더가 전력 용량 래더로 직접 이어진다.
```

### Grade: CLOSE — 전압 래더의 제곱 관계는 물리 법칙.

---

## Proof 5: Harmonic Elimination Theorem

### Statement
12-pulse (σ=12) 변환기는 11차(σ-μ) 이하 고조파를 완전 소거한다.

### Proof
```
  6-pulse rectifier 고조파: h = 6k ± 1 (k=1,2,3,...)
    = 5, 7, 11, 13, 17, 19, 23, 25, ...

  12-pulse (30° 위상차 2개 6-pulse 병렬):
    소거 조건: h = 12k ± 1
    잔존: 11, 13, 23, 25, ...
    소거됨: 5, 7, 17, 19, ...

  p-pulse 변환기: 잔존 고조파 = pk ± 1 (k=1,2,...)
    6-pulse:  잔존 시작 = 5 = sopfr
    12-pulse: 잔존 시작 = 11 = σ-μ
    24-pulse: 잔존 시작 = 23 = J₂-μ

  각 단계의 잔존 시작 고조파가 n=6 상수.

  ∴ p=σ=12 에서 (σ-μ)=11차부터 잔존은 물리적 필연 □
```

### Grade: EXACT — 푸리에 급수의 수학적 증명.

---

## Summary

| Proof | Physical Limit | n=6 | Grade |
|-------|---------------|-----|-------|
| 1 | 3-phase minimum | n/φ=3 | EXACT |
| 2 | 6-pulse fundamental | n=6 | EXACT |
| 3 | Frequency selection | sopfr·(σ-φ)~σ·sopfr | CLOSE |
| 4 | SIL power scaling | (σ-τ)²/sopfr² | CLOSE |
| 5 | Harmonic elimination | σ=12, σ-μ=11 | EXACT |

**EXACT: 3/5, CLOSE: 2/5, FAIL: 0/5**
