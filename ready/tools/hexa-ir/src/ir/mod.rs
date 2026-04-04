/// HEXA-IR Core — J₂=24 opcodes, σ=12 types, SSA instructions
///
/// This is the GRAVITY CENTER — every module depends on ir/.
pub mod opcode;
pub mod types;
pub mod instr;
pub mod builder;
pub mod print;

pub use opcode::HexaOp;
pub use types::HexaType;
pub use instr::{HexaInstr, HexaBlock, HexaFunction};
pub use builder::IRBuilder;

/// Pass result for benchmark tracking
#[derive(Clone, Debug)]
pub struct PassResult {
    pub name: &'static str,
    pub group: &'static str,
    pub instrs_before: usize,
    pub instrs_after: usize,
    pub time_us: u64,
}
