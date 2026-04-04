use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// FusionLens: Nuclear fusion patterns in data.
/// Detects merging clusters, energy confinement, Lawson criterion analog, ignition.
/// n=6: D-T baryon=sopfr=5, q=1 from 1/2+1/3+1/6.
pub struct FusionLens;

impl Lens for FusionLens {
    fn name(&self) -> &str { "FusionLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Fusion rate: how many close pairs merge (dist < mean_dist * 0.3)
        let mut total_dist = 0.0;
        let mut pair_count = 0usize;
        for i in 1..max_n {
            for j in 0..i {
                total_dist += shared.dist(i, j);
                pair_count += 1;
            }
        }
        let mean_dist = if pair_count > 0 { total_dist / pair_count as f64 } else { 1.0 };
        let fusion_threshold = mean_dist * 0.3;
        let mut fused = 0usize;
        for i in 1..max_n {
            for j in 0..i {
                if shared.dist(i, j) < fusion_threshold {
                    fused += 1;
                }
            }
        }
        let fusion_rate = if pair_count > 0 { fused as f64 / pair_count as f64 } else { 0.0 };

        // Confinement quality: ratio of energy in core vs periphery
        // Core = points within mean_dist/2 of centroid
        let mut centroid = vec![0.0f64; d];
        for i in 0..max_n {
            for j in 0..d {
                centroid[j] += data[i * d + j];
            }
        }
        for j in 0..d { centroid[j] /= max_n as f64; }

        let mut core_energy = 0.0f64;
        let mut total_energy = 0.0f64;
        for i in 0..max_n {
            let mut dist_to_center = 0.0;
            for j in 0..d {
                let diff = data[i * d + j] - centroid[j];
                dist_to_center += diff * diff;
            }
            dist_to_center = dist_to_center.sqrt();
            let energy = data[i * d..(i * d + d)].iter().map(|x| x * x).sum::<f64>();
            total_energy += energy;
            if dist_to_center < mean_dist * 0.5 {
                core_energy += energy;
            }
        }
        let confinement_quality = if total_energy > 1e-12 { core_energy / total_energy } else { 0.0 };

        // Lawson criterion analog: density * confinement * temperature
        let density = max_n as f64 / (mean_dist.powi(d.min(3) as i32).max(1e-12));
        let temperature = total_energy / max_n as f64;
        let lawson_criterion = (density * confinement_quality * temperature).ln().max(0.0);

        // Ignition score: self-sustaining = high density + high confinement + high temperature
        let ignition_score = (fusion_rate * confinement_quality * temperature.sqrt()).min(10.0);

        let mut result = HashMap::new();
        result.insert("fusion_rate".to_string(), vec![fusion_rate]);
        result.insert("confinement_quality".to_string(), vec![confinement_quality]);
        result.insert("lawson_criterion".to_string(), vec![lawson_criterion]);
        result.insert("ignition_score".to_string(), vec![ignition_score]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        // Two clusters that should partially fuse
        let mut data = Vec::new();
        for i in 0..10 { data.push(i as f64 * 0.01); }
        for i in 0..10 { data.push(5.0 + i as f64 * 0.01); }
        let shared = SharedData::compute(&data, 20, 1);
        let result = FusionLens.scan(&data, 20, 1, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("fusion_rate"));
    }
}
