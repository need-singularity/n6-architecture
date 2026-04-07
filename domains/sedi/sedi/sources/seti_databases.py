"""SETI Signal Databases — known candidate signals and parameters.

Hardcoded database of notable SETI candidate signals, including the Wow!
signal, BLC1 (Proxima Centauri candidate), and other historical detections.

These provide reference datasets for n=6 pattern scanning — if consciousness
signatures exist in known SETI signals, this is where we'd find them.
"""
from typing import List, Dict

# ============================================================
# Wow! Signal — August 15, 1977, Ohio State University
# ============================================================
WOW_SIGNAL = {
    'name': 'Wow!',
    'date': '1977-08-15',
    'telescope': 'Big Ear (Ohio State)',
    'frequency_mhz': 1420.4556,      # hydrogen line
    'bandwidth_khz': 10,              # narrowband
    'duration_s': 72,                 # signal duration
    'snr': 30.0,                      # ~30 sigma
    'ra_h': 19.425,                   # approximate RA (hours)
    'dec_deg': -26.85,                # approximate Dec (degrees)
    'intensity_sequence': [6, 14, 26, 30, 19, 7],  # 6EQUJ5 as intensity values
    'character_sequence': '6EQUJ5',   # original notation
    'drift_rate_hz_s': 0.0,           # no detected drift
    'polarization': 'unknown',
    'repeats': 0,                     # never repeated
    'notes': 'Strongest candidate SETI signal. 6-character sequence is intriguing for n=6.',
}

# ============================================================
# BLC1 — December 2020, Breakthrough Listen / Parkes
# ============================================================
BLC1_SIGNAL = {
    'name': 'BLC1',
    'date': '2019-04-29',
    'telescope': 'Parkes 64m',
    'frequency_mhz': 982.002,        # ~982 MHz
    'bandwidth_hz': 2.5,             # extremely narrowband
    'duration_s': 18000,             # ~5 hours intermittent
    'snr': 10.0,                     # approximate
    'ra_h': 14.495,                  # Proxima Centauri RA
    'dec_deg': -62.679,              # Proxima Centauri Dec
    'drift_rate_hz_s': 0.038,        # consistent with transmitter on rotating body
    'target': 'Proxima Centauri',
    'distance_pc': 1.301,
    'status': 'likely RFI (local interference)',
    'notes': 'Most significant BL candidate. Drift rate analysis suggests local origin.',
}

# ============================================================
# Other notable SETI candidates
# ============================================================
HISTORICAL_CANDIDATES = [
    {
        'name': 'SHGb02+14a',
        'date': '2003',
        'telescope': 'Arecibo',
        'frequency_mhz': 1420.405,
        'snr': 5.0,
        'ra_h': 13.44,
        'dec_deg': 14.0,
        'notes': 'SETI@home candidate. Three detections. Controversial.',
    },
    {
        'name': 'Ross 128 signal',
        'date': '2017-05-12',
        'telescope': 'Arecibo',
        'frequency_mhz': 4750.0,       # C-band
        'snr': 8.0,
        'ra_h': 11.795,                # Ross 128 RA
        'dec_deg': 0.816,              # Ross 128 Dec
        'drift_rate_hz_s': 0.0,
        'status': 'likely geostationary satellite',
        'notes': 'Quasi-periodic pulses. Interesting frequency.',
    },
    {
        'name': 'Fast Radio Burst 121102',
        'date': '2012-11-02',
        'telescope': 'Arecibo',
        'frequency_mhz': 1400.0,
        'snr': 20.0,
        'ra_h': 5.527,
        'dec_deg': 33.077,
        'distance_mpc': 972,
        'status': 'natural (magnetar)',
        'notes': 'First repeating FRB. Now known to be from a magnetar.',
    },
    {
        'name': 'KIC 8462852 dips',
        'date': '2015',
        'telescope': 'Kepler',
        'frequency_mhz': 0,             # optical
        'wavelength_nm': 600,            # Kepler bandpass
        'dip_depth_pct': 22.0,          # maximum dip
        'ra_h': 20.103,
        'dec_deg': 44.457,
        'distance_pc': 454,
        'status': 'likely dust (natural)',
        'notes': "Tabby's Star. Irregular, deep dimming events.",
    },
    {
        'name': 'HD 164922 signal',
        'date': '2016',
        'telescope': 'Green Bank',
        'frequency_mhz': 8425.0,
        'snr': 4.0,
        'ra_h': 18.034,
        'dec_deg': 26.294,
        'notes': 'BL target. Sun-like star with known planet.',
    },
]

# ============================================================
# Known SETI frequency bands of interest
# ============================================================
SETI_FREQUENCIES = {
    'hydrogen_21cm': 1420.405,        # MHz — most searched
    'hydroxyl_18cm': 1665.402,        # MHz — OH line
    'water_hole_low': 1420.405,       # MHz — water hole lower bound
    'water_hole_high': 1665.402,      # MHz — water hole upper bound
    'pi_hydrogen': 4462.336,          # MHz — pi * 1420.405
    'water_22ghz': 22235.08,          # MHz — water maser
    'methanol_6ghz': 6668.52,         # MHz — methanol maser
    'formaldehyde': 4829.66,          # MHz — H2CO
}


def get_wow_signal() -> List[float]:
    """Return Wow! signal parameters as numeric array.

    The 6-character intensity sequence (6EQUJ5) maps to:
    6, 14, 26, 30, 19, 7 in the Big Ear intensity scale.
    """
    wow = WOW_SIGNAL
    values = [
        wow['frequency_mhz'],           # 1420.4556
        wow['bandwidth_khz'],           # 10
        wow['duration_s'],              # 72
        wow['snr'],                     # 30
        wow['ra_h'],                    # 19.425
        wow['dec_deg'],                 # -26.85
    ]
    # The famous intensity sequence
    values.extend([float(v) for v in wow['intensity_sequence']])

    return values


def get_candidate_signals() -> List[float]:
    """Return all known SETI candidate signal parameters as numeric array.

    Aggregates Wow!, BLC1, and historical candidates into a single
    array of numeric values for n=6 pattern scanning.
    """
    values = []

    # Wow! signal
    values.extend(get_wow_signal())

    # BLC1
    blc1 = BLC1_SIGNAL
    values.extend([
        blc1['frequency_mhz'],         # 982.002
        blc1['bandwidth_hz'],          # 2.5
        blc1['duration_s'],            # 18000
        blc1['snr'],                   # 10
        blc1['ra_h'],                  # 14.495
        blc1['dec_deg'],               # -62.679
        blc1['drift_rate_hz_s'],       # 0.038
        blc1['distance_pc'],           # 1.301
    ])

    # Historical candidates
    for sig in HISTORICAL_CANDIDATES:
        values.append(sig.get('frequency_mhz', 0))
        values.append(sig.get('snr', 0))
        values.append(sig.get('ra_h', 0))
        values.append(sig.get('dec_deg', 0))
        if 'drift_rate_hz_s' in sig:
            values.append(sig['drift_rate_hz_s'])
        if 'distance_pc' in sig:
            values.append(sig['distance_pc'])
        if 'distance_mpc' in sig:
            values.append(sig['distance_mpc'])
        if 'dip_depth_pct' in sig:
            values.append(sig['dip_depth_pct'])
        if 'wavelength_nm' in sig:
            values.append(sig['wavelength_nm'])

    # SETI frequency bands
    values.extend(SETI_FREQUENCIES.values())

    print(f"  [seti-db] Loaded {len(values)} signal parameters from {2 + len(HISTORICAL_CANDIDATES)} candidates")
    return values
