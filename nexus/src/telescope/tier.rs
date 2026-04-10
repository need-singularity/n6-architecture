use std::collections::HashMap;
use std::panic;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// Tiered scanning: runs lenses in tiers (T0, T1, T2...).
/// If T0 produces no signal, stops early. If signal found, proceeds to T1, etc.
pub struct TieredScanner {
    /// Tiers: (tier_name, lenses)
    tiers: Vec<(String, Vec<Box<dyn Lens>>)>,
}

impl TieredScanner {
    pub fn new() -> Self {
        TieredScanner { tiers: Vec::new() }
    }

    /// Add a tier with its lenses.
    pub fn add_tier(&mut self, name: &str, lenses: Vec<Box<dyn Lens>>) {
        self.tiers.push((name.to_string(), lenses));
    }

    /// Run tiered scan. Returns all results from tiers that were executed.
    /// Stops after a tier if no signal was found.
    pub fn scan(
        &self,
        data: &[f64],
        n: usize,
        d: usize,
    ) -> HashMap<String, LensResult> {
        let shared = SharedData::compute(data, n, d);
        let mut all_results = HashMap::new();

        for (tier_name, lenses) in &self.tiers {
            let mut tier_has_signal = false;

            for lens in lenses {
                let lens_name = lens.name().to_string();

                // Isolate each lens with catch_unwind
                let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                    lens.scan(data, n, d, &shared)
                }));

                match result {
                    Ok(lr) => {
                        // Check if this lens found any signal
                        let has_data = lr
                            .values()
                            .any(|v| !v.is_empty());
                        if has_data {
                            tier_has_signal = true;
                        }
                        all_results.insert(
                            format!("{}:{}", tier_name, lens_name),
                            lr,
                        );
                    }
                    Err(_) => {
                        // Lens panicked — record empty result, continue
                        all_results.insert(
                            format!("{}:{}", tier_name, lens_name),
                            HashMap::new(),
                        );
                    }
                }
            }

            // Early exit if no signal in this tier
            if !tier_has_signal {
                break;
            }
        }

        all_results
    }
}

impl Default for TieredScanner {
    fn default() -> Self {
        Self::new()
    }
}
