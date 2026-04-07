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
