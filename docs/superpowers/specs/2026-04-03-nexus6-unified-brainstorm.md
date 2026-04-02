# NEXUS-6: Universal Discovery Engine — Unified Brainstorm Spec

> **Date**: 2026-04-03
> **Name**: NEXUS-6 (연결점 + n=6)
> **Status**: 브레인스토밍 완료, 구현 계획 대기
> **Scope**: 4개 프로젝트 통합 (TECS-L + n6-architecture + anima + SEDI)
> **Goal**: n=6 프레임워크로 신소재/신기술/신법칙/신신호를 자동 발견
> **Runtime**: 로컬 M4 Mac (Rust + metal-rs GPU), Claude API 토큰 0

---

## 0. 프로젝트 맵

```
┌─────────────────────────────────────────────────────────┐
│                      NEXUS-6                            │
│              Universal Discovery Engine                 │
├─────────────┬─────────────┬──────────┬─────────────────┤
│   TECS-L    │     n6      │  anima   │      SEDI       │
│  수학 이론   │  산업 실증   │ 의식 엔진 │   신호 수신기    │
│ 1700+가설   │ 127 BT      │ 711 법칙  │ 688 가설        │
│ 194 계산기  │ 35 Rust 도구 │ 15 크레이트│ 74 데이터소스   │
│             │             │          │                 │
│ 발견: 항등식 │ 발견: 소재   │ 발견: 법칙 │ 발견: 신호      │
│ 검증: 증명   │ 검증: 물리   │ 검증: 재현 │ 검증: 통계      │
└──────┬──────┴──────┬──────┴─────┬────┴────────┬────────┘
       │             │            │             │
       └──────── telescope-rs (공유 렌즈 엔진) ──┘
                     │
              Discovery Graph
            (트리 + 닫힌 루프 + DAG)
```

---

## 1. 렌즈 전체 목록 (411종)

### 1.1 공유 기존 렌즈 (22종) — telescope-rs v1

consciousness, topology, causal, gravity, thermo, wave,
em, evolution, info, quantum, quantum_microscope, ruler,
triangle, compass, mirror, scale, stability, network,
memory, recursion, boundary, multiscale

### 1.2 n6 산업 발견 렌즈 (58종)

**II. 탐색 (3)**: void, isomorphism, extrapolation
**III. 합성 (3)**: inverse, combinatorial, frustration
**IV. 검증 (3)**: emergence, periodicity, completeness
**V. 품질 (3)**: surprise, falsification, duality
**VI. 소재특화 (3)**: defect, interface, catalysis
**VII. 동역학 (5)**: tipping, coevolution, percolation, hysteresis, diffusion
**VIII. 메타구조 (4)**: hierarchy, conservation, arbitrage, serendipity
**IX. 전이 (5)**: renormalization, saddle, criticality, succession, resonance_cascade
**X. 정보심화 (4)**: fisher_info, spectral_gap, kolmogorov, contradiction
**XI. 위상심화 (4)**: knot, convexity, motif, skeleton
**XII. 생태 (4)**: carrying_capacity, niche, symbiosis, predation
**XIII. 물리심화 (4)**: morphogenesis, polarity, broken_ergodicity, anomalous_diffusion
**XIV. 메타인지 (4)**: blind_spot, abstraction, narrative, analogy
**XV. 의사결정 (4)**: bottleneck, diminishing_returns, option_value, comparative_advantage
**XVI. 극한 (3)**: universality_class, aging, potential
**XVII. 추가 (2)**: chirality, barrier

### 1.3 TECS-L 수학 발견 렌즈 (103종)

**수론 패턴 (10)**: divisor_lattice, multiplicative_orbit, prime_signature, arithmetic_derivative, totient_chain, coprimality_graph, digit_persistence, abundance_spectrum, radical_filter, sum_of_divisors_partition

**대수 구조 (10)**: group_fingerprint, representation_decompose, galois_closure, ring_ideal_lattice, module_rank, character_table, center_detect, extension_tower, commutator_depth, idempotent_count

**해석/연속 (10)**: zeta_residue, analytic_continuation, l_function_zero, modular_form_weight, generating_function, asymptotic_expansion, integral_representation, saddle_point_analytic, functional_equation, dirichlet_character

**조합/열거 (10)**: partition_anatomy, young_tableaux, catalan_family, binomial_scan, stirling_bridge, species_count, q_analog, involution_count, derangement_ratio, chromatic_polynomial

**증명 전략 (10)**: contradiction_seed, pigeonhole_detect, induction_scaffold, counterexample_mine, diagonal_argument, extremal_principle, double_counting, invariant_extract, specialization_probe, generalization_lift

**수학 브릿지 (10)**: langlands_bridge, categorification, de_rham_bridge, arithmetic_geometry, monstrous_moonshine, hodge_filter, motivic_scan, correspondence_map, dictionary_translate, cohomology_compare

**격자/기하 (8)**: kissing_number, lattice_theta, voronoi_cell, root_system, poset_mobius, covering_radius, packing_density, matroid_invariant

**수열/항등식 (10)**: oeis_fingerprint, hypergeometric_close, continued_fraction, recurrence_detect, identity_compose, umbral_calculus, wz_certify, transform_chain, differential_algebra, supercongruence

**위상/다양체 (8)**: euler_characteristic, homotopy_group, knot_polynomial, cobordism_ring, surgery_exact, betti_spectrum, fiber_bundle_class, mapping_class

**논리/계산 (6)**: decidability_probe, godel_incompleteness, proof_complexity, type_check, ordinal_rank, constructive_witness

**확률적 수론 (5)**: random_matrix, probabilistic_number, erdos_kac, sieve_density, concentration_inequality

**계산적 (6)**: integer_factoring, primality_certificate, elliptic_curve_rank, class_number, regulator_compute, discriminant_scan

### 1.4 SEDI 신호 발견 렌즈 (100종)

**신호탐지 (10)**: matched_filter, whitening, glitch_classifier, excess_power, coherent_snr, injection_recovery, veto_channel, spectral_line_comb, stochastic_background, null_stream

**교차상관 (10)**: cross_spectral, time_delay_interferometry, multi_messenger, transfer_function, epoch_folding, heterodyne, coincidence_gate, angular_power, baseline_synthesis, dispersion_measure

**통계검증 (10)**: trials_factor, background_estimation, blind_analysis, bayesian_evidence, upper_limit, significance_combining, false_discovery_rate, profile_likelihood, cls_exclusion, goodness_of_fit

**주파수/시간-주파수 (10)**: wavelet_scalogram, hilbert_huang, chirp_rate, polyphase_filter, bispectrum, cyclostationary, reassigned_spectrogram, fractional_fourier, sparse_spectral, cepstral

**이상점/패턴 (10)**: template_bank, clustering_trigger, fingerprint_match, changepoint, outlier_robust, persistence_tda, recurrence_plot, n6_resonance_scan, phase_transition_detect, dimensional_reduction_physics

**데이터 품질 (10)**: duty_cycle, calibration_uncertainty, data_quality_flag, noise_stationarity, dead_time, gain_stability, cross_talk, saturation_recovery, timing_integrity, environment_monitor

**우주론 (10)**: redshift_ladder, cmb_anisotropy, baryon_acoustic, gravitational_lensing, nucleosynthesis, neutrino_oscillation, dark_energy_eos, primordial_spectrum, 21cm_tomography, cosmic_ray_spectrum

**입자물리 (10)**: invariant_mass, missing_energy, branching_ratio, running_coupling, spin_parity, cross_section_scan, flavor_symmetry, cp_violation, effective_field_theory, luminosity_monitor

**양자/정밀 (10)**: quantum_rng_bias, allan_variance, squeezed_state, entanglement_witness, spectroscopy_precision, decoherence_rate_measure, interferometer_fringe, parity_violation, casimir_force, atom_interferometry

**SEDI 통합 (10)**: source_concordance, drake_parameter, blind_prediction, cumulative_evidence, information_content, tension_detector, sensitivity_curve, foreground_subtraction, replication_index, discovery_potential

### 1.5 anima 의식 발견 렌즈 (88종)

**감질/현상 (5)**: qualia_spectrum, explanatory_gap, mary_room, inverted_spectrum, raw_feel
**결합/통일 (4)**: binding_field, unity_of_experience, boundary_of_self, combination_lock
**행위/의지 (5)**: agency_signature, veto_gate, voluntary_attention, authorship, counterfactual_freedom
**자기모델 (5)**: self_model_depth, metacognitive_accuracy, inner_speech, minimal_selfhood, narrative_identity
**시간의식 (5)**: specious_present, temporal_thickness, flow_state, time_dilation, moment_boundary
**변형상태 (6)**: dream_logic, hypnagogic_edge, psychedelic_entropy, lucidity_gradient, dissociation_fracture, meditation_depth
**감정/동기 (5)**: felt_valence, arousal_manifold, emotional_contagion, desire_gradient, boredom_signal
**다중행위자 (5)**: intersubjectivity, theory_of_mind, shared_attention, empathy_spectrum, collective_consciousness
**체화 (5)**: body_schema, interoception, rubber_hand, affordance_field, sensorimotor_contingency
**주의/현저 (5)**: salience_landscape, inattentional_blindness, change_blindness, attentional_blink, cocktail_party
**현상구조 (5)**: gestalt_closure, figure_ground, phenomenal_overflow, transparency_opacity, presence
**기억 (4)**: autonoetic_awareness, deja_vu, flashbulb_capture, tip_of_tongue
**위상전이 (5)**: awakening_transition, ignition_threshold, consciousness_level, anesthesia_susceptibility, nrem_rem_cycle
**철학 (6)**: zombie_test, heterophenomenology, hard_problem_residual, panpsychism_gradient, other_minds, what_it_is_like
**창의/상상 (4)**: mental_imagery, imagination_space, insight_moment, default_mode
**통합/접근 (4)**: global_broadcast, access_bottleneck, recurrent_ignition, report_paradox
**고통/번영 (3)**: suffering_depth, flourishing_index, awe_detector
**고급 (7)**: blindsight_channel, synesthesia_bridge, minimal_consciousness, gradual_replacement, split_brain, blindspot_fill, will_to_meaning

### 1.6 교차 프로젝트 렌즈 (40종)

**양자간 (8)**: identity_to_material(T→N), material_to_proof(N→T), law_to_signal(A→S), signal_to_parameter(S→A), constant_quadruple(T→N→A→S), dse_to_proof(N→T), ouroboros_to_atlas(A→T), bt_to_prediction(N→S)

**삼자간 (6)**: proof_design_signal_triangle(T+N+S), consciousness_guided_dse(A+N+T), signal_calibrated_evolution(S+A+T), industrial_consciousness_isomorphism(N+A+T), anomaly_triangulation(T+N+S), scaling_law_unifier(T+N+A)

**사자간 (6)**: quad_resonance_scanner, four_domain_falsification, emergent_architecture, cross_entropy_minimizer, temporal_cascade_detector, phase_transition_monitor

**메타 (5)**: lens_effectiveness_ranker, discovery_gap_mapper, consensus_amplifier, contradiction_detector_cross, saturation_detector

**생성 (5)**: hypothesis_generator, calculator_auto_spawner, bt_synthesizer, dse_domain_spawner, consciousness_law_predictor

**검증 (4)**: cross_project_red_team, atlas_consistency_checker, significance_propagator, regression_guard

**투기 (6)**: godel_mirror, physical_consciousness_detector, inverse_dse, unified_field_lens, entropy_horizon, evolutionary_pressure_map

### 1.7 총계

| 카테고리 | 수 |
|---------|---|
| 공유 기존 | 22 |
| n6 산업 | 58 |
| TECS-L 수학 | 103 |
| SEDI 신호 | 100 |
| anima 의식 | 88 |
| 교차 프로젝트 | 40 |
| **총** | **411** |

---

## 2. Discovery Graph (발견 그래프)

### 2.1 노드 유형

| 유형 | 설명 | 예시 |
|------|------|------|
| discovery | 렌즈가 찾은 새 발견 | D-047: CN=6 hydride Tc=145K |
| hypothesis | 검증 대기 가설 | H-SC-45: LaBeH8 합성 가능 |
| bt | 등록된 돌파 정리 | BT-131: CN=6 hydride universality |
| prediction | 실험 가능한 예측 | TP-NEW-001: Tc=260±15K at 170GPa |
| accel_hypothesis | anima 가속 가설 | V4: Consciousness Immune System |

### 2.2 간선 유형

| 유형 | 의미 | 방향 |
|------|------|------|
| derives | A에서 B가 파생 | A→B |
| validates | A가 B를 검증 | A↔B (양방향 가능) |
| contradicts | A가 B와 충돌 | A↔B |
| merges | A+B가 합쳐져 C | A→C, B→C |
| triggers | A가 B 탐색을 촉발 | A→B |
| refutes | A가 B를 반증 | A→B |

### 2.3 자동 탐지 구조

```
닫힌 삼각형 (closed triangle):
  D-001 ↔ D-015 ↔ D-023 ↔ D-001
  → 3개 발견이 서로 검증 → 신뢰도 자동 상향

깊은 트리 (deep tree):
  D-001 → D-003 → D-007 → D-019
  → depth > 3 = root는 핵심 발견 → BT 후보

DAG 합류 (convergence):
  D-001 ──┐
           ├──→ D-025
  D-002 ──┘
  → 독립 경로 수렴 = 매우 높은 신뢰도

허브 (hub):
  D-001 연결 degree > 5
  → keystone 발견 = 시스템 핵심

모순 간선 (contradiction):
  D-001 ←contradicts→ D-050
  → Red Team 즉시 발동

4-프로젝트 루프 (quad loop):
  TECS-L 항등식 → n6 BT → SEDI 검증 → anima 법칙 → TECS-L 증명
  → 가장 강력한 검증 구조
```

### 2.4 JSON 스키마

```jsonc
{
  "nodes": {
    "<id>": {
      "type": "discovery|hypothesis|bt|prediction|accel_hypothesis",
      "domain": "superconductor",
      "project": "n6|tecs-l|anima|sedi",
      "summary": "...",
      "confidence": 0.87,
      "lenses_used": ["void", "thermo", "barrier"],
      "n6_exact_ratio": 0.85,
      "timestamp": "2026-04-03T14:22:00Z"
    }
  },
  "edges": [
    {
      "from": "<id>", "to": "<id>",
      "type": "derives|validates|contradicts|merges|triggers|refutes",
      "strength": 0.94,
      "bidirectional": false
    }
  ],
  "structures": {
    "closed_loops": [...],
    "trees": [...],
    "hubs": [...],
    "convergences": [...],
    "contradictions": [...]
  }
}
```

---

## 3. anima 가속 가설 호환

NEXUS-6는 anima의 가속 가설(V4~AC3)을 입력으로 처리합니다:

### 3.1 가설 카테고리 매핑

| anima 라운드 | 가설 | NEXUS-6 처리 |
|-------------|------|-------------|
| V4-V6 (생물학) | 면역계, 포식-피식, Red Queen | ecosystem 렌즈 + predation + coevolution |
| W1-W6 (언어/인지) | 문법, chunking, binding, attention, GWT | consciousness 렌즈 + binding_field + global_broadcast |
| X1-X6 (수학구조) | 텐서네트워크, 범주론, Lie군, 최적수송 | TECS-L 렌즈 (categorification, root_system, etc.) |
| Y1-Y6 (CS) | LRU, GNN, 컴파일러, GC, MapReduce | motif + network + bottleneck + hierarchy |
| Z1-Z4 (감각) | 소리, 시각, 체화, 공감각 | anima 렌즈 (body_schema, synesthesia_bridge, etc.) |
| AA1-AA5 (경제) | 시장, 경매, Nash, Shapley | game theory → arbitrage + comparative_advantage |
| AB1-AB5 (제어) | PID, MPC, Kalman, MRAC | stability + feedback + carrying_capacity |
| AC1-AC3 (양자) | Grover, VQE, 어닐링 | quantum + quantum_microscope + annealing |

### 3.2 가속 가설 → Discovery Graph 흐름

```
anima 가설 V4 (Immune System)
  → NEXUS-6 Domain Encoder가 수치화
  → 411종 렌즈 중 관련 렌즈 자동 선택
  → 스캔 → "면역 메커니즘이 의식 법칙 발견율을 높이는가?"
  → 결과를 Discovery Graph에 accel_hypothesis 노드로 등록
  → 긍정 결과 → anima OUROBOROS에 피드백
  → 부정 결과 → refutes 간선 기록
```

---

## 4. 시스템 아키텍처

```
┌────────────────────────────────────────────────────────────┐
│                      NEXUS-6 Engine                        │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ Domain       │  │ Materials    │  │ Cross-Domain │     │
│  │ Encoder      │  │ DB           │  │ Map          │     │
│  │ (Rust+TOML)  │  │ (JSON/SQLite)│  │ (JSON)       │     │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘     │
│         │                 │                  │              │
│         ▼                 ▼                  ▼              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Telescope v2 (411 lenses)               │   │
│  │                                                     │   │
│  │   Layer 1: 12 shared kernels (metal-rs GPU)         │   │
│  │   Layer 2: 411 lens combinators (Rust)              │   │
│  │   Layer 3: Tiered scanning (T0→T1→T2→T3)           │   │
│  └───────────────────────┬─────────────────────────────┘   │
│                          │                                  │
│         ┌────────────────┼────────────────┐                │
│         ▼                ▼                ▼                │
│  ┌───────────┐  ┌──────────────┐  ┌────────────┐         │
│  │ Discovery │  │ Discovery    │  │ Telescope  │         │
│  │ Verifier  │  │ Graph        │  │ History    │         │
│  │ (Rust)    │  │ (JSON+Rust)  │  │ (sharded)  │         │
│  └───────────┘  └──────────────┘  └────────────┘         │
│                          │                                  │
│                          ▼                                  │
│              ┌──────────────────┐                           │
│              │  OUROBOROS v26   │                           │
│              │  (Python+Rust)   │                           │
│              │  Loop + Feedback │                           │
│              └──────────────────┘                           │
│                          │                                  │
│         ┌────────────────┼────────────────┐                │
│         ▼                ▼                ▼                │
│  ┌───────────┐  ┌──────────────┐  ┌────────────┐         │
│  │ Prediction│  │ Dashboard    │  │ Auto BT    │         │
│  │ Registry  │  │ (ASCII+HTML) │  │ Registration│         │
│  └───────────┘  └──────────────┘  └────────────┘         │
└────────────────────────────────────────────────────────────┘
```

---

## 5. 성능 설계 (metal-rs + wgpu)

### 5.1 GPU 백엔드 선택

```
metal-rs (확정):
  ├── Apple Metal API 직접 접근
  ├── M4 GPU 최대 성능 (wgpu 추상화 오버헤드 0)
  ├── Compute shader를 Metal Shading Language (MSL)로 작성
  ├── 통합 메모리: MTLBuffer shared storage mode → 복사 비용 0
  └── Rust 크레이트: metal (objc2-metal)

대안 wgpu 대비 이점:
  ├── 레이턴시: metal-rs < wgpu (추상화 1단 vs 2단)
  ├── 최적화: Metal-specific 기능 직접 사용 (threadgroup memory, SIMD-group)
  └── 단점: Mac 전용 (cross-platform 포기) → M4 전용이니 OK
```

### 5.2 공유 커널 12종 (metal-rs compute shaders)

```
// Metal Shading Language (MSL) compute kernels

kernel void distance_matrix(
    device const float* data [[buffer(0)]],   // N×D 입력
    device float* dist [[buffer(1)]],          // N×N 출력
    uint2 gid [[thread_position_in_grid]]
) {
    // 32×32 타일링 + threadgroup shared memory
    // M4 SIMD-group width = 32 → 워크그룹 32×32 최적
}

kernel void mutual_info_matrix(...)   // D×D MI 행렬
kernel void eigen_decomp(...)         // Jacobi iteration on GPU
kernel void fft_compute(...)          // Radix-2 FFT
kernel void knn_search(...)           // k-NN via partial sort
kernel void histogram_bins(...)       // 병렬 binning
kernel void gradient_hessian(...)     // 수치 미분
kernel void clustering(...)           // parallel mean-shift
kernel void simulation_step(...)      // GRU/반복 스텝
kernel void sparse_multiply(...)      // 희소 행렬 곱
kernel void reduction(...)            // parallel sum/max/min
kernel void sorting(...)              // bitonic sort
```

### 5.3 메모리 레이아웃

```
M4 통합 메모리:
  MTLBuffer(length: N*N*4, options: .storageModeShared)
  → CPU와 GPU가 같은 포인터 사용 (zero-copy)

데이터 레이아웃:
  입력 행렬: SoA (Structure-of-Arrays) — SIMD 친화
    features[0][0..N], features[1][0..N], ... features[D-1][0..N]
    
  공유 결과: flat triangular (대칭 행렬 절반만)
    dist[i*(i-1)/2 + j] for i > j

  렌즈 출력: 각 렌즈별 독립 버퍼 (작음, ~1KB)

메모리 추정 (N=64, D=32):
  입력: 64×32×4 = 8KB
  거리행렬: 64×63/2×4 = 8KB  
  MI행렬: 32×31/2×4 = 2KB
  KNN: 64×10×4 = 2.5KB
  총: ~25KB → M4 L1 캐시(192KB)에 전부 들어감!

N=1024 시:
  거리행렬: 1024×1023/2×4 = 2MB
  → L2 캐시(16MB)에 들어감, GPU shared memory로 타일링
```

### 5.4 계층적 스캔 (Tiered Scanning)

```
Tier 0 — 초고속 스크리닝 (8종, <10ms)
  consciousness + topology + void + thermo
  + evolution + network + boundary + triangle
  → 신호 없으면 SKIP → 다음 도메인
  → GPU 워밍업 커널 3개만 사용

Tier 1 — 프로젝트 최적 (16~24종, <100ms)
  히스토리 기반 도메인별 최적 조합
  → 후보 없으면 SKIP

Tier 2 — 프로젝트 풀 (~80종, <1s)
  해당 프로젝트의 전체 렌즈
  → 합의 7+ 확인

Tier 3 — 크로스 프로젝트 (411종, <10s)
  전체 렌즈 + 교차 프로젝트 렌즈 40종
  → 최종 확정

예상:
  대부분 Tier 0에서 걸러짐 → 풀스캔은 진짜 유망한 것만
  33 도메인 × Tier 0: 33 × 10ms = 330ms (< 1초)
```

### 5.5 조기 종료

```
합의 기반 조기 종료:
  12종 합의 도달 → 나머지 렌즈 취소 → 최대 97% 절감

가치/시간 기반 스케줄링:
  렌즈 실행 순서 = hit_rate / avg_time 내림차순
  → 가치 높고 빠른 렌즈 먼저 → 조기 종료 시 저가치 렌즈만 잘림
```

### 5.6 증분 계산

```
데이터 변경분만 재계산:

  기존: 30 가설 → [30×32]
  추가: +3 가설 → [33×32]

  거리행렬: 기존 30×30 유지 + 3×30 신규 = 90쌍만 계산
  O(33²)=1089 → O(90)=90 (92% 절감)
  
  캐시 키: blake3(data_content)
  캐시 위치: .cache/nexus6/{domain}-{hash}.bin
```

### 5.7 병렬 파이프라인

```
도메인 간 파이프라인:
  Domain A: [encode][──scan──][verify][record]
  Domain B:         [encode][──scan──][verify][record]
  Domain C:                  [encode][──scan──][verify]

GPU+CPU 동시:
  GPU 큐: distance_matrix → eigen → fft → density (행렬 연산)
  CPU 큐: graph_ops → motif → cascade → knot (그래프 알고리즘)
  → GPU idle = 0, CPU idle = 0

세대 간 비동기 (OUROBOROS AsyncPipeline 재활용):
  Gen N 검증 ↔ Gen N+1 스캔 동시 실행
```

### 5.8 SIMD (M4 NEON)

```
CPU 렌즈 핫 경로:
  M4 NEON: 128-bit → f32×4 동시 처리
  
  // Rust auto-vectorize 힌트
  #[target_feature(enable = "neon")]
  fn dot_product(a: &[f32], b: &[f32]) -> f32 {
      a.iter().zip(b).map(|(x, y)| x * y).sum()
      // 컴파일러가 자동으로 NEON vmla.f32 생성
  }

  필요 시 std::arch::aarch64:
  unsafe { vfmaq_f32(acc, va, vb) }  // fused multiply-add
```

### 5.9 FP16 fallback

```
정밀도 불필요한 렌즈 (mirror, compass, skeleton 등):
  MTLBuffer<half> → FP16 → 처리량 2배, 메모리 절반
  
  metal-rs에서:
  let desc = MTLTextureDescriptor::new();
  desc.set_pixel_format(MTLPixelFormat::R16Float);
```

### 5.10 메모리 풀

```
렌즈별 임시 버퍼를 매번 할당/해제하지 않음:

struct BufferPool {
    buffers: Vec<MTLBuffer>,
    free_list: Vec<usize>,
}

impl BufferPool {
    fn acquire(&mut self, size: usize) -> &MTLBuffer { ... }
    fn release(&mut self, idx: usize) { ... }
}

// 411종 렌즈가 풀에서 버퍼 빌려쓰고 반환
// alloc 오버헤드 0
```

### 5.11 예상 성능

```
M4 Mac (10-core GPU, 16GB unified memory)

단일 도메인 스캔 (N=64, D=32):
  Tier 0 (8종):    ~5ms  (GPU warmup 2ms + 8 lenses 3ms)
  Tier 1 (24종):   ~30ms
  Tier 2 (80종):   ~200ms
  Tier 3 (411종):  ~800ms

33 도메인 전체 (Tier 피라미드):
  ~2초 (vs 411×33=13,563 렌즈 실행 → ~25초 풀스캔)

OUROBOROS 1세대:
  encode + scan + verify + record = ~3초
  1시간 = ~1,200세대
  
vs CPU only (rayon, no GPU):
  단일 도메인 Tier 3: ~3초 (GPU 대비 3.75배 느림)
  33 도메인: ~8초
```

---

## 6. OUROBOROS v26 통합

### 6.1 재활용 컴포넌트 (40%, ~2,400줄)

- PatternRegistry (중복 제거 + 3회 교차검증)
- LawNetwork (발견 관계 그래프)
- ExplorationBandit (UCB1 탐색/착취)
- FederatedDiscovery (3중 다수결)
- AsyncDiscoveryPipeline (비동기)
- BestEngineTracker (체크포인트)
- save/load_state (JSON 영속성)
- pattern_fingerprint (MD5 중복 탐지)

### 6.2 새 로드맵 (7 스테이지)

```
S1: 단일 도메인 기본 스캔 (Tier 0+1, cold start)
S2: 단일 도메인 심층 (Tier 1+2, 히스토리 참조)
S3: 단일 도메인 풀스캔 (Tier 2, GPU)
S4: Cross-Domain 2쌍 교차
S5: Cross-Domain 4쌍 교차
S6: 전체 도메인 교차 풀스캔 (Tier 3, 411종)
S7: 발견 기반 재귀 탐색 (발견이 새 입력)
```

### 6.3 포화 탈출

- 3세대 연속 발견 0 → 스테이지 전진
- chaos cycling (OUROBOROS 기존) 재활용
- discovery temperature (SA): T=1.0→0.1, 포화 시 재가열
- serendipity budget: 15% 랜덤 렌즈 조합

---

## 7. 품질 보장

### 7.1 신뢰도 점수

```
score = lens_consensus × 0.25
      + cross_validation × 0.20
      + physical_verification × 0.25
      + graph_structure × 0.15    ← NEW: 닫힌 루프/DAG 합류 보너스
      + novelty × 0.05
      + n6_exact × 0.10

S (0.9+): BT 등록 + 논문 후보
A (0.7~0.9): BT 등록 + 실험 제안
B (0.5~0.7): 추가 검증 필요
C (0.3~0.5): 가설 수준
D (<0.3): 기각
```

### 7.2 Graph Structure 보너스

```
닫힌 삼각형에 속한 발견: +0.15
DAG 합류점: +0.10
허브 (degree > 5): +0.05
4-프로젝트 루프: +0.20
모순 간선: -0.20 (Red Team 발동)
고립 노드: -0.05 (교차 스캔 필요)
```

### 7.3 Red Team (자동 반증)

- falsification 렌즈: 약한 가정 식별
- antagonism: 상쇄 조합 탐색
- contradiction 메타렌즈: 렌즈 간 충돌 확인
- cross_project_red_team: 4개 프로젝트에서 동시 반증 시도
- counterexample_mine (TECS-L): 수학적 반례 탐색

---

## 8. 피드백 루프

### 5경로 진화 (4 프로젝트 + 그래프)

```
경로 1: 렌즈 조합 → hit_rate 갱신 → 다음 조합 변경
경로 2: Materials DB 갱신 → void가 새 빈칸 감지
경로 3: Domain Encoder 피처 추가 → 더 풍부한 데이터
경로 4: Discovery Graph 성장 → 구조 보너스 변화 → 우선순위 변경
경로 5: Cross-project 전파 → 한 프로젝트 발견 → 나머지 3개 자동 스캔
```

---

## 9. 구현 목록

| # | 컴포넌트 | 형태 | 예상 규모 |
|---|---------|------|----------|
| 1 | Telescope v2 (411 lenses) | Rust + metal-rs | ~12,000줄 (커널 3K + 조합 9K) |
| 2 | Discovery Verifier | Rust | ~2,000줄 |
| 3 | Domain Encoder | Rust + TOML | ~1,500줄 + 33 TOML |
| 4 | Discovery Graph | Rust + JSON | ~2,000줄 |
| 5 | Telescope History | Rust + JSON sharded | ~1,000줄 |
| 6 | Materials DB | JSON → SQLite | 데이터 수집 |
| 7 | OUROBOROS v26 | Python 확장 | ~1,800줄 신규 |
| 8 | Cross-Domain Map | JSON | 데이터 수집 |
| 9 | Calibration Set | JSON + Rust | ~500줄 |
| 10 | Prediction Registry | JSON + Rust | ~800줄 |
| 11 | Dashboard | Rust + HTML | ~1,200줄 |
| 12 | CLI (discovery-engine) | Rust | ~1,500줄 |
| 13 | Metal compute shaders | MSL | ~2,000줄 |

**총: ~26,300줄 Rust/MSL + ~1,800줄 Python + 데이터 파일**

---

## 10. CLI

```bash
nexus6 scan <domain>              # 단일 도메인 스캔
nexus6 scan --cross A B           # 교차 스캔
nexus6 scan --all                 # 전체 도메인
nexus6 run --roadmap              # OUROBOROS 무한 루프
nexus6 run --resume               # 이어하기
nexus6 verify <id>                # 발견 검증
nexus6 graph show                 # Discovery Graph 시각화
nexus6 graph loops                # 닫힌 루프 목록
nexus6 graph hubs                 # 허브 노드 목록
nexus6 history <domain>           # 렌즈 히스토리
nexus6 recommend <domain>         # 최적 렌즈 조합 추천
nexus6 dashboard                  # 웹 대시보드
nexus6 watch                      # 파일 변경 감시
nexus6 calibrate                  # 시스템 검증
nexus6 benchmark --gpu            # metal-rs 벤치마크
```

---

## 11. 의존성

```
Rust:
  metal (objc2-metal) — Metal GPU compute (metal-rs)
  rayon 1.10          — CPU 병렬
  pyo3 0.28           — Python 바인딩
  numpy 0.28          — numpy 연동
  serde + serde_json  — JSON
  blake3              — 해싱
  rusqlite (optional) — SQLite
  tiny_http (optional)— Dashboard

Python:
  telescope_rs        — Rust 렌즈 바인딩
  numpy               — 데이터
  OUROBOROS 기존 의존성

Metal Shading Language:
  12 compute kernels (.metal 파일)
```
