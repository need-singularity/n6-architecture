"""
Particle Decay Branching Ratios vs TECS-L Egyptian Fractions

TECS-L's Egyptian fraction identity for n=6: 1/2 + 1/3 + 1/6 = 1
Particle decay branching ratios sum to 1 by definition.

This module checks whether any known particle's major decay channels
have branching ratios that match n=6 Egyptian fraction decompositions.

Data source: Particle Data Group (PDG) 2024 Review of Particle Physics.
"""

from __future__ import annotations

import itertools
import math
import random
from dataclasses import dataclass, field
from typing import Dict, List, Tuple

import numpy as np


# ---------------------------------------------------------------------------
# Data: PDG branching ratios (percentages)
# ---------------------------------------------------------------------------

@dataclass
class DecayChannel:
    label: str
    br_pct: float  # branching ratio in percent

    @property
    def br(self) -> float:
        """Branching ratio as a fraction."""
        return self.br_pct / 100.0


@dataclass
class Particle:
    name: str
    channels: List[DecayChannel] = field(default_factory=list)

    @property
    def n_channels(self) -> int:
        return len(self.channels)


def build_particle_database() -> List[Particle]:
    """Comprehensive PDG branching ratio database."""
    particles = []

    # --- Z boson ---
    particles.append(Particle("Z boson", [
        DecayChannel("hadrons", 69.91),
        DecayChannel("invisible (nu nu)", 20.00),
        DecayChannel("tau+ tau-", 3.370),
        DecayChannel("mu+ mu-", 3.366),
        DecayChannel("e+ e-", 3.363),
    ]))

    # --- W boson ---
    particles.append(Particle("W boson", [
        DecayChannel("hadrons", 67.41),
        DecayChannel("tau nu_tau", 11.38),
        DecayChannel("e nu_e", 10.71),
        DecayChannel("mu nu_mu", 10.63),
    ]))

    # --- Higgs boson ---
    particles.append(Particle("Higgs boson", [
        DecayChannel("b b-bar", 58.2),
        DecayChannel("W W*", 21.4),
        DecayChannel("g g", 8.18),
        DecayChannel("tau+ tau-", 6.27),
        DecayChannel("c c-bar", 2.89),
        DecayChannel("Z Z*", 2.62),
    ]))

    # --- J/psi ---
    particles.append(Particle("J/psi", [
        DecayChannel("hadrons", 87.7),
        DecayChannel("e+ e-", 5.97),
        DecayChannel("mu+ mu-", 5.96),
    ]))

    # --- Upsilon(1S) ---
    particles.append(Particle("Upsilon(1S)", [
        DecayChannel("g g g", 81.7),
        DecayChannel("tau+ tau-", 2.60),
        DecayChannel("mu+ mu-", 2.48),
        DecayChannel("e+ e-", 2.38),
        DecayChannel("gamma g g", 2.2),
    ]))

    # --- phi(1020) ---
    particles.append(Particle("phi(1020)", [
        DecayChannel("K+ K-", 49.2),
        DecayChannel("K_L K_S", 34.0),
        DecayChannel("rho pi / pi+pi-pi0", 15.3),
    ]))

    # --- eta ---
    particles.append(Particle("eta", [
        DecayChannel("gamma gamma", 39.4),
        DecayChannel("3 pi0", 32.6),
        DecayChannel("pi+ pi- pi0", 22.7),
        DecayChannel("pi+ pi- gamma", 4.2),
    ]))

    # --- pi0 ---
    particles.append(Particle("pi0", [
        DecayChannel("gamma gamma", 98.8),
        DecayChannel("e+ e- gamma", 1.17),
    ]))

    # --- rho(770) ---
    particles.append(Particle("rho(770)", [
        DecayChannel("pi pi", 99.9),
    ]))

    # --- omega(782) ---
    particles.append(Particle("omega(782)", [
        DecayChannel("pi+ pi- pi0", 89.2),
        DecayChannel("pi0 gamma", 8.28),
        DecayChannel("pi+ pi-", 1.53),
    ]))

    # --- eta'(958) ---
    particles.append(Particle("eta'(958)", [
        DecayChannel("pi+ pi- eta", 42.6),
        DecayChannel("rho0 gamma", 28.9),
        DecayChannel("pi0 pi0 eta", 22.8),
    ]))

    # --- D0 (major channels) ---
    particles.append(Particle("D0", [
        DecayChannel("K- pi+", 3.950),
        DecayChannel("K- pi+ pi0", 14.4),
        DecayChannel("K- pi+ pi+ pi-", 8.22),
        DecayChannel("K_S pi+ pi-", 2.99),
        DecayChannel("K- e+ nu_e", 3.538),
        DecayChannel("K- mu+ nu_mu", 3.41),
    ]))

    # --- D+ ---
    particles.append(Particle("D+", [
        DecayChannel("K- pi+ pi+", 9.38),
        DecayChannel("K_S pi+", 1.55),
        DecayChannel("K- pi+ pi+ pi0", 6.10),
        DecayChannel("K_S pi+ pi0", 6.99),
        DecayChannel("K_S pi+ pi+ pi-", 3.12),
    ]))

    # --- B0 (major channels) ---
    particles.append(Particle("B0", [
        DecayChannel("D*- l+ nu", 4.97),
        DecayChannel("D- l+ nu", 2.19),
        DecayChannel("J/psi K_S", 0.0437),
        DecayChannel("pi+ pi-", 0.000517),
        DecayChannel("inclusive charm", 79.0),
    ]))

    # --- B+ ---
    particles.append(Particle("B+", [
        DecayChannel("D0-bar l+ nu", 2.27),
        DecayChannel("D*0-bar l+ nu", 5.58),
        DecayChannel("J/psi K+", 0.1009),
        DecayChannel("inclusive charm", 78.0),
    ]))

    # --- Lambda ---
    particles.append(Particle("Lambda", [
        DecayChannel("p pi-", 63.9),
        DecayChannel("n pi0", 35.8),
    ]))

    # --- Sigma+ ---
    particles.append(Particle("Sigma+", [
        DecayChannel("p pi0", 51.57),
        DecayChannel("n pi+", 48.31),
    ]))

    # --- Sigma- ---
    particles.append(Particle("Sigma-", [
        DecayChannel("n pi-", 99.85),
    ]))

    # --- Xi0 ---
    particles.append(Particle("Xi0", [
        DecayChannel("Lambda pi0", 99.52),
    ]))

    # --- Xi- ---
    particles.append(Particle("Xi-", [
        DecayChannel("Lambda pi-", 99.89),
    ]))

    # --- Omega- ---
    particles.append(Particle("Omega-", [
        DecayChannel("Lambda K-", 67.8),
        DecayChannel("Xi0 pi-", 23.6),
        DecayChannel("Xi- pi0", 8.6),
    ]))

    # --- neutron ---
    particles.append(Particle("neutron", [
        DecayChannel("p e- nu_e-bar", 100.0),
    ]))

    # --- K+ ---
    particles.append(Particle("K+", [
        DecayChannel("mu+ nu_mu", 63.56),
        DecayChannel("pi+ pi0", 20.67),
        DecayChannel("pi+ pi+ pi-", 5.583),
        DecayChannel("pi0 e+ nu_e", 5.07),
        DecayChannel("pi0 mu+ nu_mu", 3.352),
    ]))

    # --- K_S ---
    particles.append(Particle("K_S", [
        DecayChannel("pi+ pi-", 69.20),
        DecayChannel("pi0 pi0", 30.69),
    ]))

    # --- K_L ---
    particles.append(Particle("K_L", [
        DecayChannel("pi+- e-+ nu", 40.55),
        DecayChannel("pi+- mu-+ nu", 27.04),
        DecayChannel("3 pi0", 19.52),
        DecayChannel("pi+ pi- pi0", 12.54),
    ]))

    # --- tau lepton ---
    particles.append(Particle("tau lepton", [
        DecayChannel("hadrons (1-prong)", 49.52),
        DecayChannel("hadrons (3-prong)", 15.18),
        DecayChannel("e nu_e nu_tau", 17.82),
        DecayChannel("mu nu_mu nu_tau", 17.39),
    ]))

    # --- muon ---
    particles.append(Particle("muon", [
        DecayChannel("e nu_e nu_mu", 100.0),
    ]))

    # --- top quark ---
    particles.append(Particle("top quark", [
        DecayChannel("W b", 99.9),
    ]))

    return particles


# ---------------------------------------------------------------------------
# TECS-L constants
# ---------------------------------------------------------------------------

TECS_N = 6
TECS_PHI = (1 + math.sqrt(5)) / 2  # golden ratio
TECS_TAU = 4   # tau(6) = number of divisors
TECS_SIGMA = 12  # sigma(6) = sum of divisors
TECS_SOPFR = 5  # sopfr(6) = sum of prime factors with repetition

# Key fractions from TECS-L
EGYPTIAN_FRACTIONS = {
    "1/2 + 1/3 + 1/6 = 1": [1/2, 1/3, 1/6],
    "1/2 + 1/4 + 1/4 = 1": [1/2, 1/4, 1/4],
    "1/3 + 1/3 + 1/3 = 1": [1/3, 1/3, 1/3],
    "2/3 + 1/6 + 1/6 = 1": [2/3, 1/6, 1/6],
    "5/6 + 1/12 + 1/12 = 1": [5/6, 1/12, 1/12],
}

# Individual TECS-L fractions with interpretations
TECS_FRACTIONS = {
    "1/2 = phi/tau":    (1/2, "phi/tau"),
    "1/3 = tau/sigma":  (1/3, "tau/sigma"),
    "1/6 = 1/n":        (1/6, "1/n"),
    "5/6 = sopfr/n":    (5/6, "sopfr/n"),
    "1/4 = 1/tau":      (1/4, "1/tau"),
    "1/12 = 1/sigma":   (1/12, "1/sigma"),
    "2/3 = sigma/tau^2 or 2*tau/sigma": (2/3, "2/3"),
}


# ---------------------------------------------------------------------------
# Analysis functions
# ---------------------------------------------------------------------------

def check_egyptian_match(
    brs: List[float],
    target: List[float],
    tol: float = 0.10,
) -> Tuple[bool, float, List[int]]:
    """
    Check if a combination of 3 branching ratios, when normalized to sum=1,
    matches a target Egyptian fraction set {f1, f2, f3}.

    Returns (match, max_relative_error, best_permutation_indices).
    """
    s = sum(brs)
    if s < 1e-12:
        return False, float('inf'), []

    normed = [b / s for b in brs]

    best_err = float('inf')
    best_perm = None

    # Try all permutations of targets against the sorted normed values
    sorted_normed = sorted(normed, reverse=True)
    for perm in itertools.permutations(target):
        sorted_perm = list(perm)
        # compute max relative error
        max_rel = 0.0
        for n_val, t_val in zip(sorted_normed, sorted_perm):
            if t_val < 1e-12:
                if n_val > 1e-6:
                    max_rel = float('inf')
                continue
            rel = abs(n_val - t_val) / t_val
            max_rel = max(max_rel, rel)
        if max_rel < best_err:
            best_err = max_rel
            best_perm = sorted_perm

    return best_err <= tol, best_err, best_perm


def check_individual_fraction(br: float, target: float, tol: float = 0.10) -> Tuple[bool, float]:
    """Check if a single BR matches a target fraction within tolerance."""
    if target < 1e-12:
        return False, float('inf')
    rel = abs(br - target) / target
    return rel <= tol, rel


def mc_probability(
    n_channels: int,
    target_fracs: List[float],
    n_trials: int = 100_000,
    tol: float = 0.10,
) -> float:
    """
    Monte Carlo: probability that random branching ratios (Dirichlet)
    with `n_channels` channels produce 3 channels matching `target_fracs`.
    """
    if n_channels < 3:
        return 0.0

    rng = np.random.default_rng(42)
    hits = 0

    # Use Dirichlet with uniform alpha=1 (flat distribution over simplex)
    alpha = np.ones(n_channels)

    for _ in range(n_trials):
        sample = rng.dirichlet(alpha)
        sample_sorted = sorted(sample, reverse=True)

        # Check all 3-combinations from top channels
        n_check = min(n_channels, 6)  # only check top channels for speed
        for combo in itertools.combinations(range(n_check), 3):
            brs = [sample_sorted[i] for i in combo]
            match, _, _ = check_egyptian_match(brs, target_fracs, tol)
            if match:
                hits += 1
                break  # count once per trial

    return hits / n_trials


def mc_individual_probability(
    n_channels: int,
    target_frac: float,
    n_trials: int = 100_000,
    tol: float = 0.10,
) -> float:
    """
    Monte Carlo: probability that any channel in random branching ratios
    matches a target fraction.
    """
    if n_channels < 1:
        return 0.0

    rng = np.random.default_rng(42)
    hits = 0
    alpha = np.ones(n_channels)

    for _ in range(n_trials):
        sample = rng.dirichlet(alpha)
        for br in sample:
            match, _ = check_individual_fraction(br, target_frac, tol)
            if match:
                hits += 1
                break

    return hits / n_trials


# ---------------------------------------------------------------------------
# Main analysis
# ---------------------------------------------------------------------------

@dataclass
class EgyptianMatch:
    particle: str
    channels: List[str]
    brs: List[float]
    pattern_name: str
    target: List[float]
    max_rel_error: float
    mc_prob: float = 0.0
    significance: float = 0.0  # -log10(mc_prob)


@dataclass
class IndividualMatch:
    particle: str
    channel: str
    br: float
    fraction_name: str
    target: float
    rel_error: float
    mc_prob: float = 0.0
    significance: float = 0.0


def run_analysis(verbose: bool = True) -> Tuple[List[EgyptianMatch], List[IndividualMatch]]:
    """Run the full branching ratio analysis."""
    particles = build_particle_database()

    egyptian_matches: List[EgyptianMatch] = []
    individual_matches: List[IndividualMatch] = []

    if verbose:
        print("=" * 78)
        print("PARTICLE DECAY BRANCHING RATIOS vs TECS-L EGYPTIAN FRACTIONS")
        print("=" * 78)
        print()
        print("TECS-L identity for n=6:  1/2 + 1/3 + 1/6 = 1")
        print("Particle branching ratios sum to 1 by definition.")
        print()

    # -----------------------------------------------------------------------
    # Part 1: Print database
    # -----------------------------------------------------------------------
    if verbose:
        print("-" * 78)
        print("PARTICLE DECAY DATABASE (PDG 2024)")
        print("-" * 78)
        for p in particles:
            print(f"\n  {p.name}:")
            for ch in p.channels:
                print(f"    {ch.label:30s}  {ch.br_pct:8.4f}%  ({ch.br:.6f})")
        print()

    # -----------------------------------------------------------------------
    # Part 2: Egyptian fraction matching
    # -----------------------------------------------------------------------
    if verbose:
        print("=" * 78)
        print("EGYPTIAN FRACTION MATCHING (3-channel combinations)")
        print(f"Tolerance: 10% relative error on each fraction")
        print("=" * 78)

    for p in particles:
        if p.n_channels < 3:
            continue

        brs_list = [(ch.label, ch.br) for ch in p.channels]

        for combo in itertools.combinations(range(p.n_channels), 3):
            combo_labels = [brs_list[i][0] for i in combo]
            combo_brs = [brs_list[i][1] for i in combo]

            for pat_name, target in EGYPTIAN_FRACTIONS.items():
                match, err, best_perm = check_egyptian_match(combo_brs, target, tol=0.10)
                if match:
                    em = EgyptianMatch(
                        particle=p.name,
                        channels=combo_labels,
                        brs=combo_brs,
                        pattern_name=pat_name,
                        target=target,
                        max_rel_error=err,
                    )
                    egyptian_matches.append(em)

    # -----------------------------------------------------------------------
    # Part 3: Individual fraction matching
    # -----------------------------------------------------------------------
    if verbose:
        print()
        print("=" * 78)
        print("INDIVIDUAL TECS-L FRACTION MATCHING")
        print("=" * 78)

    for p in particles:
        for ch in p.channels:
            for frac_name, (target, interp) in TECS_FRACTIONS.items():
                match, rel = check_individual_fraction(ch.br, target, tol=0.10)
                if match:
                    im = IndividualMatch(
                        particle=p.name,
                        channel=ch.label,
                        br=ch.br,
                        fraction_name=frac_name,
                        target=target,
                        rel_error=rel,
                    )
                    individual_matches.append(im)

    # -----------------------------------------------------------------------
    # Part 4: MC validation
    # -----------------------------------------------------------------------
    if verbose:
        print()
        print("=" * 78)
        print("MONTE CARLO VALIDATION (100k trials per match)")
        print("=" * 78)
        print()

    # MC for Egyptian matches
    seen_mc_eg = {}
    for em in egyptian_matches:
        # find the particle to get n_channels
        p = next(pp for pp in particles if pp.name == em.particle)
        key = (p.name, tuple(sorted(em.target)))
        if key not in seen_mc_eg:
            prob = mc_probability(p.n_channels, em.target, n_trials=100_000)
            seen_mc_eg[key] = prob
        em.mc_prob = seen_mc_eg[key]
        em.significance = -math.log10(em.mc_prob) if em.mc_prob > 0 else 99.0

    # MC for individual matches
    seen_mc_ind = {}
    for im in individual_matches:
        p = next(pp for pp in particles if pp.name == im.particle)
        key = (p.name, im.target)
        if key not in seen_mc_ind:
            prob = mc_individual_probability(p.n_channels, im.target, n_trials=100_000)
            seen_mc_ind[key] = prob
        im.mc_prob = seen_mc_ind[key]
        im.significance = -math.log10(im.mc_prob) if im.mc_prob > 0 else 99.0

    # -----------------------------------------------------------------------
    # Part 5: Report
    # -----------------------------------------------------------------------
    if verbose:
        # Egyptian matches sorted by significance
        egyptian_matches.sort(key=lambda m: -m.significance)

        print("-" * 78)
        print("EGYPTIAN FRACTION MATCHES (sorted by significance)")
        print("-" * 78)
        if not egyptian_matches:
            print("  No matches found.")
        for i, em in enumerate(egyptian_matches, 1):
            s = sum(em.brs)
            normed = [b / s for b in em.brs]
            normed_sorted = sorted(normed, reverse=True)
            print(f"\n  #{i}  {em.particle}")
            print(f"      Pattern: {em.pattern_name}")
            print(f"      Channels:")
            for lbl, br in zip(em.channels, em.brs):
                print(f"        {lbl:30s}  BR = {br:.6f}")
            print(f"      Normalized (sum={s:.6f}):")
            for ns in normed_sorted:
                print(f"        {ns:.6f}")
            print(f"      Target fractions: {[f'{t:.6f}' for t in sorted(em.target, reverse=True)]}")
            print(f"      Max relative error: {em.max_rel_error:.4f} ({em.max_rel_error*100:.2f}%)")
            print(f"      MC probability (random): {em.mc_prob:.6f}")
            print(f"      Significance: {em.significance:.2f} sigma (-log10 p)")

        print()
        print("-" * 78)
        print("INDIVIDUAL FRACTION MATCHES (sorted by significance)")
        print("-" * 78)

        individual_matches.sort(key=lambda m: -m.significance)
        if not individual_matches:
            print("  No matches found.")
        for i, im in enumerate(individual_matches, 1):
            print(f"\n  #{i}  {im.particle} -> {im.channel}")
            print(f"      BR = {im.br:.6f},  Target = {im.target:.6f}  ({im.fraction_name})")
            print(f"      Relative error: {im.rel_error:.4f} ({im.rel_error*100:.2f}%)")
            print(f"      MC probability: {im.mc_prob:.6f}")
            print(f"      Significance: {im.significance:.2f} sigma")

        # Summary
        print()
        print("=" * 78)
        print("SUMMARY")
        print("=" * 78)
        print()

        if egyptian_matches:
            best = egyptian_matches[0]
            print(f"  Best Egyptian fraction match:")
            print(f"    {best.particle}: {best.pattern_name}")
            print(f"    Channels: {', '.join(best.channels)}")
            print(f"    Max relative error: {best.max_rel_error*100:.2f}%")
            print(f"    MC probability: {best.mc_prob:.6f}")
            print()

        # Check specifically for the 1/2+1/3+1/6 pattern
        key_matches = [m for m in egyptian_matches
                       if m.target == [1/2, 1/3, 1/6]]
        if key_matches:
            print(f"  Matches for the KEY pattern 1/2 + 1/3 + 1/6:")
            for m in key_matches:
                print(f"    {m.particle}: {', '.join(m.channels)} "
                      f"(err={m.max_rel_error*100:.2f}%, "
                      f"MC p={m.mc_prob:.6f})")
        else:
            print("  No matches for the key pattern 1/2 + 1/3 + 1/6.")

        print()

        # Highlight phi(1020) specifically
        phi_matches = [m for m in egyptian_matches if m.particle == "phi(1020)"]
        if phi_matches:
            print(f"  phi(1020) matches:")
            for m in phi_matches:
                s = sum(m.brs)
                normed = sorted([b/s for b in m.brs], reverse=True)
                print(f"    Pattern: {m.pattern_name}")
                print(f"    Normalized BRs: {[f'{n:.4f}' for n in normed]}")
                print(f"    vs target:      {[f'{t:.4f}' for t in sorted(m.target, reverse=True)]}")
                print(f"    Error: {m.max_rel_error*100:.2f}%, MC p={m.mc_prob:.6f}")
            print()

        # Count unique particles
        unique_particles_eg = set(m.particle for m in egyptian_matches)
        unique_particles_ind = set(m.particle for m in individual_matches)
        print(f"  Total Egyptian fraction matches: {len(egyptian_matches)} "
              f"across {len(unique_particles_eg)} particles")
        print(f"  Total individual fraction matches: {len(individual_matches)} "
              f"across {len(unique_particles_ind)} particles")
        print()

        # Final verdict
        print("-" * 78)
        print("INTERPRETATION")
        print("-" * 78)
        print()
        print("  The Egyptian fraction 1/2 + 1/3 + 1/6 = 1 is a mathematical identity.")
        print("  Particle branching ratios are determined by coupling constants,")
        print("  phase space, and quantum numbers -- not number theory.")
        print()
        print("  Any matches found should be evaluated against their MC probability:")
        print("    - MC p > 0.05: consistent with random chance")
        print("    - MC p < 0.01: potentially interesting coincidence")
        print("    - MC p < 0.001: remarkable coincidence worth noting")
        print()

    return egyptian_matches, individual_matches


if __name__ == "__main__":
    run_analysis(verbose=True)
