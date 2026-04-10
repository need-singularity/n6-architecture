use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy, mean_var};

/// MetricLens: Comprehensive metric change tracker — delta/change rates between first and second half.
pub struct MetricLens;

fn ratio(a: f64, b: f64) -> f64 {
    if a.abs() < 1e-15 { if b.abs() < 1e-15 { 0.0 } else { 1.0 } } else { (b / a - 1.0).abs().min(10.0) }
}
fn max_gap(s: &[f64]) -> f64 {
    let mut v: Vec<f64> = s.iter().filter(|v| v.is_finite()).copied().collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v.windows(2).map(|w| (w[1] - w[0]).abs()).fold(0.0_f64, f64::max)
}
fn void_frac(s: &[f64], bins: usize) -> f64 {
    let (mn, mx) = s.iter().fold((f64::MAX, f64::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    let r = (mx - mn).max(1e-15);
    let mut f = vec![false; bins];
    for &v in s { f[((v - mn) / r * (bins - 1) as f64).min((bins - 1) as f64) as usize] = true; }
    1.0 - f.iter().filter(|&&x| x).count() as f64 / bins as f64
}
fn chaos_m(s: &[f64]) -> f64 {
    if s.len() < 2 { return 0.0; }
    s.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<f64>() / (s.len() - 1) as f64
}
fn skew(s: &[f64]) -> f64 {
    let n = s.len() as f64; let m = s.iter().sum::<f64>() / n;
    let sd = (s.iter().map(|x| (x - m).powi(2)).sum::<f64>() / n).sqrt();
    if sd < 1e-15 { 0.0 } else { s.iter().map(|x| ((x - m) / sd).powi(3)).sum::<f64>() / n }
}
fn hurst(s: &[f64]) -> f64 {
    let n = s.len(); if n < 4 { return 0.5; }
    let m = s.iter().sum::<f64>() / n as f64;
    let mut cum = vec![0.0; n]; cum[0] = s[0] - m;
    for i in 1..n { cum[i] = cum[i-1] + (s[i] - m); }
    let r = cum.iter().fold(f64::MIN, |a, &v| a.max(v)) - cum.iter().fold(f64::MAX, |a, &v| a.min(v));
    let sd = (s.iter().map(|v| (v - m).powi(2)).sum::<f64>() / n as f64).sqrt();
    if sd < 1e-15 { 0.5 } else { (r / sd).ln() / (n as f64).ln() }
}
fn row_mag_var(s: &[f64], nn: usize, dd: usize) -> f64 {
    let mags: Vec<f64> = (0..nn).map(|i| (0..dd).map(|j| s[i*dd+j].powi(2)).sum::<f64>().sqrt()).collect();
    let m = mags.iter().sum::<f64>() / nn as f64;
    mags.iter().map(|v| (v - m).powi(2)).sum::<f64>() / nn as f64
}
fn pareto_f(s: &[f64]) -> f64 {
    let mut v: Vec<f64> = s.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let idx = (v.len() as f64 * 0.8) as usize;
    if idx >= v.len() { return 0.0; }
    let top: f64 = v[idx..].iter().sum(); let tot: f64 = v.iter().sum();
    if tot.abs() < 1e-15 { 0.5 } else { top / tot }
}

impl Lens for MetricLens {
    fn name(&self) -> &str { "MetricLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200); let half = max_n / 2;
        if half < 3 { return HashMap::new(); }
        let first = &data[..half * d];
        let second = &data[half * d..(max_n * d).min(data.len())];
        let n2 = max_n - half;
        let (_, v1) = mean_var(first, half, d);
        let (_, v2) = mean_var(second, n2, d);
        let mv1: f64 = v1.iter().sum::<f64>() / d.max(1) as f64;
        let mv2: f64 = v2.iter().sum::<f64>() / d.max(1) as f64;
        // KL divergence
        let bins = 32;
        let hist = |s: &[f64]| -> Vec<f64> {
            let (mn, mx) = s.iter().fold((f64::MAX, f64::MIN), |(l,h),&v|(l.min(v),h.max(v)));
            let r = (mx-mn).max(1e-15);
            let mut h = vec![1e-10; bins];
            for &v in s { h[((v-mn)/r*(bins-1) as f64).min((bins-1) as f64) as usize] += 1.0; }
            let t: f64 = h.iter().sum(); h.iter_mut().for_each(|v| *v /= t); h
        };
        let (h1, h2) = (hist(first), hist(second));
        let kl: f64 = h1.iter().zip(h2.iter()).map(|(&p,&q)| if p>1e-15 { p*(p/q).ln() } else { 0.0 }).sum();
        let cc = ratio(chaos_m(first), chaos_m(second));
        let mut r = HashMap::new();
        r.insert("barrier_height_change".into(), vec![ratio(max_gap(first), max_gap(second))]);
        r.insert("density_peak_change".into(), vec![ratio(mv1, mv2)]);
        r.insert("void_change".into(), vec![ratio(void_frac(first, bins), void_frac(second, bins))]);
        r.insert("kl_divergence_change".into(), vec![kl.min(10.0)]);
        r.insert("chaos_change".into(), vec![cc]);
        r.insert("phi_change".into(), vec![ratio(mv1.sqrt(), mv2.sqrt())]);
        r.insert("hurst_change".into(), vec![ratio(hurst(first), hurst(second))]);
        r.insert("entropy_change".into(), vec![ratio(shannon_entropy(first,32), shannon_entropy(second,32))]);
        r.insert("symmetry_change".into(), vec![ratio(skew(first).abs(), skew(second).abs())]);
        r.insert("degree_variance_change".into(), vec![ratio(row_mag_var(first,half,d), row_mag_var(second,n2,d))]);
        r.insert("lyapunov_change".into(), vec![cc]);
        r.insert("pareto_change".into(), vec![ratio(pareto_f(first), pareto_f(second))]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1).sin()).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = MetricLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.len() >= 12);
    }
}
