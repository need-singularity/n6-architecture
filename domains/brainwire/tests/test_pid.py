import pytest
from brainwire.engine.pid import PIDController, PIDBank


def test_pid_zero_error():
    pid = PIDController(Kp=1.0, Ki=0.1, Kd=0.01)
    output = pid.update(setpoint=2.0, measured=2.0, dt=0.1)
    assert output == pytest.approx(0.0, abs=0.01)


def test_pid_positive_error():
    pid = PIDController(Kp=1.0, Ki=0.0, Kd=0.0)
    output = pid.update(setpoint=2.0, measured=1.0, dt=0.1)
    assert output == pytest.approx(1.0)


def test_pid_integral_accumulates():
    pid = PIDController(Kp=0.0, Ki=1.0, Kd=0.0)
    pid.update(setpoint=2.0, measured=1.0, dt=1.0)
    output = pid.update(setpoint=2.0, measured=1.0, dt=1.0)
    assert output == pytest.approx(2.0)


def test_pid_anti_windup():
    pid = PIDController(Kp=0.0, Ki=1.0, Kd=0.0, max_integral=5.0)
    for _ in range(100):
        pid.update(setpoint=100.0, measured=0.0, dt=1.0)
    output = pid.update(setpoint=100.0, measured=0.0, dt=1.0)
    assert output <= 5.0


def test_pid_reset():
    pid = PIDController(Kp=1.0, Ki=1.0, Kd=1.0)
    pid.update(setpoint=2.0, measured=1.0, dt=0.1)
    pid.reset()
    assert pid._integral == 0.0
    assert pid._prev_error == 0.0


def test_pid_bank_12_controllers():
    bank = PIDBank()
    assert len(bank.controllers) == 12


def test_pid_bank_update():
    bank = PIDBank()
    target = {'DA': 2.5, 'eCB': 3.0, '5HT': 1.5, 'GABA': 1.8, 'NE': 0.4,
              'Theta': 2.5, 'Alpha': 0.5, 'Gamma': 1.8,
              'PFC': 0.5, 'Sensory': 2.0, 'Body': 2.5, 'Coherence': 2.0}
    measured = {k: 1.0 for k in target}
    outputs = bank.update(target, measured, dt=0.1)
    assert len(outputs) == 12
    assert outputs['DA'] > 0
    assert outputs['NE'] < 0


def test_pid_bank_apply_hints():
    from brainwire.profiles.base import PidHint
    bank = PIDBank()
    hints = [PidHint(var='eCB', Kp_scale=2.0)]
    bank.apply_hints(hints)
    assert bank.controllers['eCB'].Kp == pytest.approx(2.0)
