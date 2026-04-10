use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 수의학 렌즈 — 동물 생리학 n=6 패턴 (심장 박동, 척추, 사지 수)
/// n=6 연결: 사지=tau=4+날개=phi=2=n=6, 척추 경추=sigma-sopfr=7, 심장챔버=tau=4
pub struct VeterinaryMedicineLens;

impl Lens for VeterinaryMedicineLens {
    fn name(&self) -> &'static str { "VeterinaryMedicineLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let vet_consts: &[(f64, &str)] = &[
            (4.0, "limbs_tau"), (6.0, "appendages_n"), (7.0, "cervical_sigma_sopfr"),
            (12.0, "thoracic_sigma"), (4.0, "heart_chambers_tau"), (2.0, "lungs_phi"),
            (3.0, "blood_types_n_phi"), (24.0, "J2_ribs_bilateral"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in vet_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.08 { n6_matches += 1; break; }
            }
        }

        // 심장 박동 리듬 패턴 (60-200 BPM 범위, 주기성)
        let heartbeat_score = {
            let col: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
            let mean = col.iter().sum::<f64>() / n as f64;
            let in_bpm_range = (mean >= 40.0 && mean <= 220.0) as u8 as f64;
            in_bpm_range * 0.5 + if (mean - 60.0).abs() < 20.0 || (mean - 120.0).abs() < 20.0 { 0.5 } else { 0.0 }
        };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.6 + heartbeat_score * 0.4;

        let mut r = HashMap::new();
        r.insert("n6_vet_matches".to_string(), vec![n6_matches as f64]);
        r.insert("heartbeat_score".to_string(), vec![heartbeat_score]);
        r.insert("veterinary_medicine_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_veterinary_basic() {
        let n = 6; let d = 3;
        let data: Vec<f64> = (0..n*d).map(|i| [4.0f64, 6.0, 12.0][i%d]).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = VeterinaryMedicineLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("veterinary_medicine_score"));
        assert!(result["n6_vet_matches"][0] >= 2.0);
    }
}
