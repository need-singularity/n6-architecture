import pytest
from brainwire.pharmacokinetics import (
    variable_at_time, simulate_thc_pharmacokinetics,
    generate_hardware_timing, VAR_KINETICS
)
from brainwire.variables import VAR_NAMES

def test_baseline_at_t0():
    for var in VAR_NAMES:
        assert variable_at_time(var, 0, 2.0) == pytest.approx(1.0)

def test_da_rises_fast():
    da_60s = variable_at_time('DA', 60, 2.5)
    ecb_60s = variable_at_time('eCB', 60, 3.0)
    # DA should rise faster than eCB
    da_progress = (da_60s - 1.0) / (2.5 - 1.0)
    ecb_progress = (ecb_60s - 1.0) / (3.0 - 1.0)
    assert da_progress > ecb_progress

def test_ne_fastest_onset():
    # NE has smallest onset_tau (20s) — should be fastest
    ne_30s = variable_at_time('NE', 30, 0.4)
    ne_progress = abs(ne_30s - 1.0) / abs(0.4 - 1.0)
    # At 30s, NE should be >50% of its way to target
    assert ne_progress > 0.5

def test_values_decay_after_peak():
    # After peak, values should decline
    da_peak = VAR_KINETICS['DA'][1]  # peak_time
    da_at_peak = variable_at_time('DA', da_peak, 2.5)
    da_after = variable_at_time('DA', da_peak + 3600, 2.5)
    assert da_after < da_at_peak

def test_simulate_returns_timeline():
    result = simulate_thc_pharmacokinetics('strong', duration_s=600, dt=10)
    assert len(result['timeline']) > 0
    assert 'peak_times' in result
    assert len(result['peak_times']) == 12

def test_peak_order():
    result = simulate_thc_pharmacokinetics('strong', duration_s=3600, dt=10)
    peaks = result['peak_times']
    # NE should peak before eCB
    assert peaks['NE'] < peaks['eCB']
    # DA should peak before Body
    assert peaks['DA'] < peaks['Body']

def test_hardware_timing_generated():
    result = simulate_thc_pharmacokinetics('strong', duration_s=600, dt=30)
    hw = generate_hardware_timing(result)
    assert len(hw['hardware_timeline']) > 0
    assert all(0 <= hw['hardware_timeline'][i]['fractions']['DA'] <= 1.5
               for i in range(len(hw['hardware_timeline'])))

def test_concentration_scaling():
    micro = simulate_thc_pharmacokinetics('micro', duration_s=600, dt=30)
    strong = simulate_thc_pharmacokinetics('strong', duration_s=600, dt=30)
    # At peak, micro DA should be less than strong DA
    micro_peak_da = max(d['variables']['DA'] for d in micro['timeline'])
    strong_peak_da = max(d['variables']['DA'] for d in strong['timeline'])
    assert micro_peak_da < strong_peak_da
