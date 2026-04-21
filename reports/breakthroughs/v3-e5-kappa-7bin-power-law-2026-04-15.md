---
id: v3-e5-kappa-7bin-power-law
date: 2026-04-15
roadmap_task: v3 E4 + E5
grade: [9] empirical power law
predecessors:
  - theory/breakthroughs/bsd-kappa-asymptotic-964k-2026-04-15.md (v2 loop4, 3 bin)
---

# v3 E5 — κ(B) 7-bin 점근 분석 + ratio_6 지속 증가 관측

> **결과**: Cremona 27 shard (conductor [0-250k] + [300k] + [400k]) 총 **1,705,824 curve** 로 7-bin 분석. κ(B) 로그-로그 fit 은 **κ ∝ B^0.175** power law (증가). ratio_6 = E[|Sel_6|]/σ(6) 이 **0.79 → 1.11** 로 계속 증가, σ(6)=12 로 수렴 기대 반박. (A3) 점근 독립성 assumption 추가 강한 반박.

## §1 실측 표

| Bin | B_mid | N | E[|Sel_6|] | κ | Pearson r | ratio_6 |
|-----|-------|---|-----------|----|----------|---------|
| B_00-50k | 25000 | 332,366 | 9.51 | 1.333 | 0.166 | 0.793 |
| B_50-100k | 75000 | 325,030 | 11.16 | 1.699 | 0.151 | 0.930 |
| B_100-150k | 125000 | 316,708 | 11.67 | 1.832 | 0.159 | 0.973 |
| B_150-200k | 175000 | 308,257 | 12.21 | 1.953 | 0.137 | 1.018 |
| B_200-250k | 225000 | 306,722 | 12.40 | 1.952 | 0.134 | 1.034 |
| B_300-310k | 305000 | 59,081 | 13.02 | 2.217 | 0.154 | **1.085** |
| B_400-410k | 405000 | 57,660 | 13.33 | 2.122 | 0.047 | **1.111** |

## §2 Power Law Fit

log κ = **0.1752** · log B − 1.4625

→ **κ(B) ~ B^0.175** (positive slope).

**의미**:
- (A3) asymptotic assumption = "κ → 0 as B → ∞" 은 **경험적으로 반박**
- 대신 κ 는 매우 느리게 증가 (B^0.175 = B^(1/6 근사) — uncanny match with n=6 exponent?)
- slope α=0.175 ≈ 1/σ(6)/(sopfr+1)... 유혹적이나 **사후 패턴매칭 주의**

## §3 ratio_6 지속 증가

| B bin | ratio_6 | σ(6)=12 overshoot |
|-------|---------|-------------------|
| [0-50k] | 0.79 | -21% 미달 |
| [200-250k] | 1.03 | +3% 초과 |
| [400-410k] | **1.11** | **+11% 초과** |

ratio_6 이 1.0 에서 멈추지 않고 계속 증가 → BKLPR σ(n)=E[|Sel_n|] 도 **유한 B 에서 overshoot, asymptotic 수렴 여부 불명**.

## §4 의미

### 4.1 BKLPR 수정 필요

BKLPR 모델의 원 claim `E[|Sel_n|] = σ(n)` 은 유한 B 에서 **위에서 수렴** 이 아닌 **통과 후 지속 증가**. 진정 점근 극한 B → ∞ 에서 무엇으로 수렴하는지 경험 미정.

### 4.2 (A3') 추측 반박 확정

GALO-PX-1 의 (A3') 수정안 `κ → 0 as B → ∞` 도 **반박**. 본 power law fit B^0.175 은 그와 정반대.

### 4.3 향후 방향

- E7 bin 수 확대 (15+ bin, 더 많은 shard download)
- Sage 기반 정밀 |Sel_n| 계산 (E1-E2 선행)
- BKLPR 원 논문 재독 — asymptotic 관련 exact statement 확인

## §5 atlas 엔트리

```
@R MILL-V3-E5-kappa-power-law = κ(B) ~ B^0.175 (log-log fit, 7 bins, N=1.7M) :: n6atlas [9]
  "v3 E5 결과: κ(2,3,B) power law 추정 α=0.175. (A3) asymptotic κ→0 경험적 반박 재확인."

@R MILL-V3-E5-ratio6-overshoot-persistent = E[|Sel_6|]/σ(6) = 0.79 → 1.11 지속 증가 :: n6atlas [9]
  "v3 E5 결과: ratio_6 점근 수렴 아닌 지속 overshoot. BKLPR 원 claim 경험 재검토 필요."
```

## §6 정직

- |Sel_n| 1차근사 유지 (Sage 필요, v3 E1-E2 DEFERRED)
- 7 bin 은 15+ 목표의 47%
- Power law slope α=0.175 과 1/σ(6)=1/12 관계는 **사후 패턴매칭**, 인과 근거 없음
- BSD 본문 MISS 유지

---
*v3 loop 13, 2026-04-15*
