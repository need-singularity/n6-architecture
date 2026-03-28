"""
Anima Tension Loss
===================
PureField dual-engine concept: Engine A (standard) vs Engine G (adversarial).
Tension = |A - G|^2 serves as meta-loss regularizer.
High tension = internal conflict = wasted computation.
Stable tension near 1.0 = homeostatic optimum.
"""

import torch
import torch.nn as nn
import torch.nn.functional as F
import numpy as np
import math

SEED = 42
torch.manual_seed(SEED)


class TensionWrapper(nn.Module):
    def __init__(self, model):
        super().__init__()
        self.model = model
        self.tension_history = []

    def forward(self, x):
        out_a = self.model(x)
        with torch.no_grad():
            out_g = self._adversarial_forward(x)
        tension = (out_a - out_g).pow(2).mean()
        self.tension_history.append(tension.item())
        return out_a, tension

    def _adversarial_forward(self, x):
        final_layer = None
        for module in reversed(list(self.model.modules())):
            if isinstance(module, nn.Linear) and module.bias is not None:
                final_layer = module
                break

        if final_layer is None:
            out = self.model(x)
            return out + torch.randn_like(out) * 0.1

        original_bias = final_layer.bias.data.clone()
        final_layer.bias.data = -original_bias
        out_g = self.model(x)
        final_layer.bias.data = original_bias
        return out_g

    def get_tension_stats(self):
        if not self.tension_history:
            return {"mean": 0, "std": 0, "current": 0}
        recent = self.tension_history[-20:]
        return {
            "mean": np.mean(recent),
            "std": np.std(recent),
            "current": self.tension_history[-1],
            "trend": np.mean(recent[-5:]) - np.mean(recent[:5]) if len(recent) >= 10 else 0,
        }


def tension_meta_loss(task_loss, tension, alpha=0.01):
    return task_loss + alpha * tension


def homeostasis_target(tension, setpoint=1.0, deadband=0.3):
    deviation = abs(tension - setpoint)
    if deviation <= deadband:
        return 0.0
    return (deviation - deadband) ** 2


def main():
    print("=" * 70)
    print("  Anima Tension Loss")
    print("  PureField: |Engine_A - Engine_G|^2 as meta-loss")
    print("=" * 70)

    class SimpleModel(nn.Module):
        def __init__(self, d_in, d_hidden, d_out):
            super().__init__()
            self.fc1 = nn.Linear(d_in, d_hidden)
            self.fc2 = nn.Linear(d_hidden, d_out)

        def forward(self, x):
            return self.fc2(F.relu(self.fc1(x)))

    D_IN, D_HIDDEN, D_OUT = 64, 128, 10
    model = SimpleModel(D_IN, D_HIDDEN, D_OUT)
    wrapped = TensionWrapper(model)

    X = torch.randn(100, D_IN)
    Y = torch.randint(0, D_OUT, (100,))

    opt = torch.optim.Adam(wrapped.parameters(), lr=1e-3)

    print("\n--- Training with Tension Meta-Loss ---")
    print(f"{'Step':>5} {'Task Loss':>10} {'Tension':>10} {'Total':>10} {'Homeo':>10}")
    print("-" * 50)

    for step in range(100):
        idx = torch.randint(0, 100, (16,))
        x, y = X[idx], Y[idx]

        out, tension = wrapped(x)
        task_loss = F.cross_entropy(out, y)

        alpha = 0.001 + (0.05 - 0.001) * step / 100
        total_loss = tension_meta_loss(task_loss, tension, alpha)

        opt.zero_grad()
        total_loss.backward()
        opt.step()

        homeo = homeostasis_target(tension.item())

        if step % 20 == 0 or step == 99:
            print(f"{step:>5} {task_loss.item():>10.4f} {tension.item():>10.4f} "
                  f"{total_loss.item():>10.4f} {homeo:>10.4f}")

    stats = wrapped.get_tension_stats()
    print(f"\n--- Tension Statistics ---")
    print(f"Mean tension: {stats['mean']:.4f}")
    print(f"Std tension:  {stats['std']:.4f}")
    print(f"Trend:        {stats['trend']:+.4f}")
    print(f"\nSetpoint: 1.0 (Anima homeostatic target)")
    print(f"Deadband: +/- 0.3")
    print(f"Tension regularizes by penalizing internal conflict.")


if __name__ == "__main__":
    main()
