/// HEXA-IR Instructions, Blocks, Functions — SSA form
use super::opcode::HexaOp;
use super::types::HexaType;

/// Single SSA instruction
#[derive(Clone, Debug)]
pub struct HexaInstr {
    pub op: HexaOp,
    pub dest: Option<usize>,   // SSA register (None for Store/Jump/etc.)
    pub args: Vec<usize>,      // operand registers
    pub ty: HexaType,
    /// Optional label for Call targets (function name) or other metadata
    #[allow(dead_code)]
    pub label: Option<String>,
}

impl HexaInstr {
    /// Create a new instruction (label defaults to None)
    pub fn new(op: HexaOp, dest: Option<usize>, args: Vec<usize>, ty: HexaType) -> Self {
        HexaInstr { op, dest, args, ty, label: None }
    }

    /// Create a new instruction with a label (for Call targets)
    pub fn with_label(op: HexaOp, dest: Option<usize>, args: Vec<usize>, ty: HexaType, label: String) -> Self {
        HexaInstr { op, dest, args, ty, label: Some(label) }
    }
}

/// Basic block — linear sequence ending with terminator
#[derive(Clone, Debug)]
pub struct HexaBlock {
    pub id: usize,
    pub instrs: Vec<HexaInstr>,
    pub successors: Vec<usize>,
}

/// Function — entry is blocks[0]
#[derive(Clone, Debug)]
pub struct HexaFunction {
    pub name: String,
    pub blocks: Vec<HexaBlock>,
    pub params: Vec<(String, HexaType)>,
    pub ret_ty: HexaType,
    /// String constant pool — string literals referenced by index
    pub string_pool: Vec<String>,
}

impl HexaFunction {
    pub fn count_instrs(&self) -> usize {
        self.blocks.iter().map(|b| b.instrs.len()).sum()
    }

    pub fn entry_block(&self) -> Option<&HexaBlock> {
        self.blocks.first()
    }
}
