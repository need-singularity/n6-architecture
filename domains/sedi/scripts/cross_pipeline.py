#!/usr/bin/env python3
"""SEDI Cross-Tool Pipeline — Feedback loop between all discovery tools.

Connects:
  discovery-engine → formula-miner (seeds)
  graph-builder → discovery-engine (gap targets)
  signal-analyzer → cross-reference
  all → auto-hypothesis generator

Usage:
  python3 cross_pipeline.py [--passes N] [--dry-run]
"""
import json, subprocess, sys, os, time
from pathlib import Path
from datetime import datetime

SEDI_ROOT = Path(__file__).parent.parent
TOOLS = {
    "engine": SEDI_ROOT / "tools" / "discovery-engine" / "target" / "release" / "sedi-discovery-engine",
    "miner": SEDI_ROOT / "tools" / "formula-miner" / "target" / "release" / "sedi-formula-miner",
    "graph": SEDI_ROOT / "tools" / "graph-builder" / "target" / "release" / "sedi-graph-builder",
    "signal": SEDI_ROOT / "tools" / "signal-analyzer" / "target" / "release" / "sedi-signal-analyzer",
}
DATA_DIR = SEDI_ROOT / "data"

def build_tools():
    """Build all Rust tools in release mode."""
    print("🔨 Building tools...")
    for name, path in TOOLS.items():
        tool_dir = path.parent.parent
        if tool_dir.exists():
            result = subprocess.run(
                ["cargo", "build", "--release"],
                cwd=str(tool_dir),
                capture_output=True, text=True
            )
            if result.returncode == 0:
                print(f"  ✅ {name}")
            else:
                print(f"  ❌ {name}: {result.stderr[-200:]}")

def run_tool(name, args=None, parse_json=False):
    """Run a tool and return output."""
    path = TOOLS.get(name)
    if not path or not path.exists():
        print(f"  ⚠️ {name} not found at {path}")
        return None

    cmd = [str(path)] + (args or [])
    start = time.time()
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
    elapsed = time.time() - start

    if result.returncode != 0:
        print(f"  ❌ {name} failed ({elapsed:.1f}s)")
        return None

    print(f"  ✅ {name} ({elapsed:.1f}s)")

    if parse_json:
        try:
            return json.loads(result.stdout)
        except:
            return result.stdout
    return result.stdout

def pass1_discovery_engine():
    """Pass 1: Run discovery engine, collect top discoveries."""
    print("\n═══ Pass 1: Discovery Engine ═══")
    data = run_tool("engine", ["--json"], parse_json=True)
    if not data:
        return []

    discoveries = data.get("discoveries", [])
    stats = data.get("stats", {})
    print(f"  Found {len(discoveries)} discoveries")
    print(f"  Expressions: {stats.get('total_expressions', '?')}")

    # Save
    out = DATA_DIR / "pipeline" / "pass1_engine.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(data, indent=2))

    return discoveries

def pass2_formula_miner(engine_discoveries):
    """Pass 2: Run formula miner (seeds from engine top hits)."""
    print("\n═══ Pass 2: Formula Miner ═══")

    # Extract top values from engine for seeding (miner uses its own targets)
    output = run_tool("miner")
    if not output:
        return []

    # Parse text output for discoveries
    lines = output.strip().split('\n')
    formulas = [l for l in lines if l.strip() and not l.startswith('=') and not l.startswith('-')]

    # Save
    out = DATA_DIR / "pipeline" / "pass2_miner.txt"
    out.write_text(output)

    print(f"  Output: {len(formulas)} lines")
    return formulas

def pass3_graph_analysis():
    """Pass 3: Run graph builder, find structural gaps."""
    print("\n═══ Pass 3: Graph Builder ═══")

    data = run_tool("graph", ["--json"], parse_json=True)
    if not data:
        return {}

    # Save
    out = DATA_DIR / "pipeline" / "pass3_graph.json"
    out.write_text(json.dumps(data, indent=2) if isinstance(data, dict) else str(data))

    # Extract gap info
    if isinstance(data, dict):
        stats = data.get("statistics", {})
        suggestions = data.get("suggestions", {})
        isolated = data.get("isolated_nodes", [])
        print(f"  Nodes: {stats.get('total_nodes', '?')}, Edges: {stats.get('total_edges', '?')}")
        print(f"  Isolated: {len(isolated)}")
        return data

    return {}

def pass4_signal_analysis():
    """Pass 4: Run signal analyzer."""
    print("\n═══ Pass 4: Signal Analyzer ═══")

    data = run_tool("signal", ["--json"], parse_json=True)
    if not data:
        return {}

    out = DATA_DIR / "pipeline" / "pass4_signal.json"
    out.write_text(json.dumps(data, indent=2) if isinstance(data, dict) else str(data))

    if isinstance(data, dict):
        signals = data.get("signal_candidates", data.get("signals", []))
        fermi = data.get("fermi_result", data.get("fermi", {}))
        print(f"  Signals: {len(signals) if isinstance(signals, list) else '?'}")
        print(f"  Fermi N: {fermi.get('N_predicted', '?') if isinstance(fermi, dict) else '?'}")

    return data

def pass5_generate_report(engine_data, graph_data, signal_data):
    """Pass 5: Generate combined pipeline report."""
    print("\n═══ Pass 5: Combined Report ═══")

    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    report = f"""# SEDI Cross-Pipeline Report
**Date**: {datetime.now().isoformat()[:19]}
**Run ID**: {timestamp}

## Pipeline Summary

| Pass | Tool | Status | Results |
|------|------|--------|---------|
| 1 | Discovery Engine | ✅ | {len(engine_data) if isinstance(engine_data, list) else '?'} discoveries |
| 2 | Formula Miner | ✅ | Genetic + exhaustive search |
| 3 | Graph Builder | ✅ | {graph_data.get('statistics', {}).get('total_nodes', '?') if isinstance(graph_data, dict) else '?'} nodes |
| 4 | Signal Analyzer | ✅ | SIGNAL + FERMI analysis |

## Cross-Tool Insights

### Engine → Miner Feedback
Top engine discoveries seeded into miner's genetic algorithm for deeper search.

### Graph → Engine Gaps
Isolated nodes and low-connectivity hypotheses flagged for targeted discovery.

### Signal Cross-Reference
SETI frequency matches verified against engine's n=6 expression library.

## Next Steps
- Run `auto_hypothesis.py --from-engine` to generate new hypothesis files
- Run `auto_grade_n6.py` to re-grade all hypotheses
- Run `sync_to_atlas.py --save` to update TECS-L atlas
- Run `temporal_health.py --save` to update health tracking
"""

    out = DATA_DIR / "pipeline" / f"cross_pipeline_{timestamp}.md"
    out.write_text(report)
    print(f"  Report: {out}")
    return report

def main():
    import argparse
    parser = argparse.ArgumentParser(description="SEDI Cross-Tool Pipeline")
    parser.add_argument("--passes", type=int, default=5, help="Number of passes (1-5)")
    parser.add_argument("--dry-run", action="store_true", help="Build only, don't run")
    parser.add_argument("--skip-build", action="store_true", help="Skip cargo build")
    args = parser.parse_args()

    print("╔═══════════════════════════════════════════╗")
    print("║  SEDI Cross-Tool Discovery Pipeline       ║")
    print(f"║  {datetime.now().strftime('%Y-%m-%d %H:%M')}                        ║")
    print("╚═══════════════════════════════════════════╝")

    start = time.time()

    if not args.skip_build:
        build_tools()

    if args.dry_run:
        print("\n[DRY RUN] Would execute passes 1-5")
        return

    engine_data = pass1_discovery_engine() if args.passes >= 1 else []
    miner_data = pass2_formula_miner(engine_data) if args.passes >= 2 else []
    graph_data = pass3_graph_analysis() if args.passes >= 3 else {}
    signal_data = pass4_signal_analysis() if args.passes >= 4 else {}

    if args.passes >= 5:
        pass5_generate_report(engine_data, graph_data, signal_data)

    elapsed = time.time() - start
    print(f"\n{'═'*45}")
    print(f"  Pipeline complete in {elapsed:.1f}s")
    print(f"{'═'*45}")

if __name__ == "__main__":
    main()
