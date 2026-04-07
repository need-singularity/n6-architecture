"""CERN Open Data Phase B: R-filter on invariant mass distributions.

Generates realistic synthetic invariant mass spectra (dimuon, diphoton)
based on known physics (Breit-Wigner resonances + continuum backgrounds),
then applies the SEDI R-filter and SignalReceiver to search for n=6 patterns
in the resonance structure itself.
"""
import numpy as np
from itertools import combinations

from ..filter import r_filter
from ..detector import analyze, format_alert
from ..tecs import ALL_TARGETS, mass_ratio_matches
from ..receiver import SignalReceiver


# ─── Known Resonance Parameters ───

DIMUON_RESONANCES = {
    'rho':     {'mass': 0.775,   'width': 0.149,   'yield_frac': 0.05},
    'J_psi':   {'mass': 3.097,   'width': 0.0929,  'yield_frac': 0.08},
    'Upsilon': {'mass': 9.460,   'width': 0.054,   'yield_frac': 0.04},
    'Z':       {'mass': 91.188,  'width': 2.495,   'yield_frac': 0.15},
}

DIPHOTON_RESONANCES = {
    'Higgs': {'mass': 125.25, 'width': 0.004, 'det_resolution': 1.5, 'yield_frac': 0.03},
}

ALL_RESONANCE_MASSES = {
    'rho':     0.775,
    'J_psi':   3.097,
    'Upsilon': 9.460,
    'Z':       91.188,
    'Higgs':   125.25,
}


# ─── Spectrum Generation ───

def breit_wigner(m, m0, gamma):
    """Relativistic Breit-Wigner probability density."""
    return (2 * np.sqrt(2) * m0 * gamma * np.sqrt(m0**2 * (m0**2 + gamma**2))) / (
        np.pi * np.sqrt(m0**2 + np.sqrt(m0**4 + m0**2 * gamma**2))
    ) * 1.0 / ((m**2 - m0**2)**2 + m0**2 * gamma**2)


def generate_dimuon_spectrum(n_events=10000, m_min=0.5, m_max=200.0, n_bins=500, seed=42):
    """Generate realistic dimuon invariant mass spectrum.

    Background: Drell-Yan continuum ~ 1/m^2
    Signals: rho, J/psi, Upsilon, Z as Breit-Wigner peaks + Poisson noise
    """
    rng = np.random.RandomState(seed)
    bin_edges = np.linspace(m_min, m_max, n_bins + 1)
    bin_centers = 0.5 * (bin_edges[:-1] + bin_edges[1:])
    bin_width = bin_edges[1] - bin_edges[0]

    # Drell-Yan background: dN/dm ~ 1/m^2
    bg = 1.0 / bin_centers**2
    bg = bg / bg.sum() * n_events * 0.68  # 68% of events are background

    # Add resonance peaks
    spectrum = bg.copy()
    for name, params in DIMUON_RESONANCES.items():
        m0, gamma = params['mass'], params['width']
        n_sig = int(n_events * params['yield_frac'])
        bw = breit_wigner(bin_centers, m0, gamma)
        bw_norm = bw / (bw.sum() * bin_width) if bw.sum() > 0 else bw
        spectrum += bw_norm * n_sig * bin_width

    # Poisson fluctuations
    spectrum = rng.poisson(np.maximum(spectrum, 0).astype(float))

    return bin_centers, spectrum.astype(float), bin_edges


def generate_diphoton_spectrum(n_events=5000, m_min=50.0, m_max=200.0, n_bins=300, seed=43):
    """Generate realistic diphoton invariant mass spectrum.

    Background: exponentially falling
    Signal: Higgs(125.25) with detector resolution ~1.5 GeV (Gaussian smearing)
    """
    rng = np.random.RandomState(seed)
    bin_edges = np.linspace(m_min, m_max, n_bins + 1)
    bin_centers = 0.5 * (bin_edges[:-1] + bin_edges[1:])
    bin_width = bin_edges[1] - bin_edges[0]

    # Exponential background
    bg = np.exp(-(bin_centers - m_min) / 30.0)
    bg = bg / bg.sum() * n_events * 0.97

    # Higgs signal (Gaussian with detector resolution, not natural width)
    h = DIPHOTON_RESONANCES['Higgs']
    n_sig = int(n_events * h['yield_frac'])
    sig = np.exp(-0.5 * ((bin_centers - h['mass']) / h['det_resolution'])**2)
    sig = sig / (sig.sum() * bin_width) if sig.sum() > 0 else sig
    spectrum = bg + sig * n_sig * bin_width

    # Poisson fluctuations
    spectrum = rng.poisson(np.maximum(spectrum, 0).astype(float))

    return bin_centers, spectrum.astype(float), bin_edges


# ─── Analysis Functions ───

def analyze_spectrum_rfilter(bin_centers, spectrum, name):
    """Apply R-filter to an invariant mass spectrum."""
    result = r_filter(spectrum)
    alerts = analyze(result, f'cern-{name}')
    return result, alerts


def analyze_spectrum_receiver(bin_centers, spectrum, name):
    """Apply SignalReceiver to an invariant mass spectrum."""
    receiver = SignalReceiver(sensitivity=3.0, window=32, ph_enabled=False)

    # Calibrate on pure Poisson noise of similar magnitude
    rng = np.random.RandomState(99)
    noise = rng.poisson(np.mean(spectrum), size=len(spectrum)).astype(float)
    receiver.calibrate(noise)

    result = receiver.receive(spectrum)
    return result


def analyze_resonance_ratios():
    """Compute all pairwise ratios between resonance masses and check TECS-L targets."""
    hits = mass_ratio_matches(ALL_RESONANCE_MASSES, tolerance=0.05)
    return hits


def analyze_consecutive_ratios():
    """Compute consecutive mass ratios: J_psi/rho, Upsilon/J_psi, Z/Upsilon, Higgs/Z."""
    masses_sorted = sorted(ALL_RESONANCE_MASSES.items(), key=lambda x: x[1])
    ratios = []
    for i in range(len(masses_sorted) - 1):
        name_lo, m_lo = masses_sorted[i]
        name_hi, m_hi = masses_sorted[i + 1]
        ratio = m_hi / m_lo
        ratios.append({
            'pair': f'{name_hi}/{name_lo}',
            'ratio': ratio,
            'log_ratio': np.log(ratio),
        })
    return ratios


def analyze_log_spacing():
    """Check if resonance masses are log-spaced (geometric progression)."""
    masses = np.array(sorted(ALL_RESONANCE_MASSES.values()))
    log_masses = np.log(masses)

    # Consecutive log-spacings
    log_diffs = np.diff(log_masses)

    # R-filter on log-masses
    result = r_filter(log_masses)

    # Also check: are the log-spacings uniform?
    mean_spacing = np.mean(log_diffs)
    std_spacing = np.std(log_diffs)
    cv = std_spacing / mean_spacing if mean_spacing > 0 else float('inf')

    return {
        'masses': masses.tolist(),
        'log_masses': log_masses.tolist(),
        'log_diffs': log_diffs.tolist(),
        'mean_log_spacing': float(mean_spacing),
        'std_log_spacing': float(std_spacing),
        'cv': float(cv),
        'r_filter': result,
    }


def analyze_pairwise_ratios_all():
    """Compute ALL pairwise ratios and check each against TECS-L targets."""
    masses = sorted(ALL_RESONANCE_MASSES.values())
    names = [k for k, v in sorted(ALL_RESONANCE_MASSES.items(), key=lambda x: x[1])]

    all_ratios = []
    for i in range(len(masses)):
        for j in range(i + 1, len(masses)):
            ratio = masses[j] / masses[i]
            log_ratio = np.log(ratio)

            # Check against each TECS-L target
            best_match = None
            best_err = float('inf')
            for tname, tval in ALL_TARGETS.items():
                if tval <= 0:
                    continue
                err = abs(ratio - tval) / tval
                if err < best_err:
                    best_err = err
                    best_match = tname

            all_ratios.append({
                'pair': f'{names[j]}/{names[i]}',
                'ratio': ratio,
                'log_ratio': log_ratio,
                'nearest_tecs': best_match,
                'nearest_tecs_val': ALL_TARGETS.get(best_match, 0),
                'nearest_err_pct': best_err * 100,
            })

    return all_ratios


# ─── Report ───

def run_analysis():
    """Run complete Phase B invariant mass analysis and print report."""
    print("=" * 72)
    print("  SEDI Phase B: R-filter on Particle Physics Invariant Mass Spectra")
    print("=" * 72)
    print()

    # ── 1. Generate spectra ──
    print("  [1/6] Generating dimuon invariant mass spectrum...")
    dm_centers, dm_spectrum, _ = generate_dimuon_spectrum()
    print(f"        Range: {dm_centers[0]:.1f} - {dm_centers[-1]:.1f} GeV, "
          f"{len(dm_spectrum)} bins, {int(dm_spectrum.sum())} events")

    print("  [2/6] Generating diphoton invariant mass spectrum...")
    dp_centers, dp_spectrum, _ = generate_diphoton_spectrum()
    print(f"        Range: {dp_centers[0]:.1f} - {dp_centers[-1]:.1f} GeV, "
          f"{len(dp_spectrum)} bins, {int(dp_spectrum.sum())} events")
    print()

    # ── 2. R-filter on spectra ──
    print("-" * 72)
    print("  DIMUON SPECTRUM: R-filter analysis")
    print("-" * 72)
    dm_rfilter, dm_alerts = analyze_spectrum_rfilter(dm_centers, dm_spectrum, 'dimuon')

    print(f"  R-filter stats: n={dm_rfilter['stats']['n']}, "
          f"mean={dm_rfilter['stats']['mean']:.2f}, "
          f"std={dm_rfilter['stats']['std']:.2f}")
    print(f"  Ratio hits: {len(dm_rfilter['ratio_hits'])}")
    for hit in dm_rfilter['ratio_hits']:
        print(f"    {hit['pattern']}: target={hit['target']:.4f}, "
              f"count={hit['count']}, z={hit['z_score']:.2f}")
    print(f"  Spectral peaks: {len(dm_rfilter['spectral_peaks'])}")
    for name, peak in dm_rfilter['spectral_peaks'].items():
        print(f"    {name}: SNR={peak['snr']:.2f}")
    print(f"  Alerts: {len(dm_alerts)}")
    for alert in dm_alerts:
        print(f"    {format_alert(alert)}")
    print()

    # SignalReceiver on dimuon
    print("  DIMUON SPECTRUM: SignalReceiver")
    dm_recv = analyze_spectrum_receiver(dm_centers, dm_spectrum, 'dimuon')
    print(f"  Verdict: {dm_recv['verdict']} "
          f"(signal_strength={dm_recv['signal_strength']:.2f})")
    print(f"  Anomalies detected: {len(dm_recv['anomalies'])}")
    for anom in dm_recv['anomalies'][:5]:
        print(f"    [{anom['type']}] z={anom['z_score']:.2f}: {anom['detail']}")
    print()

    print("-" * 72)
    print("  DIPHOTON SPECTRUM: R-filter analysis")
    print("-" * 72)
    dp_rfilter, dp_alerts = analyze_spectrum_rfilter(dp_centers, dp_spectrum, 'diphoton')

    print(f"  R-filter stats: n={dp_rfilter['stats']['n']}, "
          f"mean={dp_rfilter['stats']['mean']:.2f}, "
          f"std={dp_rfilter['stats']['std']:.2f}")
    print(f"  Ratio hits: {len(dp_rfilter['ratio_hits'])}")
    for hit in dp_rfilter['ratio_hits']:
        print(f"    {hit['pattern']}: target={hit['target']:.4f}, "
              f"count={hit['count']}, z={hit['z_score']:.2f}")
    print(f"  Spectral peaks: {len(dp_rfilter['spectral_peaks'])}")
    for name, peak in dp_rfilter['spectral_peaks'].items():
        print(f"    {name}: SNR={peak['snr']:.2f}")
    print(f"  Alerts: {len(dp_alerts)}")
    for alert in dp_alerts:
        print(f"    {format_alert(alert)}")
    print()

    # SignalReceiver on diphoton
    print("  DIPHOTON SPECTRUM: SignalReceiver")
    dp_recv = analyze_spectrum_receiver(dp_centers, dp_spectrum, 'diphoton')
    print(f"  Verdict: {dp_recv['verdict']} "
          f"(signal_strength={dp_recv['signal_strength']:.2f})")
    print(f"  Anomalies detected: {len(dp_recv['anomalies'])}")
    for anom in dp_recv['anomalies'][:5]:
        print(f"    [{anom['type']}] z={anom['z_score']:.2f}: {anom['detail']}")
    print()

    # ── 3. Resonance spacing analysis ──
    print("=" * 72)
    print("  RESONANCE MASS STRUCTURE ANALYSIS")
    print("=" * 72)
    print()

    # Consecutive ratios
    print("  Consecutive mass ratios:")
    consec = analyze_consecutive_ratios()
    for r in consec:
        print(f"    {r['pair']:>20s} = {r['ratio']:>8.3f}  (ln = {r['log_ratio']:.4f})")
    print()

    # Log-spacing analysis
    print("  Log-spacing pattern:")
    logsp = analyze_log_spacing()
    print(f"    Log-masses: {['%.3f' % x for x in logsp['log_masses']]}")
    print(f"    Log-diffs:  {['%.3f' % x for x in logsp['log_diffs']]}")
    print(f"    Mean log-spacing: {logsp['mean_log_spacing']:.4f}")
    print(f"    Std log-spacing:  {logsp['std_log_spacing']:.4f}")
    print(f"    CV (uniformity):  {logsp['cv']:.4f} (0 = perfect geometric series)")
    print()

    # R-filter on log-masses
    print("  R-filter on log-mass values:")
    log_rf = logsp['r_filter']
    print(f"    Stats: mean={log_rf['stats']['mean']:.4f}, "
          f"std={log_rf['stats']['std']:.4f}, "
          f"focal_check={log_rf['stats']['focal_check']:.6f}")
    if log_rf['ratio_hits']:
        for hit in log_rf['ratio_hits']:
            print(f"    Ratio hit: {hit['pattern']} target={hit['target']:.4f} "
                  f"count={hit['count']} z={hit['z_score']:.2f}")
    else:
        print("    No ratio hits (too few data points)")
    print()

    # All pairwise ratios vs TECS-L targets
    print("  All pairwise mass ratios vs TECS-L targets:")
    pw_ratios = analyze_pairwise_ratios_all()
    for r in pw_ratios:
        match_flag = " <-- MATCH" if r['nearest_err_pct'] < 5 else ""
        print(f"    {r['pair']:>20s} = {r['ratio']:>10.4f}  "
              f"nearest: {r['nearest_tecs']:<25s} ({r['nearest_tecs_val']:.4f}) "
              f"err={r['nearest_err_pct']:.2f}%{match_flag}")
    print()

    # TECS-L mass_ratio_matches
    print("  TECS-L mass_ratio_matches (tolerance=5%):")
    tecs_hits = analyze_resonance_ratios()
    if tecs_hits:
        for h in tecs_hits:
            print(f"    {h['p1']}/{h['p2']}: ratio={h['ratio']:.4f} "
                  f"-> {h['target_name']} ({h['target_val']:.4f}) "
                  f"err={h['error_pct']:.2f}%")
    else:
        print("    No matches within 5% tolerance")
    print()

    # ── 4. Key ratio checks ──
    print("=" * 72)
    print("  KEY RATIO CHECKS")
    print("=" * 72)
    print()

    masses = ALL_RESONANCE_MASSES
    jpsi_rho = masses['J_psi'] / masses['rho']
    ups_jpsi = masses['Upsilon'] / masses['J_psi']
    z_ups = masses['Z'] / masses['Upsilon']
    h_z = masses['Higgs'] / masses['Z']

    print(f"  J/psi / rho     = {jpsi_rho:.4f}")
    print(f"  Upsilon / J/psi = {ups_jpsi:.4f}")
    print(f"  Z / Upsilon     = {z_ups:.4f}")
    print(f"  Higgs / Z       = {h_z:.4f}")
    print()

    # Check these against named TECS-L targets
    key_ratios = {
        'J_psi/rho': jpsi_rho,
        'Upsilon/J_psi': ups_jpsi,
        'Z/Upsilon': z_ups,
        'Higgs/Z': h_z,
    }
    print("  Checking key ratios against ALL_TARGETS:")
    for rname, rval in key_ratios.items():
        best_name, best_val, best_err = None, None, float('inf')
        for tname, tval in ALL_TARGETS.items():
            if tval <= 0:
                continue
            err = abs(rval - tval) / tval
            if err < best_err:
                best_err = err
                best_name = tname
                best_val = tval
        status = "MATCH" if best_err < 0.05 else "near" if best_err < 0.15 else "no match"
        print(f"    {rname:>20s} = {rval:>8.4f}  "
              f"closest: {best_name} = {best_val:.4f}  err={best_err*100:.2f}%  [{status}]")
    print()

    # ── 5. Koide on resonance mass triples ──
    print("  Koide formula on resonance mass triples:")
    from ..tecs import koide_check
    mass_list = sorted(ALL_RESONANCE_MASSES.items(), key=lambda x: x[1])
    for combo in combinations(range(len(mass_list)), 3):
        names_c = [mass_list[i][0] for i in combo]
        masses_c = [mass_list[i][1] for i in combo]
        ratio, error = koide_check(masses_c)
        flag = " <-- near 2/3!" if error < 5 else ""
        print(f"    ({', '.join(names_c):>30s}): Q = {ratio:.6f}  "
              f"err from 2/3 = {error:.2f}%{flag}")
    print()

    # ── Summary ──
    print("=" * 72)
    print("  PHASE B SUMMARY")
    print("=" * 72)
    print()
    print(f"  Dimuon spectrum:  {len(dm_alerts)} R-filter alerts, "
          f"receiver verdict = {dm_recv['verdict']}")
    print(f"  Diphoton spectrum: {len(dp_alerts)} R-filter alerts, "
          f"receiver verdict = {dp_recv['verdict']}")
    print(f"  TECS-L ratio matches in resonance masses: {len(tecs_hits)}")
    print(f"  Consecutive ratios span: {jpsi_rho:.2f} to {z_ups:.2f}")
    print(f"  Log-spacing CV: {logsp['cv']:.4f} "
          f"({'near-geometric' if logsp['cv'] < 0.5 else 'non-geometric'})")
    print()

    n_koide_near = sum(1 for combo in combinations(range(len(mass_list)), 3)
                       if koide_check([mass_list[i][1] for i in combo])[1] < 5)
    print(f"  Koide near-matches (< 5% from 2/3): {n_koide_near} / "
          f"{len(list(combinations(range(len(mass_list)), 3)))} triples")
    print()
    print("=" * 72)
    print("  END PHASE B")
    print("=" * 72)


if __name__ == '__main__':
    run_analysis()
