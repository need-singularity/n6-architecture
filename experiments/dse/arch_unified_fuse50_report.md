# DSE-P4-2 — arch_unified 4-mode fuse 시뮬 (상위 50셀)

**작성일**: 2026-04-14  
**엔진**: `engine/arch_unified.hexa` (INDUSTRIAL / QUANTUM / SELFORG / ADAPTIVE)  
**소스**: `cross_matrix_v3_full.json` — 86,240 셀 중 fit 내림차순 상위 50  
**seed**: 42  
**규칙**: fit 대역별 2 모드 선정 → 가중합 hybrid_score

## 매핑 규칙 (fit → mode pair)

주 분기는 fit 대역, 서브 분기는 `hash(tech|domain) % 3` 결정적 해시로 mode_b 를 3 갈래 분산 (tie 다양성 확보).

| fit 대역 | mode_a | mode_b 후보 (sub=0/1/2) | 의도 |
|---|---|---|---|
| > 0.9 | INDUSTRIAL (w=7) | QUANTUM / ADAPTIVE / SELFORG | 확정 주축 + 다양탐색 |
| 0.8~0.9 | QUANTUM (w=4) | ADAPTIVE / SELFORG / INDUSTRIAL | 불확실 진화 |
| 0.7~0.8 | SELFORG (w=5) | ADAPTIVE / INDUSTRIAL / QUANTUM | 정적창발 보강 |
| ≤ 0.7 | SELFORG (w=5) | INDUSTRIAL / QUANTUM / ADAPTIVE | 저확신 보수혼합 |

## 모드 쌍 분포 (상위 50셀)

| 모드 쌍 | 빈도 | 비율 |
|---|---|---|
| INDUSTRIAL+QUANTUM | 26 | 52% |
| INDUSTRIAL+ADAPTIVE | 24 | 48% |

## 상위 10 hybrid_score

| 순위 | cell_idx | tech | domain | fit | 모드쌍 | hybrid |
|---|---|---|---|---|---|---|
| 29 | 84 | `additive_attention` | compute | 1.0000 | INDUSTRIAL+ADAPTIVE | 869 |
| 43 | 120 | `additive_attention` | compute | 1.0000 | INDUSTRIAL+ADAPTIVE | 869 |
| 47 | 129 | `additive_attention` | compute | 1.0000 | INDUSTRIAL+ADAPTIVE | 869 |
| 34 | 94 | `additive_attention` | compute | 1.0000 | INDUSTRIAL+ADAPTIVE | 862 |
| 39 | 112 | `additive_attention` | compute | 1.0000 | INDUSTRIAL+ADAPTIVE | 862 |
| 44 | 121 | `additive_attention` | compute | 1.0000 | INDUSTRIAL+ADAPTIVE | 862 |
| 35 | 95 | `additive_attention` | compute | 1.0000 | INDUSTRIAL+ADAPTIVE | 855 |
| 37 | 104 | `additive_attention` | compute | 1.0000 | INDUSTRIAL+ADAPTIVE | 855 |
| 48 | 131 | `additive_attention` | compute | 1.0000 | INDUSTRIAL+ADAPTIVE | 855 |
| 30 | 87 | `additive_attention` | compute | 1.0000 | INDUSTRIAL+ADAPTIVE | 848 |

## 50 셀 전체 표

| # | cell_idx | tech | domain | source | fit | alien | mode_a(wa) | mode_b(wb) | score_a | score_b | hybrid |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 4 | `additive_attention` | cognitive | `attention::engine` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 910 | 385 | 752 |
| 2 | 5 | `additive_attention` | cognitive | `attention::theory` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 900 | 370 | 741 |
| 3 | 7 | `additive_attention` | cognitive | `attention::reports` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 880 | 340 | 718 |
| 4 | 10 | `additive_attention` | cognitive | `attention::rules` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 850 | 295 | 683 |
| 5 | 12 | `additive_attention` | cognitive | `moe::papers` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 920 | 400 | 764 |
| 6 | 13 | `additive_attention` | cognitive | `moe::techniques` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 910 | 385 | 752 |
| 7 | 15 | `additive_attention` | cognitive | `moe::engine` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 890 | 355 | 729 |
| 8 | 17 | `additive_attention` | cognitive | `moe::domains` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 870 | 325 | 706 |
| 9 | 18 | `additive_attention` | cognitive | `moe::reports` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 860 | 310 | 695 |
| 10 | 19 | `additive_attention` | cognitive | `moe::bridge` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 850 | 295 | 683 |
| 11 | 23 | `additive_attention` | cognitive | `optim::papers` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 900 | 370 | 741 |
| 12 | 25 | `additive_attention` | cognitive | `optim::experiments` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 880 | 340 | 718 |
| 13 | 27 | `additive_attention` | cognitive | `optim::theory` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 860 | 310 | 695 |
| 14 | 29 | `additive_attention` | cognitive | `optim::reports` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 840 | 280 | 672 |
| 15 | 35 | `additive_attention` | cognitive | `sparse::techniques` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 870 | 325 | 706 |
| 16 | 37 | `additive_attention` | cognitive | `sparse::engine` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 850 | 295 | 683 |
| 17 | 38 | `additive_attention` | cognitive | `sparse::theory` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 840 | 280 | 672 |
| 18 | 40 | `additive_attention` | cognitive | `sparse::reports` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 910 | 385 | 752 |
| 19 | 43 | `additive_attention` | cognitive | `sparse::rules` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 880 | 340 | 718 |
| 20 | 44 | `additive_attention` | cognitive | `graph::atlas` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 870 | 325 | 706 |
| 21 | 47 | `additive_attention` | cognitive | `graph::experiments` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 840 | 280 | 672 |
| 22 | 55 | `additive_attention` | cognitive | `compress::atlas` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 850 | 295 | 683 |
| 23 | 56 | `additive_attention` | cognitive | `compress::papers` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 840 | 280 | 672 |
| 24 | 58 | `additive_attention` | cognitive | `compress::experiments` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 910 | 385 | 752 |
| 25 | 64 | `additive_attention` | cognitive | `compress::products` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 850 | 295 | 683 |
| 26 | 73 | `additive_attention` | cognitive | `arch::reports` | 1.0000 | 12 | INDUSTRIAL(7) | QUANTUM(3) | 850 | 295 | 683 |
| 27 | 81 | `additive_attention` | compute | `attention::engine` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 860 | 750 | 827 |
| 28 | 82 | `additive_attention` | compute | `attention::theory` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 850 | 750 | 820 |
| 29 | 84 | `additive_attention` | compute | `attention::reports` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 920 | 750 | 869 |
| 30 | 87 | `additive_attention` | compute | `attention::rules` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 890 | 750 | 848 |
| 31 | 89 | `additive_attention` | compute | `moe::papers` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 870 | 750 | 834 |
| 32 | 90 | `additive_attention` | compute | `moe::techniques` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 860 | 750 | 827 |
| 33 | 92 | `additive_attention` | compute | `moe::engine` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 840 | 700 | 798 |
| 34 | 94 | `additive_attention` | compute | `moe::domains` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 910 | 750 | 862 |
| 35 | 95 | `additive_attention` | compute | `moe::reports` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 900 | 750 | 855 |
| 36 | 96 | `additive_attention` | compute | `moe::bridge` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 890 | 750 | 848 |
| 37 | 104 | `additive_attention` | compute | `optim::theory` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 900 | 750 | 855 |
| 38 | 106 | `additive_attention` | compute | `optim::reports` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 880 | 750 | 841 |
| 39 | 112 | `additive_attention` | compute | `sparse::techniques` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 910 | 750 | 862 |
| 40 | 114 | `additive_attention` | compute | `sparse::engine` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 890 | 750 | 848 |
| 41 | 115 | `additive_attention` | compute | `sparse::theory` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 880 | 750 | 841 |
| 42 | 117 | `additive_attention` | compute | `sparse::reports` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 860 | 750 | 827 |
| 43 | 120 | `additive_attention` | compute | `sparse::rules` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 920 | 750 | 869 |
| 44 | 121 | `additive_attention` | compute | `graph::atlas` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 910 | 750 | 862 |
| 45 | 124 | `additive_attention` | compute | `graph::experiments` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 880 | 750 | 841 |
| 46 | 126 | `additive_attention` | compute | `graph::theory` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 860 | 750 | 827 |
| 47 | 129 | `additive_attention` | compute | `graph::bridge` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 920 | 750 | 869 |
| 48 | 131 | `additive_attention` | compute | `graph::rules` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 900 | 750 | 855 |
| 49 | 132 | `additive_attention` | compute | `compress::atlas` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 890 | 750 | 848 |
| 50 | 133 | `additive_attention` | compute | `compress::papers` | 1.0000 | 12 | INDUSTRIAL(7) | ADAPTIVE(3) | 880 | 750 | 841 |

## hybrid_score 통계

- min: 672
- max: 869
- mean: 774.1

## 검증 노트

- 계산은 `arch_unified.hexa` 의 `run_pipeline`/`fuse_modes`/`mode_weight` 를 Python으로 1:1 포팅 (정수 연산만).
- n=6 상수(σ=12, τ=4, φ=2, sopfr=5) 외 임의 값 없음.
- cross_matrix_v3_full.json 원본 미수정 (read-only).
- seed=42, sort stable → 재현 가능.
