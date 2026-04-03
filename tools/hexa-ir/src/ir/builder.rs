/// IRBuilder — ergonomic API for constructing HEXA-IR
use super::opcode::HexaOp;
use super::types::HexaType;
use super::instr::{HexaInstr, HexaBlock, HexaFunction};

pub struct IRBuilder {
    name: String,
    blocks: Vec<HexaBlock>,
    current_block: usize,
    reg_counter: usize,
    params: Vec<(String, HexaType)>,
    ret_ty: HexaType,
}

impl IRBuilder {
    pub fn new(name: &str, params: Vec<(String, HexaType)>, ret_ty: HexaType) -> Self {
        let entry = HexaBlock { id: 0, instrs: Vec::new(), successors: Vec::new() };
        IRBuilder {
            name: name.to_string(),
            blocks: vec![entry],
            current_block: 0,
            reg_counter: params.len(),
            params,
            ret_ty,
        }
    }

    /// Allocate a fresh SSA register
    fn fresh_reg(&mut self) -> usize {
        let r = self.reg_counter;
        self.reg_counter += 1;
        r
    }

    /// Emit an instruction with a result register
    pub fn emit(&mut self, op: HexaOp, args: Vec<usize>, ty: HexaType) -> usize {
        let dest = self.fresh_reg();
        self.blocks[self.current_block].instrs.push(HexaInstr {
            op, dest: Some(dest), args, ty, label: None,
        });
        dest
    }

    /// Emit a void instruction (no result)
    pub fn emit_void(&mut self, op: HexaOp, args: Vec<usize>) {
        self.blocks[self.current_block].instrs.push(HexaInstr {
            op, dest: None, args, ty: HexaType::Void, label: None,
        });
    }

    /// Create a new basic block, returns its ID
    pub fn new_block(&mut self) -> usize {
        let id = self.blocks.len();
        self.blocks.push(HexaBlock { id, instrs: Vec::new(), successors: Vec::new() });
        id
    }

    /// Switch to emitting into a different block
    pub fn switch_to(&mut self, block_id: usize) {
        self.current_block = block_id;
    }

    /// Add a successor edge
    pub fn add_successor(&mut self, from: usize, to: usize) {
        self.blocks[from].successors.push(to);
    }

    /// Finalize into a HexaFunction
    pub fn build(self) -> HexaFunction {
        HexaFunction {
            name: self.name,
            blocks: self.blocks,
            params: self.params,
            ret_ty: self.ret_ty,
        }
    }
}
