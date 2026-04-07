# N1 Deep Access Strategies — Reaching Deep Brain Through Cortical Electrodes

## The Problem

N1 electrodes sit at 3-6mm depth (cortical layers I-VI). The deep structures that
drive the 5 "deep" Joywire variables are far beyond reach:

| Structure       | Depth   | Variable(s)         |
|----------------|---------|---------------------|
| VTA            | ~70mm   | DA (dopamine)       |
| Locus coeruleus| ~80mm   | NE (norepinephrine) |
| Dorsal raphe   | ~80mm   | 5HT (serotonin)     |
| Hippocampus    | ~40mm   | eCB, Theta          |

Conventional analysis (Category 12, H-BW-086) confirms: N1 cortical-only access
does NOT improve deep variables over Tier 4 baseline. The gap is real.

## The Solution: 5 Indirect Pathways

N1 cannot reach deep structures directly. But cortical neurons have **projections**
to these deep structures. By stimulating the right cortical neurons at the right
timing, N1 can indirectly control deep structures through the brain's own wiring.

---

### 1. Cortico-Subcortical Projections (most promising)

The brain has direct axonal projections from cortex to deep structures. N1 electrodes
stimulate the cortical END of axons that already project to deep targets.

**Pathways:**

- **DLPFC -> VTA** (mesocortical pathway): Layer 5 pyramidal neurons in DLPFC project
  directly to VTA. Stimulation triggers DA release. This is the same mechanism as tDCS
  over F3, but N1 achieves it with ~1000x spatial precision (single-neuron targeting vs
  ~25cm2 electrode patches).

- **Prefrontal -> Raphe**: PFC projects to dorsal raphe nucleus. N1 stimulation of
  these projection neurons modulates 5HT release at the source.

- **Prefrontal -> LC**: PFC projects to locus coeruleus. Precise activation of
  inhibitory projection neurons enables NE suppression without affecting neighboring
  circuits.

- **Entorhinal cortex -> Hippocampus**: Layer 2/3 of entorhinal cortex projects to
  hippocampus via the perforant path. This is the most direct cortical-to-hippocampal
  connection in the brain. N1 stimulation here drives Theta oscillations and eCB
  release in hippocampus.

**Estimated effectiveness:** 30-50% of direct deep access (vs ~10% for scalp tDCS).

**Estimated coefficients:**
- DA via DLPFC->VTA: tDCS coeff (0.25) x 3.0 (precision) = 0.75
- 5HT via PFC->raphe: tDCS coeff (0.15) x 3.0 = 0.45
- NE via PFC->LC: indirect coefficient = 0.50
- eCB via entorhinal->hippocampus: tDCS coeff (0.20) x 3.0 = 0.60

---

### 2. Temporal Interference from Cortical Base

N1 has 1024 electrodes distributed across cortex. By driving two groups at slightly
different frequencies (e.g., 1000Hz and 1040Hz), the interference pattern (40Hz)
focuses at a point BETWEEN the electrode groups -- potentially deeper than either
group alone.

**Principle:** Same as multipolar temporal interference (mTI, Grossman et al. 2017),
but using N1's cortical electrodes instead of scalp electrodes.

**Advantage over scalp mTI:**
- Starting points already at 3-6mm depth (vs 0mm for scalp)
- 1024 electrodes allow much more precise beam-forming
- Could potentially reach 15-25mm depth (not VTA at 70mm, but reaches superficial
  hippocampus CA1 region approaching ~30mm)

**Spatial precision improvement:**
- Scalp mTI: starts at 0mm, focus at ~20mm, spread ~10mm diameter
- N1 mTI: starts at 3-6mm, focus at ~20mm from surface, spread ~5mm diameter
- Estimated 3x improvement in spatial precision

**Estimated additional depth:** 10-20mm beyond electrode tips = 13-26mm total depth.

---

### 3. Phase-Locked Network Driving

At <1ms stimulation latency, N1 can fire cortical neurons at the exact moment when
their downstream synapses are most effective.

**Mechanism: Spike-Timing Dependent Plasticity (STDP)**

STDP window: pre-synaptic neuron must fire 5-20ms before post-synaptic neuron for
connection strengthening. N1's <1ms latency means it can target any point in the
STDP window with sub-millisecond precision.

Compare to external stimulation latency (~40ms for tDCS/TMS) -- this MISSES the
entire STDP window. N1 is the first technology that can exploit STDP for deep driving.

**Protocol:**
1. Read EEG to detect deep structure oscillation phase (e.g., VTA firing pattern
   inferred from cortical correlates)
2. Fire cortical projection neurons at precisely t_deep - 10ms
3. Cortical volley arrives at deep structure at t_deep
4. STDP strengthens cortical->deep connection over minutes
5. Eventually, cortical stimulation alone drives deep structure activity

**Estimated coefficient boost:** +0.15-0.25 above baseline projection coefficient,
compounding over 10-30 minute sessions.

---

### 4. Oscillation Entrainment

Cortical oscillations naturally entrain subcortical structures through volume
conduction and synaptic coupling. N1 amplifies this natural mechanism.

**Pathways:**

- **Cortical theta (6Hz) -> hippocampal theta**: Proven in animal studies (Sirota
  et al. 2008). Cortical theta oscillations synchronize with hippocampal theta via
  the entorhinal cortex. N1 can drive spatially precise theta across entorhinal
  cortex with 1024 electrodes.

- **Cortical gamma (40Hz) -> thalamocortical loops**: Gamma coherence propagates
  through thalamocortical circuits. N1-driven gamma patterns entrain thalamic
  oscillators which then broadcast to subcortical structures.

- **Alpha suppression -> thalamic alpha**: Cortical alpha is generated by
  thalamocortical loops. N1 suppression of cortical alpha feeds back to thalamus,
  effectively controlling thalamic state from cortex.

**Estimated coupling coefficients:**
- Cortical -> hippocampal theta: 0.4-0.6 (well-documented in literature)
- Cortical -> thalamocortical gamma: 0.5-0.7
- Cortical alpha suppression -> thalamic: 0.6-0.8

---

### 5. Insular Autonomic Gateway

The insular cortex sits on the cortical surface and is accessible by N1 electrodes.
It projects to critical autonomic and emotional processing centers.

**Projection targets:**
- **Nucleus tractus solitarius (NTS)** -> vagal pathway -> same effects as taVNS
  but driven from cortex. DA, 5HT up; NE down.
- **Hypothalamus** -> autonomic control -> NE regulation, cortisol, body state
- **Amygdala** -> emotional processing, fear/reward circuits

**Estimated effect (% of taVNS):**
- DA: 30-40% of taVNS effect (taVNS coeff 0.80 -> insula 0.30)
- 5HT: 35-40% of taVNS effect (taVNS coeff 1.20 -> insula 0.45)
- NE: 35-40% of taVNS effect (taVNS coeff 1.50 -> insula 0.55)

**Key advantage:** No external device required. N1 alone can partially replicate
taVNS effects through the insular autonomic gateway.

---

## Revised Transfer Coefficients for N1-Only

Combining all 5 strategies, the estimated N1-only coefficients for deep variables:

| Variable | tDCS indirect | N1 projections | N1 TI | N1 STDP | N1 entrainment | N1 insula | N1 total | Direct target |
|----------|--------------|----------------|-------|---------|----------------|-----------|----------|---------------|
| DA       | 0.25         | 0.75           | 0.00  | 0.15    | 0.00           | 0.30      | 1.05*    | 3.6           |
| 5HT      | 0.15         | 0.45           | 0.00  | 0.10    | 0.00           | 0.45      | 0.60*    | 2.0           |
| NE       | indirect     | 0.50           | 0.00  | 0.10    | 0.00           | 0.55      | 0.55*    | 1.5           |
| eCB      | 0.20         | 0.60           | 0.10  | 0.10    | 0.15           | 0.00      | 0.60*    | 3.0           |
| Theta    | 0.00         | 0.00           | 0.10  | 0.05    | 0.40           | 0.00      | 0.40*    | 2.5           |

*Not additive -- overlap between strategies reduces combined coefficient. Values
shown are estimated effective totals after overlap correction.

## Comparison: N1-only vs Hybrid

| Variable    | Tier 4 (external) | N1-only (5 strategies) | Hybrid (N1 + external) |
|------------|-------------------|----------------------|----------------------|
| DA         | ~100%             | ~29% of target       | ~120%                |
| eCB        | ~100%             | ~20% of target       | ~110%                |
| 5HT        | ~100%             | ~30% of target       | ~115%                |
| NE         | ~100%             | ~37% of target       | ~110%                |
| Theta      | ~100%             | ~16% of target       | ~105%                |
| GABA       | ~100%             | ~300% (cortical)     | ~300%                |
| Alpha      | ~100%             | ~300% (cortical)     | ~300%                |
| Gamma      | ~100%             | ~300% (cortical)     | ~300%                |
| PFC        | ~100%             | ~300% (cortical)     | ~300%                |
| Sensory    | ~100%             | ~300% (cortical)     | ~300%                |
| Body       | ~100%             | ~300% (cortical)     | ~300%                |
| Coherence  | ~100%             | ~300% (cortical)     | ~300%                |

**N1-only deep vars are the bottleneck.** Cortical vars are solved (3x boost).
Deep vars reach 16-37% of target -- meaningful but insufficient for 100% state match.

**Conclusion:** N1-only with all 5 strategies achieves roughly 85-100% of Tier 3
performance. Full state match still requires hybrid (N1 + taVNS minimum).

## Experimental Validation

### Animal model validation (pre-human)

1. **Projection mapping:** Optogenetic tracing of DLPFC->VTA pathway in primates.
   Stimulate cortical end, measure VTA DA with microdialysis.

2. **Temporal interference depth test:** Place N1-like electrode array on cortex of
   NHP. Drive two groups at f1, f2. Measure oscillation power at depth with
   implanted depth electrodes.

3. **STDP verification:** Cortical stimulation at precise timing relative to VTA
   firing. Measure synaptic strength changes over 30-minute sessions via evoked
   potential amplitude.

4. **Entrainment coupling:** Drive cortical theta at 6Hz with N1. Record hippocampal
   theta with depth electrode. Compute coherence as function of N1 electrode count
   and spatial pattern.

5. **Insular autonomic test:** Stimulate anterior insula with N1. Measure heart rate
   variability, pupil dilation, and blood catecholamines as proxy for vagal
   activation. Compare to direct taVNS.

### Human validation sequence

1. N1 implant patients (epilepsy monitoring) -- measure scalp correlates of deep
   activity during cortical stimulation.
2. Combined N1 + fMRI to verify deep structure activation patterns.
3. Neurochemical PET imaging before/after N1 deep-driving protocols.
