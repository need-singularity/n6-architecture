"""Cross-Source Correlation Analysis Engine.

Detects temporal correlations between anomalies in different data sources.
Core question: does a LIGO gravitational wave event coincide with a quantum
RNG anomaly? Does a solar flare affect earthquake patterns with n=6 structure?

Uses:
  - Cross-correlation of event time series
  - Coincidence counting with Monte Carlo significance testing
  - Multi-source scanning with unified event logging
"""
import json
import os
import time
import math
from dataclasses import dataclass, field, asdict
from datetime import datetime, timezone
from pathlib import Path
from typing import List, Dict, Optional, Tuple, Callable, Any

import numpy as np
from scipy import stats

# Default paths
DATA_DIR = Path(__file__).parent.parent / "data"
EVENTS_DIR = DATA_DIR / "events"
CORRELATIONS_DIR = DATA_DIR / "correlations"


# ============================================================
# 1. TimeSeriesEvent dataclass
# ============================================================

@dataclass
class TimeSeriesEvent:
    """A single anomaly event from any data source."""
    timestamp: float           # Unix epoch seconds
    source: str                # e.g. 'ligo', 'earthquake', 'solar', 'qrng'
    score: float               # Anomaly score (SETI z-score or consciousness level)
    metadata: Dict = field(default_factory=dict)

    @property
    def datetime_utc(self) -> datetime:
        return datetime.fromtimestamp(self.timestamp, tz=timezone.utc)

    def to_dict(self) -> Dict:
        d = asdict(self)
        d['datetime_utc'] = self.datetime_utc.isoformat()
        return d

    @classmethod
    def from_dict(cls, d: Dict) -> 'TimeSeriesEvent':
        return cls(
            timestamp=d['timestamp'],
            source=d['source'],
            score=d['score'],
            metadata=d.get('metadata', {}),
        )


# ============================================================
# 2. EventLog class
# ============================================================

class EventLog:
    """Persistent log of cross-source anomaly events."""

    def __init__(self):
        self.events: List[TimeSeriesEvent] = []

    def add_event(self, timestamp: float, source: str,
                  seti_score: float = 0.0, consciousness_level: float = 0.0,
                  metadata: Optional[Dict] = None):
        """Add an event to the log.

        The score is max(seti_score, consciousness_level) so that either
        detection modality can flag an event.
        """
        score = max(seti_score, consciousness_level)
        event = TimeSeriesEvent(
            timestamp=timestamp,
            source=source,
            score=score,
            metadata={
                'seti_score': seti_score,
                'consciousness_level': consciousness_level,
                **(metadata or {}),
            },
        )
        self.events.append(event)
        return event

    def get_events(self, source: Optional[str] = None,
                   time_range: Optional[Tuple[float, float]] = None
                   ) -> List[TimeSeriesEvent]:
        """Filter events by source and/or time range.

        Args:
            source: filter to this source name (None = all)
            time_range: (start_epoch, end_epoch) tuple (None = all)
        """
        result = self.events
        if source is not None:
            result = [e for e in result if e.source == source]
        if time_range is not None:
            t0, t1 = time_range
            result = [e for e in result if t0 <= e.timestamp <= t1]
        return result

    def sources(self) -> List[str]:
        """List unique source names."""
        return sorted(set(e.source for e in self.events))

    def save(self, filepath: Optional[str] = None):
        """Save event log to JSON."""
        if filepath is None:
            EVENTS_DIR.mkdir(parents=True, exist_ok=True)
            filepath = str(EVENTS_DIR / f"events_{int(time.time())}.json")
        data = {
            'saved_at': datetime.now(timezone.utc).isoformat(),
            'n_events': len(self.events),
            'sources': self.sources(),
            'events': [e.to_dict() for e in self.events],
        }
        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2)
        print(f"  [event_log] Saved {len(self.events)} events to {filepath}")
        return filepath

    @classmethod
    def load(cls, filepath: str) -> 'EventLog':
        """Load event log from JSON."""
        log = cls()
        with open(filepath) as f:
            data = json.load(f)
        for ed in data['events']:
            log.events.append(TimeSeriesEvent.from_dict(ed))
        print(f"  [event_log] Loaded {len(log.events)} events from {filepath}")
        return log


# ============================================================
# 3. temporal_correlation
# ============================================================

def temporal_correlation(events_a: List[TimeSeriesEvent],
                         events_b: List[TimeSeriesEvent],
                         max_lag_hours: float = 24,
                         bin_minutes: float = 60) -> Dict:
    """Cross-correlation of two event time series.

    Bins events into time bins, then computes normalized cross-correlation
    at multiple lags from -max_lag to +max_lag.

    Args:
        events_a, events_b: Event lists from two different sources
        max_lag_hours: Maximum lag to test (hours)
        bin_minutes: Time bin width (minutes)

    Returns:
        Dictionary with lag values, correlation coefficients, peak lag, etc.
    """
    if not events_a or not events_b:
        return {'lags_hours': [], 'correlations': [], 'peak_lag_hours': 0,
                'peak_correlation': 0, 'significant': False}

    times_a = np.array([e.timestamp for e in events_a])
    times_b = np.array([e.timestamp for e in events_b])
    scores_a = np.array([e.score for e in events_a])
    scores_b = np.array([e.score for e in events_b])

    # Determine common time range
    t_min = min(times_a.min(), times_b.min())
    t_max = max(times_a.max(), times_b.max())

    bin_sec = bin_minutes * 60
    n_bins = int((t_max - t_min) / bin_sec) + 1
    if n_bins < 3:
        return {'lags_hours': [], 'correlations': [], 'peak_lag_hours': 0,
                'peak_correlation': 0, 'significant': False}

    # Build score-weighted histograms
    bins_a = np.zeros(n_bins)
    bins_b = np.zeros(n_bins)

    for t, s in zip(times_a, scores_a):
        idx = min(int((t - t_min) / bin_sec), n_bins - 1)
        bins_a[idx] += s
    for t, s in zip(times_b, scores_b):
        idx = min(int((t - t_min) / bin_sec), n_bins - 1)
        bins_b[idx] += s

    # Normalize
    a_norm = bins_a - bins_a.mean()
    b_norm = bins_b - bins_b.mean()
    a_std = np.std(a_norm)
    b_std = np.std(b_norm)

    if a_std == 0 or b_std == 0:
        return {'lags_hours': [], 'correlations': [], 'peak_lag_hours': 0,
                'peak_correlation': 0, 'significant': False}

    max_lag_bins = int(max_lag_hours * 60 / bin_minutes)
    max_lag_bins = min(max_lag_bins, n_bins - 1)

    lags = []
    correlations = []

    for lag in range(-max_lag_bins, max_lag_bins + 1):
        if lag >= 0:
            c = np.sum(a_norm[:n_bins - lag] * b_norm[lag:]) / (n_bins * a_std * b_std)
        else:
            c = np.sum(a_norm[-lag:] * b_norm[:n_bins + lag]) / (n_bins * a_std * b_std)
        lags.append(lag * bin_minutes / 60)  # Convert to hours
        correlations.append(float(c))

    lags = np.array(lags)
    correlations = np.array(correlations)

    peak_idx = np.argmax(np.abs(correlations))
    peak_lag = float(lags[peak_idx])
    peak_corr = float(correlations[peak_idx])

    # Significance: correlation > 2/sqrt(N) is roughly 2-sigma
    threshold = 2.0 / np.sqrt(n_bins)
    significant = abs(peak_corr) > threshold

    return {
        'lags_hours': lags.tolist(),
        'correlations': correlations.tolist(),
        'peak_lag_hours': peak_lag,
        'peak_correlation': peak_corr,
        'n_bins': n_bins,
        'bin_minutes': bin_minutes,
        'threshold_2sigma': float(threshold),
        'significant': significant,
    }


# ============================================================
# 4. coincidence_test
# ============================================================

def coincidence_test(events_a: List[TimeSeriesEvent],
                     events_b: List[TimeSeriesEvent],
                     window_seconds: float = 3600,
                     n_mc: int = 10000) -> Dict:
    """Test whether two event sets have more temporal coincidences than chance.

    For each event in A, count how many B events fall within +/- window.
    Compare observed count to Monte Carlo expectation (shuffle B times).

    Args:
        events_a, events_b: Event lists from different sources
        window_seconds: Coincidence window (seconds)
        n_mc: Number of Monte Carlo shuffles

    Returns:
        Dictionary with observed/expected counts, p-value, sigma.
    """
    if not events_a or not events_b:
        return {'observed': 0, 'expected': 0, 'sigma': 0, 'p_value': 1.0,
                'significant': False}

    times_a = np.array(sorted(e.timestamp for e in events_a))
    times_b = np.array(sorted(e.timestamp for e in events_b))

    # Observed coincidences
    observed = _count_coincidences(times_a, times_b, window_seconds)

    # Monte Carlo: shuffle B event times uniformly within their range
    t_min_b = times_b.min()
    t_max_b = times_b.max()
    t_span = t_max_b - t_min_b

    if t_span <= 0:
        return {'observed': observed, 'expected': 0, 'sigma': 0,
                'p_value': 1.0, 'significant': False}

    # Adaptive MC: reduce trials for large B to keep runtime reasonable
    effective_mc = min(n_mc, max(1000, 10000 // max(1, len(times_b) // 500)))

    mc_counts = np.zeros(effective_mc)
    for i in range(effective_mc):
        shuffled_b = t_min_b + np.random.uniform(0, t_span, size=len(times_b))
        shuffled_b.sort()
        mc_counts[i] = _count_coincidences(times_a, shuffled_b, window_seconds)

    expected = float(np.mean(mc_counts))
    std_mc = float(np.std(mc_counts))

    if std_mc > 0:
        sigma = (observed - expected) / std_mc
    else:
        sigma = 0.0 if observed == expected else float('inf')

    # p-value: fraction of MC trials with >= observed coincidences
    p_value = float(np.mean(mc_counts >= observed))
    if p_value == 0:
        p_value = 1.0 / (effective_mc + 1)  # Upper bound

    return {
        'observed': int(observed),
        'expected': round(expected, 2),
        'std': round(std_mc, 2),
        'sigma': round(sigma, 2),
        'p_value': round(p_value, 6),
        'n_mc': effective_mc,
        'window_seconds': window_seconds,
        'n_a': len(times_a),
        'n_b': len(times_b),
        'significant': sigma >= 2.0,
    }


def _count_coincidences(times_a: np.ndarray, times_b: np.ndarray,
                        window: float) -> int:
    """Count number of A events with at least one B event within window.

    Vectorized: for each A, binary-search B for the window bounds.
    """
    lo = np.searchsorted(times_b, times_a - window)
    hi = np.searchsorted(times_b, times_a + window)
    return int(np.sum(hi > lo))


# ============================================================
# 5. multi_source_scan
# ============================================================

def multi_source_scan(sources_config: List[Tuple[str, Callable]],
                      event_log: Optional[EventLog] = None) -> EventLog:
    """Scan multiple data sources, log all events with timestamps.

    Args:
        sources_config: List of (name, fetch_function) tuples.
            Each fetch_function() should return a list of dicts with at least
            'time' (epoch) and some score field ('mag', 'score', 'z_score', etc.)
        event_log: Existing log to append to (creates new if None)

    Returns:
        EventLog with all fetched events
    """
    if event_log is None:
        event_log = EventLog()

    for source_name, fetch_fn in sources_config:
        print(f"  [scan] Fetching {source_name}...")
        try:
            raw_events = fetch_fn()
            if raw_events is None:
                print(f"  [scan] {source_name}: no data returned")
                continue

            n_added = 0
            for evt in raw_events:
                ts = evt.get('time', evt.get('timestamp', 0))
                # Try common score fields
                score = evt.get('score', evt.get('z_score',
                         evt.get('mag', evt.get('magnitude', 1.0))))
                event_log.add_event(
                    timestamp=float(ts),
                    source=source_name,
                    seti_score=float(score),
                    metadata={k: v for k, v in evt.items()
                              if k not in ('time', 'timestamp')},
                )
                n_added += 1
            print(f"  [scan] {source_name}: logged {n_added} events")
        except Exception as e:
            print(f"  [scan] {source_name} error: {e}")

    return event_log


# ============================================================
# 6. find_correlations
# ============================================================

def find_correlations(event_log: EventLog,
                      min_significance: float = 2.0,
                      window_seconds: float = 3600,
                      max_lag_hours: float = 24) -> List[Dict]:
    """Find all significant cross-source correlations in an event log.

    Tests every pair of sources for both temporal cross-correlation
    and coincidence significance.

    Args:
        event_log: EventLog containing multi-source events
        min_significance: Minimum sigma for reporting
        window_seconds: Coincidence window
        max_lag_hours: Max lag for cross-correlation

    Returns:
        List of correlation results sorted by significance
    """
    sources = event_log.sources()
    correlations = []

    for i, src_a in enumerate(sources):
        for src_b in sources[i + 1:]:
            events_a = event_log.get_events(source=src_a)
            events_b = event_log.get_events(source=src_b)

            if len(events_a) < 3 or len(events_b) < 3:
                continue

            print(f"  [correlate] Testing {src_a} ({len(events_a)} events) "
                  f"vs {src_b} ({len(events_b)} events)...")

            # Cross-correlation
            xcorr = temporal_correlation(
                events_a, events_b,
                max_lag_hours=max_lag_hours,
            )

            # Coincidence test
            coinc = coincidence_test(
                events_a, events_b,
                window_seconds=window_seconds,
            )

            # Combined significance: max of either test
            significance = max(
                abs(xcorr.get('peak_correlation', 0))
                / max(xcorr.get('threshold_2sigma', 1), 1e-10) * 2,
                coinc.get('sigma', 0),
            )

            entry = {
                'source_a': src_a,
                'source_b': src_b,
                'n_a': len(events_a),
                'n_b': len(events_b),
                'significance_sigma': round(significance, 2),
                'cross_correlation': xcorr,
                'coincidence': coinc,
                'is_significant': significance >= min_significance,
            }
            correlations.append(entry)

    # Sort by significance descending
    correlations.sort(key=lambda c: c['significance_sigma'], reverse=True)
    return correlations


# ============================================================
# 7. format_correlation_report
# ============================================================

def format_correlation_report(correlations: List[Dict]) -> str:
    """Generate human-readable correlation report."""
    lines = []
    lines.append("=" * 72)
    lines.append("  SEDI Cross-Source Correlation Report")
    lines.append(f"  Generated: {datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M:%S UTC')}")
    lines.append("=" * 72)
    lines.append("")

    if not correlations:
        lines.append("  No source pairs to analyze.")
        return '\n'.join(lines)

    significant = [c for c in correlations if c['is_significant']]
    lines.append(f"  Pairs tested:     {len(correlations)}")
    lines.append(f"  Significant (2s): {len(significant)}")
    lines.append("")

    for c in correlations:
        sig = c['significance_sigma']
        marker = "***" if sig >= 3 else "** " if sig >= 2 else "   "
        lines.append(f"  {marker} {c['source_a']} x {c['source_b']}  "
                      f"({c['n_a']} x {c['n_b']} events)")
        lines.append(f"      Combined significance: {sig:.2f} sigma")

        # Cross-correlation details
        xc = c['cross_correlation']
        if xc.get('peak_correlation') is not None:
            lines.append(f"      Cross-correlation peak: r={xc['peak_correlation']:.4f} "
                          f"at lag={xc['peak_lag_hours']:.1f}h "
                          f"(threshold={xc.get('threshold_2sigma', 0):.4f})")

        # Coincidence test details
        co = c['coincidence']
        lines.append(f"      Coincidences: {co['observed']} observed vs "
                      f"{co['expected']} expected "
                      f"(sigma={co['sigma']:.2f}, p={co['p_value']:.4f})")
        lines.append(f"      Window: {co['window_seconds']}s, MC trials: {co.get('n_mc', 0)}")
        lines.append("")

    # Summary
    lines.append("-" * 72)
    if significant:
        lines.append("  SIGNIFICANT CORRELATIONS DETECTED:")
        for c in significant:
            lag = c['cross_correlation'].get('peak_lag_hours', 0)
            lines.append(f"    {c['source_a']} <-> {c['source_b']}: "
                          f"{c['significance_sigma']:.1f}sigma, lag={lag:.1f}h")
    else:
        lines.append("  No significant correlations found at 2-sigma threshold.")
        lines.append("  This is consistent with independent event sources.")
    lines.append("=" * 72)

    return '\n'.join(lines)


# ============================================================
# 8. demo_ligo_earthquake_correlation
# ============================================================

def _parse_ligo_gps_to_epoch(gps_time: float) -> float:
    """Convert LIGO GPS time to Unix epoch.

    GPS epoch: 1980-01-06 00:00:00 UTC
    Unix epoch: 1970-01-01 00:00:00 UTC
    Offset: 315964800 seconds (+ leap seconds adjustment ~18s)
    """
    GPS_UNIX_OFFSET = 315964800
    LEAP_SECONDS = 18  # approximate for 2015-2023 era
    return gps_time + GPS_UNIX_OFFSET - LEAP_SECONDS


def demo_ligo_earthquake_correlation() -> Dict:
    """Fetch LIGO events and earthquakes, test for temporal correlation.

    Tests whether gravitational wave detections correlate with changes
    in seismic activity -- either as a direct physical coupling or as
    a shared sensitivity to some unknown influence.

    Returns:
        Dictionary with event log, correlations, and formatted report.
    """
    from .sources.ligo import fetch_event_catalog
    from .sources.earthquake import fetch_earthquakes

    print("=" * 60)
    print("  SEDI Demo: LIGO-Earthquake Cross-Correlation")
    print("=" * 60)

    event_log = EventLog()

    # --- Fetch LIGO events ---
    print("\n  [1/3] Fetching LIGO gravitational wave catalog...")
    catalog = fetch_event_catalog()

    ligo_events = []
    if catalog and 'events' in catalog:
        for name, data in catalog['events'].items():
            gps = data.get('GPS', None)
            if gps is None:
                continue
            epoch = _parse_ligo_gps_to_epoch(float(gps))
            # Use FAR (false alarm rate) inverse as score -- rarer = higher score
            far = data.get('far', {})
            if isinstance(far, dict):
                far_val = far.get('median', far.get('best', 1.0))
            else:
                far_val = far if far else 1.0
            # Score: -log10(FAR) so that rarer events score higher
            try:
                score = -math.log10(max(float(far_val), 1e-300))
            except (ValueError, TypeError):
                score = 1.0

            ligo_events.append({
                'name': name,
                'gps': float(gps),
                'epoch': epoch,
                'far': far_val,
                'score': score,
            })

            event_log.add_event(
                timestamp=epoch,
                source='ligo',
                seti_score=score,
                metadata={'event_name': name, 'gps': float(gps)},
            )

        print(f"  Found {len(ligo_events)} LIGO events")
    else:
        print("  WARNING: Could not fetch LIGO catalog")

    # --- Determine earthquake query range from LIGO events ---
    if not ligo_events:
        print("  No LIGO events to correlate. Aborting.")
        return {'event_log': event_log, 'correlations': [], 'report': 'No LIGO data.'}

    ligo_epochs = [e['epoch'] for e in ligo_events]
    t_min = min(ligo_epochs)
    t_max = max(ligo_epochs)

    start_dt = datetime.fromtimestamp(t_min, tz=timezone.utc)
    end_dt = datetime.fromtimestamp(t_max, tz=timezone.utc)
    start_str = start_dt.strftime('%Y-%m-%d')
    end_str = end_dt.strftime('%Y-%m-%d')

    print(f"\n  LIGO event span: {start_str} to {end_str}")

    # --- Fetch earthquakes for the same period ---
    # Break into yearly chunks to stay within USGS limits
    print(f"\n  [2/3] Fetching earthquakes ({start_str} to {end_str}, M>=5.0)...")

    all_eq_events = []
    start_year = start_dt.year
    end_year = end_dt.year

    for year in range(start_year, end_year + 1):
        y_start = f"{year}-01-01"
        y_end = f"{year}-12-31"
        # Clip to LIGO range
        if year == start_year:
            y_start = start_str
        if year == end_year:
            y_end = end_str

        eq = fetch_earthquakes(
            starttime=y_start,
            endtime=y_end,
            minmagnitude=5.0,
            limit=2000,
        )
        if eq:
            all_eq_events.extend(eq)
            print(f"    {year}: {len(eq)} earthquakes")
        time.sleep(0.5)  # Rate limit

    print(f"  Total earthquakes: {len(all_eq_events)}")

    for eq in all_eq_events:
        event_log.add_event(
            timestamp=eq['time'],
            source='earthquake',
            seti_score=eq['mag'],
            metadata={'mag': eq['mag'], 'depth': eq.get('depth', 0),
                      'place': eq.get('place', '')},
        )

    # --- Run correlation analysis ---
    print(f"\n  [3/3] Running cross-correlation analysis...")

    correlations = find_correlations(
        event_log,
        min_significance=2.0,
        window_seconds=3600,    # 1-hour coincidence window
        max_lag_hours=48,       # Test up to 48h lag
    )

    # Also run with tighter windows
    tight_coinc = coincidence_test(
        event_log.get_events(source='ligo'),
        event_log.get_events(source='earthquake'),
        window_seconds=300,     # 5-minute window
    )

    day_coinc = coincidence_test(
        event_log.get_events(source='ligo'),
        event_log.get_events(source='earthquake'),
        window_seconds=86400,   # 24-hour window
    )

    report = format_correlation_report(correlations)

    # Append additional analysis
    extra = [
        "",
        "  Additional coincidence windows:",
        f"    5-min window:  {tight_coinc['observed']} obs vs "
        f"{tight_coinc['expected']} exp (sigma={tight_coinc['sigma']})",
        f"    24-hour window: {day_coinc['observed']} obs vs "
        f"{day_coinc['expected']} exp (sigma={day_coinc['sigma']})",
        "",
    ]
    report += '\n' + '\n'.join(extra)

    print("\n" + report)

    # --- Persist results ---
    EVENTS_DIR.mkdir(parents=True, exist_ok=True)
    CORRELATIONS_DIR.mkdir(parents=True, exist_ok=True)

    events_path = event_log.save(
        str(EVENTS_DIR / "ligo_earthquake_events.json"))
    corr_path = str(CORRELATIONS_DIR / "ligo_earthquake_correlation.json")

    results = {
        'timestamp': datetime.now(timezone.utc).isoformat(),
        'n_ligo': len(ligo_events),
        'n_earthquakes': len(all_eq_events),
        'ligo_span': f"{start_str} to {end_str}",
        'correlations': correlations,
        'tight_5min': tight_coinc,
        'wide_24h': day_coinc,
        'report': report,
    }

    with open(corr_path, 'w') as f:
        json.dump(results, f, indent=2, default=str)
    print(f"  [saved] {corr_path}")

    return results


# ============================================================
# CLI entry point
# ============================================================

if __name__ == '__main__':
    demo_ligo_earthquake_correlation()
