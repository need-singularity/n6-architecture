# tests/test_profiles.py
import pytest
from brainwire.profiles.base import StateProfile, Envelope
from brainwire.profiles import load_profile, list_profiles

def test_envelope_defaults():
    e = Envelope()
    assert e.onset_s > 0
    assert e.plateau_s > 0
    assert e.offset_s > 0
    assert e.curve in ('linear', 'sigmoid', 'exponential')

def test_state_profile_has_12_vars():
    p = StateProfile(
        name="test", category="test",
        target={'DA': 2.0, 'eCB': 2.0, '5HT': 1.5, 'GABA': 1.5, 'NE': 0.5,
                'Theta': 2.0, 'Alpha': 0.5, 'Gamma': 1.8, 'PFC': 0.5,
                'Sensory': 2.0, 'Body': 2.0, 'Coherence': 2.0},
    )
    assert len(p.target) == 12

def test_state_profile_rejects_missing_var():
    with pytest.raises(ValueError):
        StateProfile(name="bad", category="bad", target={'DA': 2.0})

def test_load_thc_profile():
    p = load_profile('thc')
    assert p.name == "THC Strong (25%)"
    assert p.category == "cannabinoid"
    assert len(p.target) == 12
    assert p.target['DA'] == 2.5
    assert p.target['eCB'] == 3.0

def test_load_lsd_profile():
    p = load_profile('lsd')
    assert p.category == "psychedelic"
    assert p.target['5HT'] == 3.5
    assert p.target['NE'] == 2.0

def test_load_dmt_profile():
    p = load_profile('dmt')
    assert p.target['5HT'] == 4.5
    assert p.target['Sensory'] == 5.0
    assert p.target['Coherence'] == 0.2
    assert p.envelope.onset_s <= 30

def test_load_psilocybin_profile():
    p = load_profile('psilocybin')
    assert p.target['Theta'] == 3.5

def test_load_mdma_profile():
    p = load_profile('mdma')
    assert p.target['Body'] == 3.0
    assert p.target['Coherence'] == 1.8

def test_load_flow_profile():
    p = load_profile('flow')
    assert p.target['Alpha'] == 1.5
    assert p.target['Coherence'] == 2.5

def test_list_profiles():
    profiles = list_profiles()
    assert len(profiles) >= 6
    assert 'thc' in profiles
    assert 'dmt' in profiles

def test_thc_concentration_scaling():
    p = load_profile('thc')
    micro = p.scale(0.25)
    assert micro['DA'] < p.target['DA']
    assert micro['DA'] == pytest.approx(1.0 + (2.5 - 1.0) * 0.25)
