# Nexus drill Darwin reach — workaround documented

**source**: `reports/n6_blockers.json` blk-1 (cross_repo atlas_gap, priority 95)
**roi-item**: `n6-roi-007` — auto-promoted todo → running via `tool/_n6_lib
promote_cross_repo` (n6a-20260422-004 convention, priority_floor=95).
**status**: resolved via local-fallback env toggle.

## Symptom

```
$ ~/.hx/bin/nexus drill --help
hexa resolver: ⚠ heavy-compute (...run.hexa drill) remote dispatch failed (exit 64)
              — local Mac execution risks stage0 SIGKILL (Gatekeeper + 4GB RSS cap)
NEXUS_REMOTE_DOWNGRADE {"heavy_compute":true,"cmd":"drill","hosts_tried":[],
  "reason":"remote_unavailable","fallback":"abort"}
```

Hexa's resolver declines local execution of `heavy_compute` commands on macOS
because Gatekeeper + the 4 GB RSS cap can SIGKILL stage0. When the remote
host is unreachable, `drill` aborts instead of running.

## Fix

Set `HEXA_ALLOW_LOCAL_FALLBACK=1`. With that env, the resolver downgrades
the abort to a local execution:

```
$ HEXA_ALLOW_LOCAL_FALLBACK=1 ~/.hx/bin/nexus drill --help
NEXUS_REMOTE_DOWNGRADE {"heavy_compute":true,"cmd":"drill","hosts_tried":[],
  "reason":"remote_unavailable","fallback":"local"}
hexa resolver: remote dispatch returned 64, running locally
nexus drill: --seed required — empty. Example: nexus drill --seed "P=NP barrier"
```

CLI is now responsive; real runs still need to be careful about seed
size (a small seed stays well under the 4 GB cap).

## Scanner wiring

`tool/_n6_lib scan_blockers` now probes both modes:

1. Try `nexus drill --help` remote (default) — `nexus_mode = remote`.
2. On failure, retry with `HEXA_ALLOW_LOCAL_FALLBACK=1` — if that
   succeeds, report `nexus_mode = local_fallback` and do **not** count
   the finding as a `med` blocker or a `cross_repo_blocker_count`.
3. If both fail, keep the previous behaviour: `nexus_mode = unreachable`,
   `severity_med += 1`, `cross_repo_high += 1`.

Reflected in `reports/n6_blockers.json.summary.nexus_reach` (bool) and
`reports/n6_blockers.json.summary.nexus_mode` (`remote` | `local_fallback`
| `unreachable`).

## Follow-up (deferred)

Rebuilding hexa_v2 on Linux (so stage0 doesn't need the Gatekeeper
workaround) is the long-term fix. Tracked as `anima-20260422-003` /
`nxs-20260422-006` — both already in the inbox. No new proposal needed
from n6-architecture side.
