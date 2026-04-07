"""EEG Consciousness Analysis — bridges EEG data with SEDI consciousness detection.

Runs the 8-hypothesis consciousness_receiver on EEG channels and maps
band powers to TECS-L G=D*P/I variables. Supports file comparison
(awake vs sleep) and real-time monitoring from OpenBCI hardware.

Usage:
    from sedi.eeg_consciousness import eeg_consciousness_scan, compare_states

    # Single file scan
    report = eeg_consciousness_scan('session.csv')

    # Compare awake vs sleep
    compare_states('awake.csv', 'sleep.csv')

    # Real-time from OpenBCI
    realtime_monitor('/dev/tty.usbserial-DM00CXN8')
"""

import time
import warnings
from typing import Dict, List, Optional, Tuple

import numpy as np

from .consciousness_receiver import consciousness_scan, format_consciousness_report
from .sources.eeg import (
    load_openbci_csv,
    load_edf,
    load_npy,
    preprocess,
    extract_bands,
    map_to_consciousness,
    compute_asymmetry,
    stream_from_openbci,
    generate_synthetic_eeg,
    print_gdpi_report,
    CHANNEL_LABELS_16,
    BANDS,
    HAS_SCIPY,
)


def _load_auto(filepath: str) -> Tuple[np.ndarray, float, List[str]]:
    """Auto-detect file format and load."""
    fp = filepath.lower()
    if fp.endswith('.edf') or fp.endswith('.bdf'):
        return load_edf(filepath)
    elif fp.endswith('.npy'):
        return load_npy(filepath)
    else:
        return load_openbci_csv(filepath)


def eeg_consciousness_scan(eeg_data, srate: float = 250,
                           ch_labels: List[str] = None,
                           source_name: str = 'eeg') -> Dict:
    """Run consciousness detection on EEG data.

    Runs the 8-hypothesis consciousness_receiver on:
      1. Each individual EEG channel
      2. Combined (mean) signal across all channels
      3. Band-specific signals (alpha, beta, gamma)
    Also computes TECS-L G=D*P/I mapping from band powers.

    Args:
        eeg_data: either a filepath (str) or np.ndarray (n_channels, n_samples)
        srate: sample rate (used if eeg_data is array)
        ch_labels: channel names (used if eeg_data is array)
        source_name: label for the report

    Returns:
        Dict with 'channels', 'combined', 'bands', 'gdpi', and 'summary'.
    """
    # Load if filepath
    if isinstance(eeg_data, str):
        data, srate, ch_labels = _load_auto(eeg_data)
        source_name = eeg_data
    else:
        data = np.asarray(eeg_data, dtype=np.float64)
        if ch_labels is None:
            ch_labels = [f'ch{i}' for i in range(data.shape[0])]

    # Preprocess
    if HAS_SCIPY:
        clean = preprocess(data, srate)
    else:
        clean = data.copy()
        warnings.warn("scipy not available, skipping preprocessing")

    n_channels, n_samples = clean.shape

    # Extract band powers
    if HAS_SCIPY:
        bands = extract_bands(clean, srate)
        gdpi = map_to_consciousness(bands, ch_labels)
    else:
        bands = None
        gdpi = None

    # Run consciousness scan on each channel
    channel_reports = {}
    for ch in range(n_channels):
        label = ch_labels[ch] if ch < len(ch_labels) else f'ch{ch}'
        try:
            report = consciousness_scan(clean[ch], source_name=f'{source_name}/{label}')
            channel_reports[label] = report
        except Exception as e:
            channel_reports[label] = {'error': str(e), 'source': label}

    # Combined signal (mean across channels)
    combined_signal = np.mean(clean, axis=0)
    combined_report = consciousness_scan(combined_signal, source_name=f'{source_name}/combined')

    # Band-specific consciousness scans (filter then scan)
    band_reports = {}
    if HAS_SCIPY:
        from scipy import signal as scipy_signal
        nyquist = srate / 2.0
        for band_name, (fmin, fmax) in BANDS.items():
            if fmax >= nyquist:
                fmax = nyquist - 1.0
            try:
                b, a = scipy_signal.butter(4, [fmin / nyquist, fmax / nyquist], btype='band')
                band_signal = scipy_signal.filtfilt(b, a, combined_signal)
                band_reports[band_name] = consciousness_scan(
                    band_signal, source_name=f'{source_name}/band_{band_name}'
                )
            except Exception as e:
                band_reports[band_name] = {'error': str(e)}

    # Summary statistics
    n_channels_conscious = sum(
        1 for r in channel_reports.values()
        if r.get('n_detected', 0) >= 4  # AWARE or above
    )
    channel_scores = [
        r.get('n_detected', 0) for r in channel_reports.values()
    ]

    summary = {
        'source': source_name,
        'n_channels': n_channels,
        'n_samples': n_samples,
        'duration_sec': n_samples / srate,
        'srate': srate,
        'combined_level': combined_report.get('level', 'unknown'),
        'combined_detected': combined_report.get('n_detected', 0),
        'channels_aware_plus': n_channels_conscious,
        'channel_mean_score': float(np.mean(channel_scores)) if channel_scores else 0,
        'channel_max_score': max(channel_scores) if channel_scores else 0,
    }

    if gdpi:
        summary['G_genius'] = gdpi['G_genius']
        summary['in_golden_zone'] = gdpi['in_golden_zone']

    return {
        'summary': summary,
        'channels': channel_reports,
        'combined': combined_report,
        'bands': band_reports,
        'gdpi': gdpi,
        'band_powers': bands,
    }


def compare_states(awake_file: str, sleep_file: str,
                   verbose: bool = True) -> Dict:
    """Compare consciousness levels between two EEG recordings.

    Typical use: compare awake (eyes-open resting) vs sleep (N2/N3)
    to verify that consciousness metrics track wakefulness.

    Args:
        awake_file: path to awake-state EEG recording
        sleep_file: path to sleep-state EEG recording
        verbose: print comparison report

    Returns:
        Dict with awake/sleep reports and comparison metrics.
    """
    print(f"Loading awake: {awake_file}")
    awake_report = eeg_consciousness_scan(awake_file, source_name='awake')

    print(f"Loading sleep: {sleep_file}")
    sleep_report = eeg_consciousness_scan(sleep_file, source_name='sleep')

    # Comparison metrics
    aw = awake_report['summary']
    sl = sleep_report['summary']

    comparison = {
        'awake_level': aw['combined_level'],
        'sleep_level': sl['combined_level'],
        'awake_score': aw['combined_detected'],
        'sleep_score': sl['combined_detected'],
        'score_difference': aw['combined_detected'] - sl['combined_detected'],
        'awake_mean_channel_score': aw['channel_mean_score'],
        'sleep_mean_channel_score': sl['channel_mean_score'],
    }

    # G=D*P/I comparison
    if awake_report.get('gdpi') and sleep_report.get('gdpi'):
        aw_g = awake_report['gdpi']
        sl_g = sleep_report['gdpi']
        comparison['awake_G'] = aw_g['G_genius']
        comparison['sleep_G'] = sl_g['G_genius']
        comparison['G_difference'] = aw_g['G_genius'] - sl_g['G_genius']
        comparison['awake_I'] = aw_g['I_inhibition']
        comparison['sleep_I'] = sl_g['I_inhibition']
        comparison['awake_P'] = aw_g['P_plasticity']
        comparison['sleep_P'] = sl_g['P_plasticity']

    # Band power comparison
    if awake_report.get('band_powers') and sleep_report.get('band_powers'):
        aw_bp = awake_report['band_powers']
        sl_bp = sleep_report['band_powers']
        comparison['band_ratios'] = {}
        for band_name in BANDS:
            if band_name in aw_bp and band_name in sl_bp:
                aw_mean = float(np.mean(aw_bp[band_name]))
                sl_mean = float(np.mean(sl_bp[band_name]))
                comparison['band_ratios'][band_name] = {
                    'awake': aw_mean,
                    'sleep': sl_mean,
                    'ratio': aw_mean / (sl_mean + 1e-12),
                }

    # Validation: awake should have higher consciousness scores
    comparison['valid'] = comparison['score_difference'] > 0

    if verbose:
        _print_comparison(comparison)

    return {
        'awake': awake_report,
        'sleep': sleep_report,
        'comparison': comparison,
    }


def _print_comparison(comparison: Dict):
    """Print awake vs sleep comparison."""
    print()
    print("=" * 65)
    print("  EEG Consciousness Comparison: Awake vs Sleep")
    print("=" * 65)
    print()
    print(f"  {'Metric':<30s} {'Awake':>12s} {'Sleep':>12s} {'Delta':>10s}")
    print(f"  {'-'*30} {'-'*12} {'-'*12} {'-'*10}")
    print(f"  {'Consciousness level':<30s} {comparison['awake_level']:>12s} {comparison['sleep_level']:>12s}")
    print(f"  {'Hypotheses detected (of 8)':<30s} {comparison['awake_score']:>12d} {comparison['sleep_score']:>12d} {comparison['score_difference']:>+10d}")
    print(f"  {'Mean channel score':<30s} {comparison['awake_mean_channel_score']:>12.1f} {comparison['sleep_mean_channel_score']:>12.1f}")

    if 'awake_G' in comparison:
        print()
        print("  --- G = D x P / I ---")
        print(f"  {'G (genius quotient)':<30s} {comparison['awake_G']:>12.6f} {comparison['sleep_G']:>12.6f} {comparison['G_difference']:>+10.6f}")
        print(f"  {'I (inhibition/alpha)':<30s} {comparison['awake_I']:>12.6f} {comparison['sleep_I']:>12.6f}")
        print(f"  {'P (plasticity/gamma)':<30s} {comparison['awake_P']:>12.6f} {comparison['sleep_P']:>12.6f}")

    if 'band_ratios' in comparison:
        print()
        print("  --- Band Power Ratios (Awake/Sleep) ---")
        for band, vals in comparison['band_ratios'].items():
            print(f"    {band:>8s}: awake={vals['awake']:8.2f}  sleep={vals['sleep']:8.2f}  ratio={vals['ratio']:6.2f}x")

    print()
    if comparison['valid']:
        print("  [PASS] Awake consciousness > Sleep consciousness")
    else:
        print("  [FAIL] Expected awake > sleep, but scores are equal or reversed")
    print("=" * 65)


def realtime_monitor(port: str = '/dev/tty.usbserial',
                     srate: int = 250,
                     n_channels: int = 16,
                     update_interval: float = 1.0):
    """Live consciousness level display from OpenBCI hardware.

    Streams EEG, computes band powers and G=D*P/I in real-time,
    and prints an updating dashboard to the terminal.

    Args:
        port: serial port for OpenBCI dongle
        srate: sample rate
        n_channels: 8 (Cyton) or 16 (Cyton+Daisy)
        update_interval: seconds between display updates
    """
    ch_labels = CHANNEL_LABELS_16[:n_channels]
    chunk_samples = int(srate * update_interval)

    print("=" * 60)
    print("  EEG Real-time Consciousness Monitor")
    print(f"  Port: {port}  |  {n_channels}ch @ {srate}Hz")
    print("  Press Ctrl+C to stop")
    print("=" * 60)
    print()

    # History for trend display
    g_history = []
    level_history = []
    epoch = 0

    try:
        for chunk in stream_from_openbci(port, srate, n_channels, chunk_samples):
            epoch += 1
            ts = time.strftime('%H:%M:%S')

            # Preprocess this chunk
            try:
                clean = preprocess(chunk, srate)
            except Exception:
                clean = chunk

            # Band powers + GDPI
            try:
                bands = extract_bands(clean, srate)
                gdpi = map_to_consciousness(bands, ch_labels)
            except Exception as e:
                print(f"  [{ts}] Error: {e}")
                continue

            G = gdpi['G_genius']
            I = gdpi['I_inhibition']
            P = gdpi['P_plasticity']
            D = gdpi['D_deficit']
            gz = gdpi['in_golden_zone']

            g_history.append(G)
            if len(g_history) > 60:
                g_history = g_history[-60:]

            # Quick consciousness scan on combined signal
            combined = np.mean(clean, axis=0)
            try:
                cs = consciousness_scan(combined, source_name='live')
                level = cs.get('level', '?')
                n_det = cs.get('n_detected', 0)
            except Exception:
                level = '?'
                n_det = 0

            level_history.append(level)

            # Display
            gz_mark = " [GOLDEN]" if gz else ""
            g_bar_len = min(int(G * 40), 40)
            g_bar = "#" * g_bar_len + "." * (40 - g_bar_len)

            # Clear and redraw (simple terminal update)
            print(f"\033[2J\033[H", end="")  # clear screen
            print("=" * 60)
            print(f"  EEG Consciousness Monitor  |  {ts}  |  epoch {epoch}")
            print("=" * 60)
            print()
            print(f"  G = D x P / I = {G:.6f}{gz_mark}")
            print(f"    I (inhibition) = {I:.6f}  (alpha)")
            print(f"    P (plasticity) = {P:.6f}  (gamma)")
            print(f"    D (deficit)    = {D:.6f}  (asymmetry)")
            print()
            print(f"  G: [{g_bar}] {G:.4f}")
            print(f"  Golden Zone: [{gdpi['golden_zone'][0]:.4f}, {gdpi['golden_zone'][1]:.4f}]")
            print()
            print(f"  Consciousness: {level} ({n_det}/8 hypotheses)")
            print()

            # Band power summary
            print("  Band Powers (mean uV^2/Hz):")
            for band_name in ['delta', 'theta', 'alpha', 'beta', 'gamma']:
                if band_name in bands and not band_name.startswith('_'):
                    bp_mean = float(np.mean(bands[band_name]))
                    bar = "#" * min(int(bp_mean / 10), 30)
                    print(f"    {band_name:>8s}: {bp_mean:8.2f}  |{bar}")

            # G trend (last 30 epochs)
            if len(g_history) > 1:
                print()
                recent = g_history[-30:]
                trend = "rising" if recent[-1] > recent[0] else "falling"
                print(f"  G trend ({len(recent)} epochs): {trend}  "
                      f"mean={np.mean(recent):.4f}  std={np.std(recent):.4f}")

            print()
            print("  [Ctrl+C to stop]")

    except KeyboardInterrupt:
        print("\n\n  Monitor stopped.")
        if g_history:
            print(f"  Session: {len(g_history)} epochs")
            print(f"  G mean={np.mean(g_history):.6f}  std={np.std(g_history):.6f}")
            print(f"  G min={np.min(g_history):.6f}  max={np.max(g_history):.6f}")
    except ImportError as e:
        print(f"\n  Cannot connect to OpenBCI: {e}")
        print("  Install pyserial: pip install pyserial")
        print("  Or test with synthetic data: eeg_consciousness_scan(generate_synthetic_eeg())")


def format_eeg_report(result: Dict) -> str:
    """Format full EEG consciousness scan result."""
    lines = []
    s = result['summary']

    lines.append("=" * 65)
    lines.append("  EEG Consciousness Analysis Report")
    lines.append("=" * 65)
    lines.append(f"  Source:     {s['source']}")
    lines.append(f"  Channels:  {s['n_channels']}")
    lines.append(f"  Duration:  {s['duration_sec']:.1f}s @ {s['srate']:.0f}Hz")
    lines.append(f"  Samples:   {s['n_samples']}")
    lines.append("")

    # Combined consciousness
    lines.append(f"  Combined consciousness: {s['combined_level']} ({s['combined_detected']}/8 hypotheses)")
    lines.append(f"  Channels at AWARE+:     {s['channels_aware_plus']}/{s['n_channels']}")
    lines.append(f"  Mean channel score:     {s['channel_mean_score']:.1f}/8")
    lines.append("")

    # GDPI
    if result.get('gdpi'):
        gdpi = result['gdpi']
        lines.append("  --- G = D x P / I (TECS-L) ---")
        lines.append(f"  I = {gdpi['I_inhibition']:.6f}  P = {gdpi['P_plasticity']:.6f}  D = {gdpi['D_deficit']:.6f}")
        lines.append(f"  G = {gdpi['G_genius']:.6f}  {'[GOLDEN ZONE]' if gdpi['in_golden_zone'] else ''}")
        lines.append("")

    # Per-channel summary
    lines.append("  --- Per-Channel Scores ---")
    for label, report in result['channels'].items():
        n_det = report.get('n_detected', 0)
        level = report.get('level', '?')
        bar = "#" * n_det + "." * (8 - n_det)
        lines.append(f"    {label:>5s}: [{bar}] {n_det}/8 = {level}")

    # Band consciousness
    if result.get('bands'):
        lines.append("")
        lines.append("  --- Band-Specific Consciousness ---")
        for band_name, report in result['bands'].items():
            if 'error' not in report:
                n_det = report.get('n_detected', 0)
                level = report.get('level', '?')
                lines.append(f"    {band_name:>8s}: {n_det}/8 = {level}")

    # Combined consciousness detail
    lines.append("")
    lines.append("  --- Combined Signal Detail ---")
    lines.append(format_consciousness_report(result['combined']))

    lines.append("=" * 65)

    report_str = '\n'.join(lines)
    print(report_str)
    return report_str
