# E6 Signal Graph Homology / Betti -- 2026-04-15

> Input: `/Users/ghost/Dev/nexus/shared/n6/atlas.signals.n6`
> Edge definition: SIG-* references in cross_repo/cross/refs/predicts
> b0/b1 on undirected 1-skeleton only. Higher Betti not computed.
> Seven-major-problems resolution held at 0/7.

## 1. Graph Statistics

- Nodes V = 390 | Directed edges = 126 | Unique undirected E = 97

## 2. Betti Numbers

- b0 (connected components) = 311
- b1 (independent loops) = E - V + b0 = 97 - 390 + 311 = 18
- Homology class: MILD-LOOP

## 3. Degree Distribution

- max degree: 18 | mean: 0.65 | median: 0 | isolated: 303 (77.69%)

### Top-5 hub

| node | degree |
|------|-------:|
| SIG-META-001 | 18 |
| SIG-N6-BERN-001 | 9 |
| SIG-DFS-204 | 9 |
| SIG-BERN-CAND-K3 | 8 |
| SIG-7R-401 | 7 |

## 4. Interpretation

- b0 > 1: multi-component graph, not a single cluster
- b1 > 0: independent loops present
- isolated_share high: many signals are self-contained entries

## 5. Honest Limits

- 1-skeleton only; b2 not computed
- Edge definition is SIG-* body matching; semantic cross-links may be missed
- Isolated nodes self-contained under this measurement; not a weakness
- Seven-major-problems held at 0/7
