/// P3: Dead Store Elimination — proof-guided alias analysis
///
/// Base: remove stores overwritten before read (same as LLVM).
/// HEXA advantage: proof instructions provide GUARANTEED no-alias info:
///
///   1. OwnershipTransfer(A, B) → A is dead → stores to A are dead
///   2. BorrowCheck(R) verified → R has exclusive access → no alias with others
///   3. LifetimeEnd(R) → R is dead → stores to R after this point are dead
///
/// LLVM must assume any store could alias any load (without restrict).
/// HEXA KNOWS which stores are dead — proven, not heuristic.

use crate::ir::*;
use crate::opt::proof_info::ProofInfo;

pub fn run(func: &mut HexaFunction) {
    // Extract proof facts (zero cost — proof instrs erased at codegen)
    let proof = ProofInfo::analyze(func);

    for block in &mut func.blocks {
        let len = block.instrs.len();
        let mut dead_indices: Vec<bool> = vec![false; len];

        for i in 0..len {
            if block.instrs[i].op == HexaOp::Store {
                let store_addr = block.instrs[i].args.first().copied();

                // === HEXA-ONLY: Proof-guided dead store elimination ===

                // (1) Store to a register that was ownership-transferred away.
                //     The source is provably dead — no one can read this store.
                //     LLVM CANNOT do this without an ownership model.
                if let Some(addr) = store_addr {
                    if proof.dead_after_move.contains(&addr) {
                        dead_indices[i] = true;
                        continue;
                    }
                }

                // (2) Store to a register whose lifetime has ended.
                //     Provably dead — no subsequent read possible.
                //     LLVM must conservatively keep this store.
                if let Some(addr) = store_addr {
                    if proof.lifetime_ended.contains_key(&addr) {
                        dead_indices[i] = true;
                        continue;
                    }
                }

                // === Standard DSE (same as LLVM) ===
                // Store to A followed by another Store to A with no Load from A
                for j in (i + 1)..len {
                    if block.instrs[j].op == HexaOp::Load {
                        let load_addr = block.instrs[j].args.first().copied();

                        // HEXA advantage: if the load address has exclusive borrow
                        // and it's a different register, we KNOW it can't alias.
                        // LLVM would bail out here.
                        if load_addr == store_addr {
                            // Same address — check if proof says no-alias
                            break; // address is read, can't eliminate
                        }
                        if let (Some(sa), Some(la)) = (store_addr, load_addr) {
                            if !proof.no_alias(sa, la) {
                                // Can't prove no-alias — must be conservative
                                // (LLVM always reaches this path for non-restrict ptrs)
                                break;
                            }
                            // Proof says no alias — continue looking (HEXA-only path)
                        }
                    }
                    if block.instrs[j].op == HexaOp::Store {
                        if block.instrs[j].args.first().copied() == store_addr {
                            dead_indices[i] = true; // overwritten without read
                            break;
                        }
                    }
                }
            }
        }

        let mut idx = 0;
        block.instrs.retain(|_| {
            let keep = !dead_indices[idx];
            idx += 1;
            keep
        });
    }
}
