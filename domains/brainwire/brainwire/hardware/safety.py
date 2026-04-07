from dataclasses import dataclass, field
from brainwire.hardware.devices import DeviceRegistry

@dataclass
class SafetyViolation:
    layer: int
    var: str
    value: float
    limit: float
    message: str

DEVICE_HARD_LIMITS = {
    'tDCS': 4.0, 'tACS': 4.0, 'TMS': 2.5, 'taVNS': 0.5,
    'TENS': 80.0, 'tFUS': 720.0, 'GVS': 2.0, 'mTI': 2.0,
    'tSCS': 40.0, 'tRNS': 2.0, 'HD-tDCS': 4.0,
}

DEVICE_SLEW_RATES = {
    'tDCS': 0.5, 'tACS': 0.5, 'TMS': 1.0, 'taVNS': 0.1,
    'TENS': 1.0, 'tFUS': 50.0, 'GVS': 0.2, 'mTI': 0.3,
    'tSCS': 1.0, 'tRNS': 0.5, 'HD-tDCS': 0.5,
}

class SafetyEngine:
    def __init__(self):
        self._var_ranges: dict[str, tuple[float, float]] = {}
        self._default_range = (0.1, 3.0)
        self._max_session_min = 40
        self._max_daily_sessions = 2
        self._session_count = 0

    def check_device_limit(self, device: str, value: float) -> bool:
        limit = DEVICE_HARD_LIMITS.get(device)
        if limit is None:
            return True
        return value <= limit

    def check_slew_rate(self, device: str, rate: float) -> bool:
        limit = DEVICE_SLEW_RATES.get(device)
        if limit is None:
            return True
        return rate <= limit

    def set_variable_range(self, var: str, low: float, high: float):
        self._var_ranges[var] = (low, high)

    def check_variable_range(self, var: str, value: float) -> bool:
        low, high = self._var_ranges.get(var, self._default_range)
        return low <= value <= high

    def check_emergency(self, variables: dict[str, float]) -> list[SafetyViolation]:
        violations = []
        for var, value in variables.items():
            low, high = self._var_ranges.get(var, self._default_range)
            if value > high:
                violations.append(SafetyViolation(
                    layer=2, var=var, value=value, limit=high,
                    message=f"{var}={value:.2f} exceeds max {high:.1f}"
                ))
            elif value < low:
                violations.append(SafetyViolation(
                    layer=2, var=var, value=value, limit=low,
                    message=f"{var}={value:.2f} below min {low:.1f}"
                ))
        return violations

    def set_max_session_min(self, minutes: int):
        self._max_session_min = minutes

    def check_session_time(self, elapsed_min: float) -> bool:
        return elapsed_min <= self._max_session_min

    def record_session(self):
        self._session_count += 1

    def can_start_session(self) -> bool:
        return self._session_count < self._max_daily_sessions
