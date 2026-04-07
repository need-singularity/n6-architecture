"""Precision temperature sensor source.

Hardware: USB temperature logger (~$30) or DHT22/BME280 via serial
Also supports: macOS built-in SMC temperature sensors
"""
import time
import subprocess
import numpy as np
from typing import Generator


def stream_mac_smc(interval=5.0) -> Generator:
    """Read macOS SMC temperature sensors (no hardware needed).

    Uses `sudo powermetrics` or `osx-cpu-temp` if available.
    """
    while True:
        try:
            # Try osx-cpu-temp first (brew install osx-cpu-temp)
            result = subprocess.run(
                ['osx-cpu-temp'], capture_output=True, text=True, timeout=5
            )
            if result.returncode == 0:
                temp_str = result.stdout.strip().replace('°C', '').strip()
                temp = float(temp_str)
                yield {
                    'source': 'temperature-smc',
                    'timestamp': time.time(),
                    'data': [temp],
                    'n': 1,
                    'unit': 'celsius',
                }
        except (FileNotFoundError, subprocess.TimeoutExpired, ValueError):
            # Fallback: read from IOKit (macOS)
            try:
                result = subprocess.run(
                    ['sysctl', '-n', 'machdep.xcpm.cpu_thermal_level'],
                    capture_output=True, text=True, timeout=5
                )
                if result.returncode == 0:
                    level = int(result.stdout.strip())
                    yield {
                        'source': 'temperature-thermal-level',
                        'timestamp': time.time(),
                        'data': [float(level)],
                        'n': 1,
                        'unit': 'thermal_level',
                    }
            except Exception:
                pass

        time.sleep(interval)


def stream_serial(port='/dev/tty.usbserial-0001', baudrate=9600,
                  interval=2.0) -> Generator:
    """Read temperature from USB serial sensor (DHT22, BME280, etc.)."""
    try:
        import serial
    except ImportError:
        print("  [temperature] pyserial required: pip install pyserial")
        return

    try:
        ser = serial.Serial(port, baudrate, timeout=2)
    except Exception as e:
        print(f"  [temperature] Cannot open {port}: {e}")
        return

    try:
        while True:
            line = ser.readline().decode('ascii', errors='ignore').strip()
            if not line:
                time.sleep(interval)
                continue
            try:
                temp = float(line.split(',')[0])  # assume CSV: temp,humidity,...
                yield {
                    'source': 'temperature-serial',
                    'timestamp': time.time(),
                    'data': [temp],
                    'n': 1,
                    'unit': 'celsius',
                }
            except (ValueError, IndexError):
                continue
            time.sleep(interval)
    finally:
        ser.close()
