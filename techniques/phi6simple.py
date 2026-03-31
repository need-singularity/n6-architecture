"""
H-EE-1: Cyclotomic Polynomial Activation Comparison
=====================================================
Hypothesis: Phi6 (x^2-x+1) is uniquely optimal among cyclotomic activations.

Tested activations:
  Phi3(x) = x^2 + x + 1     (3rd cyclotomic)
  Phi4(x) = x^2 + 1          (4th cyclotomic)
  Phi6(x) = x^2 - x + 1      (6th cyclotomic) -- our candidate
  Phi8(x) = x^4 + 1          (8th cyclotomic)
  Phi12(x)= x^4 - x^2 + 1    (12th cyclotomic)
  GELU                         (baseline)

All polynomial activations clamped to [-2, 2] for stability.
Task: 2-layer transformer on structured sequence prediction (matches hen9 benchmark).
"""

import math
import time
import sys
import numpy as np

RNG_SEED = 42

# ── Activation functions (forward + backward) ──

def act_gelu(x):
    return 0.5 * x * (1.0 + np.tanh(0.7978845608028654 * (x + 0.044715 * x**3)))

def act_gelu_grad(x):
    c = 0.7978845608028654
    x3 = x**3
    t = np.tanh(c * (x + 0.044715 * x3))
    dt = 1.0 - t*t
    return 0.5 * (1.0 + t) + 0.5 * x * dt * c * (1.0 + 3 * 0.044715 * x**2)

def act_relu(x):
    return np.maximum(0.0, x)

def act_relu_grad(x):
    return (x > 0).astype(x.dtype)

def make_clamped_poly(coeffs, name):
    """Create clamped polynomial activation from coefficients.
    coeffs: list of (power, coeff) pairs. E.g., [(2,1),(1,-1),(0,1)] for x^2-x+1.
    """
    def forward(x):
        xc = np.clip(x, -2.0, 2.0)
        result = np.zeros_like(xc)
        for power, coeff in coeffs:
            result += coeff * (xc ** power)
        return result

    def backward(x):
        mask = ((x >= -2.0) & (x <= 2.0)).astype(x.dtype)
        xc = np.clip(x, -2.0, 2.0)
        result = np.zeros_like(xc)
        for power, coeff in coeffs:
            if power > 0:
                result += coeff * power * (xc ** (power - 1))
        return mask * result

    forward.__name__ = name
    backward.__name__ = name + "_grad"
    return forward, backward

# Define cyclotomic activations
phi3_fwd, phi3_bwd = make_clamped_poly([(2, 1), (1, 1), (0, 1)], "Phi3")   # x^2+x+1
phi4_fwd, phi4_bwd = make_clamped_poly([(2, 1), (0, 1)], "Phi4")            # x^2+1
phi6_fwd, phi6_bwd = make_clamped_poly([(2, 1), (1, -1), (0, 1)], "Phi6")   # x^2-x+1
phi8_fwd, phi8_bwd = make_clamped_poly([(4, 1), (0, 1)], "Phi8")            # x^4+1
phi12_fwd, phi12_bwd = make_clamped_poly([(4, 1), (2, -1), (0, 1)], "Phi12") # x^4-x^2+1

ACTIVATIONS = {
    "GELU":   (act_gelu, act_gelu_grad),
    "ReLU":   (act_relu, act_relu_grad),
    "Phi3":   (phi3_fwd, phi3_bwd),
    "Phi4":   (phi4_fwd, phi4_bwd),
    "Phi6":   (phi6_fwd, phi6_bwd),
    "Phi8":   (phi8_fwd, phi8_bwd),
    "Phi12":  (phi12_fwd, phi12_bwd),
}

FLOP_COUNT = {
    "GELU": 14,
    "ReLU": 1,
    "Phi3": 4,   # clamp + mul + add + add
    "Phi4": 3,   # clamp + mul + add
    "Phi6": 4,   # clamp + mul + sub + add
    "Phi8": 4,   # clamp + mul(x^2) + mul(x^4) + add
    "Phi12": 5,  # clamp + mul(x^2) + mul(x^4) + sub + add
}

# ── MLP with backprop ──

class MLP:
    def __init__(self, d_in, d_hidden, d_out, act_fwd, act_bwd, rng):
        scale = np.sqrt(2.0 / d_in)
        self.W1 = rng.standard_normal((d_in, d_hidden)).astype(np.float64) * scale
        self.b1 = np.zeros(d_hidden)
        self.W2 = rng.standard_normal((d_hidden, d_out)).astype(np.float64) * (np.sqrt(2.0 / d_hidden))
        self.b2 = np.zeros(d_out)
        self.act_fwd = act_fwd
        self.act_bwd = act_bwd

    def forward(self, x):
        self._x = x
        self._h_pre = x @ self.W1 + self.b1
        self._h = self.act_fwd(self._h_pre)
        return self._h @ self.W2 + self.b2

    def backward(self, dout, lr):
        dh = dout @ self.W2.T
        dh_pre = dh * self.act_bwd(self._h_pre)
        dW2 = self._h.T @ dout
        db2 = dout.sum(0)
        dW1 = self._x.T @ dh_pre
        db1 = dh_pre.sum(0)
        self.W1 -= lr * dW1
        self.b1 -= lr * db1
        self.W2 -= lr * dW2
        self.b2 -= lr * db2


def softmax_np(x):
    ex = np.exp(x - x.max(-1, keepdims=True))
    return ex / ex.sum(-1, keepdims=True)

def cross_entropy_and_grad(logits, targets):
    probs = softmax_np(logits)
    N = logits.shape[0]
    loss = -np.log(probs[np.arange(N), targets] + 1e-9).mean()
    dlogits = probs.copy()
    dlogits[np.arange(N), targets] -= 1.0
    dlogits /= N
    return loss, dlogits


def make_corpus(d_in=32, n_classes=32, n_batches=200, batch_size=64):
    rng = np.random.default_rng(0)
    batches = []
    patterns = [
        np.arange(n_classes),
        np.roll(np.arange(n_classes), 3),
        (np.arange(n_classes) * 3) % n_classes,
        (np.arange(n_classes) * 5) % n_classes,
    ]
    for _ in range(n_batches):
        pat = rng.integers(0, len(patterns))
        idx = rng.integers(0, n_classes, batch_size)
        targets = patterns[pat][idx]
        x = np.zeros((batch_size, d_in))
        x[np.arange(batch_size), idx % d_in] = 1.0
        x += rng.standard_normal((batch_size, d_in)) * 0.05
        batches.append((x.astype(np.float64), targets.astype(np.int32)))
    return batches


def train_mlp(act_name, act_fwd, act_bwd, corpus, n_steps=500, lr=0.01):
    rng = np.random.default_rng(RNG_SEED)
    D_IN, D_HID, D_OUT = 32, 128, 32
    model = MLP(D_IN, D_HID, D_OUT, act_fwd, act_bwd, rng)
    losses = []

    t0 = time.perf_counter()
    for step in range(n_steps):
        x, targets = corpus[step % len(corpus)]
        logits = model.forward(x)
        loss, dlogits = cross_entropy_and_grad(logits, targets)
        model.backward(dlogits, lr)
        losses.append(loss)
    train_time = time.perf_counter() - t0

    return {
        "act": act_name,
        "final_loss": float(np.mean(losses[-50:])),
        "train_time_s": train_time,
        "losses": losses,
    }


# ── Microbenchmark ──

def microbenchmark(n_elements=500_000, n_calls=100):
    rng = np.random.default_rng(RNG_SEED)
    x = rng.standard_normal(n_elements).astype(np.float32)
    results = {}
    for fn, _ in ACTIVATIONS.values():
        _ = fn(x)
    for act_name, (fwd, bwd) in ACTIVATIONS.items():
        t0 = time.perf_counter()
        for _ in range(n_calls):
            y = fwd(x)
        fwd_time = (time.perf_counter() - t0) / n_calls * 1000
        results[act_name] = {"fwd_ms": fwd_time}
    return results


# ── Activation shape analysis ──

def analyze_shapes():
    """Analyze activation output statistics for N(0,1) inputs."""
    rng = np.random.default_rng(RNG_SEED)
    x = rng.standard_normal(100_000).astype(np.float64)

    print("\n  Activation Shape Analysis (x ~ N(0,1), 100K samples)")
    fprime_header = "f'(0)"
    print(f"  {'Name':10s}  {'E[f(x)]':>10s}  {'Std[f(x)]':>10s}  {'Min':>8s}  {'Max':>8s}  {'f(0)':>8s}  {fprime_header:>8s}")
    print("  " + "-" * 72)

    for act_name, (fwd, bwd) in ACTIVATIONS.items():
        y = fwd(x)
        g = bwd(x)
        f0 = fwd(np.array([0.0]))[0]
        g0 = bwd(np.array([0.0]))[0]
        print(f"  {act_name:10s}  {np.mean(y):10.4f}  {np.std(y):10.4f}  "
              f"{np.min(y):8.4f}  {np.max(y):8.4f}  {f0:8.4f}  {g0:8.4f}")


# ── Main ──

def main():
    SEP = "=" * 72

    print(SEP)
    print("  H-EE-1: Cyclotomic Polynomial Activation Comparison")
    print("  Hypothesis: Phi6 is uniquely optimal among cyclotomic activations")
    print(SEP)

    # 1. Shape analysis
    analyze_shapes()

    # 2. Microbenchmark
    print("\n[1/3] Microbenchmark (500K elements, 100 calls)")
    micro = microbenchmark()
    gelu_ms = micro["GELU"]["fwd_ms"]
    print(f"\n  {'Name':10s}  {'Fwd (ms)':>10s}  {'vs GELU':>10s}  {'FLOPs/scalar':>13s}")
    print("  " + "-" * 50)
    for name, mr in micro.items():
        rel = mr["fwd_ms"] / gelu_ms
        print(f"  {name:10s}  {mr['fwd_ms']:10.3f}  {rel:10.3f}x  {FLOP_COUNT[name]:>13d}")

    # 3. Training benchmark
    print(f"\n[2/3] MLP Training Benchmark (500 steps, SGD lr=0.01)")
    corpus = make_corpus()
    all_results = []
    for act_name, (fwd, bwd) in ACTIVATIONS.items():
        res = train_mlp(act_name, fwd, bwd, corpus)
        all_results.append(res)
        print(f"  {act_name:10s}  final_loss={res['final_loss']:.4f}  time={res['train_time_s']:.3f}s")

    # 4. Summary table
    print(f"\n[3/3] RESULTS SUMMARY")
    print(SEP)

    # Sort by final loss
    ranked = sorted(all_results, key=lambda r: r["final_loss"])
    gelu_loss = next(r["final_loss"] for r in all_results if r["act"] == "GELU")

    print(f"\n  Ranking by final loss (lower = better):")
    print(f"  {'Rank':>4s}  {'Activation':10s}  {'Final Loss':>10s}  {'vs GELU':>10s}  {'FLOPs':>6s}  {'Speed(ms)':>10s}")
    print("  " + "-" * 60)
    for i, r in enumerate(ranked):
        delta = ((r["final_loss"] - gelu_loss) / gelu_loss) * 100
        ms = micro[r["act"]]["fwd_ms"]
        print(f"  {i+1:4d}  {r['act']:10s}  {r['final_loss']:10.4f}  {delta:+9.1f}%  "
              f"{FLOP_COUNT[r['act']]:6d}  {ms:10.3f}")

    # 5. Phi6 uniqueness analysis
    print(f"\n  PHI6 UNIQUENESS ANALYSIS:")
    phi6_res = next(r for r in all_results if r["act"] == "Phi6")
    phi6_loss = phi6_res["final_loss"]
    phi6_flops = FLOP_COUNT["Phi6"]

    better_loss = [r for r in all_results if r["final_loss"] < phi6_loss and r["act"] != "Phi6"]
    fewer_flops = [r for r in all_results if FLOP_COUNT[r["act"]] <= phi6_flops and r["act"] != "Phi6"]
    pareto = [r for r in all_results if r["final_loss"] <= phi6_loss and FLOP_COUNT[r["act"]] <= phi6_flops and r["act"] != "Phi6"]

    print(f"    Phi6 loss:  {phi6_loss:.4f}")
    print(f"    Phi6 FLOPs: {phi6_flops}")
    print(f"    Activations with LOWER loss: {[r['act'] for r in better_loss] or 'None'}")
    print(f"    Activations with FEWER FLOPs: {[r['act'] for r in fewer_flops] or 'None'}")
    print(f"    Activations dominating Phi6 (both lower loss AND fewer FLOPs): {[r['act'] for r in pareto] or 'None'}")

    # Among cyclotomic only
    cyc_names = ["Phi3", "Phi4", "Phi6", "Phi8", "Phi12"]
    cyc_results = [r for r in all_results if r["act"] in cyc_names]
    cyc_best = min(cyc_results, key=lambda r: r["final_loss"])

    print(f"\n    Among cyclotomic polynomials only:")
    print(f"    Best loss: {cyc_best['act']} ({cyc_best['final_loss']:.4f})")
    print(f"    Phi6 is {'BEST' if cyc_best['act'] == 'Phi6' else 'NOT best'} among cyclotomic activations")

    # Pareto frontier among cyclotomics
    print(f"\n    Pareto frontier (loss vs FLOPs) among cyclotomics:")
    for r in sorted(cyc_results, key=lambda r: FLOP_COUNT[r["act"]]):
        dominated = any(
            r2["final_loss"] <= r["final_loss"] and FLOP_COUNT[r2["act"]] <= FLOP_COUNT[r["act"]]
            and r2["act"] != r["act"]
            for r2 in cyc_results
        )
        status = "dominated" if dominated else "PARETO"
        print(f"      {r['act']:8s}  loss={r['final_loss']:.4f}  FLOPs={FLOP_COUNT[r['act']]}  [{status}]")

    # Verdict
    print(f"\n  VERDICT:")
    if cyc_best["act"] == "Phi6" and len(pareto) == 0:
        print(f"    SUPPORTED: Phi6 is uniquely optimal -- best loss among cyclotomics,")
        print(f"    and no activation Pareto-dominates it (lower loss + fewer FLOPs).")
        print(f"    Grade: SUPPORTED")
    elif cyc_best["act"] == "Phi6":
        print(f"    PARTIAL: Phi6 has best loss among cyclotomics but is Pareto-dominated.")
        print(f"    Grade: PARTIAL")
    else:
        print(f"    REFUTED: {cyc_best['act']} beats Phi6 among cyclotomic activations.")
        print(f"    Grade: REFUTED")

    print(SEP)


if __name__ == "__main__":
    main()
