/// Pattern match lowering — Mk.II stub
///
/// In HEXA-LANG, match expressions destructure enums and structs.
/// For Mk.I, we define the data structures but defer full lowering.

use crate::ir::*;

/// A single arm of a match expression
#[derive(Clone, Debug)]
pub struct MatchArm {
    /// Pattern to test against (tag index for enums, field list for structs)
    pub tag: usize,
    /// Bindings extracted from the pattern: (name, register)
    pub bindings: Vec<(String, usize)>,
    /// The block to jump to when this arm matches
    pub target_block: usize,
}

/// Result of lowering a match expression
#[derive(Clone, Debug)]
pub struct MatchResult {
    /// The register holding the scrutinee value
    pub scrutinee: usize,
    /// Arms in order of appearance
    pub arms: Vec<MatchArm>,
    /// Block that receives the merged result (via Phi)
    pub merge_block: usize,
    /// Register holding the final Phi result
    pub result_reg: usize,
}

/// Lower a match expression (Mk.II stub — returns Ok with empty result)
///
/// Full implementation will:
/// 1. Evaluate the scrutinee
/// 2. Emit Switch on the tag
/// 3. For each arm, emit bindings + body
/// 4. Merge results with Phi in the join block
pub fn lower_match(
    _scrutinee_reg: usize,
    _arms: &[MatchArm],
    _block: &mut HexaBlock,
) -> Result<MatchResult, String> {
    // Mk.II: full pattern compilation with decision trees
    Ok(MatchResult {
        scrutinee: 0,
        arms: Vec::new(),
        merge_block: 0,
        result_reg: 0,
    })
}
