# TECS-L ↔ n6-architecture Connection Document

> Last updated: 2026-04-02

## Role separation

| Item | TECS-L | n6-architecture |
|------|--------|-----------------|
| **Identity** | mathematical system (pure theory) | industrial-field application (engineering design) |
| **Core outputs** | proofs, hypotheses, constant derivations | DSE designs, BT validations, AI techniques |
| **Direction** | "why n=6?" | "what do we build with n=6?" |
| **Validation criteria** | mathematical rigor, p-value | EXACT match against measured data |

```
  TECS-L (theory)                   n6-architecture (validation)
  ┌─────────────┐                 ┌─────────────────┐
  │ proofs/hyp  │ ──→ design ──→  │ DSE/BT/tech     │
  │ constants   │ ←── measure ←── │ 307 domains     │
  │ DFS mining  │ ──→ candidates→ │ BT expansion    │
  │ physics     │ ←── industry ←  │ chip/battery/nrg│
  └─────────────┘                 └─────────────────┘
```

## Shared infrastructure

### Sync system
- **Center**: `~/Dev/TECS-L/.shared/`
- **Scripts**: sync-calculators.sh, sync-readmes.sh, sync-claude-rules.sh, sync-dse.sh
- **Atlas**: scan_math_atlas.py → math_atlas.json (2,504 hypotheses + 339 constant maps)
- **DSE domains**: .shared/dse/domains/ (306 TOML, reverse-synced from n6)

### Shared constants
```
  σ(6)·φ(6) = 6·τ(6) = 24     ← core identity
  R(6) = 1                     ← full reversibility
  1/2 + 1/3 + 1/6 = 1          ← Egyptian fractions
```

### Data flow
```
  TECS-L hypothesis (H-XX-NNN)
    ↓ grade by n6 constants (auto_grade_n6.py)
    ↓ on EXACT match
  register n6 BT candidate (BT-94+)
    ↓ validate across 307-domain DSE
    ↓ measured-data confirmation
  both atlases updated together
```

## Current status (2026-04-02)

| Metric | TECS-L | n6 | Combined |
|------|--------|-----|------|
| hypotheses | 1,985 | 1,400+ | 3,385+ |
| constant maps | 339 | 1,100+ | 1,439+ |
| DSE domains | 306 | 307 | synced |
| BTs | - | 93 | 93 |
| Rust LOC | 2,932 | 14,063 | 16,995 |
| Python calc | 150+ | 58 | 208+ |

## Integration work history

| Date | Work | Effect |
|------|------|------|
| 2026-04-02 | 304 TOML reverse-synced (n6→TECS-L) | TECS-L DSE 31→306 |
| 2026-04-02 | 618 ungraded auto-grading script | 573 (93%) n6 matches found |
| 2026-04-02 | BT→hypothesis reverse-generation (n6→TECS-L) | BT ⭐⭐⭐ → H-N6 hypothesis files |
| 2026-04-02 | Cross-Repo constant mining | 307-domain pattern analysis |
| 2026-04-02 | Constant-map reverse-extraction (n6→TECS-L) | atlas cross-reinforcement |

## Sync commands

```bash
# TECS-L → n6 (forward)
cd ~/Dev/TECS-L && bash .shared/sync-calculators.sh

# n6 → TECS-L (reverse: DSE domains)
cp $N6_ARCH/tools/universal-dse/domains/*.toml ~/Dev/TECS-L/.shared/dse/domains/

# Atlas scan
python3 ~/Dev/TECS-L/.shared/scan_math_atlas.py --save --summary

# Auto-grading
python3 ~/Dev/TECS-L/calc/auto_grade_n6.py
```
