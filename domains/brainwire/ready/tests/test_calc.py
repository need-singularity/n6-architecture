# tests/test_calc.py
import pytest
from brainwire.calc import multi_state_sensitivity, state_gap_analysis, blend_command

def test_multi_state_sensitivity():
    results = multi_state_sensitivity('thc', tier=3)
    assert len(results) > 0
    for r in results:
        assert 'param' in r
        assert 'delta' in r

def test_state_gap_analysis():
    gaps = state_gap_analysis('dmt', tier=3)
    assert isinstance(gaps, list)

def test_blend_command():
    result = blend_command(['thc', 'flow'], [0.7, 0.3], tier=3)
    assert len(result['target']) == 12
    assert result['target']['DA'] == pytest.approx(0.7 * 2.5 + 0.3 * 1.8)
