//! Gate-2: HASH — 무결성 검증 (σ·J₂=288 bit)
//!
//! 순수 Rust SHA-288 변형: 내부 상태 288bit (36 byte)
//! φ^τ=16 라운드, 2^(σ-τ)=256 B 블록
//!
//! NOTE: 암호학적 보안이 필요하면 BLAKE3 등으로 교체. 여기선 오염 검출(결정적
//! 일치/불일치)이 목적이므로 n=6 산술 해시로 충분.

use super::{Gate, GateContext, Verdict};
use super::consts::*;

/// 288비트 해시 = 36바이트
const HASH_BYTES: usize = (SIGMA_J2 / 8) as usize; // 36

/// n=6 산술 해시 — 36 byte 고정 출력.
///
/// 구조:
/// - 36개의 u64 레인 (> 288bit 상태)
/// - φ^τ=16 라운드 믹싱
/// - 2^(σ-τ)=256 byte 블록 처리
pub fn compute_hash(data: &[u8]) -> String {
    // 36 바이트 상태 초기화 (6f03630f 앵커 시드)
    let mut state = [0u8; HASH_BYTES];
    let seed: [u8; 4] = [0x6f, 0x03, 0x63, 0x0f];
    for i in 0..HASH_BYTES {
        state[i] = seed[i % 4] ^ (i as u8).wrapping_mul(N as u8);
    }

    // 블록 단위 처리 (256 B)
    let block_size = BLOCK as usize;
    let mut offset = 0;
    while offset < data.len() {
        let end = (offset + block_size).min(data.len());
        let block = &data[offset..end];
        absorb_block(&mut state, block);
        offset = end;
    }

    // 최종 마무리: φ^τ=16 라운드
    for r in 0..PHI_TAU {
        mix_round(&mut state, r);
    }

    // hex 인코딩
    state.iter().map(|b| format!("{:02x}", b)).collect()
}

fn absorb_block(state: &mut [u8; HASH_BYTES], block: &[u8]) {
    for (i, &b) in block.iter().enumerate() {
        let idx = i % HASH_BYTES;
        state[idx] = state[idx].wrapping_add(b);
        state[idx] = state[idx].rotate_left(((i as u32) % SIGMA) + 1);
    }
    // 한 번 믹스
    mix_round(state, 0);
}

fn mix_round(state: &mut [u8; HASH_BYTES], round: u32) {
    // τ=4 서브라운드
    for sub in 0..TAU {
        let shift = (round * TAU + sub) as usize;
        for i in 0..HASH_BYTES {
            let j = (i + shift + 1) % HASH_BYTES;
            let k = (i + shift + SOPFR as usize) % HASH_BYTES;
            state[i] = state[i]
                .wrapping_add(state[j].rotate_left(((i as u32) % 7) + 1))
                ^ state[k]
                .wrapping_mul(N as u8);
        }
    }
}

pub struct HashGate {
    pub anchor_required: bool,
}

impl HashGate {
    pub fn new() -> Self {
        Self { anchor_required: true }
    }

    /// 선언된 해시와 계산된 해시가 일치하는가?
    pub fn verify(&self, data: &[u8], declared: &str) -> bool {
        let actual = compute_hash(data);
        actual == declared
    }
}

impl Default for HashGate {
    fn default() -> Self { Self::new() }
}

impl Gate for HashGate {
    fn id(&self) -> u32 { 2 }
    fn name(&self) -> &'static str { "HASH" }

    fn inspect(&self, data: &[u8], ctx: &GateContext) -> Verdict {
        match &ctx.declared_hash {
            None => {
                if self.anchor_required {
                    Verdict::Quarantine {
                        gate: 2,
                        reason: "no declared hash anchor".into(),
                    }
                } else {
                    Verdict::Pass { confidence: 0.5 }
                }
            }
            Some(declared) => {
                if self.verify(data, declared) {
                    Verdict::Pass { confidence: 1.0 }
                } else {
                    Verdict::Quarantine {
                        gate: 2,
                        reason: "hash mismatch".into(),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_288_bits() {
        let h = compute_hash(b"test");
        // 288 bit = 36 byte = 72 hex chars
        assert_eq!(h.len(), 72);
    }

    #[test]
    fn hash_deterministic() {
        assert_eq!(compute_hash(b"nexus6"), compute_hash(b"nexus6"));
    }

    #[test]
    fn hash_sensitive() {
        assert_ne!(compute_hash(b"nexus6"), compute_hash(b"nexus7"));
    }

    #[test]
    fn hash_differs_by_single_bit() {
        let a = compute_hash(b"A");
        let b = compute_hash(b"B");
        assert_ne!(a, b);
    }

    #[test]
    fn gate_rejects_mismatch() {
        let g = HashGate::new();
        let ctx = GateContext {
            declared_hash: Some("deadbeef".to_string()),
            ..Default::default()
        };
        assert!(!g.inspect(b"real-data", &ctx).passed());
    }

    #[test]
    fn gate_accepts_match() {
        let g = HashGate::new();
        let data = b"nexus6-clean";
        let h = compute_hash(data);
        let ctx = GateContext {
            declared_hash: Some(h),
            ..Default::default()
        };
        assert!(g.inspect(data, &ctx).passed());
    }

    #[test]
    fn gate_rejects_missing_anchor() {
        let g = HashGate::new();
        let ctx = GateContext::default();
        assert!(!g.inspect(b"x", &ctx).passed());
    }
}
