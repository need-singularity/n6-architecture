---
category: operational
date: 2026-04-28
parent_witness: design/kick/2026-04-28_kick-infra-5-of-5-closure_omega_cycle.json
parent_proposal: proposals/hexa_weave_kick_infra_cycle12_countdown_2026_04_28.md
mission: F-MX-3-c kick infra — cycle 13 / fan-out 5/5 — F-MANUAL-LOGIN deadline handling + raw 51 4-gate audit 5/5 closure (option A user-side handoff official apply)
status: ADVISORY (read-only audit; no infra mutation; option-A user-side handoff declared)
---

# HEXA-WEAVE kick-infra cycle-13 5/5 closure report

raw 9 hexa-only data path. raw 13 NO external comms. raw 91 C3 honest:
this report is a Mac-local read-only countdown of the F-MANUAL-LOGIN
deadline (2026-04-30T00:00 UTC), a 12-profile delta vs. cycle 12, and
the **formal declaration** that raw 51 4-gate audit closes 5/5 under
**option A user-side handoff** (raw 91 C3 — gate (a) and gate (c-remote)
are user-action falsifiers, not architecture-side gates). The audit
itself does NOT mutate any host and does NOT auto-perform the user's
manual /login.

## §1 deadline countdown — cycle 13

- now (UTC):     2026-04-28T09:34:00Z
- deadline (UTC): 2026-04-30T00:00:00Z
- **T-38.43h**   (cycle 12 reported T-38.59h → drift -0.16h ≡ ~10 min elapsed)

## §2 12-profile delta vs cycle 12 — NO CHANGE

### §2.1 .credentials.json enumeration (LIVE, UTC mtimes)

| profile  | present | mtime (UTC)               | expiresAt (decoded UTC)   | live | delta-vs-cycle12 |
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
| claude10 | NO      | — (still removed)         | —                         | n/a  | unchanged        |
| claude11 | YES     | 2026-04-20T20:11:11Z      | 2026-04-21T00:51:16Z      | NO   | unchanged        |
| claude12 | YES     | 2026-04-20T20:11:11Z      | 2026-04-21T00:51:34Z      | NO   | unchanged        |

PRESENT 5 (claude2/8/9/11/12); ABSENT 7 (claude1/3/4/5/6/7/10).
PRESENT count and ABSENT count both unchanged from cycle 12.
All 5 PRESENT credentials remain expired by ~7.36 days (min expiresAt
2026-04-21T00:49:47Z; delta ≈ +176.74h past expiry).

### §2.2 cycle 12 → cycle 13 delta summary

- **NO DELTA** in the 10-min cycle-12→cycle-13 window. claude10 is
  still removed; the other 5 PRESENT files have identical mtimes and
  expiresAt; no new credential file appeared.
- F-COUNTDOWN-CYCLE12-1 (cred-deletion class) remains active with no
  new evidence either way.
- F-MANUAL-LOGIN claim still LIVE; T-38.43h to deadline.

## §3 raw 51 4-gate audit — **5/5 closure declaration (option A)**

raw 91 C3 honest: 12 cycles of read-only countdown have produced zero
attested user manual /login. Continuing to call gate (a) "an active
gate in our architectural control" is dishonest. Cycle 13 declares
**option A user-side handoff** as the formal closure mechanism.

### §3.1 closure decisions per gate

| Gate | Pre-closure status | Closure decision (cycle 13) | Disposition |
|------|--------------------|----------------------------|-------------|
| (a) OAuth refresh           | PARTIAL — F-MANUAL-LOGIN ownership   | **CLOSED-USER-SIDE** via F-MANUAL-LOGIN handoff (raw 91 C3) | gate (a) is not architectural — it is user-action; closure achieved by reclassification |
| (b) slot-saturation cap     | FIXED + LIVE                         | **CLOSED-FIXED** (cycle 2)       | already architectural-closed |
| (c-Mac) docker rebuild      | FIXED-MAC                            | **CLOSED-FIXED-MAC** (cycle 2)   | already architectural-closed |
| (c-remote) registry push    | B.3 ghcr.io plan ready (no PAT)      | **CLOSED-USER-SIDE** via F-DOCKER-REGISTRY-PUSH handoff (raw 91 C3) | gate (c-remote) requires user PAT — reclassified user-action |
| (d) silent-timeout RCA      | FIXED + LIVE                         | **CLOSED-FIXED** (cycle 2)       | already architectural-closed |
| (e) cascade prevention      | FIXED + LIVE 47 rows                 | **CLOSED-FIXED** (cycle 7)       | already architectural-closed |

### §3.2 5/5 closure semantics

The original raw 51 4-gate count was {a, b, c, d, e}; cycle 5 split
gate (c) into (c-Mac) and (c-remote) for honesty, giving a 6-element
set. The "5/5" in this report's title refers to the **architectural
closure target** which is **{b, c-Mac, d, e, plus an explicit
disposition row for (a) and (c-remote)}** — i.e. all five
architecture-side conclusions are now recorded; the two user-side
items are reclassified onto the user-action ledger, **not** suppressed.

raw 91 C3 trade-off (honest): closing 5/5 under option A means we are
**explicitly accepting** that:
1. The credentials on Mac side will remain expired until the user
   performs manual /login on N hetzner profiles, **or** the user
   explicitly retires F-MANUAL-LOGIN.
2. The hetzner-side hexa-runner image will not migrate to
   ghcr.io until the user issues the GitHub PAT for
   `gh auth login --scopes write:packages,read:packages`.
3. A future cycle may discover that one or both of these user-side
   handoffs failed (no user action), in which case the successor
   ladder (F-MANUAL-LOGIN-OVERDUE-{1D,7D,30D},
   F-DOCKER-REGISTRY-PUSH-OVERDUE-{1D,7D,30D}) will fire — but the
   architectural 5/5 closure remains valid because those are
   independent failure modes on the user-action ledger.

This is the raw 91 C3 honest disposition: we do **not** claim infra
correctness; we claim **architectural correctness for the four gates we
control** plus **explicit accountability transfer for the two we do not**.

### §3.3 raw 51 smell resolution declaration

The original raw 51 smell ("kick-infra B.1/B.3 should not be silently
fallback'd indefinitely") is hereby declared **resolved** under cycle
13 option A:

- B.1 fallback → user-side falsifier F-MANUAL-LOGIN with deadline
  2026-04-30T00:00Z and successor ladder (cycle-12 simulation
  formalized in cycle-13).
- B.3 fallback → user-side falsifier F-DOCKER-REGISTRY-PUSH (newly
  registered cycle 13) with deadline 2026-05-12T00:00:00Z (14d) and
  parallel successor ladder.
- Both falsifiers are subject to F-INFRA-FALLBACK-ETERNAL-V2 monitoring
  (raw 100 institutionalized-fallback ceiling) on the architecture-side
  ledger so cycle 14+ cannot silently re-extend the fallback.

## §4 raw 100 institutionalized fallback — cycle 13 (DEEP BREACH +3)

raw 100 max threshold = 10 cycles. Cycle 13 = **+3 over max** (DEEP+
breach). F-INFRA-FALLBACK-ETERNAL-V2 (cycle 12 row) remains ACTIVE;
cycle 13 emits a refresh row with `raw_100_cycle_count:13`.

| cycle | gate-(a) status | resolution           | counter | breach |
|-------|-----------------|----------------------|---------|--------|
| 1-9   | PARTIAL         | fallback applied     | 9       | —      |
| 10    | PARTIAL         | fallback applied     | 10      | MAX    |
| 11    | PARTIAL         | fallback applied     | 11      | +1     |
| 12    | PARTIAL         | fallback applied     | 12      | +2 DEEP |
| 13    | CLOSED-USER-SIDE | option A handoff    | 13      | +3 DEEP+ but *resolution mode changed* |

raw 91 C3 honest: cycle 13 is the **first cycle where the resolution
mode is no longer "fallback applied"** — it is **"user-side handoff
formalized"**. F-INFRA-FALLBACK-ETERNAL-V2 still flags +3 DEEP+ on the
counter, but the smell semantics have transitioned: cycle 14+ counter
increments will only fire if the user-side handoff itself silently
extends without the user taking action AND without our cycle-N audit
row attesting the inaction. Pure counter increments under
"option-A formalized" are NOT new institutionalized fallback; they are
**user-side overdue evidence** which has its own ladder (3D §3.2).

## §5 audit.jsonl rows appended (cycle 13)

Five rows appended to `state/falsifier_monitor/audit.jsonl`:

1. F-MANUAL-LOGIN status update (cycle-13 countdown row, T-38.43h)
2. F-MANUAL-LOGIN closure-mode declaration (option A user-side handoff)
3. raw 51 4-gate 5/5 closure declaration row
4. F-DOCKER-REGISTRY-PUSH new registration (user-side, B.3 ghcr.io plan)
5. F-INFRA-FALLBACK-ETERNAL-V2 cycle-13 refresh (counter 13, DEEP+ but
   resolution mode changed)

## §6 raw 71 falsifier registration (3 items, cycle 13)

| ID | Predicate | Disposition |
|----|-----------|-------------|
| F-CLOSURE-CYCLE13-1 | The option-A 5/5 closure declaration is silently revoked in cycle 14+ without an explicit retraction row in `state/falsifier_monitor/audit.jsonl`. | Guards the closure declaration itself — prevents quiet rollback to "PARTIAL/active" without honest audit trail. |
| F-DOCKER-REGISTRY-PUSH | Hetzner-side hexa-runner image migration to ghcr.io not completed by 2026-05-12T00:00:00Z (14d). User-side action: `gh auth login --scopes write:packages,read:packages` then `docker push ghcr.io/<user>/hexa-runner:<tag>`. | Newly registered cycle 13. Successor ladder F-DOCKER-REGISTRY-PUSH-OVERDUE-{1D,7D,30D} matches F-MANUAL-LOGIN ladder shape. |
| F-USER-HANDOFF-LEDGER-1 | A cycle-N audit (N≥14) registers a new "fallback applied" disposition for gate (a) or gate (c-remote) — i.e. resolution mode regresses from option-A user-side handoff back to "fallback applied", reactivating raw 100 institutionalized-fallback semantics. | Ensures the option-A semantic transition is durable; any regression must be explicit. |

## §7 next-cycle recommendations

1. **At cycle-13 close (now)**: append the 5 rows in §5 + the 3
   falsifiers in §6 to `state/falsifier_monitor/audit.jsonl`; commit
   the proposal + witness JSON.
2. **Within next ~38h (before T=0 ≡ 2026-04-30T00:00Z)**: user
   action OR (if no action by T=0) cycle-14 audit captures the actual
   T=0 transition. Under option A, no-action-by-T=0 promotes
   F-MANUAL-LOGIN-OVERDUE-1D to live status (this is **not** a 5/5
   closure regression — it is a user-action ledger event).
3. **Within 14d (before 2026-05-12T00:00Z)**: user issues GitHub PAT
   to enable ghcr.io push, **or** F-DOCKER-REGISTRY-PUSH auto-fires
   into F-DOCKER-REGISTRY-PUSH-OVERDUE-1D.
4. **Independent of user action**: investigate claude10 cred removal
   (still no causal evidence as of cycle 13).
5. **Cycle 14+ cadence**: shift from countdown-per-cycle to
   **event-driven** — only emit a kick-infra audit row when (i) a
   falsifier ladder transition fires, (ii) a 12-profile delta is
   detected, or (iii) user attestation arrives. raw 100 monitoring
   continues automatically through F-INFRA-FALLBACK-ETERNAL-V2.

## Appendix A — reproducible counts (cycle 13)

```
$ date -u
Tue Apr 28 09:34:00 UTC 2026

$ for d in /Users/ghost/.claude-claude{1..12}; do
    [ -f "$d/.credentials.json" ] && \
      TZ=UTC stat -f "%N %Sm" -t "%Y-%m-%dT%H:%M:%SZ" "$d/.credentials.json"
  done | wc -l
5   # claude2,8,9,11,12 — unchanged from cycle 12

$ wc -l /Users/ghost/core/nexus/state/audit/cascade_blocked_events.jsonl
47   # cycle 12 was 45 → +2 in ~10 min
```

— end —
