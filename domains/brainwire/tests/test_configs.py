import pytest
from brainwire.hardware.configs import TIER_CONFIGS, get_tier_params


def test_four_tiers_defined():
    assert len(TIER_CONFIGS) >= 4
    for tier in [1, 2, 3, 4]:
        assert tier in TIER_CONFIGS


def test_tier_costs_ascending():
    costs = [TIER_CONFIGS[t]['cost'] for t in sorted(TIER_CONFIGS.keys())]
    for i in range(len(costs) - 1):
        assert costs[i] < costs[i + 1]


def test_tier4_has_new_devices():
    t4 = get_tier_params(4)
    assert t4.get('tFUS_VTA_intensity', 0) > 0
    assert t4.get('GVS_current_mA', 0) > 0
    assert t4.get('mTI_dlPFC_intensity', 0) > 0
    assert t4.get('tSCS_intensity', 0) > 0


def test_tier1_no_tms():
    t1 = get_tier_params(1)
    assert t1.get('TMS_theta', 0) == 0
    assert t1.get('TMS_1Hz', 0) == 0


def test_tier3_has_tms():
    t3 = get_tier_params(3)
    assert t3.get('TMS_theta', 0) > 0
