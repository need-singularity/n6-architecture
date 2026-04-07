"""Multi-source parallel monitor — the heart of SEDI.

Runs all available data sources in parallel threads,
applies R-filter + PH detection to each stream,
aggregates anomalies across sources.
"""
import threading
import queue
import time
import numpy as np
from .filter import r_filter
from .detector import analyze, format_alert, log_alert
from .constants import ALERT_YELLOW


class SEDIMonitor:
    """Parallel multi-source signal monitor."""

    def __init__(self, alert_threshold=ALERT_YELLOW, logfile='sedi_alerts.jsonl'):
        self.alert_queue = queue.Queue()
        self.threads = []
        self.running = False
        self.alert_threshold = alert_threshold
        self.logfile = logfile
        self.stats = {}

    def _source_worker(self, source_name, stream_func, stream_kwargs):
        """Worker thread for a single data source."""
        self.stats[source_name] = {'batches': 0, 'alerts': 0, 'errors': 0}
        try:
            for batch in stream_func(**stream_kwargs):
                if not self.running:
                    break
                try:
                    data = np.array(batch['data'], dtype=float)
                    result = r_filter(data)
                    alerts = analyze(result, source_name=source_name)

                    self.stats[source_name]['batches'] += 1

                    for alert in alerts:
                        if alert['z_score'] >= self.alert_threshold:
                            self.stats[source_name]['alerts'] += 1
                            self.alert_queue.put(alert)
                except Exception as e:
                    self.stats[source_name]['errors'] += 1
        except Exception as e:
            print(f"  [{source_name}] Stream error: {e}")

    def add_source(self, name, stream_func, **kwargs):
        """Register a data source for parallel monitoring."""
        t = threading.Thread(
            target=self._source_worker,
            args=(name, stream_func, kwargs),
            daemon=True,
        )
        self.threads.append((name, t))

    def start(self, duration=None):
        """Start all source threads and process alerts."""
        self.running = True
        print(f"🛸 SEDI Monitor — Starting {len(self.threads)} sources")
        print(f"   Alert threshold: Z > {self.alert_threshold}")
        print()

        for name, t in self.threads:
            print(f"   📡 Starting {name}...")
            t.start()

        print()
        start_time = time.time()

        try:
            while self.running:
                try:
                    alert = self.alert_queue.get(timeout=1.0)
                    msg = format_alert(alert)
                    print(msg)
                    log_alert(alert, self.logfile)
                except queue.Empty:
                    # Status update every 30s
                    elapsed = time.time() - start_time
                    if int(elapsed) % 30 == 0:
                        total_batches = sum(s['batches'] for s in self.stats.values())
                        total_alerts = sum(s['alerts'] for s in self.stats.values())
                        ts = time.strftime('%H:%M:%S')
                        print(f"   [{ts}] {total_batches} batches, {total_alerts} alerts", end='\r')

                if duration and (time.time() - start_time) > duration:
                    break

        except KeyboardInterrupt:
            print("\n   Stopping...")

        self.running = False
        self._print_summary()

    def _print_summary(self):
        """Print final summary."""
        print(f"\n{'='*60}")
        print(f"   SEDI Monitor Summary")
        print(f"{'='*60}")
        for name, stats in self.stats.items():
            print(f"   {name}: {stats['batches']} batches, "
                  f"{stats['alerts']} alerts, {stats['errors']} errors")
        total = sum(s['alerts'] for s in self.stats.values())
        print(f"\n   Total alerts: {total}")
        print(f"   Log file: {self.logfile}")


def run_all_available(duration=60):
    """Start monitoring with all available sources.

    Auto-detects: quantum RNG (always), RTL-SDR, Geiger, TrueRNG.
    """
    monitor = SEDIMonitor()

    # Always available: Quantum RNG
    from .sources.quantum_rng import stream as qrng_stream
    monitor.add_source('quantum-rng', qrng_stream, interval=5.0, batch=1024)

    # Check RTL-SDR
    try:
        from .sources.rtlsdr import check_available
        if check_available():
            from .sources.rtlsdr import stream as sdr_stream
            monitor.add_source('rtl-sdr', sdr_stream, interval=1.0)
            print("   📡 RTL-SDR detected!")
    except Exception:
        pass

    # Check serial devices (Geiger, TrueRNG, Temperature)
    try:
        import serial.tools.list_ports
        for port in serial.tools.list_ports.comports():
            desc = (port.description or '').lower()
            if 'truernig' in desc or 'truerng' in desc:
                from .sources.truernig import stream as trng_stream
                monitor.add_source('truernig', trng_stream, port=port.device)
                print(f"   🎲 TrueRNG detected on {port.device}!")
            elif 'geiger' in desc or 'gm' in desc:
                from .sources.geiger import stream as geiger_stream
                monitor.add_source('geiger', geiger_stream, port=port.device)
                print(f"   🔬 Geiger counter detected on {port.device}!")
    except ImportError:
        pass

    # Bitcoin (always available, slow)
    from .sources.bitcoin import stream as btc_stream
    monitor.add_source('bitcoin', btc_stream, interval=120)

    monitor.start(duration=duration)
    return monitor
