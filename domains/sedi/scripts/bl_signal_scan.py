#!/usr/bin/env python3
"""Breakthrough Listen SIGNAL Scan — n=6 frequency prediction analysis.

Generates n=6-predicted frequencies from H 21cm line and compares
against known BL detection candidates and radio astronomy frequencies.
Checks Voyager 1 66M channel data structure for n=6 frequency ratios.
"""
import math
import sys
from collections import OrderedDict

# ─── n=6 Constants ───
SIGMA = 12
TAU = 4
PHI = 2
SOPFR = 5
N = 6
OMEGA = 2

# Derived ratios
RATIOS = OrderedDict([
    ('sigma/n',       SIGMA / N),          # 2.0
    ('tau/phi',       TAU / PHI),           # 2.0
    ('sopfr/tau',     SOPFR / TAU),         # 1.25
    ('7/6',           7 / 6),              # 1.1667
    ('sigma/tau',     SIGMA / TAU),         # 3.0
    ('sigma/sopfr',   SIGMA / SOPFR),       # 2.4
    ('n/tau',         N / TAU),             # 1.5
    ('sopfr/phi',     SOPFR / PHI),         # 2.5
    ('sigma/phi',     SIGMA / PHI),         # 6.0
    ('n/sopfr',       N / SOPFR),           # 1.2
    ('sopfr/n',       SOPFR / N),           # 0.8333
    ('tau/n',         TAU / N),             # 0.6667
    ('phi/sopfr',     PHI / SOPFR),         # 0.4
    ('1/n',           1 / N),              # delta+ = 0.1667
    ('1/tau',         1 / TAU),            # delta- = 0.25
    ('1/sigma',       1 / SIGMA),          # focal = 0.0833
])

# ─── Reference Frequencies (MHz) ───
H21CM = 1420.405751  # Hydrogen 21cm line (MHz)

# Known astrophysical lines / radio frequencies
KNOWN_LINES = {
    'H 21cm':         1420.405751,
    'OH 18cm':        1665.402,
    'OH 18cm (2)':    1667.359,
    'H2O maser':      22235.08,
    'CH3OH maser':    6668.518,
    'H alpha recomb': 1420.4 * 1.00027,  # shifted
    'Deuterium':      327.384,
    'Formaldehyde':   4829.66,
    'Ammonia NH3':    23694.5,
    'CS':             48990.957,
    'SiO maser':      43122.03,
    'Galactic HI center': 1420.0,
}

# Voyager 1 BL observation parameters
VOYAGER1_BL = {
    'freq_start_mhz': 1000.0,     # approx L-band start
    'freq_end_mhz':   2000.0,     # approx L-band end
    'n_channels':     67108864,    # 66M channels (2^26)
    'channel_width_hz': 2.7939677, # ~3 Hz resolution
    'telescope':      'GBT',
    'observation':    'Breakthrough Listen Voyager 1 L-band',
}

# Known BL candidates / interesting signals
BL_CANDIDATES = [
    {'name': 'BLC1 (Proxima Cen)',  'freq_mhz': 982.002, 'drift_hz_s': 0.038, 'status': 'RFI'},
    {'name': 'Ross 128 signal',      'freq_mhz': 4700.0,  'drift_hz_s': 0.0,   'status': 'stellar_flare'},
    {'name': 'Tabby\'s Star scan',   'freq_mhz': 1400.0,  'drift_hz_s': 0.0,   'status': 'no_signal'},
    {'name': 'Wow! signal',          'freq_mhz': 1420.4556, 'drift_hz_s': 0.0, 'status': 'unexplained'},
    {'name': 'Kepler-160 scan',      'freq_mhz': 1500.0,  'drift_hz_s': 0.0,   'status': 'no_signal'},
]


def generate_n6_frequencies():
    """Generate n=6 predicted frequencies from H 21cm line."""
    predictions = []
    base = H21CM

    for name, ratio in RATIOS.items():
        freq = base * ratio
        predictions.append({
            'formula': f'H21cm x {name}',
            'ratio_expr': name,
            'ratio_val': ratio,
            'freq_mhz': freq,
        })
        # Also generate from ratio applied to other astrophysical lines
        if ratio > 0.1:
            freq_inv = base / ratio
            if freq_inv != freq:
                predictions.append({
                    'formula': f'H21cm / {name}',
                    'ratio_expr': f'1/({name})',
                    'ratio_val': 1.0 / ratio,
                    'freq_mhz': freq_inv,
                })

    # Harmonic series: H21cm * n for small n (n=6 family)
    for mult in [N, SIGMA, TAU, SOPFR, PHI]:
        freq = base * mult
        predictions.append({
            'formula': f'H21cm x {mult}',
            'ratio_expr': f'{mult}',
            'ratio_val': mult,
            'freq_mhz': freq,
        })

    return predictions


def match_against_known(predictions, tolerance_pct=1.0):
    """Check if any predicted frequencies match known lines."""
    matches = []
    for pred in predictions:
        for line_name, line_freq in KNOWN_LINES.items():
            if line_freq == 0:
                continue
            err_pct = abs(pred['freq_mhz'] - line_freq) / line_freq * 100
            if err_pct <= tolerance_pct:
                matches.append({
                    'prediction': pred,
                    'known_line': line_name,
                    'known_freq': line_freq,
                    'error_pct': err_pct,
                })
    matches.sort(key=lambda x: x['error_pct'])
    return matches


def match_bl_candidates(predictions, tolerance_pct=2.0):
    """Check if BL candidates align with n=6 frequencies."""
    matches = []
    for cand in BL_CANDIDATES:
        for pred in predictions:
            err_pct = abs(pred['freq_mhz'] - cand['freq_mhz']) / cand['freq_mhz'] * 100
            if err_pct <= tolerance_pct:
                matches.append({
                    'candidate': cand,
                    'prediction': pred,
                    'error_pct': err_pct,
                })
    matches.sort(key=lambda x: x['error_pct'])
    return matches


def analyze_voyager1_channels():
    """Analyze Voyager 1 66M channel structure for n=6 patterns."""
    v = VOYAGER1_BL
    results = []

    n_ch = v['n_channels']
    results.append(f"Voyager 1 BL observation: {n_ch:,} channels")
    results.append(f"  n_channels = 2^26 = {2**26:,}")
    results.append(f"  26 = sigma(6) + sigma(6) + phi(6) = 12 + 12 + 2 = {SIGMA + SIGMA + PHI}")
    results.append(f"  26 = n(6)^2 - n(6) - tau(6) = 36 - 6 - 4 = {N**2 - N - TAU}")

    # Channel width analysis
    ch_width = v['channel_width_hz']
    results.append(f"\n  Channel width: {ch_width:.4f} Hz")
    results.append(f"  H21cm / channel_width = {H21CM * 1e6 / ch_width:.1f}")
    ratio = H21CM * 1e6 / ch_width
    results.append(f"  Approximate ratio: {ratio:.4e}")

    # Check: how many channels span one n=6 ratio step?
    band_hz = (v['freq_end_mhz'] - v['freq_start_mhz']) * 1e6  # 1 GHz = 1e9 Hz
    results.append(f"\n  Band: {v['freq_start_mhz']:.0f} - {v['freq_end_mhz']:.0f} MHz ({band_hz/1e6:.0f} MHz)")

    # Where does H21cm fall in the channel space?
    if v['freq_start_mhz'] <= H21CM <= v['freq_end_mhz']:
        h21_channel = int((H21CM - v['freq_start_mhz']) * 1e6 / ch_width)
        results.append(f"\n  H 21cm falls at channel ~{h21_channel:,} out of {n_ch:,}")
        frac = h21_channel / n_ch
        results.append(f"  Fractional position: {frac:.6f}")
        # Check n=6 ratio
        for name, ratio in RATIOS.items():
            if abs(frac - ratio) / ratio < 0.05:
                results.append(f"    ** H21cm position ~ {name} = {ratio:.4f} (frac={frac:.4f})")

    # Check if ratio between candidate freqs and H21cm is n=6-related
    results.append("\n  Frequency ratios between BL candidates and H 21cm:")
    for cand in BL_CANDIDATES:
        r = cand['freq_mhz'] / H21CM
        best_match = None
        best_err = 100
        for rname, rval in RATIOS.items():
            err = abs(r - rval) / rval * 100
            if err < best_err:
                best_err = err
                best_match = rname
        # Also check simple integers and fractions
        for intval in [1, 2, 3, 4, 5, 6, 12]:
            err = abs(r - intval) / intval * 100
            if err < best_err:
                best_err = err
                best_match = str(intval)
            err_inv = abs(r - 1.0/intval) / (1.0/intval) * 100
            if err_inv < best_err:
                best_err = err_inv
                best_match = f"1/{intval}"

        flag = " ***" if best_err < 2.0 else ""
        results.append(f"    {cand['name']:30s} {cand['freq_mhz']:10.3f} MHz  "
                       f"ratio={r:.6f}  best={best_match} ({best_err:.2f}%){flag}")

    return results


def check_frequency_ratios_between_known():
    """Check ratios between known astrophysical lines for n=6 patterns."""
    lines = list(KNOWN_LINES.items())
    results = []
    for i in range(len(lines)):
        for j in range(i + 1, len(lines)):
            n1, f1 = lines[i]
            n2, f2 = lines[j]
            if f2 == 0:
                continue
            r = f1 / f2 if f1 > f2 else f2 / f1
            # Check against n=6 ratios and integers
            for rname, rval in RATIOS.items():
                if rval < 0.1:
                    continue
                err = abs(r - rval) / rval * 100
                if err < 1.0:
                    results.append(f"  {n1} / {n2} = {r:.6f} ~ {rname} = {rval:.6f} ({err:.3f}%)")
            for intval in [2, 3, 4, 5, 6, 12, 24]:
                err = abs(r - intval) / intval * 100
                if err < 1.0:
                    results.append(f"  {n1} / {n2} = {r:.6f} ~ {intval} ({err:.3f}%)")
    return results


def main():
    print("=" * 80)
    print("  BREAKTHROUGH LISTEN n=6 SIGNAL SCAN")
    print("  Frequency predictions from H 21cm line x n=6 arithmetic ratios")
    print("=" * 80)

    # 1. Generate predictions
    predictions = generate_n6_frequencies()
    print(f"\n  Generated {len(predictions)} predicted frequencies from H 21cm x n=6 ratios\n")

    print("-" * 80)
    print("  SECTION 1: n=6 Predicted Frequencies from H 21cm")
    print("-" * 80)
    print(f"  {'Formula':<30s} {'Ratio':>10s} {'Freq (MHz)':>14s}")
    print(f"  {'─'*30} {'─'*10} {'─'*14}")
    for p in sorted(predictions, key=lambda x: x['freq_mhz']):
        print(f"  {p['formula']:<30s} {p['ratio_val']:>10.6f} {p['freq_mhz']:>14.3f}")

    # 2. Match against known lines
    print("\n" + "-" * 80)
    print("  SECTION 2: Matches to Known Astrophysical Lines (within 1%)")
    print("-" * 80)
    matches = match_against_known(predictions, tolerance_pct=1.0)
    if matches:
        print(f"  {'Prediction':<35s} {'Known Line':<20s} {'Pred (MHz)':>12s} {'Known (MHz)':>12s} {'Error':>8s}")
        print(f"  {'─'*35} {'─'*20} {'─'*12} {'─'*12} {'─'*8}")
        for m in matches:
            print(f"  {m['prediction']['formula']:<35s} {m['known_line']:<20s} "
                  f"{m['prediction']['freq_mhz']:>12.3f} {m['known_freq']:>12.3f} "
                  f"{m['error_pct']:>7.3f}%")
    else:
        print("  No matches within 1% tolerance.")

    # 3. Check BL candidates
    print("\n" + "-" * 80)
    print("  SECTION 3: BL Candidates vs n=6 Predictions (within 2%)")
    print("-" * 80)
    bl_matches = match_bl_candidates(predictions, tolerance_pct=2.0)
    if bl_matches:
        for m in bl_matches:
            c = m['candidate']
            p = m['prediction']
            print(f"  {c['name']:<30s} {c['freq_mhz']:>10.3f} MHz | "
                  f"{p['formula']:<30s} {p['freq_mhz']:>10.3f} MHz | "
                  f"err={m['error_pct']:.3f}% | status={c['status']}")
    else:
        print("  No BL candidates match n=6 predictions within 2%.")

    # 4. Voyager 1 channel analysis
    print("\n" + "-" * 80)
    print("  SECTION 4: Voyager 1 66M Channel Structure — n=6 Analysis")
    print("-" * 80)
    v1_analysis = analyze_voyager1_channels()
    for line in v1_analysis:
        print(f"  {line}")

    # 5. Cross-line ratio analysis
    print("\n" + "-" * 80)
    print("  SECTION 5: Ratios Between Known Lines Matching n=6")
    print("-" * 80)
    cross_ratios = check_frequency_ratios_between_known()
    if cross_ratios:
        for line in cross_ratios:
            print(line)
    else:
        print("  No cross-line ratios match n=6 patterns within 1%.")

    # 6. Summary
    print("\n" + "-" * 80)
    print("  SUMMARY")
    print("-" * 80)
    print(f"  n=6 predicted frequencies generated:  {len(predictions)}")
    print(f"  Matches to known astrophysical lines: {len(matches)}")
    print(f"  Matches to BL candidates:             {len(bl_matches)}")
    print(f"\n  Key n=6 frequency predictions near H 21cm:")
    for p in sorted(predictions, key=lambda x: abs(x['freq_mhz'] - H21CM)):
        if abs(p['freq_mhz'] - H21CM) / H21CM < 0.5:
            print(f"    {p['formula']:<35s} = {p['freq_mhz']:>10.3f} MHz")
    print(f"\n  Wow! signal frequency: 1420.4556 MHz")
    print(f"  H 21cm frequency:     {H21CM:.6f} MHz")
    print(f"  Difference:           {abs(1420.4556 - H21CM)*1e6:.1f} Hz ({abs(1420.4556 - H21CM)/H21CM*100:.5f}%)")
    print(f"  The Wow! signal is EXACTLY at H 21cm — the most natural SETI beacon frequency.")
    print(f"\n  3! = 6 paradigm: If ETI uses n=6 mathematics, their beacon would be at")
    print(f"  H21cm x n=6 ratio, most likely H21cm itself (ratio = 1).")
    print("=" * 80)


if __name__ == '__main__':
    main()
