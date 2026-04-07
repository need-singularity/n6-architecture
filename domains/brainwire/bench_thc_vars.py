#!/usr/bin/env python3
"""BrainWire THC Variable Benchmark — 뇌자극 전용 12변수 하드웨어 재현 검증.

뇌자극 하드웨어만 사용 (전기자극 + 자기자극 + 신경 엔트레인먼트):
  - tDCS, tACS, TMS (뇌 직접 자극)
  - TENS, taVNS (말초신경 전기자극)
  - LED/Audio/Vibro 40Hz (감각경로 뇌 엔트레인먼트)
  - Stochastic resonance noise (신경 확률공명)

제외: 온열패드, 가중담요, 음악, 일반진동, 적외선 — 뇌자극 아님

PureField Tension Framework:
  T_total = √(T_chem² + T_wave² + T_state²)
  T_THC   = 4.280 (기준)

Variables (12-dimensional tension vector):
  V1:  DA (dopamine)          target 2.5×
  V2:  eCB (endocannabinoid)  target 3.0×
  V3:  5HT (serotonin)        target 1.5×
  V4:  GABA                    target 1.8×
  V5:  NE↓ (norepinephrine)   target 0.4×
  V6:  Theta↑↑ (4-8Hz)        target 2.5×
  V7:  Alpha↓ (8-12Hz)        target 0.5×
  V8:  Gamma↑ (30-100Hz)      target 1.8×
  V9:  PFC↓                   target 0.5×
  V10: Sensory↑                target 2.0×
  V11: Body↑                  target 2.5×
  V12: Coherence↑              target 2.0×
"""

import math
import json
import argparse
from dataclasses import dataclass

# ═══════════════════════════════════════════════════════════
# THC Target Vector
# ═══════════════════════════════════════════════════════════

THC_TARGET_25 = {
    'DA': 2.5, 'eCB': 3.0, '5HT': 1.5, 'GABA': 1.8, 'NE': 0.4,
    'Theta': 2.5, 'Alpha': 0.5, 'Gamma': 1.8, 'PFC': 0.5,
    'Sensory': 2.0, 'Body': 2.5, 'Coherence': 2.0,
}

# ═══════════════════════════════════════════════════════════
# THC Concentration Model
# ═══════════════════════════════════════════════════════════
#
# THC 농도별 효과는 비선형 (sigmoidal dose-response)
# Hill equation: E = Emax × C^n / (EC50^n + C^n)
# 간소화: 25% THC = 1.0× (기준), 다른 농도는 스케일 팩터
#
# 약동학 (pharmacokinetics by route):
#   흡연: onset 1-5min, peak 15-30min, duration 1-3hr
#   경구: onset 30-90min, peak 2-4hr, duration 4-8hr
#   BrainWire: onset 5-15min, duration = 전원 ON 시간 (안전 한계 내)

THC_LEVELS = {
    # name: (scale, 등가 THC%, onset_min, peak_min, duration_min, description)
    'micro':     (0.25, 1,  1, 10,  45, 'micro-dose — 가벼운 기분전환'),
    'light':     (0.50, 5,  3, 15,  60, '가벼운 하이 — 사교/창작 적합'),
    'medium':    (0.75, 15, 5, 20,  90, '중간 하이 — 일반적 레크리에이션'),
    'strong':    (1.00, 25, 5, 25, 120, '강한 하이 — 숙련자 수준'),
    'intense':   (1.15, 30, 5, 30, 150, '강렬 — 고농축/dabbing 수준'),
}

# 하드웨어 안전 유지시간 (분)
HW_DURATION = {
    'tDCS':  20,   # 20min on, 5min off, repeat
    'tACS':  30,   # 30min continuous
    'TMS':   20,   # pulse count limited, ~20min
    'TENS':  60,   # 60min continuous
    'taVNS': 30,   # 30min continuous
    'LED':   45,   # no hard limit, comfort
    'audio': 120,  # no safety limit
    'vibro': 60,   # motor heating limit
}


def scale_target(level: str) -> dict:
    """THC 농도별 목표 벡터 생성.

    baseline=1.0에서 target까지의 편차를 scale만큼 조정.
    예: DA target=2.5, scale=0.5 → 1.0 + (2.5-1.0)×0.5 = 1.75
        NE target=0.4, scale=0.5 → 1.0 + (0.4-1.0)×0.5 = 0.70
    """
    scale = THC_LEVELS[level][0]
    target = {}
    for k, v in THC_TARGET_25.items():
        target[k] = 1.0 + (v - 1.0) * scale
    return target


def compute_session(hw_config, level: str) -> dict:
    """세션 계산: 달성률 + 유지시간 + 안전 스케줄."""
    target = scale_target(level)
    scale, thc_pct, onset, peak, duration, desc = THC_LEVELS[level]

    # 안전 유지시간 = 가장 짧은 필수 장비의 한계
    # tDCS/tACS/TMS는 휴식 후 재개 가능 → 실효 유지시간 계산
    # tDCS: 20min on + 5min off → 80% 가동률 → 실효 48min/hr
    # 패턴: [on 20]-[off 5]-[on 20]-[off 5]-[on 20] = 60min 중 60min 가동 (3cycle)
    max_continuous = min(HW_DURATION.values())  # 20min (tDCS or TMS)
    # 하지만 cycling으로 실질적으로 무한 유지 가능 (강도 80%로)
    effective_duration = duration  # THC 등가 시간과 동일하게 유지 가능
    cycles_needed = math.ceil(effective_duration / max_continuous)

    return {
        'level': level,
        'thc_pct': thc_pct,
        'scale': scale,
        'target': target,
        'onset_min': onset,
        'peak_min': peak,
        'duration_min': effective_duration,
        'cycles': cycles_needed,
        'cycle_on': max_continuous,
        'cycle_off': 5,
        'desc': desc,
    }


# backward compat
THC_TARGET = THC_TARGET_25


# ═══════════════════════════════════════════════════════════
# Transfer Function Coefficients (brain stimulation only)
# ═══════════════════════════════════════════════════════════

# V1: DA = 1 + α1·tDCS(F3) + α2·taVNS + α3·TMS(10Hz)
#     tDCS F3 → DLPFC → VTA projection (Fonteneau 2018)
#     taVNS → NTS → VTA → DA release (Frangos 2015)
#     TMS 10Hz → DLPFC → striatal DA release (Strafella 2001)
COEFF_DA = {'tDCS': 0.25, 'VNS': 0.80, 'TMS_10Hz': 0.80}

# V2: eCB = 1 + β1·TENS + β2·taVNS + β3·tDCS + β4·tACS(θ) + β5·TMS(θ)
#     TENS 2Hz → peripheral eCB release (Resende 2004)
#     taVNS → vagal-eCB axis upregulation (Meregnani 2011)
#     tDCS → cortical eCB tone modulation (Yavari 2018)
#     tACS theta → hippocampal eCB plasticity (Bhatt 2020)
#     TMS iTBS → eCB signaling modulation (Centonze 2007)
COEFF_ECB = {'TENS_low': 0.80, 'VNS': 0.60, 'tDCS': 0.20,
             'tACS_theta': 0.15, 'TMS_theta': 0.20}

# V3: 5HT = 1 + γ1·taVNS + γ2·tDCS
#     taVNS → NTS → raphe → 5HT release (Frangos 2015)
#     tDCS → cortical 5HT modulation (Nitsche 2009)
COEFF_5HT = {'VNS': 1.20, 'tDCS': 0.15}

# V4: GABA = 1 + δ1·tDCS + δ2·α_ent + δ3·TMS(θ) + δ4·tACS(α)
#     tDCS → cortical GABA↑ (MRS-verified, Stagg 2009)
#     Alpha audio entrainment → alpha oscillation → GABA proxy
#     TMS iTBS → cortical GABA increase (Stagg 2009, MRS)
#     tACS 10Hz → alpha entrainment → GABAergic enhancement (Wach 2013)
COEFF_GABA = {'tDCS': 0.20, 'alpha_ent': 0.30, 'TMS_theta': 0.25,
              'tACS_10Hz': 0.15}

# V5: NE = 1 - ε1·taVNS
#     taVNS → NTS → LC inhibition → NE↓ (Dietrich 2008)
COEFF_NE = {'VNS': 1.50}

# V6: Theta = 1 + ζ1·TMS(6Hz) + ζ2·binaural(6Hz) + ζ3·tACS(6Hz)
#     TMS 6Hz → hippocampal theta entrainment
#     Binaural 6Hz → auditory cortex theta
#     tACS 6Hz → direct theta oscillation drive
COEFF_THETA = {'TMS_theta': 0.80, 'binaural': 0.40, 'tACS': 0.35}

# V7: Alpha = 1 - η1·tDCS(cathode) - η2·TMS(1Hz)
#     tDCS cathode Fz → frontal alpha suppression
#     TMS 1Hz → inhibitory → alpha desynchronization
COEFF_ALPHA = {'tDCS_cathode': 0.20, 'TMS_1Hz': 0.25}

# V8: Gamma = 1 + θ1·LED(40Hz) + θ2·Audio(40Hz) + θ3·Vibro(40Hz) + θ4·tACS(40Hz) + θ5·TMS(40Hz)
#     LED 40Hz → visual cortex gamma entrainment (Iaccarino 2016)
#     Audio 40Hz → auditory cortex gamma entrainment (Martorell 2019)
#     Vibro 40Hz → somatosensory gamma entrainment
#     tACS 40Hz → direct cortical gamma entrainment (Helfrich 2014)
#     TMS 40Hz → gamma burst stimulation (Barr 2009)
COEFF_GAMMA = {'LED_40Hz': 0.30, 'audio_40Hz': 0.25, 'vibro_40Hz': 0.20,
               'tACS_40Hz': 0.15, 'TMS_40Hz': 0.10}

# V9: PFC = 1 - ι1·tDCS(cathode) - ι2·TMS(1Hz)
#     tDCS cathode F4 → right DLPFC inhibition
#     TMS 1Hz → PFC suppression
COEFF_PFC = {'tDCS_cathode': 0.20, 'TMS_1Hz': 0.25}

# V10: Sensory = 1 + κ1·tDCS(V1) + κ2·noise + κ3·LED(40Hz) + κ4·TENS + κ5·tACS(40Hz)
#      tDCS V1 anode → visual cortex excitability↑ (Antal 2004)
#      Stochastic resonance → subthreshold signal amplification (Collins 1996)
#      LED 40Hz → sensory cortex gamma → cross-modal enhancement
#      TENS → peripheral afferent → sensory gain (stochastic resonance)
#      tACS 40Hz → sensory cortex gamma entrainment
COEFF_SENSORY = {'tDCS_anode': 0.15, 'noise': 0.40, 'LED_40Hz': 0.20,
                 'TENS': 0.15, 'tACS_40Hz': 0.10}

# V11: Body = 1 + λ1·TENS(low) + λ2·TENS(high) + λ3·tDCS(S1) + λ4·Vibro(40Hz)
#      TENS 2-4Hz → endorphin + somatic activation (Sluka 2003)
#      TENS 50-100Hz → gate control → immediate body awareness
#      tDCS S1 anode → somatosensory cortex excitability↑ (Ragert 2008)
#      Vibro 40Hz → somatosensory entrainment → body schema
COEFF_BODY = {'TENS_low': 0.80, 'TENS_high': 0.30, 'tDCS_S1': 0.20,
              'vibro_40Hz': 0.15}

# V12: Coherence = 1 + μ1·γ_avg + μ2·TMS(40Hz) + μ3·sync + μ4·tACS(40Hz)
#      Multi-modal 40Hz → cross-cortical gamma coherence
#      TMS 40Hz paired → forced inter-regional coherence
#      Synchronized tri-modal → phase-locked coherence
#      tACS 40Hz → long-range phase synchronization (Polanía 2012)
COEFF_COHERENCE = {'gamma_40Hz': 0.30, 'paired_TMS': 0.40, 'sync_stim': 0.20,
                   'tACS_40Hz': 0.15}


# ═══════════════════════════════════════════════════════════
# Hardware Configuration (brain stimulation only)
# ═══════════════════════════════════════════════════════════

@dataclass
class HardwareConfig:
    """뇌자극 하드웨어 파라미터."""
    name: str = "default"
    cost: float = 0.0
    tier: str = "custom"

    # tDCS (multi-site electrode montage)
    tDCS_anode_mA: float = 0.0       # F3 anode — DLPFC (0-2 mA)
    tDCS_cathode_Fz_mA: float = 0.0  # Fz cathode (0-2 mA)
    tDCS_cathode_F4_mA: float = 0.0  # F4 cathode — right DLPFC (0-2 mA)
    tDCS_anode_V1_mA: float = 0.0    # Oz anode — visual cortex (0-2 mA)
    tDCS_anode_S1_mA: float = 0.0    # C3/C4 anode — somatosensory cortex (0-2 mA)

    # taVNS (transcutaneous auricular vagus nerve stimulation)
    VNS_mA: float = 0.0  # ear-clip current (0-0.5 mA)

    # TENS (transcutaneous electrical nerve stimulation)
    TENS_low_intensity: float = 0.0   # 2-4Hz endorphin mode (0-1)
    TENS_high_intensity: float = 0.0  # 50-100Hz gate control mode (0-1)

    # TMS (transcranial magnetic stimulation)
    TMS_theta_strength: float = 0.0   # 6Hz theta burst (0-1)
    TMS_1Hz_strength: float = 0.0     # 1Hz inhibitory rTMS (0-1)
    TMS_10Hz_strength: float = 0.0    # 10Hz excitatory rTMS (0-1)
    TMS_40Hz_strength: float = 0.0    # 40Hz gamma burst (0-1)

    # tACS (transcranial alternating current stimulation)
    tACS_6Hz_mA: float = 0.0    # theta tACS (0-2 mA)
    tACS_10Hz_mA: float = 0.0   # alpha tACS (0-2 mA)
    tACS_40Hz_mA: float = 0.0   # gamma tACS (0-2 mA)

    # Neural entrainment (sensory pathway → brain)
    LED_40Hz: float = 0.0       # 40Hz visual flicker → visual cortex (0-1)
    audio_40Hz: float = 0.0     # 40Hz click train → auditory cortex (0-1)
    binaural_6Hz: float = 0.0   # 6Hz binaural beat → theta entrainment (0-1)
    vibro_40Hz: float = 0.0     # 40Hz vibrotactile → somatosensory cortex (0-1)
    noise_level: float = 0.0    # stochastic resonance → sensory amplification (0-1)
    alpha_entrainment: float = 0.0  # alpha audio entrainment → GABA (0-1)


def compute_variables(hw: HardwareConfig) -> dict:
    """12변수 계산 — 뇌자극 전용 전달 함수."""
    v = {}

    # V1: DA — tDCS(F3) + taVNS + TMS(10Hz)
    v['DA'] = 1.0 + (COEFF_DA['tDCS'] * hw.tDCS_anode_mA +
                      COEFF_DA['VNS'] * hw.VNS_mA +
                      COEFF_DA['TMS_10Hz'] * hw.TMS_10Hz_strength)

    # V2: eCB — TENS + taVNS + tDCS + tACS(θ) + TMS(θ)
    v['eCB'] = 1.0 + (COEFF_ECB['TENS_low'] * hw.TENS_low_intensity +
                       COEFF_ECB['VNS'] * hw.VNS_mA +
                       COEFF_ECB['tDCS'] * hw.tDCS_anode_mA +
                       COEFF_ECB['tACS_theta'] * hw.tACS_6Hz_mA +
                       COEFF_ECB['TMS_theta'] * hw.TMS_theta_strength)

    # V3: 5HT — taVNS + tDCS
    v['5HT'] = 1.0 + (COEFF_5HT['VNS'] * hw.VNS_mA +
                       COEFF_5HT['tDCS'] * hw.tDCS_anode_mA)

    # V4: GABA — tDCS + alpha entrainment + TMS(θ) + tACS(α)
    v['GABA'] = 1.0 + (COEFF_GABA['tDCS'] * hw.tDCS_anode_mA +
                        COEFF_GABA['alpha_ent'] * hw.alpha_entrainment +
                        COEFF_GABA['TMS_theta'] * hw.TMS_theta_strength +
                        COEFF_GABA['tACS_10Hz'] * hw.tACS_10Hz_mA)

    # V5: NE↓ — taVNS
    v['NE'] = max(0.01, 1.0 - COEFF_NE['VNS'] * hw.VNS_mA)

    # V6: Theta — TMS(6Hz) + binaural(6Hz) + tACS(6Hz)
    v['Theta'] = 1.0 + (COEFF_THETA['TMS_theta'] * hw.TMS_theta_strength +
                         COEFF_THETA['binaural'] * hw.binaural_6Hz +
                         COEFF_THETA['tACS'] * hw.tACS_6Hz_mA)

    # V7: Alpha↓ — tDCS cathode + TMS(1Hz)
    v['Alpha'] = max(0.01, 1.0 - (COEFF_ALPHA['tDCS_cathode'] * hw.tDCS_cathode_Fz_mA +
                                    COEFF_ALPHA['TMS_1Hz'] * hw.TMS_1Hz_strength))

    # V8: Gamma — LED(40Hz) + Audio(40Hz) + Vibro(40Hz) + tACS(40Hz) + TMS(40Hz)
    v['Gamma'] = 1.0 + (COEFF_GAMMA['LED_40Hz'] * hw.LED_40Hz +
                         COEFF_GAMMA['audio_40Hz'] * hw.audio_40Hz +
                         COEFF_GAMMA['vibro_40Hz'] * hw.vibro_40Hz +
                         COEFF_GAMMA['tACS_40Hz'] * hw.tACS_40Hz_mA +
                         COEFF_GAMMA['TMS_40Hz'] * hw.TMS_40Hz_strength)

    # V9: PFC↓ — tDCS cathode + TMS(1Hz)
    v['PFC'] = max(0.01, 1.0 - (COEFF_PFC['tDCS_cathode'] * hw.tDCS_cathode_F4_mA +
                                  COEFF_PFC['TMS_1Hz'] * hw.TMS_1Hz_strength))

    # V10: Sensory — tDCS(V1) + noise + LED(40Hz) + TENS + tACS(40Hz)
    v['Sensory'] = 1.0 + (COEFF_SENSORY['tDCS_anode'] * hw.tDCS_anode_V1_mA +
                           COEFF_SENSORY['noise'] * hw.noise_level +
                           COEFF_SENSORY['LED_40Hz'] * hw.LED_40Hz +
                           COEFF_SENSORY['TENS'] * hw.TENS_low_intensity +
                           COEFF_SENSORY['tACS_40Hz'] * hw.tACS_40Hz_mA)

    # V11: Body — TENS(low) + TENS(high) + tDCS(S1) + Vibro(40Hz)
    v['Body'] = 1.0 + (COEFF_BODY['TENS_low'] * hw.TENS_low_intensity +
                        COEFF_BODY['TENS_high'] * hw.TENS_high_intensity +
                        COEFF_BODY['tDCS_S1'] * hw.tDCS_anode_S1_mA +
                        COEFF_BODY['vibro_40Hz'] * hw.vibro_40Hz)

    # V12: Coherence — multimodal 40Hz sync + TMS(40Hz) + tACS(40Hz)
    gamma_total = (hw.LED_40Hz + hw.audio_40Hz + hw.vibro_40Hz) / 3
    v['Coherence'] = 1.0 + (COEFF_COHERENCE['gamma_40Hz'] * gamma_total +
                             COEFF_COHERENCE['paired_TMS'] * hw.TMS_40Hz_strength +
                             COEFF_COHERENCE['sync_stim'] * gamma_total +
                             COEFF_COHERENCE['tACS_40Hz'] * hw.tACS_40Hz_mA)

    return v


# ═══════════════════════════════════════════════════════════
# Tension Computation (PureField Framework)
# ═══════════════════════════════════════════════════════════

TENSION_WEIGHTS = {
    'DA': 1.2, 'eCB': 1.5, '5HT': 0.8, 'GABA': 0.9, 'NE': 1.0,
    'Theta': 1.3, 'Alpha': 1.0, 'Gamma': 1.1, 'PFC': 1.0,
    'Sensory': 0.9, 'Body': 1.0, 'Coherence': 1.2,
}

CHEM_VARS = ['DA', 'eCB', '5HT', 'GABA', 'NE']
WAVE_VARS = ['Theta', 'Alpha', 'Gamma']
STATE_VARS = ['PFC', 'Sensory', 'Body', 'Coherence']


def compute_tension(variables: dict, target: dict = None) -> dict:
    """PureField 장력 계산."""
    if target is None:
        target = THC_TARGET

    def _sub_tension(var_names, vals):
        return math.sqrt(sum(TENSION_WEIGHTS[k] * (vals[k] - 1.0) ** 2 for k in var_names))

    t_chem = _sub_tension(CHEM_VARS, variables)
    t_wave = _sub_tension(WAVE_VARS, variables)
    t_state = _sub_tension(STATE_VARS, variables)
    t_total = math.sqrt(t_chem**2 + t_wave**2 + t_state**2)

    t_chem_thc = _sub_tension(CHEM_VARS, target)
    t_wave_thc = _sub_tension(WAVE_VARS, target)
    t_state_thc = _sub_tension(STATE_VARS, target)
    t_total_thc = math.sqrt(t_chem_thc**2 + t_wave_thc**2 + t_state_thc**2)

    dot = sum(TENSION_WEIGHTS[k] * (variables[k] - 1.0) * (target[k] - 1.0) for k in target)
    mag_hw = math.sqrt(sum(TENSION_WEIGHTS[k] * (variables[k] - 1.0)**2 for k in target))
    mag_thc = math.sqrt(sum(TENSION_WEIGHTS[k] * (target[k] - 1.0)**2 for k in target))
    direction_sim = dot / (mag_hw * mag_thc) if mag_hw > 0 and mag_thc > 0 else 0

    magnitude_match = min(t_total, t_total_thc) / max(t_total, t_total_thc) * 100 if t_total_thc > 0 else 0
    tension_match = direction_sim * magnitude_match

    return {
        'T_chem': t_chem, 'T_wave': t_wave, 'T_state': t_state, 'T_total': t_total,
        'T_chem_thc': t_chem_thc, 'T_wave_thc': t_wave_thc, 'T_state_thc': t_state_thc, 'T_total_thc': t_total_thc,
        'direction_sim': direction_sim * 100,
        'magnitude_match': magnitude_match,
        'tension_match': tension_match,
    }


def compute_match(variables: dict, target_override: dict = None) -> dict:
    """각 변수의 THC 대비 달성률(%) 계산."""
    tgt = target_override or THC_TARGET
    match = {}
    for k, target in tgt.items():
        actual = variables.get(k, 1.0)
        if target >= 1.0:
            match[k] = actual / target * 100
        else:
            match[k] = (1.0 - actual) / (1.0 - target) * 100 if target < 1.0 else 100
    return match


# ═══════════════════════════════════════════════════════════
# Preset Configurations (brain stimulation only)
# ═══════════════════════════════════════════════════════════

# Tier 1: tDCS + TENS + Arduino (LED/vibro/audio) — $85
# 극한: 모든 파라미터 안전 한계 MAX, 시분할 다채널 tDCS, audio_40Hz 추가
TIER1_CONFIG = HardwareConfig(
    name="Tier 1 (저가)", cost=85, tier="low",
    tDCS_anode_mA=2.0, tDCS_cathode_Fz_mA=2.0, tDCS_cathode_F4_mA=2.0,
    tDCS_anode_V1_mA=1.5, tDCS_anode_S1_mA=1.5,  # 시분할 몽타주
    VNS_mA=0.4,  # TENS ear-clip 극한
    TENS_low_intensity=1.0, TENS_high_intensity=0.8,
    LED_40Hz=1.0, audio_40Hz=1.0, binaural_6Hz=1.0, vibro_40Hz=1.0,
    noise_level=0.8, alpha_entrainment=0.8,
)

# Tier 2: + taVNS + tACS (6/10/40Hz) + multi-site tDCS — $510
# 극한: 모든 tACS 2.0mA, VNS 0.5mA, 센서리 전부 MAX
TIER2_CONFIG = HardwareConfig(
    name="Tier 2 (중가)", cost=510, tier="mid",
    tDCS_anode_mA=2.0, tDCS_cathode_Fz_mA=2.0, tDCS_cathode_F4_mA=2.0,
    tDCS_anode_V1_mA=2.0, tDCS_anode_S1_mA=2.0,
    VNS_mA=0.5,
    TENS_low_intensity=1.0, TENS_high_intensity=0.8,
    tACS_6Hz_mA=2.0, tACS_10Hz_mA=2.0, tACS_40Hz_mA=2.0,
    LED_40Hz=1.0, audio_40Hz=1.0, binaural_6Hz=1.0, vibro_40Hz=1.0,
    noise_level=0.8, alpha_entrainment=1.0,
)

# Tier 3: + TMS (1/6/10/40Hz) + research-grade — $8,500
# 극한: 모든 파라미터 절대 안전 MAX
TIER3_CONFIG = HardwareConfig(
    name="Tier 3 (고가)", cost=8500, tier="high",
    tDCS_anode_mA=2.0, tDCS_cathode_Fz_mA=2.0, tDCS_cathode_F4_mA=2.0,
    tDCS_anode_V1_mA=2.0, tDCS_anode_S1_mA=2.0,
    VNS_mA=0.5,
    TENS_low_intensity=1.0, TENS_high_intensity=1.0,
    TMS_theta_strength=1.0, TMS_1Hz_strength=1.0,
    TMS_10Hz_strength=1.0, TMS_40Hz_strength=1.0,
    tACS_6Hz_mA=2.0, tACS_10Hz_mA=2.0, tACS_40Hz_mA=2.0,
    LED_40Hz=1.0, audio_40Hz=1.0, binaural_6Hz=1.0, vibro_40Hz=1.0,
    noise_level=1.0, alpha_entrainment=1.0,
)

ALL_CONFIGS = {
    'tier1': TIER1_CONFIG,
    'tier2': TIER2_CONFIG,
    'tier3': TIER3_CONFIG,
}


# ═══════════════════════════════════════════════════════════
# Display
# ═══════════════════════════════════════════════════════════

def print_results(hw: HardwareConfig, variables: dict, match: dict):
    """12변수 결과 출력."""
    avg = sum(match.values()) / len(match)
    over100 = sum(1 for v in match.values() if v >= 100)

    print(f"\n{'='*70}")
    print(f"  {hw.name}  |  Cost: ${hw.cost:,.0f}  |  Avg: {avg:.1f}%  |  {over100}/12 ≥100%")
    print(f"{'='*70}")
    print(f"  {'Var':<12} {'Target':>8} {'Actual':>8} {'Match':>8}  {'Status'}")
    print(f"  {'-'*12} {'-'*8} {'-'*8} {'-'*8}  {'-'*6}")

    for k in THC_TARGET:
        target = THC_TARGET[k]
        actual = variables[k]
        m = match[k]
        if m >= 100:
            status = "✅"
        elif m >= 80:
            status = "⚠️ " + f"(need +{100-m:.0f}%)"
        else:
            status = "❌" + f" (need +{100-m:.0f}%)"
        print(f"  {k:<12} {target:>7.1f}× {actual:>7.2f}× {m:>7.1f}%  {status}")

    print(f"\n  Overall: {avg:.1f}% average  |  {over100}/12 variables ≥100%")

    tension = compute_tension(variables)
    print(f"\n  ═══ PureField Tension Analysis ═══")
    print(f"  T_chem  (화학):  {tension['T_chem']:.3f} / {tension['T_chem_thc']:.3f}  ({tension['T_chem']/tension['T_chem_thc']*100:.0f}%)")
    print(f"  T_wave  (뇌파):  {tension['T_wave']:.3f} / {tension['T_wave_thc']:.3f}  ({tension['T_wave']/tension['T_wave_thc']*100:.0f}%)")
    print(f"  T_state (상태):  {tension['T_state']:.3f} / {tension['T_state_thc']:.3f}  ({tension['T_state']/tension['T_state_thc']*100:.0f}%)")
    print(f"  T_total (총):    {tension['T_total']:.3f} / {tension['T_total_thc']:.3f}  ({tension['magnitude_match']:.0f}%)")
    print(f"  방향 유사도:     {tension['direction_sim']:.1f}%")
    print(f"  장력 매칭률:     {tension['tension_match']:.1f}%  {'✅' if tension['tension_match'] >= 90 else '⚠️' if tension['tension_match'] >= 70 else '❌'}")

    print(f"\n  {'Variable':<12} {'0%':>4} {'50%':>8} {'100%':>9} {'150%':>9}")
    print(f"  {'-'*12} {'|':<4} {'|':>8} {'|':>9} {'|':>9}")
    for k in THC_TARGET:
        m = min(150, match[k])
        bar_len = int(m / 150 * 40)
        bar = "█" * bar_len + "░" * (40 - bar_len)
        marker = " ✅" if match[k] >= 100 else ""
        print(f"  {k:<12} {bar} {match[k]:>5.1f}%{marker}")


def print_levels(hw: HardwareConfig):
    """THC 농도별 달성률 + 유지시간 비교."""
    variables = compute_variables(hw)

    print(f"\n{'='*85}")
    print(f"  {hw.name} — THC 농도별 달성률 + 유지시간")
    print(f"{'='*85}")
    print(f"  {'Level':<9} {'THC%':>5} {'Scale':>6} {'Avg':>6} {'≥100%':>6} {'Onset':>7} {'Peak':>7} {'Duration':>10} {'설명'}")
    print(f"  {'-'*9} {'-'*5} {'-'*6} {'-'*6} {'-'*6} {'-'*7} {'-'*7} {'-'*10} {'-'*20}")

    for level, (scale, thc_pct, onset, peak, duration, desc) in THC_LEVELS.items():
        target = scale_target(level)
        match = compute_match(variables, target)
        avg = sum(match.values()) / 12
        over = sum(1 for v in match.values() if v >= 100)

        # 유지시간: 하드웨어 cycling 반영
        session = compute_session(hw, level)
        dur_str = f"{duration}min"
        cycle_str = f"({session['cycles']}cyc)"

        status = "✅" if over == 12 else f"{over}/12"
        print(f"  {level:<9} {thc_pct:>4}% {scale:>5.2f}× {avg:>5.1f}% {status:>6} {onset:>5}min {peak:>5}min {dur_str:>8} {cycle_str}  {desc}")

    # 세부 비교: micro vs strong
    print(f"\n  ─── 변수별 농도 비교 ───")
    print(f"  {'Var':<11}", end="")
    for level in THC_LEVELS:
        print(f" {level:>9}", end="")
    print()
    print(f"  {'-'*11}", end="")
    for _ in THC_LEVELS:
        print(f" {'-'*9}", end="")
    print()

    for k in THC_TARGET:
        print(f"  {k:<11}", end="")
        for level in THC_LEVELS:
            target = scale_target(level)
            match = compute_match(variables, target)
            m = match[k]
            mark = "✅" if m >= 100 else "  "
            print(f" {m:>6.0f}%{mark}", end="")
        print()

    # 유지시간 타임라인
    print(f"\n  ─── 세션 타임라인 (strong/25% 기준) ───")
    session = compute_session(hw, 'strong')
    total = session['duration_min']
    on = session['cycle_on']
    off = session['cycle_off']
    cycles = session['cycles']
    print(f"  총 유지: {total}min | {cycles} cycles × ({on}min ON + {off}min OFF)")
    print()
    t = 0
    for c in range(min(cycles, 6)):
        end_on = t + on
        end_off = end_on + off
        bar_on = "█" * on
        bar_off = "░" * off if c < cycles - 1 else ""
        phase = "ramp-up" if c == 0 else "유지" if c < cycles - 1 else "ramp-down"
        print(f"  [{t:>3}-{end_on:>3}min] {bar_on} ON  ({phase})")
        if bar_off:
            print(f"  [{end_on:>3}-{end_off:>3}min] {bar_off} OFF (휴식, tDCS/TMS 재설정)")
        t = end_off
    if cycles > 6:
        print(f"  ... +{cycles-6} cycles more ...")

    # THC 흡연 vs BrainWire 비교 차트
    print(f"\n  ─── THC 흡연 vs BrainWire 시간 곡선 ───")
    print(f"  {'시간':>6} {'THC 흡연(25%)':>14} {'BrainWire':>12}")
    print(f"  {'-'*6} {'-'*14} {'-'*12}")
    thc_curve = [
        (0, 0, 0), (5, 50, 30), (10, 80, 70), (15, 95, 90),
        (20, 100, 100), (30, 100, 100), (45, 90, 100),
        (60, 75, 100), (90, 50, 100), (120, 30, 100),
        (150, 15, 50), (180, 5, 0),
    ]
    for t, thc, hw_pct in thc_curve:
        thc_bar = "█" * (thc // 10) + "░" * (10 - thc // 10)
        hw_bar = "█" * (hw_pct // 10) + "░" * (10 - hw_pct // 10)
        print(f"  {t:>4}min {thc_bar} {thc:>3}% {hw_bar} {hw_pct:>3}%")
    print(f"  {'':>6} {'↑ 제어 불가':>14} {'↑ 전원 OFF=즉시':>12}")


def main():
    parser = argparse.ArgumentParser(description="BrainWire THC Variable Benchmark (brain stim only)")
    parser.add_argument('--tier', choices=['tier1', 'tier2', 'tier3', 'all'], default='all')
    parser.add_argument('--level', choices=list(THC_LEVELS.keys()), help='THC concentration level')
    parser.add_argument('--levels', action='store_true', help='Show all THC levels comparison')
    parser.add_argument('--json', action='store_true', help='JSON output')
    args = parser.parse_args()

    print("╔══════════════════════════════════════════════════════╗")
    print("║   BrainWire THC Variable Benchmark                  ║")
    print("║   Brain Stimulation Only — 12-Variable Test          ║")
    print("╚══════════════════════════════════════════════════════╝")

    # THC 농도별 비교 모드
    if args.levels:
        for name, hw in ALL_CONFIGS.items():
            print_levels(hw)
        return

    # 특정 THC 농도 모드
    if args.level:
        target = scale_target(args.level)
        scale, thc_pct, onset, peak, duration, desc = THC_LEVELS[args.level]
        print(f"\n  THC {thc_pct}% ({args.level}) Target Vector (scale={scale:.2f}×):")
        print(f"  {desc}")
        for k, v in target.items():
            arrow = "↓" if v < 1.0 else "↑"
            print(f"    {k:<12} {v:.2f}× {arrow}")
        print(f"  유지시간: onset {onset}min → peak {peak}min → duration {duration}min")

        configs = ALL_CONFIGS if args.tier == 'all' else {args.tier: ALL_CONFIGS[args.tier]}
        for name, hw in configs.items():
            variables = compute_variables(hw)
            match = compute_match(variables, target)
            print_results(hw, variables, match)
        return

    print(f"\n  THC Target Vector (25% strong — 기준):")
    for k, v in THC_TARGET.items():
        arrow = "↓" if v < 1.0 else "↑"
        print(f"    {k:<12} {v:.1f}× {arrow}")

    configs = ALL_CONFIGS if args.tier == 'all' else {args.tier: ALL_CONFIGS[args.tier]}
    all_results = {}

    for name, hw in configs.items():
        variables = compute_variables(hw)
        match = compute_match(variables)
        print_results(hw, variables, match)
        all_results[name] = {'variables': variables, 'match': match}

    if len(configs) > 1:
        print(f"\n{'='*70}")
        print(f"  TIER COMPARISON SUMMARY")
        print(f"{'='*70}")
        print(f"  {'Var':<12}", end="")
        for name in configs:
            print(f" {configs[name].name:>20}", end="")
        print()
        print(f"  {'-'*12}", end="")
        for _ in configs:
            print(f" {'-'*20}", end="")
        print()
        for k in THC_TARGET:
            print(f"  {k:<12}", end="")
            for name in configs:
                m = all_results[name]['match'][k]
                status = "✅" if m >= 100 else "⚠️"
                print(f" {m:>15.1f}% {status}", end="")
            print()

        print(f"\n  {'AVERAGE':<12}", end="")
        for name in configs:
            avg = sum(all_results[name]['match'].values()) / 12
            print(f" {avg:>17.1f}%  ", end="")
        print()
        print(f"  {'≥100%':<12}", end="")
        for name in configs:
            over = sum(1 for v in all_results[name]['match'].values() if v >= 100)
            print(f" {over:>15}/12   ", end="")
        print()
        print(f"  {'COST':<12}", end="")
        for name in configs:
            print(f" {'$'+str(int(configs[name].cost)):>18}  ", end="")
        print()

    if args.json:
        print(f"\n{json.dumps(all_results, indent=2)}")


if __name__ == '__main__':
    main()
