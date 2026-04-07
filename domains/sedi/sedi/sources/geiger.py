"""Geiger counter radiation source.

Hardware: USB Geiger counter (~$50) or serial Geiger-Muller tube
Requires: pip install pyserial
"""
import time
import numpy as np
from typing import Generator


def stream(port='/dev/tty.usbserial-0001', baudrate=9600,
           window=60, interval=1.0) -> Generator:
    """Stream radiation counts from USB Geiger counter.

    Most USB Geiger counters output CPM (counts per minute) or
    pulse timestamps over serial.

    Args:
        port: Serial port (macOS: /dev/tty.usbserial-*, Linux: /dev/ttyUSB*)
        baudrate: Serial baud rate
        window: Rolling window in seconds for CPM calculation
        interval: Read interval in seconds
    """
    try:
        import serial
    except ImportError:
        print("  [geiger] pyserial required: pip install pyserial")
        return

    try:
        ser = serial.Serial(port, baudrate, timeout=1)
    except Exception as e:
        print(f"  [geiger] Cannot open {port}: {e}")
        print(f"  [geiger] Available ports: python3 -m serial.tools.list_ports")
        return

    counts = []
    try:
        while True:
            line = ser.readline().decode('ascii', errors='ignore').strip()
            if not line:
                time.sleep(interval)
                continue

            # Parse count value (format varies by device)
            try:
                # Common formats: "CPM: 15", "15", "CPS: 0.25"
                val = float(''.join(c for c in line if c.isdigit() or c == '.'))
                counts.append((time.time(), val))
            except ValueError:
                continue

            # Trim to window
            now = time.time()
            counts = [(t, v) for t, v in counts if now - t < window]

            if len(counts) >= 2:
                values = [v for _, v in counts]
                yield {
                    'source': 'geiger',
                    'timestamp': now,
                    'data': values,
                    'n': len(values),
                    'cpm': sum(values) * 60 / window,
                    'raw_line': line,
                }
    finally:
        ser.close()


def simulate(mean_cpm=15, duration=60, interval=1.0) -> Generator:
    """Simulate Geiger counter with Poisson-distributed counts.

    Default: ~15 CPM (background radiation level).
    """
    while True:
        # Poisson count for this interval
        expected = mean_cpm * interval / 60
        count = float(np.random.poisson(expected))
        yield {
            'source': 'geiger-sim',
            'timestamp': time.time(),
            'data': [count],
            'n': 1,
            'cpm': count * 60 / interval,
        }
        time.sleep(interval)
