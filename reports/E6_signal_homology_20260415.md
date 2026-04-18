# E6 Signal Graph Homology / Betti — 2026-04-15

> 입력: `/Users/ghost/Dev/nexus/shared/n6/atlas.signals.n6`
> Edge 정의: signal 본문의 cross_repo/cross/refs/predicts 에 등장하는 SIG-* 참조
> Undirected graph 1-skeleton 에서 b0/b1 만 계산. 고차 Betti 미산출.
> 7대 난제 해결 0/7 유지.

## 1. Graph 통계

- 노드 수 V = 390
- 디렉티드 edge 등장 카운트 = 126
- 유일 undirected edge 수 E = 97

## 2. Betti 수

- **b0** (connected components) = 311
- **b1** (independent loops) = E - V + b0 = 97 - 390 + 311 = 18

- Homology class: **MILD-LOOP**

## 3. Degree 분포

- max degree: 18
- mean degree: 0.65
- median degree: 0
- isolated nodes: 303 (77.69%)

### Top-5 hub

| node | degree |
|------|-------:|
| SIG-META-001 | 18 |
| SIG-N6-BERN-001 | 9 |
| SIG-DFS-204 | 9 |
| SIG-BERN-CAND-K3 | 8 |
| SIG-7R-401 | 7 |

## 4. 해석

- **b0 > 1**: signal 그래프가 단일 cluster 가 아닌 다중 component.
- **b1 > 0**: 독립 루프 존재 — cross_repo 가 단순 트리 아닌 cycle 포함.
- **isolated_share 높음 (>40%)**: 다수 신호가 외부 참조 없는 sole entry.

## 5. 정직 한계

- 1-skeleton 만 — 2차 Betti b2 (3D void) 미산출.
- Edge 정의가 SIG-* 본문 매칭 — 의미적 cross-link 누락 가능.
- isolated 노드는 본 측정의 'self-contained' 신호; 약점 아님.
- 7대 난제 0/7 유지.