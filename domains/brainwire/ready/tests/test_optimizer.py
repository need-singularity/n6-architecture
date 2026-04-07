import pytest
from brainwire.optimizer import optimize_for_profile, optimize_all


def test_optimize_thc_improves_tension():
    result = optimize_for_profile('thc', tier=4, max_iters=50)
    assert result['tension_match'] > 57.0  # current generic is 57.4%


def test_optimize_thc_maintains_avg():
    result = optimize_for_profile('thc', tier=4, max_iters=50)
    assert result['avg_match'] > 100.0  # should still be above 100%


def test_optimize_reduces_overshoot():
    result = optimize_for_profile('thc', tier=4, max_iters=50)
    # 5HT should be closer to target (not 206%)
    assert result['match']['5HT'] < 180.0


def test_optimize_all_profiles():
    results = optimize_all(tier=4)
    assert len(results) >= 6
    for state, r in results.items():
        assert r['tension_match'] > 0


def test_optimized_beats_generic_for_thc():
    from brainwire.engine.transfer import TransferEngine
    from brainwire.engine.tension import compute_tension
    from brainwire.profiles import load_profile
    from brainwire.hardware.configs import get_tier_params

    profile = load_profile('thc')
    engine = TransferEngine()
    gen_vars = engine.compute(get_tier_params(4))
    gen_tm = compute_tension(gen_vars, target=profile.target)['tension_match']

    opt = optimize_for_profile('thc', tier=4, max_iters=50)
    assert opt['tension_match'] > gen_tm
