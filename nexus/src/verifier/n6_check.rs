/// N=6 constant definition: (name, value)
const N6_CONSTANTS: &[(&str, f64)] = &[
    ("n", 6.0),
    ("sigma", 12.0),
    ("phi", 2.0),
    ("tau", 4.0),
    ("J2", 24.0),
    ("sopfr", 5.0),
    ("mu", 1.0),
    ("sigma-phi", 10.0),
    ("sigma-tau", 8.0),
    ("sigma-mu", 11.0),
    ("sigma*tau", 48.0),
    ("sigma^2", 144.0),
    ("phi^tau", 16.0),
    ("tau^2/sigma", 1.333_333_333),
    ("n/phi", 3.0),
    ("sigma/n", 2.0),    // same value as phi, but different semantic
    ("J2/sigma", 2.0),   // same value as phi, but different semantic
    ("J2-tau", 20.0),
    ("sigma*phi", 24.0), // = J2, different derivation
    ("ln(4/3)", 0.287_682_072),
];

/// Match a value against n=6 constants.
/// Returns (constant_name, match_quality) where quality is:
///   1.0 = EXACT (< 0.1% error)
///   0.8 = CLOSE (< 5% error)
///   0.5 = WEAK  (< 10% error)
///   0.0 = NONE
pub fn n6_match(value: f64) -> (&'static str, f64) {
    if value == 0.0 {
        return ("none", 0.0);
    }

    let mut best_name = "none";
    let mut best_quality = 0.0_f64;
    let mut best_error = f64::MAX;

    for &(name, constant) in N6_CONSTANTS {
        if constant == 0.0 {
            continue;
        }
        let error = ((value - constant) / constant).abs();

        let quality = if error < 0.001 {
            1.0 // EXACT
        } else if error < 0.05 {
            0.8 // CLOSE
        } else if error < 0.10 {
            0.5 // WEAK
        } else {
            0.0
        };

        // Pick highest quality; break ties by smallest error (skip quality=0)
        if quality > 0.0
            && (quality > best_quality || (quality == best_quality && error < best_error))
        {
            best_name = name;
            best_quality = quality;
            best_error = error;
        }
    }

    (best_name, best_quality)
}

/// Fraction of values that match EXACT (quality == 1.0) against any n=6 constant.
pub fn n6_exact_ratio(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let exact_count = values.iter().filter(|&&v| n6_match(v).1 >= 1.0).count();
    exact_count as f64 / values.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_count() {
        assert!(N6_CONSTANTS.len() >= 20, "Need 20+ constants, got {}", N6_CONSTANTS.len());
    }
}
