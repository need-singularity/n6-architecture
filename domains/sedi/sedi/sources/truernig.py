"""TrueRNG USB hardware random number generator source.

Hardware: TrueRNG v3 (~$50)
Appears as serial device, outputs raw random bytes.
"""
import time
import numpy as np
from typing import Generator


def stream(port='/dev/tty.usbmodem-TrueRNG', batch=1024,
           interval=0.5) -> Generator:
    """Stream random bytes from TrueRNG USB device.

    Args:
        port: Serial port (auto-detect attempted if not specified)
        batch: Bytes per read
        interval: Seconds between reads
    """
    try:
        import serial
    except ImportError:
        print("  [truernig] pyserial required: pip install pyserial")
        return

    # Auto-detect TrueRNG
    if port is None:
        import serial.tools.list_ports
        for p in serial.tools.list_ports.comports():
            if 'TrueRNG' in (p.description or '') or 'TrueRNG' in (p.product or ''):
                port = p.device
                break
        if port is None:
            print("  [truernig] No TrueRNG device found")
            return

    try:
        ser = serial.Serial(port, timeout=1)
        ser.flushInput()
    except Exception as e:
        print(f"  [truernig] Cannot open {port}: {e}")
        return

    try:
        while True:
            raw = ser.read(batch)
            if len(raw) == 0:
                time.sleep(interval)
                continue
            values = list(raw)  # bytes 0-255
            yield {
                'source': 'truernig',
                'timestamp': time.time(),
                'data': values,
                'n': len(values),
            }
            time.sleep(interval)
    finally:
        ser.close()
