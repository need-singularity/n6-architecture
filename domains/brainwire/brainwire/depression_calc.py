from __future__ import annotations

"""Major Depressive Disorder (MDD) Treatment Calculator.

Mathematical framework for N1-based depression treatment via cortical stimulation.

Core Equations:
  Eq D1: Serotonin deficit model — 5HT(t) = 5HT_base × (1 - δ_5HT) + ΔS(t)
  Eq D2: STDP pathway potentiation — W(n) = W_ceil - (W_ceil - W_0) × (1 - a_plus)^(n×η)
  Eq D3: PFC→Raphe serotonin transfer — Δ5HT = C_pfc_raphe × I_stim × f_project
  Eq D4: PFC→VTA dopamine transfer — ΔDA = C_pfc_vta × I_stim × f_project
  Eq D5: Reward recovery index — RRI = (DA/DA_target) × (eCB/eCB_target) × (NAc_activity)
  Eq D6: Rumination suppression — R(t) = R_0 × exp(-k_suppress × t × I_tms)
  Eq D7: HPA axis normalization — Cortisol(n) = C_0 × (1 - η_hpa)^n + C_baseline
  Eq D8: Remission probability — P_remit(n) = 1 - exp(-λ × n × efficacy)
  Eq D9: Treatment resistance index — TRI = Σ_i(w_i × |V_i - V_target_i|) / 12
  Eq D10: Session-to-session carryover — V_i(s+1) = V_i(s) + α × (V_target - V_i(s)) × (1-decay)

Profiles (12-variable):
  Pathology: DA↓ eCB↓ 5HT↓↓ GABA↓ NE↑ Theta↑ Alpha↓ Gamma↓ PFC↑ Sensory↓ Body↓ Coherence↓
  Treatment: Overcorrect acutely to break depressive attractor state
  Maintenance: Gradual return to baseline with STDP consolidation
"""
import math


# ══════════════════════════════════════════════════════════════════════════
# MDD 12-Variable Profiles (3-phase)
# ══════════════════════════════════════════════════════════════════════════

MDD_PATHOLOGY = {
    'DA': 0.6, 'eCB': 0.7, '5HT': 0.5, 'GABA': 0.8, 'NE': 1.6,
    'Theta': 1.4, 'Alpha': 0.6, 'Gamma': 0.7,
    'PFC': 1.5, 'Sensory': 0.7, 'Body': 0.6, 'Coherence': 0.6,
}

MDD_TREATMENT = {
    'DA': 1.3, 'eCB': 1.2, '5HT': 1.3, 'GABA': 1.2, 'NE': 0.7,
    'Theta': 0.8, 'Alpha': 1.2, 'Gamma': 1.2,
    'PFC': 0.8, 'Sensory': 1.2, 'Body': 1.2, 'Coherence': 1.3,
}

MDD_MAINTENANCE = {
    'DA': 1.1, 'eCB': 1.05, '5HT': 1.1, 'GABA': 1.05, 'NE': 0.9,
    'Theta': 0.95, 'Alpha': 1.05, 'Gamma': 1.05,
    'PFC': 0.95, 'Sensory': 1.05, 'Body': 1.05, 'Coherence': 1.1,
}

MDD_BASELINE = {k: 1.0 for k in MDD_PATHOLOGY}


# ══════════════════════════════════════════════════════════════════════════
# Circuit Definitions (literature-based)
# ══════════════════════════════════════════════════════════════════════════

MDD_CIRCUITS = {
    'serotonin_deficit': {
        'pathway': 'PFC → Raphe → Forebrain',
        'pathology': 'DRN hypoactivity → 5HT deficit',
        'cortical_source': 'PFC',
        'subcortical_target': 'Raphe',
        'f_project': 0.30,  # Celada 2001
        'primary_variable': '5HT',
        'refs': ['Celada 2001', 'Coppen 1967'],
    },
    'reward_anhedonia': {
        'pathway': 'PFC → VTA → NAc',
        'pathology': 'DA tone deficit → reward insensitivity',
        'cortical_source': 'PFC',
        'subcortical_target': 'VTA',
        'f_project': 0.25,  # Carr & Sesack 2000
        'primary_variable': 'DA',
        'refs': ['Nestler 2006', 'Carr & Sesack 2000'],
    },
    'rumination': {
        'pathway': 'DLPFC ↔ ACC → Amygdala',
        'pathology': 'DMN hyperactivity, left PFC hypoactivity',
        'cortical_source': 'PFC',
        'subcortical_target': 'Amygdala',
        'f_project': 0.35,  # Ghashghaei & Barbas 2002
        'primary_variable': 'PFC',
        'refs': ['Mayberg 2005', 'Hamilton 2015'],
    },
    'hpa_axis': {
        'pathway': 'PFC → Hypothalamus → Adrenal',
        'pathology': 'HPA overactivity → chronic cortisol elevation',
        'cortical_source': 'PFC',
        'subcortical_target': 'Hypothalamus',
        'f_project': 0.20,  # Ongur & Price 2000
        'primary_variable': 'NE',
        'refs': ['Sapolsky 2000', 'Pariante 2008'],
    },
}


# ══════════════════════════════════════════════════════════════════════════
# Eq D1: Serotonin Deficit Model
# ══════════════════════════════════════════════════════════════════════════

def serotonin_deficit(base_5ht: float = 1.0, delta_deficit: float = 0.5,
                      stim_boost: float = 0.0) -> dict:
    """5HT(t) = 5HT_base × (1 - δ_5HT) + ΔS(t).

    Models serotonin level as baseline minus deficit plus stimulation recovery.
    Args:
        base_5ht: healthy baseline (1.0)
        delta_deficit: fractional deficit (0.5 = 50% reduction in MDD)
        stim_boost: stimulation-induced recovery (0 to ~0.8)
    """
    pathological = base_5ht * (1.0 - delta_deficit)
    current = pathological + stim_boost
    recovery_pct = (current / base_5ht) * 100
    return {
        'base_5ht': base_5ht,
        'deficit_fraction': delta_deficit,
        'pathological_level': pathological,
        'stim_boost': stim_boost,
        'current_level': min(current, base_5ht * 1.5),  # cap at 150%
        'recovery_pct': min(recovery_pct, 150.0),
        'normalized': current >= base_5ht * 0.95,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq D2: STDP Pathway Potentiation (inverse of epilepsy weakening)
# ══════════════════════════════════════════════════════════════════════════

def stdp_potentiation(n_sessions: int = 30, pulses_per_session: int = 1000,
                      eta: float = 0.8, a_plus: float = 0.005,
                      w0: float = 0.5, w_ceil: float = 1.0) -> dict:
    """W(n) = W_ceil - (W_ceil - W_0) × (1 - a_plus)^(n × η).

    STDP potentiation: pre-before-post timing strengthens synaptic weight.
    Mirror of epilepsy anti-kindling (Eq 11-15 from P-002) but in reverse.

    Args:
        n_sessions: treatment sessions
        pulses_per_session: STDP pairings per session
        eta: targeting efficiency (0.8 for N1)
        a_plus: potentiation rate per pulse
        w0: initial (pathological) weight
        w_ceil: maximum weight ceiling
    """
    effective = n_sessions * pulses_per_session * eta
    w_final = w_ceil - (w_ceil - w0) * (1.0 - a_plus) ** effective
    recovery_pct = ((w_final - w0) / (w_ceil - w0)) * 100

    # Trajectory
    trajectory = []
    for s in range(0, n_sessions + 1, max(1, n_sessions // 10)):
        eff = s * pulses_per_session * eta
        w = w_ceil - (w_ceil - w0) * (1.0 - a_plus) ** eff
        trajectory.append({'session': s, 'weight': w, 'recovery_pct': ((w - w0) / (w_ceil - w0)) * 100})

    return {
        'n_sessions': n_sessions,
        'total_pulses': n_sessions * pulses_per_session,
        'effective_pulses': effective,
        'w_initial': w0,
        'w_final': w_final,
        'w_ceiling': w_ceil,
        'recovery_pct': recovery_pct,
        'trajectory': trajectory,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq D3: PFC→Raphe Serotonin Transfer Function
# ══════════════════════════════════════════════════════════════════════════

def pfc_raphe_transfer(I_mA: float = 2.0, f_project: float = 0.30,
                       c_cortical: float = 0.20, n1_boost: float = 3.0) -> dict:
    """Δ5HT = C_pfc_raphe × I_stim × f_project × N1_boost.

    Models serotonin increase from PFC stimulation via raphe projection.
    """
    c_effective = c_cortical * n1_boost * f_project
    delta_5ht = c_effective * I_mA
    return {
        'I_mA': I_mA,
        'f_project': f_project,
        'c_cortical': c_cortical,
        'n1_boost': n1_boost,
        'c_effective': c_effective,
        'delta_5ht': delta_5ht,
        'delta_5ht_pct': delta_5ht * 100,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq D4: PFC→VTA Dopamine Transfer Function
# ══════════════════════════════════════════════════════════════════════════

def pfc_vta_transfer(I_mA: float = 2.0, f_project: float = 0.25,
                     c_cortical: float = 0.20, n1_boost: float = 3.0) -> dict:
    """ΔDA = C_pfc_vta × I_stim × f_project × N1_boost.

    Models dopamine increase from DLPFC stimulation via VTA projection.
    """
    c_effective = c_cortical * n1_boost * f_project
    delta_da = c_effective * I_mA
    return {
        'I_mA': I_mA,
        'f_project': f_project,
        'c_cortical': c_cortical,
        'n1_boost': n1_boost,
        'c_effective': c_effective,
        'delta_da': delta_da,
        'delta_da_pct': delta_da * 100,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq D5: Reward Recovery Index
# ══════════════════════════════════════════════════════════════════════════

def reward_recovery_index(da_current: float = 0.6, da_target: float = 1.0,
                          ecb_current: float = 0.7, ecb_target: float = 1.0,
                          nac_activity: float = 0.5) -> dict:
    """RRI = (DA/DA_target) × (eCB/eCB_target) × NAc_activity.

    Composite index of reward circuit recovery. RRI=1.0 = full recovery.
    """
    da_ratio = min(da_current / da_target, 1.5)
    ecb_ratio = min(ecb_current / ecb_target, 1.5)
    rri = da_ratio * ecb_ratio * nac_activity
    return {
        'da_ratio': da_ratio,
        'ecb_ratio': ecb_ratio,
        'nac_activity': nac_activity,
        'rri': rri,
        'recovered': rri >= 0.8,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq D6: Rumination Suppression (DMN → TMS 1Hz)
# ══════════════════════════════════════════════════════════════════════════

def rumination_suppression(R_0: float = 1.5, k_suppress: float = 0.15,
                           t_minutes: float = 20.0, I_tms_normalized: float = 1.0) -> dict:
    """R(t) = R_0 × exp(-k_suppress × t × I_tms).

    Models exponential decay of DMN/PFC overactivity during rTMS session.
    R_0: initial rumination level (1.5 = 50% above normal for MDD).
    """
    R_t = R_0 * math.exp(-k_suppress * t_minutes * I_tms_normalized)
    suppression_pct = (1.0 - R_t / R_0) * 100
    # Time to reach normal (R=1.0)
    if R_0 > 1.0 and k_suppress > 0 and I_tms_normalized > 0:
        t_normalize = math.log(R_0) / (k_suppress * I_tms_normalized)
    else:
        t_normalize = float('inf')
    return {
        'R_0': R_0,
        'R_final': R_t,
        'suppression_pct': suppression_pct,
        't_normalize_min': t_normalize,
        'normalized': R_t <= 1.05,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq D7: HPA Axis Normalization (cortisol reduction over sessions)
# ══════════════════════════════════════════════════════════════════════════

def hpa_normalization(C_0: float = 25.0, C_baseline: float = 15.0,
                      eta_hpa: float = 0.05, n_sessions: int = 30) -> dict:
    """Cortisol(n) = C_baseline + (C_0 - C_baseline) × (1 - η_hpa)^n.

    Models gradual cortisol normalization over treatment sessions.
    C_0: pathological cortisol (μg/dL morning), C_baseline: healthy level.
    """
    trajectory = []
    for s in range(0, n_sessions + 1, max(1, n_sessions // 10)):
        c = C_baseline + (C_0 - C_baseline) * (1.0 - eta_hpa) ** s
        trajectory.append({'session': s, 'cortisol': c})
    c_final = C_baseline + (C_0 - C_baseline) * (1.0 - eta_hpa) ** n_sessions
    reduction_pct = ((C_0 - c_final) / (C_0 - C_baseline)) * 100
    return {
        'C_0': C_0,
        'C_baseline': C_baseline,
        'C_final': c_final,
        'reduction_pct': reduction_pct,
        'normalized': c_final <= C_baseline * 1.1,
        'trajectory': trajectory,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq D8: Remission Probability Model
# ══════════════════════════════════════════════════════════════════════════

def remission_probability(n_sessions: int = 30, lam: float = 0.03,
                          efficacy: float = 1.0) -> dict:
    """P_remit(n) = 1 - exp(-λ × n × efficacy).

    Exponential CDF for remission probability.
    λ: base remission rate per session, efficacy: treatment multiplier.
    """
    trajectory = []
    for s in range(0, n_sessions + 1, max(1, n_sessions // 10)):
        p = 1.0 - math.exp(-lam * s * efficacy)
        trajectory.append({'session': s, 'p_remit': p})
    p_final = 1.0 - math.exp(-lam * n_sessions * efficacy)

    # Sessions to 50% and 80% remission probability
    s_50 = -math.log(0.5) / (lam * efficacy) if lam * efficacy > 0 else float('inf')
    s_80 = -math.log(0.2) / (lam * efficacy) if lam * efficacy > 0 else float('inf')

    return {
        'n_sessions': n_sessions,
        'lambda': lam,
        'efficacy': efficacy,
        'p_remission': p_final,
        'sessions_to_50pct': s_50,
        'sessions_to_80pct': s_80,
        'trajectory': trajectory,
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq D9: Treatment Resistance Index
# ══════════════════════════════════════════════════════════════════════════

def treatment_resistance_index(current: dict, target: dict,
                                weights: dict | None = None) -> dict:
    """TRI = Σ_i(w_i × |V_i - V_target_i|) / N.

    Weighted mean absolute deviation from treatment target.
    TRI=0: perfect response. TRI>0.5: treatment resistant.
    """
    if weights is None:
        weights = {k: 1.0 for k in current}
    n = len(current)
    deviations = {}
    weighted_sum = 0.0
    for var in current:
        dev = abs(current[var] - target[var])
        w = weights.get(var, 1.0)
        deviations[var] = dev
        weighted_sum += w * dev
    tri = weighted_sum / (n * max(sum(weights.values()) / n, 1e-9))
    return {
        'tri': tri,
        'deviations': deviations,
        'resistant': tri > 0.5,
        'worst_vars': sorted(deviations, key=deviations.get, reverse=True)[:3],
    }


# ══════════════════════════════════════════════════════════════════════════
# Eq D10: Session-to-Session Carryover
# ══════════════════════════════════════════════════════════════════════════

def session_carryover(current: dict, target: dict, alpha: float = 0.15,
                      decay: float = 0.05, n_sessions: int = 30) -> dict:
    """V_i(s+1) = V_i(s) + α × (V_target - V_i(s)) × (1 - decay).

    Models how each session moves variables toward target with inter-session decay.
    α: learning rate per session. decay: inter-session regression.
    """
    state = dict(current)
    trajectory = [dict(state)]
    for s in range(n_sessions):
        new_state = {}
        for var in state:
            delta = alpha * (target[var] - state[var]) * (1.0 - decay)
            new_state[var] = state[var] + delta
        state = new_state
        trajectory.append(dict(state))

    # Final distance from target
    final_dist = sum(abs(state[v] - target[v]) for v in state) / len(state)
    return {
        'n_sessions': n_sessions,
        'alpha': alpha,
        'decay': decay,
        'final_state': state,
        'mean_distance': final_dist,
        'converged': final_dist < 0.05,
        'trajectory_endpoints': [trajectory[0], trajectory[n_sessions // 2], trajectory[-1]],
    }


# ══════════════════════════════════════════════════════════════════════════
# N1 Coverage Proof: MDD Circuit Accessibility
# ══════════════════════════════════════════════════════════════════════════

MDD_COVERAGE = {
    'Raphe': {'sources': ['PFC', 'Insula', 'ACC'], 'f_project_max': 0.30,
              'refs': ['Celada 2001', 'Peyron 1998']},
    'VTA': {'sources': ['DLPFC', 'OFC', 'ACC'], 'f_project_max': 0.25,
            'refs': ['Carr & Sesack 2000', 'Frankle 2006']},
    'Amygdala': {'sources': ['PFC', 'Insula', 'Temporal'], 'f_project_max': 0.35,
                 'refs': ['Ghashghaei & Barbas 2002', 'Augustine 1996']},
    'Hypothalamus': {'sources': ['Insula', 'PFC', 'ACC'], 'f_project_max': 0.20,
                     'refs': ['Ongur & Price 2000', 'Cechetto & Saper 1987']},
}


def mdd_circuit_coverage() -> dict:
    """Theorem 9: All 4 MDD-critical structures accessible via {PFC, ACC}.

    Proof by enumeration from Theorem 6 projection table.
    """
    two_hub = {'PFC', 'ACC'}
    covered = {}
    for structure, info in MDD_COVERAGE.items():
        accessible_via = [s for s in info['sources'] if s in two_hub or s == 'DLPFC']
        covered[structure] = {
            'accessible': len(accessible_via) > 0,
            'via': accessible_via,
            'f_project_max': info['f_project_max'],
        }
    all_covered = all(v['accessible'] for v in covered.values())
    coverage_count = sum(1 for v in covered.values() if v['accessible'])
    return {
        'theorem': 'Theorem 9 (MDD Circuit Coverage)',
        'hub_set': two_hub,
        'structures_covered': coverage_count,
        'structures_total': len(MDD_COVERAGE),
        'all_covered': all_covered,
        'details': covered,
        'proof_type': 'pure_math',  # Golden Zone independent
    }


# ══════════════════════════════════════════════════════════════════════════
# Comprehensive Report
# ══════════════════════════════════════════════════════════════════════════

def print_report():
    """Print full MDD treatment calculator report."""
    print(f"\n{'=' * 70}")
    print(f"  Major Depressive Disorder (MDD) Treatment Calculator")
    print(f"  N1 Cortical Stimulation — Mathematical Framework")
    print(f"{'=' * 70}")

    # Profiles
    print(f"\n  === 12-Variable Profiles ===")
    print(f"  {'Var':<12} {'Pathology':>10} {'Treatment':>10} {'Maintenance':>12} {'Baseline':>10}")
    print(f"  {'-'*12} {'-'*10} {'-'*10} {'-'*12} {'-'*10}")
    for var in MDD_PATHOLOGY:
        print(f"  {var:<12} {MDD_PATHOLOGY[var]:>10.2f} {MDD_TREATMENT[var]:>10.2f} "
              f"{MDD_MAINTENANCE[var]:>12.2f} {MDD_BASELINE[var]:>10.2f}")

    # Eq D1: Serotonin deficit
    print(f"\n  === Eq D1: Serotonin Deficit Model ===")
    print(f"  5HT(t) = 5HT_base × (1 - δ) + ΔS(t)")
    for boost in [0.0, 0.2, 0.4, 0.6]:
        r = serotonin_deficit(stim_boost=boost)
        print(f"  ΔS={boost:.1f}: 5HT={r['current_level']:.2f} ({r['recovery_pct']:.0f}%) "
              f"{'✓' if r['normalized'] else '✗'}")

    # Eq D2: STDP potentiation
    print(f"\n  === Eq D2: STDP Pathway Potentiation ===")
    print(f"  W(n) = W_ceil - (W_ceil - W_0) × (1 - a+)^(n×η)")
    for sessions in [10, 20, 30, 50]:
        r = stdp_potentiation(n_sessions=sessions)
        print(f"  {sessions:>3} sessions: W={r['w_final']:.4f}, recovery={r['recovery_pct']:.1f}%")

    # Eq D3-D4: Transfer functions
    print(f"\n  === Eq D3-D4: Cortical Transfer Functions ===")
    r3 = pfc_raphe_transfer()
    r4 = pfc_vta_transfer()
    print(f"  PFC→Raphe: C_eff={r3['c_effective']:.3f}, Δ5HT={r3['delta_5ht']:.3f} ({r3['delta_5ht_pct']:.1f}%)")
    print(f"  PFC→VTA:   C_eff={r4['c_effective']:.3f}, ΔDA ={r4['delta_da']:.3f} ({r4['delta_da_pct']:.1f}%)")

    # Eq D5: Reward recovery
    print(f"\n  === Eq D5: Reward Recovery Index ===")
    print(f"  RRI = (DA/target) × (eCB/target) × NAc")
    for da, ecb, nac in [(0.6, 0.7, 0.5), (0.8, 0.85, 0.7), (1.0, 1.0, 0.9), (1.2, 1.1, 1.0)]:
        r = reward_recovery_index(da, 1.0, ecb, 1.0, nac)
        print(f"  DA={da:.1f} eCB={ecb:.2f} NAc={nac:.1f}: RRI={r['rri']:.3f} {'✓' if r['recovered'] else '✗'}")

    # Eq D6: Rumination
    print(f"\n  === Eq D6: Rumination Suppression ===")
    print(f"  R(t) = R_0 × exp(-k × t × I)")
    r = rumination_suppression()
    print(f"  R_0={r['R_0']:.1f} → R_final={r['R_final']:.3f}, suppression={r['suppression_pct']:.1f}%")
    print(f"  Time to normalize: {r['t_normalize_min']:.1f} min")

    # Eq D7: HPA
    print(f"\n  === Eq D7: HPA Axis Normalization ===")
    print(f"  Cortisol(n) = C_base + (C_0 - C_base) × (1-η)^n")
    r = hpa_normalization()
    print(f"  C_0={r['C_0']:.0f} → C_final={r['C_final']:.1f} μg/dL, reduction={r['reduction_pct']:.1f}%")
    print(f"  Normalized: {r['normalized']}")

    # Eq D8: Remission
    print(f"\n  === Eq D8: Remission Probability ===")
    print(f"  P(n) = 1 - exp(-λ × n × efficacy)")
    for eff in [0.5, 1.0, 1.5, 2.0]:
        r = remission_probability(efficacy=eff)
        print(f"  efficacy={eff:.1f}: P(30)={r['p_remission']:.3f}, "
              f"50%@{r['sessions_to_50pct']:.0f}sess, 80%@{r['sessions_to_80pct']:.0f}sess")

    # Eq D9: TRI
    print(f"\n  === Eq D9: Treatment Resistance Index ===")
    r = treatment_resistance_index(MDD_PATHOLOGY, MDD_BASELINE)
    print(f"  TRI (pathology→baseline): {r['tri']:.3f}, resistant={r['resistant']}")
    print(f"  Worst variables: {', '.join(r['worst_vars'])}")

    # Eq D10: Carryover
    print(f"\n  === Eq D10: Session Carryover ===")
    r = session_carryover(MDD_PATHOLOGY, MDD_BASELINE)
    print(f"  30 sessions: mean_dist={r['mean_distance']:.4f}, converged={r['converged']}")

    # Theorem 9: Coverage
    print(f"\n  === Theorem 9: MDD Circuit Coverage ===")
    c = mdd_circuit_coverage()
    print(f"  Hub set: {c['hub_set']}")
    print(f"  Coverage: {c['structures_covered']}/{c['structures_total']}")
    for struct, info in c['details'].items():
        print(f"    {struct:<15} via {info['via']}, f_max={info['f_project_max']:.2f} {'✓' if info['accessible'] else '✗'}")


if __name__ == '__main__':
    print_report()
