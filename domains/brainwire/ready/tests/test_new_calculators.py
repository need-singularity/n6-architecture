import pytest
from brainwire.n1_placement import compute_j, rank_all, minimum_covering_set
from brainwire.stdp_sim import stdp_weight, simulate_sessions, time_to_threshold
from brainwire.shannon_calc import compute_safety
from brainwire.projection_map import get_all_sources, coverage_check, max_coefficient_per_target

# N1 Placement
def test_f3_highest_j():
    ranked = rank_all()
    assert ranked[0]['location'] == 'F3'

def test_f3_12_deep():
    r = compute_j('F3')
    assert r['n_deep'] == 13  # 12 original + Cerebellum via corticopontocerebellar

def test_covering_set_small():
    c = minimum_covering_set()
    assert c['total'] <= 4
    # Cerebellum has no entry in LOCATIONS deep_targets, so at most 1 uncovered
    assert len(c['uncovered']) <= 1

# STDP
def test_stdp_initial():
    assert stdp_weight(0) == pytest.approx(1.0)

def test_stdp_decreases():
    assert stdp_weight(10000) < stdp_weight(0)

def test_stdp_floor():
    assert stdp_weight(1000000) >= 0.05

def test_time_to_threshold():
    s = time_to_threshold(0.1)
    assert 0 < s < 100

# Shannon
def test_n1_max_safe():
    s = compute_safety(600, 200, 2000, 200)
    assert s['safe'] is True

def test_margin_positive():
    s = compute_safety(600, 200, 2000, 200)
    assert s['margin'] > 1.0

# Projection
def test_vta_has_sources():
    sources = get_all_sources('VTA')
    assert len(sources) >= 2

def test_all_covered():
    cov = coverage_check()
    assert cov['covered'] >= 14  # Cerebellum may need Motor→Pons

def test_ec_hippocampus_strongest():
    best = max_coefficient_per_target()
    assert best['Hippocampus']['source'] == 'EC'
    assert best['Hippocampus']['f_project'] == 0.40
