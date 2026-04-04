/// HEXA-IR Opcodes — J₂=24 instructions in τ=4 categories of n=6 each
use crate::util::n6::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HexaOp {
    // Arithmetic (n=6)
    Add, Sub, Mul, Div, Mod, Neg,
    // Memory (n=6)
    Load, Store, Alloc, Free, Copy, Move,
    // Control (n=6)
    Jump, Branch, Call, Return, Phi, Switch,
    // Proof (n=6) — HEXA-LANG native, absent in Rust/LLVM
    ProofAssert, ProofInvariant, ProofWitness,
    OwnershipTransfer, BorrowCheck, LifetimeEnd,
}

impl HexaOp {
    /// All J₂=24 opcodes in canonical order
    pub const ALL: [HexaOp; J2] = [
        HexaOp::Add, HexaOp::Sub, HexaOp::Mul, HexaOp::Div, HexaOp::Mod, HexaOp::Neg,
        HexaOp::Load, HexaOp::Store, HexaOp::Alloc, HexaOp::Free, HexaOp::Copy, HexaOp::Move,
        HexaOp::Jump, HexaOp::Branch, HexaOp::Call, HexaOp::Return, HexaOp::Phi, HexaOp::Switch,
        HexaOp::ProofAssert, HexaOp::ProofInvariant, HexaOp::ProofWitness,
        HexaOp::OwnershipTransfer, HexaOp::BorrowCheck, HexaOp::LifetimeEnd,
    ];

    /// Category index: 0=Arithmetic, 1=Memory, 2=Control, 3=Proof (τ=4)
    pub fn category(&self) -> usize {
        match self {
            HexaOp::Add | HexaOp::Sub | HexaOp::Mul |
            HexaOp::Div | HexaOp::Mod | HexaOp::Neg => 0,
            HexaOp::Load | HexaOp::Store | HexaOp::Alloc |
            HexaOp::Free | HexaOp::Copy | HexaOp::Move => 1,
            HexaOp::Jump | HexaOp::Branch | HexaOp::Call |
            HexaOp::Return | HexaOp::Phi | HexaOp::Switch => 2,
            _ => 3,
        }
    }

    /// σ=12 side-effect ops: Memory(φ=2) + Control(τ=4) + Proof(n=6)
    pub fn has_side_effect(&self) -> bool {
        matches!(self,
            HexaOp::Store | HexaOp::Free |
            HexaOp::Call | HexaOp::Return | HexaOp::Jump | HexaOp::Branch |
            HexaOp::ProofAssert | HexaOp::ProofInvariant | HexaOp::ProofWitness |
            HexaOp::OwnershipTransfer | HexaOp::BorrowCheck | HexaOp::Switch
        )
    }

    pub fn is_proof(&self) -> bool {
        self.category() == 3
    }

    pub fn is_terminator(&self) -> bool {
        matches!(self, HexaOp::Jump | HexaOp::Branch | HexaOp::Return | HexaOp::Switch)
    }
}

pub const CATEGORY_NAMES: [&str; TAU] = ["Arithmetic", "Memory", "Control", "Proof"];
