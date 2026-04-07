"""Extended PDG Particle Database — ~200 states including excited, exotic.

Source: Particle Data Group (PDG) Review of Particle Physics 2024.
All masses in GeV/c^2.

Extends the base pdg.py (84 particles) with:
  - Excited mesons (~40 additional)
  - Excited baryons (~30 additional)
  - Exotic/recent states (~10 additional)

Each entry: (name, mass_GeV, uncertainty_GeV, category, status)
  status: 'established' (4-star PDG), 'seen' (3-star), 'claimed' (1-2 star)
"""

from .pdg import PARTICLES as BASE_PARTICLES, get_all as _base_get_all


# ─── Extended particles beyond the base 84 ───
# (name, mass_GeV, uncertainty_GeV, category, status)

EXTENDED_PARTICLES = [
    # =================================================================
    # EXCITED MESONS — Light unflavored
    # =================================================================
    ('f0_500_sigma',    0.400,        0.200,  'meson', 'established'),
    ('rho_1450',        1.465,        0.025,  'meson', 'established'),
    ('rho_1700',        1.720,        0.020,  'meson', 'established'),
    ('rho_2150',        2.150,        0.100,  'meson', 'seen'),
    ('omega_1420',      1.410,        0.060,  'meson', 'established'),
    ('omega_1650',      1.670,        0.030,  'meson', 'established'),
    ('phi_1680',        1.680,        0.020,  'meson', 'established'),
    ('phi_2170',        2.162,        0.007,  'meson', 'established'),
    ('f0_1710',         1.704,        0.012,  'meson', 'established'),
    ('f0_2020',         1.992,        0.016,  'meson', 'established'),
    ('f0_2100',         2.101,        0.007,  'meson', 'established'),
    ('f0_2200',         2.189,        0.013,  'meson', 'seen'),
    ('a1_1260',         1.230,        0.040,  'meson', 'established'),
    ('a2_1700',         1.698,        0.012,  'meson', 'seen'),
    ('b1_1235',         1.2295,       0.0032, 'meson', 'established'),
    ('pi2_1670',        1.6706,       0.0012, 'meson', 'established'),
    ('eta2_1645',       1.617,        0.005,  'meson', 'established'),
    ('omega3_1670',     1.667,        0.004,  'meson', 'established'),
    ('pi1_1600',        1.660,        0.011,  'meson', 'seen'),
    ('a0_1450',         1.474,        0.019,  'meson', 'established'),
    ('h1_1170',         1.166,        0.006,  'meson', 'established'),
    ('f1_1285',         1.2819,       0.0005, 'meson', 'established'),
    ('f1_1420',         1.4264,       0.0009, 'meson', 'established'),

    # =================================================================
    # EXCITED MESONS — Strange
    # =================================================================
    ('K_star_1410',     1.414,        0.015,  'meson', 'established'),
    ('K_star_1680',     1.718,        0.018,  'meson', 'established'),
    ('K1_1400',         1.403,        0.007,  'meson', 'established'),
    ('K2_1770',         1.773,        0.008,  'meson', 'seen'),
    ('K0_star_1430',    1.425,        0.050,  'meson', 'established'),
    ('K3_star_1780',    1.776,        0.007,  'meson', 'established'),

    # =================================================================
    # EXCITED MESONS — Charmed
    # =================================================================
    ('D_star_2010',     2.01026,      0.00005, 'meson', 'established'),
    ('D1_2420',         2.4232,       0.0006, 'meson', 'established'),
    ('D2_star_2460',    2.4611,       0.0007, 'meson', 'established'),
    ('Ds1_2536',        2.5353,       0.0004, 'meson', 'established'),
    ('Ds2_star_2573',   2.5691,       0.0008, 'meson', 'established'),
    ('Ds0_star_2317',   2.3178,       0.0005, 'meson', 'established'),
    ('D_s1_2460',       2.4595,       0.0006, 'meson', 'established'),

    # =================================================================
    # EXCITED MESONS — Bottom
    # =================================================================
    ('B_star_5325',     5.3247,       0.0006, 'meson', 'established'),
    ('B1_5721',         5.7261,       0.0023, 'meson', 'established'),
    ('B2_star_5747',    5.7372,       0.0007, 'meson', 'established'),
    ('Bs_star_5415',    5.4154,       0.0014, 'meson', 'established'),
    ('Bs1_5830',        5.8287,       0.0013, 'meson', 'established'),
    ('Bs2_star_5840',   5.8397,       0.0006, 'meson', 'established'),

    # =================================================================
    # EXCITED MESONS — Charmonium / Bottomonium
    # =================================================================
    ('psi_4040',        4.039,        0.001,  'meson', 'established'),
    ('psi_4160',        4.191,        0.005,  'meson', 'established'),
    ('psi_4415',        4.421,        0.004,  'meson', 'established'),
    ('chi_c2',          3.55617,      0.00007, 'meson', 'established'),
    ('h_c',             3.52538,      0.00011, 'meson', 'established'),
    ('eta_c_2S',        3.6375,       0.0011, 'meson', 'established'),
    ('chi_b1_1P',       9.89278,      0.00026, 'meson', 'established'),
    ('chi_b2_1P',       9.91221,      0.00026, 'meson', 'established'),
    ('chi_b0_2P',      10.2325,       0.0004, 'meson', 'established'),
    ('chi_b1_2P',      10.25546,      0.00022, 'meson', 'established'),
    ('chi_b2_2P',      10.26865,      0.00022, 'meson', 'established'),
    ('Upsilon_10860',  10.8852,       0.0011, 'meson', 'established'),
    ('Upsilon_11020',  11.000,        0.004,  'meson', 'established'),
    ('h_b_1P',          9.8993,       0.0008, 'meson', 'established'),
    ('h_b_2P',         10.2598,       0.0012, 'meson', 'established'),
    ('eta_b_2S',        9.999,        0.004,  'meson', 'established'),

    # =================================================================
    # EXCITED BARYONS — Nucleon resonances
    # =================================================================
    ('N_1650',          1.655,        0.015,  'baryon', 'established'),
    ('N_1675',          1.675,        0.005,  'baryon', 'established'),
    ('N_1700',          1.700,        0.050,  'baryon', 'seen'),
    ('N_1710',          1.710,        0.030,  'baryon', 'established'),
    ('N_1720',          1.720,        0.030,  'baryon', 'established'),
    ('N_1860',          1.860,        0.060,  'baryon', 'seen'),
    ('N_1875',          1.875,        0.025,  'baryon', 'seen'),
    ('N_1880',          1.880,        0.030,  'baryon', 'seen'),
    ('N_1895',          1.895,        0.025,  'baryon', 'seen'),
    ('N_1900',          1.920,        0.030,  'baryon', 'seen'),
    ('N_2190',          2.180,        0.040,  'baryon', 'established'),
    ('N_2220',          2.230,        0.050,  'baryon', 'established'),
    ('N_2250',          2.280,        0.050,  'baryon', 'established'),

    # =================================================================
    # EXCITED BARYONS — Delta resonances
    # =================================================================
    ('Delta_1600',      1.570,        0.070,  'baryon', 'established'),
    ('Delta_1620',      1.615,        0.015,  'baryon', 'established'),
    ('Delta_1700',      1.710,        0.030,  'baryon', 'established'),
    ('Delta_1900',      1.860,        0.060,  'baryon', 'seen'),
    ('Delta_1905',      1.880,        0.030,  'baryon', 'established'),
    ('Delta_1910',      1.900,        0.040,  'baryon', 'established'),
    ('Delta_1920',      1.920,        0.050,  'baryon', 'established'),
    ('Delta_1930',      1.950,        0.050,  'baryon', 'seen'),
    ('Delta_1950',      1.930,        0.015,  'baryon', 'established'),

    # =================================================================
    # EXCITED BARYONS — Lambda resonances
    # =================================================================
    ('Lambda_1405',     1.4051,       0.0013, 'baryon', 'established'),
    ('Lambda_1800',     1.800,        0.050,  'baryon', 'seen'),
    ('Lambda_1810',     1.800,        0.040,  'baryon', 'seen'),
    ('Lambda_1830',     1.830,        0.020,  'baryon', 'established'),
    ('Lambda_1890',     1.890,        0.020,  'baryon', 'established'),
    ('Lambda_2100',     2.090,        0.020,  'baryon', 'established'),
    ('Lambda_2110',     2.110,        0.030,  'baryon', 'seen'),

    # =================================================================
    # EXCITED BARYONS — Sigma resonances
    # =================================================================
    ('Sigma_1660',      1.660,        0.030,  'baryon', 'seen'),
    ('Sigma_1670',      1.670,        0.015,  'baryon', 'established'),
    ('Sigma_1750',      1.750,        0.050,  'baryon', 'seen'),
    ('Sigma_1775',      1.775,        0.005,  'baryon', 'established'),
    ('Sigma_1915',      1.915,        0.020,  'baryon', 'established'),
    ('Sigma_2030',      2.025,        0.010,  'baryon', 'established'),

    # =================================================================
    # EXCITED BARYONS — Xi resonances
    # =================================================================
    ('Xi_1620',         1.620,        0.020,  'baryon', 'claimed'),
    ('Xi_1690',         1.690,        0.010,  'baryon', 'seen'),
    ('Xi_1820',         1.823,        0.005,  'baryon', 'established'),
    ('Xi_1950',         1.950,        0.015,  'baryon', 'seen'),
    ('Xi_2030',         2.025,        0.005,  'baryon', 'established'),

    # =================================================================
    # EXCITED BARYONS — Omega
    # =================================================================
    ('Omega_2012',      2.0121,       0.0003, 'baryon', 'established'),

    # =================================================================
    # EXOTIC / RECENT DISCOVERIES
    # =================================================================
    ('X_3872',          3.87168,      0.00017, 'exotic', 'established'),
    ('X_3940',          3.942,        0.009,   'exotic', 'seen'),
    ('Zc_3900',         3.8884,       0.0012,  'exotic', 'established'),
    ('Zc_4020',         4.0243,       0.0009,  'exotic', 'established'),
    ('Zcs_3985',        3.9826,       0.0023,  'exotic', 'established'),
    ('Pc_4312',         4.3118,       0.0008,  'exotic', 'established'),
    ('Pc_4440',         4.4406,       0.0012,  'exotic', 'established'),
    ('Pc_4457',         4.4573,       0.0009,  'exotic', 'established'),
    ('Tcc_3875',        3.8748,       0.0001,  'exotic', 'established'),
    ('X_4140',          4.1463,       0.0033,  'exotic', 'established'),
    ('X_4274',          4.2741,       0.0079,  'exotic', 'established'),
    ('X_4500',          4.506,        0.011,   'exotic', 'seen'),
    ('X_4700',          4.704,        0.010,   'exotic', 'seen'),
    ('X_6900',          6.905,        0.011,   'exotic', 'seen'),

    # =================================================================
    # CHARMED BARYONS — additional
    # =================================================================
    ('Sigma_c_2520',    2.5184,       0.0006, 'baryon', 'established'),
    ('Xi_c_prime',      2.5782,       0.0005, 'baryon', 'established'),
    ('Xi_c_2645',       2.6461,       0.0003, 'baryon', 'established'),
    ('Xi_c_2790',       2.7921,       0.0005, 'baryon', 'established'),
    ('Xi_c_2815',       2.8196,       0.0003, 'baryon', 'established'),
    ('Omega_c_2770',    2.7659,       0.0020, 'baryon', 'established'),
    ('Lambda_c_2595',   2.5924,       0.0005, 'baryon', 'established'),
    ('Lambda_c_2625',   2.6282,       0.0002, 'baryon', 'established'),
    ('Lambda_c_2860',   2.8562,       0.0009, 'baryon', 'established'),

    # =================================================================
    # BOTTOM BARYONS — additional
    # =================================================================
    ('Sigma_b_plus',    5.8105,       0.0017, 'baryon', 'established'),
    ('Sigma_b_minus',   5.8155,       0.0018, 'baryon', 'established'),
    ('Sigma_b_star_p',  5.8321,       0.0017, 'baryon', 'established'),
    ('Sigma_b_star_m',  5.8353,       0.0018, 'baryon', 'established'),
    ('Xi_b_prime',      5.9352,       0.0005, 'baryon', 'established'),
    ('Xi_b_star',       5.9553,       0.0006, 'baryon', 'established'),
    ('Omega_b_6316',    6.3152,       0.0028, 'baryon', 'seen'),
    ('Omega_b_6330',    6.3300,       0.0028, 'baryon', 'seen'),
    ('Omega_b_6340',    6.3396,       0.0024, 'baryon', 'seen'),
    ('Omega_b_6350',    6.3498,       0.0028, 'baryon', 'seen'),
]


def _convert_base(p):
    """Convert base 4-tuple to 5-tuple with status='established'."""
    return (p[0], p[1], p[2], p[3], 'established')


# Full combined list
ALL_PARTICLES = [_convert_base(p) for p in BASE_PARTICLES] + EXTENDED_PARTICLES


def get_all():
    """Return all particles (base + extended) as list of dicts."""
    return [{'name': p[0], 'mass': p[1], 'unc': p[2],
             'category': p[3], 'status': p[4]}
            for p in ALL_PARTICLES]


def get_base_only():
    """Return only the original 84 base particles."""
    return [{'name': p[0], 'mass': p[1], 'unc': p[2],
             'category': p[3], 'status': 'established'}
            for p in BASE_PARTICLES]


def get_by_category(category):
    """Filter particles by category."""
    return [p for p in get_all() if p['category'] == category]


def get_by_status(status):
    """Filter particles by status."""
    return [p for p in get_all() if p['status'] == status]


def get_masses():
    """Return dict of {name: mass}."""
    return {p[0]: p[1] for p in ALL_PARTICLES}


def get_exotics():
    return get_by_category('exotic')


def summary():
    """Print summary statistics."""
    all_p = get_all()
    cats = {}
    stats = {}
    for p in all_p:
        cats[p['category']] = cats.get(p['category'], 0) + 1
        stats[p['status']] = stats.get(p['status'], 0) + 1
    return {
        'total': len(all_p),
        'by_category': cats,
        'by_status': stats,
        'mass_range': (min(p['mass'] for p in all_p),
                       max(p['mass'] for p in all_p)),
    }
