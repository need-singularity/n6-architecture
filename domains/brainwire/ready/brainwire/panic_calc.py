"""Panic Disorder Treatment Calculator.

Mathematical framework for N1-based panic disorder treatment via cortical stimulation.

Core Equations:
  Eq P1: LC-NE surge model — NE(t) = NE_base + A × exp(-t/τ_surge) × (1 - suppression)
  Eq P2: Amygdala-PFC inhibitory restoration — W_inh(n) = W_ceil - (W_ceil-W_0)×(1-a+)^(n×η)
  Eq P3: Fear extinction rate — F(n) = F_0 × exp(-k_ext × n × W_inh)
  Eq P4: Interoceptive gain model — G_int(t) = G_0 × (1 - β × I_insula) / (1 + γ × PFC_activity)
  Eq P5: Panic attack probability — P_panic(NE, GABA, PFC) = σ(w_NE×NE - w_GABA×GABA - w_PFC×PFC - θ)
  Eq P6: GABA deficit recovery — GABA(n) = GABA_floor + (GABA_target - GABA_floor)×(1-(1-r)^n)
  Eq P7: Autonomic storm index — ASI = NE²/GABA × (Sensory × Body) / (PFC × Coherence)
  Eq P8: Acute suppression response time — T_suppress = T_detect + T_compute + T_stim
  Eq P9: Fear circuit resonance — ω_fear = 2π × f_amygdala, damping ratio ζ = PFC/√(4×Amyg×LC)
  Eq P10: Long-term panic freedom — P_free(n) = 1 - exp(-μ × n × W_inh × GABA_recovery)

Profiles (12-variable):
  Pathology: DA= eCB↓ 5HT↓ GABA↓↓ NE↑↑↑ Theta↓ Alpha↓↓ Gamma↑↑ PFC↓↓ Sensory↑↑ Body↑↑↑ Coherence↓↓
  Treatment: Acute NE suppression + GABA restoration + PFC activation
  Maintenance: PFC→Amygdala pathway strengthening + fear extinction
"""
import math


# ══════════════════════════════════════════════════════════════════════════
# Panic Disorder 12-Variable Profiles (3-phase)
# ══════════════════════════════════════════════════════════════════════════

PANIC_PATHOLOGY = {
    'DA': 1.0, 'eCB': 0.6, '5HT': 0.7, 'GABA': 0.5, 'NE': 2.5,
    'Theta': 0.8, 'Alpha': 0.4, 'Gamma': 1.8,
    'PFC': 0.4, 'Sensory': 2.5, 'Body': 3.0, 'Coherence': 0.4,
}

PANIC_TREATMENT = {
    'DA': 1.0, 'eCB': 1.1, '5HT': 1.1, 'GABA': 1.4, 'NE': 0.6,
    'Theta': 1.1, 'Alpha': 1.3, 'Gamma': 0.9,
    'PFC': 1.3, 'Sensory': 0.8, 'Body': 0.8, 'Coherence': 1.3,
}

PANIC_MAINTENANCE = {
    'DA': 1.0, 'eCB': 1.0, '5HT': 1.0, 'GABA': 1.1, 'NE': 0.9,
    'Theta': 1.0, 'Alpha': 1.1, 'Gamma': 0.95,
    'PFC': 1.1, 'Sensory': 0.95, 'Body': 0.95, 'Coherence': 1.1,
}

PANIC_BASELINE = {k: 1.0 for k in PANIC_PATHOLOGY}


# ══════════════════════════════════════════════════════════════════════════
# Circuit Definitions
# ══════════════════════════════════════════════════════════════════════════

PANIC_CIRCUITS = {
    'fear_circuit': {
        'pathway': 'Amygdala → PAG → Autonomic cascade',
        'pathology': 'Amygdala hyperactivation, false threat signaling',
        'cortical_source': 'PFC',
        'subcortical_target': 'Amygdala',
        'f_project': 0.35,  # Ghashghaei & Barbas 2002
        'primary_variable': 'NE',
        'refs': ['Gorman 2000', 'LeDoux 1996'],
    },
    'lc_ne_surge': {
        'pathway': 'LC → Forebrain NE flooding',
        'pathology': 'Sudden sympathetic storm: tachycardia, sweating, tremor',
        'cortical_source': 'PFC',
        'subcortical_target': 'LC',
        'f_project': 0.28,  # Jodo & Aston-Jones 1997
        'primary_variable': 'NE',
        'refs': ['Bremner 1996', 'Aston-Jones 2005'],
    },
    'interoceptive': {
        'pathway': 'Insula → ACC → Amygdala',
        'pathology': 'Normal body signals misinterpreted as threat',
        'cortical_source': 'Insula',
        'subcortical_target': 'Amygdala',
        'f_project': 0.35,  # Augustine 1996
        'primary_variable': 'Sensory',
        'refs': ['Paulus & Stein 2006', 'Craig 2002'],
    },
    'pfc_inhibition_failure': {
        'pathway': 'vmPFC → Amygdala (inhibitory)',
        'pathology': 'Top-down control failure, impaired fear extinction',
        'cortical_source': 'PFC',
        'subcortical_target': 'Amygdala',
        'f_project': 0.35,
        'primary_variable': 'PFC',
        'refs': ['Milad 2005', 'Quirk 2006'],
    },
}


# ══════════════════════════════════════════════════════════════════════════
# Eq P1: LC-NE Surge Model
# ══════════════════════════════════════════════════════════════════════════

def lc_ne_surge(NE_base: float = 1.0, A_surge: float = 1.5,
                tau_surge_s: float = 30.0, t_s: float = 0.0,
                suppression: float = 0.0) -> dict:
    """NE(t) = NE_base + A × exp(-t/τ) × (1 - suppression).

    Models the norepinephrine surge during a panic attack.
    A_surge: peak amplitude above baseline (1.5 = NE reaches 2.5×).
    tau_surge: time constant of surge decay (30s untreated).
    suppression: N1 stimulation suppression factor (0=none, 1=complete).
    """
    ne_t = NE_base + A_surge * math.exp(-t_s / tau_surge_s) * (1.0 - suppression)
    # Time to return below 1.3× threshold
    if A_surge * (1.0 - suppression) > 0.3:
        t_threshold = -tau_surge_s * math.log(0.3 / (A_surge * (1.0 - suppression)))
        t_threshold = max(0, t_threshold)
    else:
        t_threshold = 0.0  # already below threshold

    return {
        'NE_base': NE_base,
        'A_surge': A_surge,
        'tau_surge_s': tau_surge_s,
        'suppression': suppression,
        'NE_peak': NE_base + A_surge * (1.0 - suppression),
        'NE_at_t': ne_t,
        't_below_threshold_s': t_threshold,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq P2: Amygdala-PFC Inhibitory Pathway Restoration (STDP)
# ══════════════════════════════════════════════════════════════════════════

def amygdala_pfc_restoration(n_sessions: int = 40, pulses_per_session: int = 800,
                              eta: float = 0.8, a_plus: float = 0.004,
                              w0: float = 0.4, w_ceil: float = 1.0) -> dict:
    """W_inh(n) = W_ceil - (W_ceil - W_0) × (1 - a+)^(n × η).

    Restores vmPFC→Amygdala inhibitory pathway via STDP potentiation.
    w0=0.4: severely weakened inhibition in panic disorder.
    Lower a_plus than depression (0.004 vs 0.005): inhibitory synapses
    potentiate slower than excitatory ones (Kullmann 2012).
    """
    effective = n_sessions * pulses_per_session * eta
    w_final = w_ceil - (w_ceil - w0) * (1.0 - a_plus) ** effective
    recovery_pct = ((w_final - w0) / (w_ceil - w0)) * 100

    trajectory = []
    for s in range(0, n_sessions + 1, max(1, n_sessions // 10)):
        eff = s * pulses_per_session * eta
        w = w_ceil - (w_ceil - w0) * (1.0 - a_plus) ** eff
        trajectory.append({'session': s, 'weight': w,
                          'recovery_pct': ((w - w0) / (w_ceil - w0)) * 100})
    return {
        'n_sessions': n_sessions,
        'effective_pulses': effective,
        'w_initial': w0,
        'w_final': w_final,
        'recovery_pct': recovery_pct,
        'trajectory': trajectory,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq P3: Fear Extinction Rate
# ══════════════════════════════════════════════════════════════════════════

def fear_extinction(F_0: float = 1.0, k_ext: float = 0.05,
                    n_sessions: int = 40, w_inh: float = 0.4) -> dict:
    """F(n) = F_0 × exp(-k_ext × n × W_inh).

    Fear response decays exponentially, rate proportional to inhibitory weight.
    Stronger PFC→Amygdala inhibition (higher W_inh) = faster extinction.
    """
    trajectory = []
    for s in range(0, n_sessions + 1, max(1, n_sessions // 10)):
        # W_inh also improves over sessions (coupled with Eq P2)
        w_s = 0.4 + (1.0 - 0.4) * (1.0 - (1.0 - 0.004) ** (s * 800 * 0.8))
        f = F_0 * math.exp(-k_ext * s * w_s)
        trajectory.append({'session': s, 'fear': f, 'w_inh': w_s})

    f_final = trajectory[-1]['fear']
    # Sessions to 50% fear reduction
    # Approximate: since w_inh changes, use numerical search
    s_50 = None
    for t in trajectory:
        if t['fear'] <= F_0 * 0.5 and s_50 is None:
            s_50 = t['session']
    return {
        'F_0': F_0,
        'F_final': f_final,
        'reduction_pct': (1.0 - f_final / F_0) * 100,
        'sessions_to_50pct': s_50,
        'trajectory': trajectory,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq P4: Interoceptive Gain Model
# ══════════════════════════════════════════════════════════════════════════

def interoceptive_gain(G_0: float = 2.5, beta_insula: float = 0.3,
                       I_insula: float = 0.0, gamma_pfc: float = 0.5,
                       PFC_activity: float = 0.4) -> dict:
    """G_int(t) = G_0 × (1 - β × I_insula) / (1 + γ × PFC_activity).

    Models how interoceptive sensitivity is modulated by:
    - Insula stimulation (reduces raw gain)
    - PFC activity (provides top-down regulation)
    G_0=2.5: pathological hypersensitivity.
    """
    numerator = G_0 * (1.0 - beta_insula * min(I_insula, 1.0))
    denominator = 1.0 + gamma_pfc * PFC_activity
    g_int = numerator / denominator
    return {
        'G_0': G_0,
        'I_insula': I_insula,
        'PFC_activity': PFC_activity,
        'G_int': g_int,
        'normalized': g_int <= 1.2,
        'reduction_pct': (1.0 - g_int / G_0) * 100,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq P5: Panic Attack Probability (logistic model)
# ══════════════════════════════════════════════════════════════════════════

def panic_probability(NE: float = 2.5, GABA: float = 0.5, PFC: float = 0.4,
                      w_NE: float = 2.0, w_GABA: float = 3.0,
                      w_PFC: float = 2.5, theta: float = 1.0) -> dict:
    """P_panic = σ(w_NE × NE - w_GABA × GABA - w_PFC × PFC - θ).

    Logistic (sigmoid) model: panic probability as function of 3 key variables.
    High NE drives panic; GABA and PFC suppress it.
    σ(x) = 1 / (1 + exp(-x)).
    """
    z = w_NE * NE - w_GABA * GABA - w_PFC * PFC - theta
    p = 1.0 / (1.0 + math.exp(-z))
    return {
        'NE': NE, 'GABA': GABA, 'PFC': PFC,
        'logit': z,
        'p_panic': p,
        'risk_level': 'HIGH' if p > 0.7 else 'MODERATE' if p > 0.3 else 'LOW',
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq P6: GABA Deficit Recovery
# ══════════════════════════════════════════════════════════════════════════

def gaba_recovery(GABA_floor: float = 0.5, GABA_target: float = 1.0,
                  r: float = 0.04, n_sessions: int = 40) -> dict:
    """GABA(n) = GABA_floor + (GABA_target - GABA_floor) × (1 - (1-r)^n).

    Saturating recovery model. r: recovery rate per session.
    """
    trajectory = []
    for s in range(0, n_sessions + 1, max(1, n_sessions // 10)):
        g = GABA_floor + (GABA_target - GABA_floor) * (1.0 - (1.0 - r) ** s)
        trajectory.append({'session': s, 'GABA': g})
    g_final = GABA_floor + (GABA_target - GABA_floor) * (1.0 - (1.0 - r) ** n_sessions)
    recovery_pct = (g_final - GABA_floor) / (GABA_target - GABA_floor) * 100

    # Sessions to 80% recovery
    if r > 0:
        s_80 = math.log(0.2) / math.log(1.0 - r)
    else:
        s_80 = float('inf')

    return {
        'GABA_floor': GABA_floor,
        'GABA_final': g_final,
        'recovery_pct': recovery_pct,
        'sessions_to_80pct': s_80,
        'trajectory': trajectory,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq P7: Autonomic Storm Index
# ══════════════════════════════════════════════════════════════════════════

def autonomic_storm_index(NE: float = 2.5, GABA: float = 0.5,
                          Sensory: float = 2.5, Body: float = 3.0,
                          PFC: float = 0.4, Coherence: float = 0.4) -> dict:
    """ASI = NE² / GABA × (Sensory × Body) / (PFC × Coherence).

    Composite index of autonomic storm severity.
    ASI > 100: severe panic attack. ASI < 10: normal. ASI < 1: suppressed.
    """
    numerator = (NE ** 2) * Sensory * Body
    denominator = GABA * PFC * Coherence
    asi = numerator / max(denominator, 1e-9)
    return {
        'NE': NE, 'GABA': GABA, 'Sensory': Sensory,
        'Body': Body, 'PFC': PFC, 'Coherence': Coherence,
        'ASI': asi,
        'severity': 'SEVERE' if asi > 100 else 'MODERATE' if asi > 10 else 'MILD' if asi > 1 else 'SUPPRESSED',
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq P8: Acute Suppression Response Time
# ══════════════════════════════════════════════════════════════════════════

def acute_response_time(n_channels: int = 1024, base_detect_ms: float = 200.0,
                        compute_ms: float = 2.0, stim_onset_ms: float = 1.0) -> dict:
    """T_suppress = T_detect + T_compute + T_stim.

    T_detect = base / sqrt(N_channels) + latency.
    For panic onset detection via sympathetic biomarkers (HR, GSR, pupil).
    """
    t_detect = base_detect_ms / math.sqrt(n_channels) + 1.0  # +1ms system latency
    t_total = t_detect + compute_ms + stim_onset_ms
    # Panic attack ramp: NE surge reaches peak in ~5-10 seconds
    # Intervention must start within ~2 seconds for pre-peak suppression
    pre_peak = t_total < 2000.0  # < 2 seconds
    return {
        'n_channels': n_channels,
        't_detect_ms': t_detect,
        't_compute_ms': compute_ms,
        't_stim_onset_ms': stim_onset_ms,
        't_total_ms': t_total,
        't_total_s': t_total / 1000.0,
        'pre_peak_possible': pre_peak,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq P9: Fear Circuit Resonance (damped oscillator model)
# ══════════════════════════════════════════════════════════════════════════

def fear_circuit_resonance(f_amygdala_hz: float = 4.0, PFC_strength: float = 0.4,
                           amygdala_gain: float = 2.5, LC_gain: float = 2.5) -> dict:
    """ω_fear = 2π × f_amygdala.
    ζ = PFC / √(4 × Amyg_gain × LC_gain).

    Damped harmonic oscillator model of the fear circuit.
    ζ < 1: underdamped (oscillating panic-recovery cycles).
    ζ = 1: critically damped (fastest return to calm).
    ζ > 1: overdamped (slow but monotonic recovery).

    Treatment goal: increase ζ from <1 (oscillating panic) to ≥1 (stable).
    """
    omega = 2 * math.pi * f_amygdala_hz
    zeta = PFC_strength / math.sqrt(4 * amygdala_gain * LC_gain + 1e-9)
    # Natural decay time
    if zeta < 1:
        damped_freq = omega * math.sqrt(1 - zeta ** 2)
        decay_time_s = 1.0 / (zeta * omega) if zeta * omega > 0 else float('inf')
        oscillating = True
    else:
        damped_freq = 0.0
        decay_time_s = 1.0 / (zeta * omega) if zeta * omega > 0 else float('inf')
        oscillating = False

    return {
        'f_amygdala_hz': f_amygdala_hz,
        'omega': omega,
        'zeta': zeta,
        'PFC_strength': PFC_strength,
        'amygdala_gain': amygdala_gain,
        'LC_gain': LC_gain,
        'oscillating': oscillating,
        'damped_freq_hz': damped_freq / (2 * math.pi) if oscillating else 0.0,
        'decay_time_s': decay_time_s,
        'regime': 'underdamped' if zeta < 1 else 'critically_damped' if abs(zeta - 1) < 0.05 else 'overdamped',
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq P10: Long-term Panic Freedom
# ══════════════════════════════════════════════════════════════════════════

def panic_freedom(n_sessions: int = 40, mu: float = 0.02,
                  w_inh_trajectory: list | None = None,
                  gaba_trajectory: list | None = None) -> dict:
    """P_free(n) = 1 - exp(-μ × Σ_{s=1}^{n} W_inh(s) × GABA(s)).

    Cumulative probability of sustained panic freedom.
    Integrates both pathway restoration and neurochemical recovery.
    """
    # Generate default trajectories if not provided
    if w_inh_trajectory is None:
        w_inh_trajectory = []
        for s in range(n_sessions + 1):
            eff = s * 800 * 0.8
            w = 1.0 - (1.0 - 0.4) * (1.0 - 0.004) ** eff
            w_inh_trajectory.append(w)
    if gaba_trajectory is None:
        gaba_trajectory = []
        for s in range(n_sessions + 1):
            g = 0.5 + 0.5 * (1.0 - (1.0 - 0.04) ** s)
            gaba_trajectory.append(g)

    # Cumulative integral
    cumulative = 0.0
    trajectory = []
    for s in range(1, min(n_sessions + 1, len(w_inh_trajectory), len(gaba_trajectory))):
        cumulative += w_inh_trajectory[s] * gaba_trajectory[s]
        p_free = 1.0 - math.exp(-mu * cumulative)
        trajectory.append({'session': s, 'p_free': p_free,
                          'w_inh': w_inh_trajectory[s], 'gaba': gaba_trajectory[s]})

    p_final = trajectory[-1]['p_free'] if trajectory else 0.0
    return {
        'n_sessions': n_sessions,
        'mu': mu,
        'p_freedom': p_final,
        'trajectory': trajectory[::max(1, len(trajectory) // 10)],
    }


# ══════════════════════════════════════════════════════════════════════════
# N1 Coverage Proof: Panic Circuit Accessibility
# ══════════════════════════════════════════════════════════════════════════

PANIC_COVERAGE = {
    'Amygdala': {'sources': ['PFC', 'Insula', 'Temporal'], 'f_project_max': 0.35,
                 'refs': ['Ghashghaei & Barbas 2002', 'Augustine 1996']},
    'PAG': {'sources': ['PFC', 'Insula', 'ACC'], 'f_project_max': 0.25,
            'refs': ['Floyd 2000', 'An 1998']},
    'LC': {'sources': ['PFC', 'ACC', 'Insula'], 'f_project_max': 0.28,
           'refs': ['Jodo & Aston-Jones 1997', 'Aston-Jones & Cohen 2005']},
    'Insula': {'sources': ['DIRECT'], 'f_project_max': 1.0,
               'refs': ['N1 cortical electrode — direct access'],
               'note': 'Insula is cortical; N1 can place electrodes directly'},
}


def panic_circuit_coverage() -> dict:
    """Theorem 11: All 4 panic-critical structures accessible via N1.

    Proof by enumeration. Special case: Insula is cortical → direct access.
    """
    two_hub = {'PFC', 'ACC'}
    covered = {}
    for structure, info in PANIC_COVERAGE.items():
        if 'DIRECT' in info['sources']:
            accessible_via = ['DIRECT (cortical)']
            is_accessible = True
        else:
            accessible_via = [s for s in info['sources'] if s in two_hub]
            is_accessible = len(accessible_via) > 0
        covered[structure] = {
            'accessible': is_accessible,
            'via': accessible_via,
            'f_project_max': info['f_project_max'],
        }
    all_covered = all(v['accessible'] for v in covered.values())
    return {
        'theorem': 'Theorem 11 (Panic Circuit Coverage)',
        'hub_set': two_hub | {'Insula (direct)'},
        'structures_covered': sum(1 for v in covered.values() if v['accessible']),
        'structures_total': len(PANIC_COVERAGE),
        'all_covered': all_covered,
        'details': covered,
        'proof_type': 'pure_math',
    }


# ══════════════════════════════════════════════════════════════════════════
# Comprehensive Report
# ══════════════════════════════════════════════════════════════════════════

def print_report():
    """Print full Panic Disorder treatment calculator report."""
    print(f"\n{'=' * 70}")
    print(f"  Panic Disorder Treatment Calculator")
    print(f"  N1 Cortical Stimulation — Mathematical Framework")
    print(f"{'=' * 70}")

    # Profiles
    print(f"\n  === 12-Variable Profiles ===")
    print(f"  {'Var':<12} {'Pathology':>10} {'Treatment':>10} {'Maintenance':>12} {'Baseline':>10}")
    print(f"  {'-'*12} {'-'*10} {'-'*10} {'-'*12} {'-'*10}")
    for var in PANIC_PATHOLOGY:
        print(f"  {var:<12} {PANIC_PATHOLOGY[var]:>10.2f} {PANIC_TREATMENT[var]:>10.2f} "
              f"{PANIC_MAINTENANCE[var]:>12.2f} {PANIC_BASELINE[var]:>10.2f}")

    # Eq P1: LC-NE surge
    print(f"\n  === Eq P1: LC-NE Surge Model ===")
    print(f"  NE(t) = NE_base + A × exp(-t/τ) × (1 - suppression)")
    for supp in [0.0, 0.3, 0.6, 0.9]:
        r = lc_ne_surge(suppression=supp)
        print(f"  supp={supp:.1f}: NE_peak={r['NE_peak']:.2f}, "
              f"t<1.3×: {r['t_below_threshold_s']:.1f}s")

    # Eq P2: Amygdala-PFC restoration
    print(f"\n  === Eq P2: Amygdala-PFC Inhibitory Restoration ===")
    print(f"  W_inh(n) = W_ceil - (W_ceil-W_0) × (1-a+)^(n×η)")
    for sessions in [10, 20, 30, 40]:
        r = amygdala_pfc_restoration(n_sessions=sessions)
        print(f"  {sessions:>3} sessions: W={r['w_final']:.4f}, recovery={r['recovery_pct']:.1f}%")

    # Eq P3: Fear extinction
    print(f"\n  === Eq P3: Fear Extinction Rate ===")
    print(f"  F(n) = F_0 × exp(-k × n × W_inh)")
    r = fear_extinction()
    print(f"  F_0={r['F_0']:.1f} → F_final={r['F_final']:.4f}, "
          f"reduction={r['reduction_pct']:.1f}%")
    if r['sessions_to_50pct']:
        print(f"  50% reduction at session {r['sessions_to_50pct']}")

    # Eq P4: Interoceptive gain
    print(f"\n  === Eq P4: Interoceptive Gain Model ===")
    print(f"  G_int = G_0 × (1-β×I) / (1+γ×PFC)")
    for I_ins, pfc in [(0, 0.4), (0.5, 0.4), (0.5, 1.0), (1.0, 1.3)]:
        r = interoceptive_gain(I_insula=I_ins, PFC_activity=pfc)
        print(f"  I_ins={I_ins:.1f} PFC={pfc:.1f}: G={r['G_int']:.2f} "
              f"({'✓' if r['normalized'] else '✗'}) {r['reduction_pct']:.0f}% reduced")

    # Eq P5: Panic probability
    print(f"\n  === Eq P5: Panic Attack Probability ===")
    print(f"  P = σ(w_NE×NE - w_GABA×GABA - w_PFC×PFC - θ)")
    scenarios = [
        ('Pathological', 2.5, 0.5, 0.4),
        ('Partial Tx',   1.5, 0.8, 0.8),
        ('Near normal',  1.1, 1.0, 1.0),
        ('Treated',      0.8, 1.2, 1.2),
    ]
    for name, ne, gaba, pfc in scenarios:
        r = panic_probability(ne, gaba, pfc)
        print(f"  {name:<14} NE={ne:.1f} GABA={gaba:.1f} PFC={pfc:.1f}: "
              f"P={r['p_panic']:.4f} [{r['risk_level']}]")

    # Eq P6: GABA recovery
    print(f"\n  === Eq P6: GABA Deficit Recovery ===")
    print(f"  GABA(n) = floor + (target-floor) × (1-(1-r)^n)")
    r = gaba_recovery()
    print(f"  Floor={r['GABA_floor']:.1f} → Final={r['GABA_final']:.3f}, "
          f"recovery={r['recovery_pct']:.1f}%, 80%@{r['sessions_to_80pct']:.0f}sess")

    # Eq P7: ASI
    print(f"\n  === Eq P7: Autonomic Storm Index ===")
    print(f"  ASI = NE²/GABA × (Sensory×Body) / (PFC×Coherence)")
    scenarios_asi = [
        ('Panic attack',  2.5, 0.5, 2.5, 3.0, 0.4, 0.4),
        ('Partial Tx',    1.5, 0.8, 1.5, 1.5, 0.8, 0.8),
        ('Normal',        1.0, 1.0, 1.0, 1.0, 1.0, 1.0),
        ('Suppressed',    0.6, 1.3, 0.8, 0.8, 1.3, 1.2),
    ]
    for name, ne, gaba, sens, body, pfc, coh in scenarios_asi:
        r = autonomic_storm_index(ne, gaba, sens, body, pfc, coh)
        print(f"  {name:<14}: ASI={r['ASI']:>8.1f} [{r['severity']}]")

    # Eq P8: Response time
    print(f"\n  === Eq P8: Acute Suppression Response Time ===")
    print(f"  T = T_detect + T_compute + T_stim")
    for nch in [4, 64, 256, 1024]:
        r = acute_response_time(n_channels=nch)
        print(f"  {nch:>5}ch: T_detect={r['t_detect_ms']:.1f}ms, "
              f"T_total={r['t_total_ms']:.1f}ms ({r['t_total_s']:.3f}s) "
              f"{'PRE-PEAK' if r['pre_peak_possible'] else 'TOO SLOW'}")

    # Eq P9: Fear circuit resonance
    print(f"\n  === Eq P9: Fear Circuit Resonance ===")
    print(f"  ζ = PFC / √(4 × Amyg × LC)")
    for pfc, amyg, lc in [(0.4, 2.5, 2.5), (0.8, 1.5, 1.5), (1.0, 1.0, 1.0), (1.3, 0.8, 0.8)]:
        r = fear_circuit_resonance(PFC_strength=pfc, amygdala_gain=amyg, LC_gain=lc)
        print(f"  PFC={pfc:.1f} Amyg={amyg:.1f} LC={lc:.1f}: ζ={r['zeta']:.3f} "
              f"[{r['regime']}] decay={r['decay_time_s']:.2f}s")

    # Eq P10: Panic freedom
    print(f"\n  === Eq P10: Long-term Panic Freedom ===")
    print(f"  P_free = 1 - exp(-μ × Σ W_inh × GABA)")
    r = panic_freedom()
    print(f"  40 sessions: P_freedom={r['p_freedom']:.4f}")

    # Theorem 11: Coverage
    print(f"\n  === Theorem 11: Panic Circuit Coverage ===")
    c = panic_circuit_coverage()
    print(f"  Hub set: {c['hub_set']}")
    print(f"  Coverage: {c['structures_covered']}/{c['structures_total']}")
    for struct, info in c['details'].items():
        print(f"    {struct:<15} via {info['via']}, f_max={info['f_project_max']:.2f} "
              f"{'✓' if info['accessible'] else '✗'}")


if __name__ == '__main__':
    print_report()
