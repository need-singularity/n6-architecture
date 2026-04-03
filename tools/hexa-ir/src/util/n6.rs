/// n=6 Core Constants — Single Source of Truth
///
/// σ(n)·φ(n) = n·τ(n) ⟺ n = 6 (unique for all n ≥ 2)
/// Every compiler constant derives from this single identity.

// Primary constants
pub const N: usize = 6;                        // perfect number
pub const PHI: usize = 2;                      // φ(6) = Euler totient
pub const TAU: usize = 4;                      // τ(6) = divisor count
pub const SIGMA: usize = 12;                   // σ(6) = divisor sum
pub const SOPFR: usize = 5;                    // sopfr(6) = 2+3
pub const J2: usize = 24;                      // J₂(6) = Jordan totient
pub const MU: usize = 1;                       // μ(6) = Möbius (squarefree)

// Derived constants
pub const SIGMA_TAU: usize = SIGMA - TAU;      // σ-τ = 8  (primitive types, runtime %)
pub const SIGMA_PHI: usize = SIGMA - PHI;      // σ-φ = 10 (improvement multiplier)
pub const N_PHI: usize = N / PHI;              // n/φ = 3  (pass groups, Pratt levels)
pub const PHI_TAU: usize = 1 << TAU;           // 2^τ = 16 (x86-64 total registers)
pub const SIGMA_SQ: usize = SIGMA * SIGMA;     // σ² = 144 (benchmark scale)
pub const PHI_N: usize = 1 << N;               // 2^n = 64 (store addr space)
pub const PHI_SOPFR: usize = 1 << SOPFR;       // 2^sopfr = 32 (load addr space)
pub const SIGMA_MU: usize = SIGMA - MU;        // σ-μ = 11 (M-theory dim)
pub const SIGMA_J2: usize = SIGMA * J2;

pub const NEXUS_LENSES: usize = 213;  // NEXUS-6 lens count (auto-synced)        // σ·J₂ = 288 (extended opcode space)

// Block sizes for Egyptian allocator
pub const BLOCK_LARGE: usize = 1 << SIGMA;     // 2^σ  = 4096
pub const BLOCK_MEDIUM: usize = 1 << SIGMA_PHI;// 2^(σ-φ) = 1024
pub const BLOCK_SMALL: usize = 1 << SIGMA_TAU; // 2^(σ-τ) = 256
pub const BLOCK_MIN: usize = 1 << N;           // 2^n = 64

/// LCG PRNG — shared across all modules (no more copy-paste)
pub struct Rng(pub u64);

impl Rng {
    pub fn new(seed: u64) -> Self { Rng(seed) }

    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }

    pub fn next_usize(&mut self) -> usize {
        self.next() as usize
    }
}
