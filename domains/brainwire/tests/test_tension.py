import pytest
from brainwire.engine.tension import compute_tension, compute_match

def test_tension_baseline_is_zero():
    baseline = {k: 1.0 for k in ['DA','eCB','5HT','GABA','NE','Theta','Alpha','Gamma','PFC','Sensory','Body','Coherence']}
    t = compute_tension(baseline)
    assert t['T_total'] == pytest.approx(0.0)

def test_tension_thc_target():
    thc = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
    t = compute_tension(thc, target=thc)
    assert t['direction_sim'] == pytest.approx(100.0, abs=0.1)
    assert t['magnitude_match'] == pytest.approx(100.0, abs=0.1)

def test_match_perfect():
    thc = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
    m = compute_match(thc, thc)
    for k, v in m.items():
        assert v == pytest.approx(100.0)

def test_match_suppressed_var():
    target = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
    actual = target.copy()
    actual['NE'] = 0.4
    m = compute_match(actual, target)
    assert m['NE'] == pytest.approx(100.0)

def test_tension_cross_state():
    thc = {'DA':2.5,'eCB':3.0,'5HT':1.5,'GABA':1.8,'NE':0.4,'Theta':2.5,'Alpha':0.5,'Gamma':1.8,'PFC':0.5,'Sensory':2.0,'Body':2.5,'Coherence':2.0}
    lsd = {'DA':1.8,'eCB':1.3,'5HT':3.5,'GABA':0.6,'NE':2.0,'Theta':3.0,'Alpha':0.3,'Gamma':2.5,'PFC':1.5,'Sensory':3.5,'Body':1.5,'Coherence':0.4}
    t = compute_tension(lsd, target=thc)
    assert t['direction_sim'] < 100.0
