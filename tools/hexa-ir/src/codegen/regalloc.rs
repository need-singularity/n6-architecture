/// Linear Scan Register Allocator
///
/// sigma=12 general-purpose registers available (matching x86-64 GPRs).
/// Uses live interval analysis to assign SSA registers to physical registers,
/// spilling to stack when necessary.

use crate::ir::*;
use crate::util::n6::SIGMA;
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

    /// Register index (0-based)
    pub fn index(self) -> usize {
        match self {
            PhysReg::Rax => 0, PhysReg::Rbx => 1, PhysReg::Rcx => 2,
            PhysReg::Rdx => 3, PhysReg::Rsi => 4, PhysReg::Rdi => 5,
            PhysReg::R8 => 6, PhysReg::R9 => 7, PhysReg::R10 => 8,
            PhysReg::R11 => 9, PhysReg::R12 => 10, PhysReg::R13 => 11,
        }
    }

    /// x86-64 encoding for ModRM/SIB
    pub fn encoding(self) -> u8 {
        match self {
            PhysReg::Rax => 0, PhysReg::Rcx => 1, PhysReg::Rdx => 2,
            PhysReg::Rbx => 3, PhysReg::Rsi => 6, PhysReg::Rdi => 7,
            PhysReg::R8 => 0, PhysReg::R9 => 1, PhysReg::R10 => 2,
            PhysReg::R11 => 3, PhysReg::R12 => 4, PhysReg::R13 => 5,
        }
    }

    /// Whether this register needs REX.B prefix
    pub fn needs_rex(self) -> bool {
        matches!(self, PhysReg::R8 | PhysReg::R9 | PhysReg::R10 |
                      PhysReg::R11 | PhysReg::R12 | PhysReg::R13)
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

/// Run linear scan register allocation
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
