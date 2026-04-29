# RETIRED — `scripts/rcsb_cluster_split_fetch.py`

**Retire date**: 2026-04-29 (cycle 30 wave 2)
**Per**: raw 9 hexa-only mandate + raw 102 ADD-new path (one-shot fetcher,
external callers = 0, output committed)

## Original purpose

HEXA-WEAVE MVP W5 cluster-split RCSB PDB fetcher. One-shot CLI that
downloaded 6L1U-class structures via RCSB REST API, applied cluster-split
sampling, and emitted JSON to a target directory.

## Why retired

- **External callers = 0**: full repo grep for `rcsb_cluster_split_fetch`
  returns only the file itself (self-cite in docstring + rerun-hint).
- **One-shot semantic**: the script ran once during HEXA-WEAVE MVP W5 cycle
  (commit `5fd60d4d` "feat(hexa-weave): cycle 15 5-agent fan-out … W5
  scripts").
- **Output paths external**: target directory was passed via `--out`; the
  resulting JSON shards are not under git tracking but their consumption
  was completed at cycle close.
- **No CI / .own / workflow reference**: zero downstream automation.

## Recovery

```bash
git show 5fd60d4d:scripts/rcsb_cluster_split_fetch.py > scripts/rcsb_cluster_split_fetch.py
```

The original script remains accessible via git history. If a future cycle
needs the cluster-split sampling logic, port the irreducible RCSB API
fetch step via hexa `exec("curl …")` + `python3 -c` JSON parse shim
(reference: `scripts/empirical/lmfdb_cremona_fetch.hexa` for the analog
LMFDB pattern).

## Cross-reference

- raw 9 hexa-only mandate (n6-architecture /.raw)
- raw 91 honest C3 disclosure
- raw 102 ADD-new path (RETIRE option: one-shot artifact, output
  committed)
- HEXA-WEAVE MVP W5 cycle (commit `5fd60d4d`)
