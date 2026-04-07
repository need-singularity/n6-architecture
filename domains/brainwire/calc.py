#!/usr/bin/env python3
"""BrainWire 수치 계산기 — THC 12변수 장력 기반 하드웨어 최적화.

사용법:
  python calc.py                          # 전체 Tier 비교
  python calc.py compute --vns 0.4 --tdcs 1.5 --tens 0.8  # 커스텀 계산
  python calc.py optimize --budget 500    # 예산 내 최적 조합
  python calc.py sensitivity              # 민감도 분석 (어떤 파라미터가 가장 효과적)
  python calc.py inverse --var DA --target 2.5   # 역계산 (목표→필요 파라미터)
  python calc.py sweep --param VNS --min 0 --max 0.5 --steps 20  # 파라미터 스윕
  python calc.py gap                      # 미달 변수 해결 방법 제시
"""

import math
import argparse
import itertools

# ═══════════════════════════════════════════════════════════
# Constants
# ═══════════════════════════════════════════════════════════

THC_TARGET = {
    'DA': 2.5, 'eCB': 3.0, '5HT': 1.5, 'GABA': 1.8, 'NE': 0.4,
    'Theta': 2.5, 'Alpha': 0.5, 'Gamma': 1.8, 'PFC': 0.5,
    'Sensory': 2.0, 'Body': 2.5, 'Coherence': 2.0,
}

# Transfer function coefficients
C = {
    'DA_tDCS': 0.25, 'DA_VNS': 0.80, 'DA_music': 1.50,
    'eCB_TENS': 0.80, 'eCB_heat': 0.30, 'eCB_VNS': 0.60, 'eCB_vibro': 0.50,
    '5HT_VNS': 1.20, '5HT_tDCS': 0.15,
    'GABA_tDCS': 0.20, 'GABA_weight': 0.03, 'GABA_alpha': 0.30,
    'NE_VNS': 1.50,
    'Theta_TMS': 0.80, 'Theta_bin': 0.40, 'Theta_tACS': 0.35,
    'Alpha_cathode': 0.20, 'Alpha_TMS1': 0.25,
    'Gamma_LED': 0.30, 'Gamma_audio': 0.25, 'Gamma_vibro40': 0.20, 'Gamma_tACS40': 0.15, 'Gamma_TMS40': 0.10,
    'PFC_cathode': 0.20, 'PFC_TMS1': 0.25,
    'Sens_tDCS': 0.15, 'Sens_noise': 0.40, 'Sens_LED': 0.20, 'Sens_TENS': 0.15, 'Sens_heat': 0.05,
    'Body_TENS': 0.80, 'Body_heat': 0.30, 'Body_vibro': 0.50,
    'Coh_gamma': 0.30, 'Coh_TMS40': 0.40, 'Coh_sync': 0.20, 'Coh_tACS40': 0.15,
}

# Hardware cost table
HW_COST = {
    'tDCS': 30, 'taVNS': 100, 'TENS': 25, 'TMS': 5000,
    'tACS': 80, 'LED_Arduino': 15, 'vibro': 5, 'heat_pad': 20,
    'blanket': 40, 'audio': 0, 'music': 0, 'EEG_4ch': 250,
    'EEG_16ch': 1000,
}

# Param ranges (name, min, max, step, cost_key)
PARAMS = {
    'tDCS':       (0, 2.0, 0.1, 'tDCS'),
    'cathode_Fz': (0, 2.0, 0.1, 'tDCS'),
    'cathode_F4': (0, 2.0, 0.1, 'tDCS'),
    'tDCS_V1':    (0, 2.0, 0.1, 'tDCS'),
    'VNS':        (0, 0.5, 0.05, 'taVNS'),
    'TENS':       (0, 1.0, 0.1, 'TENS'),
    'TMS_theta':  (0, 1.0, 0.1, 'TMS'),
    'TMS_1Hz':    (0, 1.0, 0.1, 'TMS'),
    'TMS_40Hz':   (0, 1.0, 0.1, 'TMS'),
    'tACS_6Hz':   (0, 2.0, 0.1, 'tACS'),
    'tACS_40Hz':  (0, 2.0, 0.1, 'tACS'),
    'LED_40Hz':   (0, 1.0, 0.1, 'LED_Arduino'),
    'audio_40Hz': (0, 1.0, 0.1, 'audio'),
    'binaural':   (0, 1.0, 0.1, 'audio'),
    'vibro_40Hz': (0, 1.0, 0.1, 'vibro'),
    'vibro':      (0, 1.0, 0.1, 'vibro'),
    'noise':      (0, 1.0, 0.1, 'LED_Arduino'),
    'music':      (0, 1.0, 0.1, 'music'),
    'heat':       (0, 5.0, 0.5, 'heat_pad'),
    'weight':     (0, 15.0, 1.0, 'blanket'),
    'alpha_ent':  (0, 1.0, 0.1, 'audio'),
}


# ═══════════════════════════════════════════════════════════
# Core Computation
# ═══════════════════════════════════════════════════════════

def compute(p: dict) -> dict:
    """파라미터 dict → 12변수 + 장력 계산."""
    v = {}
    v['DA']    = 1 + C['DA_tDCS']*p.get('tDCS',0) + C['DA_VNS']*p.get('VNS',0) + C['DA_music']*p.get('music',0)
    v['eCB']   = 1 + C['eCB_TENS']*p.get('TENS',0) + C['eCB_heat']*p.get('heat',0) + C['eCB_VNS']*p.get('VNS',0) + C['eCB_vibro']*p.get('vibro',0)
    v['5HT']   = 1 + C['5HT_VNS']*p.get('VNS',0) + C['5HT_tDCS']*p.get('tDCS',0)
    v['GABA']  = 1 + C['GABA_tDCS']*p.get('tDCS',0) + C['GABA_weight']*p.get('weight',0) + C['GABA_alpha']*p.get('alpha_ent',0)
    v['NE']    = max(0.01, 1 - C['NE_VNS']*p.get('VNS',0))
    v['Theta'] = 1 + C['Theta_TMS']*p.get('TMS_theta',0) + C['Theta_bin']*p.get('binaural',0) + C['Theta_tACS']*p.get('tACS_6Hz',0)
    v['Alpha'] = max(0.01, 1 - C['Alpha_cathode']*p.get('cathode_Fz',0) - C['Alpha_TMS1']*p.get('TMS_1Hz',0))
    v['Gamma'] = 1 + C['Gamma_LED']*p.get('LED_40Hz',0) + C['Gamma_audio']*p.get('audio_40Hz',0) + C['Gamma_vibro40']*p.get('vibro_40Hz',0) + C['Gamma_tACS40']*p.get('tACS_40Hz',0) + C['Gamma_TMS40']*p.get('TMS_40Hz',0)
    v['PFC']   = max(0.01, 1 - C['PFC_cathode']*p.get('cathode_F4',0) - C['PFC_TMS1']*p.get('TMS_1Hz',0))
    v['Sensory'] = 1 + C['Sens_tDCS']*p.get('tDCS_V1',0) + C['Sens_noise']*p.get('noise',0) + C['Sens_LED']*p.get('LED_40Hz',0) + C['Sens_TENS']*p.get('TENS',0) + C['Sens_heat']*p.get('heat',0)
    v['Body']  = 1 + C['Body_TENS']*p.get('TENS',0) + C['Body_heat']*p.get('heat',0) + C['Body_vibro']*p.get('vibro',0)
    g_avg = (p.get('LED_40Hz',0) + p.get('audio_40Hz',0) + p.get('vibro_40Hz',0)) / 3
    v['Coherence'] = 1 + C['Coh_gamma']*g_avg + C['Coh_TMS40']*p.get('TMS_40Hz',0) + C['Coh_sync']*g_avg + C['Coh_tACS40']*p.get('tACS_40Hz',0)
    return v


def match_pct(v: dict) -> dict:
    """변수별 달성률(%)."""
    m = {}
    for k, t in THC_TARGET.items():
        a = v[k]
        if t >= 1.0:
            m[k] = a / t * 100
        else:
            m[k] = (1 - a) / (1 - t) * 100
    return m


def tension(v: dict) -> dict:
    """장력 계산."""
    W = {'DA':1.2,'eCB':1.5,'5HT':0.8,'GABA':0.9,'NE':1.0,'Theta':1.3,'Alpha':1.0,'Gamma':1.1,'PFC':1.0,'Sensory':0.9,'Body':1.0,'Coherence':1.2}
    chem = ['DA','eCB','5HT','GABA','NE']
    wave = ['Theta','Alpha','Gamma']
    state = ['PFC','Sensory','Body','Coherence']

    def _t(keys, vals):
        return math.sqrt(sum(W[k]*(vals[k]-1)**2 for k in keys))

    tc, tw, ts = _t(chem,v), _t(wave,v), _t(state,v)
    tt = math.sqrt(tc**2 + tw**2 + ts**2)

    tc_thc, tw_thc, ts_thc = _t(chem,THC_TARGET), _t(wave,THC_TARGET), _t(state,THC_TARGET)
    tt_thc = math.sqrt(tc_thc**2 + tw_thc**2 + ts_thc**2)

    dot = sum(W[k]*(v[k]-1)*(THC_TARGET[k]-1) for k in THC_TARGET)
    m1 = math.sqrt(sum(W[k]*(v[k]-1)**2 for k in THC_TARGET))
    m2 = math.sqrt(sum(W[k]*(THC_TARGET[k]-1)**2 for k in THC_TARGET))
    direction = dot/(m1*m2)*100 if m1>0 and m2>0 else 0
    magnitude = min(tt,tt_thc)/max(tt,tt_thc)*100 if tt_thc>0 else 0

    return {'chem':tc,'wave':tw,'state':ts,'total':tt,
            'chem_thc':tc_thc,'wave_thc':tw_thc,'state_thc':ts_thc,'total_thc':tt_thc,
            'direction':direction,'magnitude':magnitude,'match':direction*magnitude/100}


def cost(p: dict) -> float:
    """하드웨어 비용 추정."""
    used = set()
    for param, (_, _, _, cost_key) in PARAMS.items():
        if p.get(param, 0) > 0:
            used.add(cost_key)
    return sum(HW_COST.get(k, 0) for k in used)


def print_result(p: dict, label=""):
    """결과 출력."""
    v = compute(p)
    m = match_pct(v)
    t = tension(v)
    c = cost(p)
    avg = sum(m.values())/12
    over = sum(1 for x in m.values() if x >= 100)

    if label:
        print(f"\n{'='*65}")
        print(f"  {label}  |  ${c:,.0f}  |  Avg {avg:.1f}%  |  {over}/12≥100%  |  T={t['match']:.1f}%")
        print(f"{'='*65}")
    print(f"  {'Var':<11} {'Target':>7} {'Actual':>7} {'Match':>7} {'Bar'}")
    print(f"  {'-'*11} {'-'*7} {'-'*7} {'-'*7} {'-'*25}")
    for k in THC_TARGET:
        tgt = THC_TARGET[k]
        act = v[k]
        pct = m[k]
        bar = '█'*int(min(pct,150)/150*20) + '░'*(20-int(min(pct,150)/150*20))
        ok = '✅' if pct>=100 else ''
        print(f"  {k:<11} {tgt:>6.1f}× {act:>6.2f}× {pct:>6.1f}% {bar} {ok}")

    print(f"\n  ═══ 장력 ═══")
    print(f"  T_chem={t['chem']:.2f}/{t['chem_thc']:.2f}  T_wave={t['wave']:.2f}/{t['wave_thc']:.2f}  T_state={t['state']:.2f}/{t['state_thc']:.2f}")
    print(f"  T_total={t['total']:.2f}/{t['total_thc']:.2f}  방향={t['direction']:.1f}%  크기={t['magnitude']:.1f}%  매칭={t['match']:.1f}%")


# ═══════════════════════════════════════════════════════════
# Commands
# ═══════════════════════════════════════════════════════════

def cmd_compute(args):
    """커스텀 파라미터 계산."""
    p = {k: getattr(args, k, 0) or 0 for k in PARAMS}
    print_result(p, "Custom Config")


def cmd_tiers(args):
    """3 Tier 비교."""
    tiers = {
        'Tier 1 ($95)': dict(tDCS=1.5, cathode_Fz=1.5, cathode_F4=1.5, VNS=0.3,
                             TENS=0.8, LED_40Hz=0.8, binaural=0.7, vibro_40Hz=0.6,
                             vibro=0.6, noise=0.3, music=0.6, heat=3.0, weight=5, alpha_ent=0.5),
        'Tier 2 ($525)': dict(tDCS=1.5, cathode_Fz=1.5, cathode_F4=1.5, tDCS_V1=1.5,
                              VNS=0.4, TENS=0.8, tACS_6Hz=1.8, tACS_40Hz=1.5,
                              LED_40Hz=0.8, audio_40Hz=0.8, binaural=0.7, vibro_40Hz=0.8,
                              vibro=0.6, noise=0.5, music=0.6, heat=3.0, weight=10, alpha_ent=0.7),
        'Tier 3 ($8.5K)': dict(tDCS=2.0, cathode_Fz=1.5, cathode_F4=1.5, tDCS_V1=2.0,
                               VNS=0.4, TENS=1.0, TMS_theta=0.8, TMS_1Hz=0.8, TMS_40Hz=0.8,
                               tACS_6Hz=1.8, tACS_40Hz=2.0,
                               LED_40Hz=0.9, audio_40Hz=0.9, binaural=0.7, vibro_40Hz=1.0,
                               vibro=0.7, noise=0.7, music=0.6, heat=3.0, weight=10, alpha_ent=0.7),
    }
    for name, p in tiers.items():
        print_result(p, name)


def cmd_sensitivity(args):
    """민감도 분석 — 각 파라미터 1단위 변화가 전체 달성률에 미치는 영향."""
    base = dict(tDCS=1.5, cathode_Fz=1.5, cathode_F4=1.5, VNS=0.3,
                TENS=0.8, LED_40Hz=0.8, binaural=0.7, vibro_40Hz=0.6,
                vibro=0.6, noise=0.3, music=0.6, heat=3.0, weight=5, alpha_ent=0.5)
    base_avg = sum(match_pct(compute(base)).values()) / 12

    print(f"\n  ═══ 민감도 분석 (Tier 1 기준, 파라미터 +0.1 변화당 달성률 변화) ═══\n")
    print(f"  {'파라미터':<15} {'현재값':>7} {'Δ달성률':>8} {'영향 변수'}")
    print(f"  {'-'*15} {'-'*7} {'-'*8} {'-'*30}")

    results = []
    for param in PARAMS:
        p_up = base.copy()
        step = 0.1
        p_up[param] = p_up.get(param, 0) + step
        # Clamp to max
        mn, mx, _, _ = PARAMS[param]
        p_up[param] = min(p_up[param], mx)
        new_avg = sum(match_pct(compute(p_up)).values()) / 12
        delta = new_avg - base_avg

        # Which vars changed?
        m_base = match_pct(compute(base))
        m_new = match_pct(compute(p_up))
        changed = [k for k in THC_TARGET if abs(m_new[k]-m_base[k]) > 0.1]

        results.append((param, base.get(param,0), delta, changed))

    results.sort(key=lambda x: -abs(x[2]))
    for param, cur, delta, changed in results:
        arrow = '↑' if delta > 0 else '↓' if delta < 0 else '='
        ch_str = ', '.join(changed[:4])
        print(f"  {param:<15} {cur:>6.2f}  {arrow}{abs(delta):>6.2f}%  {ch_str}")

    print(f"\n  ★ 가장 효과적인 파라미터 (비용 대비):")
    for param, cur, delta, changed in results[:5]:
        _, _, _, ck = PARAMS[param]
        hw_cost = HW_COST.get(ck, 0)
        eff = abs(delta) / max(hw_cost, 1) * 1000
        print(f"    {param:<15} Δ{abs(delta):.2f}%/step  (${hw_cost})  효율={eff:.1f}")


def cmd_inverse(args):
    """역계산 — 특정 변수 목표값 달성에 필요한 최소 파라미터."""
    var = args.var
    target = args.target or THC_TARGET.get(var, 2.0)

    print(f"\n  ═══ 역계산: {var} = {target:.1f}× 달성에 필요한 하드웨어 ═══\n")

    # Variable → parameter mapping
    VAR_PARAMS = {
        'DA':    [('tDCS', C['DA_tDCS']), ('VNS', C['DA_VNS']), ('music', C['DA_music'])],
        'eCB':   [('TENS', C['eCB_TENS']), ('heat', C['eCB_heat']), ('VNS', C['eCB_VNS']), ('vibro', C['eCB_vibro'])],
        '5HT':   [('VNS', C['5HT_VNS']), ('tDCS', C['5HT_tDCS'])],
        'GABA':  [('tDCS', C['GABA_tDCS']), ('weight', C['GABA_weight']), ('alpha_ent', C['GABA_alpha'])],
        'NE':    [('VNS', C['NE_VNS'])],
        'Theta': [('TMS_theta', C['Theta_TMS']), ('binaural', C['Theta_bin']), ('tACS_6Hz', C['Theta_tACS'])],
        'Alpha': [('cathode_Fz', C['Alpha_cathode']), ('TMS_1Hz', C['Alpha_TMS1'])],
        'Gamma': [('LED_40Hz', C['Gamma_LED']), ('audio_40Hz', C['Gamma_audio']), ('vibro_40Hz', C['Gamma_vibro40']), ('tACS_40Hz', C['Gamma_tACS40']), ('TMS_40Hz', C['Gamma_TMS40'])],
        'PFC':   [('cathode_F4', C['PFC_cathode']), ('TMS_1Hz', C['PFC_TMS1'])],
        'Sensory':[('tDCS_V1', C['Sens_tDCS']), ('noise', C['Sens_noise']), ('LED_40Hz', C['Sens_LED']), ('TENS', C['Sens_TENS']), ('heat', C['Sens_heat'])],
        'Body':  [('TENS', C['Body_TENS']), ('heat', C['Body_heat']), ('vibro', C['Body_vibro'])],
        'Coherence': [('LED_40Hz', C['Coh_gamma']), ('TMS_40Hz', C['Coh_TMS40']), ('vibro_40Hz', C['Coh_sync']), ('tACS_40Hz', C['Coh_tACS40'])],
    }

    params = VAR_PARAMS.get(var, [])
    if not params:
        print(f"  Unknown variable: {var}")
        return

    is_decrease = target < 1.0
    needed = (1.0 - target) if is_decrease else (target - 1.0)

    print(f"  {'전략':<5} {'파라미터':<15} {'필요값':>8} {'범위':>10} {'비용':>8} {'가능'}")
    print(f"  {'-'*5} {'-'*15} {'-'*8} {'-'*10} {'-'*8} {'-'*4}")

    for i, (param, coeff) in enumerate(params):
        mn, mx, _, ck = PARAMS[param]
        required = needed / coeff if coeff > 0 else float('inf')
        hw_c = HW_COST.get(ck, 0)
        possible = "✅" if required <= mx else f"❌ (max {mx})"
        print(f"  {i+1:>4}. {param:<15} {required:>7.2f}  [0-{mx}]   ${hw_c:<6} {possible}")

    # Best combo: use all params at proportional levels
    print(f"\n  최적 조합 (모든 파라미터 비례 분배):")
    total_coeff = sum(c for _, c in params)
    for param, coeff in params:
        mn, mx, _, ck = PARAMS[param]
        share = needed * (coeff / total_coeff) / coeff
        share = min(share, mx)
        print(f"    {param:<15} = {share:.2f}")


def cmd_sweep(args):
    """파라미터 스윕 — 하나를 변화시키며 12변수 변화 추적."""
    param = args.param
    mn = args.min if args.min is not None else PARAMS[param][0]
    mx = args.max if args.max is not None else PARAMS[param][1]
    steps = args.steps or 20

    base = dict(tDCS=1.5, cathode_Fz=1.5, cathode_F4=1.5, VNS=0.3,
                TENS=0.8, LED_40Hz=0.8, binaural=0.7, vibro_40Hz=0.6,
                vibro=0.6, noise=0.3, music=0.6, heat=3.0, weight=5, alpha_ent=0.5)

    print(f"\n  ═══ 파라미터 스윕: {param} [{mn} → {mx}] ═══\n")
    print(f"  {param:>8}", end="")
    for k in THC_TARGET:
        print(f" {k:>6}", end="")
    print(f" {'Avg':>6} {'T_match':>7}")
    print(f"  {'-'*8}", end="")
    for _ in THC_TARGET:
        print(f" {'-'*6}", end="")
    print(f" {'-'*6} {'-'*7}")

    for i in range(steps + 1):
        val = mn + (mx - mn) * i / steps
        p = base.copy()
        p[param] = val
        v = compute(p)
        m = match_pct(v)
        t = tension(v)
        avg = sum(m.values()) / 12
        print(f"  {val:>8.2f}", end="")
        for k in THC_TARGET:
            pct = m[k]
            print(f" {pct:>5.0f}%", end="")
        print(f" {avg:>5.1f}% {t['match']:>6.1f}%")


def cmd_gap(args):
    """미달 변수 분석 — 각 Tier에서 100% 미달인 변수의 해결 방법."""
    tiers = {
        'Tier 1': dict(tDCS=1.5, cathode_Fz=1.5, cathode_F4=1.5, VNS=0.3,
                        TENS=0.8, LED_40Hz=0.8, binaural=0.7, vibro_40Hz=0.6,
                        vibro=0.6, noise=0.3, music=0.6, heat=3.0, weight=5, alpha_ent=0.5),
        'Tier 2': dict(tDCS=1.5, cathode_Fz=1.5, cathode_F4=1.5, tDCS_V1=1.5,
                        VNS=0.4, TENS=0.8, tACS_6Hz=1.8, tACS_40Hz=1.5,
                        LED_40Hz=0.8, audio_40Hz=0.8, binaural=0.7, vibro_40Hz=0.8,
                        vibro=0.6, noise=0.5, music=0.6, heat=3.0, weight=10, alpha_ent=0.7),
        'Tier 3': dict(tDCS=2.0, cathode_Fz=1.5, cathode_F4=1.5, tDCS_V1=2.0,
                        VNS=0.4, TENS=1.0, TMS_theta=0.8, TMS_1Hz=0.8, TMS_40Hz=0.8,
                        tACS_6Hz=1.8, tACS_40Hz=2.0,
                        LED_40Hz=0.9, audio_40Hz=0.9, binaural=0.7, vibro_40Hz=1.0,
                        vibro=0.7, noise=0.7, music=0.6, heat=3.0, weight=10, alpha_ent=0.7),
    }

    for tier_name, p in tiers.items():
        v = compute(p)
        m = match_pct(v)
        gaps = {k: pct for k, pct in m.items() if pct < 100}
        if not gaps:
            print(f"\n  {tier_name}: 모든 변수 100%+ ✅")
            continue
        print(f"\n  ═══ {tier_name} 미달 변수 ({len(gaps)}개) ═══")
        for var, pct in sorted(gaps.items(), key=lambda x: x[1]):
            needed = THC_TARGET[var]
            actual = v[var]
            deficit = 100 - pct
            print(f"\n  {var}: {pct:.0f}% (actual={actual:.2f}, target={needed:.1f})")
            print(f"  → 부족분: {deficit:.0f}%p")
            # Suggest solutions
            solutions = {
                'Theta': "tACS 6Hz 추가 ($80) 또는 TMS theta burst ($5K)",
                'Alpha': "TMS 1Hz rTMS ($5K) — tDCS cathode만으로는 한계",
                'Gamma': "audio_40Hz 추가 ($0) + vibro_40Hz 강도↑ ($5)",
                'PFC':   "TMS 1Hz on DLPFC ($5K) — 가장 효과적",
                'Sensory': "tDCS_V1 채널 추가 ($0, 전극만) + noise↑",
                'Body':  "TENS 강도↑ + heat↑ (이미 있는 장비 활용)",
                'Coherence': "TMS 40Hz ($5K) 또는 audio_40Hz+vibro_40Hz 강도↑ ($0-5)",
                'GABA':  "가중담요 무게↑ ($40) + alpha entrainment↑",
                'NE':    "taVNS 전류↑ 0.4mA ($100, 전용 디바이스)",
                'DA':    "music pleasure↑ (좋은 playlist) + VNS↑",
                'eCB':   "TENS↑ + heat↑ + vibro↑",
                '5HT':   "VNS↑ + tDCS↑",
            }
            print(f"  → 해결: {solutions.get(var, 'N/A')}")


def cmd_optimize(args):
    """예산 제약 내 최적 조합 탐색 (grid search)."""
    budget = args.budget or 500
    print(f"\n  ═══ 최적화: 예산 ${budget} 내 최대 달성률 ═══\n")

    # Define hardware bundles with costs
    bundles = [
        ('기본(tDCS+TENS+LED)', 70, dict(tDCS=1.5, cathode_Fz=1.5, cathode_F4=1.5, TENS=0.8,
                                          LED_40Hz=0.8, vibro_40Hz=0.6, vibro=0.6, noise=0.3,
                                          music=0.6, heat=3.0, binaural=0.7, alpha_ent=0.5, weight=5)),
        ('+taVNS', 100, dict(VNS=0.4)),
        ('+가중담요', 40, dict(weight=10)),
        ('+tACS', 80, dict(tACS_6Hz=1.8, tACS_40Hz=1.5)),
        ('+tDCS_V1', 0, dict(tDCS_V1=1.5)),
        ('+audio_40Hz', 0, dict(audio_40Hz=0.8)),
        ('+noise↑', 0, dict(noise=0.5)),
        ('+vibro_40Hz↑', 0, dict(vibro_40Hz=0.8)),
        ('+EEG 4ch', 250, dict()),  # measurement only
        ('+TMS', 5000, dict(TMS_theta=0.8, TMS_1Hz=0.8, TMS_40Hz=0.8)),
    ]

    best_avg = 0
    best_tension = 0
    best_combo = None
    best_p = None

    # Try all combinations of optional bundles (2^N, base always included)
    base_name, base_cost, base_p = bundles[0]
    optional = bundles[1:]

    for r in range(len(optional)+1):
        for combo in itertools.combinations(range(len(optional)), r):
            total_cost = base_cost + sum(optional[i][1] for i in combo)
            if total_cost > budget:
                continue
            p = base_p.copy()
            names = [base_name]
            for i in combo:
                p.update(optional[i][2])
                names.append(optional[i][0])
            v = compute(p)
            m = match_pct(v)
            t = tension(v)
            avg = sum(m.values())/12
            if t['match'] > best_tension:
                best_tension = t['match']
                best_avg = avg
                best_combo = names
                best_p = p.copy()
                best_cost = total_cost

    if best_p:
        print(f"  최적 조합: {' '.join(best_combo)}")
        print(f"  비용: ${best_cost}")
        print_result(best_p, f"최적 (${best_cost}/{budget})")
    else:
        print(f"  예산 ${budget}로는 기본 구성도 불가능합니다.")


# ═══════════════════════════════════════════════════════════
# Main
# ═══════════════════════════════════════════════════════════

def main():
    parser = argparse.ArgumentParser(description="BrainWire 수치 계산기")
    sub = parser.add_subparsers(dest='cmd')

    # compute
    p_comp = sub.add_parser('compute', help='커스텀 파라미터 계산')
    for param in PARAMS:
        p_comp.add_argument(f'--{param}', type=float, default=0)

    # tiers
    sub.add_parser('tiers', help='3 Tier 비교')

    # sensitivity
    sub.add_parser('sensitivity', help='민감도 분석')

    # inverse
    p_inv = sub.add_parser('inverse', help='역계산')
    p_inv.add_argument('--var', required=True, help='변수명 (DA, eCB, ...)')
    p_inv.add_argument('--target', type=float, help='목표값 (기본: THC target)')

    # sweep
    p_sw = sub.add_parser('sweep', help='파라미터 스윕')
    p_sw.add_argument('--param', required=True, help='파라미터명')
    p_sw.add_argument('--min', type=float)
    p_sw.add_argument('--max', type=float)
    p_sw.add_argument('--steps', type=int, default=20)

    # gap
    sub.add_parser('gap', help='미달 변수 분석')

    # optimize
    p_opt = sub.add_parser('optimize', help='예산 내 최적화')
    p_opt.add_argument('--budget', type=int, default=500)

    args = parser.parse_args()
    if args.cmd == 'compute':
        cmd_compute(args)
    elif args.cmd == 'sensitivity':
        cmd_sensitivity(args)
    elif args.cmd == 'inverse':
        cmd_inverse(args)
    elif args.cmd == 'sweep':
        cmd_sweep(args)
    elif args.cmd == 'gap':
        cmd_gap(args)
    elif args.cmd == 'optimize':
        cmd_optimize(args)
    else:
        cmd_tiers(args)


if __name__ == '__main__':
    main()
