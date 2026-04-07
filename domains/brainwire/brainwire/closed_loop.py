from __future__ import annotations

"""EEG Closed-Loop Feedback Controller for 12-modality stimulation.

Reads EEG input (real-time or simulated), processes frequency bands,
computes G=DxP/I and hemispheric alpha asymmetry, then adjusts
stimulation parameters across all modalities via PID control.

Supported modalities (12):
  Electrical: tDCS, tACS, TMS, taVNS, TENS, tFUS, GVS, mTI, tSCS, tRNS, HD-tDCS
  Non-electrical: tPBM (+ tSMS, PEMF, caloric, thermal, ionto in Tier 5)

Usage:
    controller = ClosedLoopController(state='thc', tier=3)
    controller.start(duration_s=600)  # simulation mode by default
"""

import math
import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Protocol, Iterator

from brainwire.variables import VAR_NAMES, WAVE_VARS
from brainwire.engine.pid import PIDBank, PIDController
from brainwire.engine.transfer import TransferEngine
from brainwire.engine.tension import compute_tension, compute_match
from brainwire.eeg_feedback import compute_g, GOLDEN_ZONE, GOLDEN_CENTER
from brainwire.hardware.configs import get_tier_params
from brainwire.hardware.safety import SafetyEngine, DEVICE_HARD_LIMITS, DEVICE_SLEW_RATES
from brainwire.profiles import load_profile
from brainwire.profiles.base import StateProfile


# ---------------------------------------------------------------------------
# EEG band definitions
# ---------------------------------------------------------------------------

class EEGBand(Enum):
    DELTA = 'delta'      # 0.5-4 Hz
    THETA = 'theta'      # 4-8 Hz
    ALPHA = 'alpha'      # 8-12 Hz
    BETA = 'beta'        # 12-30 Hz
    GAMMA = 'gamma'      # 30-100 Hz


BAND_RANGES_HZ = {
    EEGBand.DELTA: (0.5, 4.0),
    EEGBand.THETA: (4.0, 8.0),
    EEGBand.ALPHA: (8.0, 12.0),
    EEGBand.BETA: (12.0, 30.0),
    EEGBand.GAMMA: (30.0, 100.0),
}


# ---------------------------------------------------------------------------
# EEG data structures
# ---------------------------------------------------------------------------

@dataclass
class EEGSample:
    """Single EEG measurement across channels and bands."""
    timestamp: float
    # Band powers per region
    alpha_left: float = 0.0     # P7/O1 (left posterior)
    alpha_right: float = 0.0    # P8/O2 (right posterior)
    alpha_frontal: float = 0.0  # mean(F3, F4)
    alpha_global: float = 0.0   # whole-scalp alpha
    theta_global: float = 0.0   # whole-scalp theta
    gamma_global: float = 0.0   # whole-scalp gamma
    beta_global: float = 0.0    # whole-scalp beta
    delta_global: float = 0.0   # whole-scalp delta


@dataclass
class EEGFeatures:
    """Processed features derived from raw EEG bands."""
    timestamp: float
    # Hemispheric alpha asymmetry (ln(right) - ln(left))
    alpha_asymmetry: float = 0.0
    # Absolute asymmetry |ln(right) - ln(left)|
    alpha_asymmetry_abs: float = 0.0
    # Frontal alpha ratio (frontal / global)
    frontal_alpha_ratio: float = 0.0
    # Theta/Beta ratio (attention marker)
    theta_beta_ratio: float = 0.0
    # Gamma fraction (plasticity proxy)
    gamma_fraction: float = 0.0
    # G = D*P/I consciousness quality metric
    g_value: float = 0.0
    g_in_golden_zone: bool = False
    g_zone: str = 'unknown'
    # Individual DPI components
    d_deficit: float = 0.0
    p_plasticity: float = 0.0
    i_inhibition: float = 0.0


# ---------------------------------------------------------------------------
# EEG Source interface and simulated source
# ---------------------------------------------------------------------------

class EEGSource(Protocol):
    """Interface for EEG data sources."""
    def read(self) -> EEGSample: ...
    def close(self) -> None: ...


class SimulatedEEGSource:
    """Generates simulated EEG data that drifts toward a target profile.

    Models realistic EEG dynamics:
    - Baseline alpha ~10uV^2, gamma ~2uV^2, theta ~5uV^2
    - Natural 1/f spectral slope
    - Physiological noise and breathing artifacts
    - Response to stimulation (variables affect EEG)
    """

    def __init__(self, profile: StateProfile, noise_level: float = 0.1,
                 sample_rate_hz: float = 1.0):
        self._profile = profile
        self._noise = noise_level
        self._rate = sample_rate_hz
        self._t = 0.0
        # Baseline EEG powers (uV^2)
        self._base_alpha = 10.0
        self._base_theta = 5.0
        self._base_gamma = 2.0
        self._base_beta = 4.0
        self._base_delta = 8.0
        # Stimulation effect accumulator (set by controller)
        self._stim_variables: dict[str, float] = {v: 1.0 for v in VAR_NAMES}

    def set_stim_effect(self, variables: dict[str, float]):
        """Update the simulated EEG response to stimulation."""
        self._stim_variables = variables.copy()

    def read(self) -> EEGSample:
        t = self._t
        self._t += 1.0 / self._rate

        v = self._stim_variables

        # Alpha power modulated by profile Alpha variable
        alpha_mod = v.get('Alpha', 1.0)
        # PFC suppression creates asymmetry
        pfc_mod = v.get('PFC', 1.0)
        asymmetry_shift = max(0, 1.0 - pfc_mod) * 0.3

        # Breathing artifact (slow oscillation)
        breath = 1.0 + 0.08 * math.sin(2 * math.pi * t / 20.0)
        # Eye-blink artifact (occasional)
        blink = 1.0 + 0.15 * max(0, math.sin(2 * math.pi * t / 7.0)) ** 10

        # Noise
        noise = self._noise
        import random
        n = lambda: 1.0 + random.gauss(0, noise)

        alpha_base = self._base_alpha * alpha_mod * breath * n()
        alpha_left = alpha_base * (1.0 + asymmetry_shift) * n()
        alpha_right = alpha_base * (1.0 - asymmetry_shift) * n()
        alpha_frontal = alpha_base * pfc_mod * blink * n()
        alpha_global = (alpha_left + alpha_right + alpha_frontal) / 3.0

        theta_mod = v.get('Theta', 1.0)
        theta_global = self._base_theta * theta_mod * breath * n()

        gamma_mod = v.get('Gamma', 1.0)
        gamma_global = self._base_gamma * gamma_mod * n()

        beta_global = self._base_beta * n()

        coherence_mod = v.get('Coherence', 1.0)
        # Higher coherence reduces noise in global measures
        if coherence_mod > 1.0:
            alpha_global *= 1.0 + (coherence_mod - 1.0) * 0.1
            gamma_global *= 1.0 + (coherence_mod - 1.0) * 0.05

        return EEGSample(
            timestamp=t,
            alpha_left=max(0.01, alpha_left),
            alpha_right=max(0.01, alpha_right),
            alpha_frontal=max(0.01, alpha_frontal),
            alpha_global=max(0.01, alpha_global),
            theta_global=max(0.01, theta_global),
            gamma_global=max(0.01, gamma_global),
            beta_global=max(0.01, beta_global),
            delta_global=max(0.01, self._base_delta * n()),
        )

    def close(self):
        pass


# ---------------------------------------------------------------------------
# Signal processor
# ---------------------------------------------------------------------------

class EEGSignalProcessor:
    """Processes raw EEG samples into features for closed-loop control."""

    def __init__(self, smoothing_window: int = 5):
        self._window = smoothing_window
        self._buffer: list[EEGSample] = []

    def process(self, sample: EEGSample) -> EEGFeatures:
        """Process a single EEG sample and return features."""
        self._buffer.append(sample)
        if len(self._buffer) > self._window:
            self._buffer.pop(0)

        # Smooth over window
        n = len(self._buffer)
        avg = lambda attr: sum(getattr(s, attr) for s in self._buffer) / n

        al = avg('alpha_left')
        ar = avg('alpha_right')
        af = avg('alpha_frontal')
        ag = avg('alpha_global')
        tg = avg('theta_global')
        gg = avg('gamma_global')
        bg = avg('beta_global')

        # Compute features
        alpha_asym = math.log(max(ar, 1e-9)) - math.log(max(al, 1e-9))
        alpha_asym_abs = abs(alpha_asym)
        frontal_ratio = af / max(ag, 1e-9)
        theta_beta = tg / max(bg, 1e-9)
        gamma_frac = gg / max(ag + gg, 1e-9)

        # Compute G = D*P/I
        g_result = compute_g(al, ar, gg, ag, af)

        return EEGFeatures(
            timestamp=sample.timestamp,
            alpha_asymmetry=alpha_asym,
            alpha_asymmetry_abs=alpha_asym_abs,
            frontal_alpha_ratio=frontal_ratio,
            theta_beta_ratio=theta_beta,
            gamma_fraction=gamma_frac,
            g_value=g_result['G'],
            g_in_golden_zone=g_result['in_golden_zone'],
            g_zone=g_result['zone'],
            d_deficit=g_result['D'],
            p_plasticity=g_result['P'],
            i_inhibition=g_result['I'],
        )

    def reset(self):
        self._buffer.clear()


# ---------------------------------------------------------------------------
# Stimulation modality control
# ---------------------------------------------------------------------------

@dataclass
class ModalityState:
    """Current state of a single stimulation modality."""
    name: str
    active: bool = False
    params: dict[str, float] = field(default_factory=dict)
    target_params: dict[str, float] = field(default_factory=dict)


class StimulationController:
    """Controls all stimulation modalities with safety-limited slew rates."""

    MODALITY_NAMES = [
        'tDCS', 'tACS', 'TMS', 'taVNS', 'TENS', 'tFUS',
        'GVS', 'mTI', 'tSCS', 'tRNS', 'HD-tDCS', 'tPBM',
    ]

    def __init__(self, tier: int = 3):
        self._tier = tier
        self._safety = SafetyEngine()
        self._modalities: dict[str, ModalityState] = {}
        self._base_params = get_tier_params(tier)

        # Initialize modalities from base params
        for name in self.MODALITY_NAMES:
            mod_params = {}
            for key, val in self._base_params.items():
                if key.startswith(name + '_') or key.startswith(name.replace('-', '') + '_'):
                    mod_params[key] = val
            self._modalities[name] = ModalityState(
                name=name,
                active=len(mod_params) > 0,
                params={k: 0.0 for k in mod_params},  # start at zero
                target_params=mod_params,
            )

    @property
    def active_modality_count(self) -> int:
        return sum(1 for m in self._modalities.values() if m.active)

    @property
    def active_modalities(self) -> list[str]:
        return [m.name for m in self._modalities.values() if m.active]

    def get_modality(self, name: str) -> ModalityState | None:
        return self._modalities.get(name)

    def switch_modality(self, name: str, active: bool):
        """Enable or disable a modality."""
        if name not in self._modalities:
            raise ValueError(f"Unknown modality: {name}")
        mod = self._modalities[name]
        mod.active = active
        if not active:
            # Zero out params when disabling
            mod.params = {k: 0.0 for k in mod.params}

    def get_flat_params(self) -> dict[str, float]:
        """Get all current params as flat dict for TransferEngine."""
        params = {}
        for mod in self._modalities.values():
            if mod.active:
                params.update(mod.params)
        return params

    def apply_scaling(self, scale_factor: float, dt: float):
        """Scale all active modality params toward targets, respecting slew rates."""
        for mod in self._modalities.values():
            if not mod.active:
                continue
            for key in mod.params:
                target = mod.target_params.get(key, 0.0) * scale_factor
                current = mod.params[key]
                # Enforce safety limits
                device_name = key.split('_')[0]
                hard_limit = DEVICE_HARD_LIMITS.get(device_name, 100.0)
                target = min(target, hard_limit)
                # Enforce slew rate
                max_slew = DEVICE_SLEW_RATES.get(device_name, 1.0)
                max_delta = max_slew * dt
                delta = target - current
                if abs(delta) > max_delta:
                    delta = max_delta if delta > 0 else -max_delta
                mod.params[key] = max(0.0, current + delta)

    def emergency_stop(self):
        """Immediately zero all modality params."""
        for mod in self._modalities.values():
            mod.params = {k: 0.0 for k in mod.params}
            mod.active = False


# ---------------------------------------------------------------------------
# Closed-loop controller
# ---------------------------------------------------------------------------

@dataclass
class ClosedLoopSnapshot:
    """State snapshot at one timestep."""
    t: float
    eeg_features: EEGFeatures
    variables: dict[str, float]
    target: dict[str, float]
    avg_match: float
    tension_match: float
    g_value: float
    g_zone: str
    active_modalities: list[str]
    envelope: float


class ClosedLoopController:
    """Main closed-loop EEG feedback controller.

    Architecture:
        EEG Source -> Signal Processor -> Feature Extraction
            -> PID Bank -> Param Adjustment -> Stimulation Controller
            -> TransferEngine -> Variable Update -> (loop back to EEG Source)

    The controller uses G=DxP/I as a secondary objective:
    - Primary: match 12-variable target via PID
    - Secondary: steer G toward golden zone [0.2123, 0.5000]
    """

    def __init__(self, state: str = 'thc', tier: int = 3,
                 eeg_source: EEGSource | None = None,
                 noise_level: float = 0.1):
        self._profile = load_profile(state)
        self._tier = tier
        self._engine = TransferEngine()
        self._pid_bank = PIDBank(default_Kp=0.5, default_Ki=0.05, default_Kd=0.01)
        self._pid_bank.apply_hints(self._profile.pid_hints)

        # G-value PID (steer toward golden zone center)
        self._g_pid = PIDController(Kp=0.3, Ki=0.02, Kd=0.005,
                                     output_limit=0.5)

        self._stim = StimulationController(tier=tier)
        self._processor = EEGSignalProcessor(smoothing_window=5)

        # EEG source (simulated by default)
        if eeg_source is not None:
            self._eeg_source = eeg_source
        else:
            self._eeg_source = SimulatedEEGSource(
                self._profile, noise_level=noise_level)

        self._safety = SafetyEngine()
        self._timeline: list[ClosedLoopSnapshot] = []
        self._running = False

    @property
    def timeline(self) -> list[ClosedLoopSnapshot]:
        return self._timeline

    @property
    def profile(self) -> StateProfile:
        return self._profile

    def switch_state(self, new_state: str):
        """Switch to a different consciousness state target."""
        self._profile = load_profile(new_state)
        self._pid_bank = PIDBank(default_Kp=0.5, default_Ki=0.05, default_Kd=0.01)
        self._pid_bank.apply_hints(self._profile.pid_hints)
        self._processor.reset()
        if isinstance(self._eeg_source, SimulatedEEGSource):
            self._eeg_source._profile = self._profile

    def switch_modality(self, modality: str, active: bool):
        """Enable or disable a specific stimulation modality."""
        self._stim.switch_modality(modality, active)

    def step(self, t: float, dt: float, envelope: float = 1.0) -> ClosedLoopSnapshot:
        """Execute one closed-loop step.

        1. Read EEG
        2. Process signals
        3. Compute PID corrections
        4. Adjust stimulation
        5. Compute resulting variables
        6. Return snapshot
        """
        # 1. Read EEG
        eeg_sample = self._eeg_source.read()

        # 2. Process
        features = self._processor.process(eeg_sample)

        # 3. PID: compute target-based corrections
        target_now = self._profile.scale(envelope)
        current_vars = self._engine.compute(self._stim.get_flat_params())
        pid_corrections = self._pid_bank.update(target_now, current_vars, dt)

        # 4. G-value correction: nudge toward golden zone
        g_correction = self._g_pid.update(GOLDEN_CENTER, features.g_value, dt)

        # Combined scale factor from PID + G correction
        avg_correction = sum(pid_corrections.values()) / len(pid_corrections)
        scale = envelope * (1.0 + avg_correction * 0.01 + g_correction * 0.01)
        scale = max(0.0, min(1.5, scale))

        # 5. Apply to stimulation controller
        self._stim.apply_scaling(scale, dt)

        # 6. Compute resulting variables
        flat_params = self._stim.get_flat_params()
        variables = self._engine.compute(flat_params)

        # Update simulated EEG if using simulator
        if isinstance(self._eeg_source, SimulatedEEGSource):
            self._eeg_source.set_stim_effect(variables)

        # 7. Safety check
        violations = self._safety.check_emergency(variables)
        if violations:
            # Reduce intensity on violating variables
            for v in violations:
                if v.value > v.limit:
                    scale *= 0.9  # back off
                    self._stim.apply_scaling(scale, dt)
                    variables = self._engine.compute(self._stim.get_flat_params())

        # 8. Metrics
        match = compute_match(variables, self._profile.target)
        tension = compute_tension(variables, target=self._profile.target)
        avg_match = sum(max(0, v) for v in match.values()) / 12

        snapshot = ClosedLoopSnapshot(
            t=t,
            eeg_features=features,
            variables=variables,
            target=target_now,
            avg_match=avg_match,
            tension_match=tension['tension_match'],
            g_value=features.g_value,
            g_zone=features.g_zone,
            active_modalities=self._stim.active_modalities,
            envelope=envelope,
        )
        self._timeline.append(snapshot)
        return snapshot

    def run(self, duration_s: float = 600.0, dt: float = 1.0,
            onset_s: float = 60.0, offset_s: float = 30.0) -> list[ClosedLoopSnapshot]:
        """Run a complete closed-loop session.

        Args:
            duration_s: Total session duration in seconds.
            dt: Timestep in seconds.
            onset_s: Ramp-up duration.
            offset_s: Ramp-down duration.

        Returns:
            List of ClosedLoopSnapshot for each timestep.
        """
        self._timeline.clear()
        self._running = True
        plateau_s = duration_s - onset_s - offset_s

        from brainwire.engine.interpolation import envelope_value

        t = 0.0
        while t <= duration_s and self._running:
            env = envelope_value(t, onset_s, plateau_s, offset_s,
                                  self._profile.envelope.curve)
            self.step(t, dt, envelope=env)
            t += dt

        self._running = False
        return self._timeline

    def stop(self):
        """Stop a running session."""
        self._running = False
        self._stim.emergency_stop()

    def get_summary(self) -> dict:
        """Get session summary statistics."""
        if not self._timeline:
            return {}

        plateau = [s for s in self._timeline if s.envelope > 0.95]
        all_g = [s.g_value for s in self._timeline]
        golden_count = sum(1 for s in self._timeline if s.g_zone == 'golden')

        return {
            'total_steps': len(self._timeline),
            'duration_s': self._timeline[-1].t - self._timeline[0].t,
            'peak_avg_match': max(s.avg_match for s in self._timeline),
            'plateau_avg_match': (
                sum(s.avg_match for s in plateau) / len(plateau)
                if plateau else 0.0
            ),
            'plateau_tension_match': (
                sum(s.tension_match for s in plateau) / len(plateau)
                if plateau else 0.0
            ),
            'g_mean': sum(all_g) / len(all_g) if all_g else 0.0,
            'g_golden_zone_pct': golden_count / len(self._timeline) * 100,
            'active_modalities': self._stim.active_modalities,
            'modality_count': self._stim.active_modality_count,
        }
