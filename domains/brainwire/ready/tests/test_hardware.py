# tests/test_hardware.py
import pytest
from brainwire.hardware.devices import Device, DeviceRegistry, CORE_DEVICES
from brainwire.hardware.hal import HAL

def test_device_has_required_fields():
    d = Device(name='tDCS', slot=0, max_current_mA=4.0, cost_usd=30,
               params=['anode_mA', 'cathode_Fz_mA', 'cathode_F4_mA'])
    assert d.name == 'tDCS'
    assert d.max_current_mA == 4.0

def test_core_devices_count():
    assert len(CORE_DEVICES) >= 8

def test_device_registry_add_remove():
    reg = DeviceRegistry()
    d = Device(name='test', slot=99, max_current_mA=1.0, cost_usd=10, params=['x'])
    reg.add(d)
    assert reg.get('test') == d
    reg.remove('test')
    assert reg.get('test') is None

def test_hal_active_slots():
    hal = HAL()
    hal.connect('tDCS')
    hal.connect('TENS')
    assert hal.active_slots() == [0, 4]
    assert hal.active_device_count() == 2

def test_hal_disconnect():
    hal = HAL()
    hal.connect('tDCS')
    hal.connect('TENS')
    hal.disconnect('TENS')
    assert hal.active_slots() == [0]

def test_hal_total_cost():
    hal = HAL()
    hal.connect('tDCS')
    hal.connect('taVNS')
    cost = hal.total_cost()
    assert cost > 0

def test_hal_tier_detection():
    hal = HAL()
    hal.connect('tDCS')
    hal.connect('TENS')
    assert hal.detect_tier() == 1
    hal.connect('taVNS')
    hal.connect('tACS')
    assert hal.detect_tier() == 2
    hal.connect('TMS')
    assert hal.detect_tier() == 3
    hal.connect('tFUS')
    hal.connect('GVS')
    hal.connect('mTI')
    assert hal.detect_tier() == 4
