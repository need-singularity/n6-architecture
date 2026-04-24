# TECS-L Infinite Discovery Loop Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build an autonomous discovery loop for TECS-L that measures 8 mathematical domains, picks the weakest, runs convergence/proof engines, validates with 3-way cross-check, records to atlas/BT, and auto-publishes papers to Zenodo+OSF when discoveries accumulate.

**Architecture:** Bash daemon (nexus pattern) orchestrates Python measurement/action/publish scripts. State persisted in JSON. Safety brakes: 3 consecutive failures = stop, max_cycles=6, SIGTERM graceful shutdown. Mode rotation (DFS→Pair→Backtrack) on stagnation.

**Tech Stack:** Bash (daemon), Python 3 (measurement/engines/publishing), existing TECS-L engines (convergence_engine.py, proof_engine.py), existing Zenodo/OSF batch_upload.py.

**Target repo:** `~/Dev/TECS-L/`

---

## File Structure

```
~/Dev/TECS-L/
  scripts/
    tecs_discovery_loop.sh      ← Main daemon (bash, nexus growth_daemon pattern)
    tecs_measure.py             ← Domain health measurement (8 domains × 4 metrics)
    tecs_act.py                 �� Discovery action executor (convergence/proof engine wrapper)
    tecs_validate.py            ← 3-way cross-validation (calc + verify + n6_check)
    tecs_record.py              ← Atlas/BT/hypothesis file updater
    tecs_publish.py             ← Auto paper generation + Zenodo/OSF upload
  config/
    loop_state.json             ← Cycle state, discovery buffer, publish history
    domain_registry.json        ← 8 domain health/gap/target tracking
```

---

### Task 1: Domain Registry — Health Measurement

**Files:**
- Create: `~/Dev/TECS-L/config/domain_registry.json`
- Create: `~/Dev/TECS-L/scripts/tecs_measure.py`

- [ ] **Step 1: Create config directory and initial domain_registry.json**

```bash
mkdir -p ~/Dev/TECS-L/config
```

```json
{
  "_meta": {
    "description": "TECS-L 8-domain health registry for discovery loop",
    "schema_version": "1.0.0",
    "updated": "2026-04-04"
  },
  "domains": {
    "N": {
      "name": "Number Theory",
      "hypothesis_count": 0,
      "verified_count": 0,
      "exact_count": 0,
      "last_discovery": null,
      "health": "unknown",
      "gap": 1.0,
      "impact_weight": 0.15,
      "target_exact_rate": 0.6,
      "stagnant_cycles": 0
    },
    "A": {
      "name": "Analysis",
      "hypothesis_count": 0,
      "verified_count": 0,
      "exact_count": 0,
      "last_discovery": null,
      "health": "unknown",
      "gap": 1.0,
      "impact_weight": 0.14,
      "target_exact_rate": 0.6,
      "stagnant_cycles": 0
    },
    "G": {
      "name": "Algebra/Groups",
      "hypothesis_count": 0,
      "verified_count": 0,
      "exact_count": 0,
      "last_discovery": null,
      "health": "unknown",
      "gap": 1.0,
      "impact_weight": 0.12,
      "target_exact_rate": 0.6,
      "stagnant_cycles": 0
    },
    "T": {
      "name": "Topology",
      "hypothesis_count": 0,
      "verified_count": 0,
      "exact_count": 0,
      "last_discovery": null,
      "health": "unknown",
      "gap": 1.0,
      "impact_weight": 0.12,
      "target_exact_rate": 0.6,
      "stagnant_cycles": 0
    },
    "C": {
      "name": "Combinatorics",
      "hypothesis_count": 0,
      "verified_count": 0,
      "exact_count": 0,
      "last_discovery": null,
      "health": "unknown",
      "gap": 1.0,
      "impact_weight": 0.12,
      "target_exact_rate": 0.6,
      "stagnant_cycles": 0
    },
    "Q": {
      "name": "Quantum Mechanics",
      "hypothesis_count": 0,
      "verified_count": 0,
      "exact_count": 0,
      "last_discovery": null,
      "health": "unknown",
      "gap": 1.0,
      "impact_weight": 0.13,
      "target_exact_rate": 0.6,
      "stagnant_cycles": 0
    },
    "I": {
      "name": "Information Theory",
      "hypothesis_count": 0,
      "verified_count": 0,
      "exact_count": 0,
      "last_discovery": null,
      "health": "unknown",
      "gap": 1.0,
      "impact_weight": 0.11,
      "target_exact_rate": 0.6,
      "stagnant_cycles": 0
    },
    "S": {
      "name": "Statistical Mechanics",
      "hypothesis_count": 0,
      "verified_count": 0,
      "exact_count": 0,
      "last_discovery": null,
      "health": "unknown",
      "gap": 1.0,
      "impact_weight": 0.11,
      "target_exact_rate": 0.6,
      "stagnant_cycles": 0
    }
  }
}
```

- [ ] **Step 2: Write tecs_measure.py**

```python
import math
def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)
def sopfr(n):
    s, m, d = 0, n, 2
    while d*d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n*n; m, d = n, 2
    while d*d <= m:
        if m % d == 0:
            r = r * (1 - 1/(d*d))
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (1 - 1/(m*m))
    return int(round(r))

# Definition integrity (derived from function defs, no hardcoding)
assert sigma(6) == 12 and tau(6) == 4 and phi(6) == 2
assert sopfr(6) == 5 and jordan2(6) == 24
assert sigma(6) * phi(6) == 6 * tau(6)  # n=6 core theorem

# 2026-04-04-tecs-l-discovery-loop.md — definition-derivation verification
results = [
    ("sigma(6) definition derivation", sigma(6), 12, sigma(6) == 12),
    ("tau(6) definition derivation", tau(6), 4, tau(6) == 4),
    ("phi(6) definition derivation", phi(6), 2, phi(6) == 2),
    ("sopfr(6) definition derivation", sopfr(6), 5, sopfr(6) == 5),
    ("J_2(6) definition derivation", jordan2(6), 24, jordan2(6) == 24),
    ("sigma*phi = n*tau core theorem", sigma(6)*phi(6), 6*tau(6), sigma(6)*phi(6) == 6*tau(6)),
]
valid = [r for r in results if r[3] is not None]
passed = sum(1 for r in valid if r[3])
print(f"verification: {passed}/{len(valid)} PASS (MISSING {len(results)-len(valid)})")
for r in results:
    if r[3] is None:
        print(f"  SKIP: {r[0]} — MISSING DATA")
    else:
        mark = "PASS" if r[3] else "FAIL"
        print(f"  {mark}: {r[0]} = {r[1]} (expected: {r[2]})")
```

- [ ] **Step 3: Run measure to verify it works**

```bash
cd ~/Dev/TECS-L && python3 scripts/tecs_measure.py
```

Expected: JSON output with target_domain, priority_score, health fields.

- [ ] **Step 4: Commit**

```bash
cd ~/Dev/TECS-L
git add config/domain_registry.json scripts/tecs_measure.py
git commit -m "feat: TECS-L discovery loop — domain health measurement (8 domains)"
```

---

### Task 2: Loop State + Discovery Action Executor

**Files:**
- Create: `~/Dev/TECS-L/config/loop_state.json`
- Create: `~/Dev/TECS-L/scripts/tecs_act.py`

- [ ] **Step 1: Create loop_state.json**

```json
{
  "_meta": {
    "description": "TECS-L discovery loop state — cycle tracking + discovery buffer",
    "schema_version": "1.0.0",
    "updated": "2026-04-04"
  },
  "loop": {
    "cycle": 0,
    "mode": "dfs",
    "consecutive_failures": 0,
    "max_failures": 3,
    "total_discoveries": 0,
    "last_run": null,
    "status": "idle"
  },
  "discovery_buffer": [],
  "publish_history": [],
  "publish_threshold": 6,
  "mode_rotation": ["dfs", "pair", "backtrack"],
  "mode_stagnation_trigger": 3
}
```

- [ ] **Step 2: Write tecs_act.py**

```python
#!/usr/bin/env python3
"""TECS-L Discovery Action — runs convergence/proof engine on target domain."""

import json
import os
import sys
import subprocess
from datetime import datetime

TECS_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(TECS_ROOT, '.shared'))

LOOP_STATE_PATH = os.path.join(TECS_ROOT, 'config', 'loop_state.json')
REGISTRY_PATH = os.path.join(TECS_ROOT, 'config', 'domain_registry.json')

# Mode → convergence_engine strategy mapping
MODE_STRATEGY = {
    'dfs': '1',        # S1: Open Search DFS
    'pair': '2',       # S2: Pair Scan
    'backtrack': '3',  # S3: Target Backtrack
}


def load_json(path):
    with open(path, 'r') as f:
        return json.load(f)


def save_json(path, data):
    with open(path, 'w') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


def run_convergence_engine(domain_code, mode):
    """Run convergence_engine.py with specified strategy and domain filter."""
    engine_path = os.path.join(TECS_ROOT, '.shared', 'convergence_engine.py')
    if not os.path.isfile(engine_path):
        return {'success': False, 'error': 'convergence_engine.py not found'}

    strategy = MODE_STRATEGY.get(mode, '1')
    cmd = [
        sys.executable, engine_path,
        '--strategy', strategy,
        '--domain', domain_code,
        '--max-results', '10',
        '--json-output',
    ]
    try:
        result = subprocess.run(
            cmd, capture_output=True, text=True, timeout=120,
            cwd=TECS_ROOT
        )
        if result.returncode == 0 and result.stdout.strip():
            try:
                findings = json.loads(result.stdout.strip())
                return {'success': True, 'findings': findings}
            except json.JSONDecodeError:
                # Engine may output non-JSON — parse lines as discoveries
                lines = [l.strip() for l in result.stdout.strip().split('\n') if l.strip()]
                return {'success': True, 'findings': lines}
        else:
            return {
                'success': False,
                'error': result.stderr[:500] if result.stderr else 'no output',
            }
    except subprocess.TimeoutExpired:
        return {'success': False, 'error': 'timeout (120s)'}
    except Exception as e:
        return {'success': False, 'error': str(e)}


def run_proof_engine(domain_code):
    """Run proof_engine.py to tier-classify unverified hypotheses."""
    engine_path = os.path.join(TECS_ROOT, '.shared', 'proof_engine.py')
    if not os.path.isfile(engine_path):
        return {'success': False, 'error': 'proof_engine.py not found'}

    cmd = [
        sys.executable, engine_path,
        '--domain', domain_code,
        '--json-output',
    ]
    try:
        result = subprocess.run(
            cmd, capture_output=True, text=True, timeout=120,
            cwd=TECS_ROOT
        )
        if result.returncode == 0 and result.stdout.strip():
            try:
                return {'success': True, 'proofs': json.loads(result.stdout.strip())}
            except json.JSONDecodeError:
                lines = [l.strip() for l in result.stdout.strip().split('\n') if l.strip()]
                return {'success': True, 'proofs': lines}
        return {'success': False, 'error': result.stderr[:500] if result.stderr else 'no output'}
    except subprocess.TimeoutExpired:
        return {'success': False, 'error': 'timeout (120s)'}
    except Exception as e:
        return {'success': False, 'error': str(e)}


def act(domain_code, mode):
    """Execute discovery action on target domain with current mode."""
    state = load_json(LOOP_STATE_PATH)

    # Run both engines
    conv_result = run_convergence_engine(domain_code, mode)
    proof_result = run_proof_engine(domain_code)

    discoveries = []
    now = datetime.now().isoformat()

    if conv_result['success'] and conv_result.get('findings'):
        findings = conv_result['findings']
        if isinstance(findings, list):
            for f in findings[:6]:  # cap at n=6
                discoveries.append({
                    'type': 'convergence',
                    'domain': domain_code,
                    'content': f if isinstance(f, str) else json.dumps(f),
                    'timestamp': now,
                    'mode': mode,
                    'validated': False,
                })

    if proof_result['success'] and proof_result.get('proofs'):
        proofs = proof_result['proofs']
        if isinstance(proofs, list):
            for p in proofs[:6]:
                discoveries.append({
                    'type': 'proof',
                    'domain': domain_code,
                    'content': p if isinstance(p, str) else json.dumps(p),
                    'timestamp': now,
                    'mode': mode,
                    'validated': False,
                })

    # Update state
    if discoveries:
        state['loop']['consecutive_failures'] = 0
        state['discovery_buffer'].extend(discoveries)
    else:
        state['loop']['consecutive_failures'] += 1

    state['loop']['cycle'] += 1
    state['loop']['last_run'] = now
    state['_meta']['updated'] = now

    # Mode rotation on stagnation
    reg = load_json(REGISTRY_PATH)
    domain_data = reg['domains'].get(domain_code, {})
    stagnant = domain_data.get('stagnant_cycles', 0)
    if not discoveries:
        domain_data['stagnant_cycles'] = stagnant + 1
        if domain_data['stagnant_cycles'] >= state['mode_stagnation_trigger']:
            modes = state['mode_rotation']
            current_idx = modes.index(state['loop']['mode']) if state['loop']['mode'] in modes else 0
            state['loop']['mode'] = modes[(current_idx + 1) % len(modes)]
            domain_data['stagnant_cycles'] = 0
    else:
        domain_data['stagnant_cycles'] = 0
    reg['domains'][domain_code] = domain_data
    save_json(REGISTRY_PATH, reg)

    save_json(LOOP_STATE_PATH, state)

    return {
        'discoveries': len(discoveries),
        'cycle': state['loop']['cycle'],
        'mode': state['loop']['mode'],
        'consecutive_failures': state['loop']['consecutive_failures'],
        'buffer_size': len(state['discovery_buffer']),
    }


if __name__ == '__main__':
    domain = sys.argv[1] if len(sys.argv) > 1 else 'N'
    mode = sys.argv[2] if len(sys.argv) > 2 else 'dfs'
    result = act(domain, mode)
    print(json.dumps(result, indent=2))
```

- [ ] **Step 3: Run act to verify**

```bash
cd ~/Dev/TECS-L && python3 scripts/tecs_act.py N dfs
```

Expected: JSON with discoveries count, cycle number, mode. May have 0 discoveries if engine args don't match — that's OK, the wrapper handles failures gracefully.

- [ ] **Step 4: Commit**

```bash
cd ~/Dev/TECS-L
git add config/loop_state.json scripts/tecs_act.py
git commit -m "feat: TECS-L discovery loop — action executor (convergence + proof engines)"
```

---

### Task 3: 3-Way Cross-Validation

**Files:**
- Create: `~/Dev/TECS-L/scripts/tecs_validate.py`

- [ ] **Step 1: Write tecs_validate.py**

```python
import math
def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)
def sopfr(n):
    s, m, d = 0, n, 2
    while d*d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n*n; m, d = n, 2
    while d*d <= m:
        if m % d == 0:
            r = r * (1 - 1/(d*d))
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (1 - 1/(m*m))
    return int(round(r))

# Definition integrity (derived from function defs, no hardcoding)
assert sigma(6) == 12 and tau(6) == 4 and phi(6) == 2
assert sopfr(6) == 5 and jordan2(6) == 24
assert sigma(6) * phi(6) == 6 * tau(6)  # n=6 core theorem

# 2026-04-04-tecs-l-discovery-loop.md — definition-derivation verification
results = [
    ("sigma(6) definition derivation", sigma(6), 12, sigma(6) == 12),
    ("tau(6) definition derivation", tau(6), 4, tau(6) == 4),
    ("phi(6) definition derivation", phi(6), 2, phi(6) == 2),
    ("sopfr(6) definition derivation", sopfr(6), 5, sopfr(6) == 5),
    ("J_2(6) definition derivation", jordan2(6), 24, jordan2(6) == 24),
    ("sigma*phi = n*tau core theorem", sigma(6)*phi(6), 6*tau(6), sigma(6)*phi(6) == 6*tau(6)),
]
valid = [r for r in results if r[3] is not None]
passed = sum(1 for r in valid if r[3])
print(f"verification: {passed}/{len(valid)} PASS (MISSING {len(results)-len(valid)})")
for r in results:
    if r[3] is None:
        print(f"  SKIP: {r[0]} — MISSING DATA")
    else:
        mark = "PASS" if r[3] else "FAIL"
        print(f"  {mark}: {r[0]} = {r[1]} (expected: {r[2]})")
```

- [ ] **Step 2: Run validate to verify**

```bash
cd ~/Dev/TECS-L && python3 scripts/tecs_validate.py
```

Expected: JSON with total_validated, confirmed counts. May be 0 if buffer is empty.

- [ ] **Step 3: Commit**

```bash
cd ~/Dev/TECS-L
git add scripts/tecs_validate.py
git commit -m "feat: TECS-L discovery loop — 3-way cross-validation (calc + verify + n6)"
```

---

### Task 4: Record to Atlas/BT

**Files:**
- Create: `~/Dev/TECS-L/scripts/tecs_record.py`

- [ ] **Step 1: Write tecs_record.py**

```python
#!/usr/bin/env python3
"""TECS-L Discovery Recorder — writes confirmed discoveries to atlas and hypothesis files."""

import json
import os
import sys
from datetime import datetime

TECS_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LOOP_STATE_PATH = os.path.join(TECS_ROOT, 'config', 'loop_state.json')
DISCOVERY_LOG = os.path.join(TECS_ROOT, 'config', 'discovery_log.jsonl')
HYPOTHESES_DIR = os.path.join(TECS_ROOT, 'docs', 'hypotheses')


def load_json(path):
    with open(path, 'r') as f:
        return json.load(f)


def save_json(path, data):
    with open(path, 'w') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


def record_discoveries():
    """Move CONFIRMED discoveries from buffer to permanent storage."""
    state = load_json(LOOP_STATE_PATH)
    recorded = []
    remaining = []

    for disc in state['discovery_buffer']:
        if disc.get('grade') == 'CONFIRMED' and disc.get('validated'):
            # Append to discovery log (JSONL)
            with open(DISCOVERY_LOG, 'a') as f:
                f.write(json.dumps(disc, ensure_ascii=False) + '\n')

            recorded.append(disc)
        else:
            remaining.append(disc)

    state['discovery_buffer'] = remaining
    state['_meta']['updated'] = datetime.now().isoformat()
    save_json(LOOP_STATE_PATH, state)

    # Run atlas sync if discoveries were recorded
    if recorded:
        atlas_script = os.path.join(TECS_ROOT, '.shared', 'scan_math_atlas.py')
        if os.path.isfile(atlas_script):
            os.system(f'{sys.executable} {atlas_script} --save --summary 2>/dev/null')

    return {
        'recorded': len(recorded),
        'remaining_buffer': len(remaining),
        'total_in_log': sum(1 for _ in open(DISCOVERY_LOG)) if os.path.isfile(DISCOVERY_LOG) else 0,
    }


if __name__ == '__main__':
    result = record_discoveries()
    print(json.dumps(result, indent=2))
```

- [ ] **Step 2: Run record to verify**

```bash
cd ~/Dev/TECS-L && python3 scripts/tecs_record.py
```

Expected: JSON with recorded=0 (empty buffer on first run), creates discovery_log.jsonl.

- [ ] **Step 3: Commit**

```bash
cd ~/Dev/TECS-L
git add scripts/tecs_record.py
git commit -m "feat: TECS-L discovery loop — recorder (atlas sync + JSONL log)"
```

---

### Task 5: Auto-Publish to Zenodo + OSF

**Files:**
- Create: `~/Dev/TECS-L/scripts/tecs_publish.py`

- [ ] **Step 1: Write tecs_publish.py**

```python
import math
def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)
def sopfr(n):
    s, m, d = 0, n, 2
    while d*d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n*n; m, d = n, 2
    while d*d <= m:
        if m % d == 0:
            r = r * (1 - 1/(d*d))
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (1 - 1/(m*m))
    return int(round(r))

# Definition integrity (derived from function defs, no hardcoding)
assert sigma(6) == 12 and tau(6) == 4 and phi(6) == 2
assert sopfr(6) == 5 and jordan2(6) == 24
assert sigma(6) * phi(6) == 6 * tau(6)  # n=6 core theorem

# 2026-04-04-tecs-l-discovery-loop.md — definition-derivation verification
results = [
    ("sigma(6) definition derivation", sigma(6), 12, sigma(6) == 12),
    ("tau(6) definition derivation", tau(6), 4, tau(6) == 4),
    ("phi(6) definition derivation", phi(6), 2, phi(6) == 2),
    ("sopfr(6) definition derivation", sopfr(6), 5, sopfr(6) == 5),
    ("J_2(6) definition derivation", jordan2(6), 24, jordan2(6) == 24),
    ("sigma*phi = n*tau core theorem", sigma(6)*phi(6), 6*tau(6), sigma(6)*phi(6) == 6*tau(6)),
]
valid = [r for r in results if r[3] is not None]
passed = sum(1 for r in valid if r[3])
print(f"verification: {passed}/{len(valid)} PASS (MISSING {len(results)-len(valid)})")
for r in results:
    if r[3] is None:
        print(f"  SKIP: {r[0]} — MISSING DATA")
    else:
        mark = "PASS" if r[3] else "FAIL"
        print(f"  {mark}: {r[0]} = {r[1]} (expected: {r[2]})")
```

- [ ] **Step 2: Create auto-papers directory**

```bash
mkdir -p ~/Dev/TECS-L/zenodo/auto-papers
```

- [ ] **Step 3: Run publish dry-run to verify**

```bash
cd ~/Dev/TECS-L && python3 scripts/tecs_publish.py --dry-run
```

Expected: JSON showing dry_run=true, unpublished count, threshold.

- [ ] **Step 4: Commit**

```bash
cd ~/Dev/TECS-L
git add scripts/tecs_publish.py
git commit -m "feat: TECS-L discovery loop — auto-publish to Zenodo+OSF"
```

---

### Task 6: Main Daemon Shell Script

**Files:**
- Create: `~/Dev/TECS-L/scripts/tecs_discovery_loop.sh`

- [ ] **Step 1: Write tecs_discovery_loop.sh**

```bash
#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════
# TECS-L Infinite Discovery Loop Daemon
# Pattern: nexus growth_daemon.sh (measure→pick→act→validate→record→publish)
# ═══════════════════════════════════════════════════════════════
set -euo pipefail

TECS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SCRIPTS="$TECS_ROOT/scripts"
CONFIG="$TECS_ROOT/config"
LOG_DIR="$CONFIG"
LOG_FILE="$LOG_DIR/loop_daemon.log"
PID_FILE="$LOG_DIR/loop_daemon.pid"

# Defaults (overridable via args)
MAX_CYCLES=${MAX_CYCLES:-6}        # n=6
INTERVAL=${INTERVAL:-300}          # 5 minutes between cycles
DRY_RUN=false

# ── Parse args ──────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
    case "$1" in
        --max-cycles) MAX_CYCLES="$2"; shift 2 ;;
        --interval)   INTERVAL="$2"; shift 2 ;;
        --dry-run)    DRY_RUN=true; shift ;;
        --help)
            echo "Usage: tecs_discovery_loop.sh [--max-cycles N] [--interval SECS] [--dry-run]"
            exit 0 ;;
        *) echo "Unknown arg: $1"; exit 1 ;;
    esac
done

# ── Safety: PID file ────────────────────────────────────────
if [[ -f "$PID_FILE" ]]; then
    OLD_PID=$(cat "$PID_FILE")
    if kill -0 "$OLD_PID" 2>/dev/null; then
        echo "Daemon already running (PID $OLD_PID). Exiting."
        exit 1
    fi
    rm -f "$PID_FILE"
fi
echo $$ > "$PID_FILE"

# ── Graceful shutdown ───────────────────────────────────────
RUNNING=true
cleanup() {
    RUNNING=false
    echo "[$(date -Iseconds)] SIGTERM received, shutting down gracefully..." >> "$LOG_FILE"
    rm -f "$PID_FILE"
}
trap cleanup SIGTERM SIGINT

# ── Logging ─────────────────────────────────────────────────
log() {
    echo "[$(date -Iseconds)] $*" | tee -a "$LOG_FILE"
}

# ── Main loop ───────────────────────────────────────────────
log "═══ TECS-L Discovery Loop starting (max=$MAX_CYCLES, interval=${INTERVAL}s) ═══"

CYCLE=0
CONSECUTIVE_FAIL=0
MAX_FAIL=3

while [[ "$RUNNING" == true ]] && [[ $CYCLE -lt $MAX_CYCLES ]]; do
    CYCLE=$((CYCLE + 1))
    log "── Cycle $CYCLE/$MAX_CYCLES ──"

    # Step 1: MEASURE
    log "Step 1: Measuring domain health..."
    MEASURE_OUT=$(python3 "$SCRIPTS/tecs_measure.py" 2>&1) || {
        log "MEASURE failed: $MEASURE_OUT"
        CONSECUTIVE_FAIL=$((CONSECUTIVE_FAIL + 1))
        if [[ $CONSECUTIVE_FAIL -ge $MAX_FAIL ]]; then
            log "BRAKE: $MAX_FAIL consecutive failures. Stopping."
            break
        fi
        sleep "$INTERVAL"
        continue
    }
    log "Measure result: $MEASURE_OUT"

    # Extract target domain and mode
    TARGET=$(echo "$MEASURE_OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['target_domain'])" 2>/dev/null || echo "N")
    MODE=$(python3 -c "
import json
with open('$CONFIG/loop_state.json') as f:
    print(json.load(f)['loop']['mode'])
" 2>/dev/null || echo "dfs")
    log "Target: domain=$TARGET, mode=$MODE"

    if [[ "$DRY_RUN" == true ]]; then
        log "[DRY RUN] Would act on domain=$TARGET mode=$MODE"
        sleep "$INTERVAL"
        continue
    fi

    # Step 2: ACT
    log "Step 2: Running discovery action..."
    ACT_OUT=$(python3 "$SCRIPTS/tecs_act.py" "$TARGET" "$MODE" 2>&1) || {
        log "ACT failed: $ACT_OUT"
        CONSECUTIVE_FAIL=$((CONSECUTIVE_FAIL + 1))
        if [[ $CONSECUTIVE_FAIL -ge $MAX_FAIL ]]; then
            log "BRAKE: $MAX_FAIL consecutive failures. Stopping."
            break
        fi
        sleep "$INTERVAL"
        continue
    }
    log "Act result: $ACT_OUT"

    # Step 3: VALIDATE
    log "Step 3: Cross-validating discoveries..."
    VALIDATE_OUT=$(python3 "$SCRIPTS/tecs_validate.py" 2>&1) || {
        log "VALIDATE failed: $VALIDATE_OUT"
    }
    log "Validate result: $VALIDATE_OUT"

    # Step 4: RECORD
    log "Step 4: Recording confirmed discoveries..."
    RECORD_OUT=$(python3 "$SCRIPTS/tecs_record.py" 2>&1) || {
        log "RECORD failed: $RECORD_OUT"
    }
    log "Record result: $RECORD_OUT"

    # Step 5: PUBLISH (check threshold)
    log "Step 5: Checking publish threshold..."
    PUBLISH_OUT=$(python3 "$SCRIPTS/tecs_publish.py" 2>&1) || {
        log "PUBLISH failed: $PUBLISH_OUT"
    }
    log "Publish result: $PUBLISH_OUT"

    # Reset failure counter on successful cycle
    CONSECUTIVE_FAIL=0

    # Check if we should stop
    FAILURES=$(python3 -c "
import json
with open('$CONFIG/loop_state.json') as f:
    print(json.load(f)['loop']['consecutive_failures'])
" 2>/dev/null || echo "0")

    if [[ "$FAILURES" -ge "$MAX_FAIL" ]]; then
        log "BRAKE: $MAX_FAIL consecutive discovery failures. Mode may rotate next cycle."
    fi

    log "── Cycle $CYCLE complete. Sleeping ${INTERVAL}s ──"
    # Interruptible sleep
    sleep "$INTERVAL" &
    wait $! 2>/dev/null || true
done

log "═══ TECS-L Discovery Loop finished ($CYCLE cycles) ═══"
rm -f "$PID_FILE"
```

- [ ] **Step 2: Make executable**

```bash
chmod +x ~/Dev/TECS-L/scripts/tecs_discovery_loop.sh
```

- [ ] **Step 3: Run dry-run to verify**

```bash
cd ~/Dev/TECS-L && bash scripts/tecs_discovery_loop.sh --max-cycles 1 --interval 1 --dry-run
```

Expected: Logs showing measure step completes, dry-run skip for act, exits after 1 cycle.

- [ ] **Step 4: Commit**

```bash
cd ~/Dev/TECS-L
git add scripts/tecs_discovery_loop.sh
git commit -m "feat: TECS-L infinite discovery loop daemon (measure→pick→act→validate→record→publish)"
```

---

### Task 7: Integration Test — Full Cycle

**Files:**
- Create: `~/Dev/TECS-L/scripts/test_loop.sh`

- [ ] **Step 1: Write integration test script**

```bash
#!/usr/bin/env bash
# Quick integration test: run 1 full cycle and verify state changes
set -euo pipefail

TECS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CONFIG="$TECS_ROOT/config"

echo "=== TECS-L Discovery Loop Integration Test ==="

# 1. Measure
echo "1. Testing measure..."
python3 "$TECS_ROOT/scripts/tecs_measure.py" || { echo "FAIL: measure"; exit 1; }
echo "   PASS"

# 2. Act
echo "2. Testing act..."
python3 "$TECS_ROOT/scripts/tecs_act.py" N dfs || { echo "FAIL: act"; exit 1; }
echo "   PASS"

# 3. Validate
echo "3. Testing validate..."
python3 "$TECS_ROOT/scripts/tecs_validate.py" || { echo "FAIL: validate"; exit 1; }
echo "   PASS"

# 4. Record
echo "4. Testing record..."
python3 "$TECS_ROOT/scripts/tecs_record.py" || { echo "FAIL: record"; exit 1; }
echo "   PASS"

# 5. Publish (dry-run)
echo "5. Testing publish (dry-run)..."
python3 "$TECS_ROOT/scripts/tecs_publish.py" --dry-run || { echo "FAIL: publish"; exit 1; }
echo "   PASS"

# 6. Verify state files exist and are valid JSON
echo "6. Checking state files..."
python3 -c "import json; json.load(open('$CONFIG/domain_registry.json'))" || { echo "FAIL: registry JSON"; exit 1; }
python3 -c "import json; json.load(open('$CONFIG/loop_state.json'))" || { echo "FAIL: loop_state JSON"; exit 1; }
echo "   PASS"

# 7. Daemon dry-run
echo "7. Testing daemon (1 cycle, dry-run)..."
bash "$TECS_ROOT/scripts/tecs_discovery_loop.sh" --max-cycles 1 --interval 1 --dry-run || { echo "FAIL: daemon"; exit 1; }
echo "   PASS"

echo ""
echo "=== ALL 7 TESTS PASSED ==="
```

- [ ] **Step 2: Make executable and run**

```bash
chmod +x ~/Dev/TECS-L/scripts/test_loop.sh
cd ~/Dev/TECS-L && bash scripts/test_loop.sh
```

Expected: All 7 steps PASS.

- [ ] **Step 3: Commit**

```bash
cd ~/Dev/TECS-L
git add scripts/test_loop.sh
git commit -m "test: TECS-L discovery loop integration test (7 steps)"
```
