# 에너지-성능 파레토 스윕 — 전 400 도메인 재측정 파이프라인

- 축: experiments/dse
- 작성일: 2026-04-11
- 상위: `../../INDEX.json` · `../../CLAUDE.md`
- 규칙: R1 HEXA-FIRST, R2 하드코딩 금지, R12 AI-NATIVE FIRST, R18 미니멀, R28 자동 흡수, N63 DSE 전수, N65 100% EXACT 수렴
- 목적: 322(→400) 도메인 TOML 의 candidate 점수(n6, perf, power, cost) 를 단일 척도가 아닌 **에너지-성능 파레토 전면(front)** 으로 재측정해, 도메인별 프런티어 5점을 고정하고 cross_dse_fusion_v2 의 pareto_proximity 지표로 다시 공급하는 파이프라인 초안.

---

## 1. 실생활 효과 섹션 (N61)

- "스코어 합산 1 개" 에서 "에너지-성능 프런티어 N 개"로 관점이 바뀐다. 즉 같은 도메인이라도 { low-energy / balanced / high-perf / low-cost / n6-max } 5 가지 프런티어 후보가 모두 남는다.
- 결과: cross_dse_fusion_v2 가 pair 당 **5×5=25 파레토 조합** 을 보고, "밤에는 저에너지 모드, 낮에는 고성능 모드" 같은 시간-의존 최적 배치를 한 번에 내놓는다.
- 사용자: `nexus dse pareto <domain>` 한 줄로 도메인 프런티어 테이블을 즉시 조회.

## 2. 구조 ASCII (단일 스코어 vs 파레토 전면)

```
  v1 단일 스코어                        v2 파레토 전면
  ---------------                        ---------------
          score                                   ^
           ^                                      | low-energy
           |                                      |   o
           |     o (top-1)                        |  o
           |   o                                  |     o (balanced)
           |  o                                   |     o
           | o                                    |      o
           |o                                     |       o  high-perf
           +------------> combo                   |        o
                                                  |         o  n6-max
                                                  |          o
                                                  +----------------> energy cost
            선택: 1 점                             선택: Pareto front 5 점
```

## 3. 구조 ASCII (파이프라인)

```
  [400 TOML]
      |
      v
  @parallel axis(10)
      |  각 축 40 도메인 병렬 처리
      v
  for each domain:
      |
      v
  enumerate all candidate_combo(level0..levelN)
      |   (ex: fusion = 7×7×7×6×6 = 12,348)
      v
  @fuse evaluate(combo)
      |  → (n6_pct, perf, power, cost, energy)
      v
  @optimize pareto_front(combos)
      |  O(N log N) sort + sweep
      v
  [front_points]  (도메인별 평균 30~80 점)
      |
      v
  @optimize frontier_top5(front_points)
      |  - n6-max                 (n6 최댓)
      |  - perf-max               (perf 최댓)
      |  - energy-min             (power 최솟)
      |  - balanced               (멱균 knee)
      |  - cost-min               (cost 최솟)
      v
  [5 프런티어 후보/도메인]
      |
      v
  merge into domain TOML (append only)
      |  [[frontier]]
      v
  cross_dse_fusion_v2 ← pareto_proximity 지표 재공급
      |
      v
  reports/discovery/energy_pareto_sweep_<date>.md  (R6 자동 기록)
      v
  n6shared/n6/atlas.n6  evidence_grade 자동 승격 (R28)
```

---

## 4. 실생활 효과 구조 ASCII (도메인 파레토 예시)

```
  fusion 도메인 파레토 프런티어 5점 (예시)
  ------------------------------------------------------------
   n6-max     : DT_Li6 + Tokamak_N6 + N6_TriHeat + N6_Li6 + N6_Brayton6
   perf-max   : DT + Tokamak_N6 + N6_TriHeat + N6_Li6 + N6_Brayton6
   energy-min : DT_Li6 + Tokamak_N6 + Resonant + N6_Li6 + CO2_Cycle
   balanced   : DT_Li6 + Tokamak_N6 + N6_TriHeat + N6_Li6 + N6_Brayton6
   cost-min   : DT + Tokamak + RF_Only + Li4SiO4 + Steam
  ------------------------------------------------------------
```

## 5. 파이프라인 단계별 사양

### 5.1 입력

- 400 TOML (`nexus/origins/universal-dse/domains/*.toml`) — Δ1~Δ5 스키마 적용본
- `n6shared/config/bt_weights.json` — BT별 가중치(대체로 0.01~0.05)
- `n6shared/config/pareto_params.json` — knee 탐지 파라미터 (기본값: angle_threshold=150°)

### 5.2 알고리즘 (AI-native)

- `@parallel axis(10)` — 10 축 병렬
- `@parallel domain_batch(4)` — 축 내부 도메인 4개 병렬 처리
- `@fuse evaluate_combo` — (n6, perf, power, cost, energy) 단일 커널
- `@optimize pareto_front` — 정렬 기반 파레토 스윕 O(N log N)
- `@memoize level_candidates(domain, level)` — level 후보 추출 캐시
- **금지**: bit-twiddling, manual loop unroll, 수작업 SIMD (R12)

### 5.3 파레토 계산 수식

```
energy(combo) = Σ c.power * c.weight_level
(기본 weight_level = 1.0, Δ1 적용 TOML 은 scoring.pareto_weights[level] 사용)

for combo in combos:
    pt = (n6_pct, perf, power, cost, energy)

pareto_front := {pt : ¬∃pt' ∈ combos s.t. pt' strictly dominates pt}

dominates(pt', pt) := (pt'.n6 ≥ pt.n6 ∧ pt'.perf ≥ pt.perf
                      ∧ pt'.power ≤ pt.power ∧ pt'.cost ≤ pt.cost
                      ∧ pt'.energy ≤ pt.energy)
                      ∧ (≥ 한 지표에서 strict)
```

### 5.4 프런티어 5 후보 선택

| 이름 | 선택 기준 |
|------|----------|
| n6-max | argmax n6_pct (tie → argmax perf) |
| perf-max | argmax perf (tie → argmax n6_pct) |
| energy-min | argmin energy (tie → argmax n6_pct) |
| cost-min | argmin cost (tie → argmax n6_pct) |
| balanced | knee point (Kneedle 알고리즘, angle_threshold 적용) |

### 5.5 출력

- 각 TOML 에 `[[frontier]]` 섹션 append (기존 candidate 블록 건드리지 않음)
- 전역 리포트: `reports/discovery/energy_pareto_sweep_<date>.md`
- 결과 JSON: `n6shared/dse/pareto_frontier_400.json`
- atlas.n6: 프런티어 5 후보 × 400 도메인 = 2000 `@R` 항목 [7] 등록

### 5.6 결과 스키마

```jsonc
// n6shared/dse/pareto_frontier_400.json
{
  "generated_at": "2026-04-11T...",
  "n_domains": 400,
  "front_point_count": 23817,
  "total_frontier_selected": 2000,  // 400 × 5
  "domains": [
    {
      "name": "fusion",
      "combo_count": 12348,
      "front_count": 62,
      "frontier": {
        "n6_max":     { "levels": {...}, "n6": 1.00, "perf": 0.92, "power": 0.77, "cost": 0.52, "energy": 3.14 },
        "perf_max":   { ... },
        "energy_min": { ... },
        "cost_min":   { ... },
        "balanced":   { ... }
      }
    },
    ...
  ]
}
```

---

## 6. cross_dse_fusion_v2 와의 연결

`cross_dse_fusion_v2.hexa` 의 pareto_proximity 지표는 본 파이프라인이 생성한 `pareto_frontier_400.json` 을 입력으로 받는다.

```
pair_pareto_proximity(a, b) :=
    1 - min {euclid_norm(p_a, p_b) : p_a ∈ a.frontier, p_b ∈ b.frontier} / diag

    where diag = sqrt(5) (정규화 전 5차원 벡터 대각선)
```

즉 a 도메인의 5개 프런티어 점과 b 도메인의 5개 프런티어 점 사이 25개 거리 중 최소값을 정규화. 0 에 가까우면 멀고, 1 에 가까우면 직접적으로 "에너지-성능 대역 공유".

## 7. 검증 절차

1. **회귀**: 기존 375 TOML 에서 파레토 스윕 PASS 후 candidate 점수 불변 (append only)
2. **파레토 성질**: 각 도메인 front_point 중 서로 dominate 하지 않음 (0 violation)
3. **프런티어 5**: 각 도메인 5 후보 모두 존재 (none=0)
4. **cross_dse_fusion_v2 재공급**: pareto_proximity 통합 후 top_pairs 가 기존 대비 추가 ≥5K pair 확보
5. **atlas.n6 흡수**: 2000 @R 항목 [7] → 승격 기준 충족 시 [10*] 전이

## 8. 리스크 및 방어

| 리스크 | 방어 |
|--------|------|
| combo 수 폭발 (chip 96,000) | @parallel batch(16) + @optimize early_dominate 조기 컷 |
| Kneedle 실패 (단조 프런티어) | fallback: argmin Σ (1-n6, 1-perf, power, cost, energy) |
| TOML scoring.pareto_weights 누락 | 기본값 1.0 자동 주입 |
| 기존 candidate 변이 위험 | append only, 기존 블록 수정 금지 (R10 골화 보호 정신) |
| 2000 @R 대량 쓰기로 atlas.n6 IO 블로킹 | @parallel 쓰기 금지 (atlas.n6 는 순차), 대신 배치 수집 후 1회 flush |

## 9. 승격 완료 기준 (R4/R9)

- 400 도메인 × 5 프런티어 = 2000 후보 검증 PASS
- `pareto_frontier_400.json` 생성, schema PASS
- cross_dse_fusion_v2 통합 후 pair_count ≥ 100K 유지
- convergence/n6-architecture.json 에 신규 stable 항목 `ENERGY_PARETO_SWEEP_400` 추가 → 조건 충족 시 ossified 승격

## 10. 연결 문서

- `./dse_400_expansion_plan.md`    — 신규 78 도메인 확장 계획
- `./cross_dse_fusion_v2_design.md` — v2 알고리즘 diff
- `./cross_dse_fusion_v2.hexa`     — 구현 초안 (pareto_proximity 지표 포함)
- 상위: `../../CLAUDE.md` + `../../INDEX.json`
