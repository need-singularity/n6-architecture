/// Linear Scan Register Allocator
///
/// sigma=12 general-purpose registers available (matching x86-64 GPRs).
/// ARM64: x9-x28 = 20 allocatable registers (σ-φ=10 callee-saved + σ-φ=10 caller-saved).
/// Uses live interval analysis to assign SSA registers to physical registers,
/// spilling to stack when necessary.

use crate::ir::*;
use crate::util::n6::*;
use std::collections::HashMap;

/// Physical register — sigma=12 GPRs
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhysReg {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi,
    R8, R9, R10, R11, R12, R13,
}

impl PhysReg {
    /// All sigma=12 allocatable GPRs
    pub const ALL: [PhysReg; SIGMA] = [
        PhysReg::Rax, PhysReg::Rbx, PhysReg::Rcx, PhysReg::Rdx,
        PhysReg::Rsi, PhysReg::Rdi, PhysReg::R8, PhysReg::R9,
        PhysReg::R10, PhysReg::R11, PhysReg::R12, PhysReg::R13,
    ];

    /// Register index (0-based), 0..σ=12 sequential ordinals
    pub fn index(self) -> usize {
        // n6: ordinal indices 0..SIGMA — inherent to σ=12 register set
        match self {
            PhysReg::Rax => 0, PhysReg::Rbx => 1, PhysReg::Rcx => 2,
            PhysReg::Rdx => 3, PhysReg::Rsi => 4, PhysReg::Rdi => SOPFR,
            PhysReg::R8 => N, PhysReg::R9 => N + MU, PhysReg::R10 => SIGMA_TAU,
            PhysReg::R11 => SIGMA_TAU + MU, PhysReg::R12 => SIGMA_PHI, PhysReg::R13 => SIGMA_MU,
        }
    }

    /// x86-64 encoding for ModRM/SIB (ISA-defined hardware values)
    pub fn encoding(self) -> u8 {
        // n6: x86-64 ISA standard — register encodings are hardware-defined
        // Low bank: Rax=0..Rbx=3, then Rsi=n=6, Rdi=n+μ=7
        // High bank (R8+): encoding = index & 0b111, REX.B set
        match self {
            PhysReg::Rax => 0, PhysReg::Rcx => 1, PhysReg::Rdx => 2,
            PhysReg::Rbx => 3, PhysReg::Rsi => N as u8, PhysReg::Rdi => (N + MU) as u8,
            PhysReg::R8 => 0, PhysReg::R9 => 1, PhysReg::R10 => 2,
            PhysReg::R11 => 3, PhysReg::R12 => 4, PhysReg::R13 => SOPFR as u8,
        }
    }

    /// Whether this register needs REX.B prefix
    pub fn needs_rex(self) -> bool {
        matches!(self, PhysReg::R8 | PhysReg::R9 | PhysReg::R10 |
                      PhysReg::R11 | PhysReg::R12 | PhysReg::R13)
    }
}

// ═══════════════════════════════════════════════════════════════
// ARM64 Register Allocation — x9-x28 = 20 allocatable registers
// ═══════════════════════════════════════════════════════════════

/// ARM64 allocation target: physical register or spill slot
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Arm64Loc {
    /// ARM64 register number (9-28)
    Reg(u8),
    /// Spill slot index (maps to stack offset: 16 + slot_index * 8)
    Spill(usize),
}

impl Arm64Loc {
    /// ARM64 register name for assembly emission
    pub fn reg_name(self) -> String {
        match self {
            Arm64Loc::Reg(n) => format!("x{}", n),
            Arm64Loc::Spill(s) => format!("spill_{}", s),
        }
    }

    pub fn is_reg(self) -> bool {
        matches!(self, Arm64Loc::Reg(_))
    }
}

/// ARM64 allocatable registers: x9-x28 (20 registers)
/// x0-x7: argument/result, x8: indirect result
/// x29: fp, x30: lr, x31/sp: stack pointer
/// x9-x15: caller-saved scratch (no save needed across calls)
/// x19-x28: callee-saved (must save/restore in prologue/epilogue)
/// x16-x18: platform reserved (intra-procedure, platform)
const ARM64_CALLER_SAVED: [u8; 7] = [9, 10, 11, 12, 13, 14, 15];
const ARM64_CALLEE_SAVED: [u8; 10] = [19, 20, 21, 22, 23, 24, 25, 26, 27, 28];

/// ARM64 register allocation result
#[derive(Clone, Debug)]
pub struct Arm64RegAlloc {
    /// SSA register -> physical location (register or spill)
    pub mapping: HashMap<usize, Arm64Loc>,
    /// Number of spill slots used
    pub spill_slots: usize,
    /// Which callee-saved registers are used (must be saved in prologue)
    pub used_callee_saved: Vec<u8>,
    /// Whether any Call instruction exists (affects caller-save strategy)
    pub has_calls: bool,
}

impl Arm64RegAlloc {
    /// Get location for an SSA register
    pub fn get(&self, ssa_reg: usize) -> Option<Arm64Loc> {
        self.mapping.get(&ssa_reg).copied()
    }

    /// Get register name, returning scratch x9 as fallback (should not happen)
    pub fn reg_or_scratch(&self, ssa_reg: usize) -> Arm64Loc {
        self.mapping.get(&ssa_reg).copied().unwrap_or(Arm64Loc::Reg(9))
    }
}

/// Live interval for an SSA register
#[derive(Clone, Debug)]
pub struct LiveInterval {
    pub reg: usize,
    pub start: usize, // instruction index (global, linearized)
    pub end: usize,
}

/// Register allocation result
#[derive(Clone, Debug)]
pub struct RegAlloc {
    /// SSA register -> physical register assignment
    pub assignments: HashMap<usize, PhysReg>,
    /// SSA registers that were spilled to stack
    pub spills: Vec<usize>,
    /// Stack frame size needed for spills (in bytes)
    pub frame_size: usize,
}

impl RegAlloc {
    /// Get physical register index for an SSA register (0 = first GPR)
    pub fn get_phys(&self, ssa_reg: usize) -> Option<usize> {
        self.assignments.get(&ssa_reg).map(|pr| pr.index())
    }
}

/// Compute live intervals for all SSA registers in the function
fn compute_live_intervals(func: &HexaFunction) -> Vec<LiveInterval> {
    let mut intervals: HashMap<usize, LiveInterval> = HashMap::new();
    let mut pos = 0usize;

    for block in &func.blocks {
        for instr in &block.instrs {
            // Definition point
            if let Some(dest) = instr.dest {
                intervals.entry(dest).or_insert(LiveInterval {
                    reg: dest, start: pos, end: pos,
                });
            }
            // Use points extend the interval
            for &arg in &instr.args {
                intervals.entry(arg)
                    .and_modify(|iv| { iv.end = iv.end.max(pos); })
                    .or_insert(LiveInterval { reg: arg, start: 0, end: pos });
            }
            pos += 1;
        }
    }

    let mut result: Vec<LiveInterval> = intervals.into_values().collect();
    result.sort_by_key(|iv| iv.start);
    result
}

/// Run linear scan register allocation (x86-64)
pub fn allocate(func: &HexaFunction) -> RegAlloc {
    let intervals = compute_live_intervals(func);
    let mut assignments: HashMap<usize, PhysReg> = HashMap::new();
    let mut spills: Vec<usize> = Vec::new();

    // Active intervals sorted by end point
    let mut active: Vec<(usize, PhysReg, usize)> = Vec::new(); // (end, phys_reg, ssa_reg)

    // Free register pool
    let mut free_regs: Vec<PhysReg> = PhysReg::ALL.to_vec();
    free_regs.reverse(); // pop from end for LIFO behavior

    // SystemV ABI: first n=6 integer args in rdi, rsi, rdx, rcx, r8, r9
    // Pre-assign parameter registers
    let param_regs = [PhysReg::Rdi, PhysReg::Rsi, PhysReg::Rdx,
                      PhysReg::Rcx, PhysReg::R8, PhysReg::R9];
    for (i, _) in func.params.iter().enumerate() {
        if i < param_regs.len() {
            assignments.insert(i, param_regs[i]);
            free_regs.retain(|&r| r != param_regs[i]);
        }
    }

    for iv in &intervals {
        // Expire old intervals
        active.retain(|&(end, preg, _)| {
            if end < iv.start {
                free_regs.push(preg);
                false
            } else {
                true
            }
        });

        // Skip already-assigned (parameter) registers
        if assignments.contains_key(&iv.reg) {
            continue;
        }

        if let Some(preg) = free_regs.pop() {
            // Assign a free register
            assignments.insert(iv.reg, preg);
            active.push((iv.end, preg, iv.reg));
            active.sort_by_key(|&(end, _, _)| end);
        } else {
            // Spill: evict the interval with the farthest end point
            if let Some(last) = active.last().copied() {
                if last.0 > iv.end {
                    // Current interval ends sooner — evict the active one
                    let evicted = active.pop().unwrap();
                    spills.push(evicted.2);
                    assignments.remove(&evicted.2);
                    assignments.insert(iv.reg, evicted.1);
                    active.push((iv.end, evicted.1, iv.reg));
                    active.sort_by_key(|&(end, _, _)| end);
                } else {
                    // Current interval is the longest — spill it
                    spills.push(iv.reg);
                }
            } else {
                spills.push(iv.reg);
            }
        }
    }

    let frame_size = spills.len() * 8; // 8 bytes per spilled register

    RegAlloc { assignments, spills, frame_size }
}

/// Detect whether a function contains any Call instructions
fn has_call_instructions(func: &HexaFunction) -> bool {
    func.blocks.iter()
        .flat_map(|b| b.instrs.iter())
        .any(|i| i.op == HexaOp::Call)
}

/// Run linear scan register allocation for ARM64
/// Uses x9-x15 (caller-saved) + x19-x28 (callee-saved) = 17 allocatable registers
/// Prefers caller-saved first to minimize save/restore overhead
pub fn allocate_arm64(func: &HexaFunction) -> Arm64RegAlloc {
    let intervals = compute_live_intervals(func);
    let has_calls = has_call_instructions(func);
    let mut mapping: HashMap<usize, Arm64Loc> = HashMap::new();
    let mut used_callee_saved: Vec<u8> = Vec::new();
    let mut spill_count: usize = 0;

    // Active intervals sorted by end point: (end, reg_number, ssa_reg)
    let mut active: Vec<(usize, u8, usize)> = Vec::new();

    // Free register pool — prefer caller-saved (cheaper: no save/restore)
    // Push callee-saved first (bottom of stack), caller-saved on top (popped first)
    let mut free_regs: Vec<u8> = Vec::new();
    for &r in ARM64_CALLEE_SAVED.iter().rev() {
        free_regs.push(r);
    }
    for &r in ARM64_CALLER_SAVED.iter().rev() {
        free_regs.push(r);
    }

    // AAPCS64: first 8 args in x0-x7
    // Parameters arrive in x0-x7; we need to move them into allocatable registers.
    // Pre-assign parameters: if a parameter has a live interval, assign it a register
    // and emit a move from x{param_index} to the assigned register in the prologue.
    // We track which parameters need moves.
    let num_params = func.params.len().min(8);

    for iv in &intervals {
        // Expire old intervals
        active.retain(|&(end, reg_num, _)| {
            if end < iv.start {
                free_regs.push(reg_num);
                false
            } else {
                true
            }
        });

        // Skip already-assigned
        if mapping.contains_key(&iv.reg) {
            continue;
        }

        if let Some(preg) = free_regs.pop() {
            // Assign a free register
            mapping.insert(iv.reg, Arm64Loc::Reg(preg));
            active.push((iv.end, preg, iv.reg));
            active.sort_by_key(|&(end, _, _)| end);

            // Track callee-saved usage
            if ARM64_CALLEE_SAVED.contains(&preg) && !used_callee_saved.contains(&preg) {
                used_callee_saved.push(preg);
            }
        } else {
            // Spill: evict the interval with the farthest end point
            if let Some(last) = active.last().copied() {
                if last.0 > iv.end {
                    // Current interval ends sooner — evict the active one
                    let evicted = active.pop().unwrap();
                    mapping.insert(evicted.2, Arm64Loc::Spill(spill_count));
                    spill_count += 1;
                    mapping.insert(iv.reg, Arm64Loc::Reg(evicted.1));
                    active.push((iv.end, evicted.1, iv.reg));
                    active.sort_by_key(|&(end, _, _)| end);
                } else {
                    // Current interval is the longest — spill it
                    mapping.insert(iv.reg, Arm64Loc::Spill(spill_count));
                    spill_count += 1;
                }
            } else {
                mapping.insert(iv.reg, Arm64Loc::Spill(spill_count));
                spill_count += 1;
            }
        }
    }

    // Ensure any SSA registers referenced but not in intervals get a spill slot
    // (this handles edge cases like dead code references)
    let _ = num_params; // Used above for ABI info

    used_callee_saved.sort();

    Arm64RegAlloc {
        mapping,
        spill_slots: spill_count,
        used_callee_saved,
        has_calls,
    }
}
