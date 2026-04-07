"""Integration tests for EEG closed-loop feedback and BCI mode switching.

Task #17: BCI mode switching integration tests
- Mode switching between stimulation modalities
- EEG signal processing pipeline
- Hemispheric alpha asymmetry calculations
- All tests use simulated EEG data (no hardware dependencies)
"""

import math
import pytest
from brainwire.closed_loop import (
    ClosedLoopController,
    EEGSignalProcessor,
    EEGSample,
    EEGFeatures,
    SimulatedEEGSource,
    StimulationController,
    ModalityState,
)
from brainwire.eeg_feedback import compute_g, g_from_12var, GOLDEN_ZONE, GOLDEN_CENTER
from brainwire.engine.transfer import TransferEngine
from brainwire.engine.tension import compute_tension, compute_match
from brainwire.engine.pid import PIDBank
from brainwire.hardware.configs import get_tier_params
from brainwire.profiles import load_profile, list_profiles
from brainwire.variables import VAR_NAMES


# =========================================================================
# EEG Signal Processing Pipeline
# =========================================================================

class TestEEGSignalProcessing:
    """Tests for the EEG signal processing pipeline."""

    def test_processor_returns_features_from_sample(self):
        proc = EEGSignalProcessor(smoothing_window=3)
        sample = EEGSample(
            timestamp=0.0,
            alpha_left=10.0, alpha_right=12.0,
            alpha_frontal=8.0, alpha_global=10.0,
            theta_global=5.0, gamma_global=3.0,
            beta_global=4.0, delta_global=6.0,
        )
        features = proc.process(sample)
        assert isinstance(features, EEGFeatures)
        assert features.timestamp == 0.0
        assert features.g_value >= 0.0
        assert features.g_zone in ('golden', 'below', 'above')

    def test_processor_smoothing_reduces_noise(self):
        proc = EEGSignalProcessor(smoothing_window=5)
        # Feed noisy samples
        features_list = []
        import random
        random.seed(42)
        for i in range(20):
            noise = random.gauss(0, 2.0)
            sample = EEGSample(
                timestamp=float(i),
                alpha_left=10.0 + noise, alpha_right=12.0 + noise,
                alpha_frontal=8.0 + noise, alpha_global=10.0 + noise,
                theta_global=5.0, gamma_global=3.0,
                beta_global=4.0, delta_global=6.0,
            )
            features_list.append(proc.process(sample))

        # After window fills, features should be smoother than raw samples
        # Check that G values don't jump wildly between consecutive steps
        deltas = [abs(features_list[i].g_value - features_list[i-1].g_value)
                  for i in range(5, 20)]
        avg_delta = sum(deltas) / len(deltas)
        # Smoothed G should have moderate variation
        assert avg_delta < 2.0, f"Smoothed G jumps too much: avg delta = {avg_delta}"

    def test_processor_reset_clears_buffer(self):
        proc = EEGSignalProcessor(smoothing_window=5)
        for i in range(10):
            proc.process(EEGSample(timestamp=float(i),
                                   alpha_left=10, alpha_right=10,
                                   alpha_frontal=8, alpha_global=10,
                                   theta_global=5, gamma_global=3,
                                   beta_global=4, delta_global=6))
        proc.reset()
        assert len(proc._buffer) == 0

    def test_feature_extraction_all_fields_populated(self):
        proc = EEGSignalProcessor(smoothing_window=1)
        sample = EEGSample(
            timestamp=1.0,
            alpha_left=10.0, alpha_right=15.0,
            alpha_frontal=8.0, alpha_global=11.0,
            theta_global=6.0, gamma_global=4.0,
            beta_global=3.0, delta_global=7.0,
        )
        f = proc.process(sample)
        # All numeric fields should be finite
        assert math.isfinite(f.alpha_asymmetry)
        assert math.isfinite(f.alpha_asymmetry_abs)
        assert math.isfinite(f.frontal_alpha_ratio)
        assert math.isfinite(f.theta_beta_ratio)
        assert math.isfinite(f.gamma_fraction)
        assert math.isfinite(f.g_value)
        assert math.isfinite(f.d_deficit)
        assert math.isfinite(f.p_plasticity)
        assert math.isfinite(f.i_inhibition)


# =========================================================================
# Hemispheric Alpha Asymmetry
# =========================================================================

class TestAlphaAsymmetry:
    """Tests for hemispheric alpha asymmetry calculations."""

    def test_symmetric_alpha_gives_zero_asymmetry(self):
        proc = EEGSignalProcessor(smoothing_window=1)
        sample = EEGSample(
            timestamp=0.0,
            alpha_left=10.0, alpha_right=10.0,
            alpha_frontal=8.0, alpha_global=10.0,
            theta_global=5.0, gamma_global=3.0,
            beta_global=4.0, delta_global=6.0,
        )
        f = proc.process(sample)
        assert f.alpha_asymmetry == pytest.approx(0.0, abs=1e-6)
        assert f.alpha_asymmetry_abs == pytest.approx(0.0, abs=1e-6)

    def test_right_dominant_gives_positive_asymmetry(self):
        proc = EEGSignalProcessor(smoothing_window=1)
        sample = EEGSample(
            timestamp=0.0,
            alpha_left=8.0, alpha_right=12.0,
            alpha_frontal=8.0, alpha_global=10.0,
            theta_global=5.0, gamma_global=3.0,
            beta_global=4.0, delta_global=6.0,
        )
        f = proc.process(sample)
        assert f.alpha_asymmetry > 0.0  # ln(right) > ln(left)

    def test_left_dominant_gives_negative_asymmetry(self):
        proc = EEGSignalProcessor(smoothing_window=1)
        sample = EEGSample(
            timestamp=0.0,
            alpha_left=15.0, alpha_right=8.0,
            alpha_frontal=8.0, alpha_global=10.0,
            theta_global=5.0, gamma_global=3.0,
            beta_global=4.0, delta_global=6.0,
        )
        f = proc.process(sample)
        assert f.alpha_asymmetry < 0.0  # ln(right) < ln(left)

    def test_asymmetry_magnitude_scales_with_difference(self):
        proc = EEGSignalProcessor(smoothing_window=1)
        # Small asymmetry
        f_small = proc.process(EEGSample(
            timestamp=0.0,
            alpha_left=10.0, alpha_right=11.0,
            alpha_frontal=8.0, alpha_global=10.0,
            theta_global=5.0, gamma_global=3.0,
            beta_global=4.0, delta_global=6.0,
        ))
        proc.reset()
        # Large asymmetry
        f_large = proc.process(EEGSample(
            timestamp=0.0,
            alpha_left=5.0, alpha_right=20.0,
            alpha_frontal=8.0, alpha_global=10.0,
            theta_global=5.0, gamma_global=3.0,
            beta_global=4.0, delta_global=6.0,
        ))
        assert f_large.alpha_asymmetry_abs > f_small.alpha_asymmetry_abs

    def test_g_deficit_matches_absolute_asymmetry(self):
        """D component of G=DxP/I should equal |ln(right) - ln(left)|."""
        proc = EEGSignalProcessor(smoothing_window=1)
        sample = EEGSample(
            timestamp=0.0,
            alpha_left=8.0, alpha_right=15.0,
            alpha_frontal=7.0, alpha_global=10.0,
            theta_global=5.0, gamma_global=4.0,
            beta_global=3.0, delta_global=6.0,
        )
        f = proc.process(sample)
        # D should match absolute asymmetry
        assert f.d_deficit == pytest.approx(f.alpha_asymmetry_abs, rel=0.01)

    def test_pfc_suppression_creates_asymmetry_in_12var(self):
        """PFC suppression in 12-var model should create alpha asymmetry."""
        # Low PFC = suppressed = more asymmetry
        low_pfc = {'DA': 1.0, 'eCB': 1.0, '5HT': 1.0, 'GABA': 1.0, 'NE': 1.0,
                   'Theta': 1.0, 'Alpha': 1.0, 'Gamma': 1.0,
                   'PFC': 0.3, 'Sensory': 1.0, 'Body': 1.0, 'Coherence': 1.0}
        high_pfc = low_pfc.copy()
        high_pfc['PFC'] = 1.0

        g_low = g_from_12var(low_pfc)
        g_high = g_from_12var(high_pfc)
        assert g_low['D'] > g_high['D'], "Low PFC should create more asymmetry"


# =========================================================================
# Stimulation Modality Mode Switching
# =========================================================================

class TestModeSwitching:
    """Tests for switching between stimulation modalities."""

    def test_stim_controller_initializes_with_tier_modalities(self):
        stim = StimulationController(tier=3)
        active = stim.active_modalities
        # Tier 3 should have tDCS, tACS, TMS, taVNS, TENS
        for expected in ['tDCS', 'tACS', 'TMS', 'taVNS', 'TENS']:
            assert expected in active, f"{expected} should be active at tier 3"

    def test_stim_controller_tier4_has_more_modalities(self):
        stim3 = StimulationController(tier=3)
        stim4 = StimulationController(tier=4)
        assert stim4.active_modality_count > stim3.active_modality_count

    def test_disable_modality_zeros_params(self):
        stim = StimulationController(tier=3)
        # Apply some scaling first
        stim.apply_scaling(1.0, dt=10.0)
        # Now disable tDCS
        stim.switch_modality('tDCS', active=False)
        mod = stim.get_modality('tDCS')
        assert not mod.active
        assert all(v == 0.0 for v in mod.params.values())

    def test_enable_modality(self):
        stim = StimulationController(tier=3)
        stim.switch_modality('tDCS', active=False)
        assert 'tDCS' not in stim.active_modalities
        stim.switch_modality('tDCS', active=True)
        assert 'tDCS' in stim.active_modalities

    def test_switch_unknown_modality_raises(self):
        stim = StimulationController(tier=3)
        with pytest.raises(ValueError, match="Unknown modality"):
            stim.switch_modality('UnknownDevice', active=True)

    def test_emergency_stop_zeros_all(self):
        stim = StimulationController(tier=4)
        stim.apply_scaling(1.0, dt=100.0)
        stim.emergency_stop()
        params = stim.get_flat_params()
        assert len(params) == 0 or all(v == 0.0 for v in params.values())
        assert stim.active_modality_count == 0

    def test_mode_switch_preserves_other_modalities(self):
        stim = StimulationController(tier=3)
        stim.apply_scaling(1.0, dt=100.0)
        # Record tACS params
        tacs_before = stim.get_modality('tACS').params.copy()
        # Disable tDCS
        stim.switch_modality('tDCS', active=False)
        # tACS should be unchanged
        tacs_after = stim.get_modality('tACS').params
        for key in tacs_before:
            assert tacs_after[key] == pytest.approx(tacs_before[key])

    def test_flat_params_only_includes_active(self):
        stim = StimulationController(tier=3)
        stim.apply_scaling(1.0, dt=100.0)
        all_params = stim.get_flat_params()
        # Disable TMS
        stim.switch_modality('TMS', active=False)
        reduced_params = stim.get_flat_params()
        # TMS keys should be gone
        tms_keys = [k for k in all_params if k.startswith('TMS_')]
        for k in tms_keys:
            assert k not in reduced_params

    def test_slew_rate_limits_param_change(self):
        """Params should not jump instantly; slew rate limits the change per dt."""
        stim = StimulationController(tier=3)
        # Small dt = small change allowed
        stim.apply_scaling(1.0, dt=0.1)
        params_small_dt = stim.get_flat_params()
        # With small dt, tDCS_anode_mA should not have reached 2.0 yet
        tdcs_val = params_small_dt.get('tDCS_anode_mA', 0.0)
        assert tdcs_val < 2.0, f"Slew rate should limit change: got {tdcs_val}"

    def test_sequential_mode_switches_across_profiles(self):
        """Switch between THC -> Flow -> DMT and verify target changes."""
        ctrl = ClosedLoopController(state='thc', tier=3)
        # Run a few steps in THC mode
        for i in range(5):
            ctrl.step(float(i), dt=1.0, envelope=1.0)
        thc_target = ctrl.profile.target.copy()

        # Switch to flow
        ctrl.switch_state('flow')
        for i in range(5, 10):
            ctrl.step(float(i), dt=1.0, envelope=1.0)
        flow_target = ctrl.profile.target.copy()
        assert thc_target != flow_target

        # Switch to DMT
        ctrl.switch_state('dmt')
        dmt_target = ctrl.profile.target.copy()
        assert dmt_target != flow_target
        # DMT should have high sensory
        assert dmt_target['Sensory'] > flow_target['Sensory']


# =========================================================================
# Closed-Loop Controller Integration
# =========================================================================

class TestClosedLoopIntegration:
    """End-to-end integration tests for the closed-loop controller."""

    def test_controller_runs_full_session(self):
        ctrl = ClosedLoopController(state='thc', tier=3, noise_level=0.05)
        timeline = ctrl.run(duration_s=60, dt=1.0, onset_s=10, offset_s=5)
        assert len(timeline) > 50
        # All snapshots should have valid data
        for snap in timeline:
            assert math.isfinite(snap.avg_match)
            assert math.isfinite(snap.g_value)
            assert len(snap.active_modalities) > 0

    def test_controller_converges_toward_target(self):
        """Match should improve from start to plateau."""
        ctrl = ClosedLoopController(state='thc', tier=3, noise_level=0.02)
        timeline = ctrl.run(duration_s=120, dt=1.0, onset_s=20, offset_s=10)
        # Early match (during ramp)
        early = [s.avg_match for s in timeline[:10]]
        # Late match (during plateau)
        late = [s.avg_match for s in timeline[40:60]]
        avg_early = sum(early) / len(early)
        avg_late = sum(late) / len(late)
        assert avg_late > avg_early, (
            f"Plateau match ({avg_late:.1f}) should exceed onset match ({avg_early:.1f})"
        )

    def test_controller_with_all_profiles(self):
        """Controller should work for every defined state profile."""
        for state in list_profiles():
            ctrl = ClosedLoopController(state=state, tier=3, noise_level=0.05)
            timeline = ctrl.run(duration_s=30, dt=1.0, onset_s=5, offset_s=3)
            assert len(timeline) > 20, f"Failed for state: {state}"
            summary = ctrl.get_summary()
            assert summary['total_steps'] > 20
            assert summary['modality_count'] > 0

    def test_controller_summary_statistics(self):
        ctrl = ClosedLoopController(state='flow', tier=3, noise_level=0.05)
        ctrl.run(duration_s=60, dt=1.0, onset_s=10, offset_s=5)
        summary = ctrl.get_summary()
        assert 'peak_avg_match' in summary
        assert 'plateau_avg_match' in summary
        assert 'g_mean' in summary
        assert 'g_golden_zone_pct' in summary
        assert summary['duration_s'] > 0
        assert 0 <= summary['g_golden_zone_pct'] <= 100

    def test_g_value_tracked_throughout_session(self):
        ctrl = ClosedLoopController(state='flow', tier=3, noise_level=0.05)
        timeline = ctrl.run(duration_s=30, dt=1.0, onset_s=5, offset_s=3)
        g_values = [s.g_value for s in timeline]
        # All G values should be non-negative
        assert all(g >= 0 for g in g_values)
        # Should have some variation (not all identical)
        assert max(g_values) > min(g_values)

    def test_empty_summary_when_no_run(self):
        ctrl = ClosedLoopController(state='thc', tier=3)
        summary = ctrl.get_summary()
        assert summary == {}

    def test_modality_switching_during_session(self):
        """Switch modalities mid-session and verify the controller adapts."""
        ctrl = ClosedLoopController(state='thc', tier=4, noise_level=0.05)
        # Run some steps with all modalities
        for i in range(10):
            ctrl.step(float(i), dt=1.0, envelope=1.0)
        count_before = len(ctrl.timeline[-1].active_modalities)

        # Disable tFUS and GVS
        ctrl.switch_modality('tFUS', active=False)
        ctrl.switch_modality('GVS', active=False)

        for i in range(10, 20):
            ctrl.step(float(i), dt=1.0, envelope=1.0)
        count_after = len(ctrl.timeline[-1].active_modalities)

        assert count_after < count_before
        # Controller should still produce valid output
        assert math.isfinite(ctrl.timeline[-1].avg_match)

    def test_stop_triggers_emergency_shutdown(self):
        ctrl = ClosedLoopController(state='thc', tier=3, noise_level=0.05)
        for i in range(5):
            ctrl.step(float(i), dt=1.0, envelope=1.0)
        ctrl.stop()
        assert not ctrl._running
        assert ctrl._stim.active_modality_count == 0


# =========================================================================
# Simulated EEG Source
# =========================================================================

class TestSimulatedEEGSource:
    """Tests for the simulated EEG data source."""

    def test_source_produces_positive_values(self):
        profile = load_profile('thc')
        source = SimulatedEEGSource(profile, noise_level=0.1)
        for _ in range(20):
            sample = source.read()
            assert sample.alpha_left > 0
            assert sample.alpha_right > 0
            assert sample.alpha_frontal > 0
            assert sample.alpha_global > 0
            assert sample.theta_global > 0
            assert sample.gamma_global > 0

    def test_stim_effect_modulates_eeg(self):
        profile = load_profile('thc')
        source = SimulatedEEGSource(profile, noise_level=0.01)

        # Baseline reading
        baseline = source.read()

        # Apply strong alpha suppression
        source.set_stim_effect({
            'DA': 1.0, 'eCB': 1.0, '5HT': 1.0, 'GABA': 1.0, 'NE': 1.0,
            'Theta': 1.0, 'Alpha': 0.3, 'Gamma': 1.0,
            'PFC': 1.0, 'Sensory': 1.0, 'Body': 1.0, 'Coherence': 1.0,
        })
        suppressed = source.read()

        # Alpha should be lower after suppression
        assert suppressed.alpha_global < baseline.alpha_global

    def test_pfc_suppression_creates_eeg_asymmetry(self):
        """When PFC is suppressed in the model, simulated EEG should show asymmetry."""
        profile = load_profile('thc')
        source = SimulatedEEGSource(profile, noise_level=0.0)  # no noise for clean test

        # Symmetric baseline
        source.set_stim_effect({v: 1.0 for v in VAR_NAMES})
        sym = source.read()

        # Suppress PFC
        asym_vars = {v: 1.0 for v in VAR_NAMES}
        asym_vars['PFC'] = 0.3
        source.set_stim_effect(asym_vars)
        asym = source.read()

        # With PFC suppression, left alpha should be higher than right
        asym_diff = abs(asym.alpha_left - asym.alpha_right)
        sym_diff = abs(sym.alpha_left - sym.alpha_right)
        assert asym_diff > sym_diff


# =========================================================================
# Cross-modality Transfer Function Integration
# =========================================================================

class TestCrossModalityTransfer:
    """Tests verifying transfer function behavior across modalities."""

    def test_tier_progression_increases_variable_reach(self):
        """Higher tiers should affect more variables."""
        engine = TransferEngine()
        vars_reached = {}
        for tier in [1, 2, 3, 4, 5]:
            params = get_tier_params(tier)
            result = engine.compute(params)
            # Count variables that differ from baseline (1.0)
            changed = sum(1 for v in VAR_NAMES if abs(result[v] - 1.0) > 0.01)
            vars_reached[tier] = changed
        # Each tier should reach at least as many variables as the previous
        for tier in [2, 3, 4, 5]:
            assert vars_reached[tier] >= vars_reached[tier - 1], (
                f"Tier {tier} ({vars_reached[tier]}) should reach >= "
                f"tier {tier-1} ({vars_reached[tier-1]}) variables"
            )

    def test_single_modality_isolation(self):
        """Disabling all but one modality should only affect relevant variables."""
        engine = TransferEngine()
        # Only taVNS
        params = {'taVNS_VNS_mA': 0.5}
        result = engine.compute(params)
        # taVNS primarily affects DA, eCB, 5HT, NE
        assert result['DA'] != 1.0  # should be affected
        # Sensory should be unaffected (taVNS doesn't directly target it)
        assert result['Sensory'] == pytest.approx(1.0, abs=0.01)

    def test_pid_bank_adapts_to_profile_hints(self):
        """PID bank with profile hints should have modified gains."""
        for state in list_profiles():
            profile = load_profile(state)
            bank = PIDBank()
            bank.apply_hints(profile.pid_hints)
            # After applying hints, at least some controllers should differ
            # from default if profile has hints
            if profile.pid_hints:
                modified = any(
                    bank.controllers[h.var].Kp != 1.0
                    for h in profile.pid_hints
                )
                assert modified, f"PID hints for {state} should modify gains"
