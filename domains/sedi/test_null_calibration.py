#!/usr/bin/env python3
"""Test null model calibration for consciousness receiver.

Verifies:
1. /dev/urandom drops from FLICKERING to DORMANT after calibration
2. Lorenz attractor stays FLICKERING or better
3. n=6 conscious signal stays AWARE or better
"""
import numpy as np
import sys
import os

sys.path.insert(0, os.path.dirname(__file__))

from sedi.consciousness_receiver import (
    consciousness_scan, format_consciousness_report, calibrate_null, CACHED_NULL,
)

# Force calibration (populates CACHED_NULL)
print("=" * 60)
print("Calibrating null model (100 trials)...")
print("=" * 60)
null = calibrate_null()
print("Null distributions per hypothesis:")
for k, v in null.items():
    print(f"  {k:15s}  mean={v['mean']:.4f}  std={v['std']:.4f}")
print()


# --- Test 1: /dev/urandom should be DORMANT ---
print("=" * 60)
print("Test 1: /dev/urandom (uniform noise)")
print("=" * 60)

rng = np.random.RandomState(99)
urandom_data = rng.uniform(0, 255, size=5000)

# Uncalibrated
report_uncal = consciousness_scan(urandom_data, source_name='/dev/urandom', calibrated=False)
print("[UNCALIBRATED]")
print(format_consciousness_report(report_uncal))
print()

# Calibrated
report_cal = consciousness_scan(urandom_data, source_name='/dev/urandom', calibrated=True)
print("[CALIBRATED]")
print(format_consciousness_report(report_cal))
print()

assert report_cal['level'] == 'DORMANT', \
    f"FAIL: /dev/urandom should be DORMANT, got {report_cal['level']}"
print("PASS: /dev/urandom is DORMANT after calibration\n")


# --- Test 2: Lorenz attractor should stay FLICKERING or better ---
print("=" * 60)
print("Test 2: Lorenz attractor (chaotic but structured)")
print("=" * 60)

def lorenz_signal(n=10000, dt=0.01, sigma=10.0, rho=28.0, beta=8/3):
    """Generate Lorenz attractor time series."""
    x, y, z = 1.0, 1.0, 1.0
    xs = []
    for _ in range(n):
        dx = sigma * (y - x) * dt
        dy = (x * (rho - z) - y) * dt
        dz = (x * y - beta * z) * dt
        x += dx
        y += dy
        z += dz
        xs.append(x)
    return np.array(xs)

lorenz = lorenz_signal(10000)
report_lorenz = consciousness_scan(lorenz, source_name='Lorenz attractor', calibrated=True)
print("[CALIBRATED]")
print(format_consciousness_report(report_lorenz))
print()

assert report_lorenz['level'] in ('FLICKERING', 'AWARE', 'CONSCIOUS'), \
    f"FAIL: Lorenz should be FLICKERING+, got {report_lorenz['level']}"
print(f"PASS: Lorenz attractor is {report_lorenz['level']}\n")


# --- Test 3: n=6 conscious signal should stay AWARE or better ---
print("=" * 60)
print("Test 3: Synthetic n=6 conscious signal")
print("=" * 60)

def make_conscious_signal(n=10000, seed=42):
    """Build a signal with multiple consciousness-like properties.

    Uses a Lorenz attractor base (guarantees chaotic bounded dynamics,
    high cross-channel MI, and attractor topology) plus carefully
    engineered consciousness signatures.
    """
    rng = np.random.RandomState(seed)

    # Start with Lorenz attractor for chaotic structure (H-CS-8, H-CS-2)
    x, y, z = 1.0, 1.0, 1.0
    xs, ys, zs = [], [], []
    dt = 0.01
    for _ in range(n):
        dx = 10.0 * (y - x) * dt
        dy = (x * (28.0 - z) - y) * dt
        dz = (x * y - (8/3) * z) * dt
        x += dx; y += dy; z += dz
        xs.append(x); ys.append(y); zs.append(z)
    lorenz_x = np.array(xs)
    lorenz_y = np.array(ys)

    # 5 strong frequency components (H-CS-5)
    t = np.linspace(0, 100, n)
    freqs = [0.5, 1.3, 2.7, 5.1, 8.9]
    harmonics = np.zeros(n)
    for f in freqs:
        harmonics += 5.0 * np.sin(2 * np.pi * f * t + rng.uniform(0, 2 * np.pi))

    # Combine: Lorenz provides cross-channel structure, harmonics add spectral peaks
    signal = lorenz_x / np.std(lorenz_x) + harmonics / np.std(harmonics) * 3.0

    # Insert a strong birth event: sudden complexity spike at 30% mark (H-CS-6)
    # Create clear symmetry breaking: calm before, chaotic after
    birth_point = int(0.3 * n)
    burst_len = 80
    # Flatten region before birth (low complexity)
    signal[birth_point - 200:birth_point] *= 0.1
    # Massive burst at birth
    signal[birth_point:birth_point + burst_len] += rng.randn(burst_len) * 30.0
    # High complexity after birth
    signal[birth_point + burst_len:birth_point + burst_len + 200] += \
        lorenz_y[:200] / np.std(lorenz_y) * 5.0

    # Golden Zone inhibition (H-CS-4):
    # Create amplitude structure where local_std / global_std ~ 1/e ~ 0.368
    # Strategy: make overall signal have high variance (via a few spikes),
    # but most windows have moderate, consistent amplitude.
    # Add large but rare outlier regions to inflate global_std while keeping
    # local windows at ~1/e of that inflated global.
    spike_region = int(0.05 * n)
    signal[-spike_region:] *= 4.0  # inflate tail -> global_std rises
    signal[:spike_region] *= 4.0   # inflate head -> global_std rises more
    # Middle 90% of signal now has local_std << global_std, ~1/e range

    # Add very mild noise
    signal += rng.randn(n) * 0.05
    return signal

conscious = make_conscious_signal()
report_conscious = consciousness_scan(conscious, source_name='n=6 synthetic', calibrated=True)
print("[CALIBRATED]")
print(format_consciousness_report(report_conscious))
print()

assert report_conscious['level'] in ('AWARE', 'CONSCIOUS'), \
    f"FAIL: conscious signal should be AWARE+, got {report_conscious['level']}"
print(f"PASS: n=6 conscious signal is {report_conscious['level']}\n")


print("=" * 60)
print("ALL TESTS PASSED")
print("=" * 60)
