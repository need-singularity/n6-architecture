# cross_dse_fusion v2 실측 결과 — 2026-04-11

- 실행 스크립트: experiments/dse/cross_dse_fusion_v2_run.hexa (v2 설계 초안의 실행본)
- 입력 도메인: /Users/ghost/Dev/n6-architecture/nexus/origins/universal-dse/domains
- 도메인 수: 480
- 전체 쌍 수(이론상): 114960
- high_conf 쌍: 93327
- 임계값: resonance ≥ 0.18 AND (bt+formula) ≥ 0.0

## v1 vs v2 비교

| 지표 | v1 | v2 (실측) |
|------|-----|------|
| 도메인 수 | 375 | 480 |
| pair_total | 70125 | 114960 |
| high_conf | 67883 | 93327 |
| 공명 지표 수 | 4 | 5 |

## 지표별 기여도 (high_conf pair 평균)

| 지표 | 가중치 | 평균 overlap | 가중 기여 |
|------|--------|--------------|-----------|
| kw_overlap | 0.35 | 0.152 | 0.0532 |
| bt_overlap (신규) | 0.25 | 0.0052 | 0.0013 |
| formula_overlap (신규) | 0.2 | 0.5907 | 0.1181 |
| cross_seed_overlap | 0.1 | 0.0156 | 0.0016 |
| pareto_proximity | 0.1 | 0.8863 | 0.0886 |
| **resonance 평균** |  |  | **0.2628** |

## 상위 20 Pair

1. **space-engineering x space**  res=0.7448  kw=0.8167  bt=0.75  fm=0.7143
2. **network-protocol x network**  res=0.7333  kw=0.8333  bt=0.6  fm=0.7778
3. **cpu-microarchitecture x soc-integration**  res=0.67  kw=0.3667  bt=1.0  fm=1.0
4. **audio x display**  res=0.6598  kw=0.5667  bt=0.5556  fm=1.0
5. **insect-farming x mycology-fungus**  res=0.6534  kw=0.3  bt=1.0  fm=1.0
6. **biology-systems x biophysics**  res=0.6385  kw=0.3167  bt=1.0  fm=0.875
7. **dram-memory x memory-architecture**  res=0.635  kw=0.5  bt=1.0  fm=0.4286
8. **lsp-ide x test-framework**  res=0.6305  kw=0.25  bt=1.0  fm=0.8571
9. **linguistics x test-framework**  res=0.6209  kw=0.2667  bt=1.0  fm=0.8571
10. **dna-folding x evolutionary-biology**  res=0.6103  kw=0.3  bt=1.0  fm=0.75
11. **cpu-microarchitecture x fpga-architecture**  res=0.6051  kw=0.1667  bt=1.0  fm=1.0
12. **haptic-feedback x sports-biomechanics**  res=0.6049  kw=0.1333  bt=1.0  fm=1.0
13. **audio x earphone**  res=0.6028  kw=0.45  bt=0.5556  fm=1.0
14. **crypto x cryptography**  res=0.6013  kw=0.65  bt=0.6667  fm=0.4545
15. **ceramic-engineering x refractory-material**  res=0.5962  kw=0.4833  bt=1.0  fm=0.4
16. **cpu-microarchitecture x eda-design-automation**  res=0.5961  kw=0.2  bt=1.0  fm=0.8333
17. **eda-design-automation x fpga-architecture**  res=0.5916  kw=0.2333  bt=1.0  fm=0.8333
18. **fpga-architecture x soc-integration**  res=0.5903  kw=0.1333  bt=1.0  fm=1.0
19. **linguistics x lsp-ide**  res=0.5827  kw=0.2333  bt=1.0  fm=0.7143
20. **eda-design-automation x soc-integration**  res=0.5781  kw=0.2  bt=1.0  fm=0.8333

## 허브 (top 20, degree)

1. baking-patisserie  degree=479
2. bamboo-craft  degree=479
3. butchery-meat  degree=479
4. calligraphy-ink  degree=479
5. cheese-dairy  degree=479
6. chocolate-confectionery  degree=479
7. dye-pigment  degree=479
8. essential-oil-distillation  degree=479
9. herbal-medicine  degree=479
10. honey-apiculture  degree=479
11. lacquerware  degree=479
12. leather-tanning  degree=479
13. rice-cultivation  degree=479
14. silk-sericulture  degree=479
15. tea-fermentation  degree=479
16. wine-enology  degree=479
17. perfumery  degree=476
18. eeg-bci  degree=474
19. software-design  degree=472
20. blockchain  degree=471

## 브리지 (top 50, betweenness 근사)

1. butchery-meat  betweenness=0.3552
2. dye-pigment  betweenness=0.3543
3. leather-tanning  betweenness=0.3523
4. calligraphy-ink  betweenness=0.3521
5. honey-apiculture  betweenness=0.3448
6. cheese-dairy  betweenness=0.3427
7. tea-fermentation  betweenness=0.3426
8. lacquerware  betweenness=0.3426
9. baking-patisserie  betweenness=0.3425
10. wine-enology  betweenness=0.3422
11. herbal-medicine  betweenness=0.3369
12. bamboo-craft  betweenness=0.3352
13. essential-oil-distillation  betweenness=0.3347
14. rice-cultivation  betweenness=0.3341
15. chocolate-confectionery  betweenness=0.3338
16. silk-sericulture  betweenness=0.3266
17. horticulture  betweenness=0.2921
18. coffee-roasting  betweenness=0.2905
19. refractory-material  betweenness=0.2873
20. embodied-consciousness  betweenness=0.287
21. perfumery  betweenness=0.2869
22. turbine-generator  betweenness=0.2865
23. space-engineering  betweenness=0.2861
24. biology-systems  betweenness=0.2857
25. biophysics  betweenness=0.2852
26. debugger  betweenness=0.2849
27. mycology-fungus  betweenness=0.2847
28. fiber-optic-wearable  betweenness=0.2847
29. insect-farming  betweenness=0.2846
30. superalloy-turbine  betweenness=0.2845
31. fuel-injection  betweenness=0.2837
32. holographic-display  betweenness=0.2837
33. smart-textile  betweenness=0.2835
34. dielectric-material  betweenness=0.2833
35. e-ink-display  betweenness=0.2832
36. hydrogel  betweenness=0.2831
37. eda-design-automation  betweenness=0.283
38. veterinary-medicine  betweenness=0.2828
39. optical-glass  betweenness=0.2825
40. ferroelectric-material  betweenness=0.2825
41. gpu-lang  betweenness=0.2823
42. liquid-crystal  betweenness=0.2822
43. pkg-manager  betweenness=0.2822
44. bridge-engineering  betweenness=0.2821
45. piezoelectric-material  betweenness=0.2819
46. pipe-fitting  betweenness=0.2819
47. endoscopy-system  betweenness=0.2818
48. fastener-bolt  betweenness=0.2817
49. thin-film-coating  betweenness=0.2817
50. neuroscience  betweenness=0.2817

## R28 atlas.n6 흡수

- 상위 100 pair 를 cross_dse_v2 헤더 아래 @R [7] 로 append.
- 등급 승격은 후속 파레토 스윕에서 [10*] 로 재검증.

## 메모: v2 설계 vs 실측 차이

- 설계 초안은 Δ1~Δ5 스키마(bt_refs/cross_seeds/n6_formula/evidence_grade/energy_pareto) 기반이나, 현 380 TOML 중 1개(hexa-ios.toml)만 bt_refs 를 직접 포함. 나머지는 주석 헤더에서 `BT-NNN` 토큰을 프록시로 추출.
- n6_formula/cross_seeds 는 라인 본문에서 상수 심볼(`sigma=12`, `phi=2`, `n=6`, `tau=4`, `sopfr=5`, `j2=24`) 매칭으로 대체.
- pareto_frontier_400.json 미존재 → pareto_pts 를 TOML 후보 scoring(n6/perf/power/cost/energy_proxy) 평균 5-벡터로 대체.
- 실 Δ1~Δ5 스키마 적용 시 (78 신규 TOML + 400 기존) bt/formula 평균 기여가 크게 상승 예상.
