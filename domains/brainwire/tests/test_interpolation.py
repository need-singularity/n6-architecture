import pytest
from brainwire.engine.interpolation import lerp_states, blend_states, sigmoid_alpha, linear_alpha, envelope_value

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
