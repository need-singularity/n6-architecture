#!/usr/bin/env python3
"""H-SEDI-6: R-filter detects training phase transitions.
Apply windowed FFT (window=6,12,24,36) to per-batch loss curve.
Look for spectral peaks at 1/6, 1/4 frequencies indicating phase transitions.
"""
import sys, os; sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__)))); sys.path.insert(0, '/Users/ghost/Dev/TECS-L')
import torch, torch.nn as nn, numpy as np
from model_pure_field import PureFieldEngine
from model_utils import load_mnist

def windowed_fft(signal, window_size):
    """SEDI-style R-filter: windowed FFT, return max spectral ratio per window."""
    n = len(signal)
    ratios = []
    for i in range(0, n - window_size + 1, window_size // 2):
        chunk = signal[i:i + window_size]
        spec = np.abs(np.fft.rfft(chunk - np.mean(chunk)))
        if len(spec) > 1 and spec[1:].max() > 1e-10:
            ratios.append(spec.max() / np.median(spec[1:] + 1e-12))
        else:
            ratios.append(0.0)
    return np.array(ratios)

def detect_peaks(ratios, threshold=3.0):
    """Detect windows where spectral ratio exceeds threshold (phase transition)."""
    return np.where(ratios > threshold)[0]

def main():
    print("=" * 60)
    print("H-SEDI-6: R-filter Phase Transition Detection")
    print("=" * 60)
    train_loader, test_loader = load_mnist(batch_size=64)
    model = PureFieldEngine(784, 128, 10)
    optimizer = torch.optim.Adam(model.parameters(), lr=0.001)
    criterion = nn.CrossEntropyLoss()

    # Train 3 epochs, log per-batch loss
    batch_losses = []
    for epoch in range(3):
        model.train()
        for X, y in train_loader:
            X = X.view(X.size(0), -1)
            optimizer.zero_grad()
            out, tension = model(X)
            loss = criterion(out, y)
            loss.backward()
            optimizer.step()
            batch_losses.append(loss.item())
        print(f"  Epoch {epoch+1}: batches={len(batch_losses)}, last_loss={batch_losses[-1]:.4f}")

    losses = np.array(batch_losses)
    print(f"\nTotal batches: {len(losses)}")
    print(f"Loss range: {losses.min():.4f} - {losses.max():.4f}")
    print(f"Epoch-1 mean: {losses[:len(losses)//3].mean():.4f}, Epoch-3 mean: {losses[2*len(losses)//3:].mean():.4f}")

    # Apply R-filter at each window size
    print(f"\n{'Window':>8} {'Peaks':>6} {'MaxRatio':>10} {'PeakBatches':>30}")
    print("-" * 60)
    results = {}
    for w in [6, 12, 24, 36]:
        ratios = windowed_fft(losses, w)
        peaks = detect_peaks(ratios, threshold=3.0)
        results[w] = {'n_peaks': len(peaks), 'max_ratio': ratios.max() if len(ratios) > 0 else 0,
                       'peak_positions': peaks[:5]}
        peak_str = str(peaks[:5].tolist()) if len(peaks) > 0 else "none"
        print(f"{w:>8} {len(peaks):>6} {ratios.max():>10.2f} {peak_str:>30}")

    # Check: do peaks cluster in early batches (epoch 1)?
    n_epoch1 = len(losses) // 3
    print(f"\n--- Phase Transition Analysis ---")
    for w in [6, 12, 24, 36]:
        r = results[w]
        if r['n_peaks'] > 0:
            early = sum(1 for p in r['peak_positions'] if p * (w // 2) < n_epoch1)
            print(f"  Window={w}: {early}/{r['n_peaks']} peaks in epoch 1 (first {n_epoch1} batches)")
        else:
            print(f"  Window={w}: no peaks detected")

    # Spectral content at key frequencies
    print(f"\n--- Spectral Power at Key Frequencies ---")
    full_spec = np.abs(np.fft.rfft(losses - losses.mean()))
    freqs = np.fft.rfftfreq(len(losses))
    for target_f, label in [(1/6, "1/6"), (1/4, "1/4"), (1/3, "1/3")]:
        idx = np.argmin(np.abs(freqs - target_f))
        print(f"  f={label}: power={full_spec[idx]:.2f} (idx={idx}, actual_f={freqs[idx]:.4f})")
    print(f"  Max spectral power: {full_spec[1:].max():.2f} at f={freqs[1+np.argmax(full_spec[1:])]:.4f}")

    # Verdict
    has_transition = any(r['max_ratio'] > 5.0 for r in results.values())
    early_peaks = any(
        sum(1 for p in r['peak_positions'] if p * (w // 2) < n_epoch1) > 0
        for w, r in zip([6,12,24,36], results.values()) if r['n_peaks'] > 0
    )
    print(f"\n{'='*60}")
    print(f"VERDICT: R-filter detects phase transition = {has_transition}")
    print(f"         Early-epoch clustering = {early_peaks}")
    print(f"         Status: {'SUPPORTED' if has_transition and early_peaks else 'PARTIAL' if has_transition or early_peaks else 'REFUTED'}")

if __name__ == '__main__':
    main()
