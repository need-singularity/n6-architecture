# Weekly Audit Automation System Design Proposal

> **Target project**: n6-architecture
> **Authored**: 2026-04-11
> **Owning axis**: `reports/sessions` + `reports/changelogs`
> **Design scope**: full-spectrum audit over the last 7 days -> auto report + changelog + snapshot

---

## 1. Background and goals

### 1.1 Background

Currently, the four axes `reports/sessions`, `reports/audits`, `reports/changelogs`, `reports/discovery` are **all written manually**.

- `reports/sessions/` only contains manual session reports such as `explosive-growth-2026-04-10.md`, `millennium-lemmas-2026-04-11.md`
- `reports/changelogs/` only contains per-feature changelogs such as `3d-map-changelog-2026-04-08.md`
- Weekly audit reports have **no unified format and no automation**

### 1.2 Goals

1. Auto-generate a full-spectrum audit for the **last 7 days**
2. **English report** (includes box-drawing tables)
3. **Idempotent execution** (safe on repeat runs, originals preserved)
4. **HEXA-FIRST** — new scripts only in `.hexa`
5. **R14 compliance** — reference shared SSOT directly, no hardcoding
6. Auto-scaffold **changelog template**
7. Weekly auto-run possible via cron/hook

---

## 2. Aggregation data sources

| # | Data source | Path | Aggregation |
|---|------------|------|----------|
| 1 | **git log** | Local `.git` | `git log --since='7 days ago'` — commits/authors/files/9-axis distribution |
| 2 | **growth_bus.jsonl** | `n6shared/growth_bus.jsonl` | awk on `ts` field cutoff, line count + type/domain TOP |
| 3 | **discovery_graph.json** | `n6shared/discovery_graph.json` | Approximate nodes/edges via `"id":` / `"from":` frequency -> snapshot delta |
| 4 | **atlas.n6** | `$NEXUS/shared/n6/atlas.n6` | grep `[10*]`, `[10]`, `[9]`, `[7]`, `[N?]`, `[N!]` grade aggregation |
| 5 | **cargo test** | `nexus/` workspace | `timeout 180 cargo test --quiet --offline` then parse `test result: ok. X passed` |
| 6 | **convergence** | `n6shared/convergence/n6-architecture.json` | awk to count keys in `ossified`/`stable`/`failed` blocks |

### 2.1 Snapshot-based delta

- `n6shared/logs/weekly_audit_state.json` — metrics stored from previous run
- Fields: `dg_nodes`, `dg_edges`, `atlas_exact`, `atlas_empirical`, `gb_total`, `updated`
- Updated every run -> baseline for next weekly delta

---

## 3. Output artifact structure

### 3.1 Weekly audit report

**Path**: `reports/sessions/{YYYY-MM-DD}-weekly-audit.md`

**Format summary**:
```
# n6-architecture Weekly Audit Report — 2026-04-11

## Summary snapshot (box-drawing)
## 1. Git activity (commits/authors/files/9-axis distribution/recent 12 commits)
## 2. growth_bus.jsonl (type TOP / domain TOP)
## 3. discovery_graph.json (node/edge delta)
## 4. atlas.n6 grade aggregation
## 5. cargo test results
## 6. convergence state
## 7. Recommended follow-up actions (auto rule-based)
```

### 3.2 Changelog template

**Path**: `reports/changelogs/{YYYY-MM-DD}-weekly.md`

- Auto-aggregated block: numeric fields filled in
- Manually completed block: empty sections for `added/modified/deleted/converged/next-week TODO`

### 3.3 Snapshot JSON

**Path**: `n6shared/logs/weekly_audit_state.json`

```json
{
  "updated": "2026-04-11",
  "dg_nodes": 342,
  "dg_edges": 1205,
  "atlas_exact": 1542,
  "atlas_empirical": 38,
  "gb_total": 18425,
  "since_days": 7
}
```

### 3.4 Template originals (newly created)

- `reports/templates/weekly-audit-template.md`
- `reports/templates/changelog-template.md`

---

## 4. Idempotency design

| Scenario | Behavior |
|---------|------|
| First run | Creates `{DATE}-weekly-audit.md` |
| Same-day re-run (default) | Creates `{DATE}-weekly-audit_rerun-HHMM.md` (original preserved) |
| `--force` specified | Overwrites original |
| `--dry-run` | stdout only, no file write |
| Snapshot JSON | Updated to latest values every run (always) |

**Safety guards**:
- R19 SILENT EXIT prohibited -> all `exec()` errors printed via `log()` to stderr
- R21 no blocking -> `cargo test` wrapped with `timeout 180`
- R22 interpreter path -> `bash` only, no python/node invocation

---

## 5. Script structure (`nexus/scripts/weekly_audit.hexa`)

```
main
├── CLI parsing (--dry-run, --force, --no-cargo, --since N)
├── phase 1/6  git log aggregation
├── phase 2/6  growth_bus.jsonl scan (awk cutoff)
├── phase 3/6  discovery_graph.json delta (vs snapshot)
├── phase 4/6  atlas.n6 grade aggregation (grep)
├── phase 5/6  cargo test (optional)
├── phase 6/6  convergence state aggregation
├── render      assemble md body (box-drawing tables + sections)
├── write       reports/sessions/*.md + changelogs/*.md + state.json
└── stdout summary output
```

**Total lines**: about 600 (mid-range vs prior HEXA porting efforts)

---

## 6. Execution

### 6.1 Manual execution

```sh
cd $N6_ARCH
hexa nexus/scripts/weekly_audit.hexa                   # this week's audit
hexa nexus/scripts/weekly_audit.hexa --dry-run         # preview without writing
hexa nexus/scripts/weekly_audit.hexa --force           # overwrite
hexa nexus/scripts/weekly_audit.hexa --since 14        # 14-day range
hexa nexus/scripts/weekly_audit.hexa --no-cargo        # skip cargo test
```

### 6.2 cron registration (every Monday 09:00)

```sh
# ~/.crontab or crontab -e
0 9 * * 1 cd $N6_ARCH && $HEXA_LANG/hexa nexus/scripts/weekly_audit.hexa >> n6shared/logs/weekly_audit_cron.log 2>&1
```

### 6.3 macOS launchd (recommended — cron alternative)

`~/Library/LaunchAgents/com.n6arch.weekly-audit.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.n6arch.weekly-audit</string>
    <key>ProgramArguments</key>
    <array>
        <string>/bin/bash</string>
        <string>-c</string>
        <string>cd $N6_ARCH && $HEXA_LANG/hexa nexus/scripts/weekly_audit.hexa</string>
    </array>
    <key>StartCalendarInterval</key>
    <dict>
        <key>Weekday</key>
        <integer>1</integer>
        <key>Hour</key>
        <integer>9</integer>
        <key>Minute</key>
        <integer>0</integer>
    </dict>
    <key>StandardOutPath</key>
    <string>$N6_ARCH/n6shared/logs/weekly_audit_launchd.log</string>
    <key>StandardErrorPath</key>
    <string>$N6_ARCH/n6shared/logs/weekly_audit_launchd.err</string>
</dict>
</plist>
```

Load:
```sh
launchctl load ~/Library/LaunchAgents/com.n6arch.weekly-audit.plist
launchctl list | grep weekly-audit
```

### 6.4 Claude Code hook registration (optional — SessionStart etc.)

Add to `.claude/settings.json` (in parallel with the existing `UserPromptSubmit` block):

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "cd $N6_ARCH && $HEXA_LANG/hexa nexus/scripts/weekly_audit.hexa --dry-run 2>&1 | head -30",
            "timeout": 15
          }
        ]
      }
    ]
  }
}
```

**Note**: `.claude/settings.json` may be an L0 protected target — verify `core-lockdown.json` and obtain user approval before direct edit (R25 compliance).

**Alternative**: Register at project-local `n6shared/config/` level (L2 free to modify)
- Create `n6shared/config/weekly_audit_schedule.json`
- Have the `nexus` daemon read that config and execute periodically (extend nexus_growth_daemon.hexa)

---

## 7. Rule compliance checklist

| Rule | Compliance | Note |
|------|----------|------|
| R1 HEXA-FIRST | OK | Only `.hexa` newly created |
| R2 no hardcoding | OK | Paths relative, constants as variables |
| R5 SSOT | OK | References `shared/` originals, no duplicates created |
| R6 auto-recording | OK | Aggregation results persisted to `n6shared/logs/weekly_audit_state.json` |
| R8 no local data | OK | `n6shared/logs/` is in the allowed range (integrated inside nexus) |
| R14 common JSON | OK | Rules only reference existing `absolute_rules.json` |
| R18 minimal | OK | Focused on audit aggregation only, no feature additions |
| R19 no SILENT EXIT | OK | All errors printed via `log()` to stderr |
| R21 no blocking | OK | `cargo test` timeout 180s |
| R22 interpreter path | OK | `bash` only, no python/node invocation |
| R24 no new .sh/.py in n6shared/hooks | OK | This script is created in `nexus/scripts/` |
| English required | OK | All output/comments in English |

---

## 8. Expansion roadmap

### Phase 1 (immediate, scope of this proposal)
- [x] Write `nexus/scripts/weekly_audit.hexa`
- [x] `reports/templates/weekly-audit-template.md` + `changelog-template.md`
- [x] `reports/audits/auto-audit-system-proposal.md` (this doc)

### Phase 2 (short-term — within 1 week)
- [ ] First run (`hexa nexus/scripts/weekly_audit.hexa --dry-run`) verification
- [ ] Register cron or launchd
- [ ] Verify consistency with existing session report patterns

### Phase 3 (mid-term — 1 month)
- [ ] Detailed atlas promotion tracking (record each [7]->[10*] item)
- [ ] Add paper-count change on the papers/ axis
- [ ] Integrate NEXUS-6 scan anomalies (nexus scan --full results)
- [ ] Add monthly audit (`monthly_audit.hexa`)

### Phase 4 (long-term)
- [ ] Auto-update `n6shared/convergence/n6-architecture.json` based on audit results (detect stable->ossified transitions)
- [ ] Cross-project audit (TECS-L, anima, sedi, papers) via the nexus hub

---

## 9. Risks and mitigations

| Risk | Impact | Mitigation |
|-------|------|------|
| `atlas.n6` grep approximation error | grade counts off by a few | Introduce exact section parser in Phase 3 |
| `cargo test` exceeds 180s | Audit stalls | `--no-cargo` option + timeout wrapping |
| `discovery_graph.json` structure change | Node count inaccurate | Replace with proper JSON parser in Phase 3 |
| First-run delta = current absolute | Initial noise | Document explicitly + normalize from 2nd run |
| `n6shared/logs/weekly_audit_state.json` corruption | Delta = 0 | Document `--force` re-run + manual recovery |

---

## 10. Conclusion

This proposal automates the weekly audit of the n6-architecture project with a **single HEXA script** called `nexus/scripts/weekly_audit.hexa`. It aggregates 6 data sources (git/growth_bus/discovery_graph/atlas.n6/cargo/convergence) **idempotently and safely** to produce three outputs: English report + changelog template + snapshot JSON.

The execution mode can be adopted incrementally in the order **manual > cron > launchd > Claude hook**, and all stages can be safely validated with `--dry-run`.

It complies with all absolute rules including R1 (HEXA-FIRST), R14 (shared SSOT), R19 (no SILENT EXIT), R21 (no blocking), and English-required.

---

*This proposal belongs to the `reports/audits` axis and may progress to Phase 2 (execution verification) after approval.*
