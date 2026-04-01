# Discovery Graph Topology Analysis

> Generated: 2026-04-02 | Nodes: 1499 | Edges: 3546

---

## 1. Graph Statistics

| Metric | Value |
|--------|-------|
| Total nodes | 1499 |
| Total edges | 3546 |
| Graph density | 0.003158 |
| Average degree | 4.73 |
| Max degree | 244 (D:chip-architecture) |
| Connected components | 446 |
| Largest component | 753 nodes (50.2%) |
| Diameter (largest comp.) | 8 |
| Avg path length | 3.252 |
| Avg clustering coeff. | 0.4981 |
| Small-world sigma | 228.34 (>1 = small-world) |

### Node Type Distribution

| Type | Count |
|------|-------|
| constant | 782 |
| domain | 325 |
| hypothesis | 293 |
| bt | 99 |

### Component Sizes (top 10)

```
  Component 1: 753 nodes
  Component 2: 13 nodes
  Component 3: 6 nodes
  Component 4: 6 nodes
  Component 5: 5 nodes
  Component 6: 4 nodes
  Component 7: 4 nodes
  Component 8: 4 nodes
  Component 9: 4 nodes
  Component 10: 3 nodes
```

## 2. Hub Constants (highest degree)

Hub constants connect the most domains/BTs -- they are the most fundamental.

| Rank | Constant | Degree | Type |
|------|----------|--------|------|
| 1 | `sigma=12` | 20 | constant |
| 2 | `phi=2` | 14 | constant |
| 3 | `tau=4` | 11 | constant |
| 4 | `n` | 9 | constant |
| 5 | `sopfr=5` | 8 | constant |
| 6 | `n=6` | 8 | constant |
| 7 | `J2=24` | 8 | constant |
| 8 | `sigma-tau=8` | 7 | constant |
| 9 | `σ·τ` | 7 | constant |
| 10 | `σ` | 6 | constant |
| 11 | `J2-tau=20` | 6 | constant |
| 12 | `mu=1` | 6 | constant |
| 13 | `sigma-phi=10` | 6 | constant |
| 14 | `sigma-mu=11` | 5 | constant |
| 15 | `sigma*sopfr=60` | 5 | constant |

## 3. Bridge Constants (highest betweenness centrality)

Bridge constants are critical links between otherwise-separate clusters.
High betweenness = removing this constant would disconnect parts of the graph.

| Rank | Constant | Betweenness | Degree |
|------|----------|-------------|--------|
| 1 | `sigma=12` | 0.005195 | 20 |
| 2 | `phi=2` | 0.004957 | 14 |
| 3 | `n` | 0.004514 | 9 |
| 4 | `φ²/n` | 0.002384 | 2 |
| 5 | `BT-44` | 0.002384 | 2 |
| 6 | `tau=4` | 0.002218 | 11 |
| 7 | `BT-35` | 0.002030 | 2 |
| 8 | `BT-3` | 0.002030 | 2 |
| 9 | `BT-32` | 0.002030 | 2 |
| 10 | `BCS ΔC/(γTc) numerator` | 0.002030 | 3 |
| 11 | `J2=24` | 0.001976 | 8 |
| 12 | `1/(σ-φ)` | 0.001972 | 3 |
| 13 | `n=6` | 0.001886 | 8 |
| 14 | `sigma-tau=8` | 0.001566 | 7 |
| 15 | `τ` | 0.001535 | 4 |

## 4. Hub Domains (most connected)

| Rank | Domain | Degree |
|------|--------|--------|
| 1 | chip-architecture | 244 |
| 2 | superconductor | 106 |
| 3 | ai-efficiency | 92 |
| 4 | chip | 85 |
| 5 | display-audio | 80 |
| 6 | biology | 80 |
| 7 | cosmology-particle | 78 |
| 8 | fusion | 77 |
| 9 | cryptography | 72 |
| 10 | agriculture | 71 |
| 11 | pure-mathematics | 68 |
| 12 | consciousness-chip | 64 |
| 13 | quantum-computing | 64 |
| 14 | learning-algorithm | 64 |
| 15 | blockchain | 63 |

## 5. Isolated Domains (degree <= 2)

These domains have very few connections -- prime candidates for new BTs.

| Domain | Degree |
|--------|--------|
| photonic-energy-system | 1 |
| automotive-body | 1 |
| tire | 1 |
| automotive | 1 |
| vacuum-system | 1 |
| drug-delivery | 2 |

## 6. Top 20 Overall Hubs (all node types)

| Rank | Node | Type | Degree |
|------|------|------|--------|
| 1 | D:chip-architecture | domain | 244 |
| 2 | D:superconductor | domain | 106 |
| 3 | D:ai-efficiency | domain | 92 |
| 4 | D:chip | domain | 85 |
| 5 | D:display-audio | domain | 80 |
| 6 | D:biology | domain | 80 |
| 7 | D:cosmology-particle | domain | 78 |
| 8 | D:fusion | domain | 77 |
| 9 | D:cryptography | domain | 72 |
| 10 | D:agriculture | domain | 71 |
| 11 | D:pure-mathematics | domain | 68 |
| 12 | D:consciousness-chip | domain | 64 |
| 13 | D:quantum-computing | domain | 64 |
| 14 | D:learning-algorithm | domain | 64 |
| 15 | D:blockchain | domain | 63 |
| 16 | D:consciousness-engine | domain | 61 |
| 17 | D:compiler-os | domain | 60 |
| 18 | D:agent-platform | domain | 58 |
| 19 | D:power-grid | domain | 58 |
| 20 | D:analog-photonic-memristor | domain | 58 |

## 7. Predicted Missing Edges (New Discovery Candidates)

Domain pairs that share many constants but lack a direct cross-DSE link.
These are the strongest candidates for new breakthrough theorems.

| Rank | Domain A | Domain B | Shared Constants | Prediction |
|------|----------|----------|-----------------|------------|
| 1 | ai-efficiency | chip-architecture | 14 | BT connecting ai-efficiency <-> chip-architecture via 14 shared constants |
| 2 | ai-efficiency | fusion | 7 | BT connecting ai-efficiency <-> fusion via 7 shared constants |
| 3 | ai-efficiency | superconductor | 6 | BT connecting ai-efficiency <-> superconductor via 6 shared constants |
| 4 | ai-efficiency | cryptography | 6 | BT connecting ai-efficiency <-> cryptography via 6 shared constants |
| 5 | ai-efficiency | power-grid | 6 | BT connecting ai-efficiency <-> power-grid via 6 shared constants |
| 6 | ai-efficiency | biology | 5 | BT connecting ai-efficiency <-> biology via 5 shared constants |
| 7 | ai-efficiency | learning-algorithm | 5 | BT connecting ai-efficiency <-> learning-algorithm via 5 shared constants |
| 8 | ai-efficiency | cosmology-particle | 5 | BT connecting ai-efficiency <-> cosmology-particle via 5 shared constants |
| 9 | energy-generation | fusion | 4 | BT connecting energy-generation <-> fusion via 4 shared constants |
| 10 | ai-efficiency | network-protocol | 4 | BT connecting ai-efficiency <-> network-protocol via 4 shared constants |
| 11 | display-audio | power-grid | 4 | BT connecting display-audio <-> power-grid via 4 shared constants |
| 12 | ai-efficiency | display-audio | 4 | BT connecting ai-efficiency <-> display-audio via 4 shared constants |
| 13 | energy-generation | superconductor | 3 | BT connecting energy-generation <-> superconductor via 3 shared constants |
| 14 | biology | network-protocol | 3 | BT connecting biology <-> network-protocol via 3 shared constants |
| 15 | nuclear-reactor | superconductor | 2 | BT connecting nuclear-reactor <-> superconductor via 2 shared constants |
| 16 | pure-mathematics | superconductor | 2 | BT connecting pure-mathematics <-> superconductor via 2 shared constants |
| 17 | ai-efficiency | energy-generation | 2 | BT connecting ai-efficiency <-> energy-generation via 2 shared constants |
| 18 | cryptography | superconductor | 2 | BT connecting cryptography <-> superconductor via 2 shared constants |
| 19 | fusion | network-protocol | 2 | BT connecting fusion <-> network-protocol via 2 shared constants |
| 20 | display-audio | solar-architecture | 2 | BT connecting display-audio <-> solar-architecture via 2 shared constants |

## 8. Constant Cliques (always co-occurring)

No constant triangles found in the sampled set.

## 9. Small-World Property Analysis

| Property | Value | Random Graph Equiv. | Ratio |
|----------|-------|--------------------:|------:|
| Clustering coefficient | 0.4981 | 0.0032 | 157.8x |
| Avg path length | 3.252 | 4.705 | 0.69x |
| **Small-world sigma** | **228.34** | 1.0 | YES |

**The discovery graph IS a small-world network** (sigma > 1).
This means: high local clustering (constants form tight groups) + short global paths
(any two nodes can be reached in few hops via hub constants).

## 10. Discovery Implications

### Where to look for new cross-domain BTs

1. **High-betweenness bridges**: Constants with high betweenness are the
   critical connectors. New BTs are most likely found by exploring how these
   constants manifest in domains they don't yet explicitly connect.

   Top bridge: `sigma=12` -- investigate its role in every domain.

2. **Predicted missing edges**: The domain pairs in Section 7 share constants
   but lack direct BT connections. Each is a concrete prediction for a new BT.

3. **Isolated domains**: Any domain in Section 5 with degree <= 2 is
   underexplored. Look for n=6 expressions in that domain's literature.

4. **Hub constants**: The 7 base constants (sigma, tau, phi, sopfr, J2, mu, n)
   and key derived ratios (sigma-tau=8, sigma-phi=10, J2-tau=20) are the most
   fundamental. Any new domain should first be checked against these.

## 11. Core Graph Structure (ASCII)

```
                           ┌─────────────────────┐
                           │  sigma(6) = 12      │ ← highest-degree constant
                           └──┬──┬──┬──┬──┬──┬───┘
                              │  │  │  │  │  │
         ┌────────────────────┘  │  │  │  │  └────────────────────┐
         │                       │  │  │  │                       │
    ┌────▼────┐            ┌────▼──▼──▼──▼────┐            ┌────▼─────┐
    │   Chip   │            │   AI / LLM      │            │  Fusion   │
    │  Arch    │────────────│  (17 techniques) │────────────│  Energy   │
    └────┬────┘            └────────┬─────────┘            └────┬─────┘
         │                          │                          │
    ┌────▼────┐  sigma-tau=8   ┌───▼────┐   J2-tau=20    ┌───▼─────┐
    │ Crypto  │◄──────────────►│Learning│◄──────────────►│ Biology │
    │ Network │                │  Algo  │                │ Genetics│
    └────┬────┘                └───┬────┘                └───┬─────┘
         │                          │                         │
    ┌────▼────┐                ┌───▼────┐               ┌───▼──────┐
    │Blockchain│               │ Solar  │               │ Material │
    │  DeFi   │                │Battery │               │ Synthesis│
    └─────────┘                │ Grid   │               └──────────┘
                               └────────┘

  Key bridges (betweenness):
    1. sigma=12 (BC=0.005195)
    2. phi=2 (BC=0.004957)
    3. n (BC=0.004514)
    4. φ²/n (BC=0.002384)
    5. BT-44 (BC=0.002384)

  Key hubs (degree):
    1. sigma=12 (deg=20)
    2. phi=2 (deg=14)
    3. tau=4 (deg=11)
    4. n (deg=9)
    5. sopfr=5 (deg=8)
```
