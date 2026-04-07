TIER_CONFIGS = {
    1: {
        'name': 'Tier 1 (tDCS + TENS + Arduino)',
        'cost': 85,
        'params': {
            'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
            'tDCS_anode_V1_mA': 1.5, 'tDCS_anode_S1_mA': 1.5,
            'taVNS_VNS_mA': 0.4,
            'TENS_low': 1.0, 'TENS_high': 0.8,
        },
    },
    2: {
        'name': 'Tier 2 (+ taVNS + tACS)',
        'cost': 510,
        'params': {
            'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
            'tDCS_anode_V1_mA': 2.0, 'tDCS_anode_S1_mA': 2.0,
            'taVNS_VNS_mA': 0.5,
            'TENS_low': 1.0, 'TENS_high': 0.8,
            'tACS_6Hz_mA': 2.0, 'tACS_10Hz_mA': 2.0, 'tACS_40Hz_mA': 2.0,
        },
    },
    3: {
        'name': 'Tier 3 (+ TMS)',
        'cost': 8500,
        'params': {
            'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
            'tDCS_anode_V1_mA': 2.0, 'tDCS_anode_S1_mA': 2.0,
            'taVNS_VNS_mA': 0.5,
            'TENS_low': 1.0, 'TENS_high': 1.0,
            'TMS_theta': 1.0, 'TMS_1Hz': 1.0, 'TMS_10Hz': 1.0, 'TMS_40Hz': 1.0,
            'tACS_6Hz_mA': 2.0, 'tACS_10Hz_mA': 2.0, 'tACS_40Hz_mA': 2.0,
        },
    },
    4: {
        'name': 'Tier 4 (+ tFUS + GVS + mTI + tSCS + tRNS + HD-tDCS + 256ch EEG)',
        'cost': 25000,
        'params': {
            'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
            'tDCS_anode_V1_mA': 2.0, 'tDCS_anode_S1_mA': 2.0,
            'taVNS_VNS_mA': 0.5,
            'TENS_low': 1.0, 'TENS_high': 1.0,
            'TMS_theta': 1.0, 'TMS_1Hz': 1.0, 'TMS_10Hz': 1.0, 'TMS_40Hz': 1.0,
            'tACS_6Hz_mA': 2.0, 'tACS_10Hz_mA': 2.0, 'tACS_40Hz_mA': 2.0,
            'tFUS_VTA_intensity': 0.8, 'tFUS_hippo_intensity': 0.8,
            'tFUS_raphe_intensity': 0.8, 'tFUS_LC_intensity': 0.6,
            'tFUS_V1_intensity': 0.7, 'tFUS_40Hz_intensity': 0.6,
            'GVS_current_mA': 1.0,
            'mTI_dlPFC_intensity': 0.8, 'mTI_thalamus_intensity': 0.6,
            'mTI_LC_intensity': 0.7,
            'tSCS_intensity': 0.8,
            'tRNS_intensity': 0.7,
            'HD-tDCS_cathode_mA': 1.5,
        },
    },
    5: {
        'name': 'Tier 5 (Multi-Modal: electrical + photonic + acoustic + thermal + chemical)',
        'cost': 26400,
        'params': {
            # All Tier 4 electrical params
            'tDCS_anode_mA': 2.0, 'tDCS_cathode_Fz_mA': 2.0, 'tDCS_cathode_F4_mA': 2.0,
            'tDCS_anode_V1_mA': 2.0, 'tDCS_anode_S1_mA': 2.0,
            'taVNS_VNS_mA': 0.5,
            'TENS_low': 1.0, 'TENS_high': 1.0,
            'TMS_theta': 1.0, 'TMS_1Hz': 1.0, 'TMS_10Hz': 1.0, 'TMS_40Hz': 1.0,
            'tACS_6Hz_mA': 2.0, 'tACS_10Hz_mA': 2.0, 'tACS_40Hz_mA': 2.0,
            'tFUS_VTA_intensity': 0.8, 'tFUS_hippo_intensity': 0.8,
            'tFUS_raphe_intensity': 0.8, 'tFUS_LC_intensity': 0.6,
            'tFUS_V1_intensity': 0.7, 'tFUS_40Hz_intensity': 0.6,
            'GVS_current_mA': 1.0,
            'mTI_dlPFC_intensity': 0.8, 'mTI_thalamus_intensity': 0.6,
            'mTI_LC_intensity': 0.7,
            'tSCS_intensity': 0.8,
            'tRNS_intensity': 0.7,
            'HD-tDCS_cathode_mA': 1.5,
            # Tier 5 non-electrical additions
            'tPBM_intensity': 0.8,          # photobiomodulation ($500)
            'tPBM_prefrontal': 0.7,
            'tSMS_intensity': 0.6,          # static magnets ($20)
            'PEMF_intensity': 0.7,          # pulsed EM field ($500)
            'caloric_temperature': 0.6,     # caloric vestibular ($10)
            'thermal_cooling': 0.5,         # Peltier thermal ($200)
            'ionto_intensity': 0.5,         # iontophoresis ($300)
        },
    },
}


def get_tier_params(tier: int) -> dict[str, float]:
    if tier not in TIER_CONFIGS:
        raise ValueError(f"Unknown tier: {tier}")
    return TIER_CONFIGS[tier]['params'].copy()
