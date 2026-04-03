//! Scan history tracking and lens performance analytics.
pub mod recorder;
pub mod stats;
pub mod recommend;

pub use recorder::ScanRecord;
pub use stats::{DomainStats, LensStats};
pub use recommend::LensRecommendation;
