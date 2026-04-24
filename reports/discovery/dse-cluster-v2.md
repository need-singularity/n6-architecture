# DSE Cross-Resonance Fusion Cluster v2

> Generated: `scripts/dse_cluster_v2.py` | Input: `pair_scores.jsonl` (1225 pairs)
> Threshold: S > 0.5 | Algorithm: Union-Find connected components

## 1. Summary

- Input pairs: **1225** (all domain pairs)
- High-resonance pairs (S>0.5): **35**
- Fusion cluster count: **7**
- Maximum component size: **13** domains
- Candidate BT clusters: **1**

## 2. Cluster table

| # | Representative domain | Size | Edges | Avg S | Max S | Common n=6 formulas | BT candidate |
|--:|----------------------|----:|----:|------:|------:|--------------------|:------------:|
| 1 | `consciousness-chip` | 13 | 27 | 0.531 | 0.711 | (main: hexagonal structure (hex)) | * |
| 2 | `photonic-energy` | 3 | 2 | 0.545 | 0.563 | (main: sigma=12, n=6) | . |
| 3 | `snn-spiking` | 3 | 2 | 0.532 | 0.553 | (main: ) | . |
| 4 | `corpus-generation` | 2 | 1 | 0.674 | 0.674 | (main: ) | . |
| 5 | `cosmology-particle` | 2 | 1 | 0.662 | 0.662 | (main: ) | . |
| 6 | `embedded-lang` | 2 | 1 | 0.608 | 0.608 | (main: ) | . |
| 7 | `ocean-engineering` | 2 | 1 | 0.504 | 0.504 | (main: ) | . |

## 3. Cluster details (Top 10)

### Cluster 1 — `consciousness-chip` (size 13)

- Domains: `consciousness-chip`, `consciousness-comm`, `consciousness-rng`, `consciousness-scaling`, `consciousness-substrate`, `consciousness-training`, `consciousness-transplant`, `consciousness-wasm`, `eeg-consciousness-bridge`, `embodied-consciousness`, `hivemind-collective`, `multimodal-consciousness`, `sedi-universe`
- Top formula frequency: hexagonal structure (hex)(5)
- Mean/max resonance: 0.531 / 0.711
- BT candidate: * YES

### Cluster 2 — `photonic-energy` (size 3)

- Domains: `analog-photonic-memristor`, `photonic-energy`, `semiconductor-packaging`
- Top formula frequency: sigma=12(1), n=6(1), Diamond (C Z=6)(1), PUE=sigma/(sigma-phi)=1.2(1), Graphite (C Z=6)(1)
- Mean/max resonance: 0.545 / 0.563
- BT candidate: no

### Cluster 3 — `snn-spiking` (size 3)

- Domains: `hexad-architecture`, `neuromorphic-loihi`, `snn-spiking`
- Top formula frequency: (no formula mapping)
- Mean/max resonance: 0.532 / 0.553
- BT candidate: no

### Cluster 4 — `corpus-generation` (size 2)

- Domains: `corpus-generation`, `tokenizer-design`
- Top formula frequency: (no formula mapping)
- Mean/max resonance: 0.674 / 0.674
- BT candidate: no

### Cluster 5 — `cosmology-particle` (size 2)

- Domains: `cosmology-particle`, `pure-mathematics`
- Top formula frequency: (no formula mapping)
- Mean/max resonance: 0.662 / 0.662
- BT candidate: no

### Cluster 6 — `embedded-lang` (size 2)

- Domains: `embedded-lang`, `gpu-lang`
- Top formula frequency: (no formula mapping)
- Mean/max resonance: 0.608 / 0.608
- BT candidate: no

### Cluster 7 — `ocean-engineering` (size 2)

- Domains: `ocean-engineering`, `water-treatment`
- Top formula frequency: (no formula mapping)
- Mean/max resonance: 0.504 / 0.504
- BT candidate: no

## 4. ASCII graph — cluster size distribution

```
Cluster    Representative domain       Size
C01  consciousness-chip       ████████████████████████████████████████ 13 *
C02  photonic-energy          █████████ 3
C03  snn-spiking              █████████ 3
C04  corpus-generation        ██████ 2
C05  cosmology-particle       ██████ 2
C06  embedded-lang            ██████ 2
C07  ocean-engineering        ██████ 2
```

## 5. ASCII connection structure (Top 3 cluster core edges)

```
[C01] consciousness-chip  (n=13)
   consciousness-scaling            --0.71-- consciousness-training
   consciousness-comm               --0.61-- consciousness-chip
   consciousness-chip               --0.61-- consciousness-scaling
   consciousness-wasm               --0.58-- consciousness-comm
   consciousness-substrate          --0.58-- consciousness-transplant
   consciousness-wasm               --0.57-- consciousness-chip
   consciousness-chip               --0.52-- multimodal-consciousness
   sedi-universe                    --0.51-- consciousness-transplant

[C02] photonic-energy  (n=3)
   analog-photonic-memristor        --0.56-- photonic-energy
   photonic-energy                  --0.53-- semiconductor-packaging

[C03] snn-spiking  (n=3)
   neuromorphic-loihi               --0.55-- snn-spiking
   hexad-architecture               --0.51-- snn-spiking

```

## 6. Conclusion (draft)

- **7** fusion clusters derived, maximum **13**-domain component
- **1** candidate BT (shared formula >=3 or size >=5) as a draft target
- Each BT candidate shares the same n=6 formulas across many domains -> a draft target for cross-BT promotion review
