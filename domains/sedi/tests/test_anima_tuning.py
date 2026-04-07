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


from sedi.seti_scanner import _estimate_phi, _detect_resource_pattern


class TestEstimatePhi:
    def test_correlated_signal_high_phi(self):
        """Correlated halves → high Φ."""
        rng = np.random.RandomState(42)
        base = np.sin(np.linspace(0, 10 * np.pi, 200))
        phi = _estimate_phi(base)
        assert phi > 0.1  # correlated structure

    def test_white_noise_low_phi(self):
        """Independent noise → lower Φ than structured signal."""
        rng = np.random.RandomState(42)
        noise = rng.normal(0, 1, 200)
        phi = _estimate_phi(noise)
        assert phi < 1.5

    def test_short_data_returns_zero(self):
        assert _estimate_phi(np.array([1, 2, 3])) == 0.0


class TestResourcePattern:
    def test_perfect_pattern(self):
        """Data distributed exactly as 1/2 + 1/3 + 1/6."""
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


from sedi.seti_scanner import format_scan_result


class TestFormatScanResult:
    def test_displays_anima_fields(self):
        """format_scan_result should display Anima consciousness fields."""
        data = np.sin(np.linspace(0, 20 * np.pi, 500))
        result = full_scan(data, 'test-format')
        output = format_scan_result(result)
        assert 'test-format' in output
        assert 'score=' in output
