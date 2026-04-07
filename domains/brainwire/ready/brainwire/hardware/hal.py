from brainwire.hardware.devices import DeviceRegistry, Device

TIER_REQUIREMENTS = {
    1: {'tDCS', 'TENS'},
    2: {'tDCS', 'TENS', 'taVNS', 'tACS'},
    3: {'tDCS', 'TENS', 'taVNS', 'tACS', 'TMS'},
    4: {'tDCS', 'TENS', 'taVNS', 'tACS', 'TMS', 'tFUS', 'GVS', 'mTI'},
}

class HAL:
    def __init__(self):
        self._registry = DeviceRegistry()
        self._connected: dict[str, Device] = {}

    def connect(self, device_name: str) -> Device:
        dev = self._registry.get(device_name)
        if dev is None:
            raise ValueError(f"Unknown device: {device_name}")
        self._connected[device_name] = dev
        return dev

    def disconnect(self, device_name: str):
        self._connected.pop(device_name, None)

    def active_slots(self) -> list[int]:
        return sorted(d.slot for d in self._connected.values())

    def active_device_count(self) -> int:
        return len(self._connected)

    def active_devices(self) -> list[Device]:
        return list(self._connected.values())

    def total_cost(self) -> float:
        return sum(d.cost_usd for d in self._connected.values())

    def detect_tier(self) -> int:
        names = set(self._connected.keys())
        tier = 0
        for t, required in sorted(TIER_REQUIREMENTS.items()):
            if required.issubset(names):
                tier = t
        return tier
