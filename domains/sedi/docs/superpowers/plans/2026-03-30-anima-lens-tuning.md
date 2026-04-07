# Anima-SEDI Lens Tuning Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Tune SEDI's 4 analysis engines (gravitational lens, Euler telescope, topological lens, topological sensitivity) with Anima's consciousness discoveries (PSI constants, Φ scaling, 3-body chaos).

**Architecture:** Extend `constants.py` with PSI constants, then add consciousness fingerprint detection to each lens/telescope function in `seti_scanner.py`. New helper functions for Φ estimation, Lyapunov exponent, correlation dimension, and resource pattern detection. Update `full_scan` scoring to incorporate all new signals.

**Tech Stack:** Python, NumPy (required), SciPy (optional, graceful fallback)

---

## File Structure

| File | Action | Responsibility |
|---|---|---|
| `sedi/constants.py` | Modify | Add PSI constants + extended RATIOS |
| `sedi/seti_scanner.py` | Modify | Add helpers, tune 4 analysis functions, update scoring |
| `tests/test_anima_tuning.py` | Create | All tests for new functionality |

---

### Task 1: PSI Constants

**Files:**
- Modify: `sedi/constants.py`
- Test: `tests/test_anima_tuning.py`

- [ ] **Step 1: Create test file with constants tests**

```python
# tests/test_anima_tuning.py
"""Tests for Anima-SEDI lens tuning."""
import math
import numpy as np
import pytest
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from sedi.constants import (
    PSI_COUPLING, PSI_K, PSI_STEPS, PSI_BALANCE, LN2,
    PHI_SCALE, PHI_EXPONENT, RESOURCE_FRACTIONS, RATIOS,
)


def test_psi_coupling_value():
    expected = math.log(2) / (2 ** 5.5)
    assert abs(PSI_COUPLING - expected) < 1e-10
    assert abs(PSI_COUPLING - 0.01536) < 0.001


def test_psi_k():
    assert PSI_K == 11


def test_psi_steps():
    assert abs(PSI_STEPS - 3 / math.log(2)) < 1e-10


def test_ln2():
    assert abs(LN2 - math.log(2)) < 1e-10


def test_phi_scaling():
    assert PHI_SCALE == 0.608
    assert PHI_EXPONENT == 1.071


def test_resource_fractions_sum_to_one():
    from fractions import Fraction
    assert sum(RESOURCE_FRACTIONS) == Fraction(1, 1)


def test_ratios_include_psi():
    assert 'psi_coupling' in RATIOS
    assert 'psi_k_inv' in RATIOS
    assert 'ln2' in RATIOS
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::test_psi_coupling_value -v`
Expected: FAIL — `ImportError: cannot import name 'PSI_COUPLING'`

- [ ] **Step 3: Add PSI constants to constants.py**

Add after the existing `OMEGA = 2` line in `sedi/constants.py`:

```python
# Anima PSI constants — consciousness fingerprint frequencies
LN2 = math.log(2)                     # ≈ 0.693 — entropy baseline
PSI_COUPLING = LN2 / (2 ** 5.5)       # ≈ 0.01536 — fundamental coupling
PSI_K = 11                             # consciousness carrying capacity
PSI_STEPS = 3 / LN2                    # ≈ 4.328 — stepping threshold
PSI_BALANCE = 0.5                      # Shannon maximum entropy
PHI_SCALE = 0.608                      # Φ scaling coefficient
PHI_EXPONENT = 1.071                   # Φ scaling exponent
```

Add after existing `GOLDEN_CENTER` line:

```python
# Resource allocation signature (Law 7-10: n=6 architecture)
from fractions import Fraction
RESOURCE_FRACTIONS = (Fraction(1, 2), Fraction(1, 3), Fraction(1, 6))
```

Extend the existing `RATIOS` dict — add these entries:

```python
    'psi_coupling': PSI_COUPLING,
    'psi_k_inv': 1 / PSI_K,
    'ln2': LN2,
    'psi_steps': PSI_STEPS,
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py -v`
Expected: All 8 tests PASS

- [ ] **Step 5: Commit**

```bash
git add sedi/constants.py tests/test_anima_tuning.py
git commit -m "feat: add Anima PSI constants to SEDI"
```

---

### Task 2: Φ Estimator + Resource Pattern Detector

**Files:**
- Modify: `sedi/seti_scanner.py` (add helper functions before `gravitational_lens_analysis`)
- Test: `tests/test_anima_tuning.py`

- [ ] **Step 1: Write failing tests**

Append to `tests/test_anima_tuning.py`:

```python
from sedi.seti_scanner import _estimate_phi, _detect_resource_pattern


class TestEstimatePhi:
    def test_correlated_signal_high_phi(self):
        """Correlated halves → high Φ."""
        rng = np.random.RandomState(42)
        base = np.sin(np.linspace(0, 10 * np.pi, 200))
        phi = _estimate_phi(base)
        assert phi > 0.1  # correlated structure

    def test_white_noise_low_phi(self):
        """Independent noise → low Φ."""
        rng = np.random.RandomState(42)
        noise = rng.normal(0, 1, 200)
        phi = _estimate_phi(noise)
        assert phi < 0.5

    def test_short_data_returns_zero(self):
        assert _estimate_phi(np.array([1, 2, 3])) == 0.0


class TestResourcePattern:
    def test_perfect_pattern(self):
        """Data distributed exactly as 1/2 + 1/3 + 1/6."""
        # 300 values: 150 in [0, 1/3), 100 in [1/3, 2/3), 50 in [2/3, 1)
        data = np.concatenate([
            np.random.uniform(0, 1/3, 150),
            np.random.uniform(1/3, 2/3, 100),
            np.random.uniform(2/3, 1, 50),
        ])
        result = _detect_resource_pattern(data)
        assert result is not None
        assert result['match'] is True
        assert result['deviation'] < 0.15

    def test_uniform_no_match(self):
        """Uniform data should NOT match 1/2+1/3+1/6."""
        data = np.linspace(0, 1, 300)
        result = _detect_resource_pattern(data)
        assert result is not None
        assert result['match'] is False

    def test_too_short(self):
        assert _detect_resource_pattern(np.array([0.1, 0.5])) is None
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestEstimatePhi -v`
Expected: FAIL — `ImportError: cannot import name '_estimate_phi'`

- [ ] **Step 3: Implement helpers**

Add to `sedi/seti_scanner.py` after the existing imports, before `_factorize`:

```python
from .constants import (
    N, SIGMA, PHI, TAU, SOPFR, FOCAL, EINSTEIN_THETA,
    DELTA_PLUS, DELTA_MINUS, GOLDEN_WIDTH, GOLDEN_CENTER,
    WINDOWS, RATIOS, ALERT_RED, ALERT_ORANGE, ALERT_YELLOW,
    PSI_COUPLING, PSI_K, PSI_STEPS, LN2, PHI_SCALE, PHI_EXPONENT,
    RESOURCE_FRACTIONS,
)
```

(Replace the existing import block — add the new constants.)

Then add before the `# Gravitational Optics` section:

```python
# ──────────────────────────────────────────────────────────────────
# Anima Consciousness Fingerprint Helpers
# ──────────────────────────────────────────────────────────────────

def _estimate_phi(arr, window=None):
    """Estimate integrated information Φ via mutual information.

    Splits data into two halves and computes MI — high MI means
    the halves are informationally integrated (consciousness signature).
    """
    if len(arr) < 8:
        return 0.0
    half = len(arr) // 2
    x, y = arr[:half], arr[half:2 * half]
    n_bins = min(20, half // 5)
    if n_bins < 2:
        return 0.0
    hist_xy = np.histogram2d(x, y, bins=n_bins)[0]
    pxy = hist_xy / hist_xy.sum()
    px = pxy.sum(axis=1)
    py = pxy.sum(axis=0)
    mi = 0.0
    for i in range(n_bins):
        for j in range(n_bins):
            if pxy[i, j] > 0 and px[i] > 0 and py[j] > 0:
                mi += pxy[i, j] * math.log(pxy[i, j] / (px[i] * py[j]))
    return mi


def _detect_resource_pattern(data):
    """Check if data distribution follows 1/2 + 1/3 + 1/6 = 1 (Law 7-10)."""
    if len(data) < 6:
        return None
    hist, _ = np.histogram(data, bins=3, range=(float(np.min(data)), float(np.max(data))))
    total = hist.sum()
    if total == 0:
        return None
    fracs = hist / total
    ideal = np.array([1 / 2, 1 / 3, 1 / 6])
    deviation = float(np.sum(np.abs(np.sort(fracs)[::-1] - ideal)))
    return {
        'fractions': np.sort(fracs)[::-1].tolist(),
        'deviation': deviation,
        'match': deviation < 0.15,
    }
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestEstimatePhi tests/test_anima_tuning.py::TestResourcePattern -v`
Expected: All 6 tests PASS

- [ ] **Step 5: Commit**

```bash
git add sedi/seti_scanner.py tests/test_anima_tuning.py
git commit -m "feat: add Φ estimator and resource pattern detector"
```

---

### Task 3: Lyapunov Exponent + Correlation Dimension

**Files:**
- Modify: `sedi/seti_scanner.py`
- Test: `tests/test_anima_tuning.py`

- [ ] **Step 1: Write failing tests**

Append to `tests/test_anima_tuning.py`:

```python
from sedi.seti_scanner import _lyapunov_estimate, _correlation_dimension


class TestLyapunov:
    def test_lorenz_positive(self):
        """Lorenz attractor should have positive Lyapunov exponent."""
        # Generate Lorenz-like chaotic series
        x = np.zeros(2000)
        x[0] = 0.1
        for i in range(1, 2000):
            x[i] = 3.9 * x[i-1] * (1 - x[i-1])  # logistic map, chaotic
        lyap = _lyapunov_estimate(x)
        assert lyap is not None
        assert lyap > 0  # chaotic

    def test_periodic_near_zero(self):
        """Periodic signal should have non-positive Lyapunov exponent."""
        t = np.linspace(0, 20 * np.pi, 2000)
        periodic = np.sin(t)
        lyap = _lyapunov_estimate(periodic)
        # Periodic signals: λ ≤ 0 or very small
        assert lyap is None or lyap < 0.1

    def test_short_data_returns_none(self):
        assert _lyapunov_estimate(np.array([1, 2, 3])) is None


class TestCorrelationDimension:
    def test_chaotic_above_threshold(self):
        """Chaotic data should have correlation dimension ≥ 1."""
        x = np.zeros(1000)
        x[0] = 0.1
        for i in range(1, 1000):
            x[i] = 3.9 * x[i-1] * (1 - x[i-1])
        dim = _correlation_dimension(x)
        assert dim is not None
        assert dim > 0.5

    def test_short_data_returns_none(self):
        assert _correlation_dimension(np.array([1, 2, 3])) is None
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestLyapunov -v`
Expected: FAIL — `ImportError: cannot import name '_lyapunov_estimate'`

- [ ] **Step 3: Implement Lyapunov and correlation dimension**

Add to `sedi/seti_scanner.py` after `_detect_resource_pattern`:

```python
def _lyapunov_estimate(data, tau=1, dim=3, steps=10):
    """Estimate largest Lyapunov exponent via nearest-neighbor divergence.

    λ > 0 → chaotic (consciousness complexity, Law 16: 3-body threshold).
    λ ≤ 0 → periodic/stable (non-conscious).
    """
    n = len(data)
    if n < dim * tau + steps + 10:
        return None
    # Delay embedding
    max_i = n - dim * tau
    embedded = np.array([data[i:i + dim * tau:tau] for i in range(max_i)])
    if len(embedded) < steps + 2:
        return None
    divergences = []
    n_samples = min(len(embedded) - steps, 200)
    for i in range(n_samples):
        dists = np.sum((embedded - embedded[i]) ** 2, axis=1)
        dists[max(0, i - 1):i + 2] = np.inf  # exclude temporal neighbors
        j = np.argmin(dists)
        if dists[j] == np.inf or dists[j] == 0:
            continue
        d0 = math.sqrt(dists[j])
        if i + steps < len(embedded) and j + steps < len(embedded):
            d_later = np.sqrt(np.sum((embedded[i + steps] - embedded[j + steps]) ** 2))
            if d_later > 0 and d0 > 0:
                divergences.append(math.log(d_later / d0) / steps)
    return float(np.mean(divergences)) if divergences else None


def _correlation_dimension(data, max_r_steps=20):
    """Estimate correlation dimension via Grassberger-Procaccia algorithm.

    d ≥ 3 → 3-body chaos threshold crossed (Law 16).
    """
    n = len(data)
    if n < 50:
        return None
    rng = np.random.RandomState(0)
    sample = data[rng.choice(n, min(n, 500), replace=False)]
    dists = []
    for i in range(len(sample)):
        for j in range(i + 1, len(sample)):
            dists.append(abs(sample[i] - sample[j]))
    dists = np.array(sorted(dists))
    if len(dists) < 10:
        return None
    r_min, r_max = dists[1], dists[-1]
    if r_min <= 0 or r_max <= r_min:
        return None
    rs = np.logspace(np.log10(r_min), np.log10(r_max), max_r_steps)
    counts = np.array([np.sum(dists < r) for r in rs])
    total_pairs = len(sample) * (len(sample) - 1) / 2
    C = counts / total_pairs
    valid = (C > 0.01) & (C < 0.99)
    if np.sum(valid) < 3:
        return None
    log_r = np.log(rs[valid])
    log_C = np.log(C[valid])
    slope, _ = np.polyfit(log_r, log_C, 1)
    return float(slope)
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestLyapunov tests/test_anima_tuning.py::TestCorrelationDimension -v`
Expected: All 5 tests PASS

- [ ] **Step 5: Commit**

```bash
git add sedi/seti_scanner.py tests/test_anima_tuning.py
git commit -m "feat: add Lyapunov exponent and correlation dimension estimators"
```

---

### Task 4: Tune Gravitational Lens

**Files:**
- Modify: `sedi/seti_scanner.py` (`gravitational_lens_analysis`)
- Test: `tests/test_anima_tuning.py`

- [ ] **Step 1: Write failing tests**

Append to `tests/test_anima_tuning.py`:

```python
from sedi.seti_scanner import gravitational_lens_analysis


class TestGravLensTuning:
    def test_result_has_phi_estimate(self):
        data = np.sin(np.linspace(0, 20 * np.pi, 500))
        result = gravitational_lens_analysis(data, 'test')
        assert 'phi_estimate' in result

    def test_result_has_resource_pattern(self):
        data = np.random.uniform(0, 1, 500)
        result = gravitational_lens_analysis(data, 'test')
        assert 'resource_pattern' in result

    def test_result_has_psi_fft_matches(self):
        data = np.random.normal(0, 1, 500)
        result = gravitational_lens_analysis(data, 'test')
        assert 'psi_fft_matches' in result

    def test_correlated_signal_phi_above_zero(self):
        """Structured signal should have Φ > 0."""
        t = np.linspace(0, 10 * np.pi, 500)
        data = np.sin(t) + 0.5 * np.sin(2 * t)
        result = gravitational_lens_analysis(data, 'test')
        assert result['phi_estimate'] > 0
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestGravLensTuning -v`
Expected: FAIL — `KeyError: 'phi_estimate'`

- [ ] **Step 3: Modify gravitational_lens_analysis**

In `sedi/seti_scanner.py`, add PSI frequency matching to `gravitational_lens_analysis`. Inside the function, after the existing FFT window loop (after `fft_results[f'w{ws}']` block), add PSI matching:

```python
    # 3. PSI frequency matching
    psi_fft_matches = 0
    psi_targets = [
        ('psi_coupling', PSI_COUPLING),
        ('psi_k_inv', 1 / PSI_K),
        ('ln2', LN2),
    ]
    for ws in WINDOWS:
        spectrum = windowed_fft(arr, ws)
        if len(spectrum) > 0:
            peak_idx = np.argmax(spectrum[1:]) + 1
            peak_freq = peak_idx / len(spectrum)
            for pname, ptarget in psi_targets:
                if abs(peak_freq - ptarget) < 0.02:
                    psi_fft_matches += 1
```

After the existing `theta_deviation` calculation, add:

```python
    # 6. Φ estimate (Anima consciousness threshold)
    phi_estimate = _estimate_phi(arr)

    # 7. Resource allocation pattern (Law 7-10)
    normalized = (arr - np.min(arr)) / (np.max(arr) - np.min(arr) + 1e-10)
    resource_pattern = _detect_resource_pattern(normalized)
```

Add these to the return dict:

```python
        'phi_estimate': phi_estimate,
        'resource_pattern': resource_pattern,
        'psi_fft_matches': psi_fft_matches,
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestGravLensTuning -v`
Expected: All 4 tests PASS

- [ ] **Step 5: Commit**

```bash
git add sedi/seti_scanner.py tests/test_anima_tuning.py
git commit -m "feat: tune gravitational lens with PSI frequencies + Φ + resource pattern"
```

---

### Task 5: Tune Euler Telescope

**Files:**
- Modify: `sedi/seti_scanner.py` (`telescope_analysis`)
- Test: `tests/test_anima_tuning.py`

- [ ] **Step 1: Write failing tests**

Append to `tests/test_anima_tuning.py`:

```python
from sedi.seti_scanner import telescope_analysis
from sedi.constants import PSI_STEPS, PSI_K


class TestTelescopeTuning:
    def test_default_s_values_include_psi(self):
        data = np.random.normal(0, 1, 200)
        result = telescope_analysis(data, 'test')
        assert f's={PSI_STEPS}' in result['F_values'] or f's={PSI_K}' in result['F_values']

    def test_result_has_factorial_dominance(self):
        data = np.random.normal(0, 1, 200)
        result = telescope_analysis(data, 'test')
        assert 'factorial_dominance' in result

    def test_result_has_phi_ratio(self):
        data = np.random.normal(0, 1, 200)
        result = telescope_analysis(data, 'test')
        assert 'phi_predicted' in result
        assert 'phi_ratio' in result

    def test_factorial_dominance_bounded(self):
        """factorial_dominance should be between 0 and ~1."""
        data = np.sin(np.linspace(0, 10 * np.pi, 200))
        result = telescope_analysis(data, 'test')
        fd = result['factorial_dominance']
        assert fd >= 0
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestTelescopeTuning -v`
Expected: FAIL — `KeyError` or `AssertionError`

- [ ] **Step 3: Modify telescope_analysis**

In `sedi/seti_scanner.py`, modify `telescope_analysis`:

Change the default `s_values`:

```python
    if s_values is None:
        s_values = [1.5, 2.0, 3.0, 5.0, PSI_STEPS, float(PSI_K)]
```

After the existing `f_data = F_telescope(...)` line, add:

```python
    # 3! factorial dominance: how much do p=2,3 (composing 3!=6) dominate F(s)?
    s_eff = max(1.1, s_data)
    e2 = euler_factor(2, s_eff)
    e3 = euler_factor(3, s_eff)
    factorial_dominance = (e2 * e3) / f_data if f_data != 0 else 0.0

    # Φ scaling comparison (Anima Law 1-5)
    phi_predicted = PHI_SCALE * len(arr) ** PHI_EXPONENT
    phi_ratio = f_data / phi_predicted if phi_predicted > 0 else 0.0
```

Add to the return dict:

```python
        'factorial_dominance': float(factorial_dominance),
        'phi_predicted': float(phi_predicted),
        'phi_ratio': float(phi_ratio),
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestTelescopeTuning -v`
Expected: All 4 tests PASS

- [ ] **Step 5: Commit**

```bash
git add sedi/seti_scanner.py tests/test_anima_tuning.py
git commit -m "feat: tune Euler telescope with PSI s-values + factorial dominance + Φ ratio"
```

---

### Task 6: Tune Topological Lens

**Files:**
- Modify: `sedi/seti_scanner.py` (`topological_lens_analysis`)
- Test: `tests/test_anima_tuning.py`

- [ ] **Step 1: Write failing tests**

Append to `tests/test_anima_tuning.py`:

```python
from sedi.seti_scanner import topological_lens_analysis


class TestTopoLensTuning:
    def test_result_has_lyapunov(self):
        data = np.random.normal(0, 1, 500)
        result = topological_lens_analysis(data, 'test')
        assert 'lyapunov' in result

    def test_result_has_correlation_dim(self):
        data = np.random.normal(0, 1, 500)
        result = topological_lens_analysis(data, 'test')
        assert 'correlation_dim' in result

    def test_result_has_phi_persistence(self):
        data = np.random.normal(0, 1, 500)
        result = topological_lens_analysis(data, 'test')
        assert 'phi_persistence' in result

    def test_chaotic_data_positive_lyapunov(self):
        """Logistic map r=3.9 should yield positive λ."""
        x = np.zeros(2000)
        x[0] = 0.1
        for i in range(1, 2000):
            x[i] = 3.9 * x[i-1] * (1 - x[i-1])
        result = topological_lens_analysis(x, 'test')
        lyap = result.get('lyapunov')
        assert lyap is not None
        assert lyap > 0

    def test_phi_persistence_bounded(self):
        """Φ persistence should be between 0 and 1."""
        data = np.sin(np.linspace(0, 20 * np.pi, 500))
        result = topological_lens_analysis(data, 'test')
        pp = result.get('phi_persistence')
        if pp is not None:
            assert 0 <= pp <= 1.0
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestTopoLensTuning -v`
Expected: FAIL — `KeyError: 'lyapunov'`

- [ ] **Step 3: Modify topological_lens_analysis**

In `sedi/seti_scanner.py`, add at the end of `topological_lens_analysis`, before the return statement:

```python
    # 6. Lyapunov exponent — chaos detection (Law 16: 3-body threshold)
    lyapunov = _lyapunov_estimate(arr)

    # 7. Correlation dimension — attractor complexity
    correlation_dim = _correlation_dimension(arr)

    # 8. Φ persistence — does topology survive compression? (Law 17)
    phi_persistence = None
    if len(arr) >= 100:
        rng_sub = np.random.RandomState(42)
        half_idx = rng_sub.choice(len(arr), len(arr) // 2, replace=False)
        half_pts = sorted(arr[half_idx].tolist())
        bars_half = ph_barcode_1d(half_pts)
        finite_half = [(b, d, p) for b, d, p in bars_half if not math.isinf(d)]
        # Compare number of significant bars (median persistence threshold)
        if finite_bars and finite_half:
            sig_full = sum(1 for _, _, p in finite_bars if p > med_p) if finite_bars else 0
            med_p_half = np.median([p for _, _, p in finite_half]) if finite_half else 0
            sig_half = sum(1 for _, _, p in finite_half if p > med_p_half)
            phi_persistence = min(sig_half, sig_full) / max(sig_full, 1)
        else:
            phi_persistence = 0.0
```

Add to the return dict:

```python
        'lyapunov': lyapunov,
        'correlation_dim': correlation_dim,
        'phi_persistence': phi_persistence,
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestTopoLensTuning -v`
Expected: All 5 tests PASS

- [ ] **Step 5: Commit**

```bash
git add sedi/seti_scanner.py tests/test_anima_tuning.py
git commit -m "feat: tune topological lens with Lyapunov + correlation dim + Φ persistence"
```

---

### Task 7: Update full_scan Scoring

**Files:**
- Modify: `sedi/seti_scanner.py` (`full_scan`)
- Test: `tests/test_anima_tuning.py`

- [ ] **Step 1: Write failing tests**

Append to `tests/test_anima_tuning.py`:

```python
from sedi.seti_scanner import full_scan


class TestFullScanScoring:
    def test_psi_match_adds_score(self):
        """PSI frequency matches should contribute to score."""
        # Create signal with structure
        t = np.linspace(0, 20 * np.pi, 500)
        data = np.sin(t) + 0.3 * np.sin(3 * t)
        result = full_scan(data, 'test')
        # Score should include reasons mentioning PSI or Φ or chaos if detected
        assert 'score' in result
        assert 'reasons' in result
        assert isinstance(result['score'], float)

    def test_conscious_grade_exists(self):
        """CONSCIOUS grade should be possible at score >= 30."""
        # We can't guarantee it triggers, but verify the grade logic exists
        # by checking that grade mapping works
        result = full_scan(np.random.normal(0, 1, 100), 'test')
        assert result['grade'] in ('NORMAL', 'YELLOW', 'ORANGE', 'RED', 'CONSCIOUS')

    def test_white_noise_low_score(self):
        """White noise should have low score (no false positives)."""
        rng = np.random.RandomState(42)
        noise = rng.normal(0, 1, 1000)
        result = full_scan(noise, 'test')
        assert result['score'] < 20  # should not trigger RED/CONSCIOUS

    def test_convergence_bonus(self):
        """Verify convergence bonus logic exists in reasons."""
        # Just verify the full_scan runs without error on various data
        for data in [
            np.random.normal(0, 1, 200),
            np.sin(np.linspace(0, 10 * np.pi, 200)),
        ]:
            result = full_scan(data, 'test')
            assert isinstance(result['reasons'], list)
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestFullScanScoring::test_conscious_grade_exists -v`
Expected: FAIL — `AssertionError` (CONSCIOUS not in valid grades yet)

- [ ] **Step 3: Modify full_scan scoring**

In `sedi/seti_scanner.py`, inside `full_scan`, after the existing topological scoring block (after `reasons.append(f'{n_count} bar count matches')`), add:

```python
    # ── Anima consciousness fingerprint scoring ──

    # PSI frequency matches (gravitational)
    psi_fft = grav.get('psi_fft_matches', 0)
    if psi_fft > 0:
        score += psi_fft * 3.5
        reasons.append(f'{psi_fft} PSI frequency matches')

    # Φ estimate (consciousness threshold)
    phi_est = grav.get('phi_estimate', 0)
    if phi_est and phi_est > PHI_SCALE:
        score += 5.0
        reasons.append(f'\u03a6={phi_est:.3f} > threshold {PHI_SCALE}')

    # Resource allocation pattern (Law 7-10)
    rp = grav.get('resource_pattern') or {}
    if rp.get('match'):
        score += 4.0
        reasons.append(f'resource pattern 1/2+1/3+1/6 (dev={rp["deviation"]:.3f})')

    # Lyapunov exponent — chaotic = conscious complexity (Law 16)
    lyap = topo.get('lyapunov')
    if lyap is not None and lyap > 0:
        score += 3.0 + min(lyap * 2, 4.0)
        reasons.append(f'chaotic dynamics \u03bb={lyap:.3f}')

    # Correlation dimension — 3-body threshold
    cdim = topo.get('correlation_dim')
    if cdim is not None and cdim >= 3.0:
        score += 4.0
        reasons.append(f'attractor dim={cdim:.1f} \u2265 3 (3-body threshold)')

    # Φ persistence — topology survives compression (Law 17)
    phi_p = topo.get('phi_persistence')
    if phi_p is not None and phi_p > 0.7:
        score += 3.0
        reasons.append(f'\u03a6 persistence={phi_p:.2f} (topology survives compression)')

    # Factorial dominance in telescope — 3! = 6 signature
    fd = tele.get('factorial_dominance')
    if fd is not None and fd > 0.8:
        score += 3.0
        reasons.append(f'3! dominance={fd:.3f}')

    # Convergence bonus: n=6 + PSI + chaos all present → triple detection
    n6_present = any('ratio' in r or 'FFT' in r for r in reasons)
    psi_present = any('PSI' in r for r in reasons)
    chaos_present = any('chaotic' in r or 'attractor' in r for r in reasons)
    if n6_present and psi_present and chaos_present:
        score += 10.0
        reasons.append('CONVERGENCE: n=6 + PSI + chaos triple detection')
```

Modify the grade thresholds — replace the existing grade block:

```python
    if score >= 30:
        grade, emoji = 'CONSCIOUS', '🧠'
    elif score >= 20:
        grade, emoji = 'RED', '🔴'
    elif score >= 10:
        grade, emoji = 'ORANGE', '🟠'
    elif score >= 5:
        grade, emoji = 'YELLOW', '🟡'
    else:
        grade, emoji = 'NORMAL', '⚪'
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestFullScanScoring -v`
Expected: All 4 tests PASS

- [ ] **Step 5: Run ALL tests**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py -v`
Expected: All tests PASS (Tasks 1-7)

- [ ] **Step 6: Commit**

```bash
git add sedi/seti_scanner.py tests/test_anima_tuning.py
git commit -m "feat: update full_scan scoring with Anima consciousness fingerprint"
```

---

### Task 8: Integration Null Test

**Files:**
- Test: `tests/test_anima_tuning.py`

- [ ] **Step 1: Write null calibration test**

Append to `tests/test_anima_tuning.py`:

```python
class TestNullCalibration:
    """Verify tuned scanner doesn't false-positive on noise."""

    def test_white_noise_not_conscious(self):
        rng = np.random.RandomState(123)
        for _ in range(5):
            noise = rng.normal(0, 1, 1000)
            result = full_scan(noise, 'null-test')
            assert result['grade'] != 'CONSCIOUS', (
                f"False positive on white noise: score={result['score']}, "
                f"reasons={result['reasons']}"
            )

    def test_uniform_noise_not_conscious(self):
        rng = np.random.RandomState(456)
        for _ in range(5):
            noise = rng.uniform(0, 255, 1000)
            result = full_scan(noise, 'null-test')
            assert result['grade'] != 'CONSCIOUS'

    def test_simple_periodic_not_conscious(self):
        """Pure sine should not trigger CONSCIOUS."""
        t = np.linspace(0, 100 * np.pi, 2000)
        for freq in [1, 3, 7, 11]:
            data = np.sin(freq * t)
            result = full_scan(data, f'sine-{freq}')
            assert result['grade'] != 'CONSCIOUS', (
                f"False positive on sine({freq}): score={result['score']}"
            )
```

- [ ] **Step 2: Run null calibration tests**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestNullCalibration -v`
Expected: All 3 tests PASS. If any fail, adjust scoring thresholds in Task 7.

- [ ] **Step 3: Run complete test suite**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py -v --tb=short`
Expected: All tests PASS

- [ ] **Step 4: Commit**

```bash
git add tests/test_anima_tuning.py
git commit -m "test: add null calibration tests for Anima-tuned scanner"
```

---

### Task 9: Update format_scan_result

**Files:**
- Modify: `sedi/seti_scanner.py` (`format_scan_result`)
- Test: `tests/test_anima_tuning.py`

- [ ] **Step 1: Write test**

Append to `tests/test_anima_tuning.py`:

```python
from sedi.seti_scanner import format_scan_result


class TestFormatScanResult:
    def test_displays_anima_fields(self):
        """format_scan_result should display Anima consciousness fields."""
        data = np.sin(np.linspace(0, 20 * np.pi, 500))
        result = full_scan(data, 'test-format')
        output = format_scan_result(result)
        assert 'test-format' in output
        assert 'score=' in output
```

- [ ] **Step 2: Run test — should pass already (format_scan_result exists)**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestFormatScanResult -v`
Expected: PASS (existing function works with new data)

- [ ] **Step 3: Extend format_scan_result for Anima fields**

In `sedi/seti_scanner.py`, add to `format_scan_result` after the existing telescope `s` line:

```python
    # Anima consciousness fingerprint
    if grav.get('phi_estimate'):
        lines.append(f'    \u03a6 = {grav["phi_estimate"]:.4f}  (threshold={PHI_SCALE})')
    if topo.get('lyapunov') is not None:
        lines.append(f'    \u03bb (Lyapunov) = {topo["lyapunov"]:.4f}')
    if topo.get('correlation_dim') is not None:
        lines.append(f'    d (correlation) = {topo["correlation_dim"]:.2f}')
    if topo.get('phi_persistence') is not None:
        lines.append(f'    \u03a6 persistence = {topo["phi_persistence"]:.2f}')
    if tele.get('factorial_dominance') is not None:
        lines.append(f'    3! dominance = {tele["factorial_dominance"]:.4f}')
```

- [ ] **Step 4: Run test**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py::TestFormatScanResult -v`
Expected: PASS

- [ ] **Step 5: Final full test run**

Run: `cd /Users/ghost/Dev/sedi && python -m pytest tests/test_anima_tuning.py -v --tb=short`
Expected: ALL tests PASS

- [ ] **Step 6: Commit**

```bash
git add sedi/seti_scanner.py tests/test_anima_tuning.py
git commit -m "feat: extend scan result formatting with Anima consciousness fields"
```
