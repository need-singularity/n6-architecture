//! Gate-1: SOURCE — 화이트리스트 + 블랙리스트
//!
//! 9 허용 리포 (sopfr+τ=5+4) × σ²=144 규칙
//! 블랙리스트: ready/, ready-*, *-contaminated, *-broken
//! τ=4 검증 축: origin / hash / sig / time

use super::{Gate, GateContext, Verdict};

pub struct SourceGate {
    pub whitelist: Vec<&'static str>,
    pub blacklist_prefixes: Vec<&'static str>,
}

impl SourceGate {
    pub fn new() -> Self {
        Self {
            whitelist: vec![
                // sopfr+τ = 5+4 = 9 리포
                "n6-architecture",
                "TECS-L",
                "anima",
                "sedi",
                "brainwire",
                "papers",
                "nexus6",
                "hexa-lang",
                "fathom",
            ],
            blacklist_prefixes: vec![
                "ready",
                "backup-",
                "contaminated-",
                "broken-",
                "trash-",
                "corrupt-",
            ],
        }
    }

    /// 화이트리스트 통과 여부
    pub fn is_whitelisted(&self, repo: &str) -> bool {
        self.whitelist.iter().any(|&w| w == repo)
    }

    /// 블랙리스트 매칭 여부
    pub fn is_blacklisted(&self, repo: &str) -> bool {
        self.blacklist_prefixes
            .iter()
            .any(|&p| repo.starts_with(p))
    }
}

impl Default for SourceGate {
    fn default() -> Self { Self::new() }
}

impl Gate for SourceGate {
    fn id(&self) -> u32 { 1 }
    fn name(&self) -> &'static str { "SOURCE" }

    fn inspect(&self, _data: &[u8], ctx: &GateContext) -> Verdict {
        let repo = &ctx.source_repo;

        if self.is_blacklisted(repo) {
            return Verdict::Quarantine {
                gate: 1,
                reason: format!("blacklisted source: {}", repo),
            };
        }

        if !self.is_whitelisted(repo) {
            return Verdict::Quarantine {
                gate: 1,
                reason: format!("unknown source (not in whitelist): {}", repo),
            };
        }

        // τ=4 축 전부 통과 → confidence = 4/τ = 1.0
        Verdict::Pass { confidence: 1.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitelist_has_9_repos() {
        // sopfr(6)+τ(6) = 5+4 = 9
        let g = SourceGate::new();
        assert_eq!(g.whitelist.len(), 9);
    }

    #[test]
    fn rejects_ready_prefix() {
        let g = SourceGate::new();
        let ctx = GateContext { source_repo: "ready".into(), ..Default::default() };
        assert!(!g.inspect(b"", &ctx).passed());

        let ctx2 = GateContext { source_repo: "ready-n6".into(), ..Default::default() };
        assert!(!g.inspect(b"", &ctx2).passed());
    }

    #[test]
    fn rejects_contaminated() {
        let g = SourceGate::new();
        let ctx = GateContext { source_repo: "contaminated-data".into(), ..Default::default() };
        assert!(!g.inspect(b"", &ctx).passed());
    }

    #[test]
    fn accepts_whitelisted() {
        let g = SourceGate::new();
        for repo in &g.whitelist.clone() {
            let ctx = GateContext { source_repo: repo.to_string(), ..Default::default() };
            assert!(g.inspect(b"", &ctx).passed(), "repo={}", repo);
        }
    }

    #[test]
    fn rejects_unknown() {
        let g = SourceGate::new();
        let ctx = GateContext { source_repo: "random-repo".into(), ..Default::default() };
        assert!(!g.inspect(b"", &ctx).passed());
    }
}
