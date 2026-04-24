# Megahub Detection Report (2026-04-15)

> Structural observation based on atlas.signals.n6, not a proof.
> H9 task -- 4-BT megahub automatic search (scripts/detect_megahub.py)

- SSOT: `/Users/ghost/Dev/nexus/shared/n6/atlas.signals.n6`
- Total signals: 258
- Millennium tag set: `['7B','7H','7N','7P','7R','7S','7Y']`
- Threshold: 4-BT or more
- Existing megahub: SIG-META-001 (emergence-312-meta-analysis)

## 1. Millennium Tag Distribution

| Simultaneous holdings | signals |
|---:|---:|
| 0 | 192 | 1 | 55 | 2 | 3 | 3 | 1 | 4 | 4 | 6 | 3 |

## 2. >=4-BT Megahub Candidates (7)

| sig_id | Millennium tags | #BT | grade | evidence | witness |
|---|---|---:|---|---|---:|
| SIG-DFS-001 | 7B,7H,7N,7P,7R,7Y | 6 | M10 | E3 | 5 |
| SIG-ATLAS-117 | 7B,7H,7N,7P,7R,7Y | 6 | M10 | E3 | 3 |
| SIG-ATLAS-105 | 7B,7H,7N,7P,7R,7Y | 6 | M7 | E1 | 1 |
| SIG-N6-BERN-001 | 7B,7H,7P,7R | 4 | M10 | E3 | 16 |
| SIG-DFS-206 | 7B,7H,7N,7R | 4 | M10 | E3 | 8 |
| SIG-META-001 | 7B,7H,7P,7R | 4 | M10* | EC | 5 |
| SIG-DFS-207 | 7B,7H,7N,7R | 4 | M10 | E3 | 4 |

## 3. [M10*] Promotion Candidates (>=5-BT)

SIG-DFS-001 (M10/E3, witness 5): DFS 22~26 cumulative tight = 348, seven-problem 0/7 honestly maintained.
SIG-ATLAS-117 (M10/E3, witness 3): 151 canonical bridges 38 distinct labels -- seven-problem domains 84 (55.6%).
SIG-ATLAS-105 (M7/E1, witness 1): seven-problem 60 alien_index concentration @C grade [0.10*] 12 nodes.

## 4. Comparison

Existing megahub SIG-META-001 (4-BT) detected as expected. 6 new >=4-BT candidates require further verification before [M10*] promotion.

## 5. Next Steps

Staging via witness >=3 + cross_repo >=1 confirmation before [M10*] promotion. Script: scripts/promote_signal_to_atlas.py --dry-run. Current atlas.signals.n6 dominated by millennium area -- expand AN/CROSS collection.
