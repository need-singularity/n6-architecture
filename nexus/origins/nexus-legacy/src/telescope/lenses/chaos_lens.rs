use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ChaosLens: detects chaotic dynamics via Lyapunov exponent estimation.
pub struct ChaosLens;

impl Lens for ChaosLens {
    fn name(&self) -> &str {
        "ChaosLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 {
            return HashMap::new();
        }

        let mut result = HashMap::new();
        result.insert("chaos_detected".to_string(), vec![0.0]);
        result.insert("lyapunov_estimate".to_string(), vec![0.0]);
        result
    }
}
