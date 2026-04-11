use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for 8 frontier discovery lenses.
///
/// These lenses cover previously uncovered scientific domains:
/// acoustics, fluid dynamics, game theory, linguistics,
/// irreversible thermodynamics, polymer physics, resonance, and convexity.
pub fn frontier_lens_entries() -> Vec<LensEntry> {
    vec![
        LensEntry {
            name: "acoustic".into(),
            category: LensCategory::Extended,
            description: "Detect sound-wave patterns: periodicity, harmonics, tonality (sigma=12 semitones)".into(),
            domain_affinity: vec!["audio".into(), "music".into(), "physics".into(), "signal".into(), "display".into()],
            complementary: vec!["wave".into(), "periodicity".into(), "resonance".into()],
        },
        LensEntry {
            name: "fluid_dynamics".into(),
            category: LensCategory::Extended,
            description: "Measure turbulence via Reynolds-like number and Kolmogorov 5/3 exponent (sopfr/n/phi)".into(),
            domain_affinity: vec!["physics".into(), "engineering".into(), "plasma".into(), "atmosphere".into()],
            complementary: vec!["chaos".into(), "multiscale".into(), "scale".into()],
        },
        LensEntry {
            name: "game_theory".into(),
            category: LensCategory::Extended,
            description: "Detect Nash equilibrium proximity, cooperation index, Egyptian allocation (1/2+1/3+1/6=1)".into(),
            domain_affinity: vec!["economics".into(), "social".into(), "ai".into(), "biology".into()],
            complementary: vec!["mirror".into(), "stability".into(), "network".into()],
        },
        LensEntry {
            name: "linguistic".into(),
            category: LensCategory::Extended,
            description: "Measure Zipf's law exponent and rank-frequency distribution (vocab ~ 2^n=6)".into(),
            domain_affinity: vec!["nlp".into(), "ai".into(), "social".into(), "information".into()],
            complementary: vec!["power_law".into(), "entropy".into(), "info".into()],
        },
        LensEntry {
            name: "entropy_production_rate".into(),
            category: LensCategory::Extended,
            description: "Quantify irreversibility via forward/backward transition asymmetry (R(6)=1 reversibility)".into(),
            domain_affinity: vec!["physics".into(), "chemistry".into(), "biology".into(), "thermo".into(), "environment".into()],
            complementary: vec!["thermo".into(), "causal".into(), "memory".into()],
        },
        LensEntry {
            name: "polymer".into(),
            category: LensCategory::Extended,
            description: "Chain statistics: Flory exponent, radius of gyration, persistence (Carbon Z=6 backbone)".into(),
            domain_affinity: vec!["chemistry".into(), "materials".into(), "biology".into(), "polymer".into(), "carbon-capture".into()],
            complementary: vec!["fractal".into(), "scale".into(), "hexagonal".into()],
        },
        LensEntry {
            name: "resonance".into(),
            category: LensCategory::Extended,
            description: "Detect cross-dimensional coupling at integer frequency ratios (divisors of 6: 1,2,3,6)".into(),
            domain_affinity: vec!["physics".into(), "engineering".into(), "music".into(), "signal".into()],
            complementary: vec!["wave".into(), "acoustic".into(), "correlation".into()],
        },
        LensEntry {
            name: "convexity_geometry".into(),
            category: LensCategory::Extended,
            description: "Measure data cloud convexity, curvature, and hull vertex count (hexagon=n=6 optimal)".into(),
            domain_affinity: vec!["mathematics".into(), "optimization".into(), "geometry".into(), "ai".into()],
            complementary: vec!["topology".into(), "boundary".into(), "hexagonal".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontier_lens_count() {
        let entries = frontier_lens_entries();
        assert_eq!(entries.len(), 8, "Must have exactly 8 frontier lenses");
    }

    #[test]
    fn test_frontier_lens_names_unique() {
        let entries = frontier_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All frontier lens names must be unique");
    }

    #[test]
    fn test_frontier_all_extended() {
        let entries = frontier_lens_entries();
        for entry in &entries {
            assert_eq!(
                entry.category,
                LensCategory::Extended,
                "Lens '{}' should be Extended category",
                entry.name
            );
        }
    }
}

/// Build metadata entries for 8 new-domain lenses (todo#16).
///
/// Domains: morphology, tonal harmony, ecological niche, macroeconomics,
/// immunogenetics, phonetics, food web, behavioral economics.
pub fn new_domain_lens_entries() -> Vec<LensEntry> {
    vec![
        LensEntry {
            name: "morphology".into(),
            category: LensCategory::Extended,
            description: "Detect morpheme distribution: 6 POS classes, tau=4 inflection, sigma=12 phoneme patterns (BT-112,BT-215)".into(),
            domain_affinity: vec!["linguistics".into(), "nlp".into(), "ai".into(), "social".into()],
            complementary: vec!["linguistic".into(), "entropy".into(), "clustering".into()],
        },
        LensEntry {
            name: "tonal_harmony".into(),
            category: LensCategory::Extended,
            description: "Measure hexatonic scale (6-note), triad tau=4 inversions, circle-of-fifths cycle 24 (BT-108,BT-135)".into(),
            domain_affinity: vec!["music".into(), "audio".into(), "signal".into(), "physics".into()],
            complementary: vec!["music_harmony".into(), "acoustic".into(), "resonance".into()],
        },
        LensEntry {
            name: "ecological_niche".into(),
            category: LensCategory::Extended,
            description: "Compute Simpson diversity and 6-tier trophic hierarchy alignment (BT-178,BT-201)".into(),
            domain_affinity: vec!["ecology".into(), "biology".into(), "environment".into(), "climate".into()],
            complementary: vec!["evolution".into(), "network".into(), "stability".into()],
        },
        LensEntry {
            name: "macroeconomics".into(),
            category: LensCategory::Extended,
            description: "Detect 6-phase business cycle, 4-quarter periodicity, sigma*phi=288 attractor (BT-143,BT-189)".into(),
            domain_affinity: vec!["economics".into(), "finance".into(), "social".into(), "energy".into()],
            complementary: vec!["game_theory".into(), "periodicity".into(), "phase_transition".into()],
        },
        LensEntry {
            name: "immunogenetics".into(),
            category: LensCategory::Extended,
            description: "Measure 6 Ig class diversity, HLA 6-locus polymorphism, MHC-II 6 subunit proximity (BT-155,BT-194,BT-220)".into(),
            domain_affinity: vec!["immunology".into(), "biology".into(), "medicine".into(), "genetics".into()],
            complementary: vec!["immune_response".into(), "genome_coding".into(), "evolution".into()],
        },
        LensEntry {
            name: "phonetics".into(),
            category: LensCategory::Extended,
            description: "Detect 6 articulation positions, F1/F2 formant ratios, sigma=12 vowel formant pairs (BT-112,BT-137)".into(),
            domain_affinity: vec!["linguistics".into(), "audio".into(), "nlp".into(), "signal".into()],
            complementary: vec!["morphology".into(), "acoustic".into(), "spectral".into()],
        },
        LensEntry {
            name: "food_web".into(),
            category: LensCategory::Extended,
            description: "Compute 6-tier biomass pyramid, sigma/tau=3 predator-prey ratio, energy transfer efficiency (BT-178,BT-201,BT-233)".into(),
            domain_affinity: vec!["ecology".into(), "biology".into(), "environment".into(), "ocean".into()],
            complementary: vec!["ecological_niche".into(), "network".into(), "graph".into()],
        },
        LensEntry {
            name: "behavioral_economics".into(),
            category: LensCategory::Extended,
            description: "Detect 6 cognitive bias clusters, phi=2 loss aversion, n=6 choice overload threshold (BT-143,BT-189,BT-212)".into(),
            domain_affinity: vec!["economics".into(), "psychology".into(), "social".into(), "ai".into()],
            complementary: vec!["macroeconomics".into(), "game_theory".into(), "decision".into()],
        },
    ]
}

/// 차원지각 렌즈 메타데이터 (BT-1108)
///
/// 도메인: VR/AR/XR, 완전광학, 4D 기하학, 인지과학, 끈이론, 디스플레이, 햅틱
pub fn dimensional_perception_lens_entries() -> Vec<LensEntry> {
    vec![
        LensEntry {
            name: "dimensional_perception".into(),
            category: LensCategory::Extended,
            description: "BT-1108 차원지각: 완전광학함수 6D=n, Tesseract C(4,2)=6=n, SO(4) 6 회전평면, OpenBCI 16ch=phi^tau, 알파밴드 8-12Hz, Calabi-Yau 6D".into(),
            domain_affinity: vec![
                "vr_ar_xr".into(),
                "plenoptic".into(),
                "4d_geometry".into(),
                "cognitive_science".into(),
                "string_theory".into(),
                "display".into(),
                "haptics".into(),
                "bci".into(),
            ],
            complementary: vec![
                "brain_neural".into(),
                "brain_map".into(),
                "dimensional_bridge".into(),
                "topology".into(),
                "quantum_circuit".into(),
                "hexagonal".into(),
            ],
        },
    ]
}

/// 신규 4종 렌즈 메타데이터 (마케팅/디지털트윈/발효/시계공학)
pub fn new_quad_lens_entries() -> Vec<LensEntry> {
    vec![
        LensEntry {
            name: "marketing".into(),
            category: LensCategory::Extended,
            description: "BT-548~557 마케팅 불변법칙 sigma=12, 4P tau=4, 이집트분수 1/2+1/3+1/6=1, NPS sigma-phi=10, 바이럴 R0=n=6".into(),
            domain_affinity: vec![
                "marketing".into(),
                "economics".into(),
                "social".into(),
                "psychology".into(),
                "media".into(),
            ],
            complementary: vec![
                "behavioral_economics".into(),
                "macroeconomics".into(),
                "game_theory".into(),
                "social_network".into(),
            ],
        },
        LensEntry {
            name: "digital_twin".into(),
            category: LensCategory::Extended,
            description: "디지털 트윈: IoT 6레이어=n, ISO 23247 tau=4 레이어, OPC UA sigma-tau=8 노드, 5G sopfr=5 슬라이스, J2=24시간 루프".into(),
            domain_affinity: vec![
                "digital_twin".into(),
                "iot".into(),
                "manufacturing".into(),
                "simulation".into(),
                "ai".into(),
            ],
            complementary: vec![
                "chip_architecture".into(),
                "networking_protocol".into(),
                "simulation".into(),
                "sensor".into(),
            ],
        },
        LensEntry {
            name: "fermentation".into(),
            category: LensCategory::Extended,
            description: "발효 생물화학: 포도당 C₆=n, 해당효소 n=6, ATP 순이득 phi=2, pH tau=4, 로지스틱 성장 S곡선, sigma=12 알코올내성".into(),
            domain_affinity: vec![
                "fermentation".into(),
                "biology".into(),
                "chemistry".into(),
                "food_science".into(),
                "biotechnology".into(),
            ],
            complementary: vec![
                "molecular_transform".into(),
                "evolution".into(),
                "polymer".into(),
                "food_chemistry".into(),
            ],
        },
        LensEntry {
            name: "horology".into(),
            category: LensCategory::Extended,
            description: "시계공학: 기어 n=6, 다이얼 sigma=12, J2=24시간, 탈진 tau=4Hz, 팰릿 phi=2, 3바늘 n/phi=3, 탈진 주기성 감지".into(),
            domain_affinity: vec![
                "horology".into(),
                "precision_mechanics".into(),
                "metrology".into(),
                "physics".into(),
                "manufacturing".into(),
            ],
            complementary: vec![
                "periodicity".into(),
                "tribology".into(),
                "materials_crystal".into(),
                "resonance".into(),
            ],
        },
    ]
}

/// 신규 56종 렌즈 메타데이터 (397→453 확장)
///
/// 카테고리: 도메인 간 브리지 (12), 시계열/인과/복잡계 (14),
///           불확실성/베이지안 (10), 머신러닝 고급 (20)
pub fn expansion_56_lens_entries() -> Vec<LensEntry> {
    vec![
        // ── 도메인 간 브리지 (12종) ──
        LensEntry {
            name: "cross_domain_bridge".into(),
            category: LensCategory::Extended,
            description: "이종 도메인 구조 공명: 브리지 연결=n=6, 동형 레이어=tau=4, 공명 주파수=sigma=12".into(),
            domain_affinity: vec!["multi_domain".into(), "integration".into(), "systems".into(), "ai".into()],
            complementary: vec!["isomorphism".into(), "topology".into(), "network".into()],
        },
        LensEntry {
            name: "agent_coordination".into(),
            category: LensCategory::Extended,
            description: "다중 에이전트 조율: 에이전트=n=6, 합의=tau=4, 메시지=sigma=12".into(),
            domain_affinity: vec!["ai".into(), "robotics".into(), "social".into(), "distributed".into()],
            complementary: vec!["faction_debate".into(), "consensus".into(), "game_theory".into()],
        },
        LensEntry {
            name: "knowledge_graph".into(),
            category: LensCategory::Extended,
            description: "지식 그래프: 관계=n=6, 엔티티=sigma=12, 임베딩=tau*sigma=48".into(),
            domain_affinity: vec!["nlp".into(), "ai".into(), "semantics".into(), "database".into()],
            complementary: vec!["graph".into(), "semantic".into(), "embedding".into()],
        },
        LensEntry {
            name: "distributed_consensus".into(),
            category: LensCategory::Extended,
            description: "분산 합의: 내결함성=n/phi=3, 노드=n=6, Paxos=phi=2".into(),
            domain_affinity: vec!["distributed".into(), "networking".into(), "blockchain".into()],
            complementary: vec!["consensus".into(), "network".into(), "fault_tolerance".into()],
        },
        LensEntry {
            name: "optimal_transport".into(),
            category: LensCategory::Extended,
            description: "최적 수송: Earth-mover 분할=n=6, Sinkhorn=sigma=12, dual=phi=2".into(),
            domain_affinity: vec!["mathematics".into(), "ai".into(), "economics".into(), "physics".into()],
            complementary: vec!["density".into(), "clustering".into(), "dimension_reduction".into()],
        },
        LensEntry {
            name: "structural_equation".into(),
            category: LensCategory::Extended,
            description: "구조 방정식 SEM: 잠재 변수=n=6, 경로=phi=2, 적합도=tau=4".into(),
            domain_affinity: vec!["statistics".into(), "social".into(), "psychology".into(), "biology".into()],
            complementary: vec!["causal_chain".into(), "bayesian_inference".into(), "network".into()],
        },
        LensEntry {
            name: "community_detection".into(),
            category: LensCategory::Extended,
            description: "커뮤니티 탐지: 커뮤니티=n=6, modularity=1-1/sigma, 해상도=phi=2".into(),
            domain_affinity: vec!["social".into(), "network".into(), "biology".into(), "marketing".into()],
            complementary: vec!["clustering".into(), "graph".into(), "faction_debate".into()],
        },
        LensEntry {
            name: "link_prediction".into(),
            category: LensCategory::Extended,
            description: "링크 예측: 경로=n=6, Katz beta=1/sigma, 공통 이웃=tau=4".into(),
            domain_affinity: vec!["social".into(), "network".into(), "recommendation".into()],
            complementary: vec!["graph".into(), "spectral_graph".into(), "network".into()],
        },
        LensEntry {
            name: "spectral_graph".into(),
            category: LensCategory::Extended,
            description: "스펙트럴 그래프: k=n=6 고유벡터, Fiedler=1/n, 스펙트럴 갭=phi=2".into(),
            domain_affinity: vec!["mathematics".into(), "network".into(), "image".into(), "physics".into()],
            complementary: vec!["spectral".into(), "graph".into(), "community_detection".into()],
        },
        LensEntry {
            name: "network_flow".into(),
            category: LensCategory::Extended,
            description: "네트워크 흐름: max-flow 컷=n=6, 채널=tau=4, 용량=sigma=12".into(),
            domain_affinity: vec!["logistics".into(), "network".into(), "engineering".into(), "economics".into()],
            complementary: vec!["graph".into(), "optimization".into(), "logistics_supply".into()],
        },
        LensEntry {
            name: "topological_sort".into(),
            category: LensCategory::Extended,
            description: "위상 정렬 DAG: 의존 깊이=n=6, 병렬 레벨=tau=4, 선행=phi=2".into(),
            domain_affinity: vec!["computer_science".into(), "build_systems".into(), "scheduling".into()],
            complementary: vec!["graph".into(), "recursion".into(), "causal_chain".into()],
        },
        LensEntry {
            name: "probabilistic_graphical".into(),
            category: LensCategory::Extended,
            description: "확률 그래프: 노드=n=6, 부모=phi=2, Markov blanket=tau=4".into(),
            domain_affinity: vec!["statistics".into(), "ai".into(), "biology".into(), "robotics".into()],
            complementary: vec!["bayesian_inference".into(), "markov_chain".into(), "causal".into()],
        },
        // ── 시계열/인과/복잡계 (14종) ──
        LensEntry {
            name: "causal_discovery".into(),
            category: LensCategory::Extended,
            description: "인과 발견 PC/FCI: 조건 집합=n=6, skeleton=sigma=12, v-구조=tau=4".into(),
            domain_affinity: vec!["statistics".into(), "ai".into(), "medicine".into(), "economics".into()],
            complementary: vec!["causal_chain".into(), "causal".into(), "bayesian_inference".into()],
        },
        LensEntry {
            name: "time_series_decomp".into(),
            category: LensCategory::Extended,
            description: "시계열 분해 STL: 계절성=n=6, 분해 레이어=tau=4, 레벨=sigma/phi=6".into(),
            domain_affinity: vec!["statistics".into(), "economics".into(), "signal".into(), "climate".into()],
            complementary: vec!["periodicity".into(), "fourier_analysis".into(), "stationarity".into()],
        },
        LensEntry {
            name: "markov_chain".into(),
            category: LensCategory::Extended,
            description: "마르코프 체인: 상태=n=6, 혼합=sigma=12, 재귀=n/phi=3".into(),
            domain_affinity: vec!["statistics".into(), "physics".into(), "economics".into(), "biology".into()],
            complementary: vec!["periodicity".into(), "stationarity".into(), "monte_carlo".into()],
        },
        LensEntry {
            name: "attractor_basin".into(),
            category: LensCategory::Extended,
            description: "어트랙터 분지: 카오스→질서=n=6, 주기 배가=tau=4, 분지=n/phi=3".into(),
            domain_affinity: vec!["physics".into(), "biology".into(), "neural".into(), "climate".into()],
            complementary: vec!["chaos".into(), "phase_transition".into(), "stability".into()],
        },
        LensEntry {
            name: "complexity_network".into(),
            category: LensCategory::Extended,
            description: "복잡 네트워크 스몰월드: 이웃=n=6, scale-free gamma=tau-1, 허브=sigma=12".into(),
            domain_affinity: vec!["network".into(), "biology".into(), "social".into(), "internet".into()],
            complementary: vec!["network".into(), "community_detection".into(), "emergence".into()],
        },
        LensEntry {
            name: "fourier_analysis".into(),
            category: LensCategory::Extended,
            description: "푸리에 분석: harmonics=n=6, FFT=phi^sigma=4096, 기본주파수=1/n".into(),
            domain_affinity: vec!["signal".into(), "physics".into(), "audio".into(), "communications".into()],
            complementary: vec!["spectral".into(), "wavelet_transform".into(), "periodicity".into()],
        },
        LensEntry {
            name: "wavelet_transform".into(),
            category: LensCategory::Extended,
            description: "웨이블릿 변환 다중 해상도: 레벨=n=6, Daubechies=tau*phi=8, 스케일=phi^n=64".into(),
            domain_affinity: vec!["signal".into(), "image".into(), "physics".into(), "finance".into()],
            complementary: vec!["fourier_analysis".into(), "multiscale".into(), "compression".into()],
        },
        LensEntry {
            name: "kalman_filter".into(),
            category: LensCategory::Extended,
            description: "칼만 필터 상태 추정: 상태=n=6, 관측=tau=4, innovation=1/n".into(),
            domain_affinity: vec!["robotics".into(), "navigation".into(), "signal".into(), "finance".into()],
            complementary: vec!["particle_filter".into(), "bayesian_inference".into(), "stability".into()],
        },
        LensEntry {
            name: "particle_filter".into(),
            category: LensCategory::Extended,
            description: "파티클 필터 비선형: 파티클=n*sigma=72, 재샘플=tau=4, 효과적=n=6".into(),
            domain_affinity: vec!["robotics".into(), "navigation".into(), "signal".into(), "biology".into()],
            complementary: vec!["kalman_filter".into(), "monte_carlo".into(), "bayesian_inference".into()],
        },
        LensEntry {
            name: "sampling_theory".into(),
            category: LensCategory::Extended,
            description: "샘플링 이론 Nyquist: 비율=phi=2, 오버샘플=sigma=12, aliasing=n=6".into(),
            domain_affinity: vec!["signal".into(), "audio".into(), "statistics".into(), "communications".into()],
            complementary: vec!["fourier_analysis".into(), "signal_reconstruction".into(), "periodicity".into()],
        },
        LensEntry {
            name: "signal_reconstruction".into(),
            category: LensCategory::Extended,
            description: "신호 재구성 compressed sensing: sparsity=n=6, RIP delta=1/n, measurement=sigma*tau=48".into(),
            domain_affinity: vec!["signal".into(), "image".into(), "communications".into(), "ai".into()],
            complementary: vec!["compression".into(), "sampling_theory".into(), "fourier_analysis".into()],
        },
        LensEntry {
            name: "sensitivity_analysis".into(),
            category: LensCategory::Extended,
            description: "민감도 분석 Sobol: 지수=n=6, 1차 효과=phi=2, 교호작용=tau=4".into(),
            domain_affinity: vec!["engineering".into(), "statistics".into(), "finance".into(), "ai".into()],
            complementary: vec!["uncertainty_quantification".into(), "simulation".into(), "gradient".into()],
        },
        LensEntry {
            name: "topological_data".into(),
            category: LensCategory::Extended,
            description: "위상 데이터 TDA: 필터링=n=6, 바코드=sigma=12, 호몰로지=tau=4".into(),
            domain_affinity: vec!["mathematics".into(), "biology".into(), "materials".into(), "ai".into()],
            complementary: vec!["persistence_homology".into(), "topology".into(), "manifold_learning".into()],
        },
        // ── 불확실성/베이지안 (10종) ──
        LensEntry {
            name: "bayesian_inference".into(),
            category: LensCategory::Extended,
            description: "베이지안 추론: beta=n=6, Bayes factor=tau-1=3, KL threshold=1/n".into(),
            domain_affinity: vec!["statistics".into(), "ai".into(), "medicine".into(), "physics".into()],
            complementary: vec!["probabilistic_graphical".into(), "variational_inference".into(), "monte_carlo".into()],
        },
        LensEntry {
            name: "uncertainty_quantification".into(),
            category: LensCategory::Extended,
            description: "불확실성 정량화: 6-sigma=n=6, UQ 분류=tau=4, coverage=5/6".into(),
            domain_affinity: vec!["engineering".into(), "statistics".into(), "ai".into(), "physics".into()],
            complementary: vec!["bayesian_inference".into(), "monte_carlo".into(), "sensitivity_analysis".into()],
        },
        LensEntry {
            name: "information_bottleneck".into(),
            category: LensCategory::Extended,
            description: "정보 병목 IB: beta=n=6, 클러스터=n=6, relevance 차원=tau=4".into(),
            domain_affinity: vec!["information_theory".into(), "ai".into(), "neuroscience".into()],
            complementary: vec!["entropy".into(), "dimension_reduction".into(), "compression".into()],
        },
        LensEntry {
            name: "renyi_entropy".into(),
            category: LensCategory::Extended,
            description: "레니 엔트로피 alpha 차수: Hartley=log(n=6), collision alpha=phi=2, min-entropy=1/sigma".into(),
            domain_affinity: vec!["information_theory".into(), "physics".into(), "cryptography".into()],
            complementary: vec!["entropy".into(), "information_bottleneck".into(), "bayesian_inference".into()],
        },
        LensEntry {
            name: "monte_carlo".into(),
            category: LensCategory::Extended,
            description: "몬테카를로 MCMC: chain=n=6, burn-in=sigma^phi=144, thin=tau=4".into(),
            domain_affinity: vec!["statistics".into(), "physics".into(), "finance".into(), "biology".into()],
            complementary: vec!["bayesian_inference".into(), "variational_inference".into(), "sampling_theory".into()],
        },
        LensEntry {
            name: "variational_inference".into(),
            category: LensCategory::Extended,
            description: "변분 추론 ELBO: 잠재=n=6, KL weight=1/n, 반복=sigma=12".into(),
            domain_affinity: vec!["ai".into(), "statistics".into(), "generative".into()],
            complementary: vec!["bayesian_inference".into(), "monte_carlo".into(), "dimensionality_bottleneck".into()],
        },
        LensEntry {
            name: "manifold_learning".into(),
            category: LensCategory::Extended,
            description: "다양체 학습 내재 차원: d_int=n=6, 이웃=sigma/phi=6, geodesic=n=6".into(),
            domain_affinity: vec!["ai".into(), "mathematics".into(), "image".into(), "neuroscience".into()],
            complementary: vec!["dimension_reduction".into(), "topology".into(), "topological_data".into()],
        },
        LensEntry {
            name: "persistence_homology".into(),
            category: LensCategory::Extended,
            description: "영속 호몰로지: Betti=n=6, persistence bar=sigma=12, death/birth=phi=2".into(),
            domain_affinity: vec!["mathematics".into(), "biology".into(), "materials".into(), "data_science".into()],
            complementary: vec!["topological_data".into(), "topology".into(), "manifold_learning".into()],
        },
        LensEntry {
            name: "cross_validation".into(),
            category: LensCategory::Extended,
            description: "교차 검증 k-fold: k=n=6, stratified=tau=4, nested=phi=2".into(),
            domain_affinity: vec!["ai".into(), "statistics".into(), "data_science".into()],
            complementary: vec!["hyperparameter_tuning".into(), "overfitting".into(), "model_selection".into()],
        },
        LensEntry {
            name: "hyperparameter_tuning".into(),
            category: LensCategory::Extended,
            description: "하이퍼파라미터 조정 BO: 탐색=n=6, acquisition=phi=2, budget=n*sigma=72".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "data_science".into()],
            complementary: vec!["cross_validation".into(), "neural_architecture".into(), "active_learning".into()],
        },
        // ── 머신러닝 고급 (20종) ──
        LensEntry {
            name: "transfer_learning".into(),
            category: LensCategory::Extended,
            description: "전이 학습: 레이어=n=6, fine-tune=tau=4, 적응=1/phi=0.5".into(),
            domain_affinity: vec!["ai".into(), "nlp".into(), "vision".into(), "robotics".into()],
            complementary: vec!["meta_learning".into(), "continual_learning".into(), "neural_architecture".into()],
        },
        LensEntry {
            name: "contrastive_learning".into(),
            category: LensCategory::Extended,
            description: "대조 학습 SimCLR: negative=n=6, 온도=tau/sigma, augment=phi=2".into(),
            domain_affinity: vec!["ai".into(), "vision".into(), "nlp".into(), "representation".into()],
            complementary: vec!["self_supervised".into(), "transfer_learning".into(), "embedding".into()],
        },
        LensEntry {
            name: "self_supervised".into(),
            category: LensCategory::Extended,
            description: "자기지도 학습: 마스크=1/n=1/6, pretext=tau=4, 배치=sigma^phi=144".into(),
            domain_affinity: vec!["ai".into(), "nlp".into(), "vision".into(), "representation".into()],
            complementary: vec!["contrastive_learning".into(), "transfer_learning".into(), "mask_modeling".into()],
        },
        LensEntry {
            name: "meta_learning".into(),
            category: LensCategory::Extended,
            description: "메타 학습 MAML: shot=n=6, inner loop=phi=2, gradient=tau=4".into(),
            domain_affinity: vec!["ai".into(), "robotics".into(), "few_shot".into()],
            complementary: vec!["transfer_learning".into(), "active_learning".into(), "reinforcement_reward".into()],
        },
        LensEntry {
            name: "graph_neural".into(),
            category: LensCategory::Extended,
            description: "그래프 신경망 GNN: 집계 홉=n=6, 이웃=sigma=12, 레이어=tau=4".into(),
            domain_affinity: vec!["ai".into(), "chemistry".into(), "social".into(), "biology".into()],
            complementary: vec!["graph".into(), "spectral_graph".into(), "knowledge_graph".into()],
        },
        LensEntry {
            name: "attention_mechanism".into(),
            category: LensCategory::Extended,
            description: "어텐션 메커니즘 Transformer: 헤드=n=6, KV=sigma^phi=144, 온도=phi=2".into(),
            domain_affinity: vec!["ai".into(), "nlp".into(), "vision".into(), "speech".into()],
            complementary: vec!["transformer_anatomy".into(), "multi_task".into(), "neural_architecture".into()],
        },
        LensEntry {
            name: "reinforcement_reward".into(),
            category: LensCategory::Extended,
            description: "강화학습 정책 최적화: 행동=n=6, 할인=1-1/sigma, 에피소드=tau*sigma=48".into(),
            domain_affinity: vec!["ai".into(), "robotics".into(), "game".into(), "economics".into()],
            complementary: vec!["meta_learning".into(), "game_theory".into(), "multi_task".into()],
        },
        LensEntry {
            name: "multi_task".into(),
            category: LensCategory::Extended,
            description: "다중 과제 MTL: 과제=n=6, 공유=tau=4, MTL 손실=phi=2".into(),
            domain_affinity: vec!["ai".into(), "nlp".into(), "robotics".into(), "vision".into()],
            complementary: vec!["attention_mechanism".into(), "transfer_learning".into(), "neural_architecture".into()],
        },
        LensEntry {
            name: "adversarial_robustness".into(),
            category: LensCategory::Extended,
            description: "적대적 강건성 PGD: 단계=n=6, epsilon=1/sigma=1/12, L∞=1/n".into(),
            domain_affinity: vec!["ai".into(), "security".into(), "vision".into(), "nlp".into()],
            complementary: vec!["fairness_bias".into(), "explainability".into(), "neural_architecture".into()],
        },
        LensEntry {
            name: "continual_learning".into(),
            category: LensCategory::Extended,
            description: "연속 학습 EWC: 과제=n=6, lambda=1/n, replay=sigma*tau=48".into(),
            domain_affinity: vec!["ai".into(), "robotics".into(), "neuroscience".into()],
            complementary: vec!["transfer_learning".into(), "meta_learning".into(), "plasticity_consolidation".into()],
        },
        LensEntry {
            name: "federated_learning".into(),
            category: LensCategory::Extended,
            description: "연합 학습 FedAvg: 클라이언트=n=6, round=sigma=12, local=tau=4".into(),
            domain_affinity: vec!["ai".into(), "privacy".into(), "distributed".into(), "healthcare".into()],
            complementary: vec!["distributed_consensus".into(), "differential_privacy".into(), "multi_task".into()],
        },
        LensEntry {
            name: "neural_architecture".into(),
            category: LensCategory::Extended,
            description: "신경망 구조 NAS: depth=n=6, width=sigma^phi=144, bottleneck=tau=4".into(),
            domain_affinity: vec!["ai".into(), "hardware".into(), "optimization".into()],
            complementary: vec!["hyperparameter_tuning".into(), "transformer_anatomy".into(), "chip_architecture".into()],
        },
        LensEntry {
            name: "active_learning".into(),
            category: LensCategory::Extended,
            description: "능동 학습 쿼리: 배치=n=6, uncertainty=1/n, pool=sigma^phi=144".into(),
            domain_affinity: vec!["ai".into(), "annotation".into(), "data_science".into()],
            complementary: vec!["bayesian_inference".into(), "hyperparameter_tuning".into(), "cross_validation".into()],
        },
        LensEntry {
            name: "fairness_bias".into(),
            category: LensCategory::Extended,
            description: "공정성 편향 알고리즘: 집단=n=6, disparity=1/n, equalized odds=phi=2".into(),
            domain_affinity: vec!["ai".into(), "social".into(), "ethics".into(), "law".into()],
            complementary: vec!["adversarial_robustness".into(), "explainability".into(), "statistical_parity".into()],
        },
        LensEntry {
            name: "explainability".into(),
            category: LensCategory::Extended,
            description: "설명가능성 SHAP/LIME: 특성=n=6, 샘플=sigma^phi=144, 근사=1/n".into(),
            domain_affinity: vec!["ai".into(), "medicine".into(), "law".into(), "finance".into()],
            complementary: vec!["fairness_bias".into(), "attention_mechanism".into(), "gradient".into()],
        },
        LensEntry {
            name: "dimensionality_bottleneck".into(),
            category: LensCategory::Extended,
            description: "차원 병목 latent: 잠재=n=6, 인코더=tau=4, 재구성=1/n".into(),
            domain_affinity: vec!["ai".into(), "generative".into(), "signal".into()],
            complementary: vec!["variational_inference".into(), "information_bottleneck".into(), "compression".into()],
        },
        LensEntry {
            name: "spiking_neural".into(),
            category: LensCategory::Extended,
            description: "스파이킹 신경망 LIF: 불응기=tau=4ms, 시냅스=sigma=12, 발화=n=6Hz".into(),
            domain_affinity: vec!["neuroscience".into(), "ai".into(), "hardware".into(), "robotics".into()],
            complementary: vec!["hebbian_plasticity".into(), "plasticity_consolidation".into(), "bci_neurofeedback".into()],
        },
        LensEntry {
            name: "reservoir_computing".into(),
            category: LensCategory::Extended,
            description: "저수지 컴퓨팅 ESN: 노드=n*sigma=72, spectral=1-1/n, echo=tau=4".into(),
            domain_affinity: vec!["ai".into(), "signal".into(), "physics".into(), "neuroscience".into()],
            complementary: vec!["spiking_neural".into(), "markov_chain".into(), "time_series_decomp".into()],
        },
        LensEntry {
            name: "plasticity_consolidation".into(),
            category: LensCategory::Extended,
            description: "가소성 강화 기억 공고화: 단계=n=6, SWS=tau=4, 재활성=sigma=12".into(),
            domain_affinity: vec!["neuroscience".into(), "ai".into(), "medicine".into()],
            complementary: vec!["hebbian_plasticity".into(), "continual_learning".into(), "sleep_cycle".into()],
        },
        LensEntry {
            name: "cognitive_load".into(),
            category: LensCategory::Extended,
            description: "인지 부하 작업기억: 청크=n=6, 처리=tau=4, 주의=phi=2".into(),
            domain_affinity: vec!["psychology".into(), "education".into(), "ux".into(), "neuroscience".into()],
            complementary: vec!["brain_neural".into(), "behavioral_economics".into(), "attention_mechanism".into()],
        },
        LensEntry {
            name: "decision_boundary".into(),
            category: LensCategory::Extended,
            description: "결정 경계 SVM: 지지벡터=n=6, kernel=sigma=12, 마진=1/n".into(),
            domain_affinity: vec!["ai".into(), "statistics".into(), "pattern_recognition".into()],
            complementary: vec!["clustering".into(), "manifold_learning".into(), "adversarial_robustness".into()],
        },
    ]
}

#[cfg(test)]
mod expansion_56_tests {
    use super::*;

    #[test]
    fn test_expansion_56_count() {
        let entries = expansion_56_lens_entries();
        assert_eq!(entries.len(), 56, "Must have exactly 56 expansion lenses");
    }

    #[test]
    fn test_expansion_56_names_unique() {
        let entries = expansion_56_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All expansion lens names must be unique");
    }

    #[test]
    fn test_expansion_56_all_extended() {
        for entry in expansion_56_lens_entries() {
            assert_eq!(
                entry.category,
                LensCategory::Extended,
                "Lens '{}' should be Extended category",
                entry.name
            );
        }
    }

    #[test]
    fn test_expansion_56_no_empty_fields() {
        for entry in expansion_56_lens_entries() {
            assert!(!entry.name.is_empty());
            assert!(!entry.description.is_empty(), "Lens '{}' needs description", entry.name);
            assert!(!entry.domain_affinity.is_empty(), "Lens '{}' needs domain affinity", entry.name);
            assert!(!entry.complementary.is_empty(), "Lens '{}' needs complementary", entry.name);
        }
    }
}

#[cfg(test)]
mod new_domain_tests {
    use super::*;

    #[test]
    fn test_new_domain_lens_count() {
        let entries = new_domain_lens_entries();
        assert_eq!(entries.len(), 8, "Must have exactly 8 new-domain lenses");
    }

    #[test]
    fn test_new_domain_lens_names_unique() {
        let entries = new_domain_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All new-domain lens names must be unique");
    }

    #[test]
    fn test_new_domain_all_extended() {
        for entry in new_domain_lens_entries() {
            assert_eq!(
                entry.category,
                LensCategory::Extended,
                "Lens '{}' should be Extended category",
                entry.name
            );
        }
    }

    #[test]
    fn test_new_domain_no_empty_fields() {
        for entry in new_domain_lens_entries() {
            assert!(!entry.name.is_empty());
            assert!(!entry.description.is_empty(), "Lens '{}' needs description", entry.name);
            assert!(!entry.domain_affinity.is_empty(), "Lens '{}' needs domain affinity", entry.name);
            assert!(!entry.complementary.is_empty(), "Lens '{}' needs complementary", entry.name);
        }
    }
}
