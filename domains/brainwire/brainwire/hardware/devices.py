from __future__ import annotations

from dataclasses import dataclass, field

@dataclass
class Device:
    name: str
    slot: int
    max_current_mA: float
    cost_usd: float
    params: list[str] = field(default_factory=list)
    slew_rate_mA_per_s: float = 0.5
    max_session_min: int = 40

CORE_DEVICES = [
    Device('tDCS', 0, 4.0, 30,
           ['anode_mA', 'cathode_Fz_mA', 'cathode_F4_mA', 'anode_V1_mA', 'anode_S1_mA'],
           slew_rate_mA_per_s=0.5, max_session_min=20),
    Device('tACS', 1, 4.0, 80,
           ['tACS_6Hz_mA', 'tACS_10Hz_mA', 'tACS_40Hz_mA'],
           slew_rate_mA_per_s=0.5, max_session_min=30),
    Device('TMS', 2, 2.5, 5000,
           ['theta_strength', '1Hz_strength', '10Hz_strength', '40Hz_strength'],
           max_session_min=20),
    Device('taVNS', 3, 0.5, 100,
           ['VNS_mA'], slew_rate_mA_per_s=0.1, max_session_min=30),
    Device('TENS', 4, 80.0, 25,
           ['low_intensity', 'high_intensity'],
           slew_rate_mA_per_s=1.0, max_session_min=60),
    Device('tFUS', 5, 720.0, 8000,
           ['focus_x', 'focus_y', 'focus_z', 'intensity'],
           max_session_min=30),
    Device('GVS', 6, 2.0, 50,
           ['current_mA'], slew_rate_mA_per_s=0.2, max_session_min=20),
    Device('mTI', 7, 2.0, 2000,
           ['pair1_mA', 'pair2_mA', 'pair3_mA', 'freq_offset_Hz'],
           slew_rate_mA_per_s=0.3, max_session_min=20),
    Device('tSCS', 8, 40.0, 500,
           ['intensity'], slew_rate_mA_per_s=1.0, max_session_min=30),
    Device('tRNS', 9, 2.0, 200,
           ['intensity'], slew_rate_mA_per_s=0.5, max_session_min=20),
    Device('HD-tDCS', 10, 4.0, 3000,
           ['center_mA', 'ring_mA'], slew_rate_mA_per_s=0.5, max_session_min=20),
]

class DeviceRegistry:
    def __init__(self):
        self._devices: dict[str, Device] = {}
        for d in CORE_DEVICES:
            self._devices[d.name] = d

    def add(self, device: Device):
        self._devices[device.name] = device

    def remove(self, name: str):
        self._devices.pop(name, None)

    def get(self, name: str) -> Device | None:
        return self._devices.get(name)

    def all(self) -> list[Device]:
        return list(self._devices.values())
