# tests/test_safety.py
import pytest
from brainwire.hardware.safety import SafetyEngine, SafetyViolation

def test_hard_limit_tdcs():
    se = SafetyEngine()
    assert se.check_device_limit('tDCS', 2.0) is True
    assert se.check_device_limit('tDCS', 5.0) is False

def test_hard_limit_tms():
    se = SafetyEngine()
    assert se.check_device_limit('TMS', 2.5) is True
    assert se.check_device_limit('TMS', 3.0) is False

def test_hard_limit_tfus():
    se = SafetyEngine()
    assert se.check_device_limit('tFUS', 720.0) is True
    assert se.check_device_limit('tFUS', 721.0) is False

def test_variable_range_default():
    se = SafetyEngine()
    assert se.check_variable_range('DA', 2.5) is True
    assert se.check_variable_range('DA', 3.5) is False
    assert se.check_variable_range('DA', 0.05) is False

def test_variable_range_custom():
    se = SafetyEngine()
    se.set_variable_range('Sensory', 0.1, 5.0)
    assert se.check_variable_range('Sensory', 4.5) is True

def test_slew_rate():
    se = SafetyEngine()
    assert se.check_slew_rate('tDCS', 0.3) is True
    assert se.check_slew_rate('tDCS', 1.0) is False

def test_session_time():
    se = SafetyEngine()
    assert se.check_session_time(30) is True
    assert se.check_session_time(50) is False

def test_session_time_override():
    se = SafetyEngine()
    se.set_max_session_min(90)
    assert se.check_session_time(60) is True

def test_emergency_stop():
    se = SafetyEngine()
    variables = {'DA': 2.5, 'eCB': 3.0, '5HT': 1.5, 'GABA': 1.8, 'NE': 0.4,
                 'Theta': 2.5, 'Alpha': 0.5, 'Gamma': 1.8, 'PFC': 0.5,
                 'Sensory': 2.0, 'Body': 2.5, 'Coherence': 2.0}
    assert se.check_emergency(variables) == []
    variables['DA'] = 3.5
    violations = se.check_emergency(variables)
    assert len(violations) == 1
    assert violations[0].var == 'DA'

def test_daily_session_limit():
    se = SafetyEngine()
    for _ in range(2):
        se.record_session()
    assert se.can_start_session() is False
