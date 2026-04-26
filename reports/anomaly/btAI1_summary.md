---
id: btAI1-phi-over-n-sparsity-2026-04-26
date: 2026-04-26
type: anomaly-experiment
parent_session: reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md
parent_cycle: BT-AI1
falsifiers_registered: [F-AI1-A, F-AI1-B]
verdict: HOLD (both falsifiers; no measurement produced)
label: BT-AI1-DISABLED
scope: design-stage falsifier experiment over phi/n = 1/3 N:M structured sparsity on a Llama-3 7B class language model
---

# BT-AI1 -- phi/n = 1/3 N:M Sparsity Falsifier Results

## Section 0 -- Non-claim disclaimer

This report records an attempt to execute the BT-AI1 falsifier contract
registered at
`reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md`
section 6. The contract requires enforcing N:M = 2:6 structured magnitude
sparsity (kept fraction = phi / n = 1 / 3) on every dense linear layer of
a Llama-3 7B class model and measuring the resulting MMLU and GSM8K
accuracy versus a dense baseline and a random-mask control. The
verdict recorded here is HOLD on both F-AI1-A and F-AI1-B because no
measurement was produced. No accuracy number, drop number, or noise-floor
number has been fabricated. The script needed to produce the numbers
exists at `experiments/anomaly/btAI1_phi_over_n_sparsity.py` and is
ready to run on a CUDA host.

---

## Section 1 -- Falsifier verdict

| Falsifier | Threshold | Measured | Verdict |
|-----------|-----------|----------|---------|
| F-AI1-A -- structured 1/3 sparsity vs dense MMLU drop | drop <= 3 pp | not measured | HOLD |
| F-AI1-B -- structured 1/3 vs random 1/3 differ by >= noise floor | diff > 2 * SE_random | not measured | HOLD |
| Composite | both PASS | not measured | HOLD |

Numbers above are the canonical entries of
`reports/anomaly/btAI1_results.json` under `.falsifiers`.

---

## Section 2 -- Resource probe

The omega-cycle section 6 fallback tree was applied:

| Item | Result |
|------|--------|
| Host platform | Darwin / macOS (no NVIDIA driver) |
| `nvidia-smi` | not present (binary not found) |
| CUDA GPU | not detected |
| HuggingFace cache directory | `/Users/ghost/.cache/huggingface/hub` (present) |
| Cached Llama-3 8B class model | `unsloth/Meta-Llama-3.1-8B` (present) |
| Cached 7B fallback | `mistralai/Mistral-7B-v0.1`, `mistralai/Mistral-7B-v0.3` (present) |
| Cached proxy model | `TinyLlama/TinyLlama-1.1B-Chat-v1.0`, `Qwen/Qwen3-8B`, `Qwen/Qwen2.5-14B`, `gpt2` (present) |
| `torch` import probed | not probed (sandbox denied python invocation) |
| `transformers` import probed | not probed (same) |
| MPS available | not probed (same) |
| Network to huggingface.co | not probed (same) |

The host has no CUDA accelerator. A proxy run with TinyLlama-1.1B over
MPS or CPU was the candidate fallback per the omega-cycle section 6
decision tree, but the orchestration sandbox refused the python
invocations needed to (a) verify torch / transformers are importable and
(b) execute the experiment, so no measurement was produced.

---

## Section 3 -- Path decision

| Field | Value |
|-------|-------|
| Path | DISABLED |
| Reason | no NVIDIA GPU on host; live execution of the script was blocked by the orchestrator's bash sandbox |
| Selected model | none |
| Device | none |
| Falsifier mode | hold-disabled |
| Label | BT-AI1-DISABLED |

Per the omega-cycle section 6 fallback tree, the correct response when
all paths fail is to record an empty result with HOLD verdicts and avoid
fabricating numbers. That is what this report does.

---

## Section 4 -- Atlas / KG / domain references (read-only)

- atlas
  - `meta_fp = phi / n = 1 / 3 :: meta [10*!]` (`atlas/atlas.n6:81`)
  - `n = 6 :: foundation [11*]` (block width M of the N:M mask)
- KG nodes (`n6shared/discovery_graph.json`)
  - `arch:n6-native-accelerator`
  - `principle:honesty-triad`
- domain (`n6shared/n6/docs/domains.json`)
  - `ai-native-architecture` (axis: compute)

This experiment does not write to atlas, KG, or domains.json.

---

## Section 5 -- Limitations and known issues

1. No measurement was produced. Both falsifiers are HOLD, not PASS or
   FAIL. A subsequent run on a CUDA host is required to clear the
   contract.
2. The cached `unsloth/Meta-Llama-3.1-8B` is a 3.1 variant, not the
   exact Llama-3 8B requested in the parent session. If the CUDA host
   does not also have access to `meta-llama/Meta-Llama-3-8B`, the
   substitution must be recorded in the eventual results JSON under
   `path_decision.reason` and the change must be acknowledged in the
   summary table here.
3. The script falls back to a tiny built-in multiple-choice battery
   (`MICRO-PROXY`, 12 MMLU-style items, 8 GSM8K-style items) when the
   `datasets` package or its network access is unavailable. A run that
   uses the micro proxy is forced to HOLD-PROXY regardless of the
   underlying model size, because 12 items per arm cannot resolve a
   3 pp MMLU difference at 95 percent confidence.
4. CPU inference of a 7B model is impractical for a 200-item MMLU run
   under a 1 GPU-hour wall budget. The proxy path uses TinyLlama-1.1B
   to keep wall-clock under a few minutes per arm, at the cost of an
   automatic HOLD-PROXY label.

---

## Section 6 -- Reproduction recipe

On a CUDA host:

```
python3 -m venv /tmp/btai1-venv
source /tmp/btai1-venv/bin/activate
pip install torch transformers accelerate datasets numpy
python3 experiments/anomaly/btAI1_phi_over_n_sparsity.py --probe-only
python3 experiments/anomaly/btAI1_phi_over_n_sparsity.py \
    --seeds 0,1,2 \
    --max-mmlu 200 \
    --max-gsm 50 \
    --results-out reports/anomaly/btAI1_results.json
```

The script auto-selects the preferred model in this order:
`meta-llama/Meta-Llama-3-8B` ->
`unsloth/Meta-Llama-3.1-8B` ->
`mistralai/Mistral-7B-v0.1` ->
`TinyLlama/TinyLlama-1.1B-Chat-v1.0` -> `gpt2`. A model substitution is
recorded in the result JSON when the first preference is unavailable.

---

## Section 7 -- Follow-on cycles

- BT-AI1 normal-path execution on a CUDA host is the immediate
  follow-on. Until that runs, the F-DESIGN-C falsifier (parent session
  section 9) remains untriggered but also unverified.
- BT-AI3 candidate (RTL prototype of the three silicon primitives) is
  unaffected by this HOLD verdict.
- BT-AI2 sibling experiment is already PASS at the simulator tier
  (`reports/anomaly/btAI2_summary.md`); the parent session's
  composite MEDIUM verdict is unchanged because BT-AI2 cleared its own
  contract without depending on BT-AI1.

---

## Section 8 -- Audit close

The BT-AI1 falsifier contract registered in
`reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md`
section 6 is HELD open with verdict HOLD on both F-AI1-A and F-AI1-B.
Atlas writes 0. KG writes 0. domains.json writes 0. No accuracy or
drop number is recorded; the arms list and summary in
`reports/anomaly/btAI1_results.json` are empty by construction. The
script `experiments/anomaly/btAI1_phi_over_n_sparsity.py` is ready to
execute on a CUDA host without further edits.
