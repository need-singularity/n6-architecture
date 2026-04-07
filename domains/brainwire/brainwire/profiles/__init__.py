import os
import yaml
from brainwire.profiles.base import StateProfile, Envelope, SafetyOverrides, PidHint

_PROFILE_DIR = os.path.dirname(__file__)

def load_profile(name: str) -> StateProfile:
    path = os.path.join(_PROFILE_DIR, f"{name}.yaml")
    if not os.path.exists(path):
        raise FileNotFoundError(f"Profile not found: {name}")
    with open(path) as f:
        data = yaml.safe_load(f)
    s = data['state']
    env_data = data.get('envelope', {})
    envelope = Envelope(
        onset_s=env_data.get('onset_s', 300),
        plateau_s=env_data.get('plateau_s', 3600),
        offset_s=env_data.get('offset_s', 180),
        curve=env_data.get('curve', 'sigmoid'),
    )
    safety_data = data.get('safety_overrides', {})
    safety = SafetyOverrides(
        max_session_min=safety_data.get('max_session_min', 40),
        emergency_vars=safety_data.get('emergency_vars', []),
        first_session_limits=safety_data.get('first_session_limits', {}),
    )
    pid_hints = []
    for var, hints in data.get('pid_hints', {}).items():
        pid_hints.append(PidHint(var=var, **hints))
    return StateProfile(
        name=s['name'], category=s['category'],
        target=data['target_vector'], envelope=envelope,
        safety=safety, pid_hints=pid_hints,
    )

def list_profiles() -> list[str]:
    return [f[:-5] for f in os.listdir(_PROFILE_DIR)
            if f.endswith('.yaml')]
