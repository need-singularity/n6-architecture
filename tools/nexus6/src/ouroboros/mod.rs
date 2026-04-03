//! OUROBOROS self-evolution loop for continuous discovery.
pub mod engine;
pub mod mutation;
pub mod convergence;
pub mod meta_loop;

pub use engine::{EvolutionEngine, CycleResult, EvolutionConfig};
pub use mutation::{mutate_hypothesis, MutationStrategy};
pub use convergence::{ConvergenceChecker, ConvergenceStatus};
pub use meta_loop::{MetaLoop, MetaLoopConfig, MetaLoopResult};
