pub mod accel_lenses_a;
pub mod accel_lenses_b;
pub mod accel_lenses_c;
pub mod accel_lenses_d;
pub mod anima_lenses;
pub mod consensus;
pub mod core_lenses;
pub mod cross_lenses;
pub mod domain_combos;
pub mod lens_trait;
pub mod lenses;
pub mod n6_lenses;
pub mod physics_deep_lenses;
pub mod quantum_lenses;
pub mod registry;
pub mod sedi_lenses;
pub mod shared_data;
pub mod tecs_lenses;
pub mod tier;

use std::collections::HashMap;
use std::panic;

use lens_trait::{Lens, LensResult};
use lenses::{
    BarrierLens, BoundaryLens, CausalLens, CompassLens, ConsciousnessLens, EmLens,
    EvolutionLens, GravityLens, InfoLens, MemoryLens, MiLens, MirrorLens, MultiscaleLens,
    NetworkLens, QuantumLensImpl, QuantumMicroLens, RecursionLens, RenormalizationLens,
    RulerLens, ScaleLens, StabilityLens, ThermoLens, TopologyLens, TriangleLens,
    VoidLens, WaveLens,
};
use shared_data::SharedData;

/// The Telescope: registers all available lenses and scans data through them.
/// Each lens is isolated via catch_unwind — a panic in one lens does not crash others.
pub struct Telescope {
    lenses: Vec<Box<dyn Lens>>,
}

impl Telescope {
    /// Create a new Telescope with all 22 Core lenses registered.
    pub fn new() -> Self {
        let lenses: Vec<Box<dyn Lens>> = vec![
            // Foundational 9
            Box::new(ConsciousnessLens),
            Box::new(GravityLens),
            Box::new(TopologyLens),
            Box::new(ThermoLens),
            Box::new(WaveLens),
            Box::new(EvolutionLens),
            Box::new(InfoLens),
            Box::new(QuantumLensImpl),
            Box::new(EmLens),
            // Measurement 6
            Box::new(RulerLens),
            Box::new(TriangleLens),
            Box::new(CompassLens),
            Box::new(MirrorLens),
            Box::new(ScaleLens),
            Box::new(CausalLens),
            // Quantum microscope
            Box::new(QuantumMicroLens),
            // Structural 5
            Box::new(StabilityLens),
            Box::new(NetworkLens),
            Box::new(MemoryLens),
            Box::new(RecursionLens),
            Box::new(BoundaryLens),
            Box::new(MultiscaleLens),
            Box::new(RenormalizationLens),
            // Mutual Information (migrated from telescope-rs)
            Box::new(MiLens),
            // Original 2 (void + barrier)
            Box::new(VoidLens),
            Box::new(BarrierLens),
        ];
        Telescope { lenses }
    }

    /// Scan data through all registered lenses.
    /// Returns lens_name -> LensResult for each lens.
    /// Panics in individual lenses are caught and produce empty results.
    pub fn scan_all(
        &self,
        data: &[f64],
        n: usize,
        d: usize,
    ) -> HashMap<String, LensResult> {
        let shared = SharedData::compute(data, n, d);
        let mut results = HashMap::new();

        for lens in &self.lenses {
            let name = lens.name().to_string();

            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                lens.scan(data, n, d, &shared)
            }));

            match result {
                Ok(lr) => {
                    results.insert(name, lr);
                }
                Err(_) => {
                    results.insert(name, HashMap::new());
                }
            }
        }

        results
    }

    /// Get the number of registered lenses.
    pub fn lens_count(&self) -> usize {
        self.lenses.len()
    }
}

impl Default for Telescope {
    fn default() -> Self {
        Self::new()
    }
}
