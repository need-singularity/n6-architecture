from dataclasses import dataclass, field
from brainwire.variables import VAR_NAMES

@dataclass
class Envelope:
    onset_s: float = 300.0
    plateau_s: float = 3600.0
    offset_s: float = 180.0
    curve: str = 'sigmoid'

@dataclass
class SafetyOverrides:
    max_session_min: int = 40
    emergency_vars: list[str] = field(default_factory=list)
    first_session_limits: dict[str, float] = field(default_factory=dict)

@dataclass
class PidHint:
    var: str
    Kp_scale: float = 1.0
    Ki_scale: float = 1.0
    Kd_scale: float = 1.0

@dataclass
class StateProfile:
    name: str
    category: str
    target: dict[str, float]
    envelope: Envelope = field(default_factory=Envelope)
    safety: SafetyOverrides = field(default_factory=SafetyOverrides)
    pid_hints: list[PidHint] = field(default_factory=list)

    def __post_init__(self):
        missing = set(VAR_NAMES) - set(self.target.keys())
        if missing:
            raise ValueError(f"Missing variables: {missing}")

    def scale(self, factor: float) -> dict[str, float]:
        return {k: 1.0 + (v - 1.0) * factor for k, v in self.target.items()}
