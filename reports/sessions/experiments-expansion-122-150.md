# experiments — 122 → 150 (28 )

: 2026-04-11
: experiments-expansion-122-150
: Claude Opus 4.6 (1M context)
: — experiments/ (122 → 150, 28 .hexa )

## 

- base: _results.jsonl 115 (ai-efficiency 33 + anomaly 43 + cross 6 + structural 45 )
- .hexa (STUB , `nexus verify --hexa`)
- : 28 → 150 

## ( )

- R1 (HEXA-FIRST) : .py , .hexa 
- : /print 
- R14 (SSOT) : _results.jsonl SSOT pending , JSON 
- : + fn main() println
- CLAUDE.md atlas.n6 SSOT 

## 28 

| | | | | |---|---|---|---| | ai-efficiency | 10 | ai-technique-gap | 16 AI (YaRN/Ring/MLA/Mamba2/Griffin/Jamba/MoD/GShard/Speculative/Medusa) | | anomaly | 10 | bt380-new + chip-design | BT-372~380 9 + 1 | | cross | 4 | cross-domain | physics×ai, life×compute, energy×materials, space×infra | | structural | 4 | structural-proof | 6 , E6 , 6D RG , Discovery Graph v10 | | | 28 | — | — |

## 

### ai-efficiency (10) — AI 

1. `experiment_yarn_rope_scaling_n6.hexa` — YaRN s=6
2. `experiment_ring_attention_n6.hexa` — R=6 
3. `experiment_deepseek_mla_n6.hexa` — d_c ∈ {6,12} 
4. `experiment_mamba2_ssm_n6.hexa` — N=24 
5. `experiment_griffin_rglru_n6.hexa` — H=576 BPC 
6. `experiment_jamba_hybrid_n6.hexa` — 1:6:1
7. `experiment_mixture_of_depths_n6.hexa` — capacity 1/6 
8. `experiment_gshard_switch_n6.hexa` — E=12, top-2 
9. `experiment_speculative_decoding_n6.hexa` — γ=6 speedup 
10. `experiment_medusa_heads_n6.hexa` — M=6 

### anomaly (10) — BT-380 + 

1. `verify_bt372_geology.hexa` — 6 (PREM )
2. `verify_bt373_meteorology.hexa` — 6 (Hadley/Ferrel/Polar × 2)
3. `verify_bt374_cryosphere.hexa` — 6 
4. `verify_bt375_ocean.hexa` — 6 
5. `verify_bt376_atmospheric_chem.hexa` — 6 
6. `verify_bt377_curvature.hexa` — Ricci 6 
7. `verify_bt378_warp.hexa` — 6 
8. `verify_bt379_extra_dim.hexa` — CY3 6 
9. `verify_bt380_meta.hexa` — BT 6 
10. `verify_chip_cache_hierarchy.hexa` — 6 

### cross (4) — n=6 

1. `experiment_physics_ai_n6_bridge.hexa` — ↔ AI 
2. `experiment_bio_chip_cascade.hexa` — 6 ↔ 6 
3. `experiment_energy_materials_fusion.hexa` — 6 + 6 
4. `experiment_space_infra_topology.hexa` — 6 + 6 

### structural (4) — n=6 

1. `experiment_hexagonal_tiling_optimality.hexa` — Honeycomb conjecture 
2. `experiment_e6_lattice_packing.hexa` — E6 d=6 
3. `experiment_6d_renormalization_flow.hexa` — 6D β 
4. `experiment_discovery_graph_v10.hexa` — v9 → v10 BT-380 = 6 

## _results.jsonl 

 115 , 28 pending append.

 :

```json
{"name": "...", "category": "...", "file": "...", "status": "pending", "batch": "expansion-122-150", "priority": "ai-technique-gap | bt380-new | chip-design | cross-domain | structural-proof"}
```

 (`batch`, `priority`) . .

 : 115 + 28 = 143 ( )

## 

1. (nexus verify --hexa )
2. `status: "pending"` → `"passed"` / `"failed"` / `"inconclusive"` 
3. passed `n6shared/convergence/n6-architecture.json` ossified 
4. BT-372~380 `atlas.n6` [7] → [10*] 

## 

- R5 (SSOT) : `reports/sessions/` , 
- atlas.n6 ( )
- techniques/ .hexa ( experiments )
- .py 0, .hexa (R1 )
- 100% 
