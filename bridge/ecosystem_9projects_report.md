# 9-Project Autonomous Growth Ecosystem Integration Report

- Date: 2026-04-14
- Roadmap item: DSE-P3-3
- Project: n6-architecture
- SSOT: `$NEXUS/shared/config/projects.json` (schema v4)
- Deliverables: `bridge/ecosystem_9projects.hexa` (interface), this report

## 1. The 9 Projects

Within the projects.json categories, 7 core + 2 auxiliary projects make up the final 9.

| # | Project | Category | Role | Root | Roadmap |
|---|---|---|---|---|---|
| 1 | nexus | core | discovery-engine | `$HOME/Dev/nexus` | `shared/roadmaps/nexus.json` |
| 2 | anima | core | consciousness-engine | `$HOME/Dev/anima/anima` | `shared/roadmaps/anima.json` |
| 3 | n6-architecture | core | system-design | `$HOME/Dev/n6-architecture` | `shared/roadmaps/n6-architecture.json` |
| 4 | papers | core | paper-distribution | `$HOME/Dev/papers` | `shared/roadmaps/papers.json` |
| 5 | hexa-lang | core | language | `$HOME/Dev/hexa-lang` | `shared/roadmaps/hexa-lang.json` |
| 6 | void | core | terminal | `$HOME/Dev/void` | `shared/roadmaps/void.json` |
| 7 | airgenome | core | os-scanner | `$HOME/Dev/airgenome` | `airgenome/shared/config/roadmap/airgenome.json` |
| 8 | contribution | auxiliary | paper-submission-hub | `$HOME/Dev/contribution` | (none; aggregated via nexus) |
| 9 | openclaw | auxiliary | singularity-feed | `$HOME/Dev/openclaw` | (none; nexus feed) |

- Categories: `projects.json._meta.categories.core` + `.auxiliary` (9 total)
- The 9th priority slot (the extra 2) is filled by the two auxiliary projects. This is documented in `auxiliary._comment` as "auxiliary to the nexus singularity integration".

## 2. Interface File Design

File: `/Users/ghost/Dev/n6-architecture/bridge/ecosystem_9projects.hexa`

### 2.1 Registry Constant Table

The `PROJECTS_9` array (9 rows x 6 columns):

```
[proj_id, root_rel, role, icon, roadmap_rel, category]
```

- n6-architecture only reads this table; it does not touch external daemons.
- When the original SSOT changes, this table must be synchronized (a local cache kept specifically to avoid lock-in).

### 2.2 Three Core Functions

| Function | Signature | Role |
|---|---|---|
| `link_project(proj_id)` | `string -> [string; 5]` | Checks whether the project root/roadmap exists and issues a `handle = [id, root_abs, status, roadmap_abs, role]`. `status in {ok, missing_root, missing_roadmap, unknown}` |
| `broadcast_finding(finding)` | `[string; 4] -> i64` | Appends `[source, kind, id, payload]` to `~/.nexus/growth_bus.jsonl` append-only. Expands the other 8 projects into the `targets` field so each daemon can subscribe by tailing the file |
| `aggregate_growth_metrics()` | `() -> [[string; 4]]` | Aggregates the 9 projects' status plus the number of roadmap task keys and returns a row list |

### 2.3 CLI Usage

```
hexa run bridge/ecosystem_9projects.hexa --list        # status of the 9 links
hexa run bridge/ecosystem_9projects.hexa --metrics     # roadmap tasks aggregation
hexa run bridge/ecosystem_9projects.hexa --flow        # provide/absorb matrix
hexa run bridge/ecosystem_9projects.hexa --broadcast "message"
```

## 3. Integration Model

1. **Pull direction (n6 -> external)**: `link_project(id)` only checks for local disk existence. No external daemon API is called, so there are no side effects.
2. **Push direction (n6 -> external)**: Each line `broadcast_finding` writes into `~/.nexus/growth_bus.jsonl` contains `source, targets, kind, id, payload, ts, emitter`. The growth daemon in each project already follows the common growth_bus contract (see `nexus/scripts/growth_common.sh`). This interface performs writes only.
3. **Metrics collection**: `aggregate_growth_metrics` counts the `"tasks"` key in each project's roadmap JSON via grep. It only reads the static roadmap files; no data-plane access is performed.
4. **Loopback avoidance**: When broadcasting, if `source == n6-architecture`, the source is removed from `targets`.
5. **Scope boundary**: This interface is only invoked from within n6-architecture. It is confined to `bridge/` so that other projects do not import it.

## 4. n6-architecture Provide / Absorb Mapping

| Project | What n6-architecture **provides** | What n6-architecture **absorbs** |
|---|---|---|
| nexus | 295 domain scan targets + 77 SEDI sources + 343 BT | telescope 1028 lenses, discovery graph, OUROBOROS |
| anima | n6-SPEAK 384d intents + 6emo + tau prosody | 2509 laws, consciousness Phi measurement, 12-faction consensus |
| n6-architecture | atlas.n6 SSOT, 112 AI techniques, alien_index product | (self, loopback 0) |
| papers | alien_index=10 paper skeleton + verify.py | Zenodo/OSF DOI chain, PP1~PP3 verification |
| hexa-lang | Porting feedback for `domains/**/*.hexa`, R29 verify | Compiler improvements, self-host build, 33+ Rust tests |
| void | AI-native terminal usage profile, execution logs | hexa-only terminal runtime (Terminal.app replacement) |
| airgenome | OS genome benchmark requests, HW 4-tier matrix | OS genome scan results, core self-contained |
| contribution | Aggregation of n6-architecture math breakthroughs and experimental results | External submission hub (paper-submission) |
| openclaw | Emits breakthrough / discovery / experiment feeds | Consumes the nexus singularity cycle |

## 5. Status Checklist

Smoke test (2026-04-14):

- [x] `hexa parse` passes
- [x] `--list`: 9/9 status `ok`
- [x] `--metrics`: 9/9 status `ok`, roadmap tasks = 12/12/12/9/15/12/0/0/0 (nexus/anima/n6/papers/hexa-lang/void/airgenome*/contribution*/openclaw*, where * is auxiliary or a separate path)
- [x] `--flow`: 9 rows emitted
- [x] `--broadcast`: 1 line successfully appended to `~/.nexus/growth_bus.jsonl`
- [x] `link_project("unknown")` returns `status = "unknown"`
- [x] No modifications to external daemons (read and append-only)

## 6. Preconditions and Limitations

- Precondition: `$HOME/Dev/nexus` and `$HOME/Dev/<each project>` must exist on the local disk.
- Limitation 1: airgenome's roadmap lives inside the project itself rather than under nexus/shared, so `link_project` handles it with a special branch.
- Limitation 2: The 2 auxiliary projects have no roadmap, so metrics tasks=0 (by design). A follow-up roadmap should decide whether to average tasks over the 7 core projects only.
- Limitation 3: The hexa runtime does not support `.substr`, so a local `pad_r(s, w)` is used as a fixed-width formatting replacement.

## 7. Future Work

1. Re-check compatibility of the emitter field and schema on the growth_bus.jsonl subscriber side (link to the nexus `growth_common` contract).
2. Add roadmaps for contribution / openclaw so that metrics are automatically reflected.
3. Add a "last commit time" column to `aggregate_growth_metrics` (via one line of `git log`).
4. CI: Set up a hook that runs `hexa parse` over every hexa file under bridge/ in bulk.
