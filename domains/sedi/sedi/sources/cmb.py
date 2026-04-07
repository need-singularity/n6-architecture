"""Planck CMB (Cosmic Microwave Background) data source.

Data: https://pla.esac.esa.int/
Requires: healpy for HEALPix maps, or raw FITS files.
"""
import os
import numpy as np


def load_cmb_map(filepath):
    """Load a Planck CMB temperature map (HEALPix FITS format).

    Requires: pip install healpy astropy
    """
    try:
        import healpy as hp
    except ImportError:
        print("  [cmb] healpy required: pip install healpy")
        return None

    try:
        cmb_map = hp.read_map(filepath)
        return {
            'source': 'cmb',
            'data': cmb_map.tolist(),
            'n': len(cmb_map),
            'nside': hp.npix2nside(len(cmb_map)),
            'filepath': filepath,
        }
    except Exception as e:
        print(f"  [cmb] Error loading {filepath}: {e}")
    return None


def extract_power_spectrum(cmb_map_data, lmax=100):
    """Extract angular power spectrum C_l from CMB map.

    Returns multipole moments l=0..lmax and their power C_l.
    """
    try:
        import healpy as hp
    except ImportError:
        print("  [cmb] healpy required")
        return None

    data = np.array(cmb_map_data)
    cl = hp.anafast(data, lmax=lmax)
    return {
        'source': 'cmb-spectrum',
        'l': list(range(len(cl))),
        'cl': cl.tolist(),
        'data': cl.tolist(),
        'n': len(cl),
    }


def check_n6_multipoles(cl_data):
    """Check CMB power spectrum at n=6-related multipoles.

    Interesting l values: 6, 12, 24, 28, 36
    """
    from ..constants import N, SIGMA, TAU

    targets = {
        'l=n=6': N,
        'l=sigma=12': SIGMA,
        'l=sigma*phi=24': SIGMA * 2,
        'l=P2=28': 28,
        'l=n^2=36': N ** 2,
    }

    results = {}
    for name, l in targets.items():
        if l < len(cl_data):
            val = cl_data[l]
            # Compare to neighbors
            neighbors = [cl_data[i] for i in range(max(0, l-2), min(len(cl_data), l+3)) if i != l]
            mean_nb = np.mean(neighbors) if neighbors else val
            ratio = val / mean_nb if mean_nb != 0 else 1.0
            results[name] = {
                'l': l,
                'cl': float(val),
                'neighbor_mean': float(mean_nb),
                'ratio': float(ratio),
                'anomalous': abs(ratio - 1) > 0.5,
            }
    return results
