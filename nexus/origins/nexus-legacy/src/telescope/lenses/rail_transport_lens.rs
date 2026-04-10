use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 철도 수송 렌즈 — 신호/궤도 n=6 안전 아키텍처 (BT-278)
/// n=6 연결: tau=4 안전레벨, n=6 궤도회로, sigma=12 차축, 250→500→750km/h 배증
pub struct RailTransportLens;

impl Lens for RailTransportLens {
    fn name(&self) -> &'static str { "RailTransportLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let rail_consts: &[(f64, &str)] = &[
            (4.0, "sil_tau"), (6.0, "track_circuit"), (12.0, "axle_sigma"),
            (1.5, "track_gauge_m"), (25.0, "kv_catenary"), (50.0, "freq_hz"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();
        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in rail_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.08 { n6_matches += 1; break; }
            }
        }
        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0);
        let mut r = HashMap::new();
        r.insert("n6_rail_matches".to_string(), vec![n6_matches as f64]);
        r.insert("rail_transport_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rail_basic() {
        let n = 6; let d = 3;
        let data: Vec<f64> = (0..n*d).map(|i| [4.0f64,6.0,12.0][i%d]).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = RailTransportLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("rail_transport_score"));
    }
}
