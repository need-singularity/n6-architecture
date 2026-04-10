use std::collections::HashMap;

use crate::telescope::shared_data::SharedData;

/// Result from a single lens scan: metric_name -> values
pub type LensResult = HashMap<String, Vec<f64>>;

/// Every telescope lens must implement this trait.
/// Lenses are run in parallel via rayon, so Send + Sync are required.
pub trait Lens: Send + Sync {
    /// Human-readable lens name (e.g. "VoidLens")
    fn name(&self) -> &str;

    /// Category for tiered scanning (e.g. "T0", "T1")
    fn category(&self) -> &str;

    /// Run the lens on data (N points x D dimensions, row-major).
    /// `shared` contains pre-computed distance matrix.
    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult;
}
