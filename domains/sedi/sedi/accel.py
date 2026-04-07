"""
sedi.accel — Acceleration layer for SEDI signal processing.

Tries to import the compiled Rust extension (sedi_core) first.
Falls back to a pure-NumPy implementation if the extension is not available
(e.g. before `maturin develop` has been run).

Public API
----------
fast_fft(data, window_sizes)          -> dict[int, np.ndarray]
fast_detect_ratios(data, targets, tol)-> list[RatioHit | dict]
fast_runs_test(data)                   -> tuple[int, float, float]
fast_entropy(data, n_bins)             -> float

Build the Rust extension
------------------------
    cd sedi-core
    pip install maturin          # one-time
    maturin develop              # debug build  (fast compile)
    maturin develop --release    # release build (fast runtime, ~30 s)
"""

from __future__ import annotations

import logging
import math
from dataclasses import dataclass
from typing import Any

import numpy as np

logger = logging.getLogger(__name__)

# ---------------------------------------------------------------------------
# Try importing the compiled Rust extension
# ---------------------------------------------------------------------------
try:
    import sedi_core as _rust  # type: ignore[import]

    _RUST_AVAILABLE = True
    logger.debug("sedi_core Rust extension loaded — using accelerated paths.")
except ImportError:
    _rust = None  # type: ignore[assignment]
    _RUST_AVAILABLE = False
    logger.debug(
        "sedi_core Rust extension not found — falling back to NumPy. "
        "Run `cd sedi-core && maturin develop --release` to enable acceleration."
    )


def rust_available() -> bool:
    """Return True if the compiled Rust extension is available."""
    return _RUST_AVAILABLE


# ---------------------------------------------------------------------------
# Shared result type (used by both paths so callers see a consistent API)
# ---------------------------------------------------------------------------


@dataclass
class RatioHit:
    """A ratio match returned by fast_detect_ratios."""

    pos: int
    target: float
    observed: float
    z_score: float

    def __repr__(self) -> str:
        return (
            f"RatioHit(pos={self.pos}, target={self.target:.4f}, "
            f"observed={self.observed:.4f}, z_score={self.z_score:.3f})"
        )


# ---------------------------------------------------------------------------
# 1. fast_fft
# ---------------------------------------------------------------------------


def fast_fft(
    data: list[float] | np.ndarray,
    window_sizes: list[int],
) -> dict[int, np.ndarray]:
    """Windowed FFT magnitude spectra at multiple window sizes.

    Parameters
    ----------
    data:
        Real-valued signal samples.
    window_sizes:
        List of window sizes.  Each must be <= len(data).

    Returns
    -------
    dict mapping window_size -> 1-D magnitude array of length window_size//2+1.
    A Hann window is applied before the transform.  The most recent
    `window_size` samples are used (real-time / streaming convention).
    """
    data_list = list(data) if not isinstance(data, list) else data

    if _RUST_AVAILABLE:
        raw = _rust.fast_fft(data_list, list(window_sizes))
        # Values arrive as plain Python lists from Rust; wrap in ndarray.
        return {k: np.asarray(v, dtype=np.float64) for k, v in raw.items()}

    # --- NumPy fallback ---
    arr = np.asarray(data_list, dtype=np.float64)
    result: dict[int, np.ndarray] = {}
    for win in window_sizes:
        if win <= 0:
            raise ValueError("window_size must be > 0")
        if win > len(arr):
            raise ValueError(
                f"window_size {win} > data length {len(arr)}"
            )
        segment = arr[-win:]
        window = np.hanning(win)
        spectrum = np.fft.rfft(segment * window)
        magnitudes = np.abs(spectrum) / win
        result[win] = magnitudes
    return result


# ---------------------------------------------------------------------------
# 2. fast_detect_ratios
# ---------------------------------------------------------------------------


def fast_detect_ratios(
    data: list[float] | np.ndarray,
    targets: list[float],
    tolerance: float,
) -> list[RatioHit]:
    """Scan consecutive element ratios and return target matches.

    Parameters
    ----------
    data:
        Input signal.
    targets:
        Ratio values to search for (e.g. [1.618, 2.0, 0.5]).
    tolerance:
        Absolute tolerance: a hit is recorded when |observed - target| <= tolerance.

    Returns
    -------
    List of RatioHit objects sorted by position.
    """
    data_list = list(data) if not isinstance(data, list) else data

    if _RUST_AVAILABLE:
        raw_hits = _rust.fast_detect_ratios(data_list, list(targets), tolerance)
        # Convert Rust RatioHit objects to our Python dataclass.
        return [
            RatioHit(
                pos=h.pos,
                target=h.target,
                observed=h.observed,
                z_score=h.z_score,
            )
            for h in raw_hits
        ]

    # --- NumPy fallback ---
    arr = np.asarray(data_list, dtype=np.float64)
    if len(arr) < 2:
        raise ValueError("data must have at least 2 elements")

    denominators = arr[1:]
    valid_mask = denominators != 0.0
    indices = np.where(valid_mask)[0]
    ratios = arr[indices] / arr[indices + 1]

    if len(ratios) == 0:
        return []

    mean = ratios.mean()
    std = ratios.std()

    hits: list[RatioHit] = []
    for pos, observed in zip(indices.tolist(), ratios.tolist()):
        for target in targets:
            if abs(observed - target) <= tolerance:
                z_score = (observed - mean) / std if std > 1e-12 else 0.0
                hits.append(
                    RatioHit(
                        pos=int(pos),
                        target=float(target),
                        observed=float(observed),
                        z_score=float(z_score),
                    )
                )
    return hits


# ---------------------------------------------------------------------------
# 3. fast_runs_test
# ---------------------------------------------------------------------------


def fast_runs_test(
    data: list[float] | np.ndarray,
) -> tuple[int, float, float]:
    """Wald-Wolfowitz runs test for randomness.

    Parameters
    ----------
    data:
        Input sequence.  Must have >= 2 elements.

    Returns
    -------
    (runs, expected, z_score) where:
        runs     — observed number of runs above/below the median.
        expected — expected runs under independence (H0).
        z_score  — standard normal statistic; |z| > 1.96 → p < 0.05.
    """
    data_list = list(data) if not isinstance(data, list) else data

    if _RUST_AVAILABLE:
        return _rust.fast_runs_test(data_list)  # type: ignore[return-value]

    # --- NumPy fallback ---
    arr = np.asarray(data_list, dtype=np.float64)
    if len(arr) < 2:
        raise ValueError("data must have at least 2 elements")

    median = np.median(arr)
    # Drop ties with median.
    binary = arr[arr != median] > median
    n = len(binary)
    if n < 2:
        return (0, 0.0, 0.0)

    n1 = int(binary.sum())
    n2 = n - n1

    if n1 == 0 or n2 == 0:
        return (1, 1.0, math.inf)

    # Count runs.
    runs = int(np.sum(binary[1:] != binary[:-1])) + 1

    n1f, n2f, nf = float(n1), float(n2), float(n)
    expected = (2.0 * n1f * n2f / nf) + 1.0
    variance = (2.0 * n1f * n2f * (2.0 * n1f * n2f - nf)) / (
        nf * nf * (nf - 1.0)
    )
    z_score = (runs - expected) / math.sqrt(variance) if variance > 1e-12 else 0.0

    return (runs, expected, z_score)


# ---------------------------------------------------------------------------
# 4. fast_entropy
# ---------------------------------------------------------------------------


def fast_entropy(
    data: list[float] | np.ndarray,
    n_bins: int = 50,
) -> float:
    """Binned Shannon entropy of a signal.

    Parameters
    ----------
    data:
        Input samples.
    n_bins:
        Number of equal-width histogram bins (default 50).

    Returns
    -------
    Shannon entropy in nats.  Maximum is ln(n_bins) for a uniform distribution.
    """
    data_list = list(data) if not isinstance(data, list) else data

    if _RUST_AVAILABLE:
        return _rust.fast_entropy(data_list, n_bins)

    # --- NumPy fallback ---
    arr = np.asarray(data_list, dtype=np.float64)
    if len(arr) == 0:
        return 0.0
    if n_bins < 2:
        raise ValueError("n_bins must be >= 2")

    counts, _ = np.histogram(arr, bins=n_bins)
    probs = counts[counts > 0] / len(arr)
    return float(-np.sum(probs * np.log(probs)))


# ---------------------------------------------------------------------------
# Convenience re-exports
# ---------------------------------------------------------------------------

__all__ = [
    "rust_available",
    "RatioHit",
    "fast_fft",
    "fast_detect_ratios",
    "fast_runs_test",
    "fast_entropy",
]
