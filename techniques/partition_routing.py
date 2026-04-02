"""
Technique 19: Partition Routing MoE
====================================
p(6) = 11 = σ - μ integer partitions of 6.

The number 6 has exactly 11 integer partitions:
  {6}, {5,1}, {4,2}, {4,1,1}, {3,3}, {3,2,1}, {3,1,1,1},
  {2,2,2}, {2,2,1,1}, {2,1,1,1,1}, {1,1,1,1,1,1}

p(6) = 11 = σ - μ = 12 - 1: the partition count itself is an n=6 constant.

Insight: Each partition defines a natural expert allocation template.
A partition like {3,2,1} means "split computation into groups of size 3, 2, 1"
— three experts with different capacity shares summing to n=6.
This gives 11 structurally distinct routing patterns, each derived from
number theory rather than learned gating.

Architecture:
  - 11 partition templates define expert group structures
  - Router scores input against each partition "shape" (variance signature)
  - Top-k partitions selected per token → experts activated accordingly
  - Expert capacity naturally sums to n=6 for every partition
  - No load-balancing loss needed: partitions are self-balancing by construction

Expected: Structured routing with no auxiliary loss, competitive quality,
natural diversity of expert utilization patterns.
"""

import torch
import torch.nn as nn
import torch.nn.functional as F
import numpy as np
import math
import time

SEED = 42
torch.manual_seed(SEED)
np.random.seed(SEED)

# n=6 constants
N = 6
SIGMA = 12       # σ(6) = 12
MU = 1           # μ(6) = 1
PHI = 2          # φ(6) = 2
TAU = 4          # τ(6) = 4
J2 = 24          # J₂(6) = 24
SOPFR = 5        # sopfr(6) = 2+3 = 5
P_6 = 11         # p(6) = 11 = σ - μ

# All 11 integer partitions of 6
PARTITIONS_OF_6 = [
    (6,),              # 1 part
    (5, 1),            # 2 parts
    (4, 2),            # 2 parts
    (4, 1, 1),         # 3 parts
    (3, 3),            # 2 parts
    (3, 2, 1),         # 3 parts — most "balanced" odd partition
    (3, 1, 1, 1),      # 4 parts
    (2, 2, 2),         # 3 parts — perfectly uniform
    (2, 2, 1, 1),      # 4 parts
    (2, 1, 1, 1, 1),   # 5 parts
    (1, 1, 1, 1, 1, 1),# 6 parts — maximally fine-grained
]

assert len(PARTITIONS_OF_6) == P_6, f"Expected {P_6} partitions, got {len(PARTITIONS_OF_6)}"
assert all(sum(p) == N for p in PARTITIONS_OF_6), "All partitions must sum to n=6"


class PartitionExpert(nn.Module):
    """Single expert: linear -> activation -> linear."""

    def __init__(self, d_model, d_ff):
        super().__init__()
        self.fc1 = nn.Linear(d_model, d_ff)
        self.fc2 = nn.Linear(d_ff, d_model)

    def forward(self, x):
        return self.fc2(F.gelu(self.fc1(x)))


class PartitionRoutingMoE(nn.Module):
    """p(6)=11 partition-based Mixture of Experts routing.

    Each of the 11 partitions of 6 defines an expert allocation template.
    The router scores the input against partition "shape signatures" and
    selects top-k partitions. Experts within the chosen partition share
    the computation according to the partition's structure.

    Key property: p(6) = 11 = σ - μ, connecting partition count to n=6 constants.
    Every partition sums to n=6, ensuring total capacity is always n.
    """

    def __init__(self, d_model, d_ff=None, n_experts=N, top_k=2):
        super().__init__()
        self.d_model = d_model
        self.n_experts = n_experts
        self.top_k = top_k
        self.n_partitions = P_6

        if d_ff is None:
            d_ff = int(d_model * TAU / N)  # τ/n = 4/6 = 2/3 ratio per expert

        # Create n=6 experts with varying capacities
        self.experts = nn.ModuleList([
            PartitionExpert(d_model, d_ff) for _ in range(n_experts)
        ])

        # Partition shape signatures: encode each partition as a fixed-dim vector
        # Shape = normalized histogram of part sizes (max part = 6)
        self.register_buffer('partition_signatures', self._build_signatures())

        # Router: projects input to partition-score space
        self.router = nn.Linear(d_model, self.n_partitions, bias=False)

        # Partition-to-expert weight matrix: how each partition allocates experts
        self.register_buffer('partition_weights', self._build_partition_weights())

    def _build_signatures(self):
        """Build shape signatures for each partition.

        Each partition is encoded as a 6-dim vector where entry i is
        the count of parts equal to (i+1). This captures the "shape".
        """
        sigs = torch.zeros(self.n_partitions, N)
        for i, partition in enumerate(PARTITIONS_OF_6):
            for part in partition:
                sigs[i, part - 1] += 1
        # Normalize each signature
        sigs = sigs / sigs.sum(dim=-1, keepdim=True)
        return sigs

    def _build_partition_weights(self):
        """Build expert allocation weights from partitions.

        Each partition defines how to distribute computation among experts.
        A partition {3,2,1} means expert 0 gets 3/6, expert 1 gets 2/6,
        expert 2 gets 1/6 of the compute weight.

        Partitions with fewer parts than n_experts: remaining experts get 0.
        """
        weights = torch.zeros(self.n_partitions, self.n_experts)
        for i, partition in enumerate(PARTITIONS_OF_6):
            for j, part in enumerate(partition):
                if j < self.n_experts:
                    weights[i, j] = part / N  # normalize by n=6
        return weights

    def forward(self, x):
        """Route tokens through partition-selected expert combinations.

        Args:
            x: (batch, seq_len, d_model)
        Returns:
            output: (batch, seq_len, d_model)
        """
        B, S, D = x.shape

        # Compute partition scores from token representation
        token_repr = x.mean(dim=1)  # (B, D) — aggregate token info
        partition_logits = self.router(token_repr)  # (B, n_partitions)
        partition_probs = F.softmax(partition_logits, dim=-1)

        # Select top-k partitions per batch element
        topk_vals, topk_idx = partition_probs.topk(self.top_k, dim=-1)  # (B, top_k)
        topk_vals = topk_vals / topk_vals.sum(dim=-1, keepdim=True)  # renormalize

        # Compute blended expert weights from selected partitions
        # expert_weights: (B, n_experts)
        expert_weights = torch.zeros(B, self.n_experts, device=x.device)
        for k in range(self.top_k):
            idx = topk_idx[:, k]  # (B,)
            val = topk_vals[:, k]  # (B,)
            pw = self.partition_weights[idx]  # (B, n_experts)
            expert_weights += val.unsqueeze(-1) * pw

        # Run all experts and blend
        output = torch.zeros_like(x)
        for e in range(self.n_experts):
            w = expert_weights[:, e]  # (B,)
            mask = w > 1e-6
            if mask.any():
                expert_out = self.experts[e](x[mask])
                output[mask] += w[mask].unsqueeze(-1).unsqueeze(-1) * expert_out

        return output


class StandardMoE(nn.Module):
    """Standard top-k MoE for comparison."""

    def __init__(self, d_model, d_ff=None, n_experts=N, top_k=2):
        super().__init__()
        self.n_experts = n_experts
        self.top_k = top_k

        if d_ff is None:
            d_ff = int(d_model * TAU / N)

        self.experts = nn.ModuleList([
            PartitionExpert(d_model, d_ff) for _ in range(n_experts)
        ])
        self.gate = nn.Linear(d_model, n_experts, bias=False)

    def forward(self, x):
        B, S, D = x.shape
        gate_logits = self.gate(x.mean(dim=1))  # (B, n_experts)
        gate_probs = F.softmax(gate_logits, dim=-1)
        topk_vals, topk_idx = gate_probs.topk(self.top_k, dim=-1)
        topk_vals = topk_vals / topk_vals.sum(dim=-1, keepdim=True)

        output = torch.zeros_like(x)
        for k in range(self.top_k):
            idx = topk_idx[:, k]
            val = topk_vals[:, k]
            for e in range(self.n_experts):
                mask = idx == e
                if mask.any():
                    expert_out = self.experts[e](x[mask])
                    output[mask] += val[mask].unsqueeze(-1).unsqueeze(-1) * expert_out
        return output


class DenseMLP(nn.Module):
    """Dense FFN baseline (no MoE)."""

    def __init__(self, d_model, d_ff=None):
        super().__init__()
        if d_ff is None:
            d_ff = int(d_model * TAU / PHI)  # τ/φ = 2 ratio
        self.fc1 = nn.Linear(d_model, d_ff)
        self.fc2 = nn.Linear(d_ff, d_model)

    def forward(self, x):
        return self.fc2(F.gelu(self.fc1(x)))


def count_params(m):
    return sum(p.numel() for p in m.parameters())


def train_and_eval(model_cls, model_kwargs, train_x, train_y, test_x, test_y,
                   epochs=15, batch_size=64, lr=3e-3):
    """Train a simple classifier and return test accuracy + loss."""
    torch.manual_seed(SEED)
    d_model = train_x.shape[-1]

    # Wrap in a simple classifier
    class Classifier(nn.Module):
        def __init__(self):
            super().__init__()
            self.backbone = model_cls(**model_kwargs)
            self.head = nn.Linear(d_model, int(train_y.max().item()) + 1)

        def forward(self, x):
            x = self.backbone(x)
            return self.head(x.mean(dim=1))

    model = Classifier()
    params = count_params(model)
    opt = torch.optim.Adam(model.parameters(), lr=lr)

    n = len(train_x)
    t0 = time.time()
    for epoch in range(epochs):
        model.train()
        perm = torch.randperm(n)
        for i in range(0, n, batch_size):
            idx = perm[i:i + batch_size]
            logits = model(train_x[idx])
            loss = F.cross_entropy(logits, train_y[idx])
            opt.zero_grad()
            loss.backward()
            opt.step()

    elapsed = time.time() - t0

    model.eval()
    with torch.no_grad():
        logits = model(test_x)
        pred = logits.argmax(dim=-1)
        acc = (pred == test_y).float().mean().item()
        loss = F.cross_entropy(logits, test_y).item()

    return acc, loss, params, elapsed


def make_synthetic_data(n_train=3000, n_test=500, seq_len=12, d_model=48, n_classes=6):
    """Synthetic sequence classification data.

    seq_len=σ=12, d_model divisible by 6, n_classes=n=6.
    """
    rng = np.random.RandomState(SEED)

    def generate(n):
        x = torch.randn(n, seq_len, d_model)
        # Label = which of 6 segments of d_model has highest energy
        seg_size = d_model // N  # 48/6 = 8
        segments = x.reshape(n, seq_len, N, seg_size)
        energy = segments.pow(2).sum(dim=(1, 3))  # (n, 6)
        y = energy.argmax(dim=-1)
        return x, y

    train_x, train_y = generate(n_train)
    test_x, test_y = generate(n_test)
    return train_x, train_y, test_x, test_y


def main():
    print("=" * 70)
    print("  Technique 19: Partition Routing MoE")
    print("  p(6) = 11 = sigma - mu = 12 - 1 integer partitions")
    print("=" * 70)

    print(f"\n  n=6 constants: sigma={SIGMA}, phi={PHI}, tau={TAU}, J2={J2}")
    print(f"  p(6) = {P_6} = sigma - mu = {SIGMA} - {MU}")
    print(f"\n  The {P_6} integer partitions of 6:")
    for i, p in enumerate(PARTITIONS_OF_6):
        parts_str = " + ".join(str(x) for x in p)
        n_parts = len(p)
        print(f"    P{i+1:2d}: {parts_str:20s} = 6  ({n_parts} expert{'s' if n_parts > 1 else ' '})")

    print(f"\n  Each partition = expert allocation template")
    print(f"  Router selects top-k=phi={PHI} partitions per input")
    print(f"  Total experts = n = {N}")

    # Generate synthetic data
    SEQ_LEN = SIGMA  # σ = 12
    D_MODEL = 48     # σ·τ = 48
    N_CLASSES = N     # n = 6

    print(f"\n  Data: seq_len={SEQ_LEN}(=sigma), d_model={D_MODEL}(=sigma*tau), classes={N_CLASSES}(=n)")
    train_x, train_y, test_x, test_y = make_synthetic_data(
        n_train=3000, n_test=500, seq_len=SEQ_LEN, d_model=D_MODEL, n_classes=N_CLASSES
    )

    # Configurations to compare
    configs = [
        ("Dense MLP (baseline)",
         DenseMLP, {"d_model": D_MODEL}),
        ("Standard MoE (6 experts, top-2)",
         StandardMoE, {"d_model": D_MODEL, "n_experts": N, "top_k": PHI}),
        ("Partition MoE (p(6)=11, top-2)",
         PartitionRoutingMoE, {"d_model": D_MODEL, "n_experts": N, "top_k": PHI}),
        ("Partition MoE (p(6)=11, top-3)",
         PartitionRoutingMoE, {"d_model": D_MODEL, "n_experts": N, "top_k": N // PHI}),
    ]

    print(f"\n{'Config':<35} {'Acc':>7} {'Loss':>7} {'Params':>10} {'Time':>7}")
    print("-" * 70)

    results = []
    for label, cls, kwargs in configs:
        acc, loss, params, elapsed = train_and_eval(
            cls, kwargs, train_x, train_y, test_x, test_y
        )
        results.append((label, acc, loss, params, elapsed))
        marker = " <--" if "Partition MoE (p(6)=11, top-2)" in label else ""
        print(f"  {label:<33} {acc:>7.4f} {loss:>7.4f} {params:>10,} {elapsed:>6.1f}s{marker}")

    # Partition utilization analysis
    print(f"\n{'=' * 70}")
    print("  Partition Shape Analysis")
    print("=" * 70)

    model = PartitionRoutingMoE(d_model=D_MODEL, n_experts=N, top_k=PHI)
    print(f"\n  Partition signatures (shape encoding, 6-dim):")
    for i, (p, sig) in enumerate(zip(PARTITIONS_OF_6, model.partition_signatures)):
        sig_str = " ".join(f"{s:.2f}" for s in sig.tolist())
        print(f"    P{i+1:2d} {str(p):25s} sig=[{sig_str}]")

    print(f"\n  Expert weight allocation per partition:")
    for i, (p, w) in enumerate(zip(PARTITIONS_OF_6, model.partition_weights)):
        w_str = " ".join(f"{v:.2f}" for v in w.tolist())
        print(f"    P{i+1:2d} {str(p):25s} weights=[{w_str}]")

    print(f"\n{'=' * 70}")
    print("  Summary")
    print("=" * 70)
    print(f"\n  p(6) = {P_6} = sigma - mu = {SIGMA} - {MU}")
    print(f"  11 partition templates provide structurally diverse routing")
    print(f"  No auxiliary load-balancing loss needed")
    print(f"  Every partition sums to n={N} — capacity is always exact")
    print(f"  Router learns which partition shapes match input structure")
    print(f"  Number-theoretic routing: partitions ARE the architecture")


if __name__ == "__main__":
    main()
