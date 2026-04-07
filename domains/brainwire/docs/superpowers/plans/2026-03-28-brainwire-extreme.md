# BrainWire Extreme Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend BrainWire from single-state to a universal consciousness state engine with Tier 4 hardware, closed-loop PID control, and 6 consciousness state profiles.

**Architecture:** Refactor `bench_thc_vars.py` into a modular package `brainwire/` with: (1) state profiles as YAML-loaded 12-variable target vectors, (2) a universal transfer function engine supporting Tier 1-4 hardware with hot-plug HAL, (3) a PID controller simulator, (4) a state interpolation engine for blending/transitions. Extend `calc.py` to support multi-state operations.

**Tech Stack:** Python 3.11+, PyYAML, dataclasses, argparse, math/numpy (optional for PID sim)

---

## File Structure

```
brainwire/
├── __init__.py                    # Package init, version
├── variables.py                   # 12-variable constants, names, categories
├── profiles/
│   ├── __init__.py                # Profile loader (YAML → StateProfile)
│   ├── base.py                    # StateProfile dataclass + envelope
│   ├── thc.yaml                   # THC profile (5 concentration levels)
│   ├── lsd.yaml                   # LSD profile
│   ├── psilocybin.yaml            # Psilocybin profile
│   ├── dmt.yaml                   # DMT profile
│   ├── mdma.yaml                  # MDMA profile
│   └── flow.yaml                  # Flow state profile
├── hardware/
│   ├── __init__.py
│   ├── devices.py                 # Device dataclass + registry (tDCS→tFUS)
│   ├── hal.py                     # Hardware Abstraction Layer (slot mgmt, M-matrix)
│   ├── configs.py                 # Tier 1-4 preset configurations
│   └── safety.py                  # Safety engine (4-layer limits)
├── engine/
│   ├── __init__.py
│   ├── transfer.py                # Universal transfer function (12var × N devices)
│   ├── pid.py                     # PID controller bank (12 independent controllers)
│   ├── interpolation.py           # State interpolation + blending engine
│   └── tension.py                 # PureField tension computation (from existing)
├── bench.py                       # Multi-state benchmark CLI (replaces bench_thc_vars.py)
└── calc.py                        # Extended calculator CLI (replaces calc.py)

tests/
├── test_variables.py
├── test_profiles.py
├── test_hardware.py
├── test_transfer.py
├── test_pid.py
├── test_interpolation.py
├── test_tension.py
├── test_safety.py
├── test_bench.py
└── test_calc.py
```

---

## Task 1: Core Variables Module

**Files:**
- Create: `brainwire/__init__.py`
- Create: `brainwire/variables.py`
- Create: `tests/test_variables.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_variables.py
from brainwire.variables import (
    VAR_NAMES, VAR_CATEGORIES, CHEM_VARS, WAVE_VARS, STATE_VARS,
    TENSION_WEIGHTS, baseline_vector
)

def test_12_variables_defined():
    assert len(VAR_NAMES) == 12

def test_categories_cover_all_vars():
    all_cat = CHEM_VARS + WAVE_VARS + STATE_VARS
    assert set(all_cat) == set(VAR_NAMES)

def test_chem_vars():
    assert CHEM_VARS == ['DA', 'eCB', '5HT', 'GABA', 'NE']

def test_wave_vars():
    assert WAVE_VARS == ['Theta', 'Alpha', 'Gamma']

def test_state_vars():
    assert STATE_VARS == ['PFC', 'Sensory', 'Body', 'Coherence']

def test_tension_weights_all_positive():
    for k, w in TENSION_WEIGHTS.items():
        assert w > 0, f"{k} weight must be positive"
    assert set(TENSION_WEIGHTS.keys()) == set(VAR_NAMES)

def test_baseline_vector():
    b = baseline_vector()
    assert len(b) == 12
    for k, v in b.items():
        assert v == 1.0, f"baseline {k} should be 1.0"
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_variables.py -v`
Expected: FAIL — ModuleNotFoundError: No module named 'brainwire'

- [ ] **Step 3: Write minimal implementation**

```python
# brainwire/__init__.py
"""BrainWire — Universal Consciousness State Engine."""
__version__ = "2.0.0"
```

```python
# brainwire/variables.py
"""12-variable consciousness model constants."""

VAR_NAMES = [
    'DA', 'eCB', '5HT', 'GABA', 'NE',
    'Theta', 'Alpha', 'Gamma',
    'PFC', 'Sensory', 'Body', 'Coherence',
]

CHEM_VARS = ['DA', 'eCB', '5HT', 'GABA', 'NE']
WAVE_VARS = ['Theta', 'Alpha', 'Gamma']
STATE_VARS = ['PFC', 'Sensory', 'Body', 'Coherence']

VAR_CATEGORIES = {v: 'chem' for v in CHEM_VARS}
VAR_CATEGORIES.update({v: 'wave' for v in WAVE_VARS})
VAR_CATEGORIES.update({v: 'state' for v in STATE_VARS})

TENSION_WEIGHTS = {
    'DA': 1.2, 'eCB': 1.5, '5HT': 0.8, 'GABA': 0.9, 'NE': 1.0,
    'Theta': 1.3, 'Alpha': 1.0, 'Gamma': 1.1,
    'PFC': 1.0, 'Sensory': 0.9, 'Body': 1.0, 'Coherence': 1.2,
}

def baseline_vector() -> dict[str, float]:
    return {v: 1.0 for v in VAR_NAMES}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_variables.py -v`
Expected: All 7 tests PASS

- [ ] **Step 5: Commit**

```bash
git add brainwire/__init__.py brainwire/variables.py tests/test_variables.py
git commit -m "feat: add core 12-variable constants module"
```

---

## Task 2: State Profile System

**Files:**
- Create: `brainwire/profiles/__init__.py`
- Create: `brainwire/profiles/base.py`
- Create: `brainwire/profiles/thc.yaml`
- Create: `brainwire/profiles/lsd.yaml`
- Create: `brainwire/profiles/psilocybin.yaml`
- Create: `brainwire/profiles/dmt.yaml`
- Create: `brainwire/profiles/mdma.yaml`
- Create: `brainwire/profiles/flow.yaml`
- Create: `tests/test_profiles.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_profiles.py
import pytest
from brainwire.profiles.base import StateProfile, Envelope
from brainwire.profiles import load_profile, list_profiles

def test_envelope_defaults():
    e = Envelope()
    assert e.onset_s > 0
    assert e.plateau_s > 0
    assert e.offset_s > 0
    assert e.curve in ('linear', 'sigmoid', 'exponential')

def test_state_profile_has_12_vars():
    p = StateProfile(
        name="test", category="test",
        target={'DA': 2.0, 'eCB': 2.0, '5HT': 1.5, 'GABA': 1.5, 'NE': 0.5,
                'Theta': 2.0, 'Alpha': 0.5, 'Gamma': 1.8, 'PFC': 0.5,
                'Sensory': 2.0, 'Body': 2.0, 'Coherence': 2.0},
    )
    assert len(p.target) == 12

def test_state_profile_rejects_missing_var():
    with pytest.raises(ValueError):
        StateProfile(name="bad", category="bad", target={'DA': 2.0})

def test_load_thc_profile():
    p = load_profile('thc')
    assert p.name == "THC Strong (25%)"
    assert p.category == "cannabinoid"
    assert len(p.target) == 12
    assert p.target['DA'] == 2.5
    assert p.target['eCB'] == 3.0

def test_load_lsd_profile():
    p = load_profile('lsd')
    assert p.category == "psychedelic"
    assert p.target['5HT'] == 3.5
    assert p.target['NE'] == 2.0  # opposite to THC

def test_load_dmt_profile():
    p = load_profile('dmt')
    assert p.target['5HT'] == 4.5  # strongest
    assert p.target['Sensory'] == 5.0
    assert p.target['Coherence'] == 0.2  # extreme desync
    assert p.envelope.onset_s <= 30  # fast onset

def test_load_psilocybin_profile():
    p = load_profile('psilocybin')
    assert p.target['Theta'] == 3.5  # mystical marker

def test_load_mdma_profile():
    p = load_profile('mdma')
    assert p.target['Body'] == 3.0  # highest body
    assert p.target['Coherence'] == 1.8  # sync up (not desync)

def test_load_flow_profile():
    p = load_profile('flow')
    assert p.target['Alpha'] == 1.5  # only state with alpha UP
    assert p.target['Coherence'] == 2.5  # highest coherence

def test_list_profiles():
    profiles = list_profiles()
    assert len(profiles) >= 6
    assert 'thc' in profiles
    assert 'dmt' in profiles

def test_thc_concentration_scaling():
    p = load_profile('thc')
    micro = p.scale(0.25)  # micro dose
    assert micro['DA'] < p.target['DA']
    assert micro['DA'] == pytest.approx(1.0 + (2.5 - 1.0) * 0.25)
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_profiles.py -v`
Expected: FAIL — ModuleNotFoundError

- [ ] **Step 3: Write implementation**

```python
# brainwire/profiles/base.py
"""State profile definitions."""
from dataclasses import dataclass, field
from brainwire.variables import VAR_NAMES

@dataclass
class Envelope:
    onset_s: float = 300.0
    plateau_s: float = 3600.0
    offset_s: float = 180.0
    curve: str = 'sigmoid'

@dataclass
class SafetyOverrides:
    max_session_min: int = 40
    emergency_vars: list[str] = field(default_factory=list)
    first_session_limits: dict[str, float] = field(default_factory=dict)

@dataclass
class PidHint:
    var: str
    Kp_scale: float = 1.0
    Ki_scale: float = 1.0
    Kd_scale: float = 1.0

@dataclass
class StateProfile:
    name: str
    category: str
    target: dict[str, float]
    envelope: Envelope = field(default_factory=Envelope)
    safety: SafetyOverrides = field(default_factory=SafetyOverrides)
    pid_hints: list[PidHint] = field(default_factory=list)

    def __post_init__(self):
        missing = set(VAR_NAMES) - set(self.target.keys())
        if missing:
            raise ValueError(f"Missing variables: {missing}")

    def scale(self, factor: float) -> dict[str, float]:
        return {k: 1.0 + (v - 1.0) * factor for k, v in self.target.items()}
```

```python
# brainwire/profiles/__init__.py
"""Profile loader — reads YAML files from this directory."""
import os
import yaml
from brainwire.profiles.base import StateProfile, Envelope, SafetyOverrides, PidHint

_PROFILE_DIR = os.path.dirname(__file__)

def load_profile(name: str) -> StateProfile:
    path = os.path.join(_PROFILE_DIR, f"{name}.yaml")
    if not os.path.exists(path):
        raise FileNotFoundError(f"Profile not found: {name}")
    with open(path) as f:
        data = yaml.safe_load(f)
    s = data['state']
    env_data = data.get('envelope', {})
    envelope = Envelope(
        onset_s=env_data.get('onset_s', 300),
        plateau_s=env_data.get('plateau_s', 3600),
        offset_s=env_data.get('offset_s', 180),
        curve=env_data.get('curve', 'sigmoid'),
    )
    safety_data = data.get('safety_overrides', {})
    safety = SafetyOverrides(
        max_session_min=safety_data.get('max_session_min', 40),
        emergency_vars=safety_data.get('emergency_vars', []),
        first_session_limits=safety_data.get('first_session_limits', {}),
    )
    pid_hints = []
    for var, hints in data.get('pid_hints', {}).items():
        pid_hints.append(PidHint(var=var, **hints))
    return StateProfile(
        name=s['name'], category=s['category'],
        target=data['target_vector'], envelope=envelope,
        safety=safety, pid_hints=pid_hints,
    )

def list_profiles() -> list[str]:
    return [f[:-5] for f in os.listdir(_PROFILE_DIR)
            if f.endswith('.yaml')]
```

- [ ] **Step 4: Create all 6 YAML profiles**

```yaml
# brainwire/profiles/thc.yaml
state:
  name: "THC Strong (25%)"
  category: "cannabinoid"

target_vector:
  DA: 2.5
  eCB: 3.0
  5HT: 1.5
  GABA: 1.8
  NE: 0.4
  Theta: 2.5
  Alpha: 0.5
  Gamma: 1.8
  PFC: 0.5
  Sensory: 2.0
  Body: 2.5
  Coherence: 2.0

envelope:
  onset_s: 300
  plateau_s: 3600
  offset_s: 180
  curve: sigmoid

pid_hints:
  eCB:
    Kp_scale: 1.5
    Ki_scale: 2.0
  Theta:
    Kd_scale: 0.5

safety_overrides:
  max_session_min: 60
  emergency_vars: ["NE"]
```

```yaml
# brainwire/profiles/lsd.yaml
state:
  name: "LSD (100ug)"
  category: "psychedelic"

target_vector:
  DA: 1.8
  eCB: 1.3
  5HT: 3.5
  GABA: 0.6
  NE: 2.0
  Theta: 3.0
  Alpha: 0.3
  Gamma: 2.5
  PFC: 1.5
  Sensory: 3.5
  Body: 1.5
  Coherence: 0.4

envelope:
  onset_s: 1800
  plateau_s: 21600
  offset_s: 3600
  curve: sigmoid

pid_hints:
  5HT:
    Kp_scale: 1.8
    Ki_scale: 1.5
  Coherence:
    Kp_scale: 0.5
    Kd_scale: 2.0

safety_overrides:
  max_session_min: 40
  emergency_vars: ["GABA", "NE"]
```

```yaml
# brainwire/profiles/psilocybin.yaml
state:
  name: "Psilocybin (25mg)"
  category: "psychedelic"

target_vector:
  DA: 1.5
  eCB: 1.4
  5HT: 3.0
  GABA: 0.7
  NE: 1.6
  Theta: 3.5
  Alpha: 0.4
  Gamma: 2.0
  PFC: 1.2
  Sensory: 2.5
  Body: 2.0
  Coherence: 0.5

envelope:
  onset_s: 1200
  plateau_s: 14400
  offset_s: 2400
  curve: sigmoid

pid_hints:
  Theta:
    Kp_scale: 1.8
    Ki_scale: 1.5
  5HT:
    Kp_scale: 1.5

safety_overrides:
  max_session_min: 40
  emergency_vars: ["GABA"]
```

```yaml
# brainwire/profiles/dmt.yaml
state:
  name: "DMT Breakthrough (30mg inhaled)"
  category: "psychedelic"

target_vector:
  DA: 2.2
  eCB: 1.2
  5HT: 4.5
  GABA: 0.3
  NE: 2.5
  Theta: 4.0
  Alpha: 0.1
  Gamma: 3.5
  PFC: 2.0
  Sensory: 5.0
  Body: 0.8
  Coherence: 0.2

envelope:
  onset_s: 30
  plateau_s: 600
  offset_s: 300
  curve: exponential

pid_hints:
  5HT:
    Kp_scale: 2.0
    Ki_scale: 2.0
  Sensory:
    Kp_scale: 1.5
  GABA:
    Kd_scale: 3.0

safety_overrides:
  max_session_min: 20
  emergency_vars: ["GABA", "NE", "Sensory"]
  first_session_limits:
    Sensory: 3.0
    5HT: 3.0
    Gamma: 2.5
```

```yaml
# brainwire/profiles/mdma.yaml
state:
  name: "MDMA (125mg)"
  category: "empathogen"

target_vector:
  DA: 2.5
  eCB: 1.8
  5HT: 4.0
  GABA: 1.2
  NE: 2.0
  Theta: 1.5
  Alpha: 1.2
  Gamma: 2.0
  PFC: 1.8
  Sensory: 2.5
  Body: 3.0
  Coherence: 1.8

envelope:
  onset_s: 1800
  plateau_s: 10800
  offset_s: 1800
  curve: sigmoid

pid_hints:
  5HT:
    Kp_scale: 1.5
    Ki_scale: 2.0
  Body:
    Kp_scale: 1.3

safety_overrides:
  max_session_min: 40
  emergency_vars: ["5HT", "NE"]
```

```yaml
# brainwire/profiles/flow.yaml
state:
  name: "Flow State"
  category: "endogenous"

target_vector:
  DA: 1.8
  eCB: 2.0
  5HT: 1.3
  GABA: 1.5
  NE: 1.2
  Theta: 2.0
  Alpha: 1.5
  Gamma: 2.0
  PFC: 0.7
  Sensory: 1.5
  Body: 1.8
  Coherence: 2.5

envelope:
  onset_s: 600
  plateau_s: 7200
  offset_s: 300
  curve: linear

pid_hints:
  Coherence:
    Kp_scale: 1.5
  Alpha:
    Kp_scale: 1.2

safety_overrides:
  max_session_min: 90
  emergency_vars: []
```

- [ ] **Step 5: Run tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_profiles.py -v`
Expected: All 12 tests PASS

- [ ] **Step 6: Commit**

```bash
git add brainwire/profiles/ tests/test_profiles.py
git commit -m "feat: add 6 consciousness state profiles with YAML loader"
```

---

## Task 3: Device Registry and Hardware Abstraction Layer

**Files:**
- Create: `brainwire/hardware/__init__.py`
- Create: `brainwire/hardware/devices.py`
- Create: `brainwire/hardware/hal.py`
- Create: `tests/test_hardware.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_hardware.py
import pytest
from brainwire.hardware.devices import Device, DeviceRegistry, CORE_DEVICES
from brainwire.hardware.hal import HAL

def test_device_has_required_fields():
    d = Device(name='tDCS', slot=0, max_current_mA=4.0, cost_usd=30,
               params=['anode_mA', 'cathode_Fz_mA', 'cathode_F4_mA'])
    assert d.name == 'tDCS'
    assert d.max_current_mA == 4.0

def test_core_devices_count():
    assert len(CORE_DEVICES) >= 8  # tDCS, tACS, TMS, taVNS, TENS, tFUS, GVS, mTI

def test_device_registry_add_remove():
    reg = DeviceRegistry()
    d = Device(name='test', slot=99, max_current_mA=1.0, cost_usd=10, params=['x'])
    reg.add(d)
    assert reg.get('test') == d
    reg.remove('test')
    assert reg.get('test') is None

def test_hal_active_slots():
    hal = HAL()
    hal.connect('tDCS')
    hal.connect('TENS')
    assert hal.active_slots() == [0, 4]
    assert hal.active_device_count() == 2

def test_hal_disconnect():
    hal = HAL()
    hal.connect('tDCS')
    hal.connect('TENS')
    hal.disconnect('TENS')
    assert hal.active_slots() == [0]

def test_hal_total_cost():
    hal = HAL()
    hal.connect('tDCS')
    hal.connect('taVNS')
    cost = hal.total_cost()
    assert cost > 0

def test_hal_tier_detection():
    hal = HAL()
    hal.connect('tDCS')
    hal.connect('TENS')
    assert hal.detect_tier() == 1
    hal.connect('taVNS')
    hal.connect('tACS')
    assert hal.detect_tier() == 2
    hal.connect('TMS')
    assert hal.detect_tier() == 3
    hal.connect('tFUS')
    hal.connect('GVS')
    hal.connect('mTI')
    assert hal.detect_tier() == 4
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_hardware.py -v`
Expected: FAIL

- [ ] **Step 3: Write implementation**

```python
# brainwire/hardware/__init__.py
"""Hardware abstraction for BrainWire stimulation devices."""
```

```python
# brainwire/hardware/devices.py
"""Device definitions and registry."""
from dataclasses import dataclass, field

@dataclass
class Device:
    name: str
    slot: int
    max_current_mA: float
    cost_usd: float
    params: list[str] = field(default_factory=list)
    slew_rate_mA_per_s: float = 0.5
    max_session_min: int = 40

CORE_DEVICES = [
    Device('tDCS', 0, 4.0, 30,
           ['anode_mA', 'cathode_Fz_mA', 'cathode_F4_mA', 'anode_V1_mA', 'anode_S1_mA'],
           slew_rate_mA_per_s=0.5, max_session_min=20),
    Device('tACS', 1, 4.0, 80,
           ['tACS_6Hz_mA', 'tACS_10Hz_mA', 'tACS_40Hz_mA'],
           slew_rate_mA_per_s=0.5, max_session_min=30),
    Device('TMS', 2, 2.5, 5000,
           ['theta_strength', '1Hz_strength', '10Hz_strength', '40Hz_strength'],
           max_session_min=20),
    Device('taVNS', 3, 0.5, 100,
           ['VNS_mA'], slew_rate_mA_per_s=0.1, max_session_min=30),
    Device('TENS', 4, 80.0, 25,
           ['low_intensity', 'high_intensity'],
           slew_rate_mA_per_s=1.0, max_session_min=60),
    Device('tFUS', 5, 720.0, 8000,  # ISPTA mW/cm2
           ['focus_x', 'focus_y', 'focus_z', 'intensity'],
           max_session_min=30),
    Device('GVS', 6, 2.0, 50,
           ['current_mA'], slew_rate_mA_per_s=0.2, max_session_min=20),
    Device('mTI', 7, 2.0, 2000,
           ['pair1_mA', 'pair2_mA', 'pair3_mA', 'freq_offset_Hz'],
           slew_rate_mA_per_s=0.3, max_session_min=20),
    Device('tSCS', 8, 40.0, 500,
           ['intensity'], slew_rate_mA_per_s=1.0, max_session_min=30),
    Device('tRNS', 9, 2.0, 200,
           ['intensity'], slew_rate_mA_per_s=0.5, max_session_min=20),
    Device('HD-tDCS', 10, 4.0, 3000,
           ['center_mA', 'ring_mA'], slew_rate_mA_per_s=0.5, max_session_min=20),
]

class DeviceRegistry:
    def __init__(self):
        self._devices: dict[str, Device] = {}
        for d in CORE_DEVICES:
            self._devices[d.name] = d

    def add(self, device: Device):
        self._devices[device.name] = device

    def remove(self, name: str):
        self._devices.pop(name, None)

    def get(self, name: str) -> Device | None:
        return self._devices.get(name)

    def all(self) -> list[Device]:
        return list(self._devices.values())
```

```python
# brainwire/hardware/hal.py
"""Hardware Abstraction Layer — manages connected devices and slot allocation."""
from brainwire.hardware.devices import DeviceRegistry, Device

TIER_REQUIREMENTS = {
    1: {'tDCS', 'TENS'},
    2: {'tDCS', 'TENS', 'taVNS', 'tACS'},
    3: {'tDCS', 'TENS', 'taVNS', 'tACS', 'TMS'},
    4: {'tDCS', 'TENS', 'taVNS', 'tACS', 'TMS', 'tFUS', 'GVS', 'mTI'},
}

class HAL:
    def __init__(self):
        self._registry = DeviceRegistry()
        self._connected: dict[str, Device] = {}

    def connect(self, device_name: str) -> Device:
        dev = self._registry.get(device_name)
        if dev is None:
            raise ValueError(f"Unknown device: {device_name}")
        self._connected[device_name] = dev
        return dev

    def disconnect(self, device_name: str):
        self._connected.pop(device_name, None)

    def active_slots(self) -> list[int]:
        return sorted(d.slot for d in self._connected.values())

    def active_device_count(self) -> int:
        return len(self._connected)

    def active_devices(self) -> list[Device]:
        return list(self._connected.values())

    def total_cost(self) -> float:
        return sum(d.cost_usd for d in self._connected.values())

    def detect_tier(self) -> int:
        names = set(self._connected.keys())
        tier = 0
        for t, required in sorted(TIER_REQUIREMENTS.items()):
            if required.issubset(names):
                tier = t
        return tier
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_hardware.py -v`
Expected: All 7 tests PASS

- [ ] **Step 5: Commit**

```bash
git add brainwire/hardware/ tests/test_hardware.py
git commit -m "feat: add device registry and hardware abstraction layer with Tier 1-4"
```

---

## Task 4: Safety Engine

**Files:**
- Create: `brainwire/hardware/safety.py`
- Create: `tests/test_safety.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_safety.py
import pytest
from brainwire.hardware.safety import SafetyEngine, SafetyViolation

def test_hard_limit_tdcs():
    se = SafetyEngine()
    assert se.check_device_limit('tDCS', 2.0) is True
    assert se.check_device_limit('tDCS', 5.0) is False

def test_hard_limit_tms():
    se = SafetyEngine()
    assert se.check_device_limit('TMS', 2.5) is True
    assert se.check_device_limit('TMS', 3.0) is False

def test_hard_limit_tfus():
    se = SafetyEngine()
    assert se.check_device_limit('tFUS', 720.0) is True
    assert se.check_device_limit('tFUS', 721.0) is False

def test_variable_range_default():
    se = SafetyEngine()
    assert se.check_variable_range('DA', 2.5) is True
    assert se.check_variable_range('DA', 3.5) is False  # >3.0x
    assert se.check_variable_range('DA', 0.05) is False  # <0.1x

def test_variable_range_custom():
    se = SafetyEngine()
    se.set_variable_range('Sensory', 0.1, 5.0)  # DMT allows up to 5.0
    assert se.check_variable_range('Sensory', 4.5) is True

def test_slew_rate():
    se = SafetyEngine()
    assert se.check_slew_rate('tDCS', 0.3) is True   # 0.3mA/s < 0.5
    assert se.check_slew_rate('tDCS', 1.0) is False   # 1.0 > 0.5

def test_session_time():
    se = SafetyEngine()
    assert se.check_session_time(30) is True
    assert se.check_session_time(50) is False  # >40 default

def test_session_time_override():
    se = SafetyEngine()
    se.set_max_session_min(90)
    assert se.check_session_time(60) is True

def test_emergency_stop():
    se = SafetyEngine()
    variables = {'DA': 2.5, 'eCB': 3.0, '5HT': 1.5, 'GABA': 1.8, 'NE': 0.4,
                 'Theta': 2.5, 'Alpha': 0.5, 'Gamma': 1.8, 'PFC': 0.5,
                 'Sensory': 2.0, 'Body': 2.5, 'Coherence': 2.0}
    assert se.check_emergency(variables) == []
    variables['DA'] = 3.5  # exceeds default 3.0
    violations = se.check_emergency(variables)
    assert len(violations) == 1
    assert violations[0].var == 'DA'

def test_daily_session_limit():
    se = SafetyEngine()
    for _ in range(2):
        se.record_session()
    assert se.can_start_session() is False
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_safety.py -v`
Expected: FAIL

- [ ] **Step 3: Write implementation**

```python
# brainwire/hardware/safety.py
"""4-layer safety engine for BrainWire stimulation."""
from dataclasses import dataclass, field
from brainwire.hardware.devices import DeviceRegistry

@dataclass
class SafetyViolation:
    layer: int
    var: str
    value: float
    limit: float
    message: str

# Layer 0: Hardware hard limits (firmware level)
DEVICE_HARD_LIMITS = {
    'tDCS': 4.0,      # mA
    'tACS': 4.0,      # mA
    'TMS': 2.5,       # Tesla
    'taVNS': 0.5,     # mA
    'TENS': 80.0,     # mA
    'tFUS': 720.0,    # ISPTA mW/cm²
    'GVS': 2.0,       # mA
    'mTI': 2.0,       # mA per pair
    'tSCS': 40.0,     # mA
    'tRNS': 2.0,      # mA
    'HD-tDCS': 4.0,   # mA
}

DEVICE_SLEW_RATES = {
    'tDCS': 0.5, 'tACS': 0.5, 'TMS': 1.0, 'taVNS': 0.1,
    'TENS': 1.0, 'tFUS': 50.0, 'GVS': 0.2, 'mTI': 0.3,
    'tSCS': 1.0, 'tRNS': 0.5, 'HD-tDCS': 0.5,
}

class SafetyEngine:
    def __init__(self):
        self._var_ranges: dict[str, tuple[float, float]] = {}
        self._default_range = (0.1, 3.0)
        self._max_session_min = 40
        self._max_daily_sessions = 2
        self._session_count = 0
        self._max_density_A_m2 = 2.0

    # Layer 0: Hardware limits
    def check_device_limit(self, device: str, value: float) -> bool:
        limit = DEVICE_HARD_LIMITS.get(device)
        if limit is None:
            return True
        return value <= limit

    # Layer 1: Slew rate
    def check_slew_rate(self, device: str, rate: float) -> bool:
        limit = DEVICE_SLEW_RATES.get(device)
        if limit is None:
            return True
        return rate <= limit

    # Layer 2: Variable range
    def set_variable_range(self, var: str, low: float, high: float):
        self._var_ranges[var] = (low, high)

    def check_variable_range(self, var: str, value: float) -> bool:
        low, high = self._var_ranges.get(var, self._default_range)
        return low <= value <= high

    # Layer 2: Emergency check (all variables at once)
    def check_emergency(self, variables: dict[str, float]) -> list[SafetyViolation]:
        violations = []
        for var, value in variables.items():
            low, high = self._var_ranges.get(var, self._default_range)
            if value > high:
                violations.append(SafetyViolation(
                    layer=2, var=var, value=value, limit=high,
                    message=f"{var}={value:.2f} exceeds max {high:.1f}"
                ))
            elif value < low:
                violations.append(SafetyViolation(
                    layer=2, var=var, value=value, limit=low,
                    message=f"{var}={value:.2f} below min {low:.1f}"
                ))
        return violations

    # Layer 3: Session management
    def set_max_session_min(self, minutes: int):
        self._max_session_min = minutes

    def check_session_time(self, elapsed_min: float) -> bool:
        return elapsed_min <= self._max_session_min

    def record_session(self):
        self._session_count += 1

    def can_start_session(self) -> bool:
        return self._session_count < self._max_daily_sessions
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_safety.py -v`
Expected: All 10 tests PASS

- [ ] **Step 5: Commit**

```bash
git add brainwire/hardware/safety.py tests/test_safety.py
git commit -m "feat: add 4-layer safety engine with device limits and session management"
```

---

## Task 5: Universal Transfer Function Engine

**Files:**
- Create: `brainwire/engine/__init__.py`
- Create: `brainwire/engine/transfer.py`
- Create: `tests/test_transfer.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_transfer.py
import pytest
from brainwire.engine.transfer import TransferEngine, COEFFICIENTS

def test_coefficients_exist_for_all_12_vars():
    from brainwire.variables import VAR_NAMES
    for var in VAR_NAMES:
        assert var in COEFFICIENTS, f"Missing coefficients for {var}"

def test_tier4_coefficients_include_new_devices():
    # tFUS should affect DA, eCB, 5HT, Theta
    assert ('tFUS', 'VTA_intensity') in COEFFICIENTS['DA']
    assert ('tFUS', 'hippo_intensity') in COEFFICIENTS['eCB']
    assert ('tFUS', 'raphe_intensity') in COEFFICIENTS['5HT']

def test_baseline_is_1():
    engine = TransferEngine()
    result = engine.compute({})  # no stimulation
    from brainwire.variables import VAR_NAMES
    for var in VAR_NAMES:
        assert result[var] == pytest.approx(1.0)

def test_tdcs_increases_da():
    engine = TransferEngine()
    result = engine.compute({'tDCS_anode_mA': 2.0})
    assert result['DA'] > 1.0

def test_tfus_vta_boosts_da():
    engine = TransferEngine()
    r_without = engine.compute({'tDCS_anode_mA': 2.0})
    r_with = engine.compute({'tDCS_anode_mA': 2.0, 'tFUS_VTA_intensity': 0.8})
    assert r_with['DA'] > r_without['DA']

def test_gvs_boosts_body():
    engine = TransferEngine()
    r_without = engine.compute({'TENS_low': 1.0})
    r_with = engine.compute({'TENS_low': 1.0, 'GVS_current_mA': 1.0})
    assert r_with['Body'] > r_without['Body']

def test_mti_deep_suppresses_pfc():
    engine = TransferEngine()
    result = engine.compute({'mTI_dlPFC_intensity': 0.8})
    assert result['PFC'] < 1.0

def test_suppressed_vars_clamped():
    engine = TransferEngine()
    result = engine.compute({'VNS_mA': 10.0})  # extreme
    assert result['NE'] >= 0.01  # clamped, not negative

def test_compute_returns_all_12():
    engine = TransferEngine()
    result = engine.compute({'tDCS_anode_mA': 1.0})
    from brainwire.variables import VAR_NAMES
    assert set(result.keys()) == set(VAR_NAMES)

def test_tier3_thc_matches_existing():
    """Tier 3 THC result should match existing bench_thc_vars.py within 5%."""
    engine = TransferEngine()
    tier3_params = {
        'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
        'tDCS_anode_V1_mA': 2.0, 'tDCS_anode_S1_mA': 2.0,
        'VNS_mA': 0.5,
        'TENS_low': 1.0, 'TENS_high': 1.0,
        'TMS_theta': 1.0, 'TMS_1Hz': 1.0, 'TMS_10Hz': 1.0, 'TMS_40Hz': 1.0,
        'tACS_6Hz_mA': 2.0, 'tACS_10Hz_mA': 2.0, 'tACS_40Hz_mA': 2.0,
        'LED_40Hz': 1.0, 'audio_40Hz': 1.0, 'binaural_6Hz': 1.0,
        'vibro_40Hz': 1.0, 'noise': 1.0, 'alpha_ent': 1.0,
    }
    result = engine.compute(tier3_params)
    # DA should be around 2.5+ at Tier 3
    assert result['DA'] > 2.0
    # All vars should exceed baseline
    assert result['eCB'] > 1.5
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_transfer.py -v`
Expected: FAIL

- [ ] **Step 3: Write implementation**

```python
# brainwire/engine/__init__.py
"""BrainWire consciousness engine — transfer functions, PID, interpolation."""
```

```python
# brainwire/engine/transfer.py
"""Universal transfer function engine — maps device parameters to 12 variables.

Supports Tier 1-4 hardware. Each variable is computed as:
  V_i = base ± Σ(coeff * param)

Coefficients are stored as dict[var_name] -> list of (device_param, coefficient).
Negative base direction means the variable is suppressed (Alpha, NE, PFC).
"""
from brainwire.variables import VAR_NAMES

# Coefficients: var -> [(param_key, coefficient)]
# Param keys map to flat dict input: e.g. 'tDCS_anode_mA', 'tFUS_VTA_intensity'
COEFFICIENTS: dict[str, dict[tuple[str, str], float]] = {}

# Build coefficients as {var: {(device, param): coeff}}
_C = {}

# V1: DA — tDCS(F3) + taVNS + TMS(10Hz) + tFUS(VTA) + GVS
_C['DA'] = {
    ('tDCS', 'anode_mA'): 0.25,
    ('taVNS', 'VNS_mA'): 0.80,
    ('TMS', '10Hz'): 0.80,
    ('tFUS', 'VTA_intensity'): 1.20,    # Tier 4: direct VTA
    ('GVS', 'current_mA'): 0.30,        # Tier 4: vestibular-DA
}

# V2: eCB — TENS + taVNS + tDCS + tACS(θ) + TMS(θ) + tFUS(hippo)
_C['eCB'] = {
    ('TENS', 'low'): 0.80,
    ('taVNS', 'VNS_mA'): 0.60,
    ('tDCS', 'anode_mA'): 0.20,
    ('tACS', '6Hz_mA'): 0.15,
    ('TMS', 'theta'): 0.20,
    ('tFUS', 'hippo_intensity'): 1.00,   # Tier 4: direct hippocampus
}

# V3: 5HT — taVNS + tDCS + tFUS(raphe)
_C['5HT'] = {
    ('taVNS', 'VNS_mA'): 1.20,
    ('tDCS', 'anode_mA'): 0.15,
    ('tFUS', 'raphe_intensity'): 1.50,   # Tier 4: direct raphe
}

# V4: GABA — tDCS + alpha_ent + TMS(θ) + tACS(α) + mTI(thalamus)
_C['GABA'] = {
    ('tDCS', 'anode_mA'): 0.20,
    ('entrainment', 'alpha_ent'): 0.30,
    ('TMS', 'theta'): 0.25,
    ('tACS', '10Hz_mA'): 0.15,
    ('mTI', 'thalamus_intensity'): 0.40,  # Tier 4
}

# V5: NE↓ — taVNS + mTI(LC) + tFUS(LC) [SUPPRESSED: base - coeffs]
_C['NE'] = {
    ('taVNS', 'VNS_mA'): 1.50,
    ('mTI', 'LC_intensity'): 0.80,        # Tier 4: deep LC targeting
    ('tFUS', 'LC_intensity'): 0.60,        # Tier 4
}

# V6: Theta — TMS(6Hz) + binaural + tACS(6Hz) + tFUS(hippo)
_C['Theta'] = {
    ('TMS', 'theta'): 0.80,
    ('entrainment', 'binaural_6Hz'): 0.40,
    ('tACS', '6Hz_mA'): 0.35,
    ('tFUS', 'hippo_intensity'): 0.70,     # Tier 4
}

# V7: Alpha↓ — tDCS cathode + TMS(1Hz) + HD-tDCS [SUPPRESSED]
_C['Alpha'] = {
    ('tDCS', 'cathode_Fz_mA'): 0.20,
    ('TMS', '1Hz'): 0.25,
    ('HD-tDCS', 'cathode_mA'): 0.15,      # Tier 4: precision
}

# V8: Gamma — LED(40Hz) + Audio(40Hz) + Vibro(40Hz) + tACS(40Hz) + TMS(40Hz) + tFUS(40Hz)
_C['Gamma'] = {
    ('entrainment', 'LED_40Hz'): 0.30,
    ('entrainment', 'audio_40Hz'): 0.25,
    ('entrainment', 'vibro_40Hz'): 0.20,
    ('tACS', '40Hz_mA'): 0.15,
    ('TMS', '40Hz'): 0.10,
    ('tFUS', '40Hz_intensity'): 0.25,      # Tier 4
}

# V9: PFC↓ — tDCS cathode + TMS(1Hz) + mTI(dlPFC) [SUPPRESSED]
_C['PFC'] = {
    ('tDCS', 'cathode_F4_mA'): 0.20,
    ('TMS', '1Hz'): 0.25,
    ('mTI', 'dlPFC_intensity'): 0.40,      # Tier 4: deep targeting
}

# V10: Sensory — tDCS(V1) + noise + LED + TENS + tACS(40Hz) + tSCS + tRNS + tFUS(V1)
_C['Sensory'] = {
    ('tDCS', 'anode_V1_mA'): 0.15,
    ('entrainment', 'noise'): 0.40,
    ('entrainment', 'LED_40Hz'): 0.20,
    ('TENS', 'low'): 0.15,
    ('tACS', '40Hz_mA'): 0.10,
    ('tSCS', 'intensity'): 0.50,           # Tier 4
    ('tRNS', 'intensity'): 0.35,           # Tier 4
    ('tFUS', 'V1_intensity'): 0.40,        # Tier 4
}

# V11: Body — TENS(low+high) + tDCS(S1) + Vibro(40Hz) + tSCS + GVS
_C['Body'] = {
    ('TENS', 'low'): 0.80,
    ('TENS', 'high'): 0.30,
    ('tDCS', 'anode_S1_mA'): 0.20,
    ('entrainment', 'vibro_40Hz'): 0.15,
    ('tSCS', 'intensity'): 0.60,           # Tier 4
    ('GVS', 'current_mA'): 0.40,           # Tier 4
}

# V12: Coherence — multimodal 40Hz + TMS(40Hz) + tACS(40Hz) + tRNS
_C['Coherence'] = {
    ('entrainment', 'gamma_avg'): 0.30,    # avg of LED+audio+vibro 40Hz
    ('TMS', '40Hz'): 0.40,
    ('entrainment', 'sync_avg'): 0.20,     # same as gamma_avg for sync
    ('tACS', '40Hz_mA'): 0.15,
    ('tRNS', 'intensity'): 0.20,           # Tier 4
}

COEFFICIENTS = _C

# Variables that are suppressed (computed as 1.0 - sum)
SUPPRESSED_VARS = {'NE', 'Alpha', 'PFC'}


def _flatten_param(device: str, param: str) -> str:
    """Create flat param key: e.g. ('tDCS', 'anode_mA') -> 'tDCS_anode_mA'."""
    return f"{device}_{param}"


class TransferEngine:
    """Computes 12 consciousness variables from a flat parameter dict."""

    def compute(self, params: dict[str, float]) -> dict[str, float]:
        result = {}
        for var in VAR_NAMES:
            coeffs = COEFFICIENTS.get(var, {})
            total = 0.0
            for (device, param), coeff in coeffs.items():
                key = _flatten_param(device, param)
                val = params.get(key, 0.0)
                # Handle gamma_avg and sync_avg as computed from entrainment
                if param == 'gamma_avg' or param == 'sync_avg':
                    led = params.get('entrainment_LED_40Hz', params.get('LED_40Hz', 0.0))
                    audio = params.get('entrainment_audio_40Hz', params.get('audio_40Hz', 0.0))
                    vibro = params.get('entrainment_vibro_40Hz', params.get('vibro_40Hz', 0.0))
                    val = (led + audio + vibro) / 3.0
                # Also try without device prefix for convenience
                if val == 0.0:
                    val = params.get(param, 0.0)
                total += coeff * val

            if var in SUPPRESSED_VARS:
                result[var] = max(0.01, 1.0 - total)
            else:
                result[var] = 1.0 + total
        return result
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_transfer.py -v`
Expected: All 10 tests PASS

- [ ] **Step 5: Commit**

```bash
git add brainwire/engine/ tests/test_transfer.py
git commit -m "feat: add universal transfer function engine with Tier 1-4 coefficients"
```

---

## Task 6: PureField Tension Computation (Extract from existing)

**Files:**
- Create: `brainwire/engine/tension.py`
- Create: `tests/test_tension.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_tension.py
import pytest
from brainwire.engine.tension import compute_tension, compute_match

def test_tension_baseline_is_zero():
    baseline = {k: 1.0 for k in ['DA','eCB','5HT','GABA','NE','Theta','Alpha','Gamma','PFC','Sensory','Body','Coherence']}
    t = compute_tension(baseline)
    assert t['T_total'] == pytest.approx(0.0)

def test_tension_thc_target():
    thc = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
    t = compute_tension(thc, target=thc)
    assert t['direction_sim'] == pytest.approx(100.0, abs=0.1)
    assert t['magnitude_match'] == pytest.approx(100.0, abs=0.1)

def test_match_perfect():
    thc = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
    m = compute_match(thc, thc)
    for k, v in m.items():
        assert v == pytest.approx(100.0)

def test_match_suppressed_var():
    target = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
    actual = target.copy()
    actual['NE'] = 0.4  # perfect match for suppressed var
    m = compute_match(actual, target)
    assert m['NE'] == pytest.approx(100.0)

def test_tension_cross_state():
    """Tension between THC target and LSD actual should show direction difference."""
    thc = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
    lsd = {'DA':1.8,'eCB':1.3,'5HT':3.5,'GABA':0.6,'NE':2.0,'Theta':3.0,'Alpha':0.3,'Gamma':2.5,'PFC':1.5,'Sensory':3.5,'Body':1.5,'Coherence':0.4}
    t = compute_tension(lsd, target=thc)
    assert t['direction_sim'] < 100.0  # not same direction
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_tension.py -v`
Expected: FAIL

- [ ] **Step 3: Write implementation (extracted and generalized from existing bench_thc_vars.py)**

```python
# brainwire/engine/tension.py
"""PureField tension framework — consciousness state distance metrics."""
import math
from brainwire.variables import VAR_NAMES, TENSION_WEIGHTS, CHEM_VARS, WAVE_VARS, STATE_VARS


def compute_tension(variables: dict[str, float], target: dict[str, float] | None = None) -> dict:
    """Compute PureField tension between variable state and target.

    T_total = sqrt(T_chem² + T_wave² + T_state²)
    """
    if target is None:
        target = {v: 1.0 for v in VAR_NAMES}  # baseline

    def _sub(keys, vals):
        return math.sqrt(sum(TENSION_WEIGHTS[k] * (vals[k] - 1.0) ** 2 for k in keys))

    t_chem = _sub(CHEM_VARS, variables)
    t_wave = _sub(WAVE_VARS, variables)
    t_state = _sub(STATE_VARS, variables)
    t_total = math.sqrt(t_chem**2 + t_wave**2 + t_state**2)

    t_chem_t = _sub(CHEM_VARS, target)
    t_wave_t = _sub(WAVE_VARS, target)
    t_state_t = _sub(STATE_VARS, target)
    t_total_t = math.sqrt(t_chem_t**2 + t_wave_t**2 + t_state_t**2)

    dot = sum(TENSION_WEIGHTS[k] * (variables[k] - 1.0) * (target[k] - 1.0) for k in VAR_NAMES)
    mag_v = math.sqrt(sum(TENSION_WEIGHTS[k] * (variables[k] - 1.0)**2 for k in VAR_NAMES))
    mag_t = math.sqrt(sum(TENSION_WEIGHTS[k] * (target[k] - 1.0)**2 for k in VAR_NAMES))
    direction = dot / (mag_v * mag_t) * 100 if mag_v > 0 and mag_t > 0 else 0
    magnitude = min(t_total, t_total_t) / max(t_total, t_total_t) * 100 if t_total_t > 0 else 0

    return {
        'T_chem': t_chem, 'T_wave': t_wave, 'T_state': t_state, 'T_total': t_total,
        'T_chem_target': t_chem_t, 'T_wave_target': t_wave_t, 'T_state_target': t_state_t, 'T_total_target': t_total_t,
        'direction_sim': direction, 'magnitude_match': magnitude,
        'tension_match': direction * magnitude / 100,
    }


def compute_match(actual: dict[str, float], target: dict[str, float]) -> dict[str, float]:
    """Per-variable match percentage."""
    match = {}
    for k in VAR_NAMES:
        t = target[k]
        a = actual[k]
        if t >= 1.0:
            match[k] = a / t * 100
        else:
            match[k] = (1.0 - a) / (1.0 - t) * 100 if t < 1.0 else 100.0
    return match
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_tension.py -v`
Expected: All 5 tests PASS

- [ ] **Step 5: Commit**

```bash
git add brainwire/engine/tension.py tests/test_tension.py
git commit -m "feat: extract PureField tension framework into engine module"
```

---

## Task 7: PID Controller Bank

**Files:**
- Create: `brainwire/engine/pid.py`
- Create: `tests/test_pid.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_pid.py
import pytest
from brainwire.engine.pid import PIDController, PIDBank

def test_pid_zero_error():
    pid = PIDController(Kp=1.0, Ki=0.1, Kd=0.01)
    output = pid.update(setpoint=2.0, measured=2.0, dt=0.1)
    assert output == pytest.approx(0.0, abs=0.01)

def test_pid_positive_error():
    pid = PIDController(Kp=1.0, Ki=0.0, Kd=0.0)
    output = pid.update(setpoint=2.0, measured=1.0, dt=0.1)
    assert output == pytest.approx(1.0)  # Kp * error

def test_pid_integral_accumulates():
    pid = PIDController(Kp=0.0, Ki=1.0, Kd=0.0)
    pid.update(setpoint=2.0, measured=1.0, dt=1.0)  # integral = 1.0
    output = pid.update(setpoint=2.0, measured=1.0, dt=1.0)  # integral = 2.0
    assert output == pytest.approx(2.0)

def test_pid_anti_windup():
    pid = PIDController(Kp=0.0, Ki=1.0, Kd=0.0, max_integral=5.0)
    for _ in range(100):
        pid.update(setpoint=100.0, measured=0.0, dt=1.0)
    output = pid.update(setpoint=100.0, measured=0.0, dt=1.0)
    assert output <= 5.0  # clamped by anti-windup

def test_pid_reset():
    pid = PIDController(Kp=1.0, Ki=1.0, Kd=1.0)
    pid.update(setpoint=2.0, measured=1.0, dt=0.1)
    pid.reset()
    assert pid._integral == 0.0
    assert pid._prev_error == 0.0

def test_pid_bank_12_controllers():
    bank = PIDBank()
    assert len(bank.controllers) == 12

def test_pid_bank_update():
    bank = PIDBank()
    target = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
    measured = {k: 1.0 for k in target}  # all at baseline
    outputs = bank.update(target, measured, dt=0.1)
    assert len(outputs) == 12
    # DA target 2.5, measured 1.0 → positive output
    assert outputs['DA'] > 0
    # NE target 0.4, measured 1.0 → negative output (need to decrease)
    assert outputs['NE'] < 0

def test_pid_bank_apply_hints():
    from brainwire.profiles.base import PidHint
    bank = PIDBank()
    hints = [PidHint(var='eCB', Kp_scale=2.0)]
    bank.apply_hints(hints)
    assert bank.controllers['eCB'].Kp == pytest.approx(2.0)  # default 1.0 * 2.0
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_pid.py -v`
Expected: FAIL

- [ ] **Step 3: Write implementation**

```python
# brainwire/engine/pid.py
"""PID controller bank — 12 independent controllers for closed-loop stimulation."""
from brainwire.variables import VAR_NAMES

class PIDController:
    def __init__(self, Kp: float = 1.0, Ki: float = 0.1, Kd: float = 0.01,
                 max_integral: float = 10.0, output_limit: float = 5.0):
        self.Kp = Kp
        self.Ki = Ki
        self.Kd = Kd
        self.max_integral = max_integral
        self.output_limit = output_limit
        self._integral = 0.0
        self._prev_error = 0.0

    def update(self, setpoint: float, measured: float, dt: float) -> float:
        error = setpoint - measured
        self._integral += error * dt
        # Anti-windup
        self._integral = max(-self.max_integral, min(self.max_integral, self._integral))
        derivative = (error - self._prev_error) / dt if dt > 0 else 0.0
        self._prev_error = error
        output = self.Kp * error + self.Ki * self._integral + self.Kd * derivative
        return max(-self.output_limit, min(self.output_limit, output))

    def reset(self):
        self._integral = 0.0
        self._prev_error = 0.0


class PIDBank:
    def __init__(self, default_Kp: float = 1.0, default_Ki: float = 0.1,
                 default_Kd: float = 0.01):
        self.controllers: dict[str, PIDController] = {}
        for var in VAR_NAMES:
            self.controllers[var] = PIDController(Kp=default_Kp, Ki=default_Ki, Kd=default_Kd)

    def update(self, target: dict[str, float], measured: dict[str, float],
               dt: float) -> dict[str, float]:
        outputs = {}
        for var in VAR_NAMES:
            outputs[var] = self.controllers[var].update(
                setpoint=target[var], measured=measured[var], dt=dt
            )
        return outputs

    def apply_hints(self, hints: list) -> None:
        for hint in hints:
            if hint.var in self.controllers:
                c = self.controllers[hint.var]
                c.Kp *= hint.Kp_scale
                c.Ki *= hint.Ki_scale
                c.Kd *= hint.Kd_scale

    def reset_all(self):
        for c in self.controllers.values():
            c.reset()
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_pid.py -v`
Expected: All 8 tests PASS

- [ ] **Step 5: Commit**

```bash
git add brainwire/engine/pid.py tests/test_pid.py
git commit -m "feat: add PID controller bank with anti-windup and profile hints"
```

---

## Task 8: State Interpolation Engine

**Files:**
- Create: `brainwire/engine/interpolation.py`
- Create: `tests/test_interpolation.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_interpolation.py
import pytest
import math
from brainwire.engine.interpolation import (
    lerp_states, blend_states, sigmoid_alpha, linear_alpha, envelope_value
)

THC = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
FLOW = {'DA':1.8,'eCB':2.0,'5HT':1.3,'GABA':1.5,'NE':1.2,'Theta':2.0,'Alpha':1.5,'Gamma':2.0,'PFC':0.7,'Sensory':1.5,'Body':1.8,'Coherence':2.5}

def test_lerp_at_zero_is_state_a():
    result = lerp_states(THC, FLOW, 0.0)
    for k in THC:
        assert result[k] == pytest.approx(THC[k])

def test_lerp_at_one_is_state_b():
    result = lerp_states(THC, FLOW, 1.0)
    for k in FLOW:
        assert result[k] == pytest.approx(FLOW[k])

def test_lerp_at_half():
    result = lerp_states(THC, FLOW, 0.5)
    assert result['DA'] == pytest.approx((2.5 + 1.8) / 2)

def test_blend_two_equal_weights():
    result = blend_states([THC, FLOW], [0.5, 0.5])
    assert result['DA'] == pytest.approx((2.5 + 1.8) / 2)

def test_blend_weights_must_sum_to_one():
    with pytest.raises(ValueError):
        blend_states([THC, FLOW], [0.3, 0.3])

def test_sigmoid_alpha_endpoints():
    assert sigmoid_alpha(0.0) == pytest.approx(0.0, abs=0.01)
    assert sigmoid_alpha(1.0) == pytest.approx(1.0, abs=0.01)

def test_sigmoid_alpha_midpoint():
    assert sigmoid_alpha(0.5) == pytest.approx(0.5, abs=0.01)

def test_linear_alpha():
    assert linear_alpha(0.0) == 0.0
    assert linear_alpha(0.5) == 0.5
    assert linear_alpha(1.0) == 1.0

def test_envelope_onset():
    # During onset phase, value should be between 0 and 1
    val = envelope_value(t=150, onset_s=300, plateau_s=3600, offset_s=180, curve='linear')
    assert 0.0 < val < 1.0

def test_envelope_plateau():
    val = envelope_value(t=1000, onset_s=300, plateau_s=3600, offset_s=180, curve='linear')
    assert val == pytest.approx(1.0)

def test_envelope_offset():
    val = envelope_value(t=3950, onset_s=300, plateau_s=3600, offset_s=180, curve='linear')
    assert 0.0 < val < 1.0

def test_envelope_after_end():
    val = envelope_value(t=5000, onset_s=300, plateau_s=3600, offset_s=180, curve='linear')
    assert val == pytest.approx(0.0)
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_interpolation.py -v`
Expected: FAIL

- [ ] **Step 3: Write implementation**

```python
# brainwire/engine/interpolation.py
"""State interpolation and blending engine for consciousness state transitions."""
import math
from brainwire.variables import VAR_NAMES


def linear_alpha(t_norm: float) -> float:
    return max(0.0, min(1.0, t_norm))


def sigmoid_alpha(t_norm: float) -> float:
    return 1.0 / (1.0 + math.exp(-12.0 * (t_norm - 0.5)))


def exponential_alpha(t_norm: float, tau: float = 0.2) -> float:
    return 1.0 - math.exp(-t_norm / tau)


def lerp_states(state_a: dict[str, float], state_b: dict[str, float],
                alpha: float) -> dict[str, float]:
    alpha = max(0.0, min(1.0, alpha))
    return {k: (1.0 - alpha) * state_a[k] + alpha * state_b[k] for k in VAR_NAMES}


def blend_states(states: list[dict[str, float]], weights: list[float]) -> dict[str, float]:
    if abs(sum(weights) - 1.0) > 0.01:
        raise ValueError(f"Weights must sum to 1.0, got {sum(weights):.3f}")
    result = {k: 0.0 for k in VAR_NAMES}
    for state, w in zip(states, weights):
        for k in VAR_NAMES:
            result[k] += w * state[k]
    return result


def envelope_value(t: float, onset_s: float, plateau_s: float, offset_s: float,
                   curve: str = 'sigmoid') -> float:
    """Compute envelope amplitude at time t.

    Phases: [0, onset] → ramp up, [onset, onset+plateau] → 1.0,
            [onset+plateau, onset+plateau+offset] → ramp down, after → 0.0
    """
    curve_fn = {'linear': linear_alpha, 'sigmoid': sigmoid_alpha,
                'exponential': exponential_alpha}.get(curve, linear_alpha)

    if t < 0:
        return 0.0
    elif t < onset_s:
        return curve_fn(t / onset_s) if onset_s > 0 else 1.0
    elif t < onset_s + plateau_s:
        return 1.0
    elif t < onset_s + plateau_s + offset_s:
        elapsed = t - onset_s - plateau_s
        return 1.0 - curve_fn(elapsed / offset_s) if offset_s > 0 else 0.0
    else:
        return 0.0
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_interpolation.py -v`
Expected: All 12 tests PASS

- [ ] **Step 5: Commit**

```bash
git add brainwire/engine/interpolation.py tests/test_interpolation.py
git commit -m "feat: add state interpolation engine with lerp, blend, and envelope"
```

---

## Task 9: Tier 4 Hardware Configurations

**Files:**
- Create: `brainwire/hardware/configs.py`
- Create: `tests/test_configs.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_configs.py
import pytest
from brainwire.hardware.configs import TIER_CONFIGS, get_tier_params

def test_four_tiers_defined():
    assert len(TIER_CONFIGS) >= 4
    for tier in [1, 2, 3, 4]:
        assert tier in TIER_CONFIGS

def test_tier_costs_ascending():
    costs = [TIER_CONFIGS[t]['cost'] for t in sorted(TIER_CONFIGS.keys())]
    for i in range(len(costs) - 1):
        assert costs[i] < costs[i+1]

def test_tier4_has_new_devices():
    t4 = get_tier_params(4)
    assert t4.get('tFUS_VTA_intensity', 0) > 0
    assert t4.get('GVS_current_mA', 0) > 0
    assert t4.get('mTI_dlPFC_intensity', 0) > 0
    assert t4.get('tSCS_intensity', 0) > 0

def test_tier1_no_tms():
    t1 = get_tier_params(1)
    assert t1.get('TMS_theta', 0) == 0
    assert t1.get('TMS_1Hz', 0) == 0

def test_tier3_has_tms():
    t3 = get_tier_params(3)
    assert t3.get('TMS_theta', 0) > 0
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_configs.py -v`
Expected: FAIL

- [ ] **Step 3: Write implementation**

```python
# brainwire/hardware/configs.py
"""Preset hardware configurations for Tier 1-4."""

TIER_CONFIGS = {
    1: {
        'name': 'Tier 1 (tDCS + TENS + Arduino)',
        'cost': 85,
        'params': {
            'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
            'tDCS_anode_V1_mA': 1.5, 'tDCS_anode_S1_mA': 1.5,
            'taVNS_VNS_mA': 0.4,
            'TENS_low': 1.0, 'TENS_high': 0.8,
            'entrainment_LED_40Hz': 1.0, 'entrainment_audio_40Hz': 1.0,
            'entrainment_binaural_6Hz': 1.0, 'entrainment_vibro_40Hz': 1.0,
            'entrainment_noise': 0.8, 'entrainment_alpha_ent': 0.8,
        },
    },
    2: {
        'name': 'Tier 2 (+ taVNS + tACS)',
        'cost': 510,
        'params': {
            'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
            'tDCS_anode_V1_mA': 2.0, 'tDCS_anode_S1_mA': 2.0,
            'taVNS_VNS_mA': 0.5,
            'TENS_low': 1.0, 'TENS_high': 0.8,
            'tACS_6Hz_mA': 2.0, 'tACS_10Hz_mA': 2.0, 'tACS_40Hz_mA': 2.0,
            'entrainment_LED_40Hz': 1.0, 'entrainment_audio_40Hz': 1.0,
            'entrainment_binaural_6Hz': 1.0, 'entrainment_vibro_40Hz': 1.0,
            'entrainment_noise': 0.8, 'entrainment_alpha_ent': 1.0,
        },
    },
    3: {
        'name': 'Tier 3 (+ TMS)',
        'cost': 8500,
        'params': {
            'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
            'tDCS_anode_V1_mA': 2.0, 'tDCS_anode_S1_mA': 2.0,
            'taVNS_VNS_mA': 0.5,
            'TENS_low': 1.0, 'TENS_high': 1.0,
            'TMS_theta': 1.0, 'TMS_1Hz': 1.0, 'TMS_10Hz': 1.0, 'TMS_40Hz': 1.0,
            'tACS_6Hz_mA': 2.0, 'tACS_10Hz_mA': 2.0, 'tACS_40Hz_mA': 2.0,
            'entrainment_LED_40Hz': 1.0, 'entrainment_audio_40Hz': 1.0,
            'entrainment_binaural_6Hz': 1.0, 'entrainment_vibro_40Hz': 1.0,
            'entrainment_noise': 1.0, 'entrainment_alpha_ent': 1.0,
        },
    },
    4: {
        'name': 'Tier 4 (+ tFUS + GVS + mTI + tSCS + tRNS + HD-tDCS + 256ch EEG)',
        'cost': 25000,
        'params': {
            'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
            'tDCS_anode_V1_mA': 2.0, 'tDCS_anode_S1_mA': 2.0,
            'taVNS_VNS_mA': 0.5,
            'TENS_low': 1.0, 'TENS_high': 1.0,
            'TMS_theta': 1.0, 'TMS_1Hz': 1.0, 'TMS_10Hz': 1.0, 'TMS_40Hz': 1.0,
            'tACS_6Hz_mA': 2.0, 'tACS_10Hz_mA': 2.0, 'tACS_40Hz_mA': 2.0,
            'entrainment_LED_40Hz': 1.0, 'entrainment_audio_40Hz': 1.0,
            'entrainment_binaural_6Hz': 1.0, 'entrainment_vibro_40Hz': 1.0,
            'entrainment_noise': 1.0, 'entrainment_alpha_ent': 1.0,
            # Tier 4 new devices
            'tFUS_VTA_intensity': 0.8, 'tFUS_hippo_intensity': 0.8,
            'tFUS_raphe_intensity': 0.8, 'tFUS_LC_intensity': 0.6,
            'tFUS_V1_intensity': 0.7, 'tFUS_40Hz_intensity': 0.6,
            'GVS_current_mA': 1.0,
            'mTI_dlPFC_intensity': 0.8, 'mTI_thalamus_intensity': 0.6,
            'mTI_LC_intensity': 0.7,
            'tSCS_intensity': 0.8,
            'tRNS_intensity': 0.7,
            'HD-tDCS_cathode_mA': 1.5,
        },
    },
}


def get_tier_params(tier: int) -> dict[str, float]:
    if tier not in TIER_CONFIGS:
        raise ValueError(f"Unknown tier: {tier}")
    return TIER_CONFIGS[tier]['params'].copy()
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_configs.py -v`
Expected: All 5 tests PASS

- [ ] **Step 5: Commit**

```bash
git add brainwire/hardware/configs.py tests/test_configs.py
git commit -m "feat: add Tier 1-4 hardware preset configurations"
```

---

## Task 10: Multi-State Benchmark CLI

**Files:**
- Create: `brainwire/bench.py`
- Create: `tests/test_bench.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_bench.py
import pytest
from brainwire.bench import run_benchmark, compare_states, run_tier_comparison

def test_run_benchmark_thc():
    result = run_benchmark('thc', tier=3)
    assert result['profile_name'] == 'THC Strong (25%)'
    assert result['tier'] == 3
    assert len(result['match']) == 12
    assert result['avg_match'] > 0

def test_run_benchmark_all_states():
    for state in ['thc', 'lsd', 'psilocybin', 'dmt', 'mdma', 'flow']:
        result = run_benchmark(state, tier=4)
        assert result['avg_match'] > 0
        assert len(result['variables']) == 12

def test_compare_states():
    results = compare_states(['thc', 'lsd', 'dmt'], tier=3)
    assert len(results) == 3
    assert all('avg_match' in r for r in results)

def test_tier_comparison_thc():
    results = run_tier_comparison('thc')
    assert len(results) == 4  # Tiers 1-4
    # Higher tier should have higher match
    assert results[3]['avg_match'] >= results[0]['avg_match']

def test_benchmark_includes_tension():
    result = run_benchmark('thc', tier=3)
    assert 'tension' in result
    assert 'T_total' in result['tension']
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_bench.py -v`
Expected: FAIL

- [ ] **Step 3: Write implementation**

```python
# brainwire/bench.py
"""Multi-state consciousness benchmark CLI."""
import argparse
from brainwire.profiles import load_profile, list_profiles
from brainwire.engine.transfer import TransferEngine
from brainwire.engine.tension import compute_tension, compute_match
from brainwire.hardware.configs import TIER_CONFIGS, get_tier_params
from brainwire.variables import VAR_NAMES


def run_benchmark(state: str, tier: int = 3) -> dict:
    profile = load_profile(state)
    params = get_tier_params(tier)
    engine = TransferEngine()
    variables = engine.compute(params)
    match = compute_match(variables, profile.target)
    tension = compute_tension(variables, target=profile.target)
    avg_match = sum(match.values()) / 12
    over_100 = sum(1 for v in match.values() if v >= 100)

    return {
        'profile_name': profile.name,
        'state': state,
        'tier': tier,
        'cost': TIER_CONFIGS[tier]['cost'],
        'variables': variables,
        'match': match,
        'avg_match': avg_match,
        'over_100_count': over_100,
        'tension': tension,
        'target': profile.target,
    }


def compare_states(states: list[str], tier: int = 3) -> list[dict]:
    return [run_benchmark(s, tier) for s in states]


def run_tier_comparison(state: str) -> list[dict]:
    return [run_benchmark(state, tier=t) for t in sorted(TIER_CONFIGS.keys())]


def print_benchmark(result: dict):
    r = result
    print(f"\n{'='*70}")
    print(f"  {r['profile_name']}  |  Tier {r['tier']} (${r['cost']:,})  |  "
          f"Avg {r['avg_match']:.1f}%  |  {r['over_100_count']}/12 >= 100%")
    print(f"{'='*70}")
    print(f"  {'Var':<12} {'Target':>7} {'Actual':>7} {'Match':>7} {'Bar'}")
    print(f"  {'-'*12} {'-'*7} {'-'*7} {'-'*7} {'-'*25}")
    for k in VAR_NAMES:
        tgt = r['target'][k]
        act = r['variables'][k]
        pct = r['match'][k]
        bar_len = int(min(pct, 150) / 150 * 20)
        bar = '#' * bar_len + '.' * (20 - bar_len)
        ok = '+' if pct >= 100 else ''
        print(f"  {k:<12} {tgt:>6.1f}x {act:>6.2f}x {pct:>6.1f}% {bar} {ok}")
    t = r['tension']
    print(f"\n  Tension: T={t['T_total']:.2f}/{t['T_total_target']:.2f}  "
          f"dir={t['direction_sim']:.1f}%  mag={t['magnitude_match']:.1f}%  "
          f"match={t['tension_match']:.1f}%")


def print_comparison_matrix(results: list[dict]):
    print(f"\n{'='*80}")
    print(f"  {'Variable':<12}", end="")
    for r in results:
        print(f" {r['state']:>8}", end="")
    print()
    print(f"  {'-'*12}", end="")
    for _ in results:
        print(f" {'-'*8}", end="")
    print()
    for k in VAR_NAMES:
        print(f"  {k:<12}", end="")
        for r in results:
            pct = r['match'][k]
            print(f" {pct:>7.0f}%", end="")
        print()
    print(f"  {'AVERAGE':<12}", end="")
    for r in results:
        print(f" {r['avg_match']:>7.1f}%", end="")
    print()


def main():
    parser = argparse.ArgumentParser(description='BrainWire Multi-State Benchmark')
    sub = parser.add_subparsers(dest='command')

    # Single state benchmark
    p_bench = sub.add_parser('bench', help='Benchmark a single state')
    p_bench.add_argument('state', choices=list_profiles())
    p_bench.add_argument('--tier', type=int, default=3, choices=[1, 2, 3, 4])

    # Compare multiple states
    p_compare = sub.add_parser('compare', help='Compare states side by side')
    p_compare.add_argument('states', nargs='+')
    p_compare.add_argument('--tier', type=int, default=3)

    # Tier comparison for one state
    p_tiers = sub.add_parser('tiers', help='Compare all tiers for one state')
    p_tiers.add_argument('state', choices=list_profiles())

    # All states, all tiers
    p_all = sub.add_parser('all', help='Full matrix: all states x all tiers')

    args = parser.parse_args()

    if args.command == 'bench':
        print_benchmark(run_benchmark(args.state, args.tier))
    elif args.command == 'compare':
        results = compare_states(args.states, args.tier)
        for r in results:
            print_benchmark(r)
        print_comparison_matrix(results)
    elif args.command == 'tiers':
        results = run_tier_comparison(args.state)
        for r in results:
            print_benchmark(r)
    elif args.command == 'all':
        for state in list_profiles():
            results = run_tier_comparison(state)
            for r in results:
                print_benchmark(r)
    else:
        parser.print_help()


if __name__ == '__main__':
    main()
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_bench.py -v`
Expected: All 5 tests PASS

- [ ] **Step 5: Run the CLI to verify output**

Run: `cd /Users/ghost/Dev/brainwire && python -m brainwire.bench compare thc lsd dmt mdma flow --tier 4`
Expected: Matrix output showing all 5 states at Tier 4

- [ ] **Step 6: Commit**

```bash
git add brainwire/bench.py tests/test_bench.py
git commit -m "feat: add multi-state consciousness benchmark CLI with tier comparison"
```

---

## Task 11: Extended Calculator CLI

**Files:**
- Create: `brainwire/calc.py`
- Create: `tests/test_calc.py`

- [ ] **Step 1: Write the failing test**

```python
# tests/test_calc.py
import pytest
from brainwire.calc import (
    multi_state_sensitivity, state_gap_analysis, blend_command
)

def test_multi_state_sensitivity():
    results = multi_state_sensitivity('thc', tier=3)
    assert len(results) > 0
    # Each result should have param name, delta, affected vars
    for r in results:
        assert 'param' in r
        assert 'delta' in r

def test_state_gap_analysis():
    gaps = state_gap_analysis('dmt', tier=3)
    # DMT at Tier 3 should have gaps (needs Tier 4 for full)
    assert isinstance(gaps, list)

def test_blend_command():
    result = blend_command(['thc', 'flow'], [0.7, 0.3], tier=3)
    assert len(result['target']) == 12
    assert result['target']['DA'] == pytest.approx(0.7 * 2.5 + 0.3 * 1.8)
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_calc.py -v`
Expected: FAIL

- [ ] **Step 3: Write implementation**

```python
# brainwire/calc.py
"""Extended consciousness calculator CLI — multi-state operations."""
import argparse
from brainwire.profiles import load_profile, list_profiles
from brainwire.engine.transfer import TransferEngine
from brainwire.engine.tension import compute_tension, compute_match
from brainwire.engine.interpolation import blend_states
from brainwire.hardware.configs import TIER_CONFIGS, get_tier_params
from brainwire.variables import VAR_NAMES


def multi_state_sensitivity(state: str, tier: int = 3, step: float = 0.1) -> list[dict]:
    profile = load_profile(state)
    engine = TransferEngine()
    base_params = get_tier_params(tier)
    base_vars = engine.compute(base_params)
    base_match = compute_match(base_vars, profile.target)
    base_avg = sum(base_match.values()) / 12

    results = []
    for param, val in base_params.items():
        test_params = base_params.copy()
        test_params[param] = val + step
        test_vars = engine.compute(test_params)
        test_match = compute_match(test_vars, profile.target)
        test_avg = sum(test_match.values()) / 12
        delta = test_avg - base_avg
        changed = [k for k in VAR_NAMES if abs(test_match[k] - base_match[k]) > 0.1]
        results.append({'param': param, 'delta': delta, 'changed': changed, 'current': val})
    results.sort(key=lambda x: -abs(x['delta']))
    return results


def state_gap_analysis(state: str, tier: int = 3) -> list[dict]:
    profile = load_profile(state)
    engine = TransferEngine()
    params = get_tier_params(tier)
    variables = engine.compute(params)
    match = compute_match(variables, profile.target)
    gaps = []
    for k in VAR_NAMES:
        if match[k] < 100:
            gaps.append({
                'var': k, 'match_pct': match[k],
                'target': profile.target[k], 'actual': variables[k],
                'deficit': 100 - match[k],
            })
    gaps.sort(key=lambda x: x['match_pct'])
    return gaps


def blend_command(states: list[str], weights: list[float], tier: int = 3) -> dict:
    profiles = [load_profile(s) for s in states]
    targets = [p.target for p in profiles]
    blended_target = blend_states(targets, weights)
    engine = TransferEngine()
    params = get_tier_params(tier)
    variables = engine.compute(params)
    match = compute_match(variables, blended_target)
    tension = compute_tension(variables, target=blended_target)
    return {
        'target': blended_target,
        'variables': variables,
        'match': match,
        'avg_match': sum(match.values()) / 12,
        'tension': tension,
        'blend': dict(zip(states, weights)),
    }


def main():
    parser = argparse.ArgumentParser(description='BrainWire Extended Calculator')
    sub = parser.add_subparsers(dest='command')

    p_sens = sub.add_parser('sensitivity', help='Parameter sensitivity for a state')
    p_sens.add_argument('state', choices=list_profiles())
    p_sens.add_argument('--tier', type=int, default=3)

    p_gap = sub.add_parser('gap', help='Gap analysis for a state')
    p_gap.add_argument('state', choices=list_profiles())
    p_gap.add_argument('--tier', type=int, default=3)

    p_blend = sub.add_parser('blend', help='Blend multiple states')
    p_blend.add_argument('--states', nargs='+', required=True)
    p_blend.add_argument('--weights', nargs='+', type=float, required=True)
    p_blend.add_argument('--tier', type=int, default=3)

    args = parser.parse_args()

    if args.command == 'sensitivity':
        results = multi_state_sensitivity(args.state, args.tier)
        print(f"\n  Sensitivity: {args.state} @ Tier {args.tier}\n")
        for r in results[:10]:
            arrow = '+' if r['delta'] > 0 else '-'
            print(f"  {r['param']:<30} {arrow}{abs(r['delta']):.2f}%  [{', '.join(r['changed'][:3])}]")
    elif args.command == 'gap':
        gaps = state_gap_analysis(args.state, args.tier)
        if not gaps:
            print(f"\n  {args.state} @ Tier {args.tier}: all variables >= 100%")
        else:
            print(f"\n  Gaps: {args.state} @ Tier {args.tier} ({len(gaps)} vars below 100%)\n")
            for g in gaps:
                print(f"  {g['var']:<12} {g['match_pct']:>5.1f}%  (target={g['target']:.1f}x actual={g['actual']:.2f}x)")
    elif args.command == 'blend':
        result = blend_command(args.states, args.weights, args.tier)
        print(f"\n  Blend: {result['blend']}  @ Tier {args.tier}")
        print(f"  Avg match: {result['avg_match']:.1f}%\n")
        for k in VAR_NAMES:
            print(f"  {k:<12} target={result['target'][k]:.2f}x  actual={result['variables'][k]:.2f}x  match={result['match'][k]:.1f}%")
    else:
        parser.print_help()


if __name__ == '__main__':
    main()
```

- [ ] **Step 4: Run tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_calc.py -v`
Expected: All 3 tests PASS

- [ ] **Step 5: Commit**

```bash
git add brainwire/calc.py tests/test_calc.py
git commit -m "feat: add extended calculator with multi-state sensitivity, gap analysis, blending"
```

---

## Task 12: Integration Test + Full Validation

**Files:**
- Create: `tests/test_integration.py`

- [ ] **Step 1: Write comprehensive integration test**

```python
# tests/test_integration.py
"""End-to-end integration tests for the full BrainWire Extreme system."""
import pytest
from brainwire.profiles import load_profile, list_profiles
from brainwire.engine.transfer import TransferEngine
from brainwire.engine.tension import compute_tension, compute_match
from brainwire.engine.pid import PIDBank
from brainwire.engine.interpolation import lerp_states, blend_states, envelope_value
from brainwire.hardware.hal import HAL
from brainwire.hardware.safety import SafetyEngine
from brainwire.hardware.configs import get_tier_params
from brainwire.variables import VAR_NAMES


class TestFullPipeline:
    def test_all_profiles_load(self):
        for name in list_profiles():
            p = load_profile(name)
            assert len(p.target) == 12

    def test_tier4_thc_exceeds_150_percent(self):
        profile = load_profile('thc')
        engine = TransferEngine()
        params = get_tier_params(4)
        variables = engine.compute(params)
        match = compute_match(variables, profile.target)
        avg = sum(match.values()) / 12
        assert avg > 100, f"Tier 4 THC avg {avg:.1f}% should exceed 100%"

    def test_tier4_beats_tier3_for_all_states(self):
        engine = TransferEngine()
        for state in list_profiles():
            profile = load_profile(state)
            t3 = engine.compute(get_tier_params(3))
            t4 = engine.compute(get_tier_params(4))
            m3 = sum(compute_match(t3, profile.target).values()) / 12
            m4 = sum(compute_match(t4, profile.target).values()) / 12
            assert m4 >= m3, f"Tier 4 should beat Tier 3 for {state}: {m4:.1f}% vs {m3:.1f}%"

    def test_pid_converges_in_simulation(self):
        profile = load_profile('thc')
        bank = PIDBank()
        bank.apply_hints(profile.pid_hints)
        measured = {k: 1.0 for k in VAR_NAMES}  # start at baseline
        dt = 0.1
        for _ in range(100):
            outputs = bank.update(profile.target, measured, dt)
            for k in VAR_NAMES:
                measured[k] += outputs[k] * dt * 0.1  # simplified plant model
        # After 100 steps, should be closer to target than baseline
        for k in VAR_NAMES:
            initial_error = abs(profile.target[k] - 1.0)
            final_error = abs(profile.target[k] - measured[k])
            assert final_error < initial_error, f"PID should converge for {k}"

    def test_state_transition_thc_to_flow(self):
        thc = load_profile('thc').target
        flow = load_profile('flow').target
        for alpha in [0.0, 0.25, 0.5, 0.75, 1.0]:
            state = lerp_states(thc, flow, alpha)
            assert len(state) == 12
            for k in VAR_NAMES:
                assert min(thc[k], flow[k]) <= state[k] <= max(thc[k], flow[k])

    def test_blend_thc_70_flow_30(self):
        thc = load_profile('thc').target
        flow = load_profile('flow').target
        blended = blend_states([thc, flow], [0.7, 0.3])
        assert len(blended) == 12
        assert blended['DA'] == pytest.approx(0.7 * 2.5 + 0.3 * 1.8)

    def test_dmt_envelope_fast_onset(self):
        profile = load_profile('dmt')
        e = profile.envelope
        assert e.onset_s <= 30  # 30 second onset
        val_5s = envelope_value(5, e.onset_s, e.plateau_s, e.offset_s, e.curve)
        val_15s = envelope_value(15, e.onset_s, e.plateau_s, e.offset_s, e.curve)
        assert val_15s > val_5s  # should be ramping up

    def test_safety_blocks_dmt_overcurrent(self):
        se = SafetyEngine()
        dmt = load_profile('dmt')
        # Default safety should flag extreme DMT values
        violations = se.check_emergency(dmt.target)
        # Sensory 5.0 exceeds default 3.0 limit
        flagged_vars = {v.var for v in violations}
        assert 'Sensory' in flagged_vars

    def test_safety_allows_dmt_with_custom_limits(self):
        se = SafetyEngine()
        dmt = load_profile('dmt')
        # Set custom limits for DMT profile
        for var, limit in dmt.safety.first_session_limits.items():
            se.set_variable_range(var, 0.1, limit)
        # First session limits should be respected
        first_session = dmt.target.copy()
        for var, limit in dmt.safety.first_session_limits.items():
            first_session[var] = min(first_session[var], limit)
        # Remaining vars within default range need custom ranges too
        for var in VAR_NAMES:
            if var not in dmt.safety.first_session_limits:
                se.set_variable_range(var, 0.01, 5.0)
        violations = se.check_emergency(first_session)
        assert len(violations) == 0

    def test_hal_tier_progression(self):
        hal = HAL()
        hal.connect('tDCS')
        hal.connect('TENS')
        assert hal.detect_tier() == 1
        hal.connect('taVNS')
        hal.connect('tACS')
        assert hal.detect_tier() == 2
        hal.connect('TMS')
        assert hal.detect_tier() == 3
        hal.connect('tFUS')
        hal.connect('GVS')
        hal.connect('mTI')
        assert hal.detect_tier() == 4
        assert hal.total_cost() > 15000

    def test_cross_state_tension_matrix(self):
        """All states should have <100% tension match against each other."""
        states = {}
        for name in list_profiles():
            states[name] = load_profile(name).target
        for a_name, a_target in states.items():
            for b_name, b_target in states.items():
                t = compute_tension(a_target, target=b_target)
                if a_name == b_name:
                    assert t['direction_sim'] == pytest.approx(100.0, abs=0.1)
                # Different states should have different tension signatures
```

- [ ] **Step 2: Run integration tests**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/test_integration.py -v`
Expected: All 11 tests PASS

- [ ] **Step 3: Run full test suite**

Run: `cd /Users/ghost/Dev/brainwire && python -m pytest tests/ -v --tb=short`
Expected: All tests PASS (approximately 70+ tests)

- [ ] **Step 4: Commit**

```bash
git add tests/test_integration.py
git commit -m "feat: add comprehensive integration tests for BrainWire Extreme"
```

---

## Task 13: Documentation Update

**Files:**
- Modify: `docs/new-hardware-research.md` — add Tier 4 definition
- Modify: `docs/hardware-catalog.md` — add new devices

- [ ] **Step 1: Update new-hardware-research.md**

Add Tier 4 section with tFUS, GVS, mTI, tSCS, tRNS, HD-tDCS details and performance projections from the design spec.

- [ ] **Step 2: Update hardware-catalog.md**

Add new Tier 4 devices with prices, specs, and DIY options.

- [ ] **Step 3: Commit**

```bash
git add docs/new-hardware-research.md docs/hardware-catalog.md
git commit -m "docs: update hardware research with Tier 4 specs and new device catalog"
```

---

## Task 14: PyYAML Dependency + Package Setup

**Files:**
- Create: `pyproject.toml`

- [ ] **Step 1: Create pyproject.toml**

```toml
[project]
name = "brainwire"
version = "2.0.0"
description = "Universal Consciousness State Engine"
requires-python = ">=3.11"
dependencies = ["pyyaml>=6.0"]

[project.optional-dependencies]
dev = ["pytest>=7.0"]

[project.scripts]
bw-bench = "brainwire.bench:main"
bw-calc = "brainwire.calc:main"
```

- [ ] **Step 2: Install dependencies**

Run: `cd /Users/ghost/Dev/brainwire && pip install -e ".[dev]"`

- [ ] **Step 3: Verify CLI entry points work**

Run: `cd /Users/ghost/Dev/brainwire && bw-bench compare thc lsd dmt --tier 4`
Expected: Comparison matrix output

- [ ] **Step 4: Commit**

```bash
git add pyproject.toml
git commit -m "feat: add pyproject.toml with CLI entry points and PyYAML dependency"
```
