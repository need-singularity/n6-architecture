# NEXUS-6 Growth System

Autonomous self-improvement engine for the NEXUS-6 Discovery Engine.
Measures, tracks, plans, and executes growth across 15 dimensions.

## Architecture (5 Layers)

```
  Layer 5: Execution     scripts/*.hexa       Claude CLI drives plans
  Layer 4: Planning      planner, architect   Synthesize prioritized actions
  Layer 3: Analysis      *_grower (7 modules) Domain-specific gap assessment
  Layer 2: Tracking      tracker, registry    History, snapshots, dashboards
  Layer 1: Measurement   metrics, benchmark   Collect health + performance data
```

Data flows upward: measurement feeds tracking, tracking feeds analysis,
analysis feeds planning, planning feeds execution. Each execution cycle
loops back to measurement for the next iteration.

## 15 Growth Dimensions

| # | Dimension       | Target    | Unit    | Weight | Description                                         |
|---|-----------------|-----------|---------|--------|-----------------------------------------------------|
| 1 | Performance     | 10000     | ops/sec | 0.08   | Telescope scan throughput and latency                |
| 2 | Architecture    | 100%      | percent | 0.10   | Module structure completeness, no orphans/stubs      |
| 3 | Lenses          | 200       | count   | 0.10   | Implemented Lens trait impls (from ~24)              |
| 4 | Modules         | 4.0/5.0   | score   | 0.04   | Mean module maturity level                           |
| 5 | Tests           | 1000      | count   | 0.12   | Total test count across all modules                  |
| 6 | Hypotheses      | 150       | count   | 0.08   | Breakthrough theorem count                           |
| 7 | DSE             | 322       | count   | 0.06   | DSE domain TOML coverage                             |
| 8 | Experiments     | 50        | count   | 0.05   | Verification experiment count                        |
| 9 | Calculators     | 50        | count   | 0.04   | HEXA calculator count                                |
|10 | CrossResonance  | 100       | count   | 0.05   | Cross-domain resonance discoveries                   |
|11 | KnowledgeGraph  | 500       | count   | 0.06   | Graph nodes in knowledge base                        |
|12 | RedTeam         | 100       | count   | 0.06   | Adversarial challenge count                          |
|13 | Atlas           | 2000      | count   | 0.05   | Math atlas constant entries                          |
|14 | Documentation   | 90%       | percent | 0.03   | Doc coverage and quality                             |
|15 | Integration     | 50        | count   | 0.08   | Cross-module integration tests                       |

## HEXA Modules (14 files in `nexus/growth/`)

| File                     | Key Types                                          | Purpose                                |
|--------------------------|----------------------------------------------------|----------------------------------------|
| `mod.hexa`               | (re-exports)                                       | Module root with architecture overview |
| `metrics.hexa`           | `NexusMetrics`, `MetricsDelta`                     | System health snapshot collection      |
| `benchmark.hexa`         | `BenchmarkResult`, `BenchmarkSuite`                | Micro-benchmark core operations        |
| `tracker.hexa`           | `GrowthTracker`, `GrowthTargets`, `GrowthTrend`   | History tracking and trend analysis    |
| `planner.hexa`           | `GrowthPlan`, `GrowthAction`                       | Synthesize prioritized action plans    |
| `registry.hexa`          | `GrowthRegistry`, `GrowthDimension`, `DimensionState` | Central 15-dimension registry      |
| `architect.hexa`         | `ArchGap`, `ArchPlan`, `ModuleStatus`              | Structural gap discovery               |
| `lens_grower.hexa`       | `LensGrowthState`, `LensGrowthPlan`, `LensToImplement` | Lens implementation gap analysis  |
| `module_grower.hexa`     | `ModuleMaturity`, `ModuleInfo`, `ModuleGrowthPlan` | Module maturity classification         |
| `hypothesis_grower.hexa` | `BTState`, `BTCandidate`, `BTGrowthPlan`           | BT gap analysis and candidate gen      |
| `experiment_grower.hexa` | `ExperimentState`, `ExperimentToCreate`             | Experiment coverage planning           |
| `resonance_grower.hexa`  | `ResonanceState`, `ResonanceSearch`                | Cross-domain resonance expansion       |
| `atlas_grower.hexa`      | `AtlasState`, `AtlasEntry`, `AtlasGrowthPlan`     | Math atlas coverage expansion          |
| `redteam_grower.hexa`    | `RedTeamState`, `ChallengeToCreate`                | Red team challenge coverage            |

## HEXA Scripts (16 files in `scripts/`)

| Script                       | Usage                                                            | Description                              |
|------------------------------|------------------------------------------------------------------|------------------------------------------|
| `auto_grow.hexa`             | `$HEXA auto_grow.hexa --cycles N --dry-run --skip-commit`        | Main auto-growth loop                    |
| `nexus_growth_daemon.hexa`   | `$HEXA nexus_growth_daemon.hexa --interval MIN --max-cycles N`   | Master coordinator for all 15 dimensions |
| `grow_lens.hexa`             | `$HEXA grow_lens.hexa <lens_name>`                               | Implement a single lens via Claude CLI   |
| `grow_lenses.hexa`           | `$HEXA grow_lenses.hexa --batch N --dry-run`                     | Batch lens implementation                |
| `grow_modules.hexa`          | `$HEXA grow_modules.hexa --target MODULE --upgrade-all-stubs`    | Module maturity upgrade        |
| `grow_architecture.hexa`     | `$HEXA grow_architecture.hexa --dry-run --max-actions N`         | Architecture gap repair                  |
| `grow_tests.hexa`            | `$HEXA grow_tests.hexa [module_name]`                            | Add tests to under-tested modules        |
| `measure.hexa`               | `$HEXA measure.hexa`                                             | Collect metrics as JSON                  |
| `growth_dashboard.hexa`      | `$HEXA growth_dashboard.hexa --live --last N`                    | ASCII dashboard of all dimensions        |
| `growth_report.hexa`         | `$HEXA growth_report.hexa --last N`                              | Growth history trend table               |
| `growth_daily_report.hexa`   | `$HEXA growth_daily_report.hexa --days N --output FILE`          | Daily summary from growth log            |
| `growth_intelligence.hexa`   | (called by daemon)                                               | Adaptive strategy from past patterns     |
| `growth_notify.hexa`         | `$HEXA growth_notify.hexa "message" --level info\|warn\|error`   | macOS/terminal/log notification          |
| `health_check.hexa`          | `$HEXA health_check.hexa --quiet --start-if-dead`                | Daemon liveness check                    |
| `install_autonomous.hexa`    | `$HEXA install_autonomous.hexa --uninstall`                      | Install launchd + cron + git hooks       |
| `troubleshoot_update.hexa`   | `$HEXA troubleshoot_update.hexa --record-failure\|--auto-fix`    | Auto-record and fix failures       |

## Quick Start

```bash
# Run NEXUS-6
$HEXA nexus/main.hexa

# Check current health
$HEXA nexus/scripts/measure.hexa

# View growth dashboard
$HEXA nexus/scripts/growth_dashboard.hexa

# Run one growth cycle (dry run)
$HEXA nexus/scripts/auto_grow.hexa --cycles 1 --dry-run

# Start the daemon (30-min intervals, max 10 cycles)
$HEXA nexus/scripts/nexus_growth_daemon.hexa --interval 30 --max-cycles 10

# Install full autonomous setup (launchd + cron)
$HEXA nexus/scripts/install_autonomous.hexa
```

## Troubleshooting

| Problem                          | Solution                                                               |
|----------------------------------|------------------------------------------------------------------------|
| Daemon not running               | `$HEXA nexus/scripts/health_check.hexa --start-if-dead`                |
| Claude CLI not found             | `export CLAUDE_CLI=/path/to/claude` before running                     |
| Tests fail after growth          | `$HEXA nexus/scripts/troubleshoot_update.hexa --auto-fix`              |
| HEXA build fails                 | `$HEXA nexus/main.hexa --check 2>&1`                                   |
| Growth log missing               | Created automatically on first `measure.hexa` run                      |
| Dimension stuck at 0%            | Check daemon logs: `tail -20 scripts/growth_log.jsonl`                 |
| Too many warnings                | `$HEXA nexus/scripts/grow_architecture.hexa --max-actions 6`           |
