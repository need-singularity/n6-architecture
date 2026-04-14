# DSE-P1-3 Cross-DSE Matrix 112 x 10 요약

- 생성일: 2026-04-14
- 기법: 112 (7 섹션 비례)
- 도메인 카테: 10 (cognitive, compute, culture, energy, infra, life, materials, physics, sf-ufo, space)
- 총 셀: 1,120
- 재현 seed: 42 (해시 기반 결정적)
- 휴리스틱 공식 (재현용):
  - `fit = clamp(base_affinity[sec][dom] + bonus_10star[dom] + hash_variation, 0, 1)`
  - `bonus_10star = min(0.08, atlas.n6 [10*] / total * 0.15)` (도메인별)
  - `hash_variation = (abs(hash("{sec}:{tech}:{dom}")) % 80 / 1000) - 0.04`  (-0.04~+0.04)
  - `alien_idx = round(fit * 12)` (0~12, n6 sigma 상한)
  - `lens_candidates`: NEXUS/shared/lenses/ 도메인 키워드 버킷 → tech:dom 해시 seed 로 3개 결정적 샘플
- 기법 선택: 레지스트리 `_registry.json` 7 섹션 알파벳 정렬 후 quota 상위 N 결정적 컷
- 소스: `techniques/_registry.json` + `$NEXUS/shared/n6/atlas.n6` + `$NEXUS/shared/lenses/*.hexa`

## 섹션별 quota

| 섹션 | 선택 | 원본 |
|---|---:|---:|
| attention | 14 | 27 |
| moe | 6 | 13 |
| optim | 37 | 75 |
| sparse | 5 | 10 |
| graph | 4 | 7 |
| compress | 9 | 18 |
| arch | 37 | 74 |
| **합계** | **112** | **224** |

## 섹션 평균 fit

| 섹션 | avg fit |
|---|---:|
| attention | 0.736 |
| moe | 0.731 |
| optim | 0.774 |
| sparse | 0.759 |
| graph | 0.796 |
| compress | 0.756 |
| arch | 0.778 |

## 도메인 평균 fit

| 도메인 | avg fit |
|---|---:|
| cognitive | 0.808 |
| compute | 0.922 |
| culture | 0.620 |
| energy | 0.759 |
| infra | 0.838 |
| life | 0.759 |
| materials | 0.752 |
| physics | 0.818 |
| sf-ufo | 0.642 |
| space | 0.748 |

## 섹션 x 카테 평균 fit 행렬

| 섹션 \\ 도메인 | cognitive | compute | culture | energy | infra | life | materials | physics | sf-ufo | space |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| attention | 0.97 | 0.99 | 0.68 | 0.56 | 0.79 | 0.77 | 0.59 | 0.72 | 0.65 | 0.63 |
| moe | 0.80 | 0.99 | 0.62 | 0.70 | 0.93 | 0.69 | 0.58 | 0.64 | 0.61 | 0.75 |
| optim | 0.78 | 0.90 | 0.58 | 0.83 | 0.85 | 0.76 | 0.80 | 0.87 | 0.63 | 0.73 |
| sparse | 0.67 | 0.82 | 0.54 | 0.94 | 0.78 | 0.62 | 0.97 | 0.80 | 0.58 | 0.87 |
| graph | 0.89 | 0.76 | 0.94 | 0.72 | 0.99 | 0.86 | 0.66 | 0.78 | 0.57 | 0.79 |
| compress | 0.66 | 0.97 | 0.53 | 0.90 | 0.89 | 0.63 | 0.83 | 0.76 | 0.58 | 0.80 |
| arch | 0.82 | 0.93 | 0.63 | 0.72 | 0.80 | 0.80 | 0.76 | 0.85 | 0.68 | 0.78 |

## 최고 fit Top 20

| 순위 | 섹션 | 기법 | 도메인 | fit | alien | 렌즈 |
|---:|---|---|---|---:|---:|---|
| 1 | attention | alibi_attention | compute | 1.000 | 12 | accel_architecture_search, cross_emergent_architecture, tecs_transform_chain |
| 2 | attention | differential_transformer | compute | 1.000 | 12 | frontier_architectural_proportion, frontier_neural_architecture, archaeology_lens |
| 3 | attention | flash_attention | compute | 1.000 | 12 | tecs_regulator_compute, cross_consciousness_guided_dse, cross_lens_hierarchy |
| 4 | attention | flash_attention_v3 | compute | 1.000 | 12 | chip_compute_coupling_lens, cross_dse_domain_spawner, accel_near_memory_compute |
| 5 | attention | gqa_grouping | compute | 1.000 | 12 | frontier_architectural_proportion, molecular_transform_lens, frontier_wavelet_transform |
| 6 | attention | memory_attention | compute | 1.000 | 12 | combo_blowup_energy_architecture, accel_dataflow_architecture, archaeology_lens |
| 7 | graph | gat_heads | infra | 1.000 | 12 | supply_chain_risk_lens, frontier_plasticity_consolidation, accel_moe_routing_lens |
| 8 | graph | graph_transformer | infra | 1.000 | 12 | logistics_supply_lens, anima_transparency_opacity, n6_carrying_capacity |
| 9 | moe | jamba_hybrid | compute | 1.000 | 12 | chip_architecture_lens, cross_emergent_architecture, chip_compute_coupling_lens |
| 10 | moe | jordan_leech_moe | compute | 1.000 | 12 | cross_consciousness_guided_dse, cross_lens_hierarchy, cross_emergent_architecture |
| 11 | graph | gcn_depth | infra | 0.999 | 12 | accel_island_biogeography, frontier_plasticity_consolidation, frontier_knowledge_graph |
| 12 | moe | deepseek_moe | compute | 0.999 | 12 | cross_emergent_architecture, sedi_signal_search_dse, n6_hierarchy |
| 13 | compress | low_rank_factorization | compute | 0.998 | 12 | accel_compute_graph, tecs_regulator_compute, accel_architecture_search |
| 14 | moe | gshard_switch | compute | 0.998 | 12 | brainwire_eeg_n6_dse, accel_near_memory_compute, tecs_regulator_compute |
| 15 | attention | cross_attention | compute | 0.997 | 12 | accel_genetic_algorithm, cross_lens_hierarchy, combo_blowup_energy_architecture |
| 16 | sparse | radical_norm | materials | 0.996 | 12 | quantum_xray_crystallography, cross_identity_to_material, tecs_ring_ideal_lattice |
| 17 | attention | additive_attention | cognitive | 0.995 | 12 | cross_industrial_consciousness_isomorphism, anima_theory_of_mind, anima_consciousness_level |
| 18 | compress | activation_quantization | compute | 0.995 | 12 | transformer_anatomy_lens, brainwire_eeg_n6_dse, cross_emergent_architecture |
| 19 | attention | cross_attention | cognitive | 0.994 | 12 | anima_attentional_blink, consciousness_lens, accel_neuromorphic_chip |
| 20 | moe | expert_choice_routing | compute | 0.993 | 12 | accel_dataflow_architecture, archaeology_lens, frontier_neural_architecture |

## atlas.n6 신호 (카테별 노드 수 / [10*])

| 도메인 | 총 노드 | [10*]+ | 비율 |
|---|---:|---:|---:|
| cognitive | 294 | 259 | 88.1% |
| compute | 745 | 743 | 99.7% |
| culture | 603 | 591 | 98.0% |
| energy | 394 | 389 | 98.7% |
| infra | 617 | 614 | 99.5% |
| life | 516 | 513 | 99.4% |
| materials | 447 | 445 | 99.6% |
| physics | 775 | 373 | 48.1% |
| sf-ufo | 20 | 12 | 60.0% |
| space | 201 | 196 | 97.5% |

## 산출물

- JSON: `experiments/dse/cross_matrix_112x10.json`
- 요약: `experiments/dse/cross_matrix_112x10_summary.md` (본 문서)
- 로드맵: `$NEXUS/shared/roadmaps/n6-architecture.json` DSE-P1-3 done
