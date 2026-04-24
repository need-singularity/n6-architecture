# Nexus drill on macOS — local fallback

Operational note for running `nexus drill` on Darwin when the remote
heavy-compute host is unreachable.

## Symptom

`~/.hx/bin/nexus drill --help` prints
`NEXUS_REMOTE_DOWNGRADE ... "fallback":"abort"` and exits non-zero.
Hexa's resolver refuses to run `heavy_compute` commands locally on macOS
because Gatekeeper plus the 4 GB RSS cap can SIGKILL stage0.

## Env toggle

Set `HEXA_ALLOW_LOCAL_FALLBACK=1` to downgrade the abort to a local run:

```
HEXA_ALLOW_LOCAL_FALLBACK=1 ~/.hx/bin/nexus drill --seed "…"
```

The resolver then logs `"fallback":"local"` and executes in-process.

## When to use local fallback vs. wait for remote

- **Use local fallback** for `--help`, dry-runs, or small seeds well
  under the 4 GB cap — unblocks CLI checks and scanner probes.
- **Wait for remote** for real drills with non-trivial seeds, long
  traces, or anything that could push RSS past ~3 GB. Remote hosts do
  not have the Gatekeeper cap.

## Risk

Stage0 can be SIGKILL'd above ~4 GB RSS on Darwin (Gatekeeper + RSS cap).
Local fallback is a stopgap, not a replacement for the remote worker.
The long-term fix is rebuilding `hexa_v2` on Linux (tracked as
`anima-20260422-003` / `nxs-20260422-006`).

## References

- Promoted from: [`proposals/nexus_drill_darwin_fallback_2026_04_23.md`](../../proposals/nexus_drill_darwin_fallback_2026_04_23.md)
- Scanner wiring: `tool/_n6_lib scan_blockers` (emits
  `reports/n6_blockers.json.summary.nexus_mode`).
