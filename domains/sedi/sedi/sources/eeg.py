"""EEG data source for SEDI — OpenBCI + EDF loading, preprocessing, and TECS-L mapping.

Designed for OpenBCI Cyton+Daisy 16ch hardware. All heavy dependencies
(mne, scipy.signal, serial) are optional — the module loads without them
and raises clear errors only when the missing dep is actually needed.

TECS-L Mapping (G = D x P / I):
    Alpha power      -> Inhibition (I)   — higher alpha = more cortical inhibition
    Gamma power      -> Plasticity (P)   — higher gamma = more neural plasticity
    Alpha asymmetry  -> Deficit (D)      — frontal asymmetry indicates deficit pattern
    G = D x P / I    -> Genius quotient  — consciousness flow metric

Usage:
    from sedi.sources.eeg import load_openbci_csv, preprocess, extract_bands, map_to_consciousness
    data, srate, ch_labels = load_openbci_csv('session.csv')
    clean = preprocess(data, srate)
    bands = extract_bands(clean, srate)
    gdpi = map_to_consciousness(bands, ch_labels)
"""

import math
import struct
import warnings
from pathlib import Path
from typing import Dict, Generator, List, Optional, Tuple

import numpy as np

# ── Optional imports ──

try:
    from scipy import signal as scipy_signal
    HAS_SCIPY = True
except ImportError:
    HAS_SCIPY = False

try:
    import mne
    HAS_MNE = True
except ImportError:
    HAS_MNE = False

try:
    import serial
    HAS_SERIAL = True
except ImportError:
    HAS_SERIAL = False


def _require(lib_name: str, has_flag: bool):
    if not has_flag:
        raise ImportError(
            f"{lib_name} is required for this function. "
            f"Install with: pip install {lib_name}"
        )


# ── Constants ──

BANDS = {
    'delta': (0.5, 4),
    'theta': (4, 8),
    'alpha': (8, 12),
    'beta':  (13, 30),
    'gamma': (30, 100),
}

# 10-20 system channel labels for Cyton+Daisy 16ch
CHANNEL_LABELS_16 = [
    'Fp1', 'Fp2', 'C3', 'C4', 'P7', 'P8', 'O1', 'O2',  # Cyton 8ch
    'F7', 'F8', 'F3', 'F4', 'T7', 'T8', 'P3', 'P4',     # Daisy 8ch
]

# Standard 10-20 hemisphere pairs (left, right)
HEMISPHERE_PAIRS = [
    ('Fp1', 'Fp2'), ('F3', 'F4'), ('F7', 'F8'),
    ('C3', 'C4'), ('T7', 'T8'),
    ('P3', 'P4'), ('P7', 'P8'),
    ('O1', 'O2'),
]

# Frontal channels for I (inhibition) computation
FRONTAL_CHANNELS = ['Fp1', 'Fp2', 'F3', 'F4', 'F7', 'F8']

# Golden zone bounds from TECS-L: 0.5 - ln(4/3) < G < 0.5
GOLDEN_LOWER = 0.5 - math.log(4 / 3)  # 0.2123
GOLDEN_UPPER = 0.5


# ══════════════════════════════════════════════════════════════════
# 1. Data Loading
# ══════════════════════════════════════════════════════════════════

def load_openbci_csv(filepath: str) -> Tuple[np.ndarray, float, List[str]]:
    """Load OpenBCI CSV format (16 channels + timestamps).

    OpenBCI GUI exports CSV with columns:
        Sample Index, EEG Ch1, EEG Ch2, ..., EEG Ch16, Accel X, Y, Z, Timestamp
    The first few rows may contain header comments starting with '%'.
    Values are in microvolts (uV).

    Args:
        filepath: Path to .csv file from OpenBCI GUI or collect.py

    Returns:
        (data, sample_rate, channel_labels)
        data shape: (n_channels, n_samples), float64, in uV
        sample_rate: detected or default 250 Hz
        channel_labels: list of channel names
    """
    filepath = Path(filepath)
    if not filepath.exists():
        raise FileNotFoundError(f"EEG file not found: {filepath}")

    # Read, skipping comment lines
    lines = []
    header_line = None
    with open(filepath, 'r') as f:
        for line in f:
            stripped = line.strip()
            if not stripped or stripped.startswith('%'):
                continue
            if header_line is None and not stripped[0].isdigit() and not stripped[0] == '-':
                header_line = stripped
                continue
            lines.append(stripped)

    if not lines:
        raise ValueError(f"No data rows found in {filepath}")

    # Parse CSV
    rows = []
    for line in lines:
        values = [v.strip() for v in line.split(',')]
        try:
            row = [float(v) for v in values if v]
            rows.append(row)
        except ValueError:
            continue

    raw = np.array(rows)

    # Detect format: OpenBCI GUI vs our collect.py format
    if header_line:
        cols = [c.strip() for c in header_line.split(',')]
        # collect.py format: timestamp, Fp1, Fp2, ... columns
        if cols[0].lower() == 'timestamp' and len(cols) > 1:
            timestamps = raw[:, 0]
            eeg_data = raw[:, 1:].T  # (n_channels, n_samples)
            ch_labels = cols[1:]
            # Estimate sample rate from timestamps
            if len(timestamps) > 1:
                dt = np.median(np.diff(timestamps))
                srate = 1.0 / dt if dt > 0 else 250.0
            else:
                srate = 250.0
            return eeg_data, float(srate), ch_labels

    # OpenBCI GUI format: Sample Index, Ch1..Ch16, Accel, Timestamp(unix)
    n_cols = raw.shape[1]
    if n_cols >= 18:  # index + 16ch + at least 1 extra
        eeg_data = raw[:, 1:17].T  # columns 1-16 are EEG
        ch_labels = CHANNEL_LABELS_16[:16]
    elif n_cols >= 10:  # index + 8ch + extras
        eeg_data = raw[:, 1:9].T
        ch_labels = CHANNEL_LABELS_16[:8]
    else:
        # Assume all columns are EEG channels
        eeg_data = raw.T
        ch_labels = [f'ch{i}' for i in range(eeg_data.shape[0])]

    srate = 250.0  # OpenBCI Cyton default
    return eeg_data, srate, ch_labels


def load_edf(filepath: str) -> Tuple[np.ndarray, float, List[str]]:
    """Load EDF/BDF format (standard clinical EEG).

    Uses mne-python if available, otherwise falls back to manual EDF parsing
    (header only — reads the fixed 256-byte header + per-signal headers).

    Args:
        filepath: Path to .edf or .bdf file

    Returns:
        (data, sample_rate, channel_labels)
        data shape: (n_channels, n_samples)
    """
    filepath = Path(filepath)
    if not filepath.exists():
        raise FileNotFoundError(f"EDF file not found: {filepath}")

    # Prefer mne-python
    if HAS_MNE:
        raw = mne.io.read_raw_edf(str(filepath), preload=True, verbose=False)
        data = raw.get_data()  # (n_channels, n_samples) in volts
        data = data * 1e6  # convert to uV
        srate = raw.info['sfreq']
        ch_labels = raw.ch_names
        return data, float(srate), list(ch_labels)

    # Manual EDF parsing (basic)
    return _parse_edf_manual(filepath)


def _parse_edf_manual(filepath: Path) -> Tuple[np.ndarray, float, List[str]]:
    """Minimal EDF parser — no dependencies beyond numpy.

    Reads EDF (European Data Format) per the spec:
    https://www.edfplus.info/specs/edf.html
    """
    with open(filepath, 'rb') as f:
        # Fixed header (256 bytes)
        version = f.read(8).decode('ascii').strip()
        _patient = f.read(80)
        _recording = f.read(80)
        _start_date = f.read(8)
        _start_time = f.read(8)
        header_bytes = int(f.read(8).decode('ascii').strip())
        _reserved = f.read(44)
        n_records = int(f.read(8).decode('ascii').strip())
        record_duration = float(f.read(8).decode('ascii').strip())
        n_signals = int(f.read(4).decode('ascii').strip())

        # Per-signal headers (n_signals x 256 bytes total)
        labels = [f.read(16).decode('ascii').strip() for _ in range(n_signals)]
        _transducer = [f.read(80) for _ in range(n_signals)]
        _units = [f.read(8).decode('ascii').strip() for _ in range(n_signals)]
        phys_min = [float(f.read(8).decode('ascii').strip()) for _ in range(n_signals)]
        phys_max = [float(f.read(8).decode('ascii').strip()) for _ in range(n_signals)]
        dig_min = [float(f.read(8).decode('ascii').strip()) for _ in range(n_signals)]
        dig_max = [float(f.read(8).decode('ascii').strip()) for _ in range(n_signals)]
        _prefilter = [f.read(80) for _ in range(n_signals)]
        samples_per_record = [int(f.read(8).decode('ascii').strip()) for _ in range(n_signals)]
        _reserved_sig = [f.read(32) for _ in range(n_signals)]

        # Data records
        f.seek(header_bytes)
        all_data = []
        for _ in range(n_signals):
            all_data.append([])

        for _ in range(n_records):
            for sig in range(n_signals):
                n_samp = samples_per_record[sig]
                raw_bytes = f.read(n_samp * 2)
                values = np.frombuffer(raw_bytes, dtype='<i2')
                all_data[sig].append(values)

    # Concatenate and scale to physical units
    data = np.zeros((n_signals, sum(samples_per_record[0] for _ in range(n_records))))
    for sig in range(n_signals):
        concatenated = np.concatenate(all_data[sig])
        # Digital to physical: phys = (digital - dig_min) * gain + phys_min
        gain = (phys_max[sig] - phys_min[sig]) / (dig_max[sig] - dig_min[sig] + 1e-12)
        data[sig, :len(concatenated)] = (concatenated - dig_min[sig]) * gain + phys_min[sig]

    srate = samples_per_record[0] / record_duration if record_duration > 0 else 250.0
    return data, float(srate), labels


def load_npy(filepath: str, meta_path: str = None) -> Tuple[np.ndarray, float, List[str]]:
    """Load numpy format (from TECS-L collect.py).

    Args:
        filepath: Path to .npy file
        meta_path: Path to _meta.json (auto-detected if None)

    Returns:
        (data, sample_rate, channel_labels)
    """
    import json

    filepath = Path(filepath)
    data = np.load(str(filepath))

    # Find metadata
    if meta_path is None:
        base = str(filepath).replace('.npy', '')
        meta_path = f"{base}_meta.json"

    meta_file = Path(meta_path)
    if meta_file.exists():
        with open(meta_file, 'r') as f:
            meta = json.load(f)
        srate = meta.get('sample_rate', 250)
        ch_labels = meta.get('channels', [f'ch{i}' for i in range(data.shape[0])])
    else:
        srate = 250
        ch_labels = [f'ch{i}' for i in range(data.shape[0])]

    return data, float(srate), ch_labels


# ══════════════════════════════════════════════════════════════════
# 2. Preprocessing
# ══════════════════════════════════════════════════════════════════

def preprocess(data: np.ndarray, srate: float = 250,
               bandpass: Tuple[float, float] = (0.5, 50.0),
               notch_freqs: Tuple[float, ...] = (50.0, 60.0),
               ref_method: str = 'average') -> np.ndarray:
    """Preprocess EEG: bandpass filter, notch filter, re-reference.

    Args:
        data: (n_channels, n_samples) raw EEG in uV
        srate: sampling rate in Hz
        bandpass: (low, high) cutoff frequencies for bandpass filter
        notch_freqs: frequencies to notch out (power line noise)
        ref_method: 're-reference method: 'average', 'none', or channel name

    Returns:
        (n_channels, n_samples) cleaned EEG
    """
    _require('scipy', HAS_SCIPY)

    data = np.asarray(data, dtype=np.float64).copy()
    n_channels, n_samples = data.shape
    nyquist = srate / 2.0

    # Bandpass filter (0.5-50 Hz) — removes DC drift and high-freq noise
    low, high = bandpass
    if high >= nyquist:
        high = nyquist - 1.0
        warnings.warn(f"Bandpass high cutoff adjusted to {high:.1f} Hz (Nyquist={nyquist:.1f})")

    b_bp, a_bp = scipy_signal.butter(4, [low / nyquist, high / nyquist], btype='band')
    for ch in range(n_channels):
        data[ch] = scipy_signal.filtfilt(b_bp, a_bp, data[ch])

    # Notch filters (50 Hz and 60 Hz power line)
    for freq in notch_freqs:
        if freq < nyquist:
            b_notch, a_notch = scipy_signal.iirnotch(freq, Q=30, fs=srate)
            for ch in range(n_channels):
                data[ch] = scipy_signal.filtfilt(b_notch, a_notch, data[ch])

    # Re-reference
    if ref_method == 'average':
        avg_ref = np.mean(data, axis=0)
        data -= avg_ref[np.newaxis, :]
    elif ref_method != 'none':
        warnings.warn(f"Unknown ref_method '{ref_method}', skipping re-reference")

    return data


# ══════════════════════════════════════════════════════════════════
# 3. Band Power Extraction
# ══════════════════════════════════════════════════════════════════

def extract_bands(data: np.ndarray, srate: float = 250,
                  bands: Dict[str, Tuple[float, float]] = None) -> Dict[str, np.ndarray]:
    """Extract frequency band power per channel using Welch's method.

    Args:
        data: (n_channels, n_samples) preprocessed EEG
        srate: sampling rate in Hz
        bands: dict of {name: (fmin, fmax)}. Defaults to standard EEG bands.

    Returns:
        Dict mapping band name -> np.ndarray of shape (n_channels,) with power values.
        Also includes 'relative' key with relative power per band.
    """
    _require('scipy', HAS_SCIPY)

    if bands is None:
        bands = BANDS

    data = np.asarray(data, dtype=np.float64)
    n_channels, n_samples = data.shape
    nperseg = min(256, n_samples)

    result = {}
    for band_name, (fmin, fmax) in bands.items():
        powers = np.zeros(n_channels)
        for ch in range(n_channels):
            freqs, psd = scipy_signal.welch(data[ch], fs=srate, nperseg=nperseg)
            idx = np.logical_and(freqs >= fmin, freqs <= fmax)
            if np.any(idx):
                powers[ch] = np.trapezoid(psd[idx], freqs[idx])
        result[band_name] = powers

    # Compute total and relative power
    total = sum(result.values())
    result['_total'] = total
    result['_relative'] = {}
    for band_name in bands:
        result['_relative'][band_name] = result[band_name] / (total + 1e-12)

    return result


# ══════════════════════════════════════════════════════════════════
# 4. TECS-L Consciousness Mapping (G = D x P / I)
# ══════════════════════════════════════════════════════════════════

def compute_asymmetry(band_powers: Dict[str, np.ndarray],
                      ch_labels: List[str]) -> Dict[str, Dict[str, float]]:
    """Compute hemispheric asymmetry: ln(Right) - ln(Left) for each band.

    Positive asymmetry = right hemisphere dominant.
    """
    ch_idx = {name: i for i, name in enumerate(ch_labels)}
    asymmetry = {}

    for band_name, bp in band_powers.items():
        if band_name.startswith('_'):
            continue
        asym = {}
        for left, right in HEMISPHERE_PAIRS:
            if left in ch_idx and right in ch_idx:
                l_power = bp[ch_idx[left]]
                r_power = bp[ch_idx[right]]
                asym[f"{left}-{right}"] = (
                    np.log(r_power + 1e-12) - np.log(l_power + 1e-12)
                )
        asymmetry[band_name] = asym

    return asymmetry


def map_to_consciousness(bands: Dict[str, np.ndarray],
                         ch_labels: List[str] = None) -> Dict:
    """Map EEG band powers to TECS-L consciousness variables.

    Mapping:
        Alpha power (frontal)   -> I (Inhibition)
        Gamma power (global)    -> P (Plasticity)
        Alpha asymmetry         -> D (Deficit)
        G = D x P / I           -> Genius quotient

    Args:
        bands: output of extract_bands()
        ch_labels: channel names (needed for asymmetry). If None, uses
                   positional indices and skips asymmetry-based D.

    Returns:
        Dict with I, P, D, G values and Golden Zone status.
    """
    relative = bands.get('_relative', {})
    if not relative:
        # Compute relative from absolute
        total = sum(v for k, v in bands.items() if not k.startswith('_'))
        relative = {k: v / (total + 1e-12) for k, v in bands.items() if not k.startswith('_')}

    # I = Frontal Alpha relative power (inhibition)
    if ch_labels:
        ch_idx = {name: i for i, name in enumerate(ch_labels)}
        frontal_idx = [ch_idx[c] for c in FRONTAL_CHANNELS if c in ch_idx]
    else:
        frontal_idx = []

    alpha_rel = relative.get('alpha', np.zeros(1))
    if frontal_idx:
        I = float(np.mean(alpha_rel[frontal_idx]))
    else:
        I = float(np.mean(alpha_rel))

    # P = Global Gamma relative power (plasticity)
    gamma_rel = relative.get('gamma', np.zeros(1))
    P = float(np.mean(gamma_rel))

    # D = Alpha frontal asymmetry magnitude
    D = 0.0
    if ch_labels:
        asymmetry = compute_asymmetry(bands, ch_labels)
        alpha_asym = asymmetry.get('alpha', {})
        frontal_pairs = ['Fp1-Fp2', 'F3-F4', 'F7-F8']
        asym_vals = [abs(alpha_asym[p]) for p in frontal_pairs if p in alpha_asym]
        if asym_vals:
            D = float(np.mean(asym_vals))

    # G = D x P / I
    G = D * P / (I + 1e-12)

    # Golden Zone check
    in_golden_zone = GOLDEN_LOWER <= G <= GOLDEN_UPPER

    return {
        'I_inhibition': I,
        'P_plasticity': P,
        'D_deficit': D,
        'G_genius': G,
        'in_golden_zone': in_golden_zone,
        'golden_zone': [GOLDEN_LOWER, GOLDEN_UPPER],
        'interpretation': {
            'I': 'Frontal alpha relative power (cortical inhibition)',
            'P': 'Global gamma relative power (neural plasticity)',
            'D': 'Alpha frontal asymmetry (deficit/imbalance)',
            'G': 'D x P / I (consciousness flow quotient)',
        },
    }


# ══════════════════════════════════════════════════════════════════
# 5. Live Streaming
# ══════════════════════════════════════════════════════════════════

def stream_from_openbci(port: str = '/dev/tty.usbserial',
                        srate: int = 250,
                        n_channels: int = 16,
                        chunk_samples: int = 250) -> Generator[np.ndarray, None, None]:
    """Generator for live EEG data from OpenBCI Cyton/Daisy via serial.

    Yields chunks of shape (n_channels, chunk_samples).
    Each chunk is ~1 second of data at the default 250 Hz rate.

    The OpenBCI Cyton sends 33-byte packets:
        byte 0: 0xA0 (start)
        byte 1: sample number (0-255)
        bytes 2-25: 8 channels x 3 bytes (24-bit signed, big-endian)
        bytes 26-31: 3 accel axes x 2 bytes (16-bit signed)
        byte 32: 0xCx (stop byte, x = log packet type)

    For Daisy, odd packets contain channels 1-8, even packets 9-16.

    Args:
        port: serial port path (e.g. '/dev/tty.usbserial-DM00CXN8')
        srate: sampling rate (250 for Cyton)
        n_channels: 8 (Cyton) or 16 (Cyton+Daisy)
        chunk_samples: samples per yielded chunk

    Yields:
        np.ndarray of shape (n_channels, chunk_samples)
    """
    _require('pyserial', HAS_SERIAL)

    SCALE_UV = 4.5 / (24 * 2**23)  # ADS1299 gain=24, Vref=4.5V -> uV

    ser = serial.Serial(port, baudrate=115200, timeout=1)
    try:
        # Reset and start streaming
        ser.write(b'v')  # soft reset
        import time
        time.sleep(2)
        ser.write(b'b')  # start streaming
        time.sleep(0.5)
        ser.reset_input_buffer()

        buffer = np.zeros((n_channels, chunk_samples))
        buf_idx = 0
        is_daisy = n_channels > 8
        daisy_buffer = np.zeros(8) if is_daisy else None

        while True:
            # Find start byte
            byte = ser.read(1)
            if byte != b'\xa0':
                continue

            # Read packet (32 remaining bytes)
            packet = ser.read(32)
            if len(packet) != 32:
                continue

            sample_num = packet[0]

            # Parse 8 channels (24-bit signed, big-endian)
            channels = np.zeros(8)
            for ch in range(8):
                offset = 1 + ch * 3
                raw = (packet[offset] << 16) | (packet[offset + 1] << 8) | packet[offset + 2]
                if raw & 0x800000:  # sign extend
                    raw -= 0x1000000
                channels[ch] = raw * SCALE_UV

            stop_byte = packet[31]

            if is_daisy:
                if sample_num % 2 == 0:
                    # Odd packet = Cyton channels 1-8
                    daisy_buffer[:] = channels
                else:
                    # Even packet = Daisy channels 9-16
                    buffer[:8, buf_idx] = daisy_buffer
                    buffer[8:16, buf_idx] = channels
                    buf_idx += 1
            else:
                buffer[:8, buf_idx] = channels
                buf_idx += 1

            if buf_idx >= chunk_samples:
                yield buffer.copy()
                buf_idx = 0
                buffer[:] = 0

    except GeneratorExit:
        pass
    finally:
        ser.write(b's')  # stop streaming
        ser.close()


# ══════════════════════════════════════════════════════════════════
# 6. Synthetic Data (for testing without hardware)
# ══════════════════════════════════════════════════════════════════

def generate_synthetic_eeg(duration_sec: float = 10.0,
                           srate: float = 250.0,
                           n_channels: int = 16,
                           state: str = 'awake') -> Tuple[np.ndarray, float, List[str]]:
    """Generate synthetic EEG data for testing.

    Creates realistic-ish EEG by summing sine waves at standard band
    frequencies with channel-specific amplitudes and pink noise.

    Args:
        duration_sec: duration in seconds
        srate: sample rate
        n_channels: number of channels
        state: 'awake', 'sleep', 'meditate', or 'focus'
            - awake:    moderate alpha, low delta, some beta/gamma
            - sleep:    strong delta/theta, low alpha/beta/gamma
            - meditate: strong alpha, moderate theta, low beta
            - focus:    strong beta/gamma, low alpha/delta

    Returns:
        (data, srate, channel_labels)
    """
    n_samples = int(duration_sec * srate)
    t = np.arange(n_samples) / srate
    ch_labels = CHANNEL_LABELS_16[:n_channels]

    # State-dependent band amplitudes (uV)
    amp_profiles = {
        'awake':    {'delta': 20, 'theta': 10, 'alpha': 30, 'beta': 15, 'gamma': 5},
        'sleep':    {'delta': 80, 'theta': 40, 'alpha': 5,  'beta': 3,  'gamma': 1},
        'meditate': {'delta': 10, 'theta': 25, 'alpha': 60, 'beta': 8,  'gamma': 3},
        'focus':    {'delta': 5,  'theta': 5,  'alpha': 10, 'beta': 35, 'gamma': 20},
    }
    amps = amp_profiles.get(state, amp_profiles['awake'])

    # Center frequencies for each band
    center_freqs = {
        'delta': 2.0,
        'theta': 6.0,
        'alpha': 10.0,
        'beta':  20.0,
        'gamma': 40.0,
    }

    rng = np.random.default_rng(42)
    data = np.zeros((n_channels, n_samples))

    for ch in range(n_channels):
        # Each band contributes a sine wave + harmonics
        for band_name, amp in amps.items():
            freq = center_freqs[band_name]
            # Slight per-channel variation
            ch_amp = amp * (0.8 + 0.4 * rng.random())
            phase = rng.uniform(0, 2 * np.pi)
            data[ch] += ch_amp * np.sin(2 * np.pi * freq * t + phase)

        # Add pink noise (1/f spectrum)
        white = rng.standard_normal(n_samples)
        fft_w = np.fft.rfft(white)
        freqs = np.fft.rfftfreq(n_samples, 1.0 / srate)
        freqs[0] = 1  # avoid division by zero
        pink_filter = 1.0 / np.sqrt(freqs)
        pink = np.fft.irfft(fft_w * pink_filter, n=n_samples)
        data[ch] += pink * 10  # scale pink noise

        # Add slight asymmetry for odd-indexed channels (left hemisphere)
        # to create a measurable D (deficit) value
        if ch % 2 == 0:
            data[ch] *= 1.05  # slightly stronger left
        else:
            data[ch] *= 0.95  # slightly weaker right

    return data, srate, ch_labels


def print_gdpi_report(gdpi: Dict, bands: Dict = None,
                      ch_labels: List[str] = None) -> str:
    """Format G=D*P/I results as readable report."""
    lines = []
    lines.append("=" * 60)
    lines.append("  EEG -> TECS-L Consciousness Mapping (G = D x P / I)")
    lines.append("=" * 60)
    lines.append("")
    lines.append(f"  I (Inhibition)  = {gdpi['I_inhibition']:.6f}  (frontal alpha)")
    lines.append(f"  P (Plasticity)  = {gdpi['P_plasticity']:.6f}  (global gamma)")
    lines.append(f"  D (Deficit)     = {gdpi['D_deficit']:.6f}  (alpha asymmetry)")
    lines.append(f"  G (Genius)      = {gdpi['G_genius']:.6f}  (D x P / I)")
    lines.append("")
    gz = gdpi['golden_zone']
    lines.append(f"  Golden Zone: [{gz[0]:.4f}, {gz[1]:.4f}]")
    if gdpi['in_golden_zone']:
        lines.append("  >>> IN GOLDEN ZONE <<<")
    else:
        if gdpi['G_genius'] < gz[0]:
            lines.append(f"  Below Golden Zone (need +{gz[0] - gdpi['G_genius']:.4f})")
        else:
            lines.append(f"  Above Golden Zone (excess +{gdpi['G_genius'] - gz[1]:.4f})")

    if bands and ch_labels:
        lines.append("")
        lines.append("  --- Band Power (uV^2/Hz, mean across channels) ---")
        for band_name in ['delta', 'theta', 'alpha', 'beta', 'gamma']:
            if band_name in bands and not band_name.startswith('_'):
                lines.append(f"    {band_name:>8s}: {np.mean(bands[band_name]):10.2f}")

    lines.append("=" * 60)
    report = '\n'.join(lines)
    print(report)
    return report
