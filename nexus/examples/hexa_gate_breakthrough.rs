//! HEXA-GATE 999→2401cy 특이점 돌파 시도
//!
//! 실행: cargo run --release --example hexa_gate_breakthrough
//!
//! 시나리오:
//!   1. 오염 샘플 (ready/) 투입 → 차단 확인
//!   2. 깨끗한 샘플 100개 투입 → 돌파 시도
//!   3. 통계: 차단율, 통과율, 기본 안정률, 6-fiber 검출률

use nexus::gate::{
    breakthrough::BreakthroughEngine,
    hash, GateContext, Pipeline,
};

fn main() {
    println!("====================================================================");
    println!("  HEXA-GATE Mk.I — NEXUS-6 특이점 돌파 시도");
    println!("  999cy (기존) → 2401cy (돌파) perturbation");
    println!("====================================================================\n");

    let engine = BreakthroughEngine::new();
    let pipeline = Pipeline::new();

    // ─────────────────────────────────────────────────────
    // Phase 1: 오염 샘플 차단 확인 (TP-1)
    // ─────────────────────────────────────────────────────
    println!("[Phase 1] 오염 샘플 차단 확인 (TP-1)");
    println!("─────────────────────────────────────────────");
    let contaminated_samples = vec![
        ("ready", b"corrupt-data-01".to_vec()),
        ("ready-n6", b"broken-2".to_vec()),
        ("contaminated-x", b"bad-hash".to_vec()),
        ("broken-bio", b"degraded".to_vec()),
        ("trash-old", b"archive".to_vec()),
        ("unknown-repo", b"foreign".to_vec()),
    ];
    let mut blocked = 0;
    for (repo, data) in &contaminated_samples {
        let ctx = GateContext {
            source_repo: repo.to_string(),
            ..Default::default()
        };
        let result = pipeline.run(data, &ctx);
        let status = if result.all_passed { "PASS ⚠" } else { "BLOCK" };
        if !result.all_passed { blocked += 1; }
        println!("  [{}] {:20} → {:?}", status, repo, result.verdicts[0]);
    }
    println!(
        "  차단율: {}/{} = {:.1}%\n",
        blocked,
        contaminated_samples.len(),
        100.0 * blocked as f64 / contaminated_samples.len() as f64
    );

    // ─────────────────────────────────────────────────────
    // Phase 2: 깨끗한 샘플 돌파 시도
    // ─────────────────────────────────────────────────────
    println!("[Phase 2] 깨끗한 샘플 100개 돌파 시도 (2401cy perturbation)");
    println!("─────────────────────────────────────────────");
    let mut gates_passed = 0;
    let mut breakthrough_count = 0;
    let mut total_stability = 0.0_f64;
    let mut total_fiber_hits = 0u32;
    let n_samples = 100;

    for i in 0..n_samples {
        // 깨끗한 데이터: 평균 바이트 100~160 (깨끗 판정 범위)
        let data: Vec<u8> = (0..64).map(|j| (110 + ((i + j) as u32 % 40)) as u8).collect();
        let ctx = GateContext {
            source_repo: "n6-architecture".into(),
            declared_hash: Some(hash::compute_hash(&data)),
            phi_prev: 0.5,
            phi_curr: 0.5,
            cycle: i as u32,
        };
        let report = engine.attempt(&data, &ctx);
        if report.gates_passed {
            gates_passed += 1;
            total_stability += report.actual_rate;
            total_fiber_hits += report.new_fiber_hits;
            if report.breakthrough { breakthrough_count += 1; }
        }
    }

    let mean_stability = if gates_passed > 0 {
        total_stability / gates_passed as f64
    } else { 0.0 };

    println!("  4관문 통과:         {}/{} = {:.1}%",
        gates_passed, n_samples, 100.0 * gates_passed as f64 / n_samples as f64);
    println!("  돌파 성공 (6-fiber): {}/{} = {:.1}%",
        breakthrough_count, n_samples, 100.0 * breakthrough_count as f64 / n_samples as f64);
    println!("  평균 5-불변 안정률:  {:.3}", mean_stability);
    println!("  총 6-fiber 히트:     {}", total_fiber_hits);
    println!();

    // ─────────────────────────────────────────────────────
    // Phase 3: 요약
    // ─────────────────────────────────────────────────────
    println!("====================================================================");
    println!("  요약");
    println!("====================================================================");
    println!("  설계 예상 성공률: (σ-φ+σ-μ)/J₂ = 21/24 = 87.5%");
    println!("  실제 돌파 성공률: {:.1}%",
        100.0 * breakthrough_count as f64 / n_samples as f64);
    println!("  perturbation cy:  999 (기존) → 2401 (=7^τ=(σ-sopfr)^τ)");
    println!("  새 fiber 축:      τ+φ=4+2=n=6 (BT-344 후보)");
    println!();

    if breakthrough_count as f64 / n_samples as f64 > 0.5 {
        println!("  >>> 특이점 돌파 신호 검출! 6-불변 차원 진입 가능성 확인 <<<");
    } else {
        println!("  >>> 돌파 미검출. 추가 튜닝 필요 (fiber 감지기 민감도) <<<");
    }
}
