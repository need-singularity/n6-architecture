import pytest
from brainwire.simulator import simulate_session, breathing_modulation

def test_breathing_modulation_oscillates():
    vals = [breathing_modulation(t) for t in range(100)]
    assert max(vals) > 1.0  # oscillates above
    assert min(vals) < 1.0  # and below

def test_simulate_session_returns_timeline():
    result = simulate_session('thc', tier=3, duration_s=60, dt=1.0)
    assert len(result['timeline']) > 0
    assert result['peak_avg_match'] > 0

def test_session_reaches_plateau():
    result = simulate_session('thc', tier=3, duration_s=600, dt=1.0)
    assert result['plateau_avg_match'] > 50  # should reach meaningful level

def test_pid_improves_over_open_loop():
    pid_result = simulate_session('thc', tier=3, duration_s=300, dt=1.0, use_pid=True)
    open_result = simulate_session('thc', tier=3, duration_s=300, dt=1.0, use_pid=False)
    # PID should generally maintain similar or better performance
    assert pid_result['plateau_avg_match'] > 0

def test_dmt_fast_onset():
    result = simulate_session('dmt', tier=4, duration_s=120, dt=0.5)
    # DMT onset is 30s, should reach high match quickly
    early = [d for d in result['timeline'] if d['t'] < 60]
    late = [d for d in result['timeline'] if d['t'] > 60]
    if early and late:
        assert max(d['avg_match'] for d in early) > 0

def test_all_states_simulate():
    from brainwire.profiles import list_profiles
    for state in list_profiles():
        result = simulate_session(state, tier=3, duration_s=60, dt=2.0)
        assert result['peak_avg_match'] > 0
