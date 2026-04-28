---
category: operational
date: 2026-04-28
parent_witness: design/kick/2026-04-28_kick-infra-cycle12-countdown_omega_cycle.json
parent_proposal: proposals/hexa_weave_kick_infra_cycle11_countdown_2026_04_28.md
mission: F-MX-3-c kick infra — cycle 12 / fan-out 4/5 — F-MANUAL-LOGIN T-X countdown + auto-fire simulation + raw 51 4-gate progress + raw 100 fallback institutionalized (12-cycle deep breach)
status: ADVISORY (read-only audit; no infra mutation; user manual /login NOT automated; auto-fire simulated only)
---

# HEXA-WEAVE kick-infra cycle-12 countdown report

raw 9 hexa-only data path. raw 13 NO external comms. raw 91 C3 honest:
this report is a Mac-local read-only countdown of the F-MANUAL-LOGIN
deadline (2026-04-29T24:00 UTC ≡ 2026-04-30T00:00 UTC), a 12-profile
delta vs. cycle 11, and a documented (non-executed) auto-fire
simulation. It does NOT mutate any host and does NOT auto-perform the
user's manual /login.

## §1 deadline countdown

- now (UTC):     2026-04-28T09:24:51Z
- deadline (UTC): 2026-04-30T00:00:00Z (≡ 2026-04-29T24:00:00Z)
- **T-38.59h**   (cycle 11 reported T-38.84h → drift -0.25h ≡ ~15 min elapsed)

## §2 12-profile delta vs cycle 11 — REAL CHANGE DETECTED

### §2.1 .credentials.json enumeration (LIVE, UTC mtimes)

| profile  | present | mtime (UTC)               | expiresAt (decoded UTC)   | live | delta-vs-cycle11 |
|----------|---------|---------------------------|---------------------------|------|------------------|
| claude1  | NO      | —                         | —                         | n/a  | unchanged        |
| claude2  | YES     | 2026-04-20T20:11:11Z      | 2026-04-21T00:49:47Z      | NO   | unchanged        |
| claude3  | NO      | —                         | —                         | n/a  | unchanged        |
| claude4  | NO      | —                         | —                         | n/a  | unchanged        |
| claude5  | NO      | —                         | —                         | n/a  | unchanged        |
| claude6  | NO      | —                         | —                         | n/a  | unchanged        |
| claude7  | NO      | —                         | —                         | n/a  | unchanged        |
| claude8  | YES     | 2026-04-20T16:51:00Z      | 2026-04-21T00:50:26Z      | NO   | unchanged        |
| claude9  | YES     | 2026-04-20T20:11:12Z      | 2026-04-21T00:50:43Z      | NO   | unchanged        |
| claude10 | **NO**  | — (REMOVED)               | — (REMOVED)               | n/a  | **PRESENT→ABSENT** |
| claude11 | YES     | 2026-04-20T20:11:11Z      | 2026-04-21T00:51:16Z      | NO   | unchanged        |
| claude12 | YES     | 2026-04-20T20:11:11Z      | 2026-04-21T00:51:34Z      | NO   | unchanged        |

### §2.2 cycle 11 → cycle 12 delta summary

- **DELTA**: `claude10/.credentials.json` removed between cycle 11
  (09:09:19Z) and cycle 12 (09:24:51Z). PRESENT count 6→5; ABSENT count
  6→7. The remainder of `~/.claude-claude10/` (sessions, history, etc.)
  is intact — only `.credentials.json` is gone.
- **Other 5 PRESENT (claude2/8/9/11/12)**: mtime + expiresAt identical
  to cycle 11. NO drift.
- claude3 hetzner-side liveness OUT OF SCOPE here (raw 13 / read-only
  Mac audit). No new evidence either way.
- All 5 PRESENT credentials remain expired by ~7.36 days (now is
  2026-04-28T09:24:51Z; min expiresAt 2026-04-21T00:49:47Z; delta
  ≈ +176.58h past expiry).

### §2.3 raw 91 C3 honest — claude10 removal cause is UNKNOWN

The cycle-12 audit is read-only and has no causal evidence for the
claude10 deletion. Plausible candidates without manual /login attestation:

1. user manually deleted the file (housekeeping) — possible
2. external process (linter, cleanup script, hetzner-side mirror) — unknown
3. Claude harness itself rotated/cleaned an expired token — unverified

This advances **F-MAC-CRED-DRIFT-1** evidence: a `.credentials.json`
**deletion** is also a "background mutation without audit attestation"
(F-COUNTDOWN-CYCLE11-2 was scoped to mtime advance, but the broader
F-MAC-CRED-DRIFT-1 covers any unattested change). Cycle 12 registers
**F-COUNTDOWN-CYCLE12-1** to capture this specific class of event.

→ **F-MANUAL-LOGIN claim still LIVE**. NO manual `/login` activity has
been attested on Mac side in the cycle-11 → cycle-12 window (~15 min).

## §3 F-MANUAL-LOGIN auto-fire simulation (NOT executed)

The falsifier registry row in `state/falsifier_monitor/audit.jsonl`
remains:

```json
{"falsifier_id":"F-MANUAL-LOGIN","status":"active",
 "deadline":"2026-04-30T00:00:00Z","auto_check":"no",
 "alert_level":"warn"}
```

T-38.59h is **>0**, so the auto-fire transition is NOT triggered in
cycle 12. We simulate (i.e. document, do not execute) the transition
the auto-fire path would take on 2026-04-30T00:00:00Z.

### §3.1 simulated transition — T=0 (deadline reached)

Pseudocode of the simulation:

```
if now >= deadline:
    falsifier['F-MANUAL-LOGIN'].status = 'fired'
    falsifier['F-MANUAL-LOGIN'].fire_reason = 'deadline_exceeded'
    falsifier['F-MANUAL-LOGIN'].remediation = 'NONE'
    audit.append({ts: now, falsifier: 'F-MANUAL-LOGIN', status: 'fired',
                  successor: 'F-MANUAL-LOGIN-OVERDUE', ...})
    register('F-MANUAL-LOGIN-OVERDUE', deadline=now+30d, alert='alarm')
```

### §3.2 successor falsifier ladder (escalating deadlines)

If the user takes no action by 2026-04-30T00:00Z:

| Successor                      | Deadline        | Δ from F-MANUAL-LOGIN | Alert  |
|--------------------------------|-----------------|------------------------|--------|
| F-MANUAL-LOGIN-OVERDUE-1D      | 2026-05-01T00:00Z | +1d                  | alarm  |
| F-MANUAL-LOGIN-OVERDUE-7D      | 2026-05-07T00:00Z | +7d                  | alarm  |
| F-MANUAL-LOGIN-OVERDUE-30D     | 2026-05-30T00:00Z | +30d                 | error  |

Escalation rule: each successor's expiry without remediation triggers
the next-level successor (1d→7d→30d), with `alert_level` monotonically
non-decreasing. If 30d expires, the disposition forks:
(A) user-side handoff retire, (B) explicit escalation to architect-level
review, or (C) acceptance + raw-100 institutionalized record.

### §3.3 auto-fire simulation result row (cycle-12 — NOT live)

```
{ts:"2026-04-28T09:24:51Z", falsifier_id:"F-MANUAL-LOGIN",
 simulated_status:"fired", simulated_at:"2026-04-30T00:00:00Z",
 simulation_only:true, action_taken:"none (read-only countdown)",
 successor_ladder:["F-MANUAL-LOGIN-OVERDUE-1D",
                   "F-MANUAL-LOGIN-OVERDUE-7D",
                   "F-MANUAL-LOGIN-OVERDUE-30D"],
 cycle:"cycle-12/fan-out-4-of-5",
 schema:"raw_77_falsifier_monitor_v1"}
```

This row is appended to `state/falsifier_monitor/audit.jsonl` with
`simulated_status` (not `status`) so monitoring tooling does not treat
it as a live fire. Cycle 13+ will replace it with the real transition
row when (and only when) `now >= deadline`.

## §4 raw 51 4-gate audit — 5/5 closure progress

| Gate | cycle 11 | cycle 12 | 5/5 closure path |
|------|----------|----------|-------------------|
| (a) OAuth refresh           | PARTIAL — F-MANUAL-LOGIN ownership   | PARTIAL — Mac dormant 7.36 d, T-38.59 h, **claude10 cred removed** | (a) closes when EITHER (i) user manual /login completes + verification OR (ii) F-MANUAL-LOGIN retires by user raw 91 C3 user-side handoff. |
| (b) slot-saturation cap     | FIXED + LIVE                         | FIXED + LIVE (still 1 anomalous 13/12 row; no new) | already closed |
| (c) docker rebuild          | FIXED-MAC + ghcr.io 5-step plan      | FIXED-MAC + ghcr.io plan ready (no PAT) | (c-remote) closes when user issues ghcr.io PAT |
| (d) silent-timeout RCA      | FIXED + LIVE                         | FIXED + LIVE (no new silent-timeout rows) | already closed |
| (e) cascade prevention      | FIXED + LIVE 39 rows                 | FIXED + LIVE 45 rows (+6 in ~15 min)    | already closed |

**Current closure state**: 4/5 gates FIXED + LIVE. Gate (a) still
PARTIAL pending user manual action.

**5/5 closure feasibility within cycle 13+**: only if user takes one of
the three actions —
1. (i) **manual /login on N hetzner profiles** (the canonical close)
2. (ii) **explicit retirement of F-MANUAL-LOGIN** under raw 91 C3
   user-side handoff (recommended option A)
3. (iii) **register F-MANUAL-LOGIN-OVERDUE** at T=0 (option B);
   this keeps gate (a) open at PARTIAL but documents the breach via a
   live successor falsifier rather than a 12-cycle silent fallback.

**Recommendation**: Option A (user-side handoff retire). raw 91 C3 honest:
12 cycles of read-only countdown have produced zero attested user manual
/login. The honest disposition is to stop calling this an "active gate
in our control" and label it a permanent user-side responsibility,
allowing raw 51 to close 5/5 on the architectural axis (b/c-Mac/d/e)
and (a) to be tracked on a separate user-action ledger.

## §5 raw 100 fallback institutionalized — 12-cycle DEEP breach

raw 100: "if a 'fallback' becomes the steady-state path, that is itself
an architectural smell deserving raw 71 falsifier guard."

| cycle | gate-(a) status | resolution           | counter |
|-------|-----------------|----------------------|---------|
| 1-9   | PARTIAL         | fallback applied     | 9       |
| 10    | PARTIAL         | fallback applied     | 10 → MAX threshold |
| 11    | PARTIAL         | fallback applied     | 11 → BREACH |
| 12    | PARTIAL         | fallback applied     | **12 → DEEP BREACH (+2 over max)** |

raw 91 C3 honest: cycle 12 is the **second cycle past the raw-100
maximum threshold of 10**. F-INFRA-FALLBACK-ETERNAL (registered at
cycle 11 with `breach:true, raw_100_cycle_count:11`) is now stale and
must be re-emitted with `raw_100_cycle_count:12`.

The fact that we are still asking "what to do" in cycle 12 is itself
the meta-smell: a falsifier registered to detect the fallback has not
forced any disposition. The fix is to **bind F-INFRA-FALLBACK-ETERNAL
to a hard escalation deadline** (e.g. `2026-04-29T00:00:00Z`, i.e. ~14 h
from now, before F-MANUAL-LOGIN itself fires) so that the next
"continue countdown" reflex is itself blocked.

## §6 raw 71 falsifier registration (3 items, cycle 12)

| ID | Predicate | Disposition |
|----|-----------|-------------|
| F-COUNTDOWN-CYCLE12-1 | A `.credentials.json` among the cycle-11 PRESENT set is **deleted** without a corresponding cycle-N audit row attesting user action. (cycle 12 evidence: claude10 removed.) | Generalizes F-COUNTDOWN-CYCLE11-2 (mtime advance) to cover deletion. F-MAC-CRED-DRIFT-1 evidence advance. |
| F-COUNTDOWN-CYCLE12-2 | At T=0 (2026-04-30T00:00Z), `state/falsifier_monitor/audit.jsonl` has no row with `falsifier_id="F-MANUAL-LOGIN"` and `status="fired"` (or `status="retired"`). | Forces an explicit T=0 disposition; refines F-COUNTDOWN-CYCLE11-1 by including the retire path. |
| F-INFRA-FALLBACK-ETERNAL-V2 | The kick-infra audit reaches cycle 13+ with gate (a) still PARTIAL and F-MANUAL-LOGIN still active (neither fired nor retired). Hard deadline 2026-04-29T00:00:00Z. | Re-emits cycle-11 F-INFRA-FALLBACK-ETERNAL with updated `raw_100_cycle_count:12` and a binding 14h deadline so the institutionalized-fallback cannot silently extend to cycle 13. |

## §7 next-cycle recommendations

1. **At cycle-12 close (now)**: register the three falsifiers in §6 and
   write the simulated-fire row in §3.3 to `state/falsifier_monitor/audit.jsonl`.
2. **Within next 14 h (before 2026-04-29T00:00:00Z)**: user explicitly
   chooses option A (retire F-MANUAL-LOGIN under raw 91 C3 user-side
   handoff) **or** option B (acknowledge auto-fire path will trigger
   F-MANUAL-LOGIN-OVERDUE ladder at 2026-04-30T00:00Z).
3. **At cycle 13 (24-36 h from now)**: capture the actual T=0 transition.
   If F-MANUAL-LOGIN auto-fires, register F-MANUAL-LOGIN-OVERDUE-1D and
   close raw-100 breach by promoting the successor to live status.
4. **Independent of F-MANUAL-LOGIN**: investigate claude10 cred removal.
   The cycle-12 audit has zero causal evidence; user attestation or
   harness telemetry should be checked at cycle 13.

## Appendix A — reproducible counts (cycle 12)

```
$ date -u
Tue Apr 28 09:24:51 UTC 2026

$ for d in /Users/ghost/.claude-claude{1..12}; do
    [ -f "$d/.credentials.json" ] && \
      TZ=UTC stat -f "%N %Sm" -t "%Y-%m-%dT%H:%M:%SZ" "$d/.credentials.json"
  done | wc -l
5   # claude2,8,9,11,12  (cycle 11 was 6; claude10 removed)

$ wc -l /Users/ghost/core/nexus/state/audit/cascade_blocked_events.jsonl
45   # cycle 11 was 39 → +6 in ~15 min
```

— end —
