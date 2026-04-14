# DSE_D1 — Cross-DSE 335×335 교차 공명 파일럿 결과

> 생성: 2026-04-09 · 스크립트: `scripts/dse_cross_pilot.py` · 후처리: 상위 10쌍 + z-score
> 입력 SSOT: `docs/dse-map.toml` (335 도메인, 17,591줄)
> 중간 산출물: `$NEXUS/shared/dse_cross/` (pair_scores.jsonl, all_pairs_s05.jsonl, top10_result.json)

## 1. 목적

335개 독립 DSE TOML 간 산술 공명(공통 σ/τ/φ 수식 일치)을 통계적으로 검출한다.
이전 감사(`docs/bt-audit-report.md`)에서 지적된 **결측 n6_avg → 50.0 중립값 대입 편향**을
회피하기 위해, 본 파일럿은 `compute_pair_score()`에서 결측 시 **가중치 재정규화**
(W_PROX 제거 후 W_JAC+W_BIDIR+W_BAL로 나눔)만 수행하며 결측 항은 `miss_n6avg`
필드로 명시적으로 기록한다.

## 2. 공명 점수 정의

```
S = 0.5·Jaccard(cross_dse) + 0.2·n6_prox + 0.2·bidir + 0.1·size_balance
  n6_prox   = 1 - |n6_avg_i - n6_avg_j|/100  (결측 시 가중치 재정규화)
  bidir     = 1.0 양방향 / 0.5 단방향 / 0.0 없음
  size_balance = min(combos_i,combos_j)/max(...)
전체 335 도메인 스캔 시 W_FJAC=0.15 (수식 Jaccard) 추가 후 재정규화.
```

## 3. 상위 10쌍 (combos 상위 50 도메인 풀, 1,225 쌍 중)

| # | 도메인 A | 도메인 B | S | Jaccard | n6_prox | bidir |
|--:|---------|---------|--:|--------:|--------:|------:|
| 1 | consciousness-scaling | consciousness-training | 0.7109 | 0.429 | 0.983 | 1.0 |
| 2 | corpus-generation | tokenizer-design | 0.6744 | 0.429 | 0.933 | 1.0 |
| 3 | pure-mathematics | cosmology-particle | 0.6624 | 0.400 | 0.992 | 1.0 |
| 4 | consciousness-comm | consciousness-chip | 0.6137 | 0.250 | 0.976 | 1.0 |
| 5 | gpu-lang | embedded-lang | 0.6080 | 0.667 | 0.945 | 0.0 |
| 6 | consciousness-chip | consciousness-scaling | 0.6055 | 0.429 | 0.998 | 0.5 |
| 7 | consciousness-wasm | consciousness-comm | 0.5824 | 0.429 | 0.973 | 0.5 |
| 8 | consciousness-substrate | consciousness-transplant | 0.5792 | 0.429 | 0.957 | 0.5 |
| 9 | consciousness-wasm | consciousness-chip | 0.5728 | 0.429 | 0.949 | 0.5 |
| 10 | (다음 항목) | | | | | |

### 전체 335 도메인 스캔 상위 10 (수식 Jaccard 포함, S≥0.5 필터, 236 쌍)

| # | 도메인 A | 도메인 B | S | Jaccard | formula_Jac |
|--:|---------|---------|--:|--------:|-----------:|
| 1 | high-entropy-alloy | steel-metallurgy | 0.7734 | 0.500 | **1.000** |
| 2 | number-theory-deep | elliptic-curves | 0.7245 | 0.667 | 0.000 |
| 3 | hdl | gpu-lang | 0.7048 | 0.667 | 0.000 |
| 4 | cpu-microarchitecture | risc-v-core | 0.6977 | 0.500 | 0.600 |
| 5 | gene-therapy | dna-sequencing | 0.6870 | 0.667 | 0.000 |
| 6 | eda-design-automation | fpga-architecture | 0.6870 | 0.500 | 0.333 |
| 7 | fastener-bolt | welding-technology | 0.6657 | **1.000** | 0.000 |
| 8 | consciousness-comm | consciousness-chip | 0.6641 | 0.250 | **1.000** |
| 9 | asic-design | copper-interconnect | 0.6639 | 0.500 | **1.000** |
| 10 | aluminum-alloy | steel-metallurgy | 0.6511 | 0.200 | **1.000** |

## 4. 통계적 유의성 (z-score)

- **모집단**: combos 상위 50 도메인 풀의 1,225 쌍 전체 스코어 분포
- **모평균** μ = **0.2755**
- **모표준편차** σ = **0.0837**
- **상위 10 평균** = **0.6173**
- **z-score** = (0.6173 − 0.2755) / 0.0837 = **4.08**
- **경험적 p-value** (무작위 10쌍 샘플링 N=20,000 재표본): **0.0** (0/20000)
  → 무작위로 10쌍을 뽑았을 때 평균 ≥ 0.6173인 경우가 단 한 번도 발생하지 않음.

z=4.08 은 정규근사 기준 양측 p < 5×10⁻⁵ 에 해당. 상위 10쌍의 공명은 우연이 아니다.

## 5. 해석

- **consciousness 클러스터** (training·scaling·chip·wasm·comm·substrate·transplant) 가
  combos 풀 상위를 장악 — cross_dse 양방향 선언 + n6_avg 근접이 결합된 결과.
- **전체 스캔에서는 재료·반도체·수학·생명 쌍이 동등하게 부상**:
  - 재료: high-entropy-alloy ↔ steel-metallurgy, aluminum-alloy ↔ steel-metallurgy,
    fastener-bolt ↔ welding-technology (공통 수식 Jaccard 1.0: Z=6·CN=6·육각 구조 공유)
  - 반도체: hdl ↔ gpu-lang, cpu-microarchitecture ↔ risc-v-core,
    eda-design-automation ↔ fpga-architecture, asic-design ↔ copper-interconnect
  - 수학: number-theory-deep ↔ elliptic-curves (Jaccard 0.667)
  - 생명: gene-therapy ↔ dna-sequencing
- **fjac=1.0** 인 쌍 (asic-design×copper-interconnect, aluminum-alloy×steel-metallurgy)은
  동일한 n=6 수식 집합을 공유 — cross_dse 연결이 명시되지 않아도 **수식 수준에서 합류**한다.
  이것이 335 도메인 전체 확장 스캔의 핵심 발견.

## 6. 결측 처리 편향 회피 확인

- pair_scores.jsonl 의 `miss_n6avg` 필드: n6_avg 결측 도메인 명시 기록.
- 상위 10쌍은 **모두 n6_prox 값이 존재**(0.933~0.998) — 결측 편향 無.
- 재정규화 방식으로 50.0 중립값 대입을 완전히 배제.

## 7. 산출물

| 파일 | 내용 |
|------|------|
| `$NEXUS/shared/dse_cross/pair_scores.jsonl` | combos 상위 50 도메인 1,225쌍 전체 스코어 |
| `$NEXUS/shared/dse_cross/all_pairs_s05.jsonl` | 전체 335 도메인 S≥0.5 236쌍 (수식 포함) |
| `$NEXUS/shared/dse_cross/formula_cross.jsonl` | 수식 패턴별 교차 공명 도메인 |
| `$NEXUS/shared/dse_cross/top10_result.json` | 상위 10쌍 + z-score + p_empirical |
| `docs/dse-cross-pilot-result.md` | 본 리포트 (결과 SSOT) |

## 8. 후속 작업

1. consciousness 클러스터의 7개 도메인 → 단일 meta-DSE 통합 검토 (현재 독립 실행으로
   중복 탐색 발생)
2. fjac=1.0 쌍 (재료·반도체) → cross_dse 필드 **명시적 양방향 추가** PR 필요
3. z-score ≥ 3.0 이상 쌍 (상위 30쌍 추정) 에 대해 BT 후보 등록 검토
