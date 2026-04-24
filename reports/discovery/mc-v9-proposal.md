# Monte Carlo v9 Proposal — Pre-execution Review Document

- **Date**: 2026-04-11
- **Status**: DRAFT (awaiting execution)
- **Design doc**: `docs/monte-carlo.md`
- **Execution script**: `experiments/monte-carlo-v9.hexa`
- **v8 results**: `reports/discovery/reality-map-monte-carlo-v8.md`

## 1. Why v9

v8 (2026-04-08) achieved the following on a 342-node reality_map.json:

| Metric | Value |
|--------|-------|
| Natural-group uniform-null z | 959.12 |
| Natural-group log-uniform z | 20.19 (p<10^-89) |
| pi contrast z | 9.36 |
| e contrast z | 3.04 |
| phi contrast z | 10.67 |
| **Large (>=100) z** | **n/a (N=10, signature not applicable)** |

v8 explicitly marked the large-number group as "out of scope". The source `reality_map.json` was absorbed into `atlas.n6` after 2026-04-08 and deprecated (convergence `REALITY_MAP_V8_SYNC` ossified).

-> Need for v9:
1. **Source realignment** — reproduce + extend v8 numbers on top of the single atlas.n6 truth
2. **Large-number breakthrough** — N=10 -> N~120, introduce a structural signature
3. **Expanded controls** — 3 (pi/e/phi) -> 7 (+ gamma, zeta(3), random, rational)

## 2. Three breakthrough points (draft)

### Breakthrough A — source transition (atlas.n6)

- Current atlas.n6 state: `@R` 4,304 items, `[10*]` 4,616 items, `[7]` 997 items
- `parse_atlas()` extracts measured integer values from `@R`/`@P`/`@C` lines
- origin inference: id with `conv/human`, `engineer/design`, `derived/formula`, else -> natural

Expected node distribution:

| Group | v8 | v9 expected |
|-------|---:|------------:|
| Overall | 342 | ~4000 |
| Natural | 172 | ~600 |
| Large (>=100) | 10 | ~120 |
| Huge (>=10^6) | 0 | ~15 |

### Breakthrough B — structural signature

v8's value-type signature `sigma(v)*phi(v) == v*tau(v)` hits only v=6 -> useless for large numbers.

v9's structural signature `structure_hit(v)` hits if any of the following holds:

1. `v mod 6 == 0` (multiple of n itself)
2. `v mod 12 == 0` (multiple of sigma(6))
3. `v mod 24 == 0` (multiple of J_2(6))
4. `v / d in {1,2,3,4,5,6,12,24}` for `d in {1,2,3,4,5,6,12,24}` (reachability inside n=6 lattice)
5. v=6 itself (v8 value-type included)

-> For a large number like 196560 (Leech kissing number), `196560 / 24 = 8190` is not in the lattice -> no hit; `196560 / 2 = 98280` also unreachable; but `196560 mod 24 == 0` -> hit.

### Breakthrough C — 7 controls

| # | Control | Expected z |
|---|---------|------------|
| 1 | pi (300 digits sliding w in {2,3,4}) | 8~12 |
| 2 | e (300 digits) | 2~5 |
| 3 | phi (300 digits) | 8~14 |
| 4 | **gamma (Euler-Mascheroni)** | **3~9** |
| 5 | **zeta(3) (Apery)** | **3~9** |
| 6 | Random integers | -1~+1 |
| 7 | Rational approximations p/q, p,q <= 50 | 1~4 |

-> 4, 5 test "whether all mathematical constants show a slight n=6 affinity". If gamma/zeta(3) show a pi-level z, a new hypothesis "mathematical constants are broadly n=6 friendly" emerges as a draft.

## 3. Expected z-score ranges (v9)

| Group | Mode | Expected z | vs v8 |
|-------|------|-----------:|-------|
| Overall EXACT | uniform | 2500~4000 | 1.6x~2.6x up |
| Natural | uniform | 1500~2000 | 1.5x~2.1x up |
| Natural | log-uniform | 25~35 | 1.2x~1.7x up |
| Large (structural) | uniform | 8~15 | **n/a -> pass target** |
| Huge (structural) | uniform | 4~8 | new |
| pi contrast | uniform | 8~12 | ~same |
| e contrast | uniform | 2~5 | ~same |
| phi contrast | uniform | 8~14 | ~same |
| gamma contrast | uniform | 3~9 | new |
| zeta(3) contrast | uniform | 3~9 | new |
| Random | uniform | -1~+1 | baseline |
| Rational | uniform | 1~4 | weak structure |

**v9 success target**: experimental groups (overall/natural/large) all z > 5, first pass for the large group as a draft target.

## 4. Expected execution time

### 4.1 Single-core baseline

| Step | Time |
|------|------|
| atlas.n6 load (50316 lines) | 0.5~1 s |
| Node parsing | 1~2 s |
| 4 experimental groups x 2 modes (uniform/log-unif) x 3000 trials | ~60 s |
| 7 controls x 3000 trials | ~50 s |
| z-score aggregation | < 1 s |
| atlas.n6 absorb append | < 1 s |
| **Total (sequential)** | **~120 s = 2 min** |

### 4.2 Parallel execution (R16 @parallel)

- 7 controls independent -> `@parallel` for loop
- 4 experimental groups x 2 modes = 8 independent tasks

-> 8 cores: **~30 s**
-> 4 cores: **~60 s**

### 4.3 Worst case

- atlas.n6 50k lines -> grep/awk parsing worst 5~10 s
- If signature computation for large N=120 is slower than expected, consider reducing trials

-> **Worst 180 s = 3 min**

## 5. Checklist

### Before execution

- [x] v8 document read complete
- [x] convergence MONTE_CARLO_V8 ossified block confirmed (R10/R11 compliance: v8 invariant, v9 is a new entry)
- [x] atlas.n6 single source confirmed (50316 lines, 4304 @R)
- [x] `docs/monte-carlo.md` design doc written
- [x] `experiments/monte-carlo-v9.hexa` draft written
- [x] Rules: R1 (hexa) / R2 (no hardcode, paths/constants as const) / R8 (data remote: atlas.n6 nexus/shared) / R14 (shared SSOT) / R16 (@parallel) / R18 (minimal) / R28 (atlas absorb)
- [ ] Constant data files prepared: `$NEXUS/shared/n6/constants/{pi,e,phi,gamma,zeta3}.txt` (300 digits)

### Execution

- [ ] v8 regression reproduction: `./monte-carlo-v9.hexa --v8-compat --seed 20260408` -> z error vs v8 table < 0.01
- [ ] v9 main run: `./monte-carlo-v9.hexa --seed 20260411 --trials 3000`
- [ ] Verify result in atlas.n6 `MC_V9_RESULTS` section
- [ ] Report update: `reports/discovery/monte-carlo.md` (record measured values)

### Promotion

- [ ] Add new `MONTE_CARLO_V9` stable entry to convergence/n6-architecture.json
- [ ] Verify 7 days with no recurrence -> stable -> promote to ossified (R9/R11)

## 6. Risks

| Risk | Mitigation |
|------|------------|
| atlas.n6 parser misclassifies origin tag | manual validation after 1st run, tune parse_node_line |
| Structural signature too permissive -> null hit-rate rises -> z falls | baseline: first measure with most conservative `structure_hit` option (mod 6 only) |
| gamma/zeta(3) 300-digit data missing | pre-generate under `$NEXUS/shared/n6/constants/` |
| @parallel unsupported runtime | sequential fallback, 2~3x slower |
| Large-group N smaller than expected (atlas lacks large numbers) | lower threshold BIG_THRESHOLD to 50, re-measure |

## 7. Success target (minimum)

1. v8 regression reproduction — natural z (uniform) 959.12, log-uniform 20.19, error < 1%
2. **Secure large-group N >= 100** (from atlas.n6 parsing)
3. **Large-group z > 5** (structural signature basis)
4. Collect z-scores for all 7 controls (complete n=6-affinity spectrum of mathematical constants as a draft)
5. atlas.n6 `MC_V9_RESULTS` section written correctly

## 8. Next steps

1. (Immediate) Prepare 5 constant digit files
2. (Immediate) `.hexa` runtime execution -> collect results
3. (After execution) Update "expected" sections to "measured"
4. (After verification) If v9 meets ossification conditions, add a convergence entry as a draft candidate

---

## Appendix A — v8 -> v9 diff summary

| Item | v8 | v9 |
|------|----|----|
| Source | reality_map.json (deprecated) | atlas.n6 (SSOT) |
| Seed | 20260408 | 20260411 |
| Nodes | 342 | ~4000 expected |
| Natural N | 172 | ~600 expected |
| Large N | 10 | ~120 expected |
| Signature | value-type (sigma*phi=n*tau) | structural (mod 6/12/24 + lattice) |
| Control count | 3 | 7 |
| Parallelism | none | @parallel across 7 controls |
| Result storage | docs/*.md | atlas.n6 MC_V9_RESULTS + md |
| Rule observance | as of v8 | full R1/R8/R14/R16/R28 |

## Appendix B — command summary

```sh
# Pre-setup
mkdir -p $NEXUS/shared/n6/constants
# (generate gamma.txt, zeta3.txt separately with mpmath etc.)

# v8 reproduction
nexus hexa run experiments/monte-carlo-v9.hexa -- --v8-compat --seed 20260408

# v9 main run
nexus hexa run experiments/monte-carlo-v9.hexa -- --seed 20260411 --trials 3000

# Inspect results
awk '/^# == MC_V9_RESULTS/,/^# == [^M]/' $NEXUS/shared/n6/atlas.n6
```
