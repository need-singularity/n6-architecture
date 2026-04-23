# Monte Carlo Experiment Inventory — 2026-04-23

**source**: `reports/n6_roi.json` `findings.redundant_monte_carlo_files`
**roi-item**: `n6-roi-008` (priority 45, ROI 0.67)
**scanner fix**: `tool/_n6_lib scan_roi` now prunes `venv/`, `_archive/`,
`.so`, `.pyi`, `.pyc`. Count moved 8 → 5 (3 false positives were scipy
binaries inside a sub-experiment's venv).

The remaining 5 files are **not redundant**; each has a distinct purpose and
should be retained. This inventory documents that so future scans don't
re-raise the same flag.

| # | File                                                        | Role                                                              | Status   |
| - | ----------------------------------------------------------- | ----------------------------------------------------------------- | -------- |
| 1 | `experiments/meta/meta_mc_v9_cross_batch.hexa`             | Cross-batch v9 reliability estimator over 250 new experiments     | BODY     |
| 2 | `experiments/anu_mc_verification/anu_mc_verify.hexa`       | ANU QRNG–seeded verification of Catalan G, ζ(3), Euler–Mascheroni γ | BODY     |
| 3 | `experiments/mc_methodology_v3.hexa`                        | v3 methodology doc & driver (397-line Python original)            | STUB (port pending) |
| 4 | `experiments/monte_carlo_v93.hexa`                          | v93 primary run (361-line Python original)                        | STUB (port pending) |
| 5 | `experiments/mc_v93_by_domain.hexa`                         | v93 per-domain breakdown (238-line Python original)               | STUB (port pending) |

**Consolidation verdict**: no. Files 1–2 are operational; files 3–5 are
hexa-port stubs whose originals live under `scripts/`. Merging them would
lose both the methodology separation (v3 vs v93) and the domain breakdown
granularity. The correct next action is porting 3–5 from their Python
originals, which is out of scope for this ROI item.

## Verification

```
bash bin/n6_meta roi
jq '.findings.redundant_monte_carlo_files | length' reports/n6_roi.json
# 8 (with venv noise) → 5 (venv pruned) — all legitimate
```
