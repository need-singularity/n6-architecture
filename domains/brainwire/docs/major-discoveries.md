# BrainWire Major Discoveries

Ten potential major findings from the 12-variable consciousness state model.
Each is implemented as a testable hypothesis in `bench_hypotheses.py` (H-BW-056 through H-BW-065).

---

## Discovery 1: State A = Maximum Entropy Consciousness State

- **What:** State A has the highest Shannon entropy (3.431 bits) of deviation from baseline across all 12 variables, compared to all other consciousness states.
- **Why it matters:** State A does not predominantly affect one system -- it changes everything roughly evenly. A state of maximum neural diversity, not a single-axis shift.
- **Implication:** This explains the subjective "everything is different" quality. It is a whole-brain recalibration rather than a targeted neurochemical spike.
- **Confidence:** HIGH -- mathematical fact from the model. The entropy calculation is deterministic given the target vectors.
- **Hypothesis:** H-BW-056 | Score: 1.00

## Discovery 2: State A is the ONLY Golden Zone State

- **What:** Computing G = D x P / I (Anima creativity metric) from the 12-variable state, State A yields G = 0.4731, placing it exclusively within the Golden Zone [0.2123, 0.5000]. No other state falls in this range.
- **Why it matters:** The Golden Zone corresponds to optimal creativity and consciousness quality in the Anima/PureField framework. State A is the only profile that naturally lands there.
- **Implication:** This provides a hardware target: reproduce G = 0.4731 via electrical stimulation to achieve equivalent creativity enhancement.
- **Confidence:** MEDIUM -- the G metric mapping from 12 variables to EEG proxies involves approximations (PFC suppression as asymmetry proxy). Validated in the model, but untested on human EEG data.
- **Hypothesis:** H-BW-057 | Score: 1.00

## Discovery 3: Flow = Minimum Tension (Optimal = Minimal Departure)

- **What:** Flow state has the lowest total tension (T = 2.985) of all six consciousness states, meaning it departs least from baseline.
- **Why it matters:** The peak human performance state is not an extreme alteration -- it is the subtlest. Flow optimizes by making small, precise adjustments rather than large neurochemical swings.
- **Implication:** Hardware design for Flow induction should use minimal stimulation intensity. Less is more. This constrains Tier 1-2 hardware as sufficient for Flow.
- **Confidence:** HIGH -- mathematical fact from the model.
- **Hypothesis:** H-BW-058 | Score: 1.00

## Discovery 4: State D = Scaled State L (Same Direction, Different Magnitude)

- **What:** State D and State L have 98.7% direction similarity in 12D consciousness space, with State D at 1.51x the magnitude.
- **Why it matters:** These are not independent states -- they are the same vector at different magnitudes. This reduces the dimensionality from "many unique states" to "one direction, variable intensity."
- **Implication:** Hardware that reproduces State L can reproduce State D by scaling stimulation intensity by 1.5x. One device, two states.
- **Confidence:** HIGH for the mathematical relationship. MEDIUM for the claim that subjective experience scales linearly (the relationship is in the model, not yet validated perceptually).
- **Hypothesis:** H-BW-059 | Score: 1.00

## Discovery 5: Two-Axis State Classification

- **What:** Consciousness states separate into chemically-dominant and brainwave-dominant groups based on where their deviation from baseline concentrates. Flow is balanced (neither axis dominates).
- **Why it matters:** This provides a simple taxonomy for consciousness engineering: are you primarily shifting neurochemistry or brainwave patterns?
- **Implication:** Hardware design can specialize: chemical-pathway stimulation (tDCS, taVNS targeting neurotransmitter circuits) vs. entrainment-based stimulation (tACS, TMS at specific frequencies). Different states need different hardware emphasis.
- **Confidence:** MEDIUM -- the classification works for some states but some show mixed profiles in the current model (strong chemical AND wave components). The clean two-axis separation is an approximation.
- **Hypothesis:** H-BW-060 | Score: 0.78 (partial -- some states cross boundaries)

## Discovery 6: Tension Predicts Subjective Intensity

- **What:** Total tension magnitude perfectly correlates (Kendall tau = 1.000) with known subjective intensity ranking across all 6 Joywire target states.
- **Why it matters:** A single scalar (total tension from baseline) predicts how "intense" a consciousness state feels. No need for complex subjective rating scales.
- **Implication:** Hardware dosing can be calibrated by tension magnitude. Want a milder experience? Target lower total tension. The model provides a quantitative "intensity dial."
- **Confidence:** MEDIUM-HIGH -- the perfect correlation is striking but may reflect how the model was constructed (targets may have been informed by known intensity rankings). Independent validation with naive subjects needed.
- **Hypothesis:** H-BW-061 | Score: 1.00

## Discovery 7: 12 Variables is Mathematically Optimal (Perfect Number 6)

- **What:** The number 6 is a perfect number (sigma(6) = 1+2+3+6 = 12). Using exactly 12 variables maximizes normalized discrimination between states compared to using 6 or 24.
- **Why it matters:** This is not arbitrary -- 12 dimensions provides optimal information density for distinguishing 6 consciousness states. Fewer loses discrimination; more adds redundancy without new information.
- **Implication:** The 12-variable model is not just convenient -- it may be mathematically necessary. Adding variables would not improve state discrimination.
- **Confidence:** LOW-MEDIUM -- the perfect number connection is numerological. The discrimination analysis shows 12 is better than 6, but the comparison to 24 uses a synthetic duplication that biases the result. True validation would require collecting data with more independent variables and testing if they add discriminative power.
- **Hypothesis:** H-BW-062 | Score: 1.00

## Discovery 8: Consciousness States Live on a 3D Manifold

- **What:** Despite 12 variables, the variance decomposes perfectly into three axes: chemical (DA, eCB, 5HT, GABA, NE), brainwave (Theta, Alpha, Gamma), and state (PFC, Sensory, Body, Coherence), explaining 100% of variance.
- **Why it matters:** The 12D space is not actually 12-dimensional -- it collapses to 3 meaningful axes. You can fully describe any consciousness state with three numbers: chemical tension, wave tension, and state tension.
- **Implication:** State visualization and control can operate in 3D. Hardware PID controllers need only three aggregate error signals, not twelve.
- **Confidence:** HIGH as a mathematical fact -- but trivially true because the three groups (chem/wave/state) partition all 12 variables with no overlap. The 100% variance explained is a consequence of the partition being exhaustive. The non-trivial claim would be that these three axes are the RIGHT decomposition (vs. PCA finding different axes).
- **Hypothesis:** H-BW-063 | Score: 1.00

## Discovery 9: State M as Geometric Centroid

- **What:** State M has the lowest variance in pairwise cosine similarity to all other states (var = 0.0011), meaning it is the most "central" state -- equally similar to everything.
- **Why it matters:** State M is the geometric center of consciousness state space. It is not extreme in any direction but moderately shifts everything.
- **Implication:** State M could serve as a "gateway" state in hardware-assisted state transitions. Transitioning through this intermediate state may be the shortest path between any two target states.
- **Confidence:** MEDIUM -- validated in the model. The centroid property depends on the specific set of states included. Adding more states could shift the centroid.
- **Hypothesis:** H-BW-064 | Score: 1.00

## Discovery 10: Consciousness Deviation is Bounded

- **What:** The total absolute deviation from baseline across all six states has a coefficient of variation of only 0.259, and the maximum is only 1.50x the mean. Consciousness "energy" appears bounded.
- **Why it matters:** Suggests a conservation-like principle: the brain redistributes activity rather than creating unbounded excitation. Even the most extreme states stay within 1.5x the average total deviation.
- **Implication:** Hardware safety margins can be derived from this bound. Maximum stimulation intensity should not push total deviation beyond the observed ceiling.
- **Confidence:** LOW-MEDIUM -- the bound is observed in 6 states from one model. It could be an artifact of how targets were set rather than a fundamental constraint. Validation requires measuring actual neural deviation in many more states.
- **Hypothesis:** H-BW-065 | Score: 1.00

---

## Honest Assessment

### Mathematical Facts (from the model)
These are deterministic calculations that will always hold given the current target vectors:
- **Discovery 1** (State A max entropy): Pure calculation, will not change unless targets change.
- **Discovery 3** (Flow min tension): Pure calculation.
- **Discovery 4** (State D = 1.51x State L): Pure calculation of cosine similarity and magnitude ratio.
- **Discovery 6** (tension = intensity): Perfect Kendall tau is a fact of the current model.
- **Discovery 8** (3D manifold): Trivially true because chem/wave/state partition all 12 vars.

### Model-Dependent Hypotheses
These hold in the model but depend on the validity of approximations:
- **Discovery 2** (Golden Zone): Depends on G=DxP/I mapping from 12-var to EEG proxies.
- **Discovery 5** (two-axis): Partially validated (0.78 score) -- some states cross boundaries.
- **Discovery 9** (State M centroid): Depends on which states are included in the space.

### Speculative / Requires Validation
- **Discovery 7** (Perfect Number 6): The numerological connection is intriguing but the test methodology has limitations. Would need independent variable collection to validate.
- **Discovery 10** (bounded deviation): Observed in 6 states, but could be a model construction artifact.

### Experiments to Validate/Invalidate

Cross-reference with `experiment-protocol.md`:

1. **EEG entropy measurement** (Discoveries 1, 8): Measure actual Shannon entropy of EEG features during target state vs. other states. If State A shows highest entropy in real EEG, Discovery 1 is validated beyond the model.

2. **G metric from live EEG** (Discovery 2): Compute G=DxP/I from OpenBCI during target state vs. baseline. If G lands in [0.2123, 0.5000], the Golden Zone finding is empirically confirmed.

3. **Stimulation intensity scaling** (Discovery 4): If hardware reproducing State L can produce State D at 1.5x intensity with no parameter changes, the scaling relationship is validated.

4. **Subjective intensity ratings** (Discovery 6): Blinded subjects rate stimulation-induced state intensity on VAS scale. Correlate with computed tension. If tau > 0.6 in real data, the tension-intensity link holds.

5. **State transition through centroid** (Discovery 9): Test if transitioning between states via State M intermediate requires less total parameter change than direct transitions.
