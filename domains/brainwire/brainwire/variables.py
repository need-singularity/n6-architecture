"""12-variable consciousness model constants."""

VAR_NAMES = [
    'DA', 'eCB', '5HT', 'GABA', 'NE',
    'Theta', 'Alpha', 'Gamma',
    'PFC', 'Sensory', 'Body', 'Coherence',
]

CHEM_VARS = ['DA', 'eCB', '5HT', 'GABA', 'NE']
WAVE_VARS = ['Theta', 'Alpha', 'Gamma']
STATE_VARS = ['PFC', 'Sensory', 'Body', 'Coherence']

VAR_CATEGORIES = {v: 'chem' for v in CHEM_VARS}
VAR_CATEGORIES.update({v: 'wave' for v in WAVE_VARS})
VAR_CATEGORIES.update({v: 'state' for v in STATE_VARS})

TENSION_WEIGHTS = {
    'DA': 1.2, 'eCB': 1.5, '5HT': 0.8, 'GABA': 0.9, 'NE': 1.0,
    'Theta': 1.3, 'Alpha': 1.0, 'Gamma': 1.1,
    'PFC': 1.0, 'Sensory': 0.9, 'Body': 1.0, 'Coherence': 1.2,
}

def baseline_vector() -> dict[str, float]:
    return {v: 1.0 for v in VAR_NAMES}
