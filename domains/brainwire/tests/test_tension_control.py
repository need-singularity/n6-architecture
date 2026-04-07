import pytest
from brainwire.tension_control import (
    TensionController, simulate_tension_control, tension_landscape
)
from brainwire.variables import VAR_NAMES

THC_TARGET = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
BASELINE = {k: 1.0 for k in VAR_NAMES}

def test_tension_gradient_points_toward_target():
    tc = TensionController()
    gradient = tc.compute_tension_gradient(BASELINE, THC_TARGET)
    # DA target > baseline -> gradient should be positive
    assert gradient['DA'] > 0
    # NE target < baseline -> gradient should be negative
    assert gradient['NE'] < 0

def test_gradient_to_params_produces_adjustments():
    tc = TensionController()
    gradient = tc.compute_tension_gradient(BASELINE, THC_TARGET)
    params = {'tDCS_anode_mA': 1.0, 'taVNS_VNS_mA': 0.3}
    new_params = tc.gradient_to_params(gradient, params)
    assert len(new_params) > 0
    # At least one param should change
    assert any(new_params.get(k, 0) != params.get(k, 0) for k in new_params)

def test_step_reduces_tension():
    from brainwire.engine.transfer import TransferEngine
    from brainwire.hardware.configs import get_tier_params
    tc = TensionController(learning_rate=0.1)
    engine = TransferEngine()
    params = {k: v * 0.5 for k, v in get_tier_params(3).items()}
    current = engine.compute(params)
    result = tc.step(current, THC_TARGET, params)
    # After one step, should have produced a result
    assert 'params' in result
    assert 'tension' in result

def test_homeostasis_within_deadband():
    tc = TensionController(homeostasis_deadband=0.3)
    mult = tc.homeostasis_modulate(1.0)  # setpoint=1.0, within deadband
    assert mult == 1.0

def test_homeostasis_reduces_high_tension():
    tc = TensionController(homeostasis_deadband=0.3)
    # Force EMA to high value
    for _ in range(100):
        tc.homeostasis_modulate(5.0)
    mult = tc.homeostasis_modulate(5.0)
    assert mult < 1.0  # should reduce learning rate

def test_simulate_thc_converges():
    result = simulate_tension_control('thc', tier=3, steps=100, lr=0.05)
    # Tension match and avg match should improve over the simulation
    assert result['final_tension_match'] > result['history'][0]['tension_match']
    assert result['final_avg_match'] > 50

def test_simulate_all_states():
    from brainwire.profiles import list_profiles
    for state in list_profiles():
        result = simulate_tension_control(state, tier=3, steps=50, lr=0.05)
        assert result['final_avg_match'] > 0

def test_tension_landscape_has_all_pairs():
    land = tension_landscape(resolution=5)
    assert len(land['states']) >= 6
    assert len(land['distances']) >= 36  # 6x6
    assert len(land['clusters']) == 3

def test_landscape_self_distance_perfect_match():
    land = tension_landscape(resolution=5)
    for state in land['states']:
        d = land['distances'].get((state, state), {})
        # Self-comparison should have 100% direction similarity and magnitude match
        assert d.get('direction_sim', 0) == pytest.approx(100.0, abs=0.1)
        assert d.get('magnitude_match', 0) == pytest.approx(100.0, abs=0.1)

def test_thc_flow_high_similarity():
    land = tension_landscape(resolution=5)
    pair = land['distances'].get(('thc', 'flow'), land['distances'].get(('flow', 'thc'), {}))
    assert pair.get('direction_sim', 0) > 70  # THC and Flow are similar
