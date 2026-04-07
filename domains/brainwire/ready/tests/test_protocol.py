import pytest
import json
import tempfile
from brainwire.protocol import generate_protocol, params_to_devices, export_json

def test_generate_thc_protocol():
    p = generate_protocol('thc', tier=3, session_min=30)
    assert p['state'] == 'thc'
    assert p['tier'] == 3
    assert len(p['steps']) > 0
    assert len(p['safety']) > 5

def test_protocol_has_all_phases():
    p = generate_protocol('thc', tier=3, session_min=20)
    phases = set(s.phase for s in p['steps'])
    assert 'prep' in phases
    assert 'ramp_up' in phases
    assert 'plateau' in phases
    assert 'ramp_down' in phases
    assert 'cooldown' in phases

def test_params_to_devices():
    params = {'tDCS_anode_mA': 2.0, 'taVNS_VNS_mA': 0.5, 'entrainment_LED_40Hz': 1.0}
    devices = params_to_devices(params)
    assert 'tDCS' in devices
    assert 'taVNS' in devices
    assert 'Entrainment' in devices

def test_dmt_has_extra_safety():
    p = generate_protocol('dmt', tier=4, session_min=15)
    safety_text = ' '.join(p['safety'])
    assert 'FIRST SESSION' in safety_text
    assert 'Voice anchor' in safety_text or 'voice anchor' in safety_text.lower()

def test_export_json():
    p = generate_protocol('thc', tier=3, session_min=10)
    with tempfile.NamedTemporaryFile(suffix='.json', delete=False) as f:
        path = export_json(p, f.name)
    with open(path) as f:
        data = json.load(f)
    assert data['state'] == 'thc'
    assert len(data['timeline']) > 0

def test_all_states_generate():
    from brainwire.profiles import list_profiles
    for state in list_profiles():
        p = generate_protocol(state, tier=3, session_min=20)
        assert len(p['steps']) > 0
