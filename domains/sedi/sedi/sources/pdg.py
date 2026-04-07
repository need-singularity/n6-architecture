"""PDG Particle Database — comprehensive particle physics data.

Source: Particle Data Group (PDG) Review of Particle Physics.
All masses in GeV/c^2 unless noted.
"""


# (name, mass_GeV, uncertainty_GeV, category)
# uncertainty = 0 means exact or negligible
PARTICLES = [
    # ─── Leptons ───
    ('electron',        0.000510999,  0.000000000, 'lepton'),
    ('muon',            0.105658,     0.000001,    'lepton'),
    ('tau',             1.77686,      0.00012,     'lepton'),
    # neutrinos: mass upper limits only, not included for ratio analysis

    # ─── Quarks (MS-bar at 2 GeV) ───
    ('up',              0.00216,      0.00049,     'quark'),
    ('down',            0.00467,      0.00048,     'quark'),
    ('strange',         0.0934,       0.0084,      'quark'),
    ('charm',           1.27,         0.02,        'quark'),
    ('bottom',          4.18,         0.03,        'quark'),
    ('top',             172.76,       0.30,        'quark'),

    # ─── Gauge Bosons ───
    ('W',               80.377,       0.012,       'gauge'),
    ('Z',               91.1876,      0.0021,      'gauge'),
    ('Higgs',           125.25,       0.17,        'gauge'),
    # photon: massless, gluon: massless

    # ─── Light Unflavored Mesons ───
    ('pi_pm',           0.13957,      0.00001,     'meson'),
    ('pi_0',            0.13498,      0.00001,     'meson'),
    ('eta',             0.54786,      0.00017,     'meson'),
    ('rho_770',         0.77526,      0.00025,     'meson'),
    ('omega_782',       0.78266,      0.00013,     'meson'),
    ('eta_prime_958',   0.95778,      0.00006,     'meson'),
    ('f0_980',          0.990,        0.020,       'meson'),
    ('a0_980',          0.980,        0.020,       'meson'),
    ('phi_1020',        1.019461,     0.000016,    'meson'),
    ('f2_1270',         1.2755,       0.0008,      'meson'),
    ('f0_1370',         1.370,        0.040,       'meson'),
    ('a2_1320',         1.3169,       0.0009,      'meson'),
    ('f0_1500',         1.506,        0.006,       'meson'),
    ('f2p_1525',        1.5174,       0.0013,      'meson'),
    ('rho3_1690',       1.6888,       0.0021,      'meson'),

    # ─── Strange Mesons ───
    ('K_pm',            0.49368,      0.00001,     'meson'),
    ('K_0',             0.49761,      0.00001,     'meson'),
    ('K_star_892',      0.89167,      0.00026,     'meson'),
    ('K1_1270',         1.253,        0.007,       'meson'),
    ('K2_star_1430',    1.4273,       0.0015,      'meson'),

    # ─── Charmed Mesons ───
    ('D_pm',            1.86966,      0.00005,     'meson'),
    ('D_0',             1.86484,      0.00005,     'meson'),
    ('D_star_2007',     2.00685,      0.00005,     'meson'),
    ('D_s',             1.96835,      0.00033,     'meson'),
    ('D_s_star',        2.1122,       0.0004,      'meson'),

    # ─── Bottom Mesons ───
    ('B_pm',            5.27934,      0.00012,     'meson'),
    ('B_0',             5.27965,      0.00012,     'meson'),
    ('B_s',             5.36688,      0.00014,     'meson'),
    ('B_c',             6.2749,       0.0008,      'meson'),

    # ─── Charmonium ───
    ('eta_c',           2.9839,       0.0004,      'meson'),
    ('J_psi',           3.09690,      0.00001,     'meson'),
    ('chi_c0',          3.41471,      0.00030,     'meson'),
    ('chi_c1',          3.51067,      0.00005,     'meson'),
    ('psi_2S',          3.68610,      0.00001,     'meson'),
    ('psi_3770',        3.7737,       0.0007,      'meson'),

    # ─── Bottomonium ───
    ('eta_b',           9.3990,       0.0023,      'meson'),
    ('Upsilon_1S',      9.46040,      0.00010,     'meson'),
    ('chi_b0_1P',       9.85944,      0.00052,     'meson'),
    ('Upsilon_2S',      10.02326,     0.00031,     'meson'),
    ('Upsilon_3S',      10.3552,      0.0005,      'meson'),
    ('Upsilon_4S',      10.5794,      0.0012,      'meson'),

    # ─── Light Baryons ───
    ('proton',          0.938272,     0.000001,    'baryon'),
    ('neutron',         0.939565,     0.000001,    'baryon'),
    ('Delta_1232',      1.232,        0.001,       'baryon'),
    ('N_1440',          1.430,        0.020,       'baryon'),
    ('N_1520',          1.515,        0.005,       'baryon'),
    ('N_1535',          1.530,        0.010,       'baryon'),
    ('N_1680',          1.685,        0.005,       'baryon'),

    # ─── Strange Baryons ───
    ('Lambda',          1.115683,     0.000006,    'baryon'),
    ('Sigma_plus',      1.18937,      0.00001,     'baryon'),
    ('Sigma_0',         1.192642,     0.000024,    'baryon'),
    ('Sigma_minus',     1.19745,      0.00001,     'baryon'),
    ('Xi_0',            1.31486,      0.00020,     'baryon'),
    ('Xi_minus',        1.32171,      0.00007,     'baryon'),
    ('Omega_minus',     1.67245,      0.00029,     'baryon'),
    ('Sigma_1385',      1.3828,       0.0004,      'baryon'),
    ('Xi_1530',         1.5318,       0.0003,      'baryon'),
    ('Lambda_1520',     1.5195,       0.0010,      'baryon'),
    ('Lambda_1600',     1.600,        0.050,       'baryon'),
    ('Lambda_1670',     1.670,        0.010,       'baryon'),
    ('Lambda_1690',     1.690,        0.005,       'baryon'),
    ('Lambda_1820',     1.820,        0.005,       'baryon'),

    # ─── Charmed Baryons ───
    ('Lambda_c',        2.28646,      0.00014,     'baryon'),
    ('Sigma_c_2455',    2.45398,      0.00016,     'baryon'),
    ('Xi_c_plus',       2.4679,       0.0004,      'baryon'),
    ('Xi_c_0',          2.4709,       0.0004,      'baryon'),
    ('Omega_c',         2.6952,       0.0017,      'baryon'),
    ('Xi_cc',           3.6212,       0.0004,      'baryon'),

    # ─── Bottom Baryons ───
    ('Lambda_b',        5.61960,      0.00017,     'baryon'),
    ('Xi_b_minus',      5.7970,       0.0006,      'baryon'),
    ('Xi_b_0',          5.7919,       0.0005,      'baryon'),
    ('Omega_b',         6.0461,       0.0017,      'baryon'),
]


def get_all():
    """Return all particles as list of dicts."""
    return [{'name': p[0], 'mass': p[1], 'unc': p[2], 'category': p[3]}
            for p in PARTICLES]


def get_by_category(category):
    """Filter particles by category."""
    return [p for p in get_all() if p['category'] == category]


def get_masses():
    """Return dict of {name: mass}."""
    return {p[0]: p[1] for p in PARTICLES}


def get_leptons():
    return get_by_category('lepton')

def get_quarks():
    return get_by_category('quark')

def get_baryons():
    return get_by_category('baryon')

def get_mesons():
    return get_by_category('meson')

def get_gauge():
    return get_by_category('gauge')


# Particle counts by category
COUNTS = {
    'leptons_charged': 3,       # e, mu, tau
    'quark_flavors': 6,         # u, d, s, c, b, t
    'gauge_massive': 3,         # W, Z, H
    'fermion_generations': 3,   # 3 generations
    'SM_gauge_generators': 12,  # 8+3+1 = dim(su3+su2+u1)
    'quarks_per_gen': 2,        # up-type + down-type
    'leptons_per_gen': 2,       # charged + neutrino
    'color_charges': 3,         # RGB
}
