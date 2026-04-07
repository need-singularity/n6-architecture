# tests/test_bench.py
import pytest
from brainwire.bench import run_benchmark, compare_states, run_tier_comparison

def test_run_benchmark_thc():
    result = run_benchmark('thc', tier=3)
    assert result['profile_name'] == 'THC Strong (25%)'
    assert result['tier'] == 3
    assert len(result['match']) == 12
    assert result['avg_match'] > 0

def test_run_benchmark_all_states():
    for state in ['thc', 'lsd', 'psilocybin', 'dmt', 'mdma', 'flow']:
        result = run_benchmark(state, tier=4)
        assert result['avg_match'] > 0
        assert len(result['variables']) == 12

def test_compare_states():
    results = compare_states(['thc', 'lsd', 'dmt'], tier=3)
    assert len(results) == 3
    assert all('avg_match' in r for r in results)

def test_tier_comparison_thc():
    results = run_tier_comparison('thc')
    assert len(results) >= 4
    assert results[-1]['avg_match'] >= results[0]['avg_match']

def test_benchmark_includes_tension():
    result = run_benchmark('thc', tier=3)
    assert 'tension' in result
    assert 'T_total' in result['tension']
