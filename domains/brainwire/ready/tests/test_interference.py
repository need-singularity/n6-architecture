import pytest
from brainwire.interference import (
    current_density_at_point, compute_interference_map,
    frequency_interference, analyze_tier_interference, distance,
    POSITIONS, ElectrodePosition
)

def test_distance_same_point():
    a = ElectrodePosition('a', 0, 0)
    assert distance(a, a) == 0.0

def test_distance_known():
    a = ElectrodePosition('a', 0, 0)
    b = ElectrodePosition('b', 3, 4)
    assert distance(a, b) == pytest.approx(5.0)

def test_current_density_increases_near_electrode():
    devices = {'tDCS_anode_F3': 2.0}
    f3 = POSITIONS['F3']
    near = current_density_at_point(f3.x, f3.y, devices)
    far = current_density_at_point(0.0, -0.8, devices)
    assert near > far

def test_density_zero_no_devices():
    d = current_density_at_point(0, 0, {})
    assert d == 0.0

def test_interference_map_max_within_head():
    devices = {'tDCS_anode_F3': 2.0, 'taVNS': 0.5}
    m = compute_interference_map(devices, resolution=10)
    assert m['max_density'] > 0
    # Max point should be within unit circle
    x, y = m['max_point']
    assert x*x + y*y <= 1.01

def test_frequency_beat():
    result = frequency_interference([6.0, 40.0])
    assert len(result['beats']) == 1
    assert result['beats'][0]['beat'] == 34.0

def test_frequency_unintended_theta():
    # 6Hz + 10Hz → beat = 4Hz (theta)
    result = frequency_interference([6.0, 10.0])
    bands = [u['band'] for u in result['unintended_bands']]
    assert 'theta' in bands or 'delta' in bands

def test_tier3_interference():
    a = analyze_tier_interference(3)
    assert a['tier'] == 3
    assert len(a['active_devices']) > 0
    assert a['recommendation'] in ('SAFE', 'CAUTION', 'REDUCE_CURRENT')

def test_all_tiers_analyzable():
    for tier in [1, 2, 3, 4]:
        a = analyze_tier_interference(tier)
        assert a['density_map']['max_density'] >= 0
