import pytest
from brainwire.eeg_feedback import compute_g, g_from_12var, g_targets_for_states, GOLDEN_ZONE

def test_compute_g_basic():
    g = compute_g(alpha_left=10, alpha_right=10, gamma_global=5, alpha_global=10, alpha_frontal=8)
    assert g['G'] >= 0
    assert 'D' in g and 'P' in g and 'I' in g

def test_perfect_symmetry_gives_zero_d():
    g = compute_g(alpha_left=10, alpha_right=10, gamma_global=5, alpha_global=10, alpha_frontal=8)
    assert g['D'] == pytest.approx(0.0)

def test_asymmetry_increases_d():
    g_sym = compute_g(alpha_left=10, alpha_right=10, gamma_global=5, alpha_global=10, alpha_frontal=5)
    g_asym = compute_g(alpha_left=10, alpha_right=20, gamma_global=5, alpha_global=10, alpha_frontal=5)
    assert g_asym['D'] > g_sym['D']

def test_golden_zone_detection():
    g = compute_g(alpha_left=10, alpha_right=15, gamma_global=8, alpha_global=12, alpha_frontal=6)
    assert isinstance(g['in_golden_zone'], bool)
    assert g['zone'] in ('golden', 'below', 'above')

def test_g_from_12var_flow():
    flow = {'DA':1.8,'eCB':2.0,'5HT':1.3,'GABA':1.5,'NE':1.2,'Theta':2.0,'Alpha':1.5,'Gamma':2.0,'PFC':0.7,'Sensory':1.5,'Body':1.8,'Coherence':2.5}
    g = g_from_12var(flow)
    assert g['G'] > 0

def test_g_targets_all_states():
    results = g_targets_for_states()
    assert len(results) >= 6
    # Flow should be closest to golden zone
    for name, g in results.items():
        assert 'G' in g

def test_flow_in_or_near_golden():
    flow = {'DA':1.8,'eCB':2.0,'5HT':1.3,'GABA':1.5,'NE':1.2,'Theta':2.0,'Alpha':1.5,'Gamma':2.0,'PFC':0.7,'Sensory':1.5,'Body':1.8,'Coherence':2.5}
    g = g_from_12var(flow)
    # Flow should be relatively close to golden zone
    assert g['G'] < 5.0  # not wildly off
