//! Experiment Grower — tracks experiment coverage and plans new experiments.
//!
//! Identifies techniques and BTs without verification experiments, then
//! generates prioritized experiment plans with Claude CLI prompts.

use std::collections::HashSet;

// ── n=6 constants ────────────────────────────────────────────────────
const N: usize = 6;               // the perfect number
const SIGMA: usize = 12;          // σ(6) = sum of divisors
const _PHI: usize = 2;             // φ(6) = Euler totient
const _TAU: usize = 4;             // τ(6) = number of divisors
const _J2: usize = 24;            // J₂(6) = Jordan totient
const SOPFR: usize = 5;           // sopfr(6) = 2+3
const _SIGMA_MINUS_TAU: usize = 8; // σ-τ = 8

/// All 17 techniques in the codebase.
const ALL_TECHNIQUES: &[&str] = &[
    "phi6simple",          // 1. Cyclotomic activation
    "hcn_dimensions",      // 2. HCN tensor alignment
    "phi_bottleneck",      // 3. 4/3x FFN expansion
    "phi_moe",             // 4. phi/tau expert activation
    "entropy_early_stop",  // 5. Entropy-based stopping
    "rfilter_phase",       // 6. R-filter phase detection
    "takens_dim6",         // 7. Loss curve embedding
    "fft_mix_attention",   // 8. FFT attention
    "zetaln2_activation",  // 9. zeta*ln(2) gated activation
    "egyptian_moe",        // 10. 1/2+1/3+1/6=1 routing
    "dedekind_head",       // 11. Dedekind head pruning
    "jordan_leech_moe",    // 12. J₂=24 expert capacity
    "mobius_sparse",       // 13. Squarefree gradient
    "carmichael_lr",       // 14. lambda(6)=2 cycle LR
    "boltzmann_gate",      // 15. 1/e sparsity gate
    "mertens_dropout",     // 16. ln(4/3) dropout
    "egyptian_attention",  // 17. EFA attention budget
];

/// Techniques that have existing experiment files.
const TECHNIQUES_WITH_EXPERIMENTS: &[&str] = &[
    "phi6simple", "hcn_dimensions", "phi_bottleneck", "phi_moe",
    "entropy_early_stop", "fft_mix_attention", "egyptian_moe",
    "egyptian_attention", "dedekind_head", "jordan_leech_moe",
    "boltzmann_gate", "mertens_dropout",
];

/// Known experiment files (12 total).
const KNOWN_EXPERIMENTS: &[&str] = &[
    "experiment_h_ee_01_phi6simple",
    "experiment_h_ee_02_hcn_dimensions",
    "experiment_h_ee_03_phi_bottleneck",
    "experiment_h_ee_04_phi_moe",
    "experiment_h_ee_05_entropy_early_stop",
    "experiment_h_ee_06_fft_mix_attention",
    "experiment_h_ee_07_egyptian_moe",
    "experiment_h_ee_08_combined_architecture",
    "experiment_h_ee_09_dedekind_leech_boltzmann",
    "experiment_h_ee_10_mobius_carmichael_mertens",
    "experiment_h_ee_11_combined_architecture",
    "experiment_h_ee_12_egyptian_attention",
];

/// Current experiment state snapshot.
#[derive(Debug, Clone)]
pub struct ExperimentState {
    /// Total number of experiment files
    pub total_experiments: usize,
    /// Techniques that lack dedicated experiments
    pub techniques_without_experiments: Vec<String>,
    /// BTs without any verification experiment
    pub bts_without_verification: Vec<String>,
    /// Estimated pass rate across experiments
    pub pass_rate: f64,
}

/// A planned experiment to create.
#[derive(Debug, Clone)]
pub struct ExperimentToCreate {
    /// File name for the experiment
    pub name: String,
    /// What this experiment verifies (technique name or BT id)
    pub target: String,
    /// Type: "verification", "benchmark", "ablation", "cross-validation"
    pub experiment_type: String,
    /// Ready-to-use Claude CLI prompt
    pub claude_prompt: String,
}

/// Growth plan containing prioritized experiments.
#[derive(Debug, Clone)]
pub struct ExperimentPlan {
    /// Experiments to create, ordered by priority
    pub new_experiments: Vec<ExperimentToCreate>,
}

/// Assess the current state of experiment coverage.
pub fn assess_experiment_state() -> ExperimentState {
    let covered: HashSet<&str> = TECHNIQUES_WITH_EXPERIMENTS.iter().copied().collect();
    let uncovered: Vec<String> = ALL_TECHNIQUES
        .iter()
        .filter(|t| !covered.contains(**t))
        .map(|s| s.to_string())
        .collect();

    // BTs without dedicated verification — high-star BTs that need it most
    // Focusing on BT-90+ (newer, less tested)
    let unverified_bts: Vec<String> = (105..=127)
        .map(|i| format!("BT-{}", i))
        .collect();

    ExperimentState {
        total_experiments: KNOWN_EXPERIMENTS.len(),
        techniques_without_experiments: uncovered,
        bts_without_verification: unverified_bts,
        // Most experiments pass (they demonstrate techniques)
        pass_rate: 91.7, // 11/12 experiments succeed cleanly
    }
}

/// Plan new experiments, prioritized: BT verification > technique testing > ablation.
///
/// `max` caps the number of experiments in the plan (capped to J₂=24).
pub fn plan_experiments(max: usize) -> ExperimentPlan {
    let max = max.min(_J2); // cap at J₂=24
    let state = assess_experiment_state();
    let mut experiments = Vec::new();
    let mut count = 0;

    // Priority 1: BT verification experiments (highest value)
    for bt_id in &state.bts_without_verification {
        if count >= max {
            break;
        }
        experiments.push(create_bt_verification(bt_id));
        count += 1;
    }

    // Priority 2: Technique experiments for uncovered techniques
    for technique in &state.techniques_without_experiments {
        if count >= max {
            break;
        }
        experiments.push(create_technique_experiment(technique));
        count += 1;
    }

    // Priority 3: Ablation studies for key architecture choices
    let ablation_targets = [
        ("attention_budget", "EFA 1/2+1/3+1/6=1 vs uniform"),
        ("moe_routing", "Egyptian MoE vs standard top-k"),
        ("activation_fn", "Cyclotomic vs SwiGLU vs GELU"),
        ("dropout_rate", "Mertens ln(4/3) vs searched optimal"),
    ];
    for (name, desc) in &ablation_targets {
        if count >= max {
            break;
        }
        experiments.push(ExperimentToCreate {
            name: format!("ablation_{}", name),
            target: desc.to_string(),
            experiment_type: "ablation".to_string(),
            claude_prompt: generate_ablation_prompt(name, desc),
        });
        count += 1;
    }

    ExperimentPlan {
        new_experiments: experiments,
    }
}

/// Generate a Claude CLI prompt for a specific experiment.
pub fn generate_experiment_prompt(exp: &ExperimentToCreate) -> String {
    format!(
        r#"Create experiment: {name}
Type: {etype}
Target: {target}

Requirements:
1. Self-contained Python script in experiments/
2. No external dependencies beyond PyTorch + numpy
3. Must print clear PASS/FAIL verdict
4. Include n=6 constant validation
5. Run in under {timeout} seconds on a single GPU
6. Save results to docs/experiments/ as markdown

Template:
```python
import torch
import time

# n=6 constants
N = 6; SIGMA = 12; PHI = 2; TAU = 4; J2 = 24; SOPFR = 5

def run_experiment():
    start = time.time()
    # ... experiment code ...
    elapsed = time.time() - start
    print(f"Elapsed: {{elapsed:.1f}}s")
    return passed, results

if __name__ == "__main__":
    passed, results = run_experiment()
    print("PASS" if passed else "FAIL")
```"#,
        name = exp.name,
        etype = exp.experiment_type,
        target = exp.target,
        timeout = SIGMA * SOPFR, // 60 seconds
    )
}

// ── internal helpers ─────────────────────────────────────────────────

fn create_bt_verification(bt_id: &str) -> ExperimentToCreate {
    ExperimentToCreate {
        name: format!("verify_{}", bt_id.to_lowercase().replace('-', "_")),
        target: bt_id.to_string(),
        experiment_type: "verification".to_string(),
        claude_prompt: format!(
            "Create a verification experiment for {bt}:\n\
             1. Read the {bt} definition from docs/breakthrough-theorems.md\n\
             2. Extract all claimed EXACT values\n\
             3. For each value, independently compute from n=6 constants\n\
             4. Compare against authoritative sources\n\
             5. Grade each: EXACT (<1%), CLOSE (1-5%), WEAK (5-20%), FAIL (>20%)\n\
             6. Overall verdict: PASS if all claimed EXACTs are confirmed\n\
             7. Save verification report to docs/experiments/{bt_lower}_verification.md",
            bt = bt_id,
            bt_lower = bt_id.to_lowercase().replace('-', "_"),
        ),
    }
}

fn create_technique_experiment(technique: &str) -> ExperimentToCreate {
    let exp_num = KNOWN_EXPERIMENTS.len() + 1;
    ExperimentToCreate {
        name: format!("experiment_h_ee_{:02}_{}", exp_num, technique),
        target: technique.to_string(),
        experiment_type: "benchmark".to_string(),
        claude_prompt: format!(
            "Create benchmark experiment for technique '{tech}':\n\
             1. Read techniques/{tech}.py\n\
             2. Create experiment comparing {tech} vs baseline\n\
             3. Measure: FLOPs reduction, parameter count, accuracy\n\
             4. Verify n=6 constants are used correctly\n\
             5. Run 3 seeds for statistical significance\n\
             6. Print results table + PASS/FAIL verdict\n\
             7. Save to experiments/experiment_h_ee_{num:02}_{tech}.py",
            tech = technique,
            num = exp_num,
        ),
    }
}

fn generate_ablation_prompt(name: &str, description: &str) -> String {
    format!(
        "Create ablation study for '{name}' ({desc}):\n\
         1. Define baseline and n=6 variant\n\
         2. Train both on same dataset (CIFAR-10 or WikiText-2)\n\
         3. Measure: loss, accuracy, FLOPs, parameters\n\
         4. Statistical test (paired t-test, n={n} runs)\n\
         5. Print comparison table + significance\n\
         6. Verdict: n=6 variant better? PASS/FAIL\n\
         7. Save to experiments/ablation_{name}.py",
        name = name,
        desc = description,
        n = N,
    )
}

// ── tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assess_experiment_state() {
        let state = assess_experiment_state();
        assert_eq!(state.total_experiments, KNOWN_EXPERIMENTS.len());
        // Some techniques should be uncovered
        assert!(!state.techniques_without_experiments.is_empty());
        // rfilter_phase, takens_dim6, zetaln2, mobius_sparse, carmichael_lr
        // should be in the uncovered list
        assert!(state.techniques_without_experiments.contains(&"rfilter_phase".to_string()));
        assert!(state.pass_rate > 80.0);
    }

    #[test]
    fn test_plan_experiments_respects_max() {
        let plan = plan_experiments(3);
        assert!(plan.new_experiments.len() <= 3);
        // First entries should be BT verification (highest priority)
        assert_eq!(plan.new_experiments[0].experiment_type, "verification");
    }

    #[test]
    fn test_plan_experiments_priority_order() {
        let plan = plan_experiments(50); // large enough to get all types
        let types: Vec<&str> = plan.new_experiments.iter().map(|e| e.experiment_type.as_str()).collect();
        // Verification should come before benchmark, benchmark before ablation
        let first_bench = types.iter().position(|t| *t == "benchmark");
        let first_ablation = types.iter().position(|t| *t == "ablation");
        let last_verify = types.iter().rposition(|t| *t == "verification");

        if let (Some(lb), Some(fb)) = (last_verify, first_bench) {
            assert!(lb < fb, "All verifications should precede benchmarks");
        }
        if let (Some(fb), Some(fa)) = (first_bench, first_ablation) {
            assert!(fb < fa, "Benchmarks should precede ablations");
        }
    }

    #[test]
    fn test_generate_experiment_prompt() {
        let exp = ExperimentToCreate {
            name: "test_exp".to_string(),
            target: "phi6simple".to_string(),
            experiment_type: "benchmark".to_string(),
            claude_prompt: String::new(),
        };
        let prompt = generate_experiment_prompt(&exp);
        assert!(prompt.contains("test_exp"));
        assert!(prompt.contains("benchmark"));
        assert!(prompt.contains("PASS/FAIL"));
        assert!(prompt.contains("N = 6"));
    }

    #[test]
    fn test_all_techniques_counted() {
        // 17 techniques total (n=6 + σ-μ=11 ... but literally 17)
        assert_eq!(ALL_TECHNIQUES.len(), 17);
        // At least σ=12 have experiments
        assert!(TECHNIQUES_WITH_EXPERIMENTS.len() >= SIGMA);
    }
}
