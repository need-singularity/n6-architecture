# cat-litter mk2 trial proposal (2026-Q4)

> **Domain**: `domains/pets/cat-litter` (inaugural pets-axis domain)
> **Status**: PROPOSAL (mk1 DRAFT → mk2 PILOT transition spec)
> **Author**: M. Park (n6-architecture private framework)
> **Date**: 2026-05-01
> **Predecessor**: `domains/pets/cat-litter/cat-litter.md` (mk1, registered 2026-05-01)
> **Successor gate**: F-CL-MVP-1..4 falsifier resolution (2026-07-30 / 2026-08-30)

## §1 WHY

mk1 is theoretical-analytical only (literature-anchored from Grim 1968 +
supplier specs). The 5 n=6 invariant projections (σ=12 swell × τ=4 odor
tier × φ=2 grain mode × sopfr=5 mineral × J₂=24 hour) are **design
intent**, not measured outcomes. mk2 closes the empirical gap: real
100 kg pilot batch + real 24-household 6-month trial → real measurements
against the 4 pre-declared falsifiers.

## §2 STRUCTURE (why 24 households / 6 months / 4 stages)

| Number | n=6 mapping | Operational meaning |
|---|---|---|
| **24 households** | J₂(6) = 24 (Jordan totient) | Sample power: ≥16 for qualitative significance, ≥20 for quantitative — 24 brackets both |
| **6 months** | n=6 | Spans 1.5 seasonal cycles (summer NH3 burst + winter low-vent stagnation) |
| **4 stages** | τ(6) = 4 | Staggered rollout — 1 stage/6 weeks, 6 households/stage |
| **6 households/stage** | n=6 | Each stage measures all 4 falsifiers independently |
| **100 kg batch** | — | 24 hh × 3 kg/mo × 6 mo ÷ 4 stage = ~25 kg/stage; 4× safety margin |

## §3 RECRUITMENT (4 weeks, 2026-09-01 → 2026-09-28)

- Target: 24 households, ≥1 cat each, stable 6-month residency.
- Demographic split: 6:6:6:6 across (1-person / 2-person / 3-person /
  4+-person households) — equal distribution.
- Geographic: ≥4 distinct climate zones (humid coastal / dry inland /
  high-altitude / temperate continental) to span the F-CL-MVP-4 odor-
  axis sensitivity to RH/temp.
- Pre-existing-litter brand recorded as control comparison baseline.
- IRB exempt: consumer product, no medical claim, no minor subjects.

## §4 STAGED ROLLOUT (4 stages × 6 weeks = 24-week active phase)

```
2026-10-01  Stage 1 (6 households) — fine-clumping mode primary
2026-11-15  Stage 2 (6 households) — coarse-non-clumping mode primary
2027-01-01  Stage 3 (6 households) — both modes (φ=2 SKU coverage measurement)
2027-02-15  Stage 4 (6 households) — control group (commodity baseline first
                                       3 months, cross-over to mk2 at month 4)
2027-03-31  All stages closed; 6-month observation complete for stage 1
2027-04-30  Final-stage 6-month observation complete; analysis begins
```

## §5 MEASUREMENTS (per household, monthly)

Each falsifier maps to a quantitative real-household measurement:

| Falsifier | mk1 lab target | mk2 in-home measurement protocol |
|---|---|---|
| **F-CL-MVP-1** swell | 12× (DI water 24 h) | Weekly weight on incoming bag + waste bag; estimate average absorbed mass per replacement cycle |
| **F-CL-MVP-2** clump | ≥ 50 kPa | Photo at 1-week and 4-week post-deposit; clump integrity score (1–5) by 2 blinded raters |
| **F-CL-MVP-3** dust | < 200 µg/m³ pour | 3 households (1 per first-3 stages) instrumented with PM2.5 sensor (Plantower PMS5003) at 30 cm above litter box; logged over a 1-min pour event monthly |
| **F-CL-MVP-4** odor (J₂=24h) | 24h baseline | Weekly self-reported pad-change frequency + binary "noticeable smell at end of day Y/N" survey; F-CL-MVP-4 fires if median pad-change-interval falls below 24 h across stage |

Additional qualitative metrics (not falsifier-gated, but informative):

- **Cat behavior**: box-entry frequency via household webcam (computer-
  vision count). Baseline first 2 weeks before mk2 swap.
- **Tracking**: monthly visual count of stray litter granules at 1 m / 3 m
  from box.
- **Customer NPS**: monthly 0–10 survey "would you recommend?".
- **Allergic reactions** (human / cat): zero-tolerance falsifier — any
  reaction → immediate mk2 withdrawal and post-mortem analysis.

## §6 BUDGET (estimated, USD)

| Item | Estimate |
|---|---|
| 100 kg pilot batch (manufacturing) | ~$110 (1.10 $/kg × 100 kg) |
| PM2.5 sensors × 3 (Plantower PMS5003 + ESP32 logger) | ~$150 |
| Household incentive ($20/month × 24 hh × 6 mo) | ~$2,880 |
| 5 kg / 10 kg / 20 kg bag retail-pack tooling | ~$300 |
| Data collection / analysis researcher time | ~$3,000 |
| Camera-vision pipeline development | ~$500 (open-source baseline) |
| Reserve / contingency (20%) | ~$1,400 |
| **Total** | **~$8,340** |

## §7 GATE OUTCOMES (what mk3 looks like at each branch)

| Outcome | Definition | Next cycle |
|---|---|---|
| **PASS-ALL-4** | All 4 falsifiers PASS (no mid-trial breach) | mk3 = 1-ton commercial run, 2027-Q2 retail launch (3 SKUs × 2 modes) |
| **PARTIAL** (1–2 fire) | 1 or 2 falsifiers fire, root cause identified | mk2.1 reformulation cycle (3 months) → re-trial with 12 households (half-size) |
| **FAIL** (≥3 fire) | 3+ falsifiers fire OR critical safety breach | Domain PIVOT or RETIRE — re-examine n=6 invariant mapping; cat-litter may not be the right pets-axis flagship; consider replacement (cat-food / habitat-substrate) |

## §8 RISKS & MITIGATIONS

| Risk | Probability | Mitigation |
|---|---|---|
| Bentonite supplier swell-cert variance (mk2 batch < 12×) | MED | Pre-batch QC: reject lots < 11× at receipt; require 3× certificate redundancy |
| Household drop-out (target n=24 → actual n<20) | MED | Over-recruit (n=28); incentive extends 1 month for cat-sitting completion |
| Cat-aversion to new litter (rejection rate > 20%) | MED | Pre-trial 2-week mixed-litter transition (75% old / 25% new → 50/50 → 25/75 → mk2) per ACVB guideline |
| Climate-zone bias (only 1 climate represented) | LOW | Geographic diversification quota in §3 RECRUITMENT |
| PM2.5 sensor calibration drift | LOW | Monthly co-located reference (TSI DustTrak) check on 1 unit |

## §9 ACCEPTANCE / MISS criteria (pre-declared per own#12)

- **ACCEPT**: ≥ 3 of 4 falsifiers PASS (75%+) AND no safety breach AND
  ≥ 18 of 24 households complete the 6-month period.
- **MISS** if any of:
  - (a) ≥ 2 falsifiers fire,
  - (b) safety breach (cat or human allergic / toxic reaction),
  - (c) household completion rate < 75%,
  - (d) PM2.5 instrument calibration drift > 20% during trial.
- **DEFER**: F-CL-MVP-4 (odor) gate is 2026-08-30; if mk2 trial extends
  beyond, re-baseline acceptance against the most recent measurement.

## §10 LITERATURE / PRIOR ART

- **Grim, R. E.** (1968). *Clay Mineralogy*. McGraw-Hill — bentonite swell baseline.
- **ACVB** (2018). "Litter preferences in domestic cats" — pre-trial transition protocol.
- **Plantower** (2018). PMS5003 datasheet — PM2.5 sensor characterization.
- **CWA-PMS 2008-3** — bentonite swell-ratio measurement standard.
- **NIST CODATA** — thermodynamic anchor for ion-exchange enthalpy.
- **OEIS** A000005 / A000010 / A000203 / A001414 / A007434 — n=6 number-
  theoretic constants (σ, τ, φ, sopfr, J₂).

## §11 OPEN QUESTIONS

1. Should the control-group cross-over at month 4 be informed (Stage 4
   households told they will receive mk2) or blind? Blind = stronger
   data; informed = better recruitment compliance.
2. Is the φ=2 SKU coverage measurement (Stage 3) within-household or
   cross-household? Within-household requires 2 boxes per stage-3 home;
   cross-household halves stage-3 sample size to n=3 per mode.
3. Should the mk2.1 reformulation branch include a binder additive
   (CMC / xanthan gum) for clump strength, or stay pure-bentonite?

## §12 LINKS

- Predecessor (mk1): `domains/pets/cat-litter/cat-litter.md`
- Sub-SSOT: `domains/pets/_index.json`
- Parent SSOT: `domains/_index.json` (v1.1.0, _axes 11→12)
- Lint gate: `tool/own31_verify_tautology_ban_lint.hexa --file <path>`
- Master identity reference: `papers/hexa-weave-formal-mechanical-w2-2026-04-28.md`
  (Lean 4 mechanical verification of σ·φ=n·τ at n=6)

## mk-history

- 2026-05-01T05:00:00Z — initial draft. mk2 trial structure derived from
  J₂(6)=24 households + n=6 months + τ(6)=4 stages × n=6 households/stage.
  4 falsifiers map 1-to-1 from mk1 lab targets to in-home measurement
  protocols. Budget ~$8,340 (1 kg batch sufficient for full trial). IRB
  exempt; pre-existing ACVB transition protocol cited for cat-aversion
  mitigation. Open questions deferred to user resolution before
  recruitment opens 2026-09-01.
