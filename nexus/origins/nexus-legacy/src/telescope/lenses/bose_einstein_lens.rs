use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// BoseEinsteinLens: Detect Bose-Einstein condensation signatures and macroscopic quantum coherence.
///
/// Algorithm:
///   1. Assign "energy" to each point as distance from the centroid (ground state = lowest energy)
///   2. Bin points into energy levels, compute occupation numbers
///   3. Condensation fraction: fraction of points in lowest energy bin vs uniform expectation
///   4. Bunching parameter g(2): second-order correlation measuring boson-like clustering
///   5. Coherence length: characteristic scale over which density-density correlations persist
///   6. BE distribution fit: how well occupation numbers follow 1/(exp(E/T)-1) shape
pub struct BoseEinsteinLens;

impl Lens for BoseEinsteinLens {
    fn name(&self) -> &str {
        "BoseEinsteinLens"
    }

    fn category(&self) -> &str {
        "quantum"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        // 1. Compute centroid
        let mut centroid = vec![0.0f64; d];
        for i in 0..n {
            for j in 0..d {
                centroid[j] += data[i * d + j];
            }
        }
        let inv_n = 1.0 / n as f64;
        for j in 0..d {
            centroid[j] *= inv_n;
        }

        // 2. Compute "energy" for each point = squared distance from centroid
        let mut energies: Vec<(f64, usize)> = Vec::with_capacity(n);
        for i in 0..n {
            let mut e = 0.0;
            for j in 0..d {
                let diff = data[i * d + j] - centroid[j];
                e += diff * diff;
            }
            energies.push((e, i));
        }
        energies.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let e_max = energies.last().map(|x| x.0).unwrap_or(1.0).max(1e-12);

        // Normalize energies to [0, 1]
        let norm_energies: Vec<f64> = energies.iter().map(|(e, _)| e / e_max).collect();

        // 3. Bin into energy levels and compute occupation numbers
        let n_bins = ((n as f64).sqrt().ceil() as usize).max(3).min(20);
        let bin_width = 1.0 / n_bins as f64;
        let mut occupation = vec![0usize; n_bins];
        for &e in &norm_energies {
            let bin = (e / bin_width) as usize;
            occupation[bin.min(n_bins - 1)] += 1;
        }

        // 4. Condensation fraction: ground-state occupation vs uniform expectation
        let ground_occ = occupation[0] as f64;
        let uniform_expect = n as f64 / n_bins as f64;
        let condensation_fraction = ground_occ / n as f64;
        // Excess over uniform: >1 means ground-state bunching (BEC signature)
        let condensation_excess = ground_occ / uniform_expect.max(1e-12);

        // 5. Bunching parameter g(2): pair correlation at short range vs random
        //    g(2) > 1 indicates bosonic bunching, g(2) = 1 is Poisson, g(2) < 1 is antibunching
        let pair_count = n * (n - 1) / 2;
        let mut all_dists: Vec<f64> = Vec::with_capacity(pair_count);
        for i in 0..n {
            for j in (i + 1)..n {
                all_dists.push(shared.dist(i, j));
            }
        }
        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let median_dist = all_dists[pair_count / 2];
        let short_range_threshold = median_dist * 0.5;

        // Count pairs within short range
        let short_pairs = all_dists.iter().filter(|&&d| d < short_range_threshold).count();
        // Expected under uniform: fraction of volume within threshold
        // For d-dimensional sphere: V(r) ~ r^d, so expected fraction ~ (threshold/max_dist)^d
        let max_dist = all_dists.last().copied().unwrap_or(1.0).max(1e-12);
        let volume_fraction = (short_range_threshold / max_dist).powi(d.min(10) as i32);
        let expected_short = (pair_count as f64 * volume_fraction).max(1.0);
        let g2 = short_pairs as f64 / expected_short;

        // 6. Coherence length: find the distance scale where density-density correlation drops to 1/e
        //    Use KNN density and compute correlation vs distance
        let n_corr_bins = 10usize.min(n);
        let dist_bin_width = max_dist / n_corr_bins as f64;
        let mut corr_sum = vec![0.0f64; n_corr_bins];
        let mut corr_count = vec![0usize; n_corr_bins];

        // Compute local density for each point using KNN
        let mut densities = Vec::with_capacity(n);
        for i in 0..n {
            densities.push(shared.knn_density(i));
        }
        let mean_density = densities.iter().sum::<f64>() * inv_n;
        let density_var = densities.iter().map(|&rho| (rho - mean_density).powi(2)).sum::<f64>() * inv_n;

        if density_var > 1e-20 {
            // Compute density-density correlation as function of distance
            let sample_limit = 2000usize; // cap pairs for large n
            let mut sampled = 0usize;
            'outer: for i in 0..n {
                for j in (i + 1)..n {
                    let dij = shared.dist(i, j);
                    let bin = (dij / dist_bin_width) as usize;
                    let bin = bin.min(n_corr_bins - 1);
                    let corr = (densities[i] - mean_density) * (densities[j] - mean_density);
                    corr_sum[bin] += corr;
                    corr_count[bin] += 1;
                    sampled += 1;
                    if sampled >= sample_limit {
                        break 'outer;
                    }
                }
            }

            // Normalize correlation
            for k in 0..n_corr_bins {
                if corr_count[k] > 0 {
                    corr_sum[k] /= corr_count[k] as f64 * density_var;
                }
            }
        }

        // Find coherence length: first bin where correlation drops below 1/e
        let e_inv = (-1.0f64).exp(); // 1/e ~ 0.3679
        let mut coherence_length = max_dist; // default: entire dataset
        for k in 0..n_corr_bins {
            if corr_count[k] > 0 && corr_sum[k] < e_inv {
                coherence_length = (k as f64 + 0.5) * dist_bin_width;
                break;
            }
        }
        let coherence_ratio = coherence_length / max_dist;

        // 7. BE distribution fit: compare occupation numbers to Bose-Einstein shape
        //    n(E) = 1 / (exp(E/kT) - 1), estimate kT from mean energy
        let mean_energy = norm_energies.iter().sum::<f64>() * inv_n;
        let kt = mean_energy.max(1e-6); // effective temperature

        let mut be_fit_error = 0.0;
        let mut be_fit_norm = 0.0;
        for k in 0..n_bins {
            let e_center = (k as f64 + 0.5) * bin_width;
            // BE distribution (with chemical potential ~ 0 for condensation)
            let be_expected = 1.0 / ((e_center / kt).exp() - 1.0 + 1e-12);
            let observed = occupation[k] as f64;
            be_fit_norm += be_expected;
            be_fit_error += (observed - be_expected).abs();
        }
        // Normalize fit quality: 1 = perfect, 0 = terrible
        let be_fit_quality = if be_fit_norm > 1e-12 {
            1.0 - (be_fit_error / (be_fit_norm + n as f64)).min(1.0)
        } else {
            0.0
        };

        // 8. Composite BEC score: weighted combination of indicators
        //    condensation_excess > 1, g2 > 1, high coherence_ratio, good BE fit
        let bec_score = 0.3 * (condensation_excess - 1.0).max(0.0).min(5.0) / 5.0
            + 0.3 * (g2 - 1.0).max(0.0).min(10.0) / 10.0
            + 0.2 * coherence_ratio
            + 0.2 * be_fit_quality;

        let mut result = HashMap::new();
        result.insert("bec_score".to_string(), vec![bec_score]);
        result.insert(
            "condensation".to_string(),
            vec![condensation_fraction, condensation_excess],
        );
        result.insert("bunching_g2".to_string(), vec![g2]);
        result.insert(
            "coherence".to_string(),
            vec![coherence_length, coherence_ratio],
        );
        result.insert("be_fit_quality".to_string(), vec![be_fit_quality]);
        result.insert("effective_temperature".to_string(), vec![kt]);
        result.insert(
            "occupation_numbers".to_string(),
            occupation.iter().map(|&x| x as f64).collect(),
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    /// Test with clustered data (many points near origin = BEC-like ground state condensation)
    #[test]
    fn test_bec_clustered_data() {
        // 8 points near origin + 2 outliers in 2D
        let data = vec![
            0.1, 0.1, // ground state cluster
            0.2, 0.0,
            0.0, 0.2,
            0.15, 0.05,
            0.05, 0.15,
            0.1, 0.0,
            0.0, 0.1,
            0.12, 0.08,
            5.0, 5.0, // thermal excitations (outliers)
            4.0, 6.0,
        ];
        let n = 10;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = BoseEinsteinLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty results");
        assert!(result.contains_key("bec_score"));
        assert!(result.contains_key("condensation"));
        assert!(result.contains_key("bunching_g2"));
        assert!(result.contains_key("coherence"));
        assert!(result.contains_key("occupation_numbers"));

        let bec_score = result["bec_score"][0];
        assert!(bec_score.is_finite(), "bec_score must be finite");

        // Clustered data should show condensation excess > 1
        let condensation_excess = result["condensation"][1];
        assert!(
            condensation_excess > 1.0,
            "clustered data should show condensation excess > 1, got {}",
            condensation_excess
        );
    }

    /// Test with uniformly spread data (no condensation expected)
    #[test]
    fn test_bec_uniform_spread() {
        // Points evenly spaced on a line — no ground state bunching
        let data: Vec<f64> = (0..20)
            .flat_map(|i| vec![i as f64 * 1.0, 0.0])
            .collect();
        let n = 20;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = BoseEinsteinLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty results");

        let occ = &result["occupation_numbers"];
        assert!(occ.len() >= 3, "should have multiple energy bins");

        let bec_score = result["bec_score"][0];
        assert!(bec_score.is_finite());

        let g2 = result["bunching_g2"][0];
        assert!(g2.is_finite(), "g2 must be finite");
    }

    /// Test with minimal valid input (n=4)
    #[test]
    fn test_bec_minimal_input() {
        let data = vec![0.0, 1.0, 2.0, 3.0];
        let n = 4;
        let d = 1;
        let shared = SharedData::compute(&data, n, d);
        let lens = BoseEinsteinLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty());
        assert!(result["bec_score"][0].is_finite());
        assert!(result["effective_temperature"][0] > 0.0);
    }
}
