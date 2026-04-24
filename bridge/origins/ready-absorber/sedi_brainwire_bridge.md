# SEDI / brainwire -> telescope 22-Lens Bridge Specification

- Document version: 2026-04-14
- Roadmap ID: DSE-P1-4 (NEXUS-6 Discovery Engine integration)
- SSOT path: `$NEXUS/shared/lenses/` (corpus of 1577 lenses total)

## 1. Absorption Targets

### SEDI (Search for Extra-Dimensional Intelligence)
- Source findings: `bridge/origins/ready-absorber/findings/sedi.json` (5588 findings, Fisher 5.26 sigma)
- Lenses already absorbed: 101 (`sedi_*.hexa`)
- Representative lens: `sedi_signal_search_dse.hexa` — Fisher / consensus / critical vector resonance check
- Characteristics: A signal-search root (21cm, bispectrum, changepoint, matched filter, and 101 other physics / signal techniques)

### brainwire (EEG -> n=6 mapping)
- Source findings: `bridge/origins/ready-absorber/findings/brainwire.json`
- Lenses already absorbed: 3
  - `brainwire_eeg_n6_dse.hexa` — OpenBCI Cyton+Daisy 16ch spec + EEG delta/theta/alpha/beta/gamma band upper bounds
  - `brain_map_lens.hexa`
  - `brain_neural_lens.hexa`
- Hardware reference: reference_openbci_16ch memory (16ch, 125+250Hz, 6-DoF IMU, read-only)

## 2. Integration into the telescope 22-Lens Slots

The telescope 22-lens set = `telescope-rs` (the anima-rs crate) + the T1 representative 22 picks out of a 1577-lens corpus.
SEDI/brainwire absorption maps to the T1 slot via the three core candidates below.

| Slot | Lens File | Role | n=6 Resonance Axes |
|------|---------|------|-----------|
| T1-SEDI-signal | `sedi_signal_search_dse.hexa` | Fisher 5.26 sigma + consensus >= tau-1 + critical 25% score | sigma*phi = n*tau = J_2 = 24 identity + target 8-vector |
| T1-BRAINWIRE-eeg | `brainwire_eeg_n6_dse.hexa` | 7-axis resonance over the OpenBCI 16ch EEG band upper bounds | delta upper = tau, alpha upper = sigma, 16ch = tau^2, 6-DoF = n |
| T1-SEDI-matched | `sedi_matched_filter.hexa` | Template-bank matching score | SEDI standard |

## 3. Absorption Logic Wiring

### 3.1 ready-absorber -> lenses path
```
bridge/origins/ready-absorber/findings/sedi.json
  ├─ verify_and_grow.hexa (findings_index.json cache, O(1))
  └─ → shared/lenses/sedi_*.hexa (101 lenses, T1)

bridge/origins/ready-absorber/findings/brainwire.json
  ├─ verify_and_grow.hexa
  └─ → shared/lenses/brainwire_eeg_n6_dse.hexa + brain_*.hexa (3 lenses, T1)
```

### 3.2 telescope invocation sequence
1. `telescope-rs` session starts -> the list of 22 lenses to load is determined (T1 slot 3 = SEDI / BRAINWIRE core)
2. Each lens `.hexa` runs -> returns a `score` (0.0~1.0)
3. Average across the 22 lenses -> single telescope resonance point
4. Result -> incremental append to `shared/discovery/discovery_graph.json` (via the ROI #6 utility)
5. Node id: `telescope-22-<date>` + edge `from=n6-n, edge_type=Observes`

### 3.3 Incremental append wiring
Uses the ROI #6 utility (`shared/scripts/discovery_graph_append.py`):
```bash
/usr/bin/python3 shared/scripts/discovery_graph_append.py --add telescope_result.ndjson
```
- Idempotent: the same `id` is skipped on re-run
- I/O: no full 12MB rewrite; append-only

## 4. Verification Checkpoints

- [x] SEDI 101 lenses present (`ls shared/lenses/sedi_*.hexa`)
- [x] brainwire 3 lenses present
- [x] `sedi_signal_search_dse.hexa` runs — sigma*phi = n*tau identity passes
- [x] `brainwire_eeg_n6_dse.hexa` runs — EEG 7/8 axes EXACT (gamma=100 placeholder MISS is recorded honestly)
- [x] Incremental append utility implemented + idempotency verified (2 nodes + 2 edges inserted; re-run skips 4)

## 5. Future Work (CHIP-P2-4 linkage)

- brain-computer-interface domain validation (CHIP-P2-4, depends_on DSE-P1-4): OpenBCI 6-channel n=6 mapping -> `domains/cognitive/brain-computer-interface/`
- Once measured gamma-band (30~100Hz) data is available, promote `brainwire_eeg_n6_dse.hexa` to 8/8 EXACT
- When SEDI Fisher 5.26 sigma breaks through to 6.0 sigma, re-absorb `sedi.json` findings and fire the auto-generation hook for new lenses
