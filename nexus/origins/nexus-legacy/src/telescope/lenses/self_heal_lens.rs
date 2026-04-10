use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SelfHealLens: 자기 오류 탐지 + 자동 복구 트리거.
///
/// 스캔 데이터의 이상/오염/손상을 감지하고 수정 액션을 제안:
///   1. NaN/Inf 오염 탐지 → 제거/보간 필요 플래그
///   2. 차원 붕괴: 분산=0 차원 → 제거 대상
///   3. 이상치 오염도: 극단값 비율
///   4. 스케일 불균형: 차원 간 스케일 차이 → 정규화 필요
///   5. 데이터 일관성: 시간순서 위배/중복 탐지
///   6. 수정 액션 코드 (CLI 루프에서 자동 처리)
///
/// 액션 코드:
///   0 = CLEAN (문제없음)
///   1 = NORMALIZE (스케일 보정)
///   2 = REMOVE_OUTLIERS (이상치 제거)
///   3 = REMOVE_DIMS (붕괴 차원 제거)
///   4 = INTERPOLATE (NaN 보간)
///   5 = CRITICAL_ERROR (데이터 교체 필요)
pub struct SelfHealLens;

impl Lens for SelfHealLens {
    fn name(&self) -> &str {
        "SelfHealLens"
    }

    fn category(&self) -> &str {
        "T0" // 모든 스캔 전에 실행 — 오류 조기 차단
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 2 || d < 1 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // 1. NaN/Inf 탐지
        let mut nan_count = 0u32;
        let mut inf_count = 0u32;
        for r in 0..n {
            for di in 0..dims {
                let v = data[r * d + di];
                if v.is_nan() {
                    nan_count += 1;
                } else if v.is_infinite() {
                    inf_count += 1;
                }
            }
        }
        let total_cells = (n * dims) as f64;
        let contamination = (nan_count + inf_count) as f64 / total_cells;

        // 2. 차원 붕괴 (분산 ≈ 0)
        let mut collapsed_dims = 0u32;
        let mut dim_scales: Vec<f64> = Vec::with_capacity(dims);
        for di in 0..dims {
            let mut min_v = f64::MAX;
            let mut max_v = f64::MIN;
            let mut sum = 0.0_f64;
            let mut valid = 0u32;
            for r in 0..n {
                let v = data[r * d + di];
                if v.is_finite() {
                    if v < min_v { min_v = v; }
                    if v > max_v { max_v = v; }
                    sum += v;
                    valid += 1;
                }
            }
            let range = max_v - min_v;
            if range < 1e-15 || valid < 2 {
                collapsed_dims += 1;
                dim_scales.push(0.0);
            } else {
                let mean = sum / valid as f64;
                let var: f64 = (0..n)
                    .filter_map(|r| {
                        let v = data[r * d + di];
                        if v.is_finite() { Some((v - mean).powi(2)) } else { None }
                    })
                    .sum::<f64>()
                    / valid as f64;
                dim_scales.push(var.sqrt());
            }
        }

        // 3. 이상치 오염도 (Tukey method)
        let mut outlier_count = 0u32;
        for di in 0..dims {
            if dim_scales[di] < 1e-15 {
                continue;
            }
            let mut col: Vec<f64> = (0..n)
                .map(|r| data[r * d + di])
                .filter(|v| v.is_finite())
                .collect();
            if col.len() < 4 {
                continue;
            }
            col.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let q1 = col[col.len() / 4];
            let q3 = col[col.len() * 3 / 4];
            let iqr = q3 - q1;
            let lower = q1 - 1.5 * iqr;
            let upper = q3 + 1.5 * iqr;
            for &v in &col {
                if v < lower || v > upper {
                    outlier_count += 1;
                }
            }
        }
        let outlier_ratio = outlier_count as f64 / total_cells;

        // 4. 스케일 불균형
        let active_scales: Vec<f64> = dim_scales.iter().filter(|&&s| s > 1e-15).copied().collect();
        let scale_imbalance = if active_scales.len() >= 2 {
            let max_s = active_scales.iter().cloned().fold(0.0_f64, f64::max);
            let min_s = active_scales.iter().cloned().fold(f64::MAX, f64::min);
            if min_s > 1e-15 {
                (max_s / min_s).ln() / 10.0_f64.ln() // decades of imbalance
            } else {
                10.0
            }
        } else {
            0.0
        };
        let needs_normalization = scale_imbalance > 2.0; // > 100x difference

        // 5. 중복 행 탐지
        let mut duplicate_count = 0u32;
        let check = n.min(200);
        for i in 0..check {
            for j in (i + 1)..check {
                let mut identical = true;
                for di in 0..dims {
                    if (data[i * d + di] - data[j * d + di]).abs() > 1e-12 {
                        identical = false;
                        break;
                    }
                }
                if identical {
                    duplicate_count += 1;
                }
            }
        }
        let duplicate_ratio = duplicate_count as f64 / (check * (check - 1) / 2).max(1) as f64;

        // 6. 건강 점수 + 수정 액션 코드
        let health_score = 1.0
            - contamination * 10.0  // NaN/Inf는 매우 나쁨
            - outlier_ratio * 2.0
            - collapsed_dims as f64 / dims.max(1) as f64 * 3.0
            - duplicate_ratio * 1.5;
        let health_score = health_score.max(0.0).min(1.0);

        // 수정 액션 결정 (가장 심각한 것 우선)
        let action_code = if contamination > 0.01 {
            if contamination > 0.5 { 5.0 } else { 4.0 } // CRITICAL or INTERPOLATE
        } else if collapsed_dims as f64 > dims as f64 * 0.5 {
            5.0 // CRITICAL — 절반 이상 붕괴
        } else if collapsed_dims > 0 {
            3.0 // REMOVE_DIMS
        } else if outlier_ratio > 0.1 {
            2.0 // REMOVE_OUTLIERS
        } else if needs_normalization {
            1.0 // NORMALIZE
        } else {
            0.0 // CLEAN
        };

        let mut result = HashMap::new();
        result.insert("health_score".into(), vec![health_score]);
        result.insert("action_code".into(), vec![action_code]);
        result.insert("nan_count".into(), vec![nan_count as f64]);
        result.insert("inf_count".into(), vec![inf_count as f64]);
        result.insert("contamination".into(), vec![contamination]);
        result.insert("collapsed_dims".into(), vec![collapsed_dims as f64]);
        result.insert("outlier_ratio".into(), vec![outlier_ratio as f64]);
        result.insert("scale_imbalance".into(), vec![scale_imbalance]);
        result.insert("duplicate_ratio".into(), vec![duplicate_ratio as f64]);
        result.insert("needs_normalization".into(), vec![if needs_normalization { 1.0 } else { 0.0 }]);
        result
    }
}
