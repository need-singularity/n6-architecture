/// Loop and Function Invariant Proof Tracking
///
/// Invariants are properties that must hold at specific program points:
/// - Loop invariants: hold at every iteration entry
/// - Function preconditions: hold at function entry
/// - Function postconditions: hold at every return point
///
/// The optimizer uses these to enable LICM (P6) and verify correctness (P12).

use std::collections::HashMap;

/// Kind of invariant
#[derive(Clone, Debug, PartialEq)]
pub enum InvariantKind {
    /// Loop invariant: register value doesn't change across iterations
    LoopInvariant {
        /// Block ID of the loop header
        header_block: usize,
        /// Register that is invariant
        reg: usize,
    },
    /// Function precondition: must hold at entry
    Precondition {
        /// Description of the condition
        description: String,
        /// Register holding the condition value
        condition_reg: usize,
    },
    /// Function postcondition: must hold at every return
    Postcondition {
        /// Description of the condition
        description: String,
        /// Register holding the condition value
        condition_reg: usize,
    },
    /// Range invariant: register value is bounded
    RangeBound {
        reg: usize,
        lower: Option<i64>,
        upper: Option<i64>,
    },
}

/// Status of an invariant check
#[derive(Clone, Debug, PartialEq)]
pub enum InvariantStatus {
    /// Assumed but not yet verified
    Assumed,
    /// Verified by static analysis
    Verified,
    /// Requires runtime check (proof couldn't be discharged statically)
    RequiresRuntimeCheck,
    /// Proven violated
    Violated { at_block: usize, reason: String },
}

/// A tracked invariant
#[derive(Clone, Debug)]
pub struct TrackedInvariant {
    pub kind: InvariantKind,
    pub status: InvariantStatus,
}

/// Invariant tracker for a function
pub struct InvariantTracker {
    invariants: Vec<TrackedInvariant>,
    /// Map from loop header block -> invariant indices
    loop_invariants: HashMap<usize, Vec<usize>>,
}

impl InvariantTracker {
    pub fn new() -> Self {
        InvariantTracker {
            invariants: Vec::new(),
            loop_invariants: HashMap::new(),
        }
    }

    /// Add a loop invariant
    pub fn add_loop_invariant(&mut self, header_block: usize, reg: usize) -> usize {
        let idx = self.invariants.len();
        self.invariants.push(TrackedInvariant {
            kind: InvariantKind::LoopInvariant { header_block, reg },
            status: InvariantStatus::Assumed,
        });
        self.loop_invariants.entry(header_block).or_default().push(idx);
        idx
    }

    /// Add a function precondition
    pub fn add_precondition(&mut self, description: &str, condition_reg: usize) -> usize {
        let idx = self.invariants.len();
        self.invariants.push(TrackedInvariant {
            kind: InvariantKind::Precondition {
                description: description.to_string(),
                condition_reg,
            },
            status: InvariantStatus::Assumed,
        });
        idx
    }

    /// Add a function postcondition
    pub fn add_postcondition(&mut self, description: &str, condition_reg: usize) -> usize {
        let idx = self.invariants.len();
        self.invariants.push(TrackedInvariant {
            kind: InvariantKind::Postcondition {
                description: description.to_string(),
                condition_reg,
            },
            status: InvariantStatus::Assumed,
        });
        idx
    }

    /// Add a range bound invariant
    pub fn add_range_bound(&mut self, reg: usize, lower: Option<i64>, upper: Option<i64>) -> usize {
        let idx = self.invariants.len();
        self.invariants.push(TrackedInvariant {
            kind: InvariantKind::RangeBound { reg, lower, upper },
            status: InvariantStatus::Assumed,
        });
        idx
    }

    /// Mark an invariant as verified
    pub fn verify(&mut self, idx: usize) {
        if let Some(inv) = self.invariants.get_mut(idx) {
            inv.status = InvariantStatus::Verified;
        }
    }

    /// Mark an invariant as requiring a runtime check
    pub fn mark_runtime_check(&mut self, idx: usize) {
        if let Some(inv) = self.invariants.get_mut(idx) {
            inv.status = InvariantStatus::RequiresRuntimeCheck;
        }
    }

    /// Mark an invariant as violated
    pub fn violate(&mut self, idx: usize, at_block: usize, reason: &str) {
        if let Some(inv) = self.invariants.get_mut(idx) {
            inv.status = InvariantStatus::Violated {
                at_block,
                reason: reason.to_string(),
            };
        }
    }

    /// Get all loop invariants for a given header block
    pub fn loop_invariants_for(&self, header_block: usize) -> Vec<&TrackedInvariant> {
        self.loop_invariants.get(&header_block)
            .map(|indices| {
                indices.iter()
                    .filter_map(|&i| self.invariants.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all violations
    pub fn violations(&self) -> Vec<&TrackedInvariant> {
        self.invariants.iter()
            .filter(|inv| matches!(inv.status, InvariantStatus::Violated { .. }))
            .collect()
    }

    /// Total count of tracked invariants
    pub fn total(&self) -> usize {
        self.invariants.len()
    }

    /// Count of verified invariants
    pub fn verified_count(&self) -> usize {
        self.invariants.iter()
            .filter(|inv| matches!(inv.status, InvariantStatus::Verified))
            .count()
    }
}

impl Default for InvariantTracker {
    fn default() -> Self {
        Self::new()
    }
}
