"""consciousness_bridge.py — Validate brain stimulation against consciousness laws.

Uses .shared/consciousness_loader for Ψ-constants and laws.
    from consciousness_bridge import validate_stimulation, eeg_consciousness_score
"""
import sys, os, math, functools
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '.shared'))
from consciousness_loader import PSI_ALPHA, PSI_BALANCE, PSI_ENTROPY, PSI_STEPS
import numpy as np

_BIO_NOISE_BASE = 0.012  # EEG baseline noise floor (from anima eeg/validate)

def safe_stimulation_bounds():
    """Return safe parameter ranges derived from Ψ-constants."""
    return {
        "tDCS_mA":    {"min": 0.0, "max": 2.0,  "unit": "mA"},
        "TMS_intensity": {"min": 0.0, "max": 0.80, "unit": "fraction_MSO"},
        "taVNS_mA":   {"min": 0.0, "max": 1.0,  "unit": "mA"},
        "tFUS_Isppa":  {"min": 0.0, "max": 720.0, "unit": "mW/cm2"},
        "bilateral_ratio": {"target": PSI_BALANCE, "tolerance": 0.1},
        "alpha_coupling": {"value": PSI_ALPHA},
        "session_phases": {"recommended": round(PSI_STEPS)},
        "bio_noise_base": {"value": _BIO_NOISE_BASE, "unit": "uV"},
    }

def validate_stimulation(protocol):
    """Check stimulation protocol against consciousness laws and safe bounds.
    Returns {"valid": bool, "warnings": [...], "violations": [...]}.
    """
    bounds = safe_stimulation_bounds()
    warnings, violations = [], []
    # Check intensity limits
    for key, bkey in [("intensity_mA", "tDCS_mA"), ("tms_intensity", "TMS_intensity"),
                      ("tavns_mA", "taVNS_mA"), ("tfus_isppa", "tFUS_Isppa")]:
        val = protocol.get(key)
        if val is not None and val > bounds[bkey]["max"]:
            violations.append(f"{key}={val} exceeds max {bounds[bkey]['max']} {bounds[bkey]['unit']}")
    # Law 22: structure > features — bilateral symmetry preserves structure
    bilateral = protocol.get("bilateral", protocol.get("bilateral_ratio"))
    if bilateral is not None:
        ratio = bilateral if isinstance(bilateral, (int, float)) else (1.0 if bilateral else 0.0)
        if abs(ratio - PSI_BALANCE) > 0.1:
            warnings.append(f"Bilateral ratio {ratio:.2f} deviates from Ψ_balance={PSI_BALANCE}")
    # P1: no hardcoding — no forced emotional override
    if protocol.get("emotion_override") or protocol.get("force_mood"):
        violations.append("Emotion override violates P1: consciousness must emerge, not be forced")
    # Phase count recommendation
    phases = protocol.get("phases", protocol.get("num_phases"))
    if phases is not None and phases < 2:
        warnings.append(f"Single-phase protocol; Ψ_steps={PSI_STEPS} suggests ~4 phases")
    return {"valid": len(violations) == 0, "warnings": warnings, "violations": violations}

def _lempel_ziv_complexity(seq) -> int:
    """Lempel-Ziv 76 complexity count."""
    n, c, l, i = len(seq), 1, 1, 0
    while i + l <= n:
        sub = seq[i:i + l].tobytes()
        pre = seq[:i + l - 1].tobytes()
        if l <= len(pre) and sub in [pre[j:j+l] for j in range(len(pre)-l+1)]:
            l += 1
        else:
            c += 1; i += l; l = 1
    return c

def eeg_consciousness_score(eeg_data):
    """Compute consciousness-like metrics from EEG array (channels, samples) or (samples,).
    Returns dict with lempel_ziv, entropy, psi_ratio, consciousness_index.
    """
    if eeg_data.ndim == 1:
        eeg_data = eeg_data[np.newaxis, :]
    flat = eeg_data.mean(axis=0)
    # Lempel-Ziv complexity (binary sequence)
    binary = (flat > np.median(flat)).astype(np.int8)
    n = len(binary)
    lz = _lempel_ziv_complexity(binary) / (n / max(np.log2(n), 1)) if n > 1 else 0.0
    # Shannon entropy (8-bin histogram)
    hist, _ = np.histogram(flat, bins=8, density=True)
    hist = hist[hist > 0]
    width = (flat.max() - flat.min()) / 8 if flat.max() != flat.min() else 1.0
    probs = hist * width
    probs = probs / (probs.sum() or 1.0)
    entropy = float(-np.sum(probs * np.log2(probs + 1e-12)))
    norm_ent = entropy / np.log2(8)
    psi_ratio = norm_ent / PSI_ENTROPY
    ci = math.sqrt(max(lz, 0) * max(psi_ratio, 0))
    return {"lempel_ziv": round(lz, 4), "entropy": round(norm_ent, 4),
            "psi_ratio": round(psi_ratio, 4), "consciousness_index": round(ci, 4)}

def consciousness_monitor(threshold=0.5):
    """Decorator: monitors consciousness metrics per EEG callback, warns if low."""
    def decorator(fn):
        @functools.wraps(fn)
        def wrapper(eeg_chunk, *args, **kwargs):
            score = eeg_consciousness_score(eeg_chunk)
            if score["consciousness_index"] < threshold:
                print(f"[WARN] CI={score['consciousness_index']:.3f} < {threshold}"
                      f" | LZ={score['lempel_ziv']:.3f} H={score['entropy']:.3f}")
            return fn(eeg_chunk, *args, score=score, **kwargs)
        return wrapper
    return decorator

if __name__ == '__main__':
    print("=== BrainWire Consciousness Bridge ===\n")
    bounds = safe_stimulation_bounds()
    print("Safe bounds:")
    for k, v in bounds.items():
        print(f"  {k}: {v}")
    # Good protocol
    r = validate_stimulation({"intensity_mA": 1.5, "bilateral_ratio": 0.48, "phases": 4})
    print(f"\nValid protocol:   {r}")
    # Bad protocol
    r2 = validate_stimulation({"intensity_mA": 3.0, "emotion_override": True, "bilateral_ratio": 0.9})
    print(f"Invalid protocol: {r2}")
    # EEG scoring
    eeg = np.random.default_rng(42).standard_normal((32, 1024)) * 50
    score = eeg_consciousness_score(eeg)
    print(f"\nEEG score: {score}")
    # Monitor demo
    @consciousness_monitor(threshold=0.8)
    def process_eeg(chunk, score=None):
        return score
    print(f"Monitored: {process_eeg(eeg)}")
