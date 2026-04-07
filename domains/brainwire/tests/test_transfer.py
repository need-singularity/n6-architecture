"""Tests for the universal transfer function engine."""
import pytest
from brainwire.engine.transfer import TransferEngine, COEFFICIENTS


def test_coefficients_exist_for_all_12_vars():
    from brainwire.variables import VAR_NAMES
    for var in VAR_NAMES:
        assert var in COEFFICIENTS


def test_tier4_coefficients_include_new_devices():
    assert ('tFUS', 'VTA_intensity') in COEFFICIENTS['DA']
    assert ('tFUS', 'hippo_intensity') in COEFFICIENTS['eCB']
    assert ('tFUS', 'raphe_intensity') in COEFFICIENTS['5HT']


def test_baseline_is_1():
    engine = TransferEngine()
    result = engine.compute({})
    from brainwire.variables import VAR_NAMES
    for var in VAR_NAMES:
        assert result[var] == pytest.approx(1.0)


def test_tdcs_increases_da():
    engine = TransferEngine()
    result = engine.compute({'tDCS_anode_mA': 2.0})
    assert result['DA'] > 1.0


def test_tfus_vta_boosts_da():
    engine = TransferEngine()
    r_without = engine.compute({'tDCS_anode_mA': 2.0})
    r_with = engine.compute({'tDCS_anode_mA': 2.0, 'tFUS_VTA_intensity': 0.8})
    assert r_with['DA'] > r_without['DA']


def test_gvs_boosts_body():
    engine = TransferEngine()
    r_without = engine.compute({'TENS_low': 1.0})
    r_with = engine.compute({'TENS_low': 1.0, 'GVS_current_mA': 1.0})
    assert r_with['Body'] > r_without['Body']


def test_mti_deep_suppresses_pfc():
    engine = TransferEngine()
    result = engine.compute({'mTI_dlPFC_intensity': 0.8})
    assert result['PFC'] < 1.0


def test_suppressed_vars_clamped():
    engine = TransferEngine()
    result = engine.compute({'taVNS_VNS_mA': 10.0})
    assert result['NE'] >= 0.01


def test_compute_returns_all_12():
    engine = TransferEngine()
    result = engine.compute({'tDCS_anode_mA': 1.0})
    from brainwire.variables import VAR_NAMES
    assert set(result.keys()) == set(VAR_NAMES)


def test_tier3_thc_matches_existing():
    engine = TransferEngine()
    tier3_params = {
        'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
        'tDCS_anode_V1_mA': 2.0, 'tDCS_anode_S1_mA': 2.0,
        'taVNS_VNS_mA': 0.5,
        'TENS_low': 1.0, 'TENS_high': 1.0,
        'TMS_theta': 1.0, 'TMS_1Hz': 1.0, 'TMS_10Hz': 1.0, 'TMS_40Hz': 1.0,
        'tACS_6Hz_mA': 2.0, 'tACS_10Hz_mA': 2.0, 'tACS_40Hz_mA': 2.0,
    }
    result = engine.compute(tier3_params)
    assert result['DA'] > 2.0
    assert result['eCB'] > 1.5
