# experiments 축 3차 확장 리포트 — 150 → 200 (50 신규)

일자: 2026-04-11
세션: experiments-expansion-150-200
담당: Claude Opus 4.6 (1M context)
근거: 사용자 지시 — experiments/ 축 3차 확장 (150 → 200, 50종 .hexa 신규)

## 배경

- 1차 base 115 → 2차 143 (expansion-122-150 에이전트 #7 완료, ai-efficiency 10 + anomaly 10 + cross 4 + structural 4 = 28 신규)
- 3차 목표 50 신규 → 193 도달 (파일 기준 200 +) 을 통해 검증 공백 추가 보완
- 전부 .hexa STUB, 본격 측정 로직은 후속 `nexus verify --hexa` 사이클 (R18 미니멀)

## 생성 원칙 (준수 규칙)

- R1 HEXA-FIRST : .py 제로, .hexa 만 작성
- 한글 의무 : 파일 주석·print 100% 한글
- R14 SSOT : `_results.jsonl` 단일 SSOT 에 pending 행 append, 별도 JSON 미생성
- R18 미니멀 : fn main() 스텁 + 메타 주석만, 본격 구현은 후속
- 기존 스텁 패턴 준수 : 파일 상단 메타 주석 + println 3행
- n6shared/config/absolute_rules.json + n6shared/rules/n6-architecture.json 기반 검증

## 50 신규 실험 분포

| 카테고리 | 개수 | 우선순위 키 | 목적 |
|---|---|---|---|
| anomaly (BT-381~400) | 20 | bt400-new | 암흑물질/암흑에너지/양자중력/의식/정보이론/복잡계/생태/사회물리/경제물리/네트워크 신규 돌파 검증 |
| chip-verify | 10 | chip-design | Xn6 ISA 24 ops, RTL 6단, NPU 6x6, 벡터 12레인, 캐시 6레벨, NoC 6포트, RF 24엔트리, 이슈 6, 분기예측 6, 전원게이팅 6상태 |
| ai-efficiency (SOTA) | 6 | sota-bench | Mamba-2 2건 + Hyena 2건 + RWKV-6 2건 |
| lens-verify | 10 | lens-verify | 인과발견/위상데이터/스펙트럴/매니폴드/베이지안/최적운송/어트랙터/지속호몰/정보병목/변분 |
| meta | 4 | meta-ops | atlas 승격 자동화, convergence 갱신, DSE v2 회귀, MC v9 교차 배치 |
| 합계 | 50 | — | — |

## 카테고리별 상세

### anomaly (20) — BT-381~400 프론티어 신규 돌파

BT-381 WIMP 래더 / BT-382 액시온 창 / BT-383 Omega_Lambda=24/35 / BT-384 LQG 스핀폼 /
BT-385 AdS6/CFT5 / BT-386 IIT Phi_max / BT-387 GWT 방송 / BT-388 Shannon 대통합 /
BT-389 Kolmogorov / BT-390 SOC 지수 / BT-391 BA 네트워크 / BT-392 영양단계 /
BT-393 Dunbar / BT-394 Zipf-Pareto / BT-395 시장 꼬리 / BT-396 May 다양성 /
BT-397 도시 스케일링 / BT-398 Fisher / BT-399 게임이론 / BT-400 10 프론티어 메타

### chip-verify (10) — 신규 카테고리 (Xn6 칩 설계 검증)

1. `verify_xn6_isa_24ops.hexa` — 24 명령어(J2) 완전성
2. `verify_rtl_6stage_pipeline.hexa` — 6단 파이프라인 PPA
3. `verify_npu_systolic_6x6.hexa` — 6x6 시스톨릭 활용률
4. `verify_xn6_vector_width.hexa` — 12 레인 파레토
5. `verify_dse_cache_6level.hexa` — 6 레벨 캐시 DSE
6. `verify_xn6_interconnect_6port.hexa` — 6 포트 NoC
7. `verify_xn6_regfile_24_entries.hexa` — 24 RF 엔트리
8. `verify_xn6_issue_width_6.hexa` — 6 이슈 슈퍼스칼라
9. `verify_xn6_branch_predictor.hexa` — 6 비트 히스토리 BHT
10. `verify_xn6_power_gating_6.hexa` — 6 상태 전원 관리

### ai-efficiency (6) — SOTA 벤치 3종×2

1. `experiment_mamba2_ssm_bench.hexa` — Mamba-2 LongBench/MMLU
2. `experiment_mamba2_scaling_n6.hexa` — Mamba-2 Chinchilla n=6 계열
3. `experiment_hyena_long_conv.hexa` — Hyena 차수 O=6 스캔
4. `experiment_hyena_bench_sota.hexa` — Hyena LRA/PILE
5. `experiment_rwkv6_time_mix_n6.hexa` — RWKV-6 time-mix 6 계수
6. `experiment_rwkv6_bench_sota.hexa` — RWKV-6 OpenLLM

### lens-verify (10) — 신규 카테고리 (2차 렌즈 56종 중 대표 검증)

1. `verify_causal_discovery_lens.hexa` — CausalDiscoveryLens
2. `verify_topological_data_lens.hexa` — TopologicalDataLens
3. `verify_spectral_graph_lens.hexa` — SpectralGraphLens
4. `verify_manifold_learning_lens.hexa` — ManifoldLearningLens
5. `verify_bayesian_inference_lens.hexa` — BayesianInferenceLens
6. `verify_optimal_transport_lens.hexa` — OptimalTransportLens
7. `verify_attractor_basin_lens.hexa` — AttractorBasinLens
8. `verify_persistence_homology_lens.hexa` — PersistenceHomologyLens
9. `verify_information_bottleneck_lens.hexa` — InformationBottleneckLens
10. `verify_variational_inference_lens.hexa` — VariationalInferenceLens

### meta (4) — 신규 카테고리 (메타 운영)

1. `meta_atlas_promotion_automation.hexa` — atlas.n6 [7]→[10*] 자동 승격
2. `meta_convergence_refresh.hexa` — convergence/n6-architecture.json 동기
3. `meta_dse_v2_regression.hexa` — DSE v2 파레토 회귀
4. `meta_mc_v9_cross_batch.hexa` — MC v9 50 배치 신뢰도 추정

## _results.jsonl 업데이트

기존 143 행 유지, 말미에 50 pending 행 append.

각 행 스키마:

```json
{"name": "...", "category": "...", "file": "...", "status": "pending", "batch": "expansion-150-200", "priority": "bt400-new | chip-design | sota-bench | lens-verify | meta-ops"}
```

총 행 수: 143 + 50 = 193 (검증 완료, `wc -l` 로 확인).

## 신규 카테고리 디렉토리 추가

- `experiments/chip-verify/` + CLAUDE.md
- `experiments/lens-verify/` + CLAUDE.md
- `experiments/meta/` + CLAUDE.md

기존 ai-efficiency/anomaly 디렉토리에는 파일 단순 추가. INDEX.json 의 `experiments.subs` 업데이트는 후속 배치에서 일괄 반영 (본 세션은 .hexa + _results.jsonl 한정).

## 향후 수행 경로

1. 50 스텁 중 우선순위 상위 (bt400-new → chip-design → sota-bench) 순차 본격 포팅
2. 결과 확보 시 `status: pending` → `passed` / `failed` / `inconclusive`
3. passed BT-381~400 항목은 `n6shared/convergence/n6-architecture.json` ossified 승격 + `atlas.n6` [7]→[10*] 편집 후보
4. chip-verify passed → `nexus rtl` 파이프라인 통과 증거로 채택
5. `meta_atlas_promotion_automation.hexa` 자동 승격 스크립트를 본격 구현하여 루프 폐쇄

## 제약 및 주석

- R5 SSOT : 본 리포트는 `reports/sessions/` 에만 존재, 복제 금지
- atlas.n6 직접 편집 없음 (검증 완료 후 승격 단계에서만)
- techniques/ 실제 .hexa 변경 없음 (본 작업은 experiments 축 한정)
- .py 작성 0건, 전부 .hexa (R1 통과)
- 한글 주석 100% 통과
- INDEX.json `experiments.subs` ["ai-efficiency","structural","cross","anomaly"] → chip-verify/lens-verify/meta 추가 반영 필요 (후속 태스크)
