/// Comparison engine — systematic A vs B experiment comparison.

#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub a_name: String,
    pub b_name: String,
    pub phi_a: f64,
    pub phi_b: f64,
    pub winner: String,
    pub effect_size: f64,
    pub statistically_significant: bool,
    pub details: Vec<(String, f64, f64)>,
}

/// Compare two sets of metrics (A vs B).
///
/// Each metric set is a slice of (metric_name, value) pairs.
/// Returns a structured comparison with effect size and significance.
pub fn compare(
    a_name: &str,
    a_metrics: &[(String, f64)],
    b_name: &str,
    b_metrics: &[(String, f64)],
) -> ComparisonResult {
    // Build lookup for B metrics
    let b_map: std::collections::HashMap<&str, f64> = b_metrics
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();

    let mut details: Vec<(String, f64, f64)> = Vec::new();
    let mut a_wins = 0usize;
    let mut b_wins = 0usize;
    let mut diffs: Vec<f64> = Vec::new();

    let mut phi_a = 0.0;
    let mut phi_b = 0.0;

    for (name, a_val) in a_metrics {
        if let Some(&b_val) = b_map.get(name.as_str()) {
            details.push((name.clone(), *a_val, b_val));
            let diff = a_val - b_val;
            diffs.push(diff);

            if diff > 1e-12 {
                a_wins += 1;
            } else if diff < -1e-12 {
                b_wins += 1;
            }

            // Track phi-related metric
            if name.to_lowercase().contains("phi") || name.to_lowercase().contains("score") {
                phi_a = *a_val;
                phi_b = b_val;
            }
        }
    }

    // If no phi-specific metric, use mean of all metrics
    if phi_a == 0.0 && phi_b == 0.0 && !a_metrics.is_empty() {
        phi_a = a_metrics.iter().map(|(_, v)| v).sum::<f64>() / a_metrics.len() as f64;
        phi_b = b_metrics.iter().map(|(_, v)| v).sum::<f64>() / b_metrics.len() as f64;
    }

    // Cohen's d effect size
    let effect_size = cohens_d(&diffs);

    // Statistical significance: |d| > 0.5 (medium effect) with enough metrics
    let statistically_significant = effect_size.abs() > 0.5 && diffs.len() >= 3;

    let winner = if a_wins > b_wins && statistically_significant {
        "A".to_string()
    } else if b_wins > a_wins && statistically_significant {
        "B".to_string()
    } else {
        "Tie".to_string()
    };

    ComparisonResult {
        a_name: a_name.to_string(),
        b_name: b_name.to_string(),
        phi_a,
        phi_b,
        winner,
        effect_size,
        statistically_significant,
        details,
    }
}

/// Rank multiple experiment results by aggregate score.
///
/// Each entry is (name, metrics) where metrics = [(metric_name, value)].
/// Returns sorted list of (name, aggregate_score) in descending order.
pub fn rank(results: &[(String, Vec<(String, f64)>)]) -> Vec<(String, f64)> {
    let mut scored: Vec<(String, f64)> = results
        .iter()
        .map(|(name, metrics)| {
            let score = if metrics.is_empty() {
                0.0
            } else {
                metrics.iter().map(|(_, v)| v).sum::<f64>() / metrics.len() as f64
            };
            (name.clone(), score)
        })
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored
}

/// Cohen's d: mean of differences / pooled std of differences.
fn cohens_d(diffs: &[f64]) -> f64 {
    if diffs.is_empty() {
        return 0.0;
    }
    let n = diffs.len() as f64;
    let mean = diffs.iter().sum::<f64>() / n;
    let var = diffs.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / n;
    let std = var.sqrt();
    if std < 1e-12 {
        if mean.abs() < 1e-12 { 0.0 } else { mean.signum() * f64::INFINITY }
    } else {
        mean / std
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_a_wins() {
        let a = vec![
            ("phi".to_string(), 10.0),
            ("entropy".to_string(), 8.0),
            ("n6_score".to_string(), 0.9),
        ];
        let b = vec![
            ("phi".to_string(), 5.0),
            ("entropy".to_string(), 4.0),
            ("n6_score".to_string(), 0.4),
        ];
        let result = compare("A_exp", &a, "B_exp", &b);
        assert_eq!(result.winner, "A");
        assert!(result.effect_size > 0.0);
    }

    #[test]
    fn test_compare_tie() {
        let a = vec![
            ("phi".to_string(), 6.0),
            ("entropy".to_string(), 12.0),
        ];
        let b = vec![
            ("phi".to_string(), 6.0),
            ("entropy".to_string(), 12.0),
        ];
        let result = compare("X", &a, "Y", &b);
        assert_eq!(result.winner, "Tie");
    }

    #[test]
    fn test_rank() {
        let results = vec![
            ("low".to_string(), vec![("score".to_string(), 1.0)]),
            ("high".to_string(), vec![("score".to_string(), 10.0)]),
            ("mid".to_string(), vec![("score".to_string(), 5.0)]),
        ];
        let ranked = rank(&results);
        assert_eq!(ranked[0].0, "high");
        assert_eq!(ranked[1].0, "mid");
        assert_eq!(ranked[2].0, "low");
    }
}
