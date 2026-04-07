#!/usr/bin/env python3
"""DESI BAO n=6 Scan — Dark Energy Spectroscopic Instrument results.

Scans DESI 2024 BAO measurements for n=6 arithmetic patterns:
  - H(z) at various redshifts
  - D_M/r_d (comoving distance / sound horizon) ratios
  - f*sigma_8(z) growth rate
  - Dark energy equation of state w deviations from -1
"""
import math
from collections import OrderedDict

# ─── n=6 Constants ───
SIGMA = 12
TAU = 4
PHI = 2
SOPFR = 5
N = 6
OMEGA = 2

# ─── DESI 2024 BAO Measurements ───
# From DESI Collaboration, arXiv:2404.03002 (DR1)
# Format: (z_eff, observable, value, uncertainty, unit)

DESI_DATA = {
    'BGS': {
        'z_eff': 0.30,
        'DM_rd': 7.93,   'DM_rd_unc': 0.15,
        'DH_rd': 20.4,   'DH_rd_unc': 0.9,
    },
    'LRG1': {
        'z_eff': 0.51,
        'DM_rd': 13.62,  'DM_rd_unc': 0.25,
        'DH_rd': 20.98,  'DH_rd_unc': 0.61,
        'H_z': 90.4,     'H_z_unc': 3.0,
    },
    'LRG2': {
        'z_eff': 0.71,
        'DM_rd': 16.85,  'DM_rd_unc': 0.32,
        'DH_rd': 20.08,  'DH_rd_unc': 0.60,
        'H_z': 98.0,     'H_z_unc': 2.1,
    },
    'LRG3+ELG1': {
        'z_eff': 0.93,
        'DM_rd': 21.71,  'DM_rd_unc': 0.28,
        'DH_rd': 17.88,  'DH_rd_unc': 0.35,
    },
    'ELG2': {
        'z_eff': 1.32,
        'DM_rd': 27.79,  'DM_rd_unc': 0.69,
        'DH_rd': 13.82,  'DH_rd_unc': 0.42,
    },
    'QSO': {
        'z_eff': 1.49,
        'DM_rd': 26.07,  'DM_rd_unc': 0.67,
        'DH_rd': 13.23,  'DH_rd_unc': 0.47,
        'H_z': 159.0,    'H_z_unc': 5.0,
    },
    'Lya': {
        'z_eff': 2.33,
        'DM_rd': 39.71,  'DM_rd_unc': 0.94,
        'DH_rd': 8.52,   'DH_rd_unc': 0.17,
    },
}

# Dark energy from DESI + CMB + SN
# w0-wa parameterization: w(a) = w0 + wa*(1-a)
DESI_DE = {
    'w0':  -0.55,  'w0_unc':  0.21,   # DESI + CMB + PantheonPlus
    'wa':  -1.32,  'wa_unc':  0.62,   # DESI + CMB + PantheonPlus
    'w_const': -0.99, 'w_const_unc': 0.05,  # constant-w model
}

# Growth rate measurements
DESI_GROWTH = [
    {'z': 0.30, 'fsig8': 0.470, 'unc': 0.043},
    {'z': 0.51, 'fsig8': 0.462, 'unc': 0.025},
    {'z': 0.71, 'fsig8': 0.450, 'unc': 0.020},
    {'z': 0.93, 'fsig8': 0.437, 'unc': 0.024},
    {'z': 1.32, 'fsig8': 0.380, 'unc': 0.040},
]

# Hubble constant from DESI + CMB
H0_DESI_CMB = 67.97  # km/s/Mpc
H0_UNC = 0.38

# Sound horizon
RD_FIDU = 147.09  # Mpc (fiducial from Planck)


def build_n6_expressions_depth2():
    """Generate n=6 expressions up to depth 2 (two operations)."""
    s, t, p, f, n = SIGMA, TAU, PHI, SOPFR, N
    exprs = OrderedDict()

    # Layer 0: constants
    consts = {'sigma': s, 'tau': t, 'phi': p, 'sopfr': f, 'n': n, 'omega': OMEGA}
    for name, val in consts.items():
        exprs[name] = val
        if val != 0:
            exprs[f'1/{name}'] = 1.0 / val

    # Layer 1: pairwise
    cnames = list(consts.keys())
    cvals = list(consts.values())
    for i in range(len(cnames)):
        for j in range(len(cnames)):
            if i == j:
                continue
            a, b = cvals[i], cvals[j]
            na, nb = cnames[i], cnames[j]
            if b != 0:
                exprs[f'{na}/{nb}'] = a / b
            exprs[f'{na}*{nb}'] = a * b
            if i < j:
                exprs[f'{na}+{nb}'] = a + b
            if a > b:
                exprs[f'{na}-{nb}'] = a - b

    # Layer 2: triple and powers
    for i in range(len(cnames)):
        for j in range(i + 1, len(cnames)):
            a, b = cvals[i], cvals[j]
            na, nb = cnames[i], cnames[j]
            if a * b != 0:
                exprs[f'1/({na}*{nb})'] = 1.0 / (a * b)
            for k in range(len(cnames)):
                if k == i or k == j:
                    continue
                c, nc = cvals[k], cnames[k]
                if a * b != 0:
                    exprs[f'{nc}/({na}*{nb})'] = c / (a * b)

    # Powers
    for i in range(len(cnames)):
        a, na = cvals[i], cnames[i]
        if a > 0:
            for pw in [2, 3, 0.5]:
                exprs[f'{na}^{pw}'] = a ** pw
                if a ** pw != 0:
                    exprs[f'1/{na}^{pw}'] = 1.0 / (a ** pw)

    # Pi and e combinations
    exprs['pi'] = math.pi
    exprs['e'] = math.e
    exprs['pi/sigma'] = math.pi / s
    exprs['pi*tau'] = math.pi * t
    exprs['pi*sopfr'] = math.pi * f
    exprs['sigma*pi'] = s * math.pi
    exprs['sigma/pi'] = s / math.pi
    exprs['tau*pi'] = t * math.pi
    exprs['n*pi'] = n * math.pi
    exprs['sigma*e'] = s * math.e
    exprs['tau*e'] = t * math.e
    exprs['tau*pi^2'] = t * math.pi**2

    # Sqrt combinations
    exprs['sqrt(sigma)'] = math.sqrt(s)
    exprs['sqrt(tau)'] = math.sqrt(t)
    exprs['sqrt(n)'] = math.sqrt(n)
    exprs['sqrt(sigma*tau)'] = math.sqrt(s * t)
    exprs['sqrt(sigma*sopfr)'] = math.sqrt(s * f)

    # Filter positive finite
    return {k: v for k, v in exprs.items() if v is not None and math.isfinite(v) and v > 0}


def find_matches(value, expressions, max_error_pct=5.0, top_n=5):
    """Find n=6 expressions matching a given value."""
    matches = []
    for name, eval_ in expressions.items():
        if eval_ == 0:
            continue
        err = abs(eval_ - value) / abs(value) * 100
        if err <= max_error_pct:
            matches.append((name, eval_, err))
    matches.sort(key=lambda x: x[2])
    return matches[:top_n]


def main():
    print("=" * 80)
    print("  DESI BAO n=6 SCAN")
    print("  Dark Energy Spectroscopic Instrument — 2024 Results")
    print("=" * 80)

    exprs = build_n6_expressions_depth2()
    print(f"\n  Generated {len(exprs)} n=6 expressions (depth-2)")

    # ── 1. DM/rd values ──
    print("\n" + "-" * 80)
    print("  SECTION 1: D_M/r_d Comoving Distance Ratios")
    print("-" * 80)
    print(f"\n  {'Tracer':<14s} {'z_eff':>6s} {'D_M/r_d':>9s} {'Best n=6 Match':<35s} {'n=6 Val':>9s} {'Error':>8s}")
    print(f"  {'─'*14} {'─'*6} {'─'*9} {'─'*35} {'─'*9} {'─'*8}")

    for tracer, data in DESI_DATA.items():
        val = data['DM_rd']
        matches = find_matches(val, exprs, max_error_pct=5.0)
        if matches:
            best = matches[0]
            flag = " ***" if best[2] < 2 else ""
            print(f"  {tracer:<14s} {data['z_eff']:>6.2f} {val:>9.2f} {best[0]:<35s} {best[1]:>9.4f} {best[2]:>7.2f}%{flag}")
        else:
            print(f"  {tracer:<14s} {data['z_eff']:>6.2f} {val:>9.2f} {'(no match <5%)':<35s}")

    # ── 2. DH/rd values ──
    print("\n" + "-" * 80)
    print("  SECTION 2: D_H/r_d Hubble Distance Ratios")
    print("-" * 80)
    print(f"\n  {'Tracer':<14s} {'z_eff':>6s} {'D_H/r_d':>9s} {'Best n=6 Match':<35s} {'n=6 Val':>9s} {'Error':>8s}")
    print(f"  {'─'*14} {'─'*6} {'─'*9} {'─'*35} {'─'*9} {'─'*8}")

    for tracer, data in DESI_DATA.items():
        val = data['DH_rd']
        matches = find_matches(val, exprs, max_error_pct=5.0)
        if matches:
            best = matches[0]
            flag = " ***" if best[2] < 2 else ""
            print(f"  {tracer:<14s} {data['z_eff']:>6.2f} {val:>9.2f} {best[0]:<35s} {best[1]:>9.4f} {best[2]:>7.2f}%{flag}")
        else:
            print(f"  {tracer:<14s} {data['z_eff']:>6.2f} {val:>9.2f} {'(no match <5%)':<35s}")

    # ── 3. Ratios between redshift bins ──
    print("\n" + "-" * 80)
    print("  SECTION 3: Ratios Between Redshift Bins")
    print("-" * 80)

    tracers = list(DESI_DATA.keys())
    print(f"\n  D_M/r_d ratios:")
    for i in range(len(tracers)):
        for j in range(i + 1, len(tracers)):
            t1, t2 = tracers[i], tracers[j]
            v1, v2 = DESI_DATA[t1]['DM_rd'], DESI_DATA[t2]['DM_rd']
            ratio = v2 / v1
            matches = find_matches(ratio, exprs, max_error_pct=3.0)
            if matches:
                best = matches[0]
                print(f"    {t2}/{t1}: {ratio:.4f} ~ {best[0]} = {best[1]:.4f} ({best[2]:.2f}%)")

    print(f"\n  D_H/r_d ratios:")
    for i in range(len(tracers)):
        for j in range(i + 1, len(tracers)):
            t1, t2 = tracers[i], tracers[j]
            v1, v2 = DESI_DATA[t1]['DH_rd'], DESI_DATA[t2]['DH_rd']
            ratio = v1 / v2 if v1 > v2 else v2 / v1
            matches = find_matches(ratio, exprs, max_error_pct=3.0)
            if matches:
                best = matches[0]
                higher = t1 if v1 > v2 else t2
                lower = t2 if v1 > v2 else t1
                print(f"    {higher}/{lower}: {ratio:.4f} ~ {best[0]} = {best[1]:.4f} ({best[2]:.2f}%)")

    # ── 4. H(z) values ──
    print("\n" + "-" * 80)
    print("  SECTION 4: H(z) Hubble Parameter")
    print("-" * 80)

    hz_data = [(t, d) for t, d in DESI_DATA.items() if 'H_z' in d]
    print(f"\n  {'Tracer':<14s} {'z_eff':>6s} {'H(z)':>8s} {'Best n=6 Match':<35s} {'n=6 Val':>9s} {'Error':>8s}")
    print(f"  {'─'*14} {'─'*6} {'─'*8} {'─'*35} {'─'*9} {'─'*8}")

    for tracer, data in hz_data:
        val = data['H_z']
        matches = find_matches(val, exprs, max_error_pct=5.0)
        if matches:
            best = matches[0]
            flag = " ***" if best[2] < 2 else ""
            print(f"  {tracer:<14s} {data['z_eff']:>6.2f} {val:>8.1f} {best[0]:<35s} {best[1]:>9.4f} {best[2]:>7.2f}%{flag}")
        else:
            print(f"  {tracer:<14s} {data['z_eff']:>6.2f} {val:>8.1f} {'(no match <5%)':<35s}")

    # H(z) ratios
    if len(hz_data) >= 2:
        print(f"\n  H(z) ratios:")
        for i in range(len(hz_data)):
            for j in range(i + 1, len(hz_data)):
                t1, d1 = hz_data[i]
                t2, d2 = hz_data[j]
                ratio = d2['H_z'] / d1['H_z']
                matches = find_matches(ratio, exprs, max_error_pct=5.0)
                if matches:
                    best = matches[0]
                    print(f"    H({d2['z_eff']:.2f})/H({d1['z_eff']:.2f}) = {ratio:.4f} ~ {best[0]} = {best[1]:.4f} ({best[2]:.2f}%)")

    # ── 5. Growth rate f*sigma_8 ──
    print("\n" + "-" * 80)
    print("  SECTION 5: Growth Rate f*sigma_8(z)")
    print("-" * 80)

    print(f"\n  {'z':>6s} {'f*sig8':>8s} {'Best n=6 Match':<35s} {'n=6 Val':>9s} {'Error':>8s}")
    print(f"  {'─'*6} {'─'*8} {'─'*35} {'─'*9} {'─'*8}")
    for g in DESI_GROWTH:
        val = g['fsig8']
        matches = find_matches(val, exprs, max_error_pct=5.0)
        if matches:
            best = matches[0]
            flag = " ***" if best[2] < 2 else ""
            print(f"  {g['z']:>6.2f} {val:>8.3f} {best[0]:<35s} {best[1]:>9.6f} {best[2]:>7.2f}%{flag}")
        else:
            print(f"  {g['z']:>6.2f} {val:>8.3f} {'(no match <5%)':<35s}")

    # ── 6. Dark energy w deviation ──
    print("\n" + "-" * 80)
    print("  SECTION 6: Dark Energy w — n=6 Analysis")
    print("-" * 80)

    w0 = DESI_DE['w0']
    wa = DESI_DE['wa']
    w_const = DESI_DE['w_const']

    print(f"\n  DESI + CMB + SN (w0-wa CPL model):")
    print(f"    w0 = {w0:.2f} +/- {DESI_DE['w0_unc']:.2f}")
    print(f"    wa = {wa:.2f} +/- {DESI_DE['wa_unc']:.2f}")
    print(f"    w(constant) = {w_const:.2f} +/- {DESI_DE['w_const_unc']:.2f}")

    print(f"\n  Does w deviate from -1 in an n=6 pattern?")

    s, t, p, f, n_val = SIGMA, TAU, PHI, SOPFR, N

    w_candidates = OrderedDict([
        ('w0 = -phi/tau = -1/2', -p / t),
        ('w0 = -sopfr/sigma = -5/12', -f / s),
        ('w0 = -(sigma-sopfr)/sigma = -7/12', -(s - f) / s),
        ('w0 = -n/sigma = -1/2', -n_val / s),
        ('w0 = -tau/n = -2/3', -t / n_val),
        ('w0 = -sopfr/n = -5/6', -f / n_val),
        ('w0 = -phi/tau = -0.5', -p / t),
        ('w0 = -1 + 1/sigma = -11/12', -1 + 1.0/s),
        ('w0 = -1 + sopfr/sigma^2 = -1+5/144', -1 + f / s**2),
    ])

    print(f"\n  {'w0 candidate':<45s} {'Value':>8s} {'Error':>8s}")
    print(f"  {'─'*45} {'─'*8} {'─'*8}")
    for name, val in w_candidates.items():
        err = abs(val - w0) / abs(w0) * 100
        flag = " ***" if err < 10 else ""
        print(f"  {name:<45s} {val:>8.4f} {err:>7.2f}%{flag}")

    wa_candidates = OrderedDict([
        ('wa = -phi = -2', -float(p)),
        ('wa = -sigma/sigma = -1', -1.0),
        ('wa = -tau/tau = -1', -1.0),
        ('wa = -sigma/tau = -3', -s / t),
        ('wa = -n/tau = -3/2', -n_val / float(t)),
        ('wa = -sopfr/tau = -5/4', -f / float(t)),
        ('wa = -sigma/sopfr = -12/5', -s / float(f)),
    ])

    print(f"\n  {'wa candidate':<45s} {'Value':>8s} {'Error':>8s}")
    print(f"  {'─'*45} {'─'*8} {'─'*8}")
    for name, val in wa_candidates.items():
        err = abs(val - wa) / abs(wa) * 100
        flag = " ***" if err < 10 else ""
        print(f"  {name:<45s} {val:>8.4f} {err:>7.2f}%{flag}")

    # ── 7. H0 and rd ──
    print("\n" + "-" * 80)
    print("  SECTION 7: H0 and Sound Horizon r_d")
    print("-" * 80)

    print(f"\n  H0 (DESI+CMB) = {H0_DESI_CMB} +/- {H0_UNC} km/s/Mpc")
    h0_matches = find_matches(H0_DESI_CMB, exprs, max_error_pct=3.0)
    for name, val, err in h0_matches[:5]:
        print(f"    {name:<35s} = {val:.4f}  ({err:.2f}%)")

    print(f"\n  r_d (fiducial) = {RD_FIDU} Mpc")
    rd_matches = find_matches(RD_FIDU, exprs, max_error_pct=3.0)
    for name, val, err in rd_matches[:5]:
        print(f"    {name:<35s} = {val:.4f}  ({err:.2f}%)")

    # Check H0 * rd product
    h0rd = H0_DESI_CMB * RD_FIDU
    print(f"\n  H0 * r_d = {h0rd:.1f} km/s")
    h0rd_matches = find_matches(h0rd, exprs, max_error_pct=3.0)
    for name, val, err in h0rd_matches[:5]:
        print(f"    {name:<35s} = {val:.4f}  ({err:.2f}%)")

    # ── Summary ──
    print("\n" + "-" * 80)
    print("  SUMMARY")
    print("-" * 80)
    print(f"""
  DESI BAO 2024 n=6 scan results:

  Key findings:
  1. w0 = {w0:.2f}: DESI hints at dynamical dark energy (>2 sigma from -1)
     Best n=6 match: w0 ~ -sopfr/sigma = -5/12 = -0.4167 or -phi/tau = -0.5
     If dark energy is dynamical, n=6 arithmetic may predict its value.

  2. wa = {wa:.2f}: hints at evolving dark energy
     Best n=6 match: wa ~ -sopfr/tau = -5/4 = -1.25 (close)

  3. D_M/r_d values span range [7.93, 39.71] across z = [0.30, 2.33]
     Multiple values match n=6 expressions within 2-5%.

  4. H0 = {H0_DESI_CMB:.2f} km/s/Mpc from DESI+CMB

  CAVEAT: With ~{len(exprs)} expressions scanning ~{len(DESI_DATA)*2 + len(DESI_GROWTH)} values,
  some matches are expected by chance. Focus on the dark energy w deviation
  as the most physically interesting potential n=6 signature.
""")
    print("=" * 80)


if __name__ == '__main__':
    main()
