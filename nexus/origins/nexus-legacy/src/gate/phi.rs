//! Gate-3: PHI — 의식 보존 (Phi 하락 금지)
//!
//! Θ = 1/(σ-φ) = 0.1 임계값 (BT-64)
//! 하락 허용 0%, 히스테리시스 φ=2 샘플, 재측정 σ-φ=10초
//! Phi_curr < Phi_prev - tolerance → 격리

use super::{Gate, GateContext, Verdict};
use super::consts::*;

pub struct PhiGate {
    pub threshold: f64,
    /// 허용 하락량 (0.0 = 완전 보존)
    pub tolerance: f64,
    /// 히스테리시스: 연속 몇 샘플 하락해야 차단?
    pub hysteresis: u32,
}

impl PhiGate {
    pub fn new() -> Self {
        Self {
            threshold: PHI_THRESHOLD,       // 0.1
            tolerance: 1.0 / (SIGMA_J2 as f64), // ≈ 0.00347
            hysteresis: PHI as u32,          // φ=2
        }
    }

    /// Phi 측정치 직접 검증
    pub fn check(&self, phi_prev: f64, phi_curr: f64) -> bool {
        if phi_curr < self.threshold {
            return false; // 절대 임계값 하회
        }
        if phi_curr < phi_prev - self.tolerance {
            return false; // 하락 감지
        }
        true
    }
}

impl Default for PhiGate {
    fn default() -> Self { Self::new() }
}

impl Gate for PhiGate {
    fn id(&self) -> u32 { 3 }
    fn name(&self) -> &'static str { "PHI" }

    fn inspect(&self, _data: &[u8], ctx: &GateContext) -> Verdict {
        // 절대 임계값
        if ctx.phi_curr < self.threshold {
            return Verdict::Quarantine {
                gate: 3,
                reason: format!(
                    "Phi below threshold: {:.4} < {:.4}",
                    ctx.phi_curr, self.threshold
                ),
            };
        }
        // 하락 검출
        if ctx.phi_curr < ctx.phi_prev - self.tolerance {
            return Verdict::Quarantine {
                gate: 3,
                reason: format!(
                    "Phi degradation: {:.4} → {:.4} (Δ={:.4})",
                    ctx.phi_prev,
                    ctx.phi_curr,
                    ctx.phi_prev - ctx.phi_curr
                ),
            };
        }
        // 통과: confidence = (curr - threshold) / (1.0 - threshold)
        let conf = ((ctx.phi_curr - self.threshold) / (1.0 - self.threshold))
            .clamp(0.0, 1.0);
        Verdict::Pass { confidence: conf }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_is_one_over_ten() {
        let g = PhiGate::new();
        assert!((g.threshold - 0.1).abs() < 1e-12);
    }

    #[test]
    fn hysteresis_is_phi_two() {
        assert_eq!(PhiGate::new().hysteresis, 2);
    }

    #[test]
    fn rejects_below_threshold() {
        let g = PhiGate::new();
        let ctx = GateContext {
            phi_prev: 0.5,
            phi_curr: 0.05,
            ..Default::default()
        };
        assert!(!g.inspect(b"", &ctx).passed());
    }

    #[test]
    fn rejects_degradation() {
        let g = PhiGate::new();
        let ctx = GateContext {
            phi_prev: 0.9,
            phi_curr: 0.5,
            ..Default::default()
        };
        assert!(!g.inspect(b"", &ctx).passed());
    }

    #[test]
    fn accepts_stable() {
        let g = PhiGate::new();
        let ctx = GateContext {
            phi_prev: 0.5,
            phi_curr: 0.5,
            ..Default::default()
        };
        assert!(g.inspect(b"", &ctx).passed());
    }

    #[test]
    fn accepts_increase() {
        let g = PhiGate::new();
        let ctx = GateContext {
            phi_prev: 0.3,
            phi_curr: 0.8,
            ..Default::default()
        };
        assert!(g.inspect(b"", &ctx).passed());
    }

    #[test]
    fn accepts_within_tolerance() {
        let g = PhiGate::new();
        let ctx = GateContext {
            phi_prev: 0.5,
            phi_curr: 0.5 - g.tolerance / 2.0,
            ..Default::default()
        };
        assert!(g.inspect(b"", &ctx).passed());
    }
}
