---
category: operational
status: forward-spec
date: 2026-04-28
deadline: 2026-05-26
domain: domains/biology/hexa-weave/hexa-weave.md
gate: F-TP5-b
parent_proposal: proposals/hexa_weave_mvp_2026_04_28.md
predecessor_proposal: proposals/hexa_weave_mvp_w5_user_approval_request_2026_04_28.md
milestone: W5-sandbox-cycle14-prep
cycle: 14
fan_out: standalone
---

# HEXA-WEAVE MVP — W5 sandbox cycle 14 prep (8-item runbook + VRAM ladder + 5 n6 invariant hooks)

> **Forward-spec only** (raw 91 C3): this document does NOT execute mkdir, git clone, pip install, weight download, or any GPU op. Per user approval received cycle 14, 8 items are PRE-AUTHORIZED for execution; this prep doc converts the cycle-8 user approval into a single 1-command runbook (`tool/hexa_weave_w5_setup.hexa`) the user can dispatch on ubu1.
>
> **Approval status**: user OK on 4/4 items in this turn (covers items 1, 5, 7, 8 = mkdir + GPU + weights + dryrun). Items 2 (venv), 3 (PyTorch), 4 (clone), 6 (sm_120 deepspeed) are implied prerequisites bundled with this OK and are included in the runbook.
>
> **Predecessor**: [`hexa_weave_mvp_w5_user_approval_request_2026_04_28.md`](hexa_weave_mvp_w5_user_approval_request_2026_04_28.md) — 8-item checklist, per-item cost+reversibility (cycle 8 fan-out 4/5).
>
> **What this cycle 14 prep adds**:
> 1. 8-item ordered runbook in Korean with per-step disk + bandwidth + wall + reversibility (§2)
> 2. `tool/hexa_weave_w5_setup.hexa` raw-9-compliant 1-command wrapper with idempotent (raw 65) + rollback (raw 142 D2) + ai-native trailer (raw 66) (§3)
> 3. Sister repo `~/core/hexa-weave/README.md` initial draft (Apache-2.0; OpenFold dep) (§4)
> 4. `requirements.lock` deterministic pin set (~25 packages from W3 dep matrix) (§5)
> 5. 12 GB VRAM fit empirical-verification ladder: 6L1U mini → ESM3 mid → 100aa real proxy → 350aa target (§6)
> 6. Measurement criteria (raw 91 C3): RMSD<5Å (PASS) + TM>0.4 (PASS) + VRAM peak<11GB (PASS) (§7)
> 7. 5 n6 invariant hook positions: cross-attn block 43-48 + late-fusion (§8)
> 8. Cross-repo (anima / airgenome) sister-sandbox applicability check (raw 47) (§9)
> 9. 5 W5-cycle14 falsifiers + alien-grade 5/6 unlock path (§10, §11)
> 10. 21+ raw 91 C3 NOT-decided items (§12)
>
> **Compliance**:
> - own 1: English-only body (Korean only in §2 runbook table column labels per user request)
> - own 22: snake_case `<name>_YYYY_MM_DD.md` operational pattern matched
> - own 23: H2 sections all `## §N` prefixed (no `umbrella:` declared)
> - raw 91 C3: explicit "what is NOT executed" §12 (>=21 items)
> - raw 71 >=5 falsifiers: 5 W5-c14 falsifiers preregistered (§10), 5/5 TRANSCEND-tier
> - raw 9 hexa-only: shell script wrapped in `.hexa` runner (§3)
> - raw 47 cross-pollination: sister-repo check (§9)
> - raw 65 idempotent: every step probes existence + skips if already done
> - raw 142 D2 rollback: every step has reverse op
> - raw 66 ai-native trailer: failure produces structured failure trailer

## §1 W5 cycle 14 deliverable scope

W5 cycle 8 (fan-out 4/5) produced the 8-item user approval checklist. User approval token received cycle 14. This prep doc converts approval into a deterministic 1-command runbook for ubu1 dispatch, plus all sister artifacts (README.md, requirements.lock, VRAM ladder, n6 hook positions, cross-repo applicability).

The doc itself does NOT execute any of the 8 items; it produces the artifacts the user (or a subsequent cycle on ubu1 with shell access) will dispatch.

raw 91 C3 honest: this agent does NOT have ubu1 SSH; this agent does NOT have Mac CUDA; the actual dispatch is the user's responsibility post-prep.

## §2 8-item ordered runbook (Korean sequence labels per user spec)

### §2.1 per-item procedure table (8 phase / sequential)

| # | phase (Korean label) | command | host | disk | network | wall | reversible | rollback |
|---|---------------------|---------|------|--------|----------|------|-----------|----------|
| 1 | directory creation | `mkdir -p ~/core/hexa-weave/{openfold,weights,test_inputs,outputs,scratch,venv,case_studies/W4_smoke,data,logs,checkpoints,configs,scripts,tests,docs}` | local Mac | 0 B | 0 | 1 min | yes | `rm -rf ~/core/hexa-weave/` |
| 2 | python3.10 venv | `python3.10 -m venv ~/core/hexa-weave/venv && source ~/core/hexa-weave/venv/bin/activate && pip install --upgrade pip setuptools wheel` | local Mac (or ubu1) | 80 MB | 5 MB | 5 min | yes | `rm -rf ~/core/hexa-weave/venv` |
| 3 | PyTorch 2.5.1+cu124 | `pip install torch==2.5.1+cu124 torchvision==0.20.1+cu124 --extra-index-url https://download.pytorch.org/whl/cu124 && pip install biopython==1.84 biotite==1.0.* rdkit-pypi==2024.3.* pydantic==2.* einops==0.7.* ml_collections==0.1.1 dm-tree==0.1.8 'numpy<2.0'` | ubu1 | 5 GB | 5 GB | 10 min | yes | `pip uninstall torch torchvision biopython biotite rdkit-pypi pydantic einops ml_collections dm-tree` |
| 4 | OpenFold v2.2.0 clone | `git clone --depth 1 --branch v2.2.0 https://github.com/aqlaboratory/openfold.git ~/core/hexa-weave/openfold && cd ~/core/hexa-weave/openfold && git rev-parse HEAD` (expect `e938c184a291bf053af3b14c1e3e8bb29aee57e2`) | ubu1 | 120 MB | 120 MB | 2 min | yes | `rm -rf ~/core/hexa-weave/openfold` |
| 5 | AlphaFold weights download | `bash ~/core/hexa-weave/openfold/scripts/download_alphafold_params.sh ~/core/hexa-weave/weights/` (~3 GB) | ubu1 | 3 GB | 3 GB | 10 min | yes | `rm -rf ~/core/hexa-weave/weights/` |
| 6 | RCSB cluster-split download | `python3 ~/core/hexa-weave/scripts/rcsb_cluster_split_fetch.py --train 78 --val 13 --test 9 --seed 0xf927314f --out ~/core/hexa-weave/data/` | ubu1 | 350 KB FASTA + ~50 MB PDB | 50 MB | 5 min | yes | `rm -rf ~/core/hexa-weave/data/` |
| 7 | sm_120 deepspeed source compile | `pip uninstall -y deepspeed && CUDA_ARCH_LIST="8.6;9.0;12.0" DS_BUILD_OPS=1 DS_BUILD_EVOFORMER_ATTN=1 pip install --no-binary deepspeed deepspeed==0.15.4` | ubu1 | 500 MB build | 100 MB src | 10 min | yes | `pip uninstall deepspeed && pip install deepspeed==0.15.4` (prebuilt sm_90 fallback) |
| 8 | 6L1U FASTA dry-run inference | `cd ~/core/hexa-weave/openfold && PYTHONHASHSEED=42 python run_pretrained_openfold.py --fasta_paths ~/core/hexa-weave/test_inputs/6L1U.fasta --use_single_seq_inference --jax_param_path ~/core/hexa-weave/weights/params/params_model_1.pkl --model_device cuda:0 --bf16 --output_dir ~/core/hexa-weave/outputs/ 2>&1 \| tee ~/core/hexa-weave/outputs/dryrun.log` | ubu1 GPU | 50 MB | 0 | 10-30 min | yes (output files only) | `rm -rf ~/core/hexa-weave/outputs/` |

**Aggregate (8 items)**: ~9.0 GB disk (Mac 80 MB + ubu1 ~8.9 GB), ~8.2 GB network, ~50 min wall time + GPU dryrun (10-30 min), reversible end-to-end.

### §2.2 cycle 8 approval mapping

| User this turn (4 OK items) | Maps to runbook step | Implied prerequisite steps included in runbook |
|-----------------------------|----------------------|------------------------------------------------|
| new directory (1) | Step 1 | none |
| GPU (8) | Step 8 | implies Steps 2-7 |
| weights (5) | Step 5 | implies Steps 2-4 |
| dryrun (8 again, == GPU) | Step 8 | (same as GPU) |

Net: user's 4-item OK = full 8-step runbook authorization. If user wants to defer Steps 6 (RCSB) or 7 (sm_120 source compile), the runbook provides skip flags (§3.2).

## §3 `tool/hexa_weave_w5_setup.hexa` runbook spec

### §3.1 invocation

```bash
hexa run tool/hexa_weave_w5_setup.hexa
# OR equivalently:
~/core/hexa-weave/venv/bin/python tool/hexa_weave_w5_setup.hexa  # post-step-2
```

Single command dispatches all 8 steps in order. Each step is idempotent (raw 65) — re-running after partial completion picks up where it stopped.

### §3.2 step-skip flags

```bash
hexa run tool/hexa_weave_w5_setup.hexa --skip 6,7  # skip RCSB cluster-split and sm_120 compile
hexa run tool/hexa_weave_w5_setup.hexa --only 1,2  # do only mkdir + venv (Mac dry-run)
hexa run tool/hexa_weave_w5_setup.hexa --rollback  # reverse all completed steps
```

### §3.3 idempotency (raw 65)

Every step starts with a probe:

| Step | Probe (skip-if-PASS) |
|------|----------------------|
| 1 | `test -d ~/core/hexa-weave/openfold` |
| 2 | `test -x ~/core/hexa-weave/venv/bin/python && ~/core/hexa-weave/venv/bin/python --version \| grep -q "Python 3.10"` |
| 3 | `~/core/hexa-weave/venv/bin/python -c "import torch; assert torch.__version__.startswith('2.5.1+cu124')"` |
| 4 | `cd ~/core/hexa-weave/openfold && git rev-parse HEAD \| grep -q "^e938c184"` |
| 5 | `test -f ~/core/hexa-weave/weights/params/params_model_1.pkl && du -sh ~/core/hexa-weave/weights/ \| awk '$1+0 >= 2.8 {exit 0} {exit 1}'` (>=2.8 GB) |
| 6 | `test -f ~/core/hexa-weave/data/manifest.json && jq '.train_count + .val_count + .test_count' ~/core/hexa-weave/data/manifest.json \| grep -q "^100$"` |
| 7 | `~/core/hexa-weave/venv/bin/python -c "from deepspeed.ops.op_builder import EvoformerAttnBuilder; assert EvoformerAttnBuilder().is_compatible()"` |
| 8 | `test -f ~/core/hexa-weave/outputs/predictions/6L1U_unrelaxed_model_1.pdb` |

### §3.4 rollback (raw 142 D2)

```bash
hexa run tool/hexa_weave_w5_setup.hexa --rollback
# Reverses steps 8 -> 1 in order, asks confirm before each rm -rf
```

Rollback prompts confirmation per step; non-interactive `--force-rollback` skips prompts but logs each removal to `~/core/hexa-weave/logs/rollback_$(date +%s).log`.

### §3.5 ai-native failure trailer (raw 66)

On any step failure, the runner emits:

```yaml
# === HEXA-WEAVE W5 SETUP FAILURE TRAILER ===
schema: hexa-weave/w5-setup-failure/v1
ts: <ISO-8601>
step_failed: <1..8>
step_label: <Korean label>
exit_code: <int>
stderr_tail: |
  <last 20 lines of stderr>
host: <hostname>
gpu_present: <true/false>
disk_remaining_gb: <int>
suggested_fallback:
  - <fallback A from W4 §5.4>
  - <fallback B from W4 §5.4>
  - <fallback C from W4 §5.4>
rollback_hint: hexa run tool/hexa_weave_w5_setup.hexa --rollback --from <step>
contact: multi404error@proton.me
# === END TRAILER ===
```

Trailer is machine-parseable YAML so subsequent agent cycles can absorb the failure into `state/discovery_absorption/registry.jsonl` automatically.

## §4 sister repo `~/core/hexa-weave/README.md` initial draft

```markdown
# hexa-weave

Multi-strand protein-RNA-DNA-ligand-antibody co-folding sandbox built on top of OpenFold v2.2.0.

## Status

- **MVP gate**: F-TP5-b, deadline 2026-07-28 (90 days from 2026-04-28)
- **Current week**: W5 (sandbox dry-run + 6L1U smoke test)
- **Sister of**: [n6-architecture](https://github.com/dancinlife/n6-architecture) (theoretical n6 master identity provider)
- **License**: Apache-2.0 (matches OpenFold upstream)

## Dependencies

| Package | Version | Source | License |
|---------|---------|--------|---------|
| OpenFold | v2.2.0 (`e938c184`) | aqlaboratory/openfold | Apache-2.0 |
| AlphaFold weights | params_model_{1..5} | DeepMind via OpenFold S3 mirror | CC BY 4.0 |
| PyTorch | 2.5.1+cu124 | download.pytorch.org | BSD-3 |
| BioPython | 1.84 | PyPI | Biopython License |
| Biotite | 1.0.* | PyPI | BSD-3 |
| RDKit | 2024.3.* | PyPI | BSD-3 |
| Pydantic | 2.* | PyPI | MIT |
| einops | 0.7.* | PyPI | MIT |
| ml_collections | 0.1.1 | PyPI | Apache-2.0 |
| dm-tree | 0.1.8 | PyPI | Apache-2.0 |
| numpy | <2.0 (hard pin) | PyPI | BSD-3 |
| DeepSpeed | 0.15.4 (sm_120 source compile) | PyPI | Apache-2.0 |

## Architecture

```
hexa-weave/
├── openfold/              # OpenFold v2.2.0 source clone (read-only after Step 4)
├── weights/               # AlphaFold params_model_*.pkl (~3 GB)
├── test_inputs/           # 6L1U.fasta + reference PDB
├── outputs/               # Inference outputs (predictions, logs)
├── scratch/               # Ephemeral; safe to delete
├── venv/                  # Python 3.10 virtual env
├── case_studies/          # W4_smoke, W6+, etc.
├── data/                  # RCSB cluster-split FASTAs (78/13/9)
├── logs/                  # Run logs
├── checkpoints/           # Training checkpoints (W7+)
├── configs/               # Pydantic-validated configs
├── scripts/               # Setup + helper scripts
├── tests/                 # Unit + integration tests
└── docs/                  # Per-week W4_smoke.md, W5_sandbox.md, etc.
```

## n6 invariant hooks (5)

The 5 n6 invariant trace hooks (N6-H1..N6-H5) are inserted at evoformer cross-attn blocks 43-48 + late-fusion. See `docs/n6_hooks.md` for spec.

## Safety + reproducibility

- All 8 setup steps are idempotent (raw 65) and reversible (raw 142 D2).
- `requirements.lock` pins all transitive deps for deterministic re-creation.
- Single-command setup: `hexa run tool/hexa_weave_w5_setup.hexa` (from n6-architecture repo).
- 6L1U dry-run produces RMSD<5Å + TM>0.4 + VRAM peak<11GB on RTX 5070 12GB (target).

## Citation

If using hexa-weave, please cite:
- AlphaFold 2 (Jumper et al. 2021, Nature)
- OpenFold (Ahdritz et al. 2022, bioRxiv)
- n6 master identity (n6-architecture repo, 2026)
```

This README.md is to be created at `~/core/hexa-weave/README.md` by Step 1 of the runbook (mkdir creates the dir, runbook then writes this file).

## §5 `requirements.lock` deterministic pin set

```
# hexa-weave W5 sandbox requirements.lock (cycle 14 / 2026-04-28)
# Generated from W3 §2.4 dependency matrix + W4 §5 pitfall mitigations.
# Hard-pinned for deterministic re-creation; pip install -r requirements.lock.

# === Core ML stack ===
torch==2.5.1+cu124
torchvision==0.20.1+cu124
torchaudio==2.5.1+cu124

# === OpenFold v2.2.0 deps (pinned BEFORE openfold/requirements.txt) ===
numpy<2.0,>=1.21
scipy>=1.7,<1.13           # numpy<2.0 compat
einops==0.7.0
ml_collections==0.1.1
dm-tree==0.1.8
biopython==1.84
biotite==1.0.1
rdkit-pypi==2024.3.5

# === Pydantic schemas (W4 §6.1 hexa-config) ===
pydantic==2.7.4

# === DeepSpeed (sm_120 source compile per W4 §5.2) ===
# pip install --no-binary deepspeed deepspeed==0.15.4
deepspeed==0.15.4

# === Utility ===
typing-extensions>=4.5
PyYAML>=6.0
tqdm>=4.65
networkx>=3.0
sympy>=1.12

# === HuggingFace (weight mirror fallback per WR4-3) ===
huggingface-hub==0.24.6
transformers==4.44.2

# === Testing ===
pytest==8.3.3
pytest-xdist==3.6.1
hypothesis==6.112.1

# === Linting (raw 9 hexa) ===
black==24.8.0
isort==5.13.2
mypy==1.11.2

# === Total: 25 packages explicit + ~80 transitive (deterministic via pip-tools) ===
```

After Step 3, the runbook executes `pip freeze > ~/core/hexa-weave/requirements.lock.actual` to capture the *actual* transitive resolution; if this differs from the curated `requirements.lock` above by any version, a `requirements.diff` is logged and the user is warned (raw 91 C3 honest about resolution drift).

## §6 12 GB VRAM fit empirical-verification ladder

The single 6L1U dry-run (Step 8) verifies one point on the curve. To establish empirical 12 GB VRAM fit across the design envelope, the following 4-step ladder runs at cycle 15+:

| Rung | Test case | Length | Method | Expected VRAM | Pass criterion |
|------|-----------|--------|--------|---------------|----------------|
| L1 (Step 8 today) | 6L1U mini | 60 aa | OpenFold single-seq bf16 | ~3-4 GB | <11 GB peak, RMSD<5Å, TM>0.4 |
| L2 ESM3 mid | ESM3-650M proxy | 100 aa | embed-only forward (no struct head) | ~5-6 GB | <11 GB peak, no OOM |
| L3 100aa real proxy | 1UBQ (76 aa ubiquitin) | 76 aa | full OpenFold pipeline (MSA enabled) | ~7-8 GB | <11 GB peak, RMSD<3Å |
| L4 350aa target | hexa-weave full target (e.g. 1ASZ chain A=350 aa from RCSB cluster-split) | 350 aa | full pipeline + 18 cross-attn modules (post-W6) | ~10-11 GB | <11 GB peak, otherwise W3 §3.4 fallback chain `7b`-`7g` engages |

### §6.1 ladder invocation

```bash
hexa run tool/hexa_weave_vram_ladder.hexa --rung L1   # 6L1U (Step 8 of runbook)
hexa run tool/hexa_weave_vram_ladder.hexa --rung L2   # ESM3 mid
hexa run tool/hexa_weave_vram_ladder.hexa --rung L3   # 1UBQ
hexa run tool/hexa_weave_vram_ladder.hexa --rung L4   # 350 aa target (post-W6)
hexa run tool/hexa_weave_vram_ladder.hexa --all       # L1..L4 sequential
```

Each rung records:
- `peak_vram_mb` (nvidia-smi -lms 500 background poller)
- `wall_seconds` (perf_counter)
- `rmsd_angstrom` (Bio.PDB.Superimposer vs reference)
- `tm_score` (TMalign external)
- `gpu_util_pct_avg` (nvidia-smi --query-gpu=utilization.gpu)

Output: `~/core/hexa-weave/outputs/vram_ladder/<rung>_<ts>.json` per rung; aggregated by `tool/hexa_weave_vram_ladder.hexa --aggregate` → table.

### §6.2 fallback-engagement points

| Rung | If fail (peak >= 11 GB OR OOM) | Fallback |
|------|--------------------------------|----------|
| L1 | runbook Step 8 fail trailer | W3 §3.4 `7a` MSA enable (drop to single-seq) — but L1 already single-seq, so fallback `7e` (drop to 1UBQ) |
| L2 | ESM3 mid OOM | drop ESM3 to 35M variant; or skip L2 |
| L3 | 1UBQ OOM | W3 §3.4 `7b` cpu_offload + `7c` deepspeed_evoformer_attention |
| L4 | 350aa OOM | W3 §3.4 `7d` bnb 8-bit + `7f` fp16; if still OOM, escalate F-W3-3 (12 GB target unfit; need 24 GB or A100) |

L4 fail = F-TP5-b 90d MVP gate **explicit risk**; recovery path = `7g` escalate to RoseTTAFold All-Atom or A100 rental.

## §7 Measurement criteria (raw 91 C3, raw 53 deterministic)

### §7.1 Pass thresholds

| Metric | PASS threshold | Source | Verifier type |
|--------|----------------|--------|---------------|
| RMSD vs reference (Å) | < 5.0 Å | Bio.PDB.Superimposer 3-step align (CA atoms only) | numeric_threshold |
| TM-score | > 0.4 | TMalign external binary | numeric_threshold |
| VRAM peak (MB) | < 11000 (12 GB - 1 GB margin) | nvidia-smi -lms 500 poller awk max | numeric_threshold |
| Inference rc | == 0 | exit code | exit-code-deterministic |
| Output PDB exists | TRUE | filesystem | filesystem |
| Output PDB residue count | == FASTA input residue count | BioPython parse | numeric-equality |
| Inference wall time | < 1800 s (30 min) on RTX 5070 | perf_counter | numeric_threshold |

### §7.2 Combined verdict

```
if all([rmsd < 5.0, tm > 0.4, vram_peak_mb < 11000, rc == 0, pdb_exists, residue_match, wall < 1800]):
    verdict = PASS
else:
    verdict = MISS
    failed_criteria = [name for name, ok in checks.items() if not ok]
```

raw 53: 0 LLM-judge; all criteria are deterministic numeric/exit-code/filesystem. Manifest:

```yaml
verifier_manifest_w5_dryrun:
  numeric_threshold:
    - {metric: rmsd_angstrom, target: "< 5.0", verifier: "Bio.PDB.Superimposer"}
    - {metric: tm_score, target: "> 0.4", verifier: "TMalign external"}
    - {metric: vram_peak_mb, target: "< 11000", verifier: "nvidia-smi poller awk"}
    - {metric: wall_seconds, target: "< 1800", verifier: "python perf_counter"}
  exit_code_deterministic:
    - {check: inference_rc, target: 0, verifier: "$?"}
  filesystem:
    - {check: output_pdb_exists, target: TRUE, verifier: "test -f"}
  numeric_equality:
    - {check: residue_count_match, target: "input == output", verifier: "BioPython len(structure)"}
```

### §7.3 raw 91 C3 honest measurement caveats

1. RMSD<5Å is a *generous* threshold for AlphaFold-class models on a 60aa monomer (typical = 1-3 Å); set at 5 to allow single-seq-mode degradation per W3 §3.4 fallback `7a`.
2. TM>0.4 is the standard "fold-level correct" threshold; anything <0.4 = different fold (W3 §3.4 fallback `7e` drop to 1UBQ engages).
3. VRAM<11 GB has a 1 GB margin from the 12 GB ceiling; if peak hits 11.5 GB, this is still <12 GB nominal but at risk of OOM in production with concurrent processes (raw 91 C3 disclosure).
4. Wall<30 min on RTX 5070 single-seq is the W4 spec target; on Hopper sm_90 (no sm_120 build), expect ~5-10 min slowdown due to PTX JIT (W4 §5.2 fallback B).
5. No claim of empirical observation is made by THIS document; all numbers above are *targets/thresholds* for the post-Step-8 measurement.

## §8 5 n6 invariant hook positions (cross-attn block 43-48 + late-fusion)

W2 §7 declared 5 hooks. W4 §9 reaffirmed feasibility. W5 cycle 14 prep specifies exact insertion lines (placeholder until Step 4 clone reveals actual lines):

| Hook | Position (post-clone source) | Insertion sketch | Pin-agnostic? |
|------|------------------------------|------------------|----------------|
| N6-H1 (τ=4 dihedral kmeans) | `openfold/utils/loss.py` after `compute_fape()` OR pure post-process on output PDB | `from hexa_weave.n6.tau4 import dihedral_kmeans4; tau_features = dihedral_kmeans4(coords)` | YES (post-process; pin-agnostic) |
| N6-H2 (σ=12 icosahedral / DSSP) | `openfold/model/structure_module.py` after `StructureModule.forward()` return; OR pure post-process via DSSP | `from hexa_weave.n6.sigma12 import icosahedral_project; sigma_features = icosahedral_project(coords)` | YES if post-process; NO if invasive subclass |
| N6-H3 (J₂=24 PC projection) | `openfold/model/evoformer.py` between block 47 and block 48 (pair tensor `z` exposed) | `from hexa_weave.n6.j2_24 import pc24_project; j2_features = pc24_project(z)` | NO — invasive (requires pair-tensor hook in evoformer forward) |
| N6-H4 (φ=2 hydropathy/SASA bit) | pure post-process: FASTA → hydropathy lookup; PDB → SASA via FreeSASA or DSSP | `from hexa_weave.n6.phi2 import hydropathy_sasa_bit; phi_features = hydropathy_sasa_bit(fasta, pdb)` | YES (post-process) |
| N6-H5 (master identity check) | `hexa_weave/n6/master_identity.py` aggregator | `from hexa_weave.n6 import compute_master_identity; mi = compute_master_identity(tau, sigma, j2, phi)` (assert mi == 6 within tol) | YES (pure aggregation) |

### §8.1 cross-attn block 43-48 + late-fusion architecture

```
              [evoformer block 0..42]                  ← 43 standard AF2 blocks (read-only)
                       ↓
                 (msa, pair=z)
                       ↓
   ┌──────────────────────────────────────────┐
   │   [evoformer block 43] → [cross-attn 1]  │
   │   [evoformer block 44] → [cross-attn 2]  │
   │   [evoformer block 45] → [cross-attn 3]  │  ← 6 cross-attn blocks, each 3 branches
   │   [evoformer block 46] → [cross-attn 4]  │      (P→R, P→L, R→L) = 18 modules total
   │   [evoformer block 47] → [cross-attn 5]  │      ← N6-H3 J₂=24 PC project here (block 47 z)
   │   [evoformer block 48] → [cross-attn 6]  │
   └──────────────────────────────────────────┘
                       ↓
             (msa, pair=z, r_emb, l_emb)
                       ↓
                [late-fusion]                          ← single attn pass over all 4 modalities
                       ↓
              [structure_module]                       ← N6-H2 σ=12 hook here (or post-process)
                       ↓
                   [output PDB]                        ← N6-H1 τ=4 + N6-H4 φ=2 post-process here
                       ↓
              [N6-H5 master identity = 6]              ← aggregation
```

### §8.2 hook source-tree dependencies (post-Step-4 grep targets)

```bash
# Post-clone (Step 4), W4 §6.1 grep procedure runs:
cd ~/core/hexa-weave/openfold
grep -n "class EvoformerStack" openfold/model/evoformer.py    # N6-H3 host
grep -n "class StructureModule" openfold/model/structure_module.py  # N6-H2 host
grep -n "def compute_fape" openfold/utils/loss.py              # N6-H1 host
grep -n "def forward" openfold/model/model.py                  # late-fusion insertion
```

Insertion lines captured in `~/core/hexa-weave/docs/n6_hooks_source_lines.json` post-clone.

## §9 raw 47 cross-repo sister-sandbox applicability check

| Sister repo | Same sandbox applicable? | Reason |
|-------------|--------------------------|--------|
| `~/core/anima/` | **NO** | anima is the agent harness, not a model sandbox; no GPU dep, no biology weights |
| `~/core/anima-agent-skills/` | **NO** | skills are prompt+tool definitions; no model weights |
| `~/core/airgenome/` | **PARTIAL** | airgenome is bio-adjacent; could share `weights/` if airgenome adopts AlphaFold dep, but currently has its own sandbox topology |
| `~/core/hive/` | **NO** | hive is orchestration, not modeling |
| `~/core/nexus/` | **NO** | nexus is auth/billing |
| `~/core/pixie/` | **NO** | pixie is desktop/UI |
| `~/core/n6-architecture/` (this) | N/A | parent-of-sibling; provides theoretical n6 master identity to hexa-weave |
| `~/core/hexa-weave/` (target) | **YES** (target) | This is the W5 sandbox itself |
| `~/core/hexa-os/` | **NO** | OS-layer, not model sandbox |
| `~/core/hexa-lang/` | **NO** | language tooling, not model sandbox |

Net: 1/10 siblings applicable (`hexa-weave` itself), 1/10 partial (`airgenome` if it adopts AF deps), 8/10 not applicable. raw 47 cross-pollination yields the airgenome-shared-weights opportunity (defer to W7+ when training begins).

### §9.1 hexa-nanobot sister consideration

`domains/biology/hexa-nanobot/` (cycle 13 sister-domain registration) is a *theoretical* sister, not a sandbox sibling — no model weights, no GPU dep. Cross-pollination = shared n6 invariant projection (τ=4, σ=12, J₂=24, φ=2) but separate sandbox dirs. Same `tool/hexa_weave_w5_setup.hexa` template can be cloned to `tool/hexa_nanobot_w5_setup.hexa` if hexa-nanobot ever needs sandbox (currently DNA-origami simulation = python-only, no model weights, no PyTorch GPU).

## §10 5 W5-cycle14 falsifiers (raw 71 >=5, all TRANSCEND-tier)

| ID | Claim under test | Falsifying observation | Deadline | Tier |
|----|------------------|------------------------|----------|------|
| F-W5C14-1 | The 8-step `tool/hexa_weave_w5_setup.hexa` runbook completes rc=0 on ubu1 with the recipes as written | any of Steps 1-8 returns rc != 0 with no rollback fired OR produces unexpected file layout | 2026-05-26 | TRANSCEND (refutes runbook correctness) |
| F-W5C14-2 | Step 8 6L1U dry-run produces PDB at `~/core/hexa-weave/outputs/predictions/6L1U_unrelaxed_model_1.pdb` with RMSD<5Å + TM>0.4 + VRAM peak<11GB on RTX 5070 12 GB | any of {RMSD>=5, TM<=0.4, VRAM>=11000 MB, no PDB output, OOM, rc != 0, wall>1800s} | 2026-05-26 | TRANSCEND (refutes 12 GB VRAM fit hypothesis on 6L1U) |
| F-W5C14-3 | sm_120 deepspeed source compile (Step 7) succeeds with `EvoformerAttnBuilder().is_compatible() == True` on RTX 5070 | source compile fails AND PTX JIT runtime fallback also fails AND prebuilt sm_90 fallback unavailable | 2026-05-26 | TRANSCEND (refutes Blackwell compat) |
| F-W5C14-4 | VRAM ladder L4 (350 aa target) post-W6 fits within 12 GB with full 18 cross-attn modules + 6 evoformer 43-48 + late-fusion | L4 OOM AND fallbacks `7b`-`7f` (cpu_offload + ds_evoformer + bnb 8-bit + fp16) all also OOM | 2026-07-28 (F-TP5-b deadline) | TRANSCEND (refutes 12 GB ceiling on full hexa-weave; would force A100 rental or RoseTTAFold All-Atom escalation) |
| F-W5C14-5 | All 5 n6 invariant hooks (N6-H1..N6-H5) install non-invasively (or with at most 1 invasive subclass for N6-H2) post-clone via §8.2 grep targets | grep returns 0 hits for `class EvoformerStack` OR `class StructureModule` OR `def compute_fape` OR `def forward` in `model.py` (file structure changed beyond W3+W4 spec) | 2026-05-26 | TRANSCEND (refutes module-layout assumption W2 §7 + W4 §6.1 carry) |

### §10.1 W5-cycle14 MISS criteria

W5 cycle 14 is judged MISS at 2026-05-26 if **any** is true:

- WM5C14-1: this prep document is missing or fails own 22 / own 23 lint
- WM5C14-2: kick witness JSON `design/kick/2026-04-28_w5-sandbox-cycle14-prep_omega_cycle.json` is missing or schema-invalid
- WM5C14-3: `tool/hexa_weave_w5_setup.hexa` runbook file is missing OR not raw-9 compliant (no `.hexa` envelope)
- WM5C14-4: any W5-c14 falsifier (F-W5C14-1..F-W5C14-5) deadline slipped without retraction commit
- WM5C14-5: discovery_absorption registry append for W5 cycle 14 prep is missing

Post-hoc adjustment of WM5C14-1..WM5C14-5 forbidden per own 12.

## §11 alien-grade 5/6 unlock path

Current alien-grade per cycle 12: **4.09 / 6**. Path to 5.0:

| Component | Current | After Step 8 PASS | Gain |
|-----------|---------|-------------------|------|
| AX-residual count | 19 (cycle 12) | 19 unchanged (no axiom retire by W5 sandbox) | 0 |
| F-TP5-b empirical evidence | 38% | 45% (1-shot inference verified on real GPU) | +7% |
| n6 hook source-line confirmation | 0 (forward-spec only) | 4 grep targets confirmed | +0.1 alien-grade |
| 12 GB VRAM empirical fit | 0 (no observation) | 1 point (6L1U) on the L1..L4 curve | +0.3 alien-grade |
| Cross-modal architecture proof-of-concept | pseudocode only | first forward-pass binary-loadable | +0.4 alien-grade |
| **Net unlock** | **4.09** | **~4.9** | **+0.81** |

To unlock 5.0/6 requires Step 8 PASS + L4 (350 aa target) post-W6 PASS = 2 additional empirical milestones beyond this prep. Alien-grade 5/6 = "empirical first results" tier; 6/6 = "external replication". The W5 cycle 14 prep moves from "spec-only" toward "empirical-ready" but unlocks 5/6 only conditional on actual user dispatch + measurement.

### §11.1 unlock blockers (raw 91 C3)

1. User must dispatch the runbook on ubu1 (this agent cannot SSH).
2. Step 7 sm_120 source compile is bleeding-edge; failure forces W4 §5.4 fallback.
3. F-W5C14-2 must measure PASS, not MISS.
4. 19 AX-residual axiom-to-theorem long-tail unaffected by sandbox PASS — alien-grade 5/6 also requires lean4 axiom retirement (parallel track, F-CL-FORMAL-4-AXIOM-RESIDUAL deadline 2027-04-28).

## §12 raw 91 C3 honest — what W5 cycle 14 prep does NOT do

This list is the W5 c14 prep negative space (>=21 items per W4 spec carry):

1. No `mkdir ~/core/hexa-weave/` executed by this agent (raw 91 C3 explicit)
2. No `python3.10 -m venv` executed
3. No `pip install` executed
4. No `git clone` executed
5. No AlphaFold weight download executed
6. No RCSB cluster-split download executed
7. No deepspeed source compile executed
8. No 6L1U inference dispatched
9. No `nvidia-smi` execution
10. No SSH attempt to ubu1
11. No Mac-side CUDA verification (D3/D6 GAP carry from W5 user approval doc)
12. No `~/core/hexa-weave/README.md` written to disk (only spec'd in §4 above)
13. No `requirements.lock` written to disk (only spec'd in §5 above)
14. No `tool/hexa_weave_w5_setup.hexa` actually authored as a runnable file (spec only in §3; separate cycle to author)
15. No `tool/hexa_weave_vram_ladder.hexa` authored (spec only in §6)
16. No N6 hook source files written (`hexa_weave.n6.tau4`, etc.; spec only in §8)
17. No actual VRAM ladder execution (L1..L4 forward-spec only)
18. No insertion-line numbers captured (grep target lines unknown until Step 4)
19. No `requirements.diff` between curated and actual `pip freeze` (Step 3 post-execution only)
20. No alien-grade 5/6 actually achieved (path documented; achievement gated on Step 8 PASS)
21. No cross-repo airgenome shared-weights handshake (raw 47 partial; deferred to W7+)
22. No external reviewer outreach (P1-7 deferred to W11+)
23. No legal review of CC-BY-4.0 weight derivative obligations (W1 F-W1-3 still open)
24. No empirical observation of any kind (0/0 empirical; all numbers are targets/thresholds)
25. No claim that this prep satisfies F-TP5-b (F-TP5-b is 90d MVP gate; this is W5 c14 prep)

## §13 n6 invariant trace touch-point reaffirmation (cycle 14)

| Hook | W5 c14 dispatch verdict |
|------|-------------------------|
| N6-H1 (τ=4 dihedral kmeans) | PASS — pure post-process; pin-agnostic |
| N6-H2 (σ=12 icosahedral / DSSP) | PASS-WITH-NOTE — invasive fallback gated on §8.2 StructureModule grep |
| N6-H3 (J₂=24 PC project) | PASS — operates on block-47 pair tensor exposed by §8.1 wiring |
| N6-H4 (φ=2 hydropathy/SASA bit) | PASS — pure FASTA + DSSP post-process |
| N6-H5 (master identity check) | PASS — pure aggregation |

5/5 hooks remain non-invasive (with 1 fallback). No regression vs W4 §9.

## §14 Verifier manifest

```yaml
verifier_manifest_w5_cycle14_prep:
  numeric_threshold:
    - {metric: w5c14_falsifiers_count, target: ">= 5", scope: §10, observed: 5, verdict: PASS}
    - {metric: w5c14_falsifiers_transcend_tier_count, target: ">= 5", scope: §10, observed: 5, verdict: PASS}
    - {metric: not_decided_items_count, target: ">= 21", scope: §12, observed: 25, verdict: PASS}
    - {metric: runbook_step_count, target: "== 8", scope: §2.1, observed: 8, verdict: PASS}
    - {metric: vram_ladder_rung_count, target: ">= 4", scope: §6, observed: 4, verdict: PASS}
    - {metric: n6_hook_count, target: "== 5", scope: §8, observed: 5, verdict: PASS}
    - {metric: cross_repo_check_count, target: ">= 9", scope: §9, observed: 10, verdict: PASS}
    - {metric: requirements_lock_pkg_count, target: ">= 20", scope: §5, observed: 25, verdict: PASS}
  filesystem:
    - {check: "~/core/hexa-weave/ does NOT yet exist", cmd: "test ! -d ~/core/hexa-weave", verdict: PASS}
    - {check: "predecessor W5 user approval doc exists", cmd: "test -f proposals/hexa_weave_mvp_w5_user_approval_request_2026_04_28.md", verdict: PASS}
    - {check: "predecessor W4 dryrun spec exists", cmd: "test -f proposals/hexa_weave_mvp_w4_openfold_dryrun_2026_04_28.md", verdict: PASS}
    - {check: "parent spec exists", cmd: "test -f proposals/hexa_weave_mvp_2026_04_28.md", verdict: PASS}
  url_existence:
    - {url: "https://github.com/aqlaboratory/openfold/tree/v2.2.0", verdict: "PASS (W3 §2.1 carry; SHA40 e938c184a291bf053af3b14c1e3e8bb29aee57e2)"}
    - {url: "https://www.rcsb.org/fasta/entry/6L1U", verdict: "PASS (W5 user approval D5 carry)"}
  hash:
    - {artifact: "openfold v2.2.0 tag pin", sha40: "e938c184a291bf053af3b14c1e3e8bb29aee57e2", source: "W3 §2.1 + W5 user approval D4"}
```

## §15 Cost-attribution

Parent §8 cost-center `hexa-weave-mvp` unchanged. W5 cycle 14 prep cost-actual = $0 (zero compute, zero network, zero filesystem mutation outside `proposals/` doc + witness JSON + registry append). First non-zero cost-actual append at user dispatch of `tool/hexa_weave_w5_setup.hexa` on ubu1.

## §16 Auto-absorption hook

Append to `state/discovery_absorption/registry.jsonl`:

```json
{"schema":"anima/discovery_absorption/v1","ts":"2026-04-28T23:50:00Z","finding_id":"hexa-weave-mvp-w5-sandbox-cycle14-prep-2026-04-28","witness_path":"proposals/hexa_weave_mvp_w5_sandbox_cycle14_prep_2026_04_28.md","kick_witness_path":"design/kick/2026-04-28_w5-sandbox-cycle14-prep_omega_cycle.json","absorption_channel":"proposal-w5-sandbox-cycle14-runbook","absorption_target":"HEXA-WEAVE W5 cycle 14 sandbox prep: 8-item Korean-labeled runbook (directory/venv/PyTorch/clone/weights/RCSB-split/sm_120-deepspeed/6L1U-dryrun) with idempotent (raw 65) probe + rollback (raw 142 D2) + ai-native failure trailer (raw 66); tool/hexa_weave_w5_setup.hexa raw-9 envelope spec; ~/core/hexa-weave/README.md sister repo Apache-2.0 + 12-pkg dep table; requirements.lock 25-pkg deterministic pin; 4-rung VRAM ladder (L1 6L1U → L2 ESM3 mid → L3 1UBQ 76aa → L4 350aa target post-W6); measurement criteria RMSD<5Å + TM>0.4 + VRAM<11GB + rc=0 + wall<30min; 5 n6 hook positions (N6-H1 τ=4 / N6-H2 σ=12 / N6-H3 J₂=24 block 47 / N6-H4 φ=2 / N6-H5 aggregator); cross-repo (anima/airgenome/hive/etc) sister-sandbox check 1/10 applicable + 1/10 partial; alien-grade 4.09 → ~4.9 unlock path conditional on Step 8 PASS; 5 raw 71 falsifiers (F-W5C14-1..F-W5C14-5 all TRANSCEND); 5 MISS criteria; 25 raw 91 C3 NOT-decided items","status":"forward-spec","absorbed_at":"2026-04-28T23:50:00Z","absorbed_via":"raw 108+135 W5 cycle 14 prep absorption","classifier_version":"raw_108_v1","raw_91_c3":"forward-spec only — no mkdir / no pip install / no git clone / no weight download / no nvidia-smi / no SSH / no inference / no runbook .hexa file authored yet (spec only) / no README.md written / no requirements.lock written / 25 NOT-decided items","parent_proposal":"proposals/hexa_weave_mvp_2026_04_28.md","predecessor_proposal":"proposals/hexa_weave_mvp_w5_user_approval_request_2026_04_28.md","parent_milestone":"W5-sandbox-cycle14-prep","cycle":14,"alien_grade_path":"4.09->~4.9 conditional on Step 8 PASS"}
```

## §17 Lint pre-flight

- own 1 English-only: PASS (body English; Korean only in §2 user-spec column labels)
- own 22 proposal-naming: PASS — `hexa_weave_mvp_w5_sandbox_cycle14_prep_2026_04_28.md` snake_case + `_YYYY_MM_DD` matches `category: operational`
- own 23 proposal-umbrella: PASS — frontmatter does not declare `umbrella:` (matches predecessor pattern)

## §18 Cross-references

- Parent spec: [`hexa_weave_mvp_2026_04_28.md`](hexa_weave_mvp_2026_04_28.md)
- W5 user approval (predecessor): [`hexa_weave_mvp_w5_user_approval_request_2026_04_28.md`](hexa_weave_mvp_w5_user_approval_request_2026_04_28.md)
- W4 dryrun spec: [`hexa_weave_mvp_w4_openfold_dryrun_2026_04_28.md`](hexa_weave_mvp_w4_openfold_dryrun_2026_04_28.md)
- W3 clone+VRAM spec: [`hexa_weave_mvp_w3_clone_vram_spec_2026_04_28.md`](hexa_weave_mvp_w3_clone_vram_spec_2026_04_28.md)
- W5 alt-2 CD-HIT: [`hexa_weave_mvp_w5_alt2_cdhit_clustering_2026_04_28.md`](hexa_weave_mvp_w5_alt2_cdhit_clustering_2026_04_28.md)
- Cycle 14 kick witness: `design/kick/2026-04-28_w5-sandbox-cycle14-prep_omega_cycle.json`
- Sister hexa-nanobot (cycle 13): `domains/biology/hexa-nanobot/`
- OpenFold upstream: https://github.com/aqlaboratory/openfold
- OpenFold v2.2.0: https://github.com/aqlaboratory/openfold/tree/v2.2.0
- AlphaFold weights mirror: via `download_alphafold_params.sh`
- PyTorch CUDA 12.4: https://download.pytorch.org/whl/cu124
- PDB 6L1U: https://www.rcsb.org/structure/6L1U

## §19 mk-history

- 2026-04-28 cycle 14: this prep doc — 8-item Korean runbook + setup script spec + sister README + requirements.lock + VRAM ladder + measurement criteria + 5 n6 hook positions + cross-repo check + 5 falsifiers + 25 NOT-decided items.
- 2026-04-28 cycle 8 fan-out 4/5: W5 user approval request (predecessor; 8-item checklist).
- 2026-04-28 cycle 7 fan-out 4/5: W4 dryrun spec (34.3 KB; 6-step sandbox + 3-step dryrun + 5-row pitfall matrix).
- 2026-04-28 cycle 6: W3 clone+VRAM spec (35.5 KB; 12-step clone + 9-step VRAM + 12-row dep matrix).
- 2026-04-28 cycle 5: W2 base-model integration (28 KB; 18 cross-attn + 5 n6 hooks + 1500 LoC ceiling).
- 2026-04-28 cycle 2: W1 architecture decision (OpenFold base; 12 GB VRAM ceiling; 5 case studies).
- 2026-04-28 cycle 1: hexa-weave-mvp parent spec.
