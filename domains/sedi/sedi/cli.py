#!/usr/bin/env python3
"""SEDI CLI — Search for Extra-Dimensional Intelligence."""
import argparse
import sys
import time
import numpy as np
from . import __version__
from .constants import *
from .filter import r_filter
from .detector import analyze, format_alert, log_alert


def cmd_listen(args):
    """Continuous listener on a data source."""
    print(f"🛸 SEDI v{__version__} — Listening on {args.source}")
    print(f"   Tuned to n={N}: f=1/{SIGMA*PHI}, delta=1/{N} & 1/{TAU}")
    print(f"   Alert threshold: Z > {ALERT_YELLOW}")
    print()

    if args.source == 'quantum-rng':
        from .sources.quantum_rng import stream
        for batch in stream(interval=args.interval, batch=args.batch):
            data = np.array(batch['data'], dtype=float)
            result = r_filter(data)
            alerts = analyze(result, source_name='quantum-rng')

            if alerts:
                for alert in alerts:
                    msg = format_alert(alert)
                    print(msg)
                    log_alert(alert, args.log)
            elif not args.quiet:
                ts = time.strftime('%H:%M:%S')
                print(f"⚪ [{ts}] quantum-rng: {len(data)} samples, no anomaly", end='\r')

            if not args.continuous:
                break

    elif args.source == 'test':
        # Generate test data with planted n=6 signal
        print("   [test mode] Planting n=6 signal...")
        data = np.random.randn(1024)
        # Plant ratio 1/6 signal
        for i in range(0, 1000, 6):
            data[i] = data[i] * (1 + DELTA_PLUS)
        result = r_filter(data)
        alerts = analyze(result, source_name='test')
        for alert in alerts:
            print(format_alert(alert))
        if not alerts:
            print("   No anomalies detected in test signal")

    else:
        print(f"   Unknown source: {args.source}")
        print(f"   Available: quantum-rng, ligo, test")


def cmd_receive(args):
    """Universal signal receiver — PRIMARY mode."""
    print(f"🛸 SEDI v{__version__} — Universal Receiver")
    print(f"   Mode: {'ALL anomalies' if not args.n6 else 'n=6 only'}")
    print(f"   Sensitivity: {args.sensitivity}σ")
    print()

    from .receiver import SignalReceiver
    rx = SignalReceiver(sensitivity=args.sensitivity, ph_enabled=not args.no_ph,
                        distribution=args.distribution)

    if args.source == 'quantum-rng':
        from .sources.quantum_rng import stream
        # Calibrate with first batch
        print("   Calibrating on first batch...")
        first = next(stream(batch=args.batch))
        rx.calibrate(np.array(first['data'], dtype=float))
        print(f"   Baseline: mean={rx.baseline['mean']:.1f}, std={rx.baseline['std']:.1f}")
        print()

        for batch in stream(interval=args.interval, batch=args.batch):
            data = np.array(batch['data'], dtype=float)
            result = rx.receive(data)

            if result['verdict'] != 'NOISE':
                ts = time.strftime('%H:%M:%S')
                icon = '🔴' if result['verdict'] == 'SIGNAL' else '🟡'
                print(f"   {icon} [{ts}] {result['verdict']} "
                      f"strength={result['signal_strength']:.1f}σ "
                      f"anomalies={len(result['anomalies'])}")
                for a in result['anomalies']:
                    print(f"      {a['type']}: {a['detail']}")

                # Secondary: n=6 check
                if not args.n6:
                    from .filter import r_filter
                    from .detector import analyze, format_alert
                    r6 = r_filter(data)
                    alerts = analyze(r6, 'quantum-rng')
                    for alert in alerts:
                        print(f"      🔷 n=6: {format_alert(alert)}")
                print()
            elif not args.quiet:
                ts = time.strftime('%H:%M:%S')
                print(f"   ⚪ [{ts}] NOISE ({len(data)} samples)", end='\r')

            if not args.continuous:
                break

    elif args.source == 'test-signal':
        print("   Generating test: noise + hidden 6Hz sine...")
        noise = np.random.randn(2000)
        rx.calibrate(noise[:1000])
        t = np.linspace(0, 10, 1000)
        signal = noise[1000:] + float(args.amplitude) * np.sin(2*np.pi*6*t)
        result = rx.receive(signal)
        print(f"   Verdict: {result['verdict']} (strength={result['signal_strength']:.1f}σ)")
        for a in result['anomalies']:
            print(f"   {a['type']}: {a['detail']}")

    elif args.source == 'test-noise':
        print("   Generating pure noise (should be NOISE)...")
        noise = np.random.randn(2000)
        rx.calibrate(noise[:1000])
        result = rx.receive(noise[1000:])
        print(f"   Verdict: {result['verdict']} (strength={result['signal_strength']:.1f}σ)")
        if result['anomalies']:
            for a in result['anomalies']:
                print(f"   {a['type']}: {a['detail']}")
        else:
            print("   Clean — no false positives!")

    else:
        print(f"   Sources: quantum-rng, test-signal, test-noise")


def cmd_scan(args):
    """Scan a data file."""
    print(f"🛸 SEDI v{__version__} — Scanning {args.file}")

    if args.source == 'ligo':
        from .sources.ligo import load_strain_file
        result = load_strain_file(args.file)
        if result is None:
            print("   Failed to load file")
            return
        data = np.array(result['data'])
        print(f"   Loaded: {result['n']} samples, {result['sample_rate']} Hz")
    else:
        # Generic CSV/text
        data = np.loadtxt(args.file)
        print(f"   Loaded: {len(data)} values")

    result = r_filter(data)
    alerts = analyze(result, source_name=args.source)

    print(f"\n   Results: {len(alerts)} anomalies")
    for alert in alerts:
        print(f"   {format_alert(alert)}")

    if not alerts:
        print("   ⚪ No n=6 patterns detected")


def cmd_status(args):
    """Show SEDI status and constants."""
    print(f"🛸 SEDI v{__version__}")
    print(f"   Search for Extra-Dimensional Intelligence")
    print()
    print(f"   === Tuning Constants (n={N}) ===")
    print(f"   sigma    = {SIGMA}")
    print(f"   phi      = {PHI}")
    print(f"   tau      = {TAU}")
    print(f"   sopfr    = {SOPFR}")
    print(f"   focal    = 1/{SIGMA*PHI} = {FOCAL:.6f}")
    print(f"   delta+   = 1/{N} = {DELTA_PLUS:.6f}")
    print(f"   delta-   = 1/{TAU} = {DELTA_MINUS:.6f}")
    print(f"   theta_E  = sqrt(3/2) = {EINSTEIN_THETA:.6f}")
    print(f"   bandwidth= ln(4/3) = {GOLDEN_WIDTH:.6f}")
    print()
    print(f"   === Alert Thresholds ===")
    print(f"   🔴 RED    Z > {ALERT_RED}")
    print(f"   🟠 ORANGE Z > {ALERT_ORANGE}")
    print(f"   🟡 YELLOW Z > {ALERT_YELLOW}")
    print()
    print(f"   === Data Sources ===")
    print(f"   quantum-rng  ANU Quantum Random Numbers (free API)")
    print(f"   ligo         LIGO Open Science Center (HDF5 files)")
    print(f"   test         Synthetic test signal with planted n=6")


def main():
    parser = argparse.ArgumentParser(
        prog='sedi',
        description='🛸 SEDI — Search for Extra-Dimensional Intelligence'
    )
    parser.add_argument('--version', action='version', version=f'sedi {__version__}')
    sub = parser.add_subparsers(dest='command')

    # receive (PRIMARY — universal anomaly detection)
    p_rx = sub.add_parser('receive', help='Universal signal receiver (PRIMARY)')
    p_rx.add_argument('--source', default='quantum-rng', help='quantum-rng, test-signal, test-noise')
    p_rx.add_argument('--continuous', action='store_true', help='Run forever')
    p_rx.add_argument('--interval', type=float, default=5.0, help='Poll interval')
    p_rx.add_argument('--batch', type=int, default=1024, help='Batch size')
    p_rx.add_argument('--sensitivity', type=float, default=3.0, help='Z-score threshold')
    p_rx.add_argument('--amplitude', type=float, default=1.0, help='Test signal amplitude')
    p_rx.add_argument('--no-ph', action='store_true', help='Disable PH (faster)')
    p_rx.add_argument('--n6', action='store_true', help='n=6 mode only (skip general)')
    p_rx.add_argument('--quiet', action='store_true', help='Only show signals')
    p_rx.add_argument('--distribution', default='auto',
                      choices=['auto', 'normal', 'uniform'],
                      help='Expected noise distribution: auto (detect), normal, uniform (default: auto)')

    # listen (SECONDARY — n=6 specific)
    p_listen = sub.add_parser('listen', help='n=6 pattern listener (SECONDARY)')
    p_listen.add_argument('--source', default='quantum-rng', help='Data source')
    p_listen.add_argument('--continuous', action='store_true', help='Run forever')
    p_listen.add_argument('--interval', type=float, default=5.0, help='Poll interval (s)')
    p_listen.add_argument('--batch', type=int, default=1024, help='Batch size')
    p_listen.add_argument('--quiet', action='store_true', help='Only show alerts')
    p_listen.add_argument('--log', default='sedi_alerts.jsonl', help='Alert log file')

    # scan
    p_scan = sub.add_parser('scan', help='Scan a data file')
    p_scan.add_argument('--source', default='generic', help='Source type')
    p_scan.add_argument('--file', required=True, help='Data file path')

    # monitor (all sources parallel)
    p_monitor = sub.add_parser('monitor', help='Monitor all sources in parallel')
    p_monitor.add_argument('--duration', type=int, default=0, help='Duration in seconds (0=forever)')
    p_monitor.add_argument('--log', default='sedi_alerts.jsonl', help='Alert log file')

    # history (past data)
    p_hist = sub.add_parser('history', help='Scan historical data')
    p_hist.add_argument('--source', default='all',
                        help='Source: earthquake, solar, ligo-catalog, cern-masses, cern-analysis, oeis, all')
    p_hist.add_argument('--start', type=int, default=2020, help='Start year')
    p_hist.add_argument('--end', type=int, default=2025, help='End year')
    p_hist.add_argument('--query', default='perfect number', help='OEIS search query')
    p_hist.add_argument('--mc-trials', type=int, default=10000,
                        help='Monte Carlo trials for cern-analysis (default: 10000)')

    # status
    sub.add_parser('status', help='Show SEDI status')

    # dashboard (web UI)
    from .dashboard import _build_parser as _dash_parser
    _dash_parser(sub)

    args = parser.parse_args()
    if args.command == 'receive':
        cmd_receive(args)
    elif args.command == 'listen':
        cmd_listen(args)
    elif args.command == 'scan':
        cmd_scan(args)
    elif args.command == 'monitor':
        from .monitor import run_all_available
        dur = args.duration if args.duration > 0 else None
        run_all_available(duration=dur)
    elif args.command == 'history':
        from .historical import (scan_all, scan_earthquake_history,
                                  scan_solar_history, scan_ligo_catalog,
                                  scan_cern_masses, scan_oeis)
        if args.source == 'all':
            scan_all()
        elif args.source == 'earthquake':
            scan_earthquake_history(args.start, args.end)
        elif args.source == 'solar':
            scan_solar_history(args.start, args.end)
        elif args.source == 'ligo-catalog':
            scan_ligo_catalog()
        elif args.source == 'cern-masses':
            scan_cern_masses()
        elif args.source == 'cern-analysis':
            from .sources.cern_analysis import run_full_analysis
            mc = getattr(args, 'mc_trials', 10000)
            tol = getattr(args, 'tolerance', 0.03)
            run_full_analysis(mc_trials=mc, tolerance=tol)
        elif args.source == 'oeis':
            scan_oeis(args.query)
        else:
            print(f"Unknown source: {args.source}")
    elif args.command == 'status':
        cmd_status(args)
    elif args.command == 'dashboard':
        from .dashboard import cmd_dashboard
        cmd_dashboard(args)
    else:
        parser.print_help()


if __name__ == '__main__':
    main()
