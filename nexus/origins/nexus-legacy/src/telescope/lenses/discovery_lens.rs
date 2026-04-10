use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// DiscoveryLens: 데이터에서 상수/수식/패턴을 자동 발견하고 그레이딩하는 만능 렌즈.
///
/// 기능:
///   1. 상수 발견: 데이터 통계량에서 n=6 상수 자동 매칭 + EXACT/CLOSE/WEAK 등급
///   2. 수식 발견: 차원 간 관계에서 σ·φ=n·τ 패밀리 수식 탐지
///   3. 비율 지도: 모든 차원 쌍 비율을 n=6 상수맵에 매핑
///   4. 자동 그레이딩: 발견된 패턴을 EXACT(<1%)/CLOSE(<5%)/WEAK(<10%)/FAIL 분류
///   5. Atlas 연결: 발견 밀도/품질로 atlas 등록 우선순위 산출
///
/// Outputs:
///   - discovered_constants: n=6 상수 매칭 총 수
///   - exact_count / close_count / weak_count / fail_count
///   - formula_candidates: 수식 관계 발견 수
///   - atlas_priority: 0-1, atlas 등록 우선순위 (높을수록 중요)
///   - discovery_density: 발견/전체 탐색 비율
///   - best_formula_type: 가장 강한 수식 관계 유형 인덱스
///   - grading_distribution: [exact%, close%, weak%, fail%]
///   - ratio_map_coverage: 비율 지도 커버리지 (n=6 상수 중 매칭된 비율)
pub struct DiscoveryLens;

/// n=6 상수 패밀리 — 전체 매칭 대상
const N6_FAMILY: &[(f64, &str)] = &[
    (1.0, "mu(6)"),
    (2.0, "phi(6)"),
    (3.0, "n/phi"),
    (4.0, "tau(6)"),
    (5.0, "sopfr(6)"),
    (6.0, "n"),
    (7.0, "sigma-sopfr"),
    (8.0, "sigma-tau"),
    (10.0, "sigma-phi"),
    (11.0, "sigma-mu"),
    (12.0, "sigma(6)"),
    (20.0, "J2-tau"),
    (24.0, "J2(6)"),
    (48.0, "sigma*tau"),
    (72.0, "sigma*n"),
    (120.0, "sigma*(sigma-phi)"),
    (144.0, "sigma^2"),
    (288.0, "sigma*J2"),
    // 분수/소수
    (0.1, "1/(sigma-phi)"),
    (0.5, "1/phi"),
    (0.333333, "1/(n/phi)"),
    (0.166667, "1/n"),
    (0.083333, "1/sigma"),
    (0.041667, "1/J2"),
    (0.25, "1/tau"),
    (0.2, "1/sopfr"),
    (0.125, "1/(sigma-tau)"),
    // 특수 상수
    (0.28768, "ln(4/3)"),
    (0.69315, "ln(2)"),
    (1.33333, "4/3"),
    (1.20, "sigma/(sigma-phi)=PUE"),
    (0.95, "1-1/(J2-tau)"),
    (0.63212, "1-1/e"),
    (2.44949, "sqrt(n)"),
    (3.46410, "sqrt(sigma)"),
];

/// 수식 패턴 유형 (차원 간 관계)
/// 0: 곱 관계 (a*b ≈ n6), 1: 비 관계 (a/b ≈ n6), 2: 합 관계 (a+b ≈ n6),
/// 3: 차 관계 (a-b ≈ n6), 4: 거듭제곱 (a^k ≈ n6)
const FORMULA_TYPES: usize = 5;

impl Lens for DiscoveryLens {
    fn name(&self) -> &str {
        "DiscoveryLens"
    }

    fn category(&self) -> &str {
        "T0" // 핵심 렌즈 — 모든 스캔에 포함
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d < 1 {
            return HashMap::new();
        }

        let dims = d.min(24);
        let mut exact = 0u32;
        let mut close = 0u32;
        let mut weak = 0u32;
        let mut fail = 0u32;
        let mut formula_candidates = 0u32;
        let mut formula_type_scores = vec![0u32; FORMULA_TYPES];
        let mut matched_constants: Vec<bool> = vec![false; N6_FAMILY.len()];
        let mut total_probes = 0u32;

        // ── 1. 차원별 통계량 추출 ──
        let mut means = Vec::with_capacity(dims);
        let mut stds = Vec::with_capacity(dims);
        for di in 0..dims {
            let col: Vec<f64> = (0..n).map(|r| data[r * d + di]).collect();
            let m = col.iter().sum::<f64>() / n as f64;
            let v = col.iter().map(|x| (x - m).powi(2)).sum::<f64>() / n as f64;
            means.push(m);
            stds.push(v.sqrt());
        }

        // ── 2. 단일 통계량 → 상수 매칭 ──
        let features: Vec<f64> = means
            .iter()
            .chain(stds.iter())
            .copied()
            .filter(|v| v.is_finite() && v.abs() > 1e-15)
            .collect();

        for &feat in &features {
            let (grade, idx) = grade_value(feat);
            total_probes += 1;
            match grade {
                Grade::Exact => { exact += 1; matched_constants[idx] = true; }
                Grade::Close => { close += 1; matched_constants[idx] = true; }
                Grade::Weak  => { weak += 1; }
                Grade::Fail  => { fail += 1; }
            }
        }

        // ── 3. 차원 쌍 비율/곱/합/차 → 수식 발견 ──
        for i in 0..dims {
            for j in (i + 1)..dims {
                let mi = means[i];
                let mj = means[j];

                // 비율 (a/b)
                if mj.abs() > 1e-12 {
                    let ratio = (mi / mj).abs();
                    total_probes += 1;
                    let (g, idx) = grade_value(ratio);
                    if matches!(g, Grade::Exact | Grade::Close) {
                        formula_candidates += 1;
                        formula_type_scores[1] += 1;
                        matched_constants[idx] = true;
                    }
                }

                // 곱 (a*b)
                let prod = (mi * mj).abs();
                if prod > 1e-12 && prod < 1e6 {
                    total_probes += 1;
                    let (g, idx) = grade_value(prod);
                    if matches!(g, Grade::Exact | Grade::Close) {
                        formula_candidates += 1;
                        formula_type_scores[0] += 1;
                        matched_constants[idx] = true;
                    }
                }

                // 합 (a+b)
                let sum_v = mi + mj;
                if sum_v.abs() > 1e-12 {
                    total_probes += 1;
                    let (g, idx) = grade_value(sum_v.abs());
                    if matches!(g, Grade::Exact | Grade::Close) {
                        formula_candidates += 1;
                        formula_type_scores[2] += 1;
                        matched_constants[idx] = true;
                    }
                }

                // 차 (|a-b|)
                let diff = (mi - mj).abs();
                if diff > 1e-12 {
                    total_probes += 1;
                    let (g, idx) = grade_value(diff);
                    if matches!(g, Grade::Exact | Grade::Close) {
                        formula_candidates += 1;
                        formula_type_scores[3] += 1;
                        matched_constants[idx] = true;
                    }
                }
            }
        }

        // ── 4. 거듭제곱 탐색 (a^2, a^3, sqrt(a)) ──
        for &m in &means {
            let abs_m = m.abs();
            if abs_m < 1e-12 || abs_m > 1e4 {
                continue;
            }
            for &power in &[2.0_f64, 3.0, 0.5] {
                let val = abs_m.powf(power);
                if val.is_finite() && val < 1e6 {
                    total_probes += 1;
                    let (g, idx) = grade_value(val);
                    if matches!(g, Grade::Exact | Grade::Close) {
                        formula_candidates += 1;
                        formula_type_scores[4] += 1;
                        matched_constants[idx] = true;
                    }
                }
            }
        }

        // ── 5. 거리 통계량 → 상수 매칭 ──
        let pair_count = n * (n - 1) / 2;
        if pair_count > 0 {
            let mut dists: Vec<f64> = Vec::with_capacity(pair_count.min(10000));
            for i in 0..n.min(150) {
                for j in (i + 1)..n.min(150) {
                    dists.push(shared.dist(i, j));
                }
            }
            if !dists.is_empty() {
                dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let median = dists[dists.len() / 2];
                let mean_d = dists.iter().sum::<f64>() / dists.len() as f64;
                for &feat in &[median, mean_d, dists[0], *dists.last().unwrap()] {
                    if feat.is_finite() && feat.abs() > 1e-15 {
                        total_probes += 1;
                        let (g, idx) = grade_value(feat);
                        match g {
                            Grade::Exact => { exact += 1; matched_constants[idx] = true; }
                            Grade::Close => { close += 1; matched_constants[idx] = true; }
                            Grade::Weak  => { weak += 1; }
                            Grade::Fail  => { fail += 1; }
                        }
                    }
                }
            }
        }

        // ── 6. 그레이딩 종합 ──
        let total_discovered = exact + close + weak;
        total_probes = total_probes.max(1);
        let discovery_density = total_discovered as f64 / total_probes as f64;
        let ratio_map_coverage =
            matched_constants.iter().filter(|&&b| b).count() as f64 / N6_FAMILY.len() as f64;

        // Atlas 우선순위: EXACT 비중 + 수식 발견 + 커버리지
        let exact_ratio = exact as f64 / total_probes as f64;
        let formula_bonus = (formula_candidates as f64 / total_probes as f64).min(0.3);
        let atlas_priority = (exact_ratio * 0.5 + formula_bonus + ratio_map_coverage * 0.2).min(1.0);

        // 가장 강한 수식 유형
        let best_formula = formula_type_scores
            .iter()
            .enumerate()
            .max_by_key(|&(_, &v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0);

        // 그레이딩 분포 (%)
        let total_all = (exact + close + weak + fail).max(1) as f64;
        let grading_dist = vec![
            exact as f64 / total_all,
            close as f64 / total_all,
            weak as f64 / total_all,
            fail as f64 / total_all,
        ];

        let mut result = HashMap::new();
        result.insert("discovered_constants".into(), vec![total_discovered as f64]);
        result.insert("exact_count".into(), vec![exact as f64]);
        result.insert("close_count".into(), vec![close as f64]);
        result.insert("weak_count".into(), vec![weak as f64]);
        result.insert("fail_count".into(), vec![fail as f64]);
        result.insert("formula_candidates".into(), vec![formula_candidates as f64]);
        result.insert("atlas_priority".into(), vec![atlas_priority]);
        result.insert("discovery_density".into(), vec![discovery_density]);
        result.insert("best_formula_type".into(), vec![best_formula as f64]);
        result.insert("grading_distribution".into(), grading_dist);
        result.insert("ratio_map_coverage".into(), vec![ratio_map_coverage]);
        result.insert("total_probes".into(), vec![total_probes as f64]);
        result.insert(
            "formula_type_scores".into(),
            formula_type_scores.iter().map(|&v| v as f64).collect(),
        );
        result
    }
}

enum Grade {
    Exact,
    Close,
    Weak,
    Fail,
}

/// 값을 n=6 상수 패밀리와 매칭, (등급, 가장 가까운 상수 인덱스) 반환
fn grade_value(value: f64) -> (Grade, usize) {
    let mut best_dev = f64::MAX;
    let mut best_idx = 0;

    for (i, &(constant, _)) in N6_FAMILY.iter().enumerate() {
        if constant.abs() < 1e-15 {
            continue;
        }
        let dev = ((value - constant) / constant).abs();
        if dev < best_dev {
            best_dev = dev;
            best_idx = i;
        }
    }

    let grade = if best_dev < 0.01 {
        Grade::Exact
    } else if best_dev < 0.05 {
        Grade::Close
    } else if best_dev < 0.10 {
        Grade::Weak
    } else {
        Grade::Fail
    };

    (grade, best_idx)
}
