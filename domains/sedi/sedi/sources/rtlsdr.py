"""RTL-SDR radio spectrum source.

Hardware: RTL-SDR USB dongle (~$25)
Requires: pip install pyrtlsdr
"""
import time
import numpy as np
from typing import Generator


def check_available():
    """Check if RTL-SDR hardware is available."""
    try:
        from rtlsdr import RtlSdr
        sdr = RtlSdr()
        sdr.close()
        return True
    except Exception:
        return False


def stream(center_freq=100e6, sample_rate=2.4e6, gain='auto',
           batch=1024, interval=1.0) -> Generator:
    """Stream IQ samples from RTL-SDR.

    Args:
        center_freq: Center frequency in Hz (default 100 MHz)
        sample_rate: Sample rate in Hz (default 2.4 MHz)
        gain: Gain setting ('auto' or float in dB)
        batch: Number of samples per read
        interval: Seconds between reads
    """
    try:
        from rtlsdr import RtlSdr
    except ImportError:
        print("  [rtl-sdr] pyrtlsdr required: pip install pyrtlsdr")
        print("  [rtl-sdr] Also need: librtlsdr (brew install librtlsdr)")
        return

    sdr = RtlSdr()
    sdr.sample_rate = sample_rate
    sdr.center_freq = center_freq
    sdr.gain = gain

    try:
        while True:
            iq_samples = sdr.read_samples(batch)
            magnitude = np.abs(iq_samples)
            yield {
                'source': 'rtl-sdr',
                'timestamp': time.time(),
                'data': magnitude.tolist(),
                'n': len(magnitude),
                'center_freq': center_freq,
                'sample_rate': sample_rate,
            }
            time.sleep(interval)
    finally:
        sdr.close()


def scan_band(start_freq=88e6, end_freq=108e6, step=0.5e6,
              sample_rate=2.4e6, dwell=0.1):
    """Scan a frequency band and return power spectrum.

    Default: FM band 88-108 MHz.
    """
    try:
        from rtlsdr import RtlSdr
    except ImportError:
        print("  [rtl-sdr] pyrtlsdr required")
        return []

    sdr = RtlSdr()
    sdr.sample_rate = sample_rate
    sdr.gain = 'auto'

    results = []
    freq = start_freq
    try:
        while freq <= end_freq:
            sdr.center_freq = freq
            time.sleep(dwell)
            samples = sdr.read_samples(1024)
            power = float(np.mean(np.abs(samples) ** 2))
            results.append({
                'freq': freq,
                'power': power,
            })
            freq += step
    finally:
        sdr.close()

    return results
