import pytest
from brainwire.epilepsy_calc import (
    detection_time, pre_ictal_probability, phase_error,
    anti_phase_feasible, stdp_pathway_reduction, shannon_safety
)


def test_rns_detection():
    assert detection_time(4, 300, 10) == pytest.approx(160.0)


def test_n1_detection():
    assert detection_time(1024, 300, 1) == pytest.approx(10.375, abs=0.1)


def test_pre_ictal_rns():
    assert pre_ictal_probability(4) == pytest.approx(0.0394, abs=0.001)


def test_pre_ictal_n1():
    assert pre_ictal_probability(1024) > 0.999


def test_phase_error_n1_10hz():
    assert phase_error(0.001, 10) == pytest.approx(3.6)


def test_anti_phase_n1_10hz():
    assert anti_phase_feasible(0.001, 10) is True


def test_anti_phase_rns_10hz():
    assert anti_phase_feasible(0.160, 10) is False


def test_stdp_30_sessions():
    r = stdp_pathway_reduction(30)
    assert r['reduction_pct'] > 80


def test_shannon_safe():
    s = shannon_safety()
    assert s['safe'] is True
    assert s['margin'] > 1.0
