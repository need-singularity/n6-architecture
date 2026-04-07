"""CERN Open Data Portal source.

Data: https://opendata.cern.ch/
LHC collision data, simulated events, etc.
"""
import json
import logging
import urllib.request

from ._connection import fetch_json, logger

CERN_API = "https://opendata.cern.ch/api/records/"


def search_datasets(query='CMS', limit=10):
    """Search CERN Open Data portal for datasets."""
    url = f"{CERN_API}?q={query}&size={limit}&type=Dataset"
    data = fetch_json(url, source_tag='cern', timeout=15)
    if not data or not isinstance(data, dict):
        return []

    hits = data.get('hits', {}).get('hits', [])
    results = []
    for h in hits:
        try:
            results.append({
                'recid': h.get('id', ''),
                'title': h.get('metadata', {}).get('title', {}).get('title', ''),
                'experiment': h.get('metadata', {}).get('experiment', {}).get('experiment', ''),
                'year': h.get('metadata', {}).get('date_published', ''),
            })
        except (KeyError, AttributeError) as e:
            logger.warning("[cern] Skipping malformed dataset record: %s", e)
            continue
    return results


def fetch_record(recid):
    """Fetch a specific CERN Open Data record."""
    url = f"{CERN_API}{recid}"
    return fetch_json(url, source_tag='cern', timeout=15)


def list_particle_masses():
    """Known particle masses (GeV/c^2) for n=6 pattern matching.

    Check: any mass ratios near sigma/tau=3, phi/tau=0.5, etc.
    """
    particles = {
        'electron': 0.000511,
        'muon': 0.10566,
        'tau_lepton': 1.777,
        'up': 0.0022,
        'down': 0.0047,
        'strange': 0.095,
        'charm': 1.275,
        'bottom': 4.18,
        'top': 173.0,
        'W': 80.379,
        'Z': 91.1876,
        'Higgs': 125.1,
        'proton': 0.93827,
        'neutron': 0.93957,
        'pion_charged': 0.13957,
        'pion_neutral': 0.13498,
        'kaon': 0.49368,
    }
    return particles


def check_mass_ratios():
    """Check particle mass ratios against n=6 constants."""
    from ..constants import SIGMA, TAU, PHI, SOPFR, N

    targets = {
        'sigma/tau': SIGMA / TAU,
        'sopfr': float(SOPFR),
        'n': float(N),
        'sigma': float(SIGMA),
        'sigma*phi': float(SIGMA * PHI),
    }

    masses = list_particle_masses()
    mass_list = list(masses.items())
    hits = []

    for i in range(len(mass_list)):
        for j in range(i + 1, len(mass_list)):
            name_i, m_i = mass_list[i]
            name_j, m_j = mass_list[j]
            if m_i == 0 or m_j == 0:
                continue
            ratio = max(m_i, m_j) / min(m_i, m_j)
            for target_name, target_val in targets.items():
                if target_val > 0 and abs(ratio - target_val) / target_val < 0.03:
                    hits.append({
                        'particles': (name_i, name_j),
                        'ratio': ratio,
                        'target': target_name,
                        'target_val': target_val,
                        'error_pct': abs(ratio - target_val) / target_val * 100,
                    })

    return sorted(hits, key=lambda h: h['error_pct'])
