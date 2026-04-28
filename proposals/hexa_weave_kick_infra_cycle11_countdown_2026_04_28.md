---
category: operational
date: 2026-04-28
parent_witness: design/kick/2026-04-28_kick-infra-cycle11-countdown_omega_cycle.json
parent_proposal: proposals/hexa_weave_kick_infra_metric_2026_04_28.md
mission: F-MX-3-c kick infra — cycle 11 / fan-out 5/5 — F-MANUAL-LOGIN deadline countdown + auto-fire prep
status: ADVISORY (read-only audit; no infra mutation; user manual /login NOT automated)
---

# HEXA-WEAVE kick-infra cycle-11 countdown report

raw 9 hexa-only data path. raw 13 NO external comms. raw 91 C3 honest:
this report is a Mac-local read-only countdown of the F-MANUAL-LOGIN
deadline (2026-04-29T24:00 UTC ≡ 2026-04-30T00:00 UTC) and a 12-profile
delta vs. cycle 10. It does NOT mutate any host and does NOT auto-perform
the user's manual /login. raw 71 C3 honest: cycle 10 referenced filename
`auth.json`; the actual auth artifact on this host is `.credentials.json`.
The cycle-10 report itself enumerated `.credentials.json` correctly in §2.1;
cycle 11 retains that same filename for parity.

## §1 deadline countdown

- now (UTC):     2026-04-28T09:09:19Z
- deadline (UTC): 2026-04-30T00:00:00Z (≡ 2026-04-29T24:00:00Z)
- **T-38.84h**   (cycle 10 reported T-38.99 h → drift -0.15 h ≡ ~9 min elapsed)

## §2 12 profile delta vs cycle 10

### §2.1 .credentials.json enumeration (LIVE)

| profile  | present | mtime (filesystem)        | expiresAt (decoded UTC)  | live | delta-vs-cycle10 |
|----------|---------|---------------------------|--------------------------|------|------------------|
| claude1  | NO      | —                         | —                        | n/a  | unchanged        |
| claude2  | YES     | 2026-04-20T20:11:11Z      | 2026-04-21T00:49:47Z     | NO   | unchanged        |
| claude3  | NO      | —                         | —                        | n/a  | unchanged        |
| claude4  | NO      | —                         | —                        | n/a  | unchanged        |
| claude5  | NO      | —                         | —                        | n/a  | unchanged        |
| claude6  | NO      | —                         | —                        | n/a  | unchanged        |
| claude7  | NO      | —                         | —                        | n/a  | unchanged        |
| claude8  | YES     | 2026-04-20T16:51:00Z      | 2026-04-21T00:50:26Z     | NO   | unchanged        |
| claude9  | YES     | 2026-04-20T20:11:12Z      | 2026-04-21T00:50:43Z     | NO   | unchanged        |
| claude10 | YES     | 2026-04-20T16:52:01Z      | 2026-04-21T00:51:00Z     | NO   | unchanged        |
| claude11 | YES     | 2026-04-20T20:11:11Z      | 2026-04-21T00:51:16Z     | NO   | unchanged        |
| claude12 | YES     | 2026-04-20T20:11:11Z      | 2026-04-21T00:51:34Z     | NO   | unchanged        |

### §2.2 cycle 10 → cycle 11 delta summary

- 6 PRESENT (claude2/8/9/10/11/12): `.credentials.json` mtime **identical**
  to cycle 10. NO drift.
- 6 ABSENT (claude1/3/4/5/6/7): still no `.credentials.json`.
- claude3 (cycle 7 healthy via hetzner-side auto-refresh) Mac-side status
  **still ABSENT** — its hetzner-side liveness is OUT OF SCOPE here
  (raw 13 / read-only Mac audit).
- All 6 PRESENT credentials remain expired by ~7.35 days (now is
  2026-04-28T09:09:19Z; min expiresAt is 2026-04-21T00:49:47Z; delta
  ≈ +176.33 h past expiry).

→ **F-MANUAL-LOGIN claim still LIVE**. NO manual `/login` activity has
occurred on Mac side in the cycle-10 → cycle-11 window (~1 h).

## §3 F-MANUAL-LOGIN auto-fire prep

The falsifier registry row currently in
`state/falsifier_monitor/audit.jsonl` reads:

```json
{"falsifier_id":"F-MANUAL-LOGIN","status":"active",
 "deadline":"2026-04-29T24:00:00Z","auto_check":"no",
 "alert_level":"warn"}
```

### §3.1 auto-fire transition plan (executes only at deadline; NOT now)

When `now >= deadline (2026-04-30T00:00:00Z)`:

1. `tool/falsifier_monitor.hexa --status F-MANUAL-LOGIN` reads the deadline
   field, compares against `now`, and emits status transition
   `active → fired` (raw 71 fire path).
2. `state/falsifier_monitor/audit.jsonl` receives a row:
   ```
   {ts, falsifier_id:F-MANUAL-LOGIN, status:fired,
    fire_reason:"deadline_exceeded", remediation:"NONE",
    successor:"F-MANUAL-LOGIN-OVERDUE", schema:raw_77_falsifier_monitor_v1}
   ```
3. A successor falsifier `F-MANUAL-LOGIN-OVERDUE` is registered:
   ```
   {falsifier_id:F-MANUAL-LOGIN-OVERDUE, status:active,
    claim_summary:"11 hetzner profile manual /login still pending after deadline",
    deadline:"2026-05-29T24:00:00Z", days_to_deadline:30,
    predecessor:"F-MANUAL-LOGIN", alert_level:"alarm"}
   ```
4. **Alternative disposition**: if the user explicitly RETIRES the gate
   (i.e. permanently delegates manual /login to user-side responsibility
   per raw 91 C3), F-MANUAL-LOGIN transitions `active → retired` instead
   of `fired`, and **no successor is created**. Cycle 12+ would close
   the (a) gate of raw 51 4-gate via permanent user-side handoff.

### §3.2 cycle 11 NO-OP confirmation

Deadline is T-38.84 h away. **No fire action is taken in cycle 11.**
The audit is read-only; the registry row remains `active`. The auto-fire
plan above is documentation only.

## §4 raw 51 4-gate audit — 5/5 closure path forecast

| Gate | Status (cycle 10) | Status (cycle 11) | 5/5 closure path |
|------|-------------------|-------------------|-------------------|
| (a) OAuth refresh           | PARTIAL — F-MANUAL-LOGIN ownership   | PARTIAL — Mac dormant 7.35 d, T-38.84 h | (a) closes when EITHER (i) user manual /login completes + verification OR (ii) F-MANUAL-LOGIN retires by user raw 91 C3 user-side handoff. |
| (b) slot-saturation cap     | FIXED + LIVE (1 anomalous 13/12 row) | FIXED + LIVE (still 1 anomalous 13/12 row) | already closed |
| (c) docker rebuild          | FIXED-MAC + ghcr.io 5-step plan      | FIXED-MAC + ghcr.io plan ready (no PAT) | (c-remote) closes when user issues ghcr.io PAT + re-pulls on hetzner |
| (d) silent-timeout RCA      | FIXED + LIVE                         | FIXED + LIVE (no new silent-timeout rows) | already closed |
| (e) cascade prevention      | FIXED + LIVE 35 rows                 | FIXED + LIVE 39 rows (+4)               | already closed |

**Current closure state**: 4/5 gates FIXED + LIVE. Gate (a) still
PARTIAL pending user manual action. Gate (c) Mac-side FIXED; remote
(c) blocked on user ghcr.io PAT.

**5/5 closure feasibility within cycle 12+**: only if user takes one of
two actions (i) manual /login on 11 hetzner profiles or (ii) explicit
retirement of F-MANUAL-LOGIN. Otherwise the audit remains 4/5.

## §5 raw 100 fallback institutionalized — 10-cycle counter

raw 100: "if a 'fallback' becomes the steady-state path, that is itself
an architectural smell deserving raw 71 falsifier guard." This kick-infra
audit has now run 10 consecutive cycles (cycle 1 → cycle 10) WITHOUT
the (a) gate closing under user-side action, and cycle 11 is the 10th
cycle to register the same fallback pattern (i.e. read-only Mac-local
audit + raw 91 C3 honest "user TTY required").

| cycle | gate-(a) status | resolution | 10-cycle institutionalized counter |
|-------|-----------------|------------|------------------------------------|
| 1-9   | PARTIAL         | fallback applied | accumulated 9                      |
| 10    | PARTIAL         | fallback applied (kick infra metric report) | 10 → MAXIMUM smell threshold |
| 11    | PARTIAL         | fallback applied (this countdown report)    | 11 → BREACH (institutionalized > maximum) |

raw 91 C3 honest: cycle 11 has crossed the raw-100 maximum. Cycle 12
must either (i) reach 5/5 closure (requires user action), (ii) retire
the (a) gate via permanent user-side handoff, or (iii) raise an explicit
new raw 71 falsifier `F-INFRA-FALLBACK-ETERNAL` with a hard escalation
deadline. This proposal recommends path (iii) be considered if the
T-38.84 h F-MANUAL-LOGIN deadline expires without action.

## §6 raw 71 falsifier preregistration (3 items)

| ID | Predicate | Disposition |
|----|-----------|-------------|
| F-COUNTDOWN-CYCLE11-1 | At T=0 (2026-04-30T00:00Z), F-MANUAL-LOGIN registry row is still `active` and no fire row appears in `state/falsifier_monitor/audit.jsonl`. | Indicates the auto-fire path is not wired; manual `tool/falsifier_monitor.hexa --status F-MANUAL-LOGIN` invocation required. |
| F-COUNTDOWN-CYCLE11-2 | A `.credentials.json` mtime among the 6 PRESENT profiles advances WITHOUT a corresponding cycle-N audit row attesting user manual /login. | Echoes F-MAC-CRED-DRIFT-1 from cycle 10; cumulative evidence of background credential mutation. |
| F-INFRA-FALLBACK-ETERNAL | The kick-infra audit reaches cycle 12+ with gate (a) still PARTIAL and no successor falsifier replacing F-MANUAL-LOGIN. | raw 100 institutionalized-fallback breach; require explicit retire-or-escalate decision. |

## §7 next-cycle recommendations

1. cycle 12 must capture the actual `now >= deadline` transition; if the
   user has not acted, `tool/falsifier_monitor.hexa --status F-MANUAL-LOGIN`
   should fire and successor `F-MANUAL-LOGIN-OVERDUE` should register.
2. If still no user action, propose explicit retirement of the (a) gate
   under raw 91 C3 (permanent user-side handoff) so raw 51 4-gate audit
   can close 5/5 on the architectural axis (b)/(c-Mac)/(d)/(e) and (a) is
   moved to a separate user-action ledger.
3. The raw-100 institutionalized-fallback breach (cycle 11) warrants a
   one-off review: which other "fallbacks" in the n6 codebase have run
   ≥10 cycles? F-INFRA-FALLBACK-ETERNAL should be promoted to a registry
   row regardless of F-MANUAL-LOGIN disposition.

## Appendix A — reproducible counts

```
$ date -u
Tue Apr 28 09:09:19 UTC 2026

$ for d in /Users/ghost/.claude-claude{1..12}; do
    [ -f "$d/.credentials.json" ] && \
      stat -f "%N %Sm" -t "%Y-%m-%dT%H:%M:%SZ" "$d/.credentials.json"
  done | wc -l
6   # claude2,8,9,10,11,12

$ wc -l /Users/ghost/core/nexus/state/audit/cascade_blocked_events.jsonl
39   # cycle 10 was 35 → +4 in ~1 h
```

— end —
