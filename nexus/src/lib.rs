pub mod gpu;
pub mod encoder;
pub mod materials;
pub mod verifier;
pub mod graph;
pub mod telescope;
pub mod history;
pub mod ouroboros;
pub mod lens_forge;
pub mod experiment;
pub mod science;
pub mod cli;

// --- Agent-generated modules ---
pub mod alert;
pub mod api;
pub mod auto_register;
pub mod autonomous;
pub mod consciousness_bridge;
pub mod cross_intel;
pub mod distributed;
pub mod dream;
pub mod event;
pub mod feedback;
pub mod genetic_prog;
pub mod ingest;
pub mod knowledge;
pub mod multi_agent;
pub mod nlp;
pub mod pipeline;
pub mod plugin;
pub mod publish;
pub mod red_team;
pub mod reward;
pub mod sandbox;
pub mod scheduler;
pub mod self_improve;
pub mod statistics;
pub mod template;
pub mod time_travel;
pub mod versioning;

// --- Calibration & Simulation extensions ---
pub mod calibration;
pub mod simulation;

// --- Safety gate ---
pub mod safety;

// --- HEXA-GATE (breakthrough gate, Mk.I) ---
pub mod gate;

// --- Growth engine ---
pub mod growth;

// --- Topology exploration ---
pub mod topology_exploration;

// --- Adaptive & Compiler extensions ---
pub mod adaptive;
pub mod compiler_bench;
pub mod compiler_engines;
pub mod compiler_modules;

// --- Cross-module integration ---
pub mod integration;

// --- 통합 서브커맨드 dispatcher (tools/ 62 크레이트 흡수) ---
pub mod cmd;

#[cfg(feature = "python")]
pub mod python;

// Comprehensive module tests — do not remove
#[cfg(test)]
mod module_tests;
