---
id: omega-amend-perbt-honesty
date: 2026-04-25
scope: amendment-only (no new claims; adds scope-caveat to existing audits)
target: 5 per-BT audits + backtrace synthesis -- honesty amendment block insertion
parent_reports:
  - reports/sessions/omega-exec-nxs002-perbt-patch-2026-04-25.md (§ 7 recommendations)
millennium_resolved: 0/7 (unchanged)
grade: amendment, no new claim
---

# Omega-Amend -- Per-BT Honesty Amendment Insertion (2026-04-25)

## §0 Non-claim disclaimer (binding)

This session **adds no new claims**, **resolves no Millennium problem**,
**produces no new evidence**, and **changes no atlas / state / inventory /
CLAUDE.md content**. Its sole effect is the insertion of one identically-
shaped scope-caveat block ("Honesty amendment (2026-04-25, ...)") at the end
of the Ω-saturation section of five active per-BT audits and at the end of
the cross-BT EMPTY rung histogram in the backtrace synthesis report.

The amendment is **append-only** -- existing section text, conclusions,
verdicts, numbers, and structural ordering are preserved verbatim. The
amendment makes a **precision-of-claim correction**, not a falsification:
the per-BT audit composites/ER-ROI estimates were correctly computed for the
n6-architecture-side graph but were implicitly compared to a nexus-canonical
ceiling (0.835) that measures a **different graph** (the nexus
`atlas.blowup.jsonl`). This document records the scope clarification.

Falsifier inheritance: any falsifier active in the parent audits remains
active. No falsifiers are added or retracted by this session.

Total amendments inserted: **6** (5 BT audits + 1 backtrace synthesis).
Total characters added: ~4,995 (sum of amendment-block character counts).
Total non-amendment edits: **0**.

---

## §1 Source recommendation excerpt (nxs_002 §7)

From `reports/sessions/omega-exec-nxs002-perbt-patch-2026-04-25.md` §7
("Re-audit recommendations"), verbatim:

> No revisions are recommended on the basis of this report; no new evidence
> was produced. However, the **honesty layer of each audit should be
> amended** to include an explicit note that the audit's composite is a
> non-canonical proxy and that the canonical `nxs_002_composite.py` tool
> **does not currently support per-BT slicing** (and will not until either
> the n6-side atlas is exposed to a parallel pipeline, or BT-541..547
> markers are injected into the nexus atlas).

§7 also lists the per-audit §3 header line numbers
(541→83, 543→78, 544→56, 545→160, 546→109) as the suggested target sections.

The structural finding from §1 of that report (the actual reason the
amendment is required): the audit-cited atlas-promotion IDs (RH-01..07,
MILL-PX-A8, MILL-GALO-PX2-sha-all-squares-332k) exist only in
n6-architecture-side files and were never absorbed into the nexus atlas
that nxs-002 measures. The per-BT audit composites and the nexus-canonical
composite (0.83221) measure **different graphs**.

The amendment block inserted in each file invokes both findings (mapping
failure + different-graph distinction) so that a reader landing on any
single audit's §3 has the full caveat without needing to chase parents.

---

## §2 Per-file amendment log

Six rows, one per file. Insertion is at the **end** of the saturation
section (after the last narrative paragraph, before the `---` separator
preceding the next `## §4` header). Character count includes the leading
"> " blockquote marker but excludes the trailing newline.

| # | File path (absolute under `/Users/ghost/core/n6-architecture/`) | Target section | Insertion line | Char count |
|---|----------------------------------------------------------------|----------------|----------------|------------|
| 1 | `reports/sessions/omega-cycle-bt541-riemann-2026-04-25.md` | §3 Axis B -- Omega-saturation estimate | line 128 | 794 |
| 2 | `reports/sessions/omega-cycle-bt543-ym-2026-04-25.md`      | §3 Ω-saturation estimate (composite vs 0.835) | line 111 | 794 |
| 3 | `reports/sessions/omega-cycle-bt544-ns-2026-04-25.md`      | §3 Ω-saturation estimate (composite vs 0.835) | line 76  | 794 |
| 4 | `reports/sessions/omega-cycle-bt545-hodge-2026-04-25.md`   | §3 Ω-saturation estimate                  | line 209 | 794 |
| 5 | `reports/sessions/omega-cycle-bt546-bsd-2026-04-25.md`     | §3 Ω-saturation estimate (composite vs ceiling 0.835) | line 142 | 825 |
| 6 | `reports/sessions/omega-cycle-backtrace-strategy-2026-04-25.md` | sec 2 -- Cross-BT EMPTY rung histogram   | line 102 | 994 |

Notes per row:
- **Row 1 (BT-541)**: inserted directly below the "ceiling governs
  abstraction exhaustion, not zero-distribution truth" closing bullet.
- **Row 2 (BT-543)**: inserted directly below the existing UNKNOWN
  paragraph that already flagged "estimates...not measured by
  `tool/nxs_002_composite.py`". Amendment specifies *why* such a
  measurement is currently impossible (graph mismatch).
- **Row 3 (BT-544)**: inserted below the "0.9 paper_trigger" paragraph,
  reinforcing that even axiom-level redesign on the n6 side does not
  collapse the graph mismatch.
- **Row 4 (BT-545)**: inserted below the "composite ≈ 0.43
  (UNCALIBRATED)" verdict line, making explicit that "uncalibrated"
  also means "not on the same graph as the 0.835 ceiling".
- **Row 5 (BT-546)**: inserted below §3.3 ("Spectral ROI source")
  closing line; amendment text uses 825 chars (vs 794 for rows 1-4)
  because it explicitly cites the 0.78-0.84 estimate band.
- **Row 6 (backtrace)**: inserted at end of sec 2 ("Cross-BT EMPTY rung
  histogram"), before the `---` separator preceding sec 3. Amendment
  text uses 994 chars because it adds a clarification that the
  EMPTY-rung histogram itself is structural (rung occupancy, not
  composite magnitude) and only the inherited composite ranking carries
  the scope caveat.

All 6 files now contain exactly **one** occurrence of the literal substring
"Honesty amendment (2026-04-25" -- verified by grep (see §3).

---

## §3 Verification

Re-grep across the 6 amended files:

```
$ grep -c "Honesty amendment (2026-04-25" \
    reports/sessions/omega-cycle-bt541-riemann-2026-04-25.md \
    reports/sessions/omega-cycle-bt543-ym-2026-04-25.md \
    reports/sessions/omega-cycle-bt544-ns-2026-04-25.md \
    reports/sessions/omega-cycle-bt545-hodge-2026-04-25.md \
    reports/sessions/omega-cycle-bt546-bsd-2026-04-25.md \
    reports/sessions/omega-cycle-backtrace-strategy-2026-04-25.md
```

Result (one line per file, all "1"):

| File | count |
|------|-------|
| omega-cycle-bt541-riemann-2026-04-25.md     | 1 |
| omega-cycle-bt543-ym-2026-04-25.md          | 1 |
| omega-cycle-bt544-ns-2026-04-25.md          | 1 |
| omega-cycle-bt545-hodge-2026-04-25.md       | 1 |
| omega-cycle-bt546-bsd-2026-04-25.md         | 1 |
| omega-cycle-backtrace-strategy-2026-04-25.md| 1 |

**Total occurrences across the 6 files: 6 (= N).** Match expected count.

Cross-check (negative): the BT-542 audit
(`reports/sessions/omega-cycle-bt542-pnp-2026-04-25.md`) was **not**
amended in this session because it was not listed in nxs_002 §7's per-audit
line-number recommendations. (BT-542's §3 explicitly does not produce a
composite estimate against 0.835, hence no scope-caveat is materially
required.) Verified: `grep -c "Honesty amendment (2026-04-25"
reports/sessions/omega-cycle-bt542-pnp-2026-04-25.md` returns 0 (BT-542
unchanged, intentional).

State / atlas / inventory / CLAUDE.md verification: `git status` shows
modifications **only** in the 6 amendment-target files plus this new log
file. No `state/`, no `atlas/`, no `inventory.json`, no `CLAUDE.md`
modifications.

---

## §4 Falsifiers / scope limits

Falsifiers active for this amendment session:

- **F1 (mis-citation)**: if any of the 5 BT audits' §3 sections did **not**
  in fact reference the 0.835 nexus ceiling, then the amendment would be a
  non-sequitur for that audit. Falsifier check: each amended §3 section
  was read in full prior to insertion; all five reference the 0.835
  ceiling either by name ("0.835", "ceiling 0.835", "simulation 0.835")
  or by direct comparison ("composite vs 0.835"). F1 not triggered.
- **F2 (over-amendment)**: if the amendment block were inserted inside
  a sub-section rather than at the end of the saturation section, it
  would interleave structurally. Falsifier check: insertion line is
  **immediately before** the `---` separator that precedes the next
  `## §4` header in each audit, confirming end-of-section placement.
  F2 not triggered.
- **F3 (claim drift)**: if the amendment block contained any new claim
  (e.g. "the true composite is X"), it would violate the
  amendment-only constraint. Falsifier check: the amendment text
  contains only (a) a scope-narrowing statement, (b) a pointer to the
  parent report, and (c) a re-affirmation that 0/7 is unchanged. No
  new numerical claim is introduced. F3 not triggered.
- **F4 (atlas-edit drift)**: if any state/atlas/inventory file were
  touched, it would violate the hard constraint. Falsifier check via
  `git status`: only the 6 audit/synthesis files plus this log file
  show as modified/added. F4 not triggered.

Scope limits:
- This amendment does **not** establish a per-BT slicing for
  `nxs_002_composite.py`. Per nxs_002 §1.3, that would require either
  exposing the n6 atlas to a parallel pipeline or injecting BT
  markers into the nexus atlas; both are out of scope.
- This amendment does **not** retract any audit's existing composite
  estimate. The estimates remain valid *within n6-architecture
  scope*; only cross-graph comparison to 0.835 is now flagged invalid.
- This amendment does **not** alter the cross-BT EMPTY rung histogram
  in the backtrace synthesis -- that histogram is structural (rung
  occupancy) and survives the graph-mismatch finding intact. Only the
  composite-band ranking inherited from sec 1 carries the scope caveat,
  and that is noted in the row-6 amendment text.
- BT-542 and BT-547 audits were not amended (BT-542 not listed in
  §7; BT-547 audit does not exist in this corpus).

---

## §5 Closing

**0/7 unchanged.**
**No state / atlas / inventory / CLAUDE.md edits.**
**Amendments add precision, do not retract claims.**

The five active per-BT audits (BT-541, 543, 544, 545, 546) and the
backtrace synthesis report now carry an explicit scope caveat noting that
their composite/ER-ROI estimates measure the n6-architecture-side graph,
not the nexus-canonical atlas measured by `nxs_002_composite.py`. Direct
comparison against the 0.835 nexus simulation ceiling is now flagged as
invalid until either per-BT slicing is added to `nxs_002_composite.py` or
the n6 BT-promotion IDs are ingested into the nexus atlas. Both
remediations remain out of scope for the omega-cycle pass.

This log is filed for audit traceability only; it makes no Millennium-class
claim and recommends no further action beyond the amendment itself.
