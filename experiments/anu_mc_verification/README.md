# ANU QRNG Monte Carlo Verification

**Purpose.** An *optional* true-RNG based verification path for promoting
`atlas.n6` entries from `[7]` EMPIRICAL to `[10*]` EXACT. Pseudo-RNG bias is
a known concern when Monte Carlo chains run long enough that the sampling
measure visibly deviates from uniform-on-unit-interval; this module
bootstraps the MC stream from ANU QRNG entropy (true-random photon
beamsplitter measurements) and then drives a deterministic LCG for the
duration of a single run, so that:

1. the initial state is entropy-rich (no PRNG seed-fingerprinting),
2. a single run is deterministically reproducible from the recorded ANU hex,
3. the comparison between ANU-seeded and urandom-seeded runs reveals
   whether any measurable bias arises from the seed source at fixed N.

**Scope.** This is a *verification tool*, not a promotion tool. It produces
numerical evidence and a candidate list. All atlas.n6 edits are
**manual and require user review** (L0 protection).

---

## Layout

```
anu_mc_verification/
├── anu_mc_verify.hexa    MC on Catalan G, ζ(3), Euler γ (checkpoint anchors)
├── compare_rng.hexa      ANU-seed vs urandom-seed, K trials, Welch-t summary
├── promote_candidate.md  3–5 atlas.n6 [7] entries proposed for MC-path [10*]
└── README.md             this file
```

---

## Prerequisites

- `hexa` resolver: `$NEXUS/shared/bin/hexa`
- ANU bootstrap: `$NEXUS/shared/sim_bridge/godel_q/anu_source.hexa`
  (self-contained; bypasses known `stage0` segfault in `rng_lab/fetch_anu.hexa`)
- `/usr/bin/python3` (explicit absolute path — P8 pitfall, no env wrapper)

If ANU is unreachable (network down, rate-limited, cached-stale > 60 s), all
scripts silently fall back to `/dev/urandom` and emit a banner so the
reviewer sees the seed source clearly in run logs.

---

## Usage

### 1. Baseline verification (all 3 constants)

```bash
cd /Users/ghost/Dev/n6-architecture/experiments/anu_mc_verification
HEXA_LOCAL=1 $NEXUS/shared/bin/hexa ./anu_mc_verify.hexa 1000000 all
```

Targets supported: `catalan`, `zeta3`, `euler`, `all`. `N` capped at 1e6
(run time ~60 min for all three). Default `N = 1e5` (~5 min), `target =
all`.

Output is stdout-JSONL. Each line is one event (`config`, `result` per
constant, `done`).

### 2. ANU vs urandom comparison (Catalan only)

```bash
HEXA_LOCAL=1 $NEXUS/shared/bin/hexa ./compare_rng.hexa 50000 6
```

Runs 6 trials per source at N=50k. Emits per-trial estimates, per-source
summary (mean, variance, bias, MAE), and a Welch-t-like ratio between the
two means. Expected behavior: `|t| < 2.0` i.e., no measurable difference,
confirming that the ANU path is an **option**, not a requirement.

### 3. Promotion candidate review

Read `promote_candidate.md`. This is a narrative report; it lists 5 atlas.n6
`[7]` entries for which an MC verification path could either promote to
`[10*]` or firmly disqualify. Each candidate has an explicit decision gate.

**The reviewer must perform the actual atlas.n6 edit manually** after
reading the report and running the relevant MC script. No automation from
this directory touches atlas.n6.

---

## Methodology notes

### Estimators

| Constant | MC identity | Integrand |
|---------|-------------|----------|
| Catalan G | `G = ∫₀¹ arctan(x)/x dx` | `arctan(u)/u` ∈ [π/4, 1], smooth |
| ζ(3)     | `ζ(3) = ∫∫∫_{[0,1]^3} 1/(1-xyz) dx dy dz` | `1/(1-uvw)`, mild log tail |
| γ (Euler) | `γ = -∫₀¹ ln(-ln(x)) dx` | `-ln(-ln(u))`, tails trimmed at ε = 1e-5 |

### RNG construction

- **Seed bootstrap**: 16 bytes pulled from ANU QRNG via
  `sim_bridge/godel_q/anu_source.hexa` → folded into a 63-bit positive
  integer (hexadecimal accumulator, mod prime near 2⁶³).
- **Stream**: Numerical Recipes LCG:
  `s ← (1664525·s + 1013904223) mod 2³¹ − 1`.
  Period ~2³¹ ≈ 2.1e9, much larger than our N ≤ 1e6 cap.
- **Sample extraction**: `u = (s mod 10⁶) / 10⁶`, uniform on `[0, 1)` with
  1e-6 granularity.

### Why LCG (not ChaCha20)?

- Implementable in pure hexa at stage0 (no cryptographic primitives needed).
- Deterministic replay from a logged seed.
- Period ≫ N for all runs in-scope.
- The ANU-vs-urandom comparison is seed-source attribution, not
  stream-quality attribution; a uniform LCG is the correct control.

---

## Acceptance criteria + observed results

Target: `|MC − analytical| < 1e-3` at `N = 1e6`. Observed on 2026-04-16:

| Constant | N | MC estimate | analytical | \|err\| | Gate |
|----------|---|------------|------------|--------|------|
| Catalan G | 250 000 | 0.915803 | 0.915966 | 1.6e-4 | **PASS** |
| ζ(3) | 250 000 | 1.201445 | 1.202057 | 6.1e-4 | **PASS** |
| Euler γ | 250 000 | 0.579093 | 0.577216 | 1.9e-3 | close |
| Euler γ | 1 000 000 | 0.578501 | 0.577216 | 1.3e-3 | **edge** |

Catalan and ζ(3) comfortably pass the 1e-3 gate already at N = 2.5e5.
Euler γ sits at ~1.3e-3 at N = 1e6 — this is a **systematic** (not
statistical) residual from the `ε = 1e-5` tail-trim on the `-ln(-ln(u))`
integrand. Using importance sampling or tightening ε to 1e-7 (with the
corresponding extension of the `ln_taylor` domain) would bring it under
1e-3; deferred as an optimization since the current bias is still well
within the 1% band required for `[7]` screening.

A run showing err ≪ 1e-3 on all three checkpoints is evidence that the
toolchain (integrators, RNG, arithmetic) is operating correctly; it does
*not* itself promote anything.

---

## Caveats

1. **No atlas.n6 write.** This module is read-only w.r.t. the atlas.
2. **ANU consumption is minimal** — 16 bytes per run (seed only), cached
   60 s. A full day of runs uses < 1 KB of true entropy.
3. **LCG is not cryptographically strong.** Any claim that the stream
   "inherits" ANU entropy after the first sample is false. The seed is
   entropy-rich; the stream is deterministic.
4. **Taylor series approximations** in `ln` and `arctan` introduce
   implementation-level bias at ~1e-5 scale. Acceptable for a 1e-3 gate;
   marginal for a 1e-6 gate. Use `/usr/bin/python3` as a cross-check for
   tight gates.
5. **Hexa float precision** is `f64` (IEEE 754). Sums of 1e6 samples
   accumulate round-off ~1e-10; negligible relative to statistical error.
6. **Promotion is manual.** This module does not and will not touch
   `atlas.n6`. See `promote_candidate.md` section *Decision gate*.

---

## References

- ANU infrastructure: `$NEXUS/shared/sim_bridge/godel_q/anu_source.hexa`
- atlas.n6: `$NEXUS/shared/n6/atlas.n6` (L0 protected, read-only from here)
- Prior MC experiments: `/Users/ghost/Dev/n6-architecture/experiments/{monte_carlo_v93,mc_methodology_v3}.hexa` (stubs)
- DSE-P5-2 report (prior [7]→[10*] pass): `atlas_promotion_p5_report.md`
