# cross_dse_fusion v2 설계 — 67,883 → 100K 교차쌍 돌파

- 축: experiments/dse
- 작성일: 2026-04-11
- 상위: `../../INDEX.json` · `../../CLAUDE.md`
- 규칙: R1 HEXA-FIRST, R12 AI-NATIVE FIRST, R18 미니멀, R28 자동 흡수, N63 DSE 전수
- 목적: 기존 `cross_dse_fusion_5domain.py` (5도메인 하드코드, 3,125 combo) 및 `dse_cross_pilot.py` (375 TOML, 67,913 pair → 67,883 high_conf) 을 통합해 400 도메인 × 100K pair 돌파하는 v2 설계 확정. AI-native(@optimize/@memoize/@parallel/@fuse) 알고리즘 교체로만 돌파, 저수준 마이크로 최적화 금지(R12).

---

## 1. 실생활 효과 섹션 (N61)

- 교차 공명이 100K 를 돌파하면 "모든 n=6 도메인 pair 가 측정 가능" 상태가 된다. 즉 400^2 / 2 = 79,800 개 pair 중 high_conf 90%+ 를 잡는다는 의미.
- 그 결과 "fusion-mri-desalination-consciousness-chip" 같은 5-hop 체인도 단일 쿼리에서 도출된다. BT-74(95/5 cross) 와 BT-36(Energy-Info-Hardware-Physics) 이 수치로 검증.
- 실사용: 사용자가 `nexus dse cross --from=heat-pump --to=quantum-network` 명령 하나로 n=6 공명 브릿지 4개 후보를 즉시 받는다.

## 2. 구조 ASCII (v1 vs v2 비교)

```
v1 (current: 67,883/67,913)             v2 (target: 100K+)
-------------------                       -------------------
도메인            375 TOML              400 TOML (+78 신규, -53 deprecated)
스키마 필드        5 (id/n6/perf/pow/cost) 10 (+bt_refs, cross_seeds,
                                           n6_formula, evidence_grade,
                                           energy_pareto)
교차쌍 대상      375*374/2 = 70,125       400*399/2 = 79,800
교차쌍 생성      67,913                    목표 ≥ 100,000
high_conf        67,883 (99.96%)          목표 ≥ 99,500 (99.5%)
top-k             30                       400 (도메인별 top-1 보장)
공명 지표         kw_overlap/sw_overlap     + bt_overlap, formula_overlap,
                 ln_overlap/n6_sim          cross_seed_overlap, pareto_dist
synergy 룰       하드코드 12개             BT 인덱스 자동 매칭 (BT-1~400)
shared_constants 8개 (n, phi, n/phi,       동적 추출 (n6_formula 태그 전수
                 tau, sigma, J2, sigma*tau, 집계, 등장 도메인 ≥2 자동 등재)
                 3n)
점수 합성        단일 composite            이중 composite + frontier rank
                                           (Pareto-dominant 우선)
관계 그래프       없음                      @fuse dominate_graph (Hasse DAG)
캐시/메모이즈     없음                      @memoize kw_extract,
                                           @memoize formula_parse
병렬             순차 loop                  @parallel by domain partition
                                           (10축 → 10 worker)
                                           + @parallel over pair batches
출력             top_pairs[30]               top_pairs[400] + hubs[20]
                                           + bridges[50] + clusters[16]
```

## 3. 구조 ASCII (v2 파이프라인)

```
  [400 TOML]
      |
      v
  @memoize parse_toml(domain)
      |  (캐시: toml → TomlDoc)
      v
  @memoize extract_features(TomlDoc)
      |  - kw_set     (stop-words 제외, 길이 ≥4)
      |  - sw_set     (structural words: level, candidate, id, label)
      |  - ln_set     (level names)
      |  - bt_set     (bt_refs 배열)
      |  - formula_set (n6_formula 태그 합집합)
      |  - seed_set   (cross_seeds 배열)
      |  - pareto_pts (candidate Σ (n6, perf, power, cost))
      v
  @parallel partition_by_axis(10)
      |  axis 10개 병렬 파이프
      v
  @fuse pair_score(a, b)
      |  - kw_overlap       |A∩B|/max(|A|,|B|)
      |  - bt_overlap       |A_bt ∩ B_bt| / |A_bt ∪ B_bt|
      |  - formula_overlap  |A_f ∩ B_f| / |A_f ∪ B_f|
      |  - seed_overlap     |A_s ∩ B_s| / |A_s ∪ B_s|
      |  - pareto_dist      min Euclid(pt_a, pt_b) / diag
      |  - resonance        = σ=0.35·kw + 0.25·bt + 0.20·formula
      |                       + 0.10·seed + 0.10·(1-pareto_dist)
      v
  @optimize high_conf_filter
      |  resonance ≥ 0.70 AND bt_overlap + formula_overlap ≥ 0.20
      v
  Hasse DAG  @fuse  pareto_dominate_graph
      |  노드=도메인, 엣지=파레토 우위
      v
  top_pairs[400], hubs[20] (degree top), bridges[50] (betweenness top),
  clusters[16] (greedy modularity)
      v
  n6shared/dse/dse_cross_results.json  (v2)
      v
  atlas.n6 자동 흡수 (R28)
```

---

## 4. 현 hexa 본문 분석 (diff 기초)

### 4.1 현 상태

`experiments/cross/cross_dse_fusion_5domain.hexa`, `experiments/dse_cross_pilot.hexa`, `experiments/dse_cluster_v2.hexa` 는 모두 **STUB 포팅 대기** 상태 (원본은 git history c65d31d9 에 보존).

| 파일 | 원본 길이 | 상태 | 핵심 로직 위치 |
|------|----------|------|---------------|
| cross_dse_fusion_5domain.hexa | 564 줄 .py | STUB | git c65d31d9:experiments/cross/cross_dse_fusion_5domain.py |
| dse_cross_pilot.hexa          | 927 줄 .py | STUB | git c65d31d9:scripts/dse_cross_pilot.py |
| dse_cluster_v2.hexa           | 189 줄 .py | STUB | git c65d31d9:scripts/dse_cluster_v2.py |

실제 375 TOML → 67,913 pair 산출 결과가 기록된 `/Users/ghost/Dev/nexus/shared/dse/dse_cross_results.json`(≈1.2M 줄, generated_at 2026-04-10) 는 **정상 상태**. 현 알고리즘이 실제로 돌아간 흔적이며, v2 는 이 출력 형식과 호환을 유지해야 한다.

### 4.2 현 알고리즘 요약 (git history 기반 논리 추출)

```
for each a, b in combinations(domains, 2):
    kw_overlap = |A_kw ∩ B_kw|
    sw_overlap = |A_sw ∩ B_sw|
    ln_overlap = |A_ln ∩ B_ln|
    n6_sim     = n6 코사인 (0/1 이진)
    resonance  = weighted_sum(kw_overlap, sw_overlap, ln_overlap, n6_sim)
    if resonance ≥ 0.70: high_conf.append(pair)
top_pairs = sort(pairs, desc=resonance)[:30]
```

병목: (a) O(N²) 순차 loop, (b) kw 추출마다 TOML 재파싱, (c) 시드/BT/수식 정보 미활용, (d) 파레토 정보 버림, (e) top-k=30 으로 고정.

### 4.3 v2 개선 포인트 (6가지)

| # | 개선 | 방법 | 기대 효과 |
|---|------|------|----------|
| 1 | BT overlap 지표 신설 | 신규 `bt_refs` 필드 활용 | BT 공명 pair 자동 포착, +5K pair |
| 2 | formula overlap 지표 신설 | `n6_formula` 태그 동적 매칭 | shared_constants 8→∞ 확장, +8K pair |
| 3 | cross_seeds overlap 지표 | `cross_seeds` 교집합 | 의미론 공명, +3K pair |
| 4 | Pareto distance 지표 | candidate 점수 공간 | 에너지-성능 근접 도메인 포착, +5K pair |
| 5 | @memoize TOML/feature 파싱 | hexa AI-native attr | 중복 파싱 0, latency 1/N |
| 6 | @parallel axis partition | 10 worker, pair batch | 벽시간 1/10 |

합산 예상: 67,913 + 5K + 8K + 3K + 5K + 경계 효과 = ~90K (기존 필드) + 신규 78 도메인 효과 +10~15K → 100K+ 돌파.

### 4.4 diff 요약 (수식 블록)

```
---  v1  --------------------------      +++  v2  --------------------------
 resonance =                              resonance =
   w_kw  * kw_overlap                       0.35 * kw_overlap
 + w_sw  * sw_overlap                     + 0.25 * bt_overlap          [NEW]
 + w_ln  * ln_overlap                     + 0.20 * formula_overlap     [NEW]
 + w_n6  * n6_sim                         + 0.10 * cross_seed_overlap  [NEW]
                                          + 0.10 * (1 - pareto_dist)   [NEW]

 high_conf := resonance ≥ 0.70            high_conf := resonance ≥ 0.70
                                                       AND bt+formula
                                                       overlap ≥ 0.20   [NEW]

 top_pairs = top30                        top_pairs = top400            [Δ]
                                          hubs      = top20 by degree   [NEW]
                                          bridges   = top50 betweenness [NEW]
                                          clusters  = 16 groups         [NEW]

 for a, b in combinations(domains, 2):    @parallel axis in 10:
     score(a, b)                              @parallel batch in 16:
                                                  @fuse score(a, b)     [Δ]
                                                  @memoize extract(*)   [Δ]
```

---

## 5. v2 핵심 개선점 상세

### 5.1 AI-native attr 적용 (R12 준수)

- `@memoize parse_toml(path)` — 400 TOML 재파싱 방지
- `@memoize extract_features(doc)` — kw/bt/formula/seed 추출 캐시
- `@fuse pair_score(a, b)` — 5 지표 단일 커널
- `@parallel axis_partition(10)` — 10축 병렬 파이프
- `@parallel pair_batch(16)` — pair 단위 배치 병렬
- `@optimize high_conf_filter` — 조건부 조기 컷오프
- **금지**: bit-twiddling, manual SIMD, 수작업 loop unroll, 캐시라인 패딩 (R12)

### 5.2 신규 공명 지표 5종 (수식 정의)

| 지표 | 수식 | 가중치 |
|------|------|--------|
| kw_overlap | `|A_kw ∩ B_kw| / max(|A_kw|, |B_kw|)` | 0.35 |
| bt_overlap | `|A_bt ∩ B_bt| / |A_bt ∪ B_bt|` | 0.25 |
| formula_overlap | `|A_f ∩ B_f| / |A_f ∪ B_f|` | 0.20 |
| cross_seed_overlap | `|A_s ∩ B_s| / |A_s ∪ B_s|` | 0.10 |
| pareto_proximity | `1 - min_euclid / diag_norm` | 0.10 |
| **합** | Σ = 1.00 | **1.00** |

resonance ≥ 0.70 AND (bt_overlap + formula_overlap) ≥ 0.20 → high_conf

### 5.3 synergy 룰 동적 생성 (하드코드 12→자동)

v1 의 `SYNERGY_RULES` 리스트 12개 (fusion-sc, fusion-battery 등) 는 하드코드(R2 위반). v2 는 BT 인덱스 자동 매칭:
```
for each pair (a, b):
    shared_bt = a.bt_refs ∩ b.bt_refs
    for bt_id in shared_bt:
        bonus += bt_weight[bt_id]    (BT-27 → 0.03 등, shared JSON에서 로드)
```
→ 12 룰 → 400 BT 기반 무한 확장 가능.

### 5.4 top-k 확장 (30 → 400)

도메인마다 해당 도메인이 등장하는 pair 중 top-1 을 보장 (400 slots), 나머지는 전역 top-k 로 채움. 이렇게 하면 "희귀 도메인이 결과에서 사라지는" 문제 해결.

### 5.5 Hasse DAG 후처리 (@fuse)

Pareto-dominant 관계를 DAG 로 그려 hub/bridge/cluster 지표 추가:
- **hubs**: degree top 20 (cross 공명 최고 도메인)
- **bridges**: betweenness top 50 (축 간 연결 담당)
- **clusters**: greedy modularity 16 그룹 (10축과 비교)

### 5.6 결과 스키마 (호환 + 확장)

```jsonc
{
  "generated_at": "2026-04-11T...",
  "source": "dse/domains/*.toml (v2 scan)",
  "schema_version": 2,
  "n_domains": 400,
  "pair_count": 100123,
  "high_confidence_count": 99587,
  "top_n": 400,
  "top_pairs": [ { "a": "...", "b": "...", "resonance": 0.92,
                    "kw_overlap": ..., "bt_overlap": ..., "formula_overlap": ...,
                    "cross_seed_overlap": ..., "pareto_proximity": ... }, ... ],
  "hubs":     [ { "domain": "chip", "degree": 287 }, ... ],
  "bridges":  [ { "domain": "eeg-consciousness-bridge", "betweenness": 0.128 }, ... ],
  "clusters": [ { "id": 0, "members": ["...","..."], "centroid": "fusion" }, ... ]
}
```

---

## 6. diff 요약 (v1 → v2, 구현 단위)

| 구성 | v1 | v2 | 증감 |
|------|----|----|------|
| 파서 | 매 pair마다 파싱 | `@memoize parse_toml` | 400회 → 400회 (캐시 보존) |
| feature 추출 | 매 호출마다 scan | `@memoize extract_features` | 재호출 0 |
| 쌍 점수 | 4 지표 | 5 지표 (bt/formula/seed/pareto 추가) | +3 신규 |
| synergy | 하드코드 12 | BT 인덱스 자동 매칭 | 하드코드 제거 (R2) |
| 병렬 | 없음 | `@parallel axis(10) × batch(16)` | 벽시간 1/10 |
| 출력 | top_pairs[30] | top_pairs[400] + hubs + bridges + clusters | +3 신규 섹션 |
| 스키마 | v1 | v2 (schema_version 필드 추가) | 후방 호환 |
| 등급 | 없음 | `evidence_grade` 승격 경로 | atlas.n6 자동 승격 |

---

## 7. 리스크 및 방어

| 리스크 | 방어 |
|--------|------|
| `bt_refs` 누락 TOML | 기본값 [] 처리, bt_overlap=0 |
| 400 TOML 중 일부 스키마 v1 잔존 | v2 파서가 Δ1~Δ5 필드 없으면 `energy_pareto=false`, `evidence_grade=7` 기본값 주입 |
| O(N²) 메모리 폭발 | 배치 16 단위로 스트리밍 처리, high_conf 만 상주 |
| 하드코드 재유입 | synergy 룰을 `n6shared/config/bt_weights.json` 외부화 (R2) |
| 중복 발견 누락 (R28) | 실행 종료 시 결과를 atlas.n6 에 자동 흡수 |

## 8. 검증 절차 (N63)

1. 400 TOML 로드 → schema_version 검증 PASS
2. v1 과 동일 입력(375 TOML)에서 pair_count ≥ 67,913 재현 → 회귀 0
3. v2 신규 입력(400 TOML)에서 pair_count ≥ 100,000 달성
4. high_confidence ≥ 99.5%
5. 상위 20 hubs 에 { chip, fusion, superconductor, battery, eeg-consciousness-bridge } 중 3+ 포함
6. `reports/discovery/cross_dse_v2_<date>.md` 자동 생성

## 9. 승격 완료 기준 (R4/R9)

- convergence/n6-architecture.json `CROSS_DSE` → v2 ossified 블록 추가:
  - status=PASS
  - value="400 TOML 교차 융합 완료 — 100K+ pair, 99.5%+ high_conf, hubs/bridges/clusters 포함"
  - threshold="100,000 pair + 400 TOML + Δ1~Δ5 스키마"

## 10. 연결 문서

- `./dse_400_expansion_plan.md`    — 신규 78 도메인 확장 계획
- `./energy_pareto_sweep_plan.md`  — 파레토 재측정 파이프라인
- `./cross_dse_fusion_v2.hexa`     — 구현 초안
- 상위: `../../CLAUDE.md` + `../../INDEX.json`
