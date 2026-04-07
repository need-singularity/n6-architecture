/// sedi_core — Rust acceleration module for SEDI real-time signal processing.
///
/// Exposed to Python via PyO3/maturin.  Build with:
///   cd sedi-core && maturin develop          # dev (debug)
///   cd sedi-core && maturin develop --release # dev (optimised)
///   cd sedi-core && maturin build --release   # wheel
///
/// Python import:
///   import sedi_core
///   result = sedi_core.fast_fft(data, window_sizes)
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rustfft::{FftPlanner, num_complex::Complex};
use std::collections::HashMap;
use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Helper types
// ---------------------------------------------------------------------------

/// A single hit from the ratio-scan pass.
#[pyclass]
#[derive(Clone, Debug)]
pub struct RatioHit {
    /// Index of the first element in the pair (data[pos] / data[pos+1]).
    #[pyo3(get)]
    pub pos: usize,

    /// The target ratio that was matched.
    #[pyo3(get)]
    pub target: f64,

    /// The observed ratio at this position.
    #[pyo3(get)]
    pub observed: f64,

    /// Z-score: how many standard deviations away from the target the
    /// observed ratio sits (relative to the local std of all ratios).
    #[pyo3(get)]
    pub z_score: f64,
}

#[pymethods]
impl RatioHit {
    fn __repr__(&self) -> String {
        format!(
            "RatioHit(pos={}, target={:.4}, observed={:.4}, z_score={:.3})",
            self.pos, self.target, self.observed, self.z_score
        )
    }
}

// ---------------------------------------------------------------------------
// 1. fast_fft
// ---------------------------------------------------------------------------

/// Compute windowed FFT magnitude spectra at multiple window sizes simultaneously.
///
/// Parameters
/// ----------
/// data : list[float]
///     Raw signal samples (real-valued).
/// window_sizes : list[int]
///     List of window sizes to process.  Each size must be <= len(data).
///
/// Returns
/// -------
/// dict[int, list[float]]
///     Mapping from window_size -> FFT magnitude spectrum (length = window_size // 2 + 1).
///     Only the non-redundant (positive-frequency) half is returned.
///     A Hann window is applied before the FFT.
///
/// Notes
/// -----
/// * Uses rustfft for cache-friendly Cooley-Tukey decomposition.
/// * Each window is taken from the *last* `window_size` samples of `data` so
///   the function always operates on the most recent segment — which is the
///   natural choice for real-time monitoring.
#[pyfunction]
fn fast_fft(
    py: Python<'_>,
    data: Vec<f64>,
    window_sizes: Vec<usize>,
) -> PyResult<PyObject> {
    let dict = PyDict::new_bound(py);

    let mut planner: FftPlanner<f64> = FftPlanner::new();

    for &win in &window_sizes {
        if win == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "window_size must be > 0",
            ));
        }
        if win > data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "window_size {} > data length {}",
                win,
                data.len()
            )));
        }

        // Take the most recent `win` samples.
        let start = data.len() - win;
        let segment = &data[start..];

        // Apply Hann window and convert to Complex<f64>.
        let mut buf: Vec<Complex<f64>> = segment
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let w = 0.5 * (1.0 - (2.0 * PI * i as f64 / (win - 1) as f64).cos());
                Complex::new(x * w, 0.0)
            })
            .collect();

        // Forward FFT in-place.
        let fft = planner.plan_fft_forward(win);
        fft.process(&mut buf);

        // Return only the positive-frequency half (DC … Nyquist).
        let n_out = win / 2 + 1;
        let scale = 1.0 / win as f64;
        let magnitudes: Vec<f64> = buf[..n_out]
            .iter()
            .map(|c| c.norm() * scale)
            .collect();

        dict.set_item(win, magnitudes)?;
    }

    Ok(dict.into())
}

// ---------------------------------------------------------------------------
// 2. fast_detect_ratios
// ---------------------------------------------------------------------------

/// Scan consecutive element ratios in `data` and return positions that match
/// any of the supplied target ratios within `tolerance`.
///
/// Parameters
/// ----------
/// data : list[float]
///     Input signal.  Length must be >= 2.
/// targets : list[float]
///     List of ratio values to search for (e.g. [1.618, 2.0, 0.5]).
/// tolerance : float
///     Absolute tolerance for matching: |observed - target| <= tolerance.
///
/// Returns
/// -------
/// list[RatioHit]
///     Each hit contains the position, which target was matched, the observed
///     ratio, and a Z-score computed against the distribution of *all*
///     consecutive ratios in the data.
///
/// Notes
/// -----
/// * Division by zero (when data[i+1] == 0) is silently skipped.
/// * The Z-score uses the global mean and std of all valid ratios, so
///   extreme outliers will have large magnitudes.
#[pyfunction]
fn fast_detect_ratios(
    data: Vec<f64>,
    targets: Vec<f64>,
    tolerance: f64,
) -> PyResult<Vec<RatioHit>> {
    if data.len() < 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "data must have at least 2 elements",
        ));
    }

    // Compute all consecutive ratios, skipping zero denominators.
    let ratios: Vec<(usize, f64)> = data
        .windows(2)
        .enumerate()
        .filter_map(|(i, w)| {
            if w[1] == 0.0 {
                None
            } else {
                Some((i, w[0] / w[1]))
            }
        })
        .collect();

    if ratios.is_empty() {
        return Ok(vec![]);
    }

    // Global mean and std of all ratios (used for Z-score).
    let n = ratios.len() as f64;
    let mean = ratios.iter().map(|(_, r)| r).sum::<f64>() / n;
    let variance = ratios.iter().map(|(_, r)| (r - mean).powi(2)).sum::<f64>() / n;
    let std = variance.sqrt();

    let mut hits: Vec<RatioHit> = Vec::new();

    for &(pos, observed) in &ratios {
        for &target in &targets {
            if (observed - target).abs() <= tolerance {
                let z_score = if std > 1e-12 {
                    (observed - mean) / std
                } else {
                    0.0
                };
                hits.push(RatioHit {
                    pos,
                    target,
                    observed,
                    z_score,
                });
                // A position can match multiple targets — keep searching.
            }
        }
    }

    Ok(hits)
}

// ---------------------------------------------------------------------------
// 3. fast_runs_test
// ---------------------------------------------------------------------------

/// Wald-Wolfowitz runs test for randomness.
///
/// Parameters
/// ----------
/// data : list[float]
///     Input sequence.  Must have at least 2 elements.
///
/// Returns
/// -------
/// (runs: int, expected: float, z_score: float)
///     `runs`    — observed number of runs (maximal consecutive subsequences
///                 of values above/below the median).
///     `expected` — expected number of runs under the null hypothesis of
///                  independence.
///     `z_score`  — standard normal test statistic.  |z| > 1.96 → p < 0.05.
///
/// Notes
/// -----
/// Values exactly equal to the median are dropped before the test (common
/// convention when the median falls on a data point).
#[pyfunction]
fn fast_runs_test(data: Vec<f64>) -> PyResult<(usize, f64, f64)> {
    if data.len() < 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "data must have at least 2 elements",
        ));
    }

    // Compute median (sort a copy).
    let mut sorted = data.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = sorted.len() / 2;
    let median = if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    };

    // Binarise: true = above median, false = below.  Drop ties.
    let binary: Vec<bool> = data
        .iter()
        .filter_map(|&x| {
            if x > median {
                Some(true)
            } else if x < median {
                Some(false)
            } else {
                None // drop ties
            }
        })
        .collect();

    let n = binary.len();
    if n < 2 {
        return Ok((0, 0.0, 0.0));
    }

    // Count runs and n1/n2 (above/below counts).
    let mut runs: usize = 1;
    let mut n1: usize = if binary[0] { 1 } else { 0 };
    for i in 1..n {
        if binary[i] != binary[i - 1] {
            runs += 1;
        }
        if binary[i] {
            n1 += 1;
        }
    }
    let n2 = n - n1;

    if n1 == 0 || n2 == 0 {
        // All values on one side — perfectly non-random.
        return Ok((runs, 1.0, f64::INFINITY));
    }

    let n1f = n1 as f64;
    let n2f = n2 as f64;
    let nf = n as f64;

    // Expected runs and variance under H0.
    let expected = (2.0 * n1f * n2f / nf) + 1.0;
    let variance =
        (2.0 * n1f * n2f * (2.0 * n1f * n2f - nf)) / (nf * nf * (nf - 1.0));

    let z_score = if variance > 1e-12 {
        (runs as f64 - expected) / variance.sqrt()
    } else {
        0.0
    };

    Ok((runs, expected, z_score))
}

// ---------------------------------------------------------------------------
// 4. fast_entropy
// ---------------------------------------------------------------------------

/// Compute binned Shannon entropy of a signal.
///
/// Parameters
/// ----------
/// data : list[float]
///     Input samples.
/// n_bins : int
///     Number of equal-width histogram bins.  Must be >= 2.
///
/// Returns
/// -------
/// float
///     Shannon entropy in *nats* (natural logarithm base).
///     Maximum possible value is ln(n_bins) when the distribution is uniform.
///
/// Notes
/// -----
/// Bins with zero count contribute 0 to the sum (0 * ln(0) = 0 by convention).
#[pyfunction]
fn fast_entropy(data: Vec<f64>, n_bins: usize) -> PyResult<f64> {
    if n_bins < 2 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "n_bins must be >= 2",
        ));
    }
    if data.is_empty() {
        return Ok(0.0);
    }

    // Find data range.
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    if (max - min).abs() < 1e-15 {
        // All values identical — zero entropy.
        return Ok(0.0);
    }

    let bin_width = (max - min) / n_bins as f64;
    let mut counts = vec![0usize; n_bins];

    for &x in &data {
        // Clamp the last element into the final bin (handles x == max).
        let bin = ((x - min) / bin_width) as usize;
        let bin = bin.min(n_bins - 1);
        counts[bin] += 1;
    }

    let n = data.len() as f64;
    let entropy = counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / n;
            -p * p.ln()
        })
        .sum::<f64>();

    Ok(entropy)
}

// ---------------------------------------------------------------------------
// 5. n=6 constant matching
// ---------------------------------------------------------------------------

/// Base n=6 arithmetic constants.
const N6_N: f64 = 6.0;
const N6_PHI: f64 = 2.0;       // phi(6) = 2  (Euler totient)
const N6_TAU: f64 = 4.0;       // tau(6) = 4  (number of divisors)
const N6_SIGMA: f64 = 12.0;    // sigma(6) = 12  (sum of divisors)
const N6_J2: f64 = 24.0;       // Jordan J_2(6) = 24
const N6_SOPFR: f64 = 5.0;     // sopfr(6) = 2+3 = 5
const N6_MU: f64 = 1.0;        // mu(6) = 1  (Mobius)

struct N6Const {
    name: &'static str,
    value: f64,
}

fn n6_base() -> Vec<N6Const> {
    vec![
        N6Const { name: "n",     value: N6_N },
        N6Const { name: "phi",   value: N6_PHI },
        N6Const { name: "tau",   value: N6_TAU },
        N6Const { name: "sigma", value: N6_SIGMA },
        N6Const { name: "J2",    value: N6_J2 },
        N6Const { name: "sopfr", value: N6_SOPFR },
        N6Const { name: "mu",    value: N6_MU },
        N6Const { name: "pi",    value: PI },
        N6Const { name: "e",     value: std::f64::consts::E },
    ]
}

/// Generate all depth-2 n=6 expressions and return (formula_string, value) pairs.
fn n6_depth2_expressions() -> Vec<(String, f64)> {
    let base = n6_base();
    let ops: [(fn(f64, f64) -> Option<f64>, &str); 5] = [
        (|a, b| Some(a + b), "+"),
        (|a, b| Some(a - b), "-"),
        (|a, b| Some(a * b), "*"),
        (|a, b| if b.abs() < 1e-15 { None } else { Some(a / b) }, "/"),
        (|a, b| {
            if a.abs() > 1e6 || b.abs() > 50.0 { return None; }
            if a.abs() < 1e-15 && b < 0.0 { return None; }
            let r = a.powf(b);
            if r.is_finite() && r.abs() < 1e15 { Some(r) } else { None }
        }, "^"),
    ];

    let mut results: Vec<(String, f64)> = Vec::new();

    // Depth 0: base constants
    for c in &base {
        results.push((c.name.to_string(), c.value));
    }

    // Depth 1: a op b
    for (op_fn, op_sym) in &ops {
        for a in &base {
            for b in &base {
                if let Some(v) = op_fn(a.value, b.value) {
                    if v.is_finite() && v.abs() < 1e15 {
                        results.push((format!("{} {} {}", a.name, op_sym, b.name), v));
                    }
                }
            }
        }
    }

    // Depth 2: (a op1 b) op2 c
    for (op1_fn, op1_sym) in &ops {
        for a in &base {
            for b in &base {
                if let Some(v1) = op1_fn(a.value, b.value) {
                    if !v1.is_finite() || v1.abs() >= 1e15 { continue; }
                    for (op2_fn, op2_sym) in &ops {
                        for c in &base {
                            if let Some(v2) = op2_fn(v1, c.value) {
                                if v2.is_finite() && v2.abs() < 1e15 {
                                    results.push((
                                        format!("({} {} {}) {} {}", a.name, op1_sym, b.name, op2_sym, c.name),
                                        v2,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    results
}

/// Check if a value matches any n=6 expression (depth-2).
///
/// Returns a list of (formula, computed_value, relative_error) for all matches
/// within the given tolerance (relative error).
#[pyfunction]
fn n6_match(value: f64, tolerance: f64) -> Vec<(String, f64, f64)> {
    if !value.is_finite() || value == 0.0 {
        return vec![];
    }

    let exprs = n6_depth2_expressions();
    let mut hits: Vec<(String, f64, f64)> = Vec::new();

    for (formula, computed) in &exprs {
        if *computed == 0.0 { continue; }
        let rel_err = ((value - computed) / value).abs();
        if rel_err <= tolerance {
            hits.push((formula.clone(), *computed, rel_err));
        }
    }

    // Sort by error (best matches first)
    hits.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    hits
}

/// Return all base + derived n=6 constants as a HashMap.
///
/// Includes base constants and all depth-1 binary combinations.
#[pyfunction]
fn n6_constants() -> HashMap<String, f64> {
    let exprs = n6_depth2_expressions();
    let mut map = HashMap::new();
    for (name, value) in exprs {
        map.entry(name).or_insert(value);
    }
    map
}

/// Batch version of n6_match for multiple values.
///
/// Returns a list of lists — one inner list per input value.
#[pyfunction]
fn batch_n6_match(values: Vec<f64>, tolerance: f64) -> Vec<Vec<(String, f64, f64)>> {
    // Pre-compute expressions once for all values
    let exprs = n6_depth2_expressions();

    values.iter().map(|&value| {
        if !value.is_finite() || value == 0.0 {
            return vec![];
        }

        let mut hits: Vec<(String, f64, f64)> = Vec::new();
        for (formula, computed) in &exprs {
            if *computed == 0.0 { continue; }
            let rel_err = ((value - computed) / value).abs();
            if rel_err <= tolerance {
                hits.push((formula.clone(), *computed, rel_err));
            }
        }
        hits.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        hits
    }).collect()
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

/// sedi_core — Rust-accelerated signal processing primitives for SEDI.
#[pymodule]
fn sedi_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RatioHit>()?;
    m.add_function(wrap_pyfunction!(fast_fft, m)?)?;
    m.add_function(wrap_pyfunction!(fast_detect_ratios, m)?)?;
    m.add_function(wrap_pyfunction!(fast_runs_test, m)?)?;
    m.add_function(wrap_pyfunction!(fast_entropy, m)?)?;
    m.add_function(wrap_pyfunction!(n6_match, m)?)?;
    m.add_function(wrap_pyfunction!(n6_constants, m)?)?;
    m.add_function(wrap_pyfunction!(batch_n6_match, m)?)?;
    Ok(())
}
