"""
BT-AI1 phi/n = 1/3 N:M sparsity falsifier experiment.

Falsifier contract (registered at
reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md section 6):

  F-AI1-A: enforcing N:M = 2:6 (= phi/n = 1/3 keep, 2/3 prune) magnitude
           sparsity on every dense linear layer of a Llama-3 7B class
           model must NOT degrade MMLU accuracy by more than 3 percentage
           points versus the dense baseline.
  F-AI1-B: structured 2:6 sparsity must beat random 2:6 sparsity by at
           least the noise floor (defined here as 2 * standard error of
           the random arm). Otherwise the "structure carries information"
           premise behind selection C in the parent session collapses.

Atlas-pinned constants (read-only):

  - meta_fp = phi / n = 1 / 3 :: meta [10*!]   (atlas/atlas.n6:81)
  - n = 6 :: foundation [11*]                  (tile / NM block size)
  - tau = 4 :: foundation [11*]                (kept count = n - 2*tau/phi
    is NOT the rule; the rule is N:M = 2:6 with N = 2 kept = phi*n/sigma_n
    rounded; see omega-cycle section 6).

Resource policy (no-fabrication, omega-cycle section 6 fallback tree):

  - GPU (CUDA) + Llama-3 weights -> normal path, ~1 GPU-hour budget.
  - GPU + no Llama weights        -> Mistral-7B / Pythia-6.9B fallback,
                                     report MODEL_FALLBACK reason.
  - No GPU + MPS available        -> proxy path, smaller model,
                                     PROXY-VERDICT label, F-AI1 contract
                                     suspended (HOLD), tagged BT-AI1-PROXY.
  - CPU only                      -> proxy path with TinyLlama-1.1B at
                                     reduced eval count, PROXY-VERDICT,
                                     HOLD on F-AI1-A/B.
  - All paths fail                -> empty results, DISABLED verdict,
                                     no fabricated numbers.

Determinism: three seeds {0, 1, 2}; deterministic torch + numpy + python
RNGs; cached HF assets reused.

Dependencies: torch, transformers, numpy. Optional: datasets (for real
MMLU / GSM8K). When the datasets package or its network access is
unavailable, the script falls back to a small built-in multiple-choice
proxy battery and labels the eval `MMLU-MICRO-PROXY` so the falsifier is
HOLD rather than PASS / FAIL.

Usage:

  python3 experiments/anomaly/btAI1_phi_over_n_sparsity.py \
      --results-out reports/anomaly/btAI1_results.json \
      --seeds 0,1,2

References (read-only; this experiment does NOT modify atlas / KG /
domains.json):

  - atlas: atlas/atlas.n6:81  (`@C meta_fp = 1/3 :: meta [10*!]`)
  - KG: arch:n6-native-accelerator, principle:honesty-triad
        (n6shared/discovery_graph.json)
  - domain: ai-native-architecture
        (n6shared/n6/docs/domains.json)
  - parent session: omega-cycle-ai-native-arch-beyond-gpu-2026-04-26
"""

from __future__ import annotations

import argparse
import json
import math
import os
import platform
import random
import shutil
import socket
import sys
import time
from dataclasses import asdict, dataclass, field
from typing import Dict, List, Optional, Tuple

# ---------------------------------------------------------------------------
# Atlas-pinned constants (do not hand-edit; cite atlas grade + path).
# ---------------------------------------------------------------------------

ATLAS_META_FP = 1.0 / 3.0          # atlas: meta_fp = phi/n = 1/3 :: meta [10*!]
ATLAS_N = 6                        # atlas: n = 6 :: foundation [11*]
NM_M = 6                           # M in N:M = block width
NM_N_KEEP = 2                      # N in N:M = kept-per-block (= round(M/3))

# Falsifier thresholds (omega-cycle section 6).
F_AI1_A_MAX_DROP_PP = 3.0          # max MMLU drop in percentage points
F_AI1_B_NOISE_FLOOR_SIGMAS = 2.0   # multiples of random-arm SE


# ---------------------------------------------------------------------------
# Resource probe.
# ---------------------------------------------------------------------------


@dataclass
class ResourceProbe:
    """Snapshot of compute resources available on this host."""

    has_torch: bool = False
    torch_version: Optional[str] = None
    has_transformers: bool = False
    transformers_version: Optional[str] = None
    has_datasets: bool = False
    datasets_version: Optional[str] = None
    has_cuda: bool = False
    cuda_device_count: int = 0
    cuda_device_name: Optional[str] = None
    has_mps: bool = False
    cpu: str = ""
    ram_bytes: int = 0
    hf_cache_dir: Optional[str] = None
    candidate_models_cached: List[str] = field(default_factory=list)
    hf_token_present: bool = False
    network_huggingface_co: bool = False
    notes: List[str] = field(default_factory=list)


def _probe_resources() -> ResourceProbe:
    p = ResourceProbe()
    p.cpu = platform.processor() or platform.machine()
    try:
        import psutil  # type: ignore
        p.ram_bytes = int(psutil.virtual_memory().total)
    except Exception:
        try:
            import resource
            # macOS / linux fallback: ru_maxrss is per-process; use os.sysconf when present.
            pages = os.sysconf("SC_PHYS_PAGES") if hasattr(os, "sysconf") else 0
            page_size = os.sysconf("SC_PAGE_SIZE") if hasattr(os, "sysconf") else 0
            p.ram_bytes = int(pages) * int(page_size) if pages and page_size else 0
        except Exception:
            p.ram_bytes = 0

    try:
        import torch
        p.has_torch = True
        p.torch_version = torch.__version__
        try:
            p.has_cuda = bool(torch.cuda.is_available())
            if p.has_cuda:
                p.cuda_device_count = int(torch.cuda.device_count())
                if p.cuda_device_count:
                    p.cuda_device_name = torch.cuda.get_device_name(0)
        except Exception as exc:
            p.notes.append(f"cuda probe failed: {exc!r}")
        try:
            p.has_mps = bool(getattr(torch.backends, "mps", None) and torch.backends.mps.is_available())
        except Exception as exc:
            p.notes.append(f"mps probe failed: {exc!r}")
    except Exception as exc:
        p.notes.append(f"torch import failed: {exc!r}")

    try:
        import transformers
        p.has_transformers = True
        p.transformers_version = transformers.__version__
    except Exception as exc:
        p.notes.append(f"transformers import failed: {exc!r}")

    try:
        import datasets  # type: ignore
        p.has_datasets = True
        p.datasets_version = datasets.__version__
    except Exception as exc:
        p.notes.append(f"datasets import failed: {exc!r}")

    home = os.path.expanduser("~")
    candidates = [
        os.path.join(home, ".cache", "huggingface", "hub"),
        os.environ.get("HF_HOME", ""),
        os.environ.get("TRANSFORMERS_CACHE", ""),
    ]
    for c in candidates:
        if c and os.path.isdir(c):
            p.hf_cache_dir = c
            break

    if p.hf_cache_dir:
        wanted_substrings = (
            "Meta-Llama-3", "Llama-3", "Llama-3.1",
            "Mistral-7B", "Pythia",
            "TinyLlama", "Qwen2", "Qwen-1.5",
            "gpt2",
        )
        try:
            for entry in sorted(os.listdir(p.hf_cache_dir)):
                if not entry.startswith("models--"):
                    continue
                model_id = entry[len("models--"):].replace("--", "/")
                if any(s.lower() in entry.lower() for s in wanted_substrings):
                    p.candidate_models_cached.append(model_id)
        except Exception as exc:
            p.notes.append(f"hf cache scan failed: {exc!r}")

    p.hf_token_present = bool(
        os.environ.get("HF_TOKEN")
        or os.environ.get("HUGGING_FACE_HUB_TOKEN")
        or os.environ.get("HUGGINGFACE_TOKEN")
    )

    try:
        sock = socket.create_connection(("huggingface.co", 443), timeout=3.0)
        sock.close()
        p.network_huggingface_co = True
    except Exception as exc:
        p.notes.append(f"network huggingface.co unreachable: {exc!r}")

    return p


# ---------------------------------------------------------------------------
# Path selection (no-fabrication decision tree).
# ---------------------------------------------------------------------------


@dataclass
class PathDecision:
    """The execution path chosen given the resource probe."""

    path: str                        # "normal" | "proxy" | "disabled"
    reason: str
    selected_model: Optional[str]
    device: str                      # "cuda" | "mps" | "cpu" | "none"
    falsifier_mode: str              # "active" | "hold-proxy" | "hold-disabled"
    label: str                       # e.g., "BT-AI1" or "BT-AI1-PROXY"


def _choose_path(probe: ResourceProbe) -> PathDecision:
    if not probe.has_torch or not probe.has_transformers:
        return PathDecision(
            path="disabled",
            reason="torch and/or transformers not importable",
            selected_model=None,
            device="none",
            falsifier_mode="hold-disabled",
            label="BT-AI1-DISABLED",
        )

    # Preference order for normal-path models (Llama-3 7B class).
    normal_targets = [
        "meta-llama/Meta-Llama-3-8B",
        "meta-llama/Meta-Llama-3-8B-Instruct",
        "unsloth/Meta-Llama-3.1-8B",
        "unsloth/Meta-Llama-3.1-8B-Instruct",
    ]
    fallback_7b = [
        "mistralai/Mistral-7B-v0.1",
        "mistralai/Mistral-7B-v0.3",
        "EleutherAI/pythia-6.9b",
    ]
    proxy_targets = [
        "TinyLlama/TinyLlama-1.1B-Chat-v1.0",
        "Qwen/Qwen2.5-1.5B",
        "Qwen/Qwen2.5-0.5B",
        "EleutherAI/pythia-1.4b",
        "gpt2",
    ]

    cached = set(probe.candidate_models_cached)

    def _pick(targets: List[str]) -> Optional[str]:
        for t in targets:
            if t in cached:
                return t
        return None

    if probe.has_cuda:
        m = _pick(normal_targets)
        if m:
            return PathDecision("normal", "cuda + cached llama-3", m, "cuda", "active", "BT-AI1")
        m = _pick(fallback_7b)
        if m:
            return PathDecision(
                "normal", "cuda + cached 7B fallback (model substitution)",
                m, "cuda", "active", "BT-AI1",
            )
        if probe.network_huggingface_co and probe.hf_token_present:
            return PathDecision(
                "normal", "cuda + HF token + network; download will be attempted",
                normal_targets[0], "cuda", "active", "BT-AI1",
            )
        # CUDA but no model accessible -> degrade to proxy with whatever we have.
        m = _pick(proxy_targets)
        if m:
            return PathDecision(
                "proxy", "cuda available but no 7B model cached or downloadable",
                m, "cuda", "hold-proxy", "BT-AI1-PROXY",
            )
        return PathDecision(
            "disabled", "cuda available but no model accessible",
            None, "cuda", "hold-disabled", "BT-AI1-DISABLED",
        )

    if probe.has_mps:
        m = _pick(proxy_targets) or _pick(fallback_7b) or _pick(normal_targets)
        if m:
            return PathDecision(
                "proxy", "no cuda; MPS available; small-model proxy run",
                m, "mps", "hold-proxy", "BT-AI1-PROXY",
            )
        return PathDecision(
            "disabled", "no cuda; MPS available but no model cached",
            None, "mps", "hold-disabled", "BT-AI1-DISABLED",
        )

    # CPU only.
    m = _pick(proxy_targets)
    if m:
        return PathDecision(
            "proxy", "cpu-only; tiny proxy model only",
            m, "cpu", "hold-proxy", "BT-AI1-PROXY",
        )
    return PathDecision(
        "disabled", "cpu-only and no proxy model cached",
        None, "cpu", "hold-disabled", "BT-AI1-DISABLED",
    )


# ---------------------------------------------------------------------------
# N:M sparsity masks.
# ---------------------------------------------------------------------------


def _structured_nm_mask(weight, n_keep: int, m_block: int):
    """Magnitude-pruning N:M mask: keep top-N of each consecutive M-block.

    weight : torch.Tensor of shape (out, in). Pruning is along the input
             dimension (last dim), which matches modern N:M sparse-tensor
             convention (e.g., Ampere 2:4) and the omega-cycle section 6 spec.
    """

    import torch

    if weight.dim() != 2:
        raise ValueError(f"expected 2-D weight, got shape {tuple(weight.shape)}")
    out_dim, in_dim = weight.shape
    pad = (m_block - in_dim % m_block) % m_block
    if pad:
        wp = torch.nn.functional.pad(weight, (0, pad), value=0.0)
    else:
        wp = weight
    new_in = wp.shape[1]
    blocks = wp.view(out_dim, new_in // m_block, m_block)
    abs_blocks = blocks.abs()
    # topk indices to KEEP.
    _, keep_idx = abs_blocks.topk(n_keep, dim=-1)
    mask = torch.zeros_like(blocks, dtype=torch.bool)
    mask.scatter_(-1, keep_idx, True)
    mask = mask.view(out_dim, new_in)
    if pad:
        mask = mask[:, :in_dim]
    return mask


def _random_nm_mask(weight, n_keep: int, m_block: int, rng):
    """Random N:M mask: independently of magnitude, keep N random per block."""

    import torch

    if weight.dim() != 2:
        raise ValueError(f"expected 2-D weight, got shape {tuple(weight.shape)}")
    out_dim, in_dim = weight.shape
    pad = (m_block - in_dim % m_block) % m_block
    new_in = in_dim + pad
    n_blocks = (out_dim * new_in) // m_block
    mask = torch.zeros(n_blocks, m_block, dtype=torch.bool)
    # rng is numpy.random.Generator for reproducibility.
    for b in range(n_blocks):
        idx = rng.choice(m_block, size=n_keep, replace=False)
        mask[b, idx] = True
    mask = mask.view(out_dim, new_in)
    if pad:
        mask = mask[:, :in_dim]
    return mask


def _apply_mask_to_linears(model, mode: str, seed: int) -> Dict[str, int]:
    """In-place mask every nn.Linear weight matrix.

    mode : "dense" | "structured" | "random"
    Returns a dict counting linears touched and total params zeroed.
    """

    import torch
    import numpy as np

    counts = {"linears_touched": 0, "params_total": 0, "params_zeroed": 0}
    if mode == "dense":
        return counts
    rng = np.random.default_rng(seed)

    for module in model.modules():
        if not isinstance(module, torch.nn.Linear):
            continue
        w = module.weight.data
        if w.dim() != 2 or w.shape[1] < NM_M:
            continue
        if mode == "structured":
            m = _structured_nm_mask(w, NM_N_KEEP, NM_M)
        elif mode == "random":
            m = _random_nm_mask(w, NM_N_KEEP, NM_M, rng)
        else:
            raise ValueError(f"unknown sparsity mode: {mode!r}")
        m = m.to(device=w.device)
        n_params = int(w.numel())
        n_zeroed = int(n_params - m.sum().item())
        with torch.no_grad():
            module.weight.data = w * m.to(dtype=w.dtype)
        counts["linears_touched"] += 1
        counts["params_total"] += n_params
        counts["params_zeroed"] += n_zeroed
    return counts


# ---------------------------------------------------------------------------
# Evaluation harness.
# ---------------------------------------------------------------------------


# Tiny built-in multiple-choice battery used as MMLU-MICRO-PROXY when the
# real datasets package or its network access is unavailable. These items
# are intentionally elementary and span a few subjects; they are NOT a
# substitute for MMLU and the falsifier verdict for runs using this
# battery is HOLD by construction.
MICRO_MMLU = [
    {
        "q": "Which planet is known as the Red Planet?",
        "choices": ["Earth", "Mars", "Jupiter", "Venus"],
        "answer": 1,
    },
    {
        "q": "What is the chemical symbol for water?",
        "choices": ["O2", "H2O", "CO2", "NaCl"],
        "answer": 1,
    },
    {
        "q": "Who wrote the play 'Hamlet'?",
        "choices": ["Charles Dickens", "William Shakespeare", "Mark Twain", "Jane Austen"],
        "answer": 1,
    },
    {
        "q": "What is 2 + 2?",
        "choices": ["3", "4", "5", "22"],
        "answer": 1,
    },
    {
        "q": "Which gas do plants absorb from the atmosphere?",
        "choices": ["Oxygen", "Nitrogen", "Carbon dioxide", "Hydrogen"],
        "answer": 2,
    },
    {
        "q": "What is the largest ocean on Earth?",
        "choices": ["Atlantic", "Indian", "Arctic", "Pacific"],
        "answer": 3,
    },
    {
        "q": "How many continents are there?",
        "choices": ["5", "6", "7", "8"],
        "answer": 2,
    },
    {
        "q": "Which language is primarily spoken in Brazil?",
        "choices": ["Spanish", "Portuguese", "French", "English"],
        "answer": 1,
    },
    {
        "q": "What is the boiling point of water at sea level in Celsius?",
        "choices": ["50", "75", "100", "150"],
        "answer": 2,
    },
    {
        "q": "Which organ pumps blood through the human body?",
        "choices": ["Liver", "Brain", "Heart", "Kidney"],
        "answer": 2,
    },
    {
        "q": "Who painted the Mona Lisa?",
        "choices": ["Vincent van Gogh", "Pablo Picasso", "Leonardo da Vinci", "Claude Monet"],
        "answer": 2,
    },
    {
        "q": "What is the square root of 81?",
        "choices": ["7", "8", "9", "10"],
        "answer": 2,
    },
]

MICRO_GSM8K = [
    {"q": "Alice has 3 apples. She buys 4 more. How many does she have?", "answer": 7},
    {"q": "A box has 12 cookies. If 5 are eaten, how many remain?", "answer": 7},
    {"q": "What is 6 multiplied by 7?", "answer": 42},
    {"q": "If a train travels 60 miles in 1 hour, how far in 3 hours?", "answer": 180},
    {"q": "Sum of the first five positive integers?", "answer": 15},
    {"q": "What is 100 divided by 4?", "answer": 25},
    {"q": "Bob gives 8 marbles to each of his 3 friends. Total given?", "answer": 24},
    {"q": "If x + 5 = 12, what is x?", "answer": 7},
]


def _eval_multiple_choice(model, tokenizer, items, device, max_new_tokens=4):
    """Evaluate a model on multiple-choice items by selecting the option
    whose letter (A/B/C/D) gives the highest next-token log-probability
    given the prompt. Avoids generation; deterministic and fast.
    """

    import torch

    letters = ["A", "B", "C", "D"]
    n_correct = 0
    n_total = 0
    for item in items:
        prompt_lines = [
            "The following is a multiple choice question. Reply with a single letter (A, B, C, or D).",
            f"Question: {item['q']}",
        ]
        for j, choice in enumerate(item["choices"]):
            prompt_lines.append(f"{letters[j]}. {choice}")
        prompt_lines.append("Answer:")
        prompt = "\n".join(prompt_lines) + " "

        inputs = tokenizer(prompt, return_tensors="pt").to(device)
        with torch.no_grad():
            out = model(**inputs)
        logits = out.logits[0, -1, :]
        # Score each letter token.
        scores = []
        for letter in letters[: len(item["choices"])]:
            tok_ids = tokenizer.encode(letter, add_special_tokens=False)
            if not tok_ids:
                scores.append(-float("inf"))
                continue
            scores.append(float(logits[tok_ids[0]].item()))
        pred = int(max(range(len(scores)), key=lambda i: scores[i]))
        if pred == item["answer"]:
            n_correct += 1
        n_total += 1
    acc = n_correct / n_total if n_total else 0.0
    # 95% Wilson approx via normal CI (acceptable for n_total >= 8).
    if n_total > 0:
        se = math.sqrt(max(acc * (1 - acc) / n_total, 1e-12))
        ci95 = 1.96 * se
    else:
        se, ci95 = 0.0, 0.0
    return {"acc": acc, "n": n_total, "se": se, "ci95": ci95}


def _eval_gsm8k_micro(model, tokenizer, items, device, max_new_tokens=16):
    """Evaluate a model on tiny arithmetic word-problems by greedy decoding
    and integer extraction from the answer span.
    """

    import re
    import torch

    n_correct = 0
    n_total = 0
    for item in items:
        prompt = (
            "Solve the math problem. Reply with just the integer answer.\n"
            f"Problem: {item['q']}\nAnswer:"
        )
        inputs = tokenizer(prompt, return_tensors="pt").to(device)
        with torch.no_grad():
            gen_ids = model.generate(
                **inputs,
                max_new_tokens=max_new_tokens,
                do_sample=False,
                num_beams=1,
                pad_token_id=tokenizer.eos_token_id or 0,
            )
        text = tokenizer.decode(gen_ids[0][inputs["input_ids"].shape[1] :], skip_special_tokens=True)
        match = re.search(r"-?\d+", text)
        pred = int(match.group()) if match else None
        if pred is not None and pred == item["answer"]:
            n_correct += 1
        n_total += 1
    acc = n_correct / n_total if n_total else 0.0
    if n_total > 0:
        se = math.sqrt(max(acc * (1 - acc) / n_total, 1e-12))
        ci95 = 1.96 * se
    else:
        se, ci95 = 0.0, 0.0
    return {"acc": acc, "n": n_total, "se": se, "ci95": ci95}


# ---------------------------------------------------------------------------
# Try to load the real MMLU + GSM8K via the datasets package, otherwise
# return the built-in micro batteries with a "_proxy" tag.
# ---------------------------------------------------------------------------


def _load_eval_data(probe: ResourceProbe, max_mmlu: int, max_gsm: int):
    label = "MICRO-PROXY"
    mmlu = list(MICRO_MMLU[:max_mmlu]) if max_mmlu else list(MICRO_MMLU)
    gsm = list(MICRO_GSM8K[:max_gsm]) if max_gsm else list(MICRO_GSM8K)
    if not (probe.has_datasets and probe.network_huggingface_co):
        return mmlu, gsm, label
    try:
        from datasets import load_dataset  # type: ignore
        # MMLU: use the canonical "all" subset's test split.
        mmlu_real = load_dataset("cais/mmlu", "all", split="test")
        gsm_real = load_dataset("openai/gsm8k", "main", split="test")
        new_mmlu = []
        for row in mmlu_real:
            if max_mmlu and len(new_mmlu) >= max_mmlu:
                break
            new_mmlu.append({
                "q": row["question"],
                "choices": list(row["choices"]),
                "answer": int(row["answer"]),
            })
        new_gsm = []
        for row in gsm_real:
            if max_gsm and len(new_gsm) >= max_gsm:
                break
            ans_text = row["answer"].split("####")[-1].strip().replace(",", "")
            try:
                ans_int = int(ans_text)
            except ValueError:
                continue
            new_gsm.append({"q": row["question"], "answer": ans_int})
        if new_mmlu and new_gsm:
            return new_mmlu, new_gsm, "REAL"
    except Exception as exc:
        # Fall through to micro proxy.
        return mmlu, gsm, f"MICRO-PROXY (real-load-failed: {exc!r})"
    return mmlu, gsm, label


# ---------------------------------------------------------------------------
# Top-level run.
# ---------------------------------------------------------------------------


def _set_seed(seed: int):
    random.seed(seed)
    try:
        import numpy as np
        np.random.seed(seed)
    except Exception:
        pass
    try:
        import torch
        torch.manual_seed(seed)
        if torch.cuda.is_available():
            torch.cuda.manual_seed_all(seed)
    except Exception:
        pass


def _load_model(model_id: str, device: str):
    import torch
    from transformers import AutoModelForCausalLM, AutoTokenizer

    tok = AutoTokenizer.from_pretrained(model_id, use_fast=True)
    if tok.pad_token is None and tok.eos_token is not None:
        tok.pad_token = tok.eos_token
    dtype = torch.float16 if device == "cuda" else torch.float32
    model = AutoModelForCausalLM.from_pretrained(
        model_id, torch_dtype=dtype, low_cpu_mem_usage=True,
    )
    if device == "cuda":
        model = model.to("cuda")
    elif device == "mps":
        model = model.to("mps")
    else:
        model = model.to("cpu")
    model.eval()
    return model, tok


def _run_arm(model_id: str, device: str, mode: str, seed: int,
             mmlu_items, gsm_items) -> Dict:
    """Load a fresh model copy, apply the mask, and run both evals."""

    t0 = time.time()
    model, tok = _load_model(model_id, device)
    mask_counts = _apply_mask_to_linears(model, mode=mode, seed=seed)
    mmlu = _eval_multiple_choice(model, tok, mmlu_items, device)
    gsm = _eval_gsm8k_micro(model, tok, gsm_items, device)
    wall = time.time() - t0

    # Free memory between arms.
    try:
        import torch
        del model
        if device == "cuda":
            torch.cuda.empty_cache()
        elif device == "mps":
            try:
                torch.mps.empty_cache()
            except Exception:
                pass
    except Exception:
        pass

    return {
        "mode": mode,
        "seed": seed,
        "mmlu": mmlu,
        "gsm8k": gsm,
        "mask_counts": mask_counts,
        "wall_seconds": wall,
    }


def _aggregate(arms: List[Dict]) -> Dict:
    """Aggregate per-seed results -> mean + standard error per mode."""

    import statistics as st

    by_mode: Dict[str, List[Dict]] = {}
    for a in arms:
        by_mode.setdefault(a["mode"], []).append(a)

    summary: Dict[str, Dict] = {}
    for mode, lst in by_mode.items():
        mmlu_accs = [r["mmlu"]["acc"] for r in lst]
        gsm_accs = [r["gsm8k"]["acc"] for r in lst]
        n_seeds = len(lst)
        n_mmlu = lst[0]["mmlu"]["n"]
        n_gsm = lst[0]["gsm8k"]["n"]
        summary[mode] = {
            "n_seeds": n_seeds,
            "n_mmlu_items": n_mmlu,
            "n_gsm8k_items": n_gsm,
            "mmlu_mean_acc": float(sum(mmlu_accs) / n_seeds) if n_seeds else 0.0,
            "mmlu_std_acc": float(st.pstdev(mmlu_accs)) if n_seeds > 1 else 0.0,
            "mmlu_se_acc": float(st.pstdev(mmlu_accs) / math.sqrt(n_seeds))
                             if n_seeds > 1 else 0.0,
            "gsm8k_mean_acc": float(sum(gsm_accs) / n_seeds) if n_seeds else 0.0,
            "gsm8k_std_acc": float(st.pstdev(gsm_accs)) if n_seeds > 1 else 0.0,
            "gsm8k_se_acc": float(st.pstdev(gsm_accs) / math.sqrt(n_seeds))
                             if n_seeds > 1 else 0.0,
        }
    return summary


def _evaluate_falsifiers(summary: Dict, falsifier_mode: str) -> Dict:
    """Compute F-AI1-A and F-AI1-B verdicts.

    falsifier_mode == "active"        -> PASS / FAIL
    falsifier_mode == "hold-proxy"    -> HOLD with measurement reported
    falsifier_mode == "hold-disabled" -> HOLD, no measurement
    """

    out = {
        "F_AI1_A": {
            "name": "structured 1/3 sparsity vs dense MMLU drop",
            "max_drop_pp": F_AI1_A_MAX_DROP_PP,
        },
        "F_AI1_B": {
            "name": "structured 1/3 vs random 1/3 difference vs noise floor",
            "noise_floor_sigmas": F_AI1_B_NOISE_FLOOR_SIGMAS,
        },
        "verdict": "HOLD",
    }

    if falsifier_mode == "hold-disabled" or "dense" not in summary or "structured" not in summary:
        out["F_AI1_A"]["verdict"] = "HOLD"
        out["F_AI1_B"]["verdict"] = "HOLD"
        out["verdict"] = "HOLD"
        out["note"] = "falsifier suspended: no measurement available"
        return out

    dense_acc = summary["dense"]["mmlu_mean_acc"]
    struct_acc = summary["structured"]["mmlu_mean_acc"]
    drop_pp = (dense_acc - struct_acc) * 100.0
    out["F_AI1_A"]["dense_mmlu_acc"] = dense_acc
    out["F_AI1_A"]["structured_mmlu_acc"] = struct_acc
    out["F_AI1_A"]["drop_pp"] = drop_pp

    f_a_pass = drop_pp <= F_AI1_A_MAX_DROP_PP

    if "random" in summary:
        rand_acc = summary["random"]["mmlu_mean_acc"]
        rand_se = summary["random"]["mmlu_se_acc"]
        # Per-item SE if seed-SE is zero (e.g., 1 seed).
        if rand_se <= 0.0:
            n = summary["random"]["n_mmlu_items"] or 1
            rand_se = math.sqrt(max(rand_acc * (1 - rand_acc) / n, 1e-12))
        diff = struct_acc - rand_acc
        floor = F_AI1_B_NOISE_FLOOR_SIGMAS * rand_se
        out["F_AI1_B"]["random_mmlu_acc"] = rand_acc
        out["F_AI1_B"]["structured_minus_random_pp"] = diff * 100.0
        out["F_AI1_B"]["noise_floor_pp"] = floor * 100.0
        f_b_pass = diff > floor
    else:
        f_b_pass = False
        out["F_AI1_B"]["note"] = "random arm missing"

    if falsifier_mode == "active":
        out["F_AI1_A"]["verdict"] = "PASS" if f_a_pass else "FAIL"
        out["F_AI1_B"]["verdict"] = "PASS" if f_b_pass else "FAIL"
        out["verdict"] = "PASS" if (f_a_pass and f_b_pass) else "FAIL"
    else:  # hold-proxy
        out["F_AI1_A"]["verdict"] = "HOLD"
        out["F_AI1_B"]["verdict"] = "HOLD"
        out["verdict"] = "HOLD-PROXY"
        out["note"] = (
            "falsifier suspended because the run uses a smaller proxy model; "
            "raw measurement is reported but the F-AI1-A/B contract is not "
            "applied (BT-AI1-PROXY)."
        )

    return out


def _parse_args(argv: Optional[List[str]] = None) -> argparse.Namespace:
    p = argparse.ArgumentParser(description="BT-AI1 phi/n=1/3 sparsity falsifier")
    p.add_argument("--seeds", type=str, default="0,1,2",
                   help="comma-separated integer seeds")
    p.add_argument("--max-mmlu", type=int, default=200,
                   help="max MMLU items per arm (0 = no cap, micro proxy uses built-in size)")
    p.add_argument("--max-gsm", type=int, default=50,
                   help="max GSM8K items per arm (0 = no cap, micro proxy uses built-in size)")
    p.add_argument("--results-out", type=str,
                   default="reports/anomaly/btAI1_results.json")
    p.add_argument("--probe-only", action="store_true",
                   help="emit the resource probe + path decision and exit")
    p.add_argument("--force-disabled", action="store_true",
                   help="skip evaluation and emit a DISABLED HOLD result")
    return p.parse_args(argv)


def main(argv: Optional[List[str]] = None) -> int:
    args = _parse_args(argv)

    probe = _probe_resources()
    decision = _choose_path(probe)

    output: Dict = {
        "schema_version": 1,
        "experiment": "BT-AI1",
        "label": decision.label,
        "atlas_constants": {
            "meta_fp_phi_over_n": ATLAS_META_FP,
            "n": ATLAS_N,
            "nm_block_M": NM_M,
            "nm_keep_N": NM_N_KEEP,
        },
        "atlas_citations": [
            "atlas/atlas.n6:81 -> @C meta_fp = 1/3 :: meta [10*!]",
        ],
        "kg_nodes_referenced": [
            "arch:n6-native-accelerator",
            "principle:honesty-triad",
        ],
        "domain": "ai-native-architecture",
        "parent_session": (
            "reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md"
        ),
        "resource_probe": asdict(probe),
        "path_decision": asdict(decision),
        "args": vars(args),
    }

    if args.probe_only or args.force_disabled or decision.path == "disabled":
        verdict = _evaluate_falsifiers({}, "hold-disabled" if (args.force_disabled or decision.path == "disabled") else decision.falsifier_mode)
        output["arms"] = []
        output["summary"] = {}
        output["falsifiers"] = verdict
        output["wall_seconds"] = 0.0
        text = json.dumps(output, indent=2, sort_keys=True)
        os.makedirs(os.path.dirname(args.results_out) or ".", exist_ok=True)
        with open(args.results_out, "w", encoding="utf-8") as f:
            f.write(text)
        sys.stdout.write(text + "\n")
        return 0

    # Live execution path.
    seeds = [int(s) for s in args.seeds.split(",") if s.strip()]
    mmlu_items, gsm_items, eval_label = _load_eval_data(probe, args.max_mmlu, args.max_gsm)
    output["eval_data_label"] = eval_label

    arms: List[Dict] = []
    t0 = time.time()
    for seed in seeds:
        _set_seed(seed)
        for mode in ("dense", "structured", "random"):
            try:
                arm = _run_arm(
                    decision.selected_model, decision.device, mode, seed,
                    mmlu_items, gsm_items,
                )
            except Exception as exc:
                arm = {
                    "mode": mode,
                    "seed": seed,
                    "error": repr(exc),
                    "mmlu": {"acc": 0.0, "n": 0, "se": 0.0, "ci95": 0.0},
                    "gsm8k": {"acc": 0.0, "n": 0, "se": 0.0, "ci95": 0.0},
                    "mask_counts": {},
                    "wall_seconds": 0.0,
                }
            arms.append(arm)

    summary = _aggregate(arms)
    # If evaluation used the micro-proxy battery, force HOLD-PROXY even on a
    # CUDA + Llama-3 run (insufficient to clear the falsifier contract).
    fmode = decision.falsifier_mode
    if eval_label != "REAL" and fmode == "active":
        fmode = "hold-proxy"
        output["falsifier_mode_override"] = "eval_data_micro_proxy"
    verdict = _evaluate_falsifiers(summary, fmode)

    output["eval_data_label"] = eval_label
    output["arms"] = arms
    output["summary"] = summary
    output["falsifiers"] = verdict
    output["wall_seconds"] = time.time() - t0

    text = json.dumps(output, indent=2, sort_keys=True)
    os.makedirs(os.path.dirname(args.results_out) or ".", exist_ok=True)
    with open(args.results_out, "w", encoding="utf-8") as f:
        f.write(text)
    sys.stdout.write(text + "\n")
    return 0 if verdict["verdict"] in ("PASS", "HOLD-PROXY", "HOLD") else 1


if __name__ == "__main__":
    raise SystemExit(main())
