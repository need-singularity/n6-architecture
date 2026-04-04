use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// GameTheoryLens: Detect strategic interaction patterns.
///
/// Interprets data rows as strategy profiles and measures:
/// - Nash equilibrium proximity (no unilateral improvement)
/// - Cooperation vs defection balance
/// - Payoff symmetry
/// n=6 connection: 6 = smallest perfect number = complete cooperation
/// (all proper divisors sum to self). Egyptian fraction 1/2+1/3+1/6=1
/// mirrors fair allocation in cooperative games.
pub struct GameTheoryLens;

impl Lens for GameTheoryLens {
    fn name(&self) -> &str { "GameTheoryLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 || d < 2 { return HashMap::new(); }

        let nn = n.min(50);

        // Interpret each row as a "payoff vector" for a player/strategy.
        // Nash equilibrium check: for each row, measure how much
        // switching to another row would improve the "payoff" (L2 norm).

        let mut payoffs: Vec<f64> = Vec::with_capacity(nn);
        for i in 0..nn {
            let row_start = i * d;
            let row_end = row_start + d;
            let norm: f64 = data[row_start..row_end].iter().map(|x| x * x).sum::<f64>();
            payoffs.push(norm.sqrt());
        }

        // Nash deviation: max improvement any player could get by switching
        let mean_payoff = payoffs.iter().sum::<f64>() / nn as f64;
        let max_payoff = payoffs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let nash_deviation = if mean_payoff > 1e-12 {
            (max_payoff - mean_payoff) / mean_payoff
        } else {
            0.0
        };

        // Nash equilibrium score: low deviation = close to equilibrium
        let nash_score = (-nash_deviation * 6.0).exp(); // n=6 scaling

        // Cooperation index: how similar are payoffs (Gini-like)
        let mut sorted_payoffs = payoffs.clone();
        sorted_payoffs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let total = sorted_payoffs.iter().sum::<f64>();
        let gini = if total > 1e-12 && nn > 1 {
            let mut cum_sum = 0.0;
            let mut area_under = 0.0;
            for (i, &p) in sorted_payoffs.iter().enumerate() {
                cum_sum += p;
                area_under += cum_sum / total - (i as f64 + 1.0) / nn as f64;
            }
            (area_under / nn as f64).abs() * 2.0
        } else {
            0.0
        };
        let cooperation_index = 1.0 - gini.min(1.0);

        // Payoff symmetry via distance matrix
        let mut symmetry_sum = 0.0;
        let mut pair_count = 0;
        for i in 0..nn.min(20) {
            for j in (i + 1)..nn.min(20) {
                let d_ij = shared.dist(i, j);
                let d_ji = shared.dist(j, i);
                symmetry_sum += (d_ij - d_ji).abs();
                pair_count += 1;
            }
        }
        let symmetry = if pair_count > 0 {
            1.0 - (symmetry_sum / pair_count as f64).min(1.0)
        } else {
            1.0
        };

        // Egyptian fraction allocation check: does the data split ~1/2, ~1/3, ~1/6?
        let egyptian_score = if nn >= 6 {
            let third = nn / 3;
            let sixth = nn / 6;
            let half = nn / 2;
            let group_sums = [
                payoffs[..half].iter().sum::<f64>(),
                payoffs[half..half + third].iter().sum::<f64>(),
                payoffs[half + third..].iter().sum::<f64>(),
            ];
            let group_total: f64 = group_sums.iter().sum();
            if group_total > 1e-12 {
                let fracs = [
                    group_sums[0] / group_total,
                    group_sums[1] / group_total,
                    group_sums[2] / group_total,
                ];
                let target = [0.5, 1.0 / 3.0, 1.0 / 6.0];
                let deviation: f64 = fracs.iter().zip(target.iter())
                    .map(|(f, t)| (f - t).abs())
                    .sum();
                (-deviation * 6.0).exp()
            } else { 0.0 }
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("nash_deviation".to_string(), vec![nash_deviation]);
        result.insert("nash_equilibrium_score".to_string(), vec![nash_score]);
        result.insert("cooperation_index".to_string(), vec![cooperation_index]);
        result.insert("payoff_symmetry".to_string(), vec![symmetry]);
        result.insert("egyptian_allocation_score".to_string(), vec![egyptian_score]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_theory_equal_payoffs() {
        // Equal payoffs -> high Nash score, high cooperation
        let n = 12;
        let d = 2;
        let data: Vec<f64> = (0..n * d).map(|_| 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = GameTheoryLens.scan(&data, n, d, &shared);
        assert!(result["nash_equilibrium_score"][0] > 0.9);
        assert!(result["cooperation_index"][0] > 0.9);
    }

    #[test]
    fn test_game_theory_unequal() {
        // One dominant player -> lower Nash score
        let n = 12;
        let d = 2;
        let mut data = vec![1.0; n * d];
        data[0] = 100.0; // player 0 has huge payoff in dim 0
        let shared = SharedData::compute(&data, n, d);
        let result = GameTheoryLens.scan(&data, n, d, &shared);
        assert!(result["nash_deviation"][0] > 0.0);
    }
}
