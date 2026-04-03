# N6 Safety Architecture

> n=6 완전수 산술로 안전 공학의 보편 상수를 통합하는 도메인.
> 소재 안전부터 시스템 방호까지 8단계 진화 래더.

## Quick Stats

| 항목 | 수치 |
|------|------|
| Core Hypotheses | 30 (H-SF-01 ~ H-SF-30) |
| Extreme Hypotheses | 20 (H-SFX-01 ~ H-SFX-20) |
| EXACT rate (core) | 20/30 = 66.7% |
| Evolution levels | 8 (소재→궁극) |
| Cross-domain coverage | 10 domains |
| Related BTs | BT-43, BT-60, BT-80~84, BT-118~122, BT-123~127 |

## Key Discoveries

```
  방호 계층 (DiD)      = n     = 6  (IAEA + LOPA)
  안전 등급 (SIL)      = τ     = 4  (IEC 61508)
  다중화 (TMR)         = n/φ   = 3  (항공/원자력/우주)
  안전 전압 (SELV DC)  = J₂    = 24V (IEC 60364)
  위험감소/계층         = σ-φ   = 10배 (SIL ladder)
  누전차단 (GFCI)      = sopfr·n = 30mA (IEC/NFPA)
  화재 등급             = n     = 6  (NFPA)
  GHS 그림문자          = σ-n/φ = 9  (UN GHS)
  진도 등급 (MMI)       = σ     = 12 (Modified Mercalli)
  퀜치 감지 시간        = 1/(σ-φ) = 0.1s (ITER/LHC)
  하인리히 법칙 300     = sopfr·n·(σ-φ) (극한 가설)
```

## File Structure

| File | Description |
|------|------------|
| [goal.md](goal.md) | 8단계 진화 래더 + DSE 후보군 + Cross-domain 커버리지 |
| [hypotheses.md](hypotheses.md) | 30 core hypotheses (8 Tiers) |
| [extreme-hypotheses.md](extreme-hypotheses.md) | 20 extreme hypotheses |

## n=6 Safety Equation

```
  잔여위험 = (1/(σ-φ))^n = (1/10)^6 = 10⁻⁶/yr
  
  ┌─────────────────────────────────────────────────┐
  │  n=6 방벽 × (σ-φ)=10배 감소/방벽 = 10⁻⁶ 목표  │
  │                                                 │
  │  Layer 1: 본질안전     → 0.1 잔여               │
  │  Layer 2: 예방 제어    → 0.01                    │
  │  Layer 3: 감지/경보    → 0.001                   │
  │  Layer 4: 안전계장     → 10⁻⁴                   │
  │  Layer 5: 물리적 방호  → 10⁻⁵                   │
  │  Layer 6: 비상대응     → 10⁻⁶ ✓                 │
  └─────────────────────────────────────────────────┘
```
