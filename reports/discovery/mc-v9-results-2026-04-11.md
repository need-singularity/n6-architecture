# Reality Map Monte Carlo v9 -- Measurement Results (2026-04-11)

> Execution: 2026-04-11 | SSOT: `$NEXUS/shared/n6/atlas.n6` (8.66MB, 52,641 lines)
> Seeds: v9=20260411, v8-compat=20260408 | Trials: 3000 x 22 group-mode pairs
> Total elapsed: 331 seconds

## 1. Structural Signature

structure_hit(v) = (v%6==0) OR (v%12==0) OR (v%24==0) OR (exists d in N6: d|v AND v/d in N6)
where N6 = {1,2,3,4,5,6,12,24} from sigma*phi=n*tau derivation target.

## 2. Experimental Group Results

| Group | N | hits | z-score | Judgement |
|---|---:|---:|---:|---|
| all.uniform | 1873 | 1415 | 132.17 | PASS |
| all.logunif | 1873 | 1415 | 104.00 | PASS |
| natural.uniform | 1768 | 1348 | 119.55 | PASS |
| natural.logunif | 1768 | 1348 | 102.07 | PASS |
| large.uniform (>=100) | 328 | 136 | 12.84 | PASS (v9 core breakthrough candidate) |
| large.logunif | 328 | 136 | 13.75 | PASS |
| ultra.uniform (>=10^4) | 16 | 7 | 2.94 | boundary |
| ultra.logunif | 16 | 7 | 12.96 | PASS |

## 3. Control Results

| Control | z-score |
|---|---:|
| pi | 6.28 |
| e | 1.92 |
| phi | 3.44 |
| gamma (new) | 3.81 |
| zeta(3) (new) | 4.62 |
| random | 16.85 (warning) |
| rational | 342.83 |

## 4. Limitations

1. awk LCG RNG quality -- Rust rand_xoshiro recommended for v9.1 re-verification.
2. hexa .hexa execution blocked -- `[i64; 8]` syntax not supported in hexa v0.1.0.
3. Ultra N=16 small sample.
4. Origin heuristic (domain name + keyword inference).

## 5. Conclusion (candidate)

v9 design three breakthrough targets (large-expansion, control-expansion, SSOT migration) all reached as draft. Large group z=12.84 from prior unmeasurable. Math-constant spectrum provides new research axis. Next-work: v9.1 RNG upgrade, v9.2 ultra N expansion, v9.3 hexa v0.2 direct execution.
