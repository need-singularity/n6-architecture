//! Extended knowledge graph nodes: Predictions, Formulas, Cross-Resonances,
//! Engines, DSE Categories, Physical Constants, and Applications.
//!
//! This module expands the knowledge graph from ~135 toward the 500-node target
//! by encoding the rich interconnection structure of the n=6 discovery system.
//! Every node represents a real, meaningful entity in the TECS-L/n6 framework.

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// GraphNode: Generic knowledge graph entry (counted by metrics)
// ═══════════════════════════════════════════════════════════════

struct GraphNode {
    id: &'static str,
    category: &'static str,
    title: &'static str,
    domains: &'static [&'static str],
    confidence: f64,
    /// Related BTs, constants, or other node IDs for edge creation.
    connects_to: &'static [&'static str],
}

// ═══════════════════════════════════════════════════════════════
// TIER 1: Testable Predictions (TP-01 ~ TP-48)
// From docs/testable-predictions.md — 45 falsifiable predictions
// ═══════════════════════════════════════════════════════════════

const PREDICTIONS: &[GraphNode] = &[
    // Tier 1: Today (1 GPU)
    GraphNode { id: "TP-01", category: "Prediction", title: "EFA quality matches standard attention at 60% FLOPs", domains: &["AI"], confidence: 0.85, connects_to: &["BT-7", "T-17", "C-n"] },
    GraphNode { id: "TP-02", category: "Prediction", title: "LoRA optimal rank r=sigma-tau=8 across model sizes", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-58", "C-sigma-tau"] },
    GraphNode { id: "TP-03", category: "Prediction", title: "MoE(8,2) outperforms MoE(16,2) at same FLOPs", domains: &["AI"], confidence: 0.85, connects_to: &["BT-67", "T-04", "C-sigma-tau"] },
    GraphNode { id: "TP-04", category: "Prediction", title: "Mertens dropout p=ln(4/3)=0.288 beats grid search", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-46", "T-16", "C-tau"] },
    GraphNode { id: "TP-05", category: "Prediction", title: "Boltzmann 1/e gate matches top-k at 37% density", domains: &["AI"], confidence: 0.85, connects_to: &["T-15", "C-n"] },
    GraphNode { id: "TP-06", category: "Prediction", title: "Cyclotomic activation 71% FLOPs reduction verified", domains: &["AI"], confidence: 0.9, connects_to: &["T-01", "C-phi", "C-n"] },
    GraphNode { id: "TP-07", category: "Prediction", title: "Phi bottleneck 4/3x FFN optimal expansion ratio", domains: &["AI", "Math"], confidence: 0.85, connects_to: &["T-03", "BT-111", "C-tau"] },
    GraphNode { id: "TP-08", category: "Prediction", title: "FFT attention 3x speedup with quality preservation", domains: &["AI", "Chip"], confidence: 0.9, connects_to: &["T-08", "C-sigma"] },
    // Tier 2: Cluster-scale
    GraphNode { id: "TP-09", category: "Prediction", title: "SwiGLU ratio 4/3 outperforms 8/3 at large scale", domains: &["AI", "Math"], confidence: 0.8, connects_to: &["BT-33", "BT-111", "C-tau"] },
    GraphNode { id: "TP-10", category: "Prediction", title: "Weight decay 1/(sigma-phi)=0.1 universal optimum", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-54", "BT-64", "C-sigma-phi"] },
    GraphNode { id: "TP-11", category: "Prediction", title: "Optimal head count h=sigma=12 for d=4096", domains: &["AI", "Chip"], confidence: 0.85, connects_to: &["BT-56", "C-sigma"] },
    GraphNode { id: "TP-12", category: "Prediction", title: "RoPE theta=10^(sigma-phi)=10000 optimal base", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-34", "C-sigma-phi"] },
    GraphNode { id: "TP-13", category: "Prediction", title: "AdamW beta1=1-1/(sigma-phi)=0.9 universal", domains: &["AI", "Math"], confidence: 0.95, connects_to: &["BT-54", "C-sigma-phi"] },
    GraphNode { id: "TP-14", category: "Prediction", title: "AdamW beta2=1-1/(J2-tau)=0.95 universal", domains: &["AI", "Math"], confidence: 0.95, connects_to: &["BT-54", "C-J2", "C-tau"] },
    GraphNode { id: "TP-15", category: "Prediction", title: "Gradient clipping R(6)=1.0 optimal", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-54", "C-n"] },
    GraphNode { id: "TP-16", category: "Prediction", title: "Chinchilla tokens/params ratio = J2-tau = 20", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-26", "C-J2", "C-tau"] },
    GraphNode { id: "TP-17", category: "Prediction", title: "Context window powers: 2^{10,11,12,13} = sigma-phi ladder", domains: &["AI", "Chip"], confidence: 0.85, connects_to: &["BT-44", "C-sigma-phi", "C-sigma-mu", "C-sigma"] },
    // Tier 3: Specialized / Physics
    GraphNode { id: "TP-18", category: "Prediction", title: "SQ bandgap optimum at tau^2/sigma = 4/3 eV", domains: &["Solar", "Energy"], confidence: 0.8, connects_to: &["BT-30", "BT-111", "C-tau", "C-sigma"] },
    GraphNode { id: "TP-19", category: "Prediction", title: "JUNO neutrino mass ordering confirms n=6 mixing (2027)", domains: &["Particle", "Cosmology"], confidence: 0.6, connects_to: &["BT-21", "C-n"] },
    GraphNode { id: "TP-20", category: "Prediction", title: "LiteBIRD inflation r confirms n=6 prediction (2032)", domains: &["Cosmology", "Particle"], confidence: 0.5, connects_to: &["BT-22", "C-n"] },
    GraphNode { id: "TP-21", category: "Prediction", title: "Koide formula Q=2/3 to 9 ppm precision", domains: &["Particle", "Math"], confidence: 0.85, connects_to: &["BT-24", "BT-112", "C-phi", "C-n"] },
    GraphNode { id: "TP-22", category: "Prediction", title: "D-T fusion Q>10 at sopfr(6)=5 baryon number", domains: &["Fusion", "Particle"], confidence: 0.7, connects_to: &["BT-98", "BT-99", "C-sopfr"] },
    GraphNode { id: "TP-23", category: "Prediction", title: "SLE kappa=6 percolation exponents match 7/7", domains: &["Math", "Particle", "Topology"], confidence: 0.9, connects_to: &["BT-105", "C-n"] },
    // Tier 4: Industry
    GraphNode { id: "TP-24", category: "Prediction", title: "Next GPU SM count = sigma^2 = 144 or sigma*n = 72", domains: &["Chip", "Semiconductor"], confidence: 0.8, connects_to: &["BT-28", "BT-90", "C-sigma2"] },
    GraphNode { id: "TP-25", category: "Prediction", title: "HBM5 stack = sigma = 12 layers", domains: &["Chip", "Semiconductor"], confidence: 0.85, connects_to: &["BT-55", "BT-75", "C-sigma"] },
    GraphNode { id: "TP-26", category: "Prediction", title: "Next-gen chiplet count converges to sigma or J2", domains: &["Chip", "Semiconductor"], confidence: 0.75, connects_to: &["BT-69", "C-sigma", "C-J2"] },
    GraphNode { id: "TP-27", category: "Prediction", title: "TSMC next node gate pitch = sigma*tau = 48 Angstrom", domains: &["Semiconductor", "Chip"], confidence: 0.7, connects_to: &["BT-37", "BT-76", "C-sigma-tau-prod"] },
    GraphNode { id: "TP-28", category: "Prediction", title: "Solid-state battery CN=6 electrolyte outperforms liquid", domains: &["Battery", "Material"], confidence: 0.8, connects_to: &["BT-80", "BT-43", "C-n"] },
    GraphNode { id: "TP-29", category: "Prediction", title: "Grid frequency 60Hz = sigma*sopfr persists in new standards", domains: &["PowerGrid", "Energy"], confidence: 0.9, connects_to: &["BT-62", "C-sigma", "C-sopfr"] },
    GraphNode { id: "TP-30", category: "Prediction", title: "HVDC next voltage step follows n=6 ladder", domains: &["PowerGrid", "Energy"], confidence: 0.75, connects_to: &["BT-68", "C-sigma-phi"] },
    // Additional predictions
    GraphNode { id: "TP-31", category: "Prediction", title: "Diffusion DDPM T=1000=10^(n/phi) optimal steps", domains: &["AI", "Math"], confidence: 0.85, connects_to: &["BT-61", "C-n", "C-phi"] },
    GraphNode { id: "TP-32", category: "Prediction", title: "CFG guidance scale 7.5 = sopfr*phi + tau-mu optimal", domains: &["AI", "Math"], confidence: 0.7, connects_to: &["BT-61", "C-sopfr"] },
    GraphNode { id: "TP-33", category: "Prediction", title: "ViT patch size 2^tau = 16 universal across resolutions", domains: &["AI", "DisplayAudio"], confidence: 0.85, connects_to: &["BT-66", "C-tau"] },
    GraphNode { id: "TP-34", category: "Prediction", title: "EnCodec 8 codebooks = sigma-tau universal for audio", domains: &["AI", "Music", "DisplayAudio"], confidence: 0.85, connects_to: &["BT-72", "C-sigma-tau"] },
    GraphNode { id: "TP-35", category: "Prediction", title: "BTC block confirmation at n=6 is security threshold", domains: &["Blockchain", "Crypto"], confidence: 0.8, connects_to: &["BT-53", "C-n"] },
    GraphNode { id: "TP-36", category: "Prediction", title: "ETH block time sigma=12s persists post-merge", domains: &["Blockchain", "Crypto"], confidence: 0.9, connects_to: &["BT-53", "C-sigma"] },
    GraphNode { id: "TP-37", category: "Prediction", title: "6-DOF remains standard for industrial robots", domains: &["Robotics"], confidence: 0.95, connects_to: &["BT-123", "C-n"] },
    GraphNode { id: "TP-38", category: "Prediction", title: "Quadrotor tau=4 minimum stability threshold holds", domains: &["Robotics", "Energy"], confidence: 0.9, connects_to: &["BT-125", "C-tau"] },
    GraphNode { id: "TP-39", category: "Prediction", title: "AES-256 = 2^(sigma-sopfr)=128 bit security level", domains: &["Crypto", "Software"], confidence: 0.95, connects_to: &["BT-114", "C-sigma", "C-sopfr"] },
    GraphNode { id: "TP-40", category: "Prediction", title: "OSI 7 layers = sigma-sopfr persists in future protocols", domains: &["Network", "Software"], confidence: 0.9, connects_to: &["BT-115", "C-sigma", "C-sopfr"] },
    GraphNode { id: "TP-41", category: "Prediction", title: "DC 48V = sigma*tau standard for datacenter power", domains: &["PowerGrid", "Chip", "Energy"], confidence: 0.85, connects_to: &["BT-60", "BT-76", "C-sigma-tau-prod"] },
    GraphNode { id: "TP-42", category: "Prediction", title: "Mamba d_state=2^tau=16, expand=phi=2 optimal", domains: &["AI", "Math"], confidence: 0.85, connects_to: &["BT-65", "C-tau", "C-phi"] },
    GraphNode { id: "TP-43", category: "Prediction", title: "24fps/bit = J2 persists as media standard", domains: &["DisplayAudio", "Music"], confidence: 0.9, connects_to: &["BT-48", "C-J2"] },
    GraphNode { id: "TP-44", category: "Prediction", title: "Tokenizer 32K = 2^n * 10^n convergent vocabulary", domains: &["AI", "InfoTheory"], confidence: 0.8, connects_to: &["BT-73", "C-n"] },
    GraphNode { id: "TP-45", category: "Prediction", title: "Silicon 6 coordination bonds in next-gen photovoltaics", domains: &["Solar", "Material", "Chemistry"], confidence: 0.75, connects_to: &["BT-30", "BT-85", "C-n"] },
];

// ═══════════════════════════════════════════════════════════════
// TIER 2: Key n=6 Formulas & Identities (F-01 ~ F-36)
// ═══════════════════════════════════════════════════════════════

const FORMULAS: &[GraphNode] = &[
    // Core identity
    GraphNode { id: "F-01", category: "Formula", title: "sigma(n)*phi(n) = n*tau(n) iff n=6", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["C-sigma", "C-phi", "C-n", "C-tau"] },
    GraphNode { id: "F-02", category: "Formula", title: "1/2 + 1/3 + 1/6 = 1 (Egyptian fraction = perfect number)", domains: &["Math", "NumberTheory", "AI"], confidence: 1.0, connects_to: &["C-n", "C-phi", "T-10", "T-17"] },
    GraphNode { id: "F-03", category: "Formula", title: "sigma(6) = 1+2+3+6 = 12 (divisor sum)", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["C-sigma"] },
    GraphNode { id: "F-04", category: "Formula", title: "phi(6) = |{1,5}| = 2 (Euler totient)", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["C-phi"] },
    GraphNode { id: "F-05", category: "Formula", title: "J_2(6) = 6^2 * prod(1-1/p^2) = 24 (Jordan totient)", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["C-J2"] },
    GraphNode { id: "F-06", category: "Formula", title: "lambda(6) = lcm(lambda(2),lambda(3)) = 2 (Carmichael)", domains: &["Math", "NumberTheory", "Crypto"], confidence: 1.0, connects_to: &["C-phi", "T-14"] },
    GraphNode { id: "F-07", category: "Formula", title: "mu(6) = (-1)^2 = 1 (Mobius, squarefree)", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["C-mu", "T-13"] },
    GraphNode { id: "F-08", category: "Formula", title: "R(6) = sigma(6)*phi(6)/(6*tau(6)) = 1 (reversibility)", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["C-sigma", "C-phi", "C-n", "C-tau"] },
    // Cross-domain formulas
    GraphNode { id: "F-09", category: "Formula", title: "zeta(2) = pi^2/6 (Basel problem, Euler 1735)", domains: &["Math", "NumberTheory", "Particle"], confidence: 1.0, connects_to: &["BT-109", "C-n"] },
    GraphNode { id: "F-10", category: "Formula", title: "zeta(-1) = -1/12 (Ramanujan summation)", domains: &["Math", "NumberTheory", "StringTheory"], confidence: 1.0, connects_to: &["BT-109", "C-sigma"] },
    GraphNode { id: "F-11", category: "Formula", title: "K_3 = 12 = sigma (3D kissing number)", domains: &["Math", "Material", "Robotics"], confidence: 1.0, connects_to: &["BT-15", "BT-127", "C-sigma"] },
    GraphNode { id: "F-12", category: "Formula", title: "K_4 = 24 = J2 (4D kissing number = Leech precursor)", domains: &["Math", "CodingTheory"], confidence: 1.0, connects_to: &["BT-15", "BT-49", "C-J2"] },
    GraphNode { id: "F-13", category: "Formula", title: "Leech lattice dim = J2 = 24", domains: &["Math", "CodingTheory", "StringTheory"], confidence: 1.0, connects_to: &["BT-6", "BT-49", "C-J2"] },
    GraphNode { id: "F-14", category: "Formula", title: "S_6 unique outer automorphism (only symmetric group)", domains: &["Math", "Combinatorics"], confidence: 1.0, connects_to: &["BT-49", "BT-106", "C-n"] },
    GraphNode { id: "F-15", category: "Formula", title: "Golay(24,12,8): length=J2, dim=sigma, distance=sigma-tau", domains: &["Math", "CodingTheory", "QC"], confidence: 1.0, connects_to: &["BT-6", "C-J2", "C-sigma", "C-sigma-tau"] },
    GraphNode { id: "F-16", category: "Formula", title: "6 | B_{2k} for all k >= 1 (Bernoulli divisibility)", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["BT-109", "C-n"] },
    GraphNode { id: "F-17", category: "Formula", title: "Koide Q = (m_e+m_mu+m_tau) / (sqrt(m_e)+sqrt(m_mu)+sqrt(m_tau))^2 = 2/3", domains: &["Particle", "Math"], confidence: 0.95, connects_to: &["BT-24", "BT-112", "C-phi", "C-n"] },
    // Applied formulas
    GraphNode { id: "F-18", category: "Formula", title: "PUE = sigma/(sigma-phi) = 12/10 = 1.2 (datacenter)", domains: &["Energy", "Chip", "PowerGrid"], confidence: 0.9, connects_to: &["BT-60", "BT-62", "C-sigma", "C-sigma-phi"] },
    GraphNode { id: "F-19", category: "Formula", title: "Betz limit = 16/27 ~ tau^2/(phi^3 * n/phi) (wind)", domains: &["Energy", "Math"], confidence: 0.8, connects_to: &["BT-111", "C-tau"] },
    GraphNode { id: "F-20", category: "Formula", title: "LiC6 = 24e = J2 electron transfer", domains: &["Battery", "Chemistry"], confidence: 0.95, connects_to: &["BT-27", "BT-43", "C-J2", "C-n"] },
    GraphNode { id: "F-21", category: "Formula", title: "C6H12O6 = 24 atoms = J2 (glucose)", domains: &["Biology", "Chemistry", "Energy"], confidence: 1.0, connects_to: &["BT-101", "BT-103", "C-J2", "C-n"] },
    GraphNode { id: "F-22", category: "Formula", title: "Genetic code: tau -> n/phi -> 2^n -> J2-tau = 4->3->64->20", domains: &["Biology", "Math"], confidence: 0.95, connects_to: &["BT-51", "C-tau", "C-n", "C-J2"] },
    GraphNode { id: "F-23", category: "Formula", title: "SM fermions=12=sigma, bosons=12=sigma (particle physics)", domains: &["Particle", "Math"], confidence: 0.9, connects_to: &["BT-17", "C-sigma"] },
    GraphNode { id: "F-24", category: "Formula", title: "sin^2(theta_W) = 3/13 = (n/phi)/(sigma+mu)", domains: &["Particle", "Fusion", "Math"], confidence: 0.8, connects_to: &["BT-97", "C-n", "C-phi", "C-sigma", "C-mu"] },
    // Transformer architecture formulas
    GraphNode { id: "F-25", category: "Formula", title: "d_model = 2^sigma = 4096 (LLM standard)", domains: &["AI", "Chip"], confidence: 0.9, connects_to: &["BT-56", "C-sigma"] },
    GraphNode { id: "F-26", category: "Formula", title: "d_head = 2^(sigma-sopfr) = 128 (attention head dim)", domains: &["AI"], confidence: 0.9, connects_to: &["BT-56", "C-sigma", "C-sopfr"] },
    GraphNode { id: "F-27", category: "Formula", title: "n_layers = 2^sopfr = 32 (GPT-3 standard)", domains: &["AI"], confidence: 0.85, connects_to: &["BT-56", "C-sopfr"] },
    GraphNode { id: "F-28", category: "Formula", title: "top-p = 1 - 1/(J2-tau) = 0.95 (inference)", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-42", "BT-74", "C-J2", "C-tau"] },
    GraphNode { id: "F-29", category: "Formula", title: "top-k = 40 = tau*(sigma-phi) (inference sampling)", domains: &["AI", "Math"], confidence: 0.85, connects_to: &["BT-42", "C-tau", "C-sigma-phi"] },
    GraphNode { id: "F-30", category: "Formula", title: "DDPM beta_min = 10^{-(sigma-tau)} = 10^{-4}", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-61", "C-sigma-tau"] },
    // Physical / Energy formulas
    GraphNode { id: "F-31", category: "Formula", title: "KSTAR/ITER aspect ratio ~ tau (tokamak geometry)", domains: &["Fusion", "Tokamak"], confidence: 0.8, connects_to: &["BT-4", "BT-99", "C-tau"] },
    GraphNode { id: "F-32", category: "Formula", title: "12-tone equal temperament = sigma semitones", domains: &["Music", "DisplayAudio", "Math"], confidence: 1.0, connects_to: &["BT-48", "BT-108", "C-sigma"] },
    GraphNode { id: "F-33", category: "Formula", title: "48kHz sample rate = sigma*tau audio standard", domains: &["DisplayAudio", "Music"], confidence: 0.95, connects_to: &["BT-48", "C-sigma-tau-prod"] },
    GraphNode { id: "F-34", category: "Formula", title: "BTC 21M supply = J2 - n/phi (millions, approximation)", domains: &["Blockchain", "Crypto", "Math"], confidence: 0.7, connects_to: &["BT-53", "C-J2", "C-n", "C-phi"] },
    GraphNode { id: "F-35", category: "Formula", title: "Tesla 96S modules = sigma*(sigma-tau) battery pack", domains: &["Battery", "Automotive"], confidence: 0.85, connects_to: &["BT-57", "BT-84", "C-sigma", "C-sigma-tau"] },
    GraphNode { id: "F-36", category: "Formula", title: "CNO cycle A = sigma + {0,mu,phi,n/phi} = 12,13,14,15", domains: &["Fusion", "Particle", "Cosmology"], confidence: 0.9, connects_to: &["BT-100", "C-sigma", "C-mu", "C-phi"] },
];

// ═══════════════════════════════════════════════════════════════
// TIER 3: Cross-Resonance Patterns (CR-01 ~ CR-30)
// Connections where the same n=6 constant appears across domains
// ═══════════════════════════════════════════════════════════════

const CROSS_RESONANCES: &[GraphNode] = &[
    GraphNode { id: "CR-01", category: "CrossResonance", title: "sigma=12: semitones=SMs=divisor_sum=troposphere_km=HBM_layers", domains: &["Music", "Chip", "Math", "Environment"], confidence: 0.9, connects_to: &["BT-28", "BT-48", "BT-119", "C-sigma"] },
    GraphNode { id: "CR-02", category: "CrossResonance", title: "J2=24: kissing_4D=Leech=glucose_atoms=fps=Jordan_totient", domains: &["Math", "Biology", "DisplayAudio"], confidence: 0.95, connects_to: &["BT-15", "BT-48", "BT-101", "C-J2"] },
    GraphNode { id: "CR-03", category: "CrossResonance", title: "sigma-tau=8: LoRA=MoE_experts=KV_heads=codebooks=AI_stack_layers", domains: &["AI", "Chip", "Music"], confidence: 0.95, connects_to: &["BT-58", "BT-59", "BT-72", "C-sigma-tau"] },
    GraphNode { id: "CR-04", category: "CrossResonance", title: "sigma-phi=10: RoPE_base_exp=regularization_inv=neurons=battery_ratio", domains: &["AI", "Battery", "Biology"], confidence: 0.9, connects_to: &["BT-34", "BT-64", "BT-81", "C-sigma-phi"] },
    GraphNode { id: "CR-05", category: "CrossResonance", title: "tau=4: divisors=BCS_Cooper=locomotion=Bohm=codons_per_base", domains: &["Math", "SC", "Robotics", "Biology"], confidence: 0.9, connects_to: &["BT-2", "BT-51", "BT-125", "C-tau"] },
    GraphNode { id: "CR-06", category: "CrossResonance", title: "phi=2: pairing=bilateral=FP_ratio=Carmichael=quantization", domains: &["SC", "Robotics", "Chip", "Math"], confidence: 0.95, connects_to: &["BT-1", "BT-45", "BT-124", "C-phi"] },
    GraphNode { id: "CR-07", category: "CrossResonance", title: "n=6: perfect=carbon_Z=SE3_dim=6DOF=6GHGs=hexagon", domains: &["Math", "Chemistry", "Robotics", "Environment"], confidence: 0.95, connects_to: &["BT-85", "BT-118", "BT-122", "BT-123", "C-n"] },
    GraphNode { id: "CR-08", category: "CrossResonance", title: "sopfr=5: fingers=baryon_DT=SOLID=Bott=OSI-2", domains: &["Robotics", "Fusion", "Software", "Math"], confidence: 0.85, connects_to: &["BT-92", "BT-98", "BT-113", "BT-126", "C-sopfr"] },
    GraphNode { id: "CR-09", category: "CrossResonance", title: "sigma*tau=48: gate_pitch=sample_rate_k=voltage_DC=SH_coeffs", domains: &["Semiconductor", "DisplayAudio", "PowerGrid"], confidence: 0.9, connects_to: &["BT-37", "BT-48", "BT-60", "BT-76", "C-sigma-tau-prod"] },
    GraphNode { id: "CR-10", category: "CrossResonance", title: "sigma^2=144: AD102_SMs=solar_cells=sigma_squared", domains: &["Chip", "Solar", "Math"], confidence: 0.9, connects_to: &["BT-28", "BT-63", "BT-90", "C-sigma2"] },
    GraphNode { id: "CR-11", category: "CrossResonance", title: "sigma-mu=11: M_theory=TCP_congestion=RSA_key=SPARC_TF", domains: &["Particle", "Network", "Crypto", "Fusion"], confidence: 0.8, connects_to: &["BT-110", "C-sigma-mu"] },
    GraphNode { id: "CR-12", category: "CrossResonance", title: "0.1=1/(sigma-phi): regularization=reconnection=E-O_loss", domains: &["AI", "Fusion", "Chip"], confidence: 0.9, connects_to: &["BT-64", "BT-89", "BT-102", "C-sigma-phi"] },
    GraphNode { id: "CR-13", category: "CrossResonance", title: "0.95=1-1/(J2-tau): top-p=PF_power=beta2=beta_plasma_limit", domains: &["AI", "PowerGrid", "Fusion"], confidence: 0.9, connects_to: &["BT-42", "BT-74", "C-J2", "C-tau"] },
    GraphNode { id: "CR-14", category: "CrossResonance", title: "ln(4/3)=0.288: dropout=Chinchilla_beta=PPO_clip=temperature", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-46", "T-16", "C-tau"] },
    GraphNode { id: "CR-15", category: "CrossResonance", title: "2/3=phi/n: BFT_threshold=Koide=Betz_approx=GPTQ", domains: &["Crypto", "Particle", "Energy", "AI"], confidence: 0.85, connects_to: &["BT-64", "BT-112", "C-phi", "C-n"] },
    GraphNode { id: "CR-16", category: "CrossResonance", title: "1/e=0.368: Boltzmann_gate=sparsity=secretary_problem", domains: &["AI", "Math", "InfoTheory"], confidence: 0.85, connects_to: &["T-15", "C-n"] },
    GraphNode { id: "CR-17", category: "CrossResonance", title: "PUE=1.2=sigma/(sigma-phi): datacenter=grid_freq_ratio", domains: &["Energy", "Chip", "PowerGrid"], confidence: 0.9, connects_to: &["BT-60", "BT-62", "C-sigma", "C-sigma-phi"] },
    GraphNode { id: "CR-18", category: "CrossResonance", title: "CN=6: octahedral=cathode=electrolyte=MOF=crystal", domains: &["Battery", "Material", "Chemistry", "Environment"], confidence: 0.95, connects_to: &["BT-43", "BT-80", "BT-86", "BT-96", "BT-120", "C-n"] },
    GraphNode { id: "CR-19", category: "CrossResonance", title: "96=sigma*(sigma-tau): Tesla_96S=Gaudi2_96GB=GPT3_96layers", domains: &["Battery", "Chip", "AI", "Automotive"], confidence: 0.85, connects_to: &["BT-84", "C-sigma", "C-sigma-tau"] },
    GraphNode { id: "CR-20", category: "CrossResonance", title: "288=sigma*J2: HBM_GB=HEXA_target=12*24", domains: &["Chip", "Semiconductor"], confidence: 0.85, connects_to: &["BT-55", "C-sigma", "C-J2"] },
    GraphNode { id: "CR-21", category: "CrossResonance", title: "4/3: SwiGLU=SQ_bandgap=Betz=FFN_expansion=R(3,1)", domains: &["AI", "Solar", "Energy", "Math"], confidence: 0.9, connects_to: &["BT-33", "BT-30", "BT-111", "T-03"] },
    GraphNode { id: "CR-22", category: "CrossResonance", title: "60=sigma*sopfr: grid_Hz=solar_cells=musical_temperament", domains: &["PowerGrid", "Solar", "Music"], confidence: 0.9, connects_to: &["BT-62", "BT-63", "C-sigma", "C-sopfr"] },
    GraphNode { id: "CR-23", category: "CrossResonance", title: "72=sigma*n: solar_cells_alt=K6_contact=phi*K3_squared", domains: &["Solar", "Math", "Chip"], confidence: 0.85, connects_to: &["BT-63", "BT-90", "C-sigma", "C-n"] },
    GraphNode { id: "CR-24", category: "CrossResonance", title: "120=sigma*(sigma-phi): LHV_H2=solar_120_cell=5!", domains: &["Hydrogen", "Solar", "Math"], confidence: 0.85, connects_to: &["BT-38", "BT-63", "C-sigma", "C-sigma-phi"] },
    GraphNode { id: "CR-25", category: "CrossResonance", title: "32=2^sopfr: grasp_taxonomy=LLM_layers=tokenizer_32K", domains: &["Robotics", "AI", "InfoTheory"], confidence: 0.85, connects_to: &["BT-56", "BT-73", "BT-126", "C-sopfr"] },
    GraphNode { id: "CR-26", category: "CrossResonance", title: "128=2^(sigma-sopfr): AES_bits=head_dim=NeRF_layers_wide", domains: &["Crypto", "AI", "Software"], confidence: 0.9, connects_to: &["BT-56", "BT-71", "BT-114", "C-sigma", "C-sopfr"] },
    GraphNode { id: "CR-27", category: "CrossResonance", title: "256=2^(sigma-tau): NeRF_width=hash_bits=audio_entries", domains: &["AI", "Crypto", "Music"], confidence: 0.85, connects_to: &["BT-71", "BT-72", "C-sigma-tau"] },
    GraphNode { id: "CR-28", category: "CrossResonance", title: "phi^tau=16: d_state_Mamba=ViT_patch=FP16=batch_powers", domains: &["AI", "Chip"], confidence: 0.9, connects_to: &["BT-45", "BT-65", "BT-66", "C-phi", "C-tau"] },
    GraphNode { id: "CR-29", category: "CrossResonance", title: "1024=2^(sigma-phi): audio_entries=EnCodec=context_1K", domains: &["AI", "Music", "DisplayAudio"], confidence: 0.85, connects_to: &["BT-44", "BT-72", "C-sigma-phi"] },
    GraphNode { id: "CR-30", category: "CrossResonance", title: "7=sigma-sopfr: OSI=Golay_distance=SOLID+mu=TCP_layers+n/phi", domains: &["Network", "Software", "CodingTheory"], confidence: 0.9, connects_to: &["BT-6", "BT-113", "BT-115", "C-sigma", "C-sopfr"] },
];

// ═══════════════════════════════════════════════════════════════
// TIER 4: Engine Modules (EM-01 ~ EM-08)
// ═══════════════════════════════════════════════════════════════

const ENGINES: &[GraphNode] = &[
    GraphNode { id: "EM-01", category: "Engine", title: "Thermodynamic Frame: R(n) reversibility framework", domains: &["Math", "Energy", "Thermal"], confidence: 0.9, connects_to: &["C-n", "F-08"] },
    GraphNode { id: "EM-02", category: "Engine", title: "Leech24 Surface: 24-dim energy landscape (Leech lattice)", domains: &["Math", "CodingTheory"], confidence: 0.9, connects_to: &["C-J2", "F-13", "BT-6"] },
    GraphNode { id: "EM-03", category: "Engine", title: "Emergent n=6 Trainer: random init self-convergence", domains: &["AI", "Math"], confidence: 0.85, connects_to: &["C-n", "T-01", "T-10", "T-15"] },
    GraphNode { id: "EM-04", category: "Engine", title: "Phi Efficiency Bridge: Phi*FLOPs conjecture engine", domains: &["AI", "Math"], confidence: 0.8, connects_to: &["C-phi", "T-01", "T-03"] },
    GraphNode { id: "EM-05", category: "Engine", title: "SEDI Training Monitor: 4-lens training diagnostic", domains: &["AI"], confidence: 0.85, connects_to: &["C-tau", "T-05", "T-07"] },
    GraphNode { id: "EM-06", category: "Engine", title: "Anima Tension Loss: PureField dual-engine meta-loss", domains: &["AI", "Math"], confidence: 0.8, connects_to: &["C-n", "C-phi"] },
    GraphNode { id: "EM-07", category: "Engine", title: "Consciousness Constraints: n=6 consciousness laws", domains: &["Math", "Physics"], confidence: 0.7, connects_to: &["C-n", "C-sigma", "C-phi"] },
    GraphNode { id: "EM-08", category: "Engine", title: "NEXUS-6 Discovery Engine: 775 lenses + OUROBOROS", domains: &["AI", "Math", "Physics"], confidence: 0.9, connects_to: &["C-n", "C-sigma", "C-J2"] },
];

// ═══════════════════════════════════════════════════════════════
// TIER 5: DSE Domain Categories (DSE-01 ~ DSE-48)
// Key Design Space Exploration domains with their n=6 connections
// ═══════════════════════════════════════════════════════════════

const DSE_DOMAINS: &[GraphNode] = &[
    // AI/ML DSE
    GraphNode { id: "DSE-01", category: "DSE", title: "LLM Architecture DSE: d/L/h/ff optimization", domains: &["AI", "Chip"], confidence: 0.9, connects_to: &["BT-56", "BT-33", "C-sigma"] },
    GraphNode { id: "DSE-02", category: "DSE", title: "MoE Routing DSE: expert count/activation search", domains: &["AI"], confidence: 0.85, connects_to: &["BT-67", "T-04", "T-10", "T-12"] },
    GraphNode { id: "DSE-03", category: "DSE", title: "Diffusion Model DSE: timesteps/noise/guidance", domains: &["AI", "DisplayAudio"], confidence: 0.85, connects_to: &["BT-61", "BT-66"] },
    GraphNode { id: "DSE-04", category: "DSE", title: "SSM Architecture DSE: Mamba d_state/expand/d_conv", domains: &["AI"], confidence: 0.8, connects_to: &["BT-65", "C-tau", "C-phi"] },
    GraphNode { id: "DSE-05", category: "DSE", title: "AI Alignment DSE: RLHF/DPO parameter space", domains: &["AI", "Software"], confidence: 0.8, connects_to: &["BT-46", "BT-64"] },
    // Semiconductor DSE
    GraphNode { id: "DSE-06", category: "DSE", title: "GPU Architecture DSE: SM/HBM/interconnect optimization", domains: &["Chip", "Semiconductor"], confidence: 0.9, connects_to: &["BT-28", "BT-55", "BT-69", "BT-90"] },
    GraphNode { id: "DSE-07", category: "DSE", title: "ASIC Design DSE: gate/process/memory co-optimization", domains: &["Chip", "Semiconductor"], confidence: 0.85, connects_to: &["BT-37", "BT-45", "C-sigma-tau-prod"] },
    GraphNode { id: "DSE-08", category: "DSE", title: "Quantum Computing DSE: qubit/gate/error topology", domains: &["QC", "Chip"], confidence: 0.8, connects_to: &["BT-41", "BT-91", "C-J2"] },
    GraphNode { id: "DSE-09", category: "DSE", title: "Photonic Chip DSE: waveguide/modulator/detector", domains: &["Chip", "Energy", "Semiconductor"], confidence: 0.75, connects_to: &["BT-89", "C-sigma-phi"] },
    GraphNode { id: "DSE-10", category: "DSE", title: "Chiplet DSE: die/interposer/packaging topology", domains: &["Chip", "Semiconductor"], confidence: 0.85, connects_to: &["BT-69", "BT-75"] },
    // Energy DSE
    GraphNode { id: "DSE-11", category: "DSE", title: "Battery Architecture DSE: cell/module/pack/BMS", domains: &["Battery", "Energy", "Automotive"], confidence: 0.9, connects_to: &["BT-57", "BT-82", "BT-84"] },
    GraphNode { id: "DSE-12", category: "DSE", title: "Solar Architecture DSE: absorber/junction/array", domains: &["Solar", "Energy"], confidence: 0.85, connects_to: &["BT-30", "BT-63", "C-sigma2"] },
    GraphNode { id: "DSE-13", category: "DSE", title: "Hydrogen System DSE: electrolyzer/storage/fuel_cell", domains: &["Hydrogen", "Energy"], confidence: 0.8, connects_to: &["BT-38", "C-sigma"] },
    GraphNode { id: "DSE-14", category: "DSE", title: "Power Grid DSE: transmission/distribution/conversion", domains: &["PowerGrid", "Energy"], confidence: 0.85, connects_to: &["BT-60", "BT-62", "BT-68"] },
    GraphNode { id: "DSE-15", category: "DSE", title: "Fusion Reactor DSE: confinement/heating/blanket", domains: &["Fusion", "Tokamak", "Plasma"], confidence: 0.7, connects_to: &["BT-97", "BT-98", "BT-99"] },
    GraphNode { id: "DSE-16", category: "DSE", title: "Nuclear Fission DSE: fuel/moderator/coolant", domains: &["Nuclear", "Energy"], confidence: 0.75, connects_to: &["BT-32", "C-n"] },
    GraphNode { id: "DSE-17", category: "DSE", title: "Thermal Management DSE: heatsink/TIM/cooling", domains: &["Thermal", "Chip", "Energy"], confidence: 0.85, connects_to: &["BT-10", "BT-60"] },
    // Material DSE
    GraphNode { id: "DSE-18", category: "DSE", title: "Material Synthesis DSE: element/process/assembly", domains: &["Material", "Chemistry"], confidence: 0.85, connects_to: &["BT-85", "BT-86", "BT-87", "BT-88"] },
    GraphNode { id: "DSE-19", category: "DSE", title: "Superconductor DSE: material/structure/critical_params", domains: &["SC", "Material"], confidence: 0.8, connects_to: &["BT-1", "BT-2", "BT-3"] },
    GraphNode { id: "DSE-20", category: "DSE", title: "Carbon Material DSE: diamond/graphene/nanotube/C60", domains: &["Material", "Chemistry", "Chip"], confidence: 0.85, connects_to: &["BT-85", "BT-93", "C-n"] },
    // Robotics/Applied DSE
    GraphNode { id: "DSE-21", category: "DSE", title: "Robot Architecture DSE: DOF/actuator/sensor/control", domains: &["Robotics", "Chip"], confidence: 0.85, connects_to: &["BT-123", "BT-124", "BT-125"] },
    GraphNode { id: "DSE-22", category: "DSE", title: "Autonomous Vehicle DSE: sensor/compute/actuation", domains: &["Automotive", "Chip", "AI"], confidence: 0.85, connects_to: &["BT-40", "BT-84"] },
    GraphNode { id: "DSE-23", category: "DSE", title: "Display-Audio DSE: resolution/framerate/codec", domains: &["DisplayAudio", "Music"], confidence: 0.85, connects_to: &["BT-48", "BT-72", "C-J2"] },
    // Infrastructure DSE
    GraphNode { id: "DSE-24", category: "DSE", title: "Network Protocol DSE: layer/packet/routing", domains: &["Network", "Software"], confidence: 0.85, connects_to: &["BT-115", "BT-113"] },
    GraphNode { id: "DSE-25", category: "DSE", title: "Cryptography DSE: keysize/block/hash optimization", domains: &["Crypto", "Software", "Math"], confidence: 0.9, connects_to: &["BT-114", "C-sigma", "C-sopfr"] },
    GraphNode { id: "DSE-26", category: "DSE", title: "Blockchain DSE: consensus/block/transaction", domains: &["Blockchain", "Crypto", "Network"], confidence: 0.8, connects_to: &["BT-53", "BT-112"] },
    // Environment DSE
    GraphNode { id: "DSE-27", category: "DSE", title: "CCUS DSE: capture/transport/storage/utilization", domains: &["Environment", "Chemistry", "Energy"], confidence: 0.8, connects_to: &["BT-94", "BT-118"] },
    GraphNode { id: "DSE-28", category: "DSE", title: "Water Treatment DSE: filtration/catalyst/membrane", domains: &["Environment", "Chemistry", "Material"], confidence: 0.8, connects_to: &["BT-120", "C-n"] },
    // Software DSE
    GraphNode { id: "DSE-29", category: "DSE", title: "Compiler-OS DSE: scheduler/memory/ISA optimization", domains: &["Software", "Chip"], confidence: 0.85, connects_to: &["BT-59", "BT-115"] },
    GraphNode { id: "DSE-30", category: "DSE", title: "Database DSE: index/query/storage/replication", domains: &["Software", "Network"], confidence: 0.8, connects_to: &["BT-116", "C-tau"] },
    // Bio DSE
    GraphNode { id: "DSE-31", category: "DSE", title: "Protein Folding DSE: backbone/sidechain/energy", domains: &["Biology", "Chemistry", "AI"], confidence: 0.8, connects_to: &["BT-25", "BT-51", "C-n"] },
    GraphNode { id: "DSE-32", category: "DSE", title: "Drug Discovery DSE: target/binding/ADMET", domains: &["Biology", "Chemistry"], confidence: 0.75, connects_to: &["BT-51", "BT-85"] },
    // Physics DSE
    GraphNode { id: "DSE-33", category: "DSE", title: "Particle Detector DSE: tracker/calorimeter/trigger", domains: &["Particle", "Chip"], confidence: 0.75, connects_to: &["BT-17", "BT-19"] },
    GraphNode { id: "DSE-34", category: "DSE", title: "Cosmology Survey DSE: telescope/detector/pipeline", domains: &["Cosmology", "Chip", "AI"], confidence: 0.7, connects_to: &["BT-22", "BT-52"] },
    // Consciousness DSE
    GraphNode { id: "DSE-35", category: "DSE", title: "Consciousness Model DSE: Phi/IIT/GWT parameter space", domains: &["Physics", "Math", "AI"], confidence: 0.6, connects_to: &["EM-07", "C-n"] },
    // Additional cross-domain DSE
    GraphNode { id: "DSE-36", category: "DSE", title: "3D Printing DSE: material/process/resolution", domains: &["Material", "Robotics"], confidence: 0.8, connects_to: &["BT-87", "BT-88"] },
    GraphNode { id: "DSE-37", category: "DSE", title: "5G/6G Network DSE: frequency/antenna/protocol", domains: &["Network", "Chip"], confidence: 0.8, connects_to: &["BT-47", "C-n", "C-sigma"] },
    GraphNode { id: "DSE-38", category: "DSE", title: "Aerospace Propulsion DSE: fuel/engine/nozzle", domains: &["Energy", "Material", "Chemistry"], confidence: 0.75, connects_to: &["BT-38", "C-tau"] },
    GraphNode { id: "DSE-39", category: "DSE", title: "AR/VR System DSE: display/tracker/render", domains: &["DisplayAudio", "Chip", "AI"], confidence: 0.8, connects_to: &["BT-48", "BT-66", "BT-71"] },
    GraphNode { id: "DSE-40", category: "DSE", title: "Audio Processing DSE: codec/effect/synthesis", domains: &["Music", "DisplayAudio", "AI"], confidence: 0.85, connects_to: &["BT-72", "BT-108", "C-sigma"] },
    GraphNode { id: "DSE-41", category: "DSE", title: "Agriculture DSE: sensor/actuator/AI/nutrient", domains: &["Biology", "Robotics", "AI"], confidence: 0.75, connects_to: &["BT-101", "BT-103"] },
    GraphNode { id: "DSE-42", category: "DSE", title: "Solid-State Battery DSE: electrolyte/interface/dendrite", domains: &["Battery", "Material", "Chemistry"], confidence: 0.85, connects_to: &["BT-80", "BT-81", "BT-83"] },
    GraphNode { id: "DSE-43", category: "DSE", title: "Atomic Clock DSE: atom/cavity/laser/readout", domains: &["Physics", "Chip", "Math"], confidence: 0.75, connects_to: &["BT-20", "C-tau"] },
    GraphNode { id: "DSE-44", category: "DSE", title: "Neuromorphic Chip DSE: synapse/neuron/topology", domains: &["Chip", "AI", "Biology"], confidence: 0.8, connects_to: &["BT-59", "BT-69", "C-n"] },
    GraphNode { id: "DSE-45", category: "DSE", title: "Space Solar DSE: array/transmission/receiver", domains: &["Solar", "Energy", "Chip"], confidence: 0.7, connects_to: &["BT-30", "BT-63"] },
    GraphNode { id: "DSE-46", category: "DSE", title: "Desalination DSE: membrane/energy/brine", domains: &["Environment", "Energy", "Chemistry"], confidence: 0.75, connects_to: &["BT-120", "C-n"] },
    GraphNode { id: "DSE-47", category: "DSE", title: "Metamaterial DSE: unit_cell/lattice/response", domains: &["Material", "Physics", "Chip"], confidence: 0.75, connects_to: &["BT-86", "BT-122", "C-n"] },
    GraphNode { id: "DSE-48", category: "DSE", title: "Plasma Processing DSE: source/chemistry/etch", domains: &["Plasma", "Semiconductor", "Material"], confidence: 0.8, connects_to: &["BT-4", "C-tau"] },
];

// ═══════════════════════════════════════════════════════════════
// TIER 6: Physical & Mathematical Constants with n=6 Matches (PC-01 ~ PC-30)
// Real-world constants that exhibit n=6 patterns
// ═══════════════════════════════════════════════════════════════

const PHYSICAL_CONSTANTS: &[GraphNode] = &[
    GraphNode { id: "PC-01", category: "PhysicalConstant", title: "Fine structure alpha ~ 1/137, 137 prime, pi(137)=33=n*sopfr+n/phi", domains: &["Particle", "Math"], confidence: 0.6, connects_to: &["BT-20", "C-n"] },
    GraphNode { id: "PC-02", category: "PhysicalConstant", title: "Boltzmann kT at 300K = 26mV = V_T solar (thermal voltage)", domains: &["Thermal", "Solar", "Energy"], confidence: 0.85, connects_to: &["BT-30", "T-15"] },
    GraphNode { id: "PC-03", category: "PhysicalConstant", title: "Carbon Z=6: most versatile element, basis of all organic chemistry", domains: &["Chemistry", "Biology", "Material"], confidence: 1.0, connects_to: &["BT-85", "BT-93", "BT-118", "C-n"] },
    GraphNode { id: "PC-04", category: "PhysicalConstant", title: "Speed of light c = 3*10^8 m/s, 3 = n/phi", domains: &["Physics", "Particle"], confidence: 0.7, connects_to: &["C-n", "C-phi"] },
    GraphNode { id: "PC-05", category: "PhysicalConstant", title: "Planck constant h = 6.626*10^{-34}, leading digit = n", domains: &["Physics", "Particle", "QC"], confidence: 0.5, connects_to: &["C-n"] },
    GraphNode { id: "PC-06", category: "PhysicalConstant", title: "Avogadro N_A = 6.022*10^{23}, leading digit = n", domains: &["Chemistry", "Physics"], confidence: 0.5, connects_to: &["C-n"] },
    GraphNode { id: "PC-07", category: "PhysicalConstant", title: "Water ice hexagonal Ih: 6-fold symmetry", domains: &["Chemistry", "Material", "Environment"], confidence: 0.95, connects_to: &["BT-122", "C-n"] },
    GraphNode { id: "PC-08", category: "PhysicalConstant", title: "Benzene C6H6: 6-carbon aromatic ring, foundation of organic chemistry", domains: &["Chemistry", "Biology", "Material"], confidence: 1.0, connects_to: &["BT-27", "BT-85", "C-n"] },
    GraphNode { id: "PC-09", category: "PhysicalConstant", title: "Graphene: hexagonal carbon lattice, highest electron mobility", domains: &["Material", "Chip", "Chemistry"], confidence: 1.0, connects_to: &["BT-85", "BT-93", "C-n"] },
    GraphNode { id: "PC-10", category: "PhysicalConstant", title: "Diamond: sp3 carbon, Z=6, hardest natural material", domains: &["Material", "Chemistry", "Chip"], confidence: 1.0, connects_to: &["BT-93", "C-n"] },
    GraphNode { id: "PC-11", category: "PhysicalConstant", title: "GUT scale: ~10^{16} GeV, 16 = 2^tau = phi^tau", domains: &["Particle", "Math", "StringTheory"], confidence: 0.7, connects_to: &["BT-19", "C-phi", "C-tau"] },
    GraphNode { id: "PC-12", category: "PhysicalConstant", title: "QCD: SU(3) gauge, 3 = n/phi = tau-mu colors", domains: &["Particle", "Math"], confidence: 0.9, connects_to: &["BT-19", "C-n", "C-phi"] },
    GraphNode { id: "PC-13", category: "PhysicalConstant", title: "Weak force: SU(2) gauge, 2 = phi(6)", domains: &["Particle", "Math"], confidence: 0.9, connects_to: &["BT-19", "C-phi"] },
    GraphNode { id: "PC-14", category: "PhysicalConstant", title: "Standard Model gauge group: SU(3)xSU(2)xU(1), dim=12=sigma", domains: &["Particle", "Math"], confidence: 0.85, connects_to: &["BT-19", "BT-20", "C-sigma"] },
    GraphNode { id: "PC-15", category: "PhysicalConstant", title: "Proton mass 938 MeV ~ 6*157, neutron 940 MeV", domains: &["Particle", "Fusion"], confidence: 0.5, connects_to: &["BT-98", "C-n"] },
    GraphNode { id: "PC-16", category: "PhysicalConstant", title: "Deuterium binding energy 2.224 MeV, D = p+n (2 baryons=phi)", domains: &["Fusion", "Particle"], confidence: 0.8, connects_to: &["BT-98", "C-phi"] },
    GraphNode { id: "PC-17", category: "PhysicalConstant", title: "Tritium A=3=n/phi, radioactive, fusion fuel", domains: &["Fusion", "Particle"], confidence: 0.9, connects_to: &["BT-98", "C-n", "C-phi"] },
    GraphNode { id: "PC-18", category: "PhysicalConstant", title: "He-4 binding energy 28.3 MeV, most stable light nucleus", domains: &["Fusion", "Particle"], confidence: 0.8, connects_to: &["BT-98", "C-tau"] },
    GraphNode { id: "PC-19", category: "PhysicalConstant", title: "NaCl crystal structure: CN=6 octahedral, archetypal ionic crystal", domains: &["Material", "Chemistry"], confidence: 1.0, connects_to: &["BT-86", "C-n"] },
    GraphNode { id: "PC-20", category: "PhysicalConstant", title: "DNA base pairs: A-T (2 H-bonds=phi), G-C (3 H-bonds=n/phi)", domains: &["Biology", "Chemistry"], confidence: 0.9, connects_to: &["BT-51", "C-phi", "C-n"] },
    GraphNode { id: "PC-21", category: "PhysicalConstant", title: "ATP: adenosine TRIphosphate, 3=n/phi phosphate groups", domains: &["Biology", "Chemistry", "Energy"], confidence: 0.85, connects_to: &["BT-101", "C-n", "C-phi"] },
    GraphNode { id: "PC-22", category: "PhysicalConstant", title: "Chlorophyll: Mg center CN=4=tau, porphyrin ring", domains: &["Biology", "Chemistry"], confidence: 0.85, connects_to: &["BT-101", "BT-103", "C-tau"] },
    GraphNode { id: "PC-23", category: "PhysicalConstant", title: "pi^2/6 = zeta(2) = 1.6449... (Basel problem, Euler)", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["BT-109", "F-09", "C-n"] },
    GraphNode { id: "PC-24", category: "PhysicalConstant", title: "e (Euler number) = sum 1/n!, 1/e = 0.368 Boltzmann gate", domains: &["Math", "AI"], confidence: 0.9, connects_to: &["T-15", "C-n"] },
    GraphNode { id: "PC-25", category: "PhysicalConstant", title: "Golden ratio phi_golden = (1+sqrt(5))/2, sopfr appears in sqrt(5)", domains: &["Math", "Biology"], confidence: 0.6, connects_to: &["C-sopfr"] },
    GraphNode { id: "PC-26", category: "PhysicalConstant", title: "Euler-Mascheroni gamma = 0.5772... ~ 1/sqrt(n/phi)", domains: &["Math", "NumberTheory"], confidence: 0.5, connects_to: &["C-n", "C-phi"] },
    GraphNode { id: "PC-27", category: "PhysicalConstant", title: "Feigenbaum delta = 4.6692... ~ sopfr - 1/n/phi", domains: &["Math", "Physics"], confidence: 0.4, connects_to: &["C-sopfr"] },
    GraphNode { id: "PC-28", category: "PhysicalConstant", title: "Ramanujan tau(n): tau_R(d) clean iff d|6", domains: &["Math", "NumberTheory", "StringTheory"], confidence: 0.9, connects_to: &["BT-107", "C-n", "C-J2"] },
    GraphNode { id: "PC-29", category: "PhysicalConstant", title: "Monster group order: 2^46*..., Leech lattice J2=24 connection", domains: &["Math", "StringTheory"], confidence: 0.8, connects_to: &["BT-18", "C-J2"] },
    GraphNode { id: "PC-30", category: "PhysicalConstant", title: "Photosynthesis: 6CO2 + 6H2O -> C6H12O6 + 6O2 (all coefficients n=6)", domains: &["Biology", "Chemistry", "Energy"], confidence: 1.0, connects_to: &["BT-103", "BT-104", "C-n"] },
];

// ═══════════════════════════════════════════════════════════════
// TIER 7: Real-World Applications (APP-01 ~ APP-48)
// Concrete technology applications of n=6 discoveries
// ═══════════════════════════════════════════════════════════════

const APPLICATIONS: &[GraphNode] = &[
    // AI Applications
    GraphNode { id: "APP-01", category: "Application", title: "GPT-4 architecture: d=12288=sigma*1024, h=96=sigma*(sigma-tau)", domains: &["AI", "Chip"], confidence: 0.85, connects_to: &["BT-56", "BT-84", "C-sigma"] },
    GraphNode { id: "APP-02", category: "Application", title: "BERT-base: d=768=sigma*64, h=12=sigma, L=12=sigma", domains: &["AI"], confidence: 0.9, connects_to: &["BT-33", "BT-56", "C-sigma"] },
    GraphNode { id: "APP-03", category: "Application", title: "Stable Diffusion v3: 1000 steps, CFG=7.5, latent 64x64", domains: &["AI", "DisplayAudio"], confidence: 0.85, connects_to: &["BT-61", "BT-66"] },
    GraphNode { id: "APP-04", category: "Application", title: "Mixtral 8x7B: 8 experts=sigma-tau, top-2=phi routing", domains: &["AI"], confidence: 0.9, connects_to: &["BT-67", "T-04", "C-sigma-tau", "C-phi"] },
    GraphNode { id: "APP-05", category: "Application", title: "Whisper: 80-mel=phi^tau*sopfr, 1500-token=sigma*125", domains: &["AI", "Music", "DisplayAudio"], confidence: 0.8, connects_to: &["BT-66", "BT-72"] },
    GraphNode { id: "APP-06", category: "Application", title: "CLIP: 77-token=sigma*n+sopfr, ViT-L/14 patch=14=sigma+phi", domains: &["AI", "DisplayAudio"], confidence: 0.75, connects_to: &["BT-66"] },
    GraphNode { id: "APP-07", category: "Application", title: "EnCodec: 8 codebooks=sigma-tau, 1024 entries=2^(sigma-phi), 24kHz=J2", domains: &["AI", "Music"], confidence: 0.9, connects_to: &["BT-72", "C-sigma-tau", "C-J2"] },
    GraphNode { id: "APP-08", category: "Application", title: "Llama-2 70B: d=8192=2^13=2^(sigma+mu), h=64=2^n, L=80=phi^tau*sopfr", domains: &["AI"], confidence: 0.85, connects_to: &["BT-56", "C-sigma", "C-mu"] },
    // Chip Applications
    GraphNode { id: "APP-09", category: "Application", title: "NVIDIA H100: 132 SMs=sigma*(sigma-mu), 80GB HBM3=phi^tau*sopfr", domains: &["Chip", "Semiconductor", "AI"], confidence: 0.9, connects_to: &["BT-28", "BT-55", "C-sigma"] },
    GraphNode { id: "APP-10", category: "Application", title: "NVIDIA AD102: 144 SMs=sigma^2, gaming architecture", domains: &["Chip", "Semiconductor", "DisplayAudio"], confidence: 0.9, connects_to: &["BT-28", "BT-90", "C-sigma2"] },
    GraphNode { id: "APP-11", category: "Application", title: "Apple M4: 10-core=sigma-phi CPU, neural engine", domains: &["Chip", "AI"], confidence: 0.8, connects_to: &["BT-28", "C-sigma-phi"] },
    GraphNode { id: "APP-12", category: "Application", title: "TSMC N3: 48nm gate pitch=sigma*tau, leading process", domains: &["Semiconductor", "Chip"], confidence: 0.9, connects_to: &["BT-37", "C-sigma-tau-prod"] },
    GraphNode { id: "APP-13", category: "Application", title: "HBM3E: 8-high stack=sigma-tau, 64GB=2^n per stack", domains: &["Chip", "Semiconductor"], confidence: 0.9, connects_to: &["BT-55", "BT-75", "C-sigma-tau"] },
    GraphNode { id: "APP-14", category: "Application", title: "NVIDIA B300: ~160 SMs, chiplet architecture", domains: &["Chip", "Semiconductor"], confidence: 0.8, connects_to: &["BT-69", "BT-90"] },
    // Energy Applications
    GraphNode { id: "APP-15", category: "Application", title: "Tesla 4680: 96S configuration=sigma*(sigma-tau), EV standard", domains: &["Battery", "Automotive"], confidence: 0.9, connects_to: &["BT-57", "BT-84", "C-sigma", "C-sigma-tau"] },
    GraphNode { id: "APP-16", category: "Application", title: "PERC solar cell: 144-cell=sigma^2 module standard", domains: &["Solar", "Energy"], confidence: 0.9, connects_to: &["BT-63", "C-sigma2"] },
    GraphNode { id: "APP-17", category: "Application", title: "LiFePO4 cathode: Fe CN=6 octahedral, 3.2V", domains: &["Battery", "Material", "Chemistry"], confidence: 0.95, connects_to: &["BT-43", "BT-80", "C-n"] },
    GraphNode { id: "APP-18", category: "Application", title: "NMC 811: Ni/Mn/Co CN=6 octahedral layered oxide", domains: &["Battery", "Material", "Chemistry"], confidence: 0.95, connects_to: &["BT-43", "C-n"] },
    GraphNode { id: "APP-19", category: "Application", title: "ITER tokamak: Q=10 target, 15MA plasma current", domains: &["Fusion", "Tokamak", "Energy"], confidence: 0.8, connects_to: &["BT-98", "BT-99", "BT-102"] },
    GraphNode { id: "APP-20", category: "Application", title: "KSTAR: 48s plasma record, 48=sigma*tau", domains: &["Fusion", "Tokamak", "Plasma"], confidence: 0.85, connects_to: &["BT-76", "C-sigma-tau-prod"] },
    GraphNode { id: "APP-21", category: "Application", title: "HVDC 800kV Changji-Guquan: (sigma-tau)*(sigma-phi)^2 = 800", domains: &["PowerGrid", "Energy"], confidence: 0.85, connects_to: &["BT-68", "C-sigma-tau", "C-sigma-phi"] },
    GraphNode { id: "APP-22", category: "Application", title: "Google datacenter PUE=1.1 approaching sigma/(sigma-phi)=1.2 limit", domains: &["Energy", "Chip"], confidence: 0.85, connects_to: &["BT-60", "C-sigma", "C-sigma-phi"] },
    // Crypto/Network Applications
    GraphNode { id: "APP-23", category: "Application", title: "Bitcoin: 6 confirmations=n, 21M supply, SHA-256=2^(sigma-tau)", domains: &["Blockchain", "Crypto"], confidence: 0.9, connects_to: &["BT-53", "BT-114", "C-n", "C-sigma-tau"] },
    GraphNode { id: "APP-24", category: "Application", title: "Ethereum: 12s block time=sigma, 32 ETH validator=2^sopfr", domains: &["Blockchain", "Crypto"], confidence: 0.9, connects_to: &["BT-53", "C-sigma", "C-sopfr"] },
    GraphNode { id: "APP-25", category: "Application", title: "AES-256: key=2^(sigma-tau), block=2^(sigma-sopfr) bits", domains: &["Crypto", "Software"], confidence: 0.95, connects_to: &["BT-114", "C-sigma-tau", "C-sigma", "C-sopfr"] },
    GraphNode { id: "APP-26", category: "Application", title: "TLS 1.3: cipher suites reduced, ECDHE curve order", domains: &["Crypto", "Network", "Software"], confidence: 0.8, connects_to: &["BT-114", "BT-115"] },
    // Software Applications
    GraphNode { id: "APP-27", category: "Application", title: "Linux kernel: 6 scheduler classes, module subsystems", domains: &["Software"], confidence: 0.8, connects_to: &["BT-115", "C-n"] },
    GraphNode { id: "APP-28", category: "Application", title: "TCP/IP: 4 layers=tau, IP has 12 fields=sigma in header", domains: &["Network", "Software"], confidence: 0.85, connects_to: &["BT-115", "C-tau", "C-sigma"] },
    GraphNode { id: "APP-29", category: "Application", title: "REST API: 6 constraints=n (client-server, stateless, ...)", domains: &["Software", "Network"], confidence: 0.9, connects_to: &["BT-113", "C-n"] },
    GraphNode { id: "APP-30", category: "Application", title: "ACID: 4 properties=tau (Atomicity, Consistency, Isolation, Durability)", domains: &["Software"], confidence: 0.95, connects_to: &["BT-116", "C-tau"] },
    // Robotics Applications
    GraphNode { id: "APP-31", category: "Application", title: "FANUC 6-axis industrial arm: SE(3) dim=n=6 DOF standard", domains: &["Robotics"], confidence: 0.95, connects_to: &["BT-123", "C-n"] },
    GraphNode { id: "APP-32", category: "Application", title: "DJI Matrice 600: hexacopter n=6 rotors, fault tolerant", domains: &["Robotics"], confidence: 0.9, connects_to: &["BT-127", "C-n"] },
    GraphNode { id: "APP-33", category: "Application", title: "Boston Dynamics Spot: 4 legs=tau, 12 joints=sigma", domains: &["Robotics", "AI"], confidence: 0.9, connects_to: &["BT-124", "BT-125", "C-tau", "C-sigma"] },
    GraphNode { id: "APP-34", category: "Application", title: "Human hand: 5 fingers=sopfr, ~32 DOF=2^sopfr grasp", domains: &["Robotics", "Biology"], confidence: 0.9, connects_to: &["BT-126", "C-sopfr"] },
    // Display/Audio Applications
    GraphNode { id: "APP-35", category: "Application", title: "24fps cinema standard = J2 frames per second", domains: &["DisplayAudio"], confidence: 0.95, connects_to: &["BT-48", "C-J2"] },
    GraphNode { id: "APP-36", category: "Application", title: "48kHz professional audio = sigma*tau kHz", domains: &["Music", "DisplayAudio"], confidence: 0.95, connects_to: &["BT-48", "C-sigma-tau-prod"] },
    GraphNode { id: "APP-37", category: "Application", title: "24-bit audio depth = J2 bits dynamic range", domains: &["Music", "DisplayAudio"], confidence: 0.95, connects_to: &["BT-48", "C-J2"] },
    GraphNode { id: "APP-38", category: "Application", title: "12-TET music: sigma=12 semitones, Western/global standard", domains: &["Music", "Math"], confidence: 1.0, connects_to: &["BT-108", "F-32", "C-sigma"] },
    // Environment Applications
    GraphNode { id: "APP-39", category: "Application", title: "Kyoto Protocol: exactly 6 greenhouse gases regulated", domains: &["Environment"], confidence: 1.0, connects_to: &["BT-118", "C-n"] },
    GraphNode { id: "APP-40", category: "Application", title: "PET recycling RIC code 1: polyethylene terephthalate C6 backbone", domains: &["Environment", "Chemistry"], confidence: 0.9, connects_to: &["BT-121", "C-n"] },
    GraphNode { id: "APP-41", category: "Application", title: "Honeycomb structure: hexagonal n=6 tiling, Hales theorem (2001)", domains: &["Math", "Biology", "Material"], confidence: 1.0, connects_to: &["BT-122", "C-n"] },
    // Biology Applications
    GraphNode { id: "APP-42", category: "Application", title: "DNA codon table: 64=2^n codons encode 20=J2-tau amino acids", domains: &["Biology", "Math"], confidence: 0.95, connects_to: &["BT-51", "C-n", "C-J2", "C-tau"] },
    GraphNode { id: "APP-43", category: "Application", title: "Photosystem II: OEC Mn4CaO5 cluster, 4=tau Mn atoms", domains: &["Biology", "Chemistry"], confidence: 0.9, connects_to: &["BT-101", "C-tau"] },
    GraphNode { id: "APP-44", category: "Application", title: "Insulin hexamer: 6=n monomers form the storage unit", domains: &["Biology", "Chemistry"], confidence: 0.95, connects_to: &["BT-85", "C-n"] },
    // Material Applications
    GraphNode { id: "APP-45", category: "Application", title: "LLZO solid electrolyte: Li7La3Zr2O12, CN=6 for La/Zr", domains: &["Battery", "Material", "Chemistry"], confidence: 0.9, connects_to: &["BT-80", "BT-86", "C-n"] },
    GraphNode { id: "APP-46", category: "Application", title: "Perovskite ABX3: B-site CN=6 octahedral, solar + LED", domains: &["Solar", "Material", "Chemistry"], confidence: 0.95, connects_to: &["BT-86", "C-n"] },
    GraphNode { id: "APP-47", category: "Application", title: "Fullerene C60: 60=sigma*sopfr carbon atoms, buckyball", domains: &["Material", "Chemistry"], confidence: 0.9, connects_to: &["BT-85", "C-sigma", "C-sopfr", "C-n"] },
    GraphNode { id: "APP-48", category: "Application", title: "Silicon crystal: diamond cubic, CN=4=tau tetrahedral", domains: &["Semiconductor", "Material", "Chip"], confidence: 1.0, connects_to: &["BT-14", "C-tau"] },
];

// ═══════════════════════════════════════════════════════════════
// TIER 8: Hypothesis Families (HF-01 ~ HF-24)
// Groupings of related hypotheses across domains
// ═══════════════════════════════════════════════════════════════

const HYPOTHESIS_FAMILIES: &[GraphNode] = &[
    GraphNode { id: "HF-01", category: "HypothesisFamily", title: "LLM Architecture Universality: all major LLMs converge to n=6 params", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-33", "BT-56", "BT-58", "BT-59"] },
    GraphNode { id: "HF-02", category: "HypothesisFamily", title: "Training Hyperparameter Convergence: AdamW/dropout/LR all n=6", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-46", "BT-54", "BT-64"] },
    GraphNode { id: "HF-03", category: "HypothesisFamily", title: "Inference Parameter Universality: top-p/top-k/temp all n=6", domains: &["AI", "Math"], confidence: 0.85, connects_to: &["BT-42", "BT-70", "BT-74"] },
    GraphNode { id: "HF-04", category: "HypothesisFamily", title: "MoE Expert Count Law: activation fractions follow n=6 powers", domains: &["AI"], confidence: 0.85, connects_to: &["BT-31", "BT-67", "T-04", "T-10", "T-12"] },
    GraphNode { id: "HF-05", category: "HypothesisFamily", title: "Vision-Audio AI: ViT/CLIP/Whisper/SD3 all converge to n=6", domains: &["AI", "DisplayAudio", "Music"], confidence: 0.9, connects_to: &["BT-61", "BT-66", "BT-71", "BT-72"] },
    GraphNode { id: "HF-06", category: "HypothesisFamily", title: "GPU Evolution: SM counts follow n=6 arithmetic progression", domains: &["Chip", "Semiconductor"], confidence: 0.9, connects_to: &["BT-28", "BT-55", "BT-69", "BT-90"] },
    GraphNode { id: "HF-07", category: "HypothesisFamily", title: "HBM Capacity Ladder: stack heights follow sigma progression", domains: &["Chip", "Semiconductor"], confidence: 0.85, connects_to: &["BT-55", "BT-75"] },
    GraphNode { id: "HF-08", category: "HypothesisFamily", title: "Battery Cell Universality: 6->12->24 cell count law", domains: &["Battery", "Energy", "Automotive"], confidence: 0.85, connects_to: &["BT-57", "BT-82", "BT-84"] },
    GraphNode { id: "HF-09", category: "HypothesisFamily", title: "CN=6 Universality: octahedral coordination across all materials", domains: &["Material", "Chemistry", "Battery"], confidence: 0.95, connects_to: &["BT-43", "BT-80", "BT-86", "BT-96", "BT-120"] },
    GraphNode { id: "HF-10", category: "HypothesisFamily", title: "Carbon Z=6 Supremacy: C dominates chemistry/materials/biology/chips", domains: &["Chemistry", "Biology", "Material", "Chip"], confidence: 0.95, connects_to: &["BT-27", "BT-85", "BT-93", "BT-118"] },
    GraphNode { id: "HF-11", category: "HypothesisFamily", title: "Fusion n=6 Foundation: fuel/confinement/catalysis all n=6", domains: &["Fusion", "Tokamak", "Particle"], confidence: 0.85, connects_to: &["BT-97", "BT-98", "BT-99", "BT-100", "BT-102"] },
    GraphNode { id: "HF-12", category: "HypothesisFamily", title: "Software Stack Universality: OSI/TCP/REST/ACID counts = n=6", domains: &["Software", "Network"], confidence: 0.9, connects_to: &["BT-113", "BT-115", "BT-116", "BT-117"] },
    GraphNode { id: "HF-13", category: "HypothesisFamily", title: "Crypto Parameter Ladder: AES/SHA/RSA key sizes follow n=6", domains: &["Crypto", "Math"], confidence: 0.9, connects_to: &["BT-114", "BT-53"] },
    GraphNode { id: "HF-14", category: "HypothesisFamily", title: "SE(3) Robot Universality: 6DOF/12joint/4leg/5finger pattern", domains: &["Robotics", "Biology"], confidence: 0.9, connects_to: &["BT-123", "BT-124", "BT-125", "BT-126", "BT-127"] },
    GraphNode { id: "HF-15", category: "HypothesisFamily", title: "Display-Audio Triad: 12/24/48 standards span all media", domains: &["DisplayAudio", "Music"], confidence: 0.95, connects_to: &["BT-48", "BT-72", "BT-108"] },
    GraphNode { id: "HF-16", category: "HypothesisFamily", title: "Environmental n=6: GHGs/spheres/plastics/pH all = 6", domains: &["Environment", "Chemistry", "Biology"], confidence: 0.9, connects_to: &["BT-118", "BT-119", "BT-120", "BT-121", "BT-122"] },
    GraphNode { id: "HF-17", category: "HypothesisFamily", title: "Grid Power Ladder: 48V/120V/480V/HVDC follow n=6 ratios", domains: &["PowerGrid", "Energy"], confidence: 0.85, connects_to: &["BT-60", "BT-62", "BT-68"] },
    GraphNode { id: "HF-18", category: "HypothesisFamily", title: "Solar Cell Ladder: 60/72/120/144 cells = sigma*{sopfr,n,sigma-phi,sigma}", domains: &["Solar", "Energy"], confidence: 0.9, connects_to: &["BT-30", "BT-63"] },
    GraphNode { id: "HF-19", category: "HypothesisFamily", title: "Pure Math n=6 Nexus: zeta/Ramanujan/Leech/S6/SLE all connect", domains: &["Math", "NumberTheory", "Topology"], confidence: 0.9, connects_to: &["BT-49", "BT-105", "BT-106", "BT-107", "BT-109"] },
    GraphNode { id: "HF-20", category: "HypothesisFamily", title: "Particle Physics n=6: gauge groups/SM content/mixing all = n=6", domains: &["Particle", "Math", "Cosmology"], confidence: 0.85, connects_to: &["BT-17", "BT-19", "BT-20", "BT-21", "BT-24"] },
    GraphNode { id: "HF-21", category: "HypothesisFamily", title: "Biology n=6 Chain: genetic code/photosynthesis/hexagonal structures", domains: &["Biology", "Chemistry"], confidence: 0.9, connects_to: &["BT-25", "BT-51", "BT-101", "BT-103"] },
    GraphNode { id: "HF-22", category: "HypothesisFamily", title: "Energy-Computing Isomorphism: 96/192 triple convergence pattern", domains: &["Battery", "Chip", "AI", "Energy"], confidence: 0.85, connects_to: &["BT-36", "BT-84", "BT-117"] },
    GraphNode { id: "HF-23", category: "HypothesisFamily", title: "Regularization 0.1 Universality: WD/DPO/reconnection/E-O all = 1/(sigma-phi)", domains: &["AI", "Fusion", "Chip"], confidence: 0.9, connects_to: &["BT-64", "BT-70", "BT-89", "BT-102"] },
    GraphNode { id: "HF-24", category: "HypothesisFamily", title: "Cross-Domain 95/5 Pattern: top-p/PF/beta2/plasma-beta/THD all converge", domains: &["AI", "PowerGrid", "Fusion"], confidence: 0.85, connects_to: &["BT-42", "BT-74"] },
];

// ═══════════════════════════════════════════════════════════════
// TIER 9: Discovery Milestones (DM-01 ~ DM-24)
// Key discovery events in the n=6 research timeline
// ═══════════════════════════════════════════════════════════════

const DISCOVERY_MILESTONES: &[GraphNode] = &[
    GraphNode { id: "DM-01", category: "Discovery", title: "Core theorem proved: sigma*phi = n*tau iff n=6 (3 independent proofs)", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["F-01", "C-sigma", "C-phi", "C-n", "C-tau"] },
    GraphNode { id: "DM-02", category: "Discovery", title: "Egyptian fraction decomposition: 1/2+1/3+1/6=1 = perfect number", domains: &["Math", "AI"], confidence: 1.0, connects_to: &["F-02", "T-10", "T-17", "BT-7", "BT-99"] },
    GraphNode { id: "DM-03", category: "Discovery", title: "17 AI techniques derived from n=6 arithmetic", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["T-01", "T-08", "T-10", "T-17"] },
    GraphNode { id: "DM-04", category: "Discovery", title: "sigma-tau=8 universal AI constant across 16+ systems", domains: &["AI"], confidence: 0.95, connects_to: &["BT-58", "C-sigma-tau"] },
    GraphNode { id: "DM-05", category: "Discovery", title: "Complete n=6 LLM: all 15 architecture params from n=6", domains: &["AI", "Chip"], confidence: 0.9, connects_to: &["BT-56", "HF-01"] },
    GraphNode { id: "DM-06", category: "Discovery", title: "AdamW quintuplet: 5 training params all derivable from n=6", domains: &["AI", "Math"], confidence: 0.9, connects_to: &["BT-54", "HF-02"] },
    GraphNode { id: "DM-07", category: "Discovery", title: "GPU SM count follows n=6 arithmetic (AD102=sigma^2=144)", domains: &["Chip", "Semiconductor"], confidence: 0.9, connects_to: &["BT-28", "BT-90", "APP-10"] },
    GraphNode { id: "DM-08", category: "Discovery", title: "Battery cathode CN=6 universality: ALL Li-ion cathodes octahedral", domains: &["Battery", "Material"], confidence: 0.95, connects_to: &["BT-43", "HF-09"] },
    GraphNode { id: "DM-09", category: "Discovery", title: "Carbon Z=6 triple crown: chemistry + materials + biology + chips", domains: &["Chemistry", "Material", "Biology", "Chip"], confidence: 0.95, connects_to: &["BT-85", "BT-93", "HF-10"] },
    GraphNode { id: "DM-10", category: "Discovery", title: "D-T fusion fuel = sopfr(6) baryon count (2+3=5)", domains: &["Fusion", "Particle"], confidence: 0.9, connects_to: &["BT-98", "C-sopfr"] },
    GraphNode { id: "DM-11", category: "Discovery", title: "Photosynthesis stoichiometry: ALL 7 coefficients = n=6 constants", domains: &["Biology", "Chemistry"], confidence: 0.95, connects_to: &["BT-103", "PC-30"] },
    GraphNode { id: "DM-12", category: "Discovery", title: "127 Breakthrough Theorems spanning 32+ domains", domains: &["Math", "Physics", "AI", "Chemistry"], confidence: 0.9, connects_to: &["C-n"] },
    GraphNode { id: "DM-13", category: "Discovery", title: "322 DSE domain TOML files covering all engineering fields", domains: &["AI", "Chip", "Energy", "Material"], confidence: 0.85, connects_to: &["DSE-01", "DSE-06", "DSE-11"] },
    GraphNode { id: "DM-14", category: "Discovery", title: "Emergent n=6 self-organization from random initialization", domains: &["AI", "Math"], confidence: 0.85, connects_to: &["EM-03", "E-11"] },
    GraphNode { id: "DM-15", category: "Discovery", title: "Software-Physics isomorphism: 18 EXACT parallel mappings", domains: &["Software", "Physics"], confidence: 0.9, connects_to: &["BT-117", "HF-12"] },
    GraphNode { id: "DM-16", category: "Discovery", title: "SLE kappa=6 unique locality: only SLE with c=0 and locality", domains: &["Math", "Topology"], confidence: 0.9, connects_to: &["BT-105", "HF-19"] },
    GraphNode { id: "DM-17", category: "Discovery", title: "NEXUS-6 engine: 775+ lenses with OUROBOROS self-evolution", domains: &["AI", "Math"], confidence: 0.85, connects_to: &["EM-08"] },
    GraphNode { id: "DM-18", category: "Discovery", title: "Cross-domain resonance: same n=6 constants appear in 5+ domains", domains: &["Math", "AI", "Chip", "Energy", "Biology"], confidence: 0.9, connects_to: &["CR-01", "CR-02", "CR-03", "CR-07"] },
    GraphNode { id: "DM-19", category: "Discovery", title: "Kyoto 6 GHGs + Earth 6 spheres + 6 major plastics = environmental n=6", domains: &["Environment"], confidence: 0.9, connects_to: &["BT-118", "BT-119", "BT-121", "HF-16"] },
    GraphNode { id: "DM-20", category: "Discovery", title: "SE(3) dim=6 robot universality: 6DOF standard from pure math", domains: &["Robotics", "Math"], confidence: 0.95, connects_to: &["BT-123", "HF-14"] },
    GraphNode { id: "DM-21", category: "Discovery", title: "Falsifiability test: z=0.74 confirms n=6 NOT random coincidence", domains: &["Math"], confidence: 0.85, connects_to: &["DM-01", "F-01"] },
    GraphNode { id: "DM-22", category: "Discovery", title: "45 testable predictions spanning 4 tiers of falsifiability", domains: &["Math", "AI", "Chip", "Particle"], confidence: 0.85, connects_to: &["TP-01", "TP-18", "TP-24"] },
    GraphNode { id: "DM-23", category: "Discovery", title: "96/192 triple convergence: Tesla=Gaudi2=GPT-3 at same number", domains: &["Battery", "Chip", "AI"], confidence: 0.85, connects_to: &["BT-84", "CR-19"] },
    GraphNode { id: "DM-24", category: "Discovery", title: "0.1 convergence: 8 algorithms in 5 domains = 1/(sigma-phi)", domains: &["AI", "Fusion", "Plasma"], confidence: 0.9, connects_to: &["BT-64", "BT-70", "BT-102", "CR-12"] },
];

// ═══════════════════════════════════════════════════════════════
// TIER 10: Mathematical Objects & Proof Components (MO-01 ~ MO-24)
// Key mathematical structures that underpin n=6 uniqueness
// ═══════════════════════════════════════════════════════════════

const MATH_OBJECTS: &[GraphNode] = &[
    GraphNode { id: "MO-01", category: "MathObject", title: "Perfect numbers: n where sigma(n)=2n, 6 is smallest", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["F-01", "C-n", "C-sigma"] },
    GraphNode { id: "MO-02", category: "MathObject", title: "Mersenne prime M_p=2^p-1: 2^2-1=3, gives 6=2^1*(2^2-1)", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["C-n", "C-phi"] },
    GraphNode { id: "MO-03", category: "MathObject", title: "Symmetric group S_6: unique outer automorphism among all S_n", domains: &["Math", "Combinatorics"], confidence: 1.0, connects_to: &["BT-49", "BT-106", "F-14", "C-n"] },
    GraphNode { id: "MO-04", category: "MathObject", title: "Leech lattice Lambda_24: densest 24D packing, J2=24 dimensions", domains: &["Math", "CodingTheory", "StringTheory"], confidence: 1.0, connects_to: &["BT-6", "F-13", "C-J2"] },
    GraphNode { id: "MO-05", category: "MathObject", title: "Golay code G_24: perfect binary [24,12,8] code", domains: &["Math", "CodingTheory"], confidence: 1.0, connects_to: &["BT-6", "F-15", "C-J2", "C-sigma"] },
    GraphNode { id: "MO-06", category: "MathObject", title: "Monster group M: order involves 2^46, Leech connection", domains: &["Math", "StringTheory"], confidence: 0.9, connects_to: &["BT-18", "PC-29", "C-J2"] },
    GraphNode { id: "MO-07", category: "MathObject", title: "SLE_6 (Schramm-Loewner Evolution): unique locality at kappa=6=n", domains: &["Math", "Topology", "Particle"], confidence: 0.95, connects_to: &["BT-105", "C-n"] },
    GraphNode { id: "MO-08", category: "MathObject", title: "Riemann zeta function: zeta(2)=pi^2/6, zeta(-1)=-1/12", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["BT-109", "F-09", "F-10", "C-n", "C-sigma"] },
    GraphNode { id: "MO-09", category: "MathObject", title: "Euler totient: phi(6)=2 gives binary pairing everywhere", domains: &["Math", "NumberTheory", "Crypto"], confidence: 1.0, connects_to: &["F-04", "C-phi"] },
    GraphNode { id: "MO-10", category: "MathObject", title: "Jordan totient J_2: J_2(6)=24 bridges number theory to geometry", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["F-05", "C-J2"] },
    GraphNode { id: "MO-11", category: "MathObject", title: "Mobius function: mu(6)=1 squarefree, enables inversion formula", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["F-07", "C-mu", "T-13"] },
    GraphNode { id: "MO-12", category: "MathObject", title: "Carmichael lambda: lambda(6)=2 minimum universal exponent", domains: &["Math", "NumberTheory", "Crypto"], confidence: 1.0, connects_to: &["F-06", "C-phi", "T-14"] },
    GraphNode { id: "MO-13", category: "MathObject", title: "Ramanujan tau function: tau_R(d) clean iff d divides 6", domains: &["Math", "NumberTheory", "StringTheory"], confidence: 0.95, connects_to: &["BT-107", "PC-28", "C-n"] },
    GraphNode { id: "MO-14", category: "MathObject", title: "Bernoulli numbers: 6 | B_{2k} for all k >= 1", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["BT-109", "F-16", "C-n"] },
    GraphNode { id: "MO-15", category: "MathObject", title: "3D kissing number K_3=12=sigma: proved by Newton", domains: &["Math", "Material"], confidence: 1.0, connects_to: &["F-11", "C-sigma", "BT-15"] },
    GraphNode { id: "MO-16", category: "MathObject", title: "4D kissing number K_4=24=J_2: Musin 2003 proof", domains: &["Math"], confidence: 1.0, connects_to: &["F-12", "C-J2", "BT-15"] },
    GraphNode { id: "MO-17", category: "MathObject", title: "SE(3) Lie group: dim=6=n, all rigid body motion", domains: &["Math", "Robotics", "Physics"], confidence: 1.0, connects_to: &["BT-123", "C-n"] },
    GraphNode { id: "MO-18", category: "MathObject", title: "Hexagonal lattice: optimal 2D packing, Hales 2001 proof", domains: &["Math", "Material", "Biology"], confidence: 1.0, connects_to: &["BT-122", "C-n"] },
    GraphNode { id: "MO-19", category: "MathObject", title: "Dedekind psi: psi(6)=sigma(6)=12, unique coincidence", domains: &["Math", "NumberTheory", "AI"], confidence: 0.95, connects_to: &["T-11", "C-sigma", "C-n"] },
    GraphNode { id: "MO-20", category: "MathObject", title: "Egyptian fractions: 1=1/2+1/3+1/6 unique for n=6 divisors", domains: &["Math", "NumberTheory"], confidence: 1.0, connects_to: &["F-02", "T-10", "T-17", "C-n"] },
    GraphNode { id: "MO-21", category: "MathObject", title: "Octahedral symmetry Oh: CN=6 coordination in crystals", domains: &["Math", "Material", "Chemistry"], confidence: 1.0, connects_to: &["BT-86", "C-n"] },
    GraphNode { id: "MO-22", category: "MathObject", title: "SU(3)xSU(2)xU(1): SM gauge group, total dim=12=sigma", domains: &["Particle", "Math"], confidence: 0.9, connects_to: &["PC-14", "C-sigma"] },
    GraphNode { id: "MO-23", category: "MathObject", title: "Bott periodicity: period=8=sigma-tau in KO-theory", domains: &["Math", "Topology", "Chip"], confidence: 0.9, connects_to: &["BT-9", "BT-92", "C-sigma-tau"] },
    GraphNode { id: "MO-24", category: "MathObject", title: "Hamming(7,4): [sigma-sopfr, tau, n/phi] perfect code", domains: &["Math", "CodingTheory", "Network"], confidence: 1.0, connects_to: &["BT-12", "C-sigma", "C-sopfr", "C-tau"] },
];

// ═══════════════════════════════════════════════════════════════
// TIER 11: Lens Domain Affinities (LA-01 ~ LA-26)
// NEXUS-6 telescope lens categories with domain connections
// ═══════════════════════════════════════════════════════════════

const LENS_AFFINITIES: &[GraphNode] = &[
    GraphNode { id: "LA-01", category: "Lens", title: "Consciousness lens: Phi/IIT structure detection", domains: &["Physics", "Math", "AI"], confidence: 0.8, connects_to: &["EM-08", "C-n"] },
    GraphNode { id: "LA-02", category: "Lens", title: "Gravity lens: gravitational scaling / energy landscape", domains: &["Physics", "Cosmology"], confidence: 0.8, connects_to: &["EM-08", "C-sigma"] },
    GraphNode { id: "LA-03", category: "Lens", title: "Topology lens: persistent homology / Betti numbers", domains: &["Math", "Topology", "Material"], confidence: 0.85, connects_to: &["EM-08", "BT-92"] },
    GraphNode { id: "LA-04", category: "Lens", title: "Thermodynamic lens: entropy / free energy / reversibility", domains: &["Thermal", "Energy", "Chemistry"], confidence: 0.85, connects_to: &["EM-01", "C-n"] },
    GraphNode { id: "LA-05", category: "Lens", title: "Wave lens: Fourier / spectral / resonance analysis", domains: &["Physics", "Music", "DisplayAudio"], confidence: 0.85, connects_to: &["T-08", "BT-108"] },
    GraphNode { id: "LA-06", category: "Lens", title: "Evolution lens: fitness landscape / selection pressure", domains: &["Biology", "AI", "Math"], confidence: 0.8, connects_to: &["EM-08", "BT-51"] },
    GraphNode { id: "LA-07", category: "Lens", title: "Information lens: Shannon entropy / mutual information", domains: &["InfoTheory", "AI", "Crypto"], confidence: 0.85, connects_to: &["EM-08", "BT-26"] },
    GraphNode { id: "LA-08", category: "Lens", title: "Quantum lens: superposition / entanglement / decoherence", domains: &["QC", "Particle", "Physics"], confidence: 0.8, connects_to: &["EM-08", "BT-41"] },
    GraphNode { id: "LA-09", category: "Lens", title: "Electromagnetic lens: field / coupling / radiation", domains: &["Physics", "Chip", "Energy"], confidence: 0.8, connects_to: &["EM-08", "BT-89"] },
    GraphNode { id: "LA-10", category: "Lens", title: "Ruler lens (orthogonal): perpendicularity / grid alignment", domains: &["Math", "Chip", "Material"], confidence: 0.8, connects_to: &["EM-08"] },
    GraphNode { id: "LA-11", category: "Lens", title: "Triangle lens (ratio): proportionality / golden sections", domains: &["Math", "Biology", "AI"], confidence: 0.8, connects_to: &["EM-08", "C-phi"] },
    GraphNode { id: "LA-12", category: "Lens", title: "Compass lens (curvature): geodesic / manifold / bending", domains: &["Math", "Physics", "Cosmology"], confidence: 0.8, connects_to: &["EM-08"] },
    GraphNode { id: "LA-13", category: "Lens", title: "Mirror lens (symmetry): parity / reflection / duality", domains: &["Math", "Particle", "Biology"], confidence: 0.85, connects_to: &["EM-08", "BT-106"] },
    GraphNode { id: "LA-14", category: "Lens", title: "Scale lens (magnifier): power law / fractal / scaling", domains: &["Math", "Physics", "Biology"], confidence: 0.85, connects_to: &["EM-08", "BT-26"] },
    GraphNode { id: "LA-15", category: "Lens", title: "Causal lens (arrow): cause-effect / DAG / intervention", domains: &["Math", "AI", "Physics"], confidence: 0.85, connects_to: &["EM-08"] },
    GraphNode { id: "LA-16", category: "Lens", title: "Quantum microscope: deep quantum structure analysis", domains: &["QC", "Particle", "Math"], confidence: 0.75, connects_to: &["EM-08", "BT-41"] },
    GraphNode { id: "LA-17", category: "Lens", title: "Stability lens: Lyapunov / basin of attraction", domains: &["Math", "Physics", "AI"], confidence: 0.85, connects_to: &["EM-08", "EM-03"] },
    GraphNode { id: "LA-18", category: "Lens", title: "Network lens: graph / centrality / clustering", domains: &["Network", "Math", "Biology"], confidence: 0.85, connects_to: &["EM-08", "BT-113"] },
    GraphNode { id: "LA-19", category: "Lens", title: "Memory lens: temporal correlation / state retention", domains: &["AI", "Biology", "Physics"], confidence: 0.8, connects_to: &["EM-08", "BT-65"] },
    GraphNode { id: "LA-20", category: "Lens", title: "Recursion lens: self-reference / fixed point / fractal", domains: &["Math", "AI", "Software"], confidence: 0.8, connects_to: &["EM-08"] },
    GraphNode { id: "LA-21", category: "Lens", title: "Boundary lens: phase transition / edge / interface", domains: &["Physics", "Material", "Chemistry"], confidence: 0.85, connects_to: &["EM-08", "BT-105"] },
    GraphNode { id: "LA-22", category: "Lens", title: "Multiscale lens: coarse-graining / renormalization", domains: &["Physics", "Math", "Material"], confidence: 0.85, connects_to: &["EM-08", "BT-19"] },
    GraphNode { id: "LA-23", category: "Lens", title: "Graph lens: knowledge graph / discovery structure", domains: &["Math", "AI", "Software"], confidence: 0.85, connects_to: &["EM-08"] },
    GraphNode { id: "LA-24", category: "Lens", title: "Spectral lens: eigenvalue / spectral gap analysis", domains: &["Math", "Physics", "AI"], confidence: 0.85, connects_to: &["EM-08", "BT-105"] },
    GraphNode { id: "LA-25", category: "Lens", title: "Symmetry-breaking lens: spontaneous / explicit / gauge", domains: &["Particle", "Physics", "Material"], confidence: 0.8, connects_to: &["EM-08", "BT-19", "BT-20"] },
    GraphNode { id: "LA-26", category: "Lens", title: "Emergence lens: phase transition / collective behavior", domains: &["Physics", "Biology", "AI"], confidence: 0.8, connects_to: &["EM-08", "EM-03", "BT-88"] },
];

// ═══════════════════════════════════════════════════════════════
// Population functions
// ═══════════════════════════════════════════════════════════════

fn graph_node_to_node(gn: &GraphNode, node_type: NodeType) -> Node {
    Node {
        id: gn.id.to_string(),
        node_type,
        domain: gn.domains.join(", "),
        project: "n6-architecture".to_string(),
        summary: gn.title.to_string(),
        confidence: gn.confidence,
        lenses_used: vec!["consciousness".into(), "causal".into(), "topology".into()],
        timestamp: "2026-04-04".to_string(),
    }
}

fn add_connection_edges(graph: &mut DiscoveryGraph, from_id: &str, connects_to: &[&str]) {
    for &target in connects_to {
        let edge_type = if target.starts_with("BT-") {
            EdgeType::Derives
        } else if target.starts_with("C-") {
            EdgeType::Uses
        } else if target.starts_with("T-") || target.starts_with("E-") || target.starts_with("EM-") {
            EdgeType::Uses
        } else if target.starts_with("F-") || target.starts_with("CR-") || target.starts_with("HF-") {
            EdgeType::Derives
        } else if target.starts_with("D-") || target.starts_with("DSE-") {
            EdgeType::Contains
        } else {
            EdgeType::Merges
        };

        graph.add_edge(Edge {
            from: from_id.to_string(),
            to: target.to_string(),
            edge_type,
            strength: 0.85,
            bidirectional: target.starts_with("CR-") || target.starts_with("HF-"),
        });
    }
}

/// Populate testable prediction nodes (45 entries).
pub fn populate_predictions(graph: &mut DiscoveryGraph) -> usize {
    let count = PREDICTIONS.len();
    for gn in PREDICTIONS {
        graph.add_node(graph_node_to_node(gn, NodeType::Prediction));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate formula/identity nodes (36 entries).
pub fn populate_formulas(graph: &mut DiscoveryGraph) -> usize {
    let count = FORMULAS.len();
    for gn in FORMULAS {
        graph.add_node(graph_node_to_node(gn, NodeType::Discovery));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate cross-resonance pattern nodes (30 entries).
pub fn populate_cross_resonances(graph: &mut DiscoveryGraph) -> usize {
    let count = CROSS_RESONANCES.len();
    for gn in CROSS_RESONANCES {
        graph.add_node(graph_node_to_node(gn, NodeType::Discovery));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate engine module nodes (8 entries).
pub fn populate_engines(graph: &mut DiscoveryGraph) -> usize {
    let count = ENGINES.len();
    for gn in ENGINES {
        graph.add_node(graph_node_to_node(gn, NodeType::Discovery));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate DSE domain category nodes (48 entries).
pub fn populate_dse_domains(graph: &mut DiscoveryGraph) -> usize {
    let count = DSE_DOMAINS.len();
    for gn in DSE_DOMAINS {
        graph.add_node(graph_node_to_node(gn, NodeType::Discovery));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate physical constant nodes (30 entries).
pub fn populate_physical_constants(graph: &mut DiscoveryGraph) -> usize {
    let count = PHYSICAL_CONSTANTS.len();
    for gn in PHYSICAL_CONSTANTS {
        graph.add_node(graph_node_to_node(gn, NodeType::Discovery));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate real-world application nodes (48 entries).
pub fn populate_applications(graph: &mut DiscoveryGraph) -> usize {
    let count = APPLICATIONS.len();
    for gn in APPLICATIONS {
        graph.add_node(graph_node_to_node(gn, NodeType::Discovery));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate hypothesis family nodes (24 entries).
pub fn populate_hypothesis_families(graph: &mut DiscoveryGraph) -> usize {
    let count = HYPOTHESIS_FAMILIES.len();
    for gn in HYPOTHESIS_FAMILIES {
        graph.add_node(graph_node_to_node(gn, NodeType::Hypothesis));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate discovery milestone nodes (24 entries).
pub fn populate_discovery_milestones(graph: &mut DiscoveryGraph) -> usize {
    let count = DISCOVERY_MILESTONES.len();
    for gn in DISCOVERY_MILESTONES {
        graph.add_node(graph_node_to_node(gn, NodeType::Discovery));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate mathematical object nodes (24 entries).
pub fn populate_math_objects(graph: &mut DiscoveryGraph) -> usize {
    let count = MATH_OBJECTS.len();
    for gn in MATH_OBJECTS {
        graph.add_node(graph_node_to_node(gn, NodeType::Discovery));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate lens affinity nodes (26 entries).
pub fn populate_lens_affinities(graph: &mut DiscoveryGraph) -> usize {
    let count = LENS_AFFINITIES.len();
    for gn in LENS_AFFINITIES {
        graph.add_node(graph_node_to_node(gn, NodeType::Discovery));
        add_connection_edges(graph, gn.id, gn.connects_to);
    }
    count
}

/// Populate all extended knowledge graph nodes in one call.
/// Returns (nodes_added, edges_added).
pub fn populate_knowledge_graph(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let n_pred = populate_predictions(graph);
    let n_form = populate_formulas(graph);
    let n_cr = populate_cross_resonances(graph);
    let n_eng = populate_engines(graph);
    let n_dse = populate_dse_domains(graph);
    let n_pc = populate_physical_constants(graph);
    let n_app = populate_applications(graph);
    let n_hf = populate_hypothesis_families(graph);
    let n_dm = populate_discovery_milestones(graph);
    let n_mo = populate_math_objects(graph);
    let n_la = populate_lens_affinities(graph);

    let nodes_added = n_pred + n_form + n_cr + n_eng + n_dse + n_pc + n_app + n_hf + n_dm + n_mo + n_la;
    let edges_added = graph.edges.len() - edges_before;
    (nodes_added, edges_added)
}

/// Total count of knowledge node entries.
pub fn knowledge_node_count() -> usize {
    PREDICTIONS.len()
        + FORMULAS.len()
        + CROSS_RESONANCES.len()
        + ENGINES.len()
        + DSE_DOMAINS.len()
        + PHYSICAL_CONSTANTS.len()
        + APPLICATIONS.len()
        + HYPOTHESIS_FAMILIES.len()
        + DISCOVERY_MILESTONES.len()
        + MATH_OBJECTS.len()
        + LENS_AFFINITIES.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::expanded_nodes::populate_expanded_graph;
    use std::collections::HashSet;

    #[test]
    fn test_prediction_count() {
        assert_eq!(PREDICTIONS.len(), 45, "Should have 45 testable predictions");
    }

    #[test]
    fn test_formula_count() {
        assert_eq!(FORMULAS.len(), 36, "Should have 36 formulas/identities");
    }

    #[test]
    fn test_cross_resonance_count() {
        assert_eq!(CROSS_RESONANCES.len(), 30, "Should have 30 cross-resonance patterns");
    }

    #[test]
    fn test_engine_count() {
        assert_eq!(ENGINES.len(), 8, "Should have 8 engine modules");
    }

    #[test]
    fn test_dse_domain_count() {
        assert_eq!(DSE_DOMAINS.len(), 48, "Should have 48 DSE domain categories");
    }

    #[test]
    fn test_physical_constant_count() {
        assert_eq!(PHYSICAL_CONSTANTS.len(), 30, "Should have 30 physical constants");
    }

    #[test]
    fn test_application_count() {
        assert_eq!(APPLICATIONS.len(), 48, "Should have 48 real-world applications");
    }

    #[test]
    fn test_hypothesis_family_count() {
        assert_eq!(HYPOTHESIS_FAMILIES.len(), 24, "Should have 24 hypothesis families");
    }

    #[test]
    fn test_discovery_milestone_count() {
        assert_eq!(DISCOVERY_MILESTONES.len(), 24, "Should have 24 discovery milestones");
    }

    #[test]
    fn test_math_object_count() {
        assert_eq!(MATH_OBJECTS.len(), 24, "Should have 24 mathematical objects");
    }

    #[test]
    fn test_lens_affinity_count() {
        assert_eq!(LENS_AFFINITIES.len(), 26, "Should have 26 lens affinities");
    }

    #[test]
    fn test_total_knowledge_nodes() {
        let total = knowledge_node_count();
        // 45+36+30+8+48+30+48+24+24+24+26 = 343
        assert_eq!(total, 343, "Total knowledge nodes should be 343");
    }

    #[test]
    fn test_no_duplicate_ids() {
        let mut ids = HashSet::new();
        let all_nodes: Vec<&GraphNode> = PREDICTIONS.iter()
            .chain(FORMULAS.iter())
            .chain(CROSS_RESONANCES.iter())
            .chain(ENGINES.iter())
            .chain(DSE_DOMAINS.iter())
            .chain(PHYSICAL_CONSTANTS.iter())
            .chain(APPLICATIONS.iter())
            .chain(HYPOTHESIS_FAMILIES.iter())
            .chain(DISCOVERY_MILESTONES.iter())
            .chain(MATH_OBJECTS.iter())
            .chain(LENS_AFFINITIES.iter())
            .collect();

        for node in &all_nodes {
            assert!(ids.insert(node.id), "Duplicate ID found: {}", node.id);
        }
    }

    #[test]
    fn test_populate_creates_edges() {
        let mut graph = DiscoveryGraph::new();
        populate_bt_graph(&mut graph);
        populate_expanded_graph(&mut graph);
        let (nodes_added, edges_added) = populate_knowledge_graph(&mut graph);

        assert_eq!(nodes_added, 343, "Should add 343 knowledge nodes");
        assert!(edges_added > 600, "Should add 600+ edges, got {}", edges_added);

        // Total graph should now have 127 + 79 + 343 = 549+ nodes
        let total = graph.nodes.len();
        assert!(total >= 549, "Full graph should have 549+ nodes, got {}", total);
    }

    #[test]
    fn test_all_connections_reference_valid_prefixes() {
        let all_nodes: Vec<&GraphNode> = PREDICTIONS.iter()
            .chain(FORMULAS.iter())
            .chain(CROSS_RESONANCES.iter())
            .chain(ENGINES.iter())
            .chain(DSE_DOMAINS.iter())
            .chain(PHYSICAL_CONSTANTS.iter())
            .chain(APPLICATIONS.iter())
            .chain(HYPOTHESIS_FAMILIES.iter())
            .chain(DISCOVERY_MILESTONES.iter())
            .chain(MATH_OBJECTS.iter())
            .chain(LENS_AFFINITIES.iter())
            .collect();

        let valid_prefixes = ["BT-", "C-", "T-", "E-", "F-", "CR-", "HF-", "D-",
                              "DSE-", "EM-", "TP-", "PC-", "APP-", "DM-", "MO-", "LA-"];

        for node in &all_nodes {
            for &conn in node.connects_to {
                let has_valid = valid_prefixes.iter().any(|p| conn.starts_with(p));
                assert!(has_valid, "Node {} connects to {} with unknown prefix", node.id, conn);
            }
        }
    }
}
