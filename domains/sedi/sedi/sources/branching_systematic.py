"""
Systematic Branching Ratio Analysis: n=6 Fractions Across All Particles

Extends the initial Higgs bb+ττ finding (3.89σ joint Dirichlet) by scanning
ALL well-measured particle branching ratios against the complete set of n=6
fractions. Uses PDG 2024 values throughout.

Key questions:
  1. Which individual branching ratios match n=6 fractions within 3%?
  2. Which particles have MULTIPLE matches (joint significance)?
  3. Is the TOTAL count of matches across all particles anomalous?
"""

from __future__ import annotations

import itertools
import math
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple

import numpy as np
from scipy import stats


# ---------------------------------------------------------------------------
# Data structures
# ---------------------------------------------------------------------------

@dataclass
class DecayChannel:
    label: str
    br_pct: float          # branching ratio in percent

    @property
    def br(self) -> float:
        return self.br_pct / 100.0


@dataclass
class Particle:
    name: str
    channels: List[DecayChannel] = field(default_factory=list)

    @property
    def n_channels(self) -> int:
        return len(self.channels)


# ---------------------------------------------------------------------------
# n=6 target fractions
# ---------------------------------------------------------------------------

TARGET_FRACTIONS: Dict[str, float] = {
    "1/2":  1/2,
    "1/3":  1/3,
    "1/4":  1/4,
    "1/6":  1/6,
    "1/8":  1/8,
    "1/12": 1/12,
    "1/16": 1/16,
    "1/24": 1/24,
    "2/3":  2/3,
    "3/4":  3/4,
    "5/6":  5/6,
    "7/12": 7/12,
    "5/12": 5/12,
    "7/24": 7/24,
}

TOLERANCE = 0.03  # 3% relative error


# ---------------------------------------------------------------------------
# Comprehensive PDG 2024 particle database
# ---------------------------------------------------------------------------

def build_particle_database() -> List[Particle]:
    """Build database of 30+ particles with PDG 2024 branching ratios."""
    particles: List[Particle] = []

    # ===== LEPTONS =====

    # tau lepton - 17+ decay modes (PDG 2024)
    particles.append(Particle("tau", [
        DecayChannel("e- nu_e nu_tau",           17.82),
        DecayChannel("mu- nu_mu nu_tau",          17.39),
        DecayChannel("pi- nu_tau",                10.82),
        DecayChannel("pi- pi0 nu_tau (rho)",      25.49),
        DecayChannel("pi- 2pi0 nu_tau (a1)",       9.26),
        DecayChannel("pi- 3pi0 nu_tau",            1.04),
        DecayChannel("pi- pi+ pi- nu_tau (a1)",    8.99),
        DecayChannel("pi- pi+ pi- pi0 nu_tau",     2.74),
        DecayChannel("K- nu_tau",                   0.6955),
        DecayChannel("K- pi0 nu_tau",               0.433),
        DecayChannel("K*- nu_tau -> K-pi0",         0.314),
        DecayChannel("K- pi+ pi- nu_tau",           0.293),
        DecayChannel("K- pi0 pi0 nu_tau",           0.064),
        DecayChannel("K0 pi- nu_tau",               0.840),
        DecayChannel("K0 pi- pi0 nu_tau",           0.365),
        DecayChannel("eta pi- pi0 nu_tau",          0.139),
        DecayChannel("pi- 4pi0 nu_tau",             0.112),
        DecayChannel("3pi- 2pi+ nu_tau",            0.0824),
        DecayChannel("other / rare",                2.118),
    ]))

    # ===== GAUGE / HIGGS BOSONS =====

    # Z boson (PDG 2024)
    particles.append(Particle("Z boson", [
        DecayChannel("e+ e-",                3.3632),
        DecayChannel("mu+ mu-",              3.3662),
        DecayChannel("tau+ tau-",             3.3696),
        DecayChannel("invisible (nu nu)",    20.000),
        DecayChannel("u u-bar",              11.6),
        DecayChannel("d d-bar",              15.6),
        DecayChannel("c c-bar",              12.0),
        DecayChannel("s s-bar",              15.6),
        DecayChannel("b b-bar",              15.12),
    ]))

    # W boson (PDG 2024)
    particles.append(Particle("W boson", [
        DecayChannel("e nu_e",       10.71),
        DecayChannel("mu nu_mu",     10.63),
        DecayChannel("tau nu_tau",   11.38),
        DecayChannel("u d-bar",      33.7),
        DecayChannel("c s-bar",      33.7),
    ]))

    # Higgs boson (PDG 2024)
    particles.append(Particle("Higgs boson", [
        DecayChannel("b b-bar",       58.2),
        DecayChannel("W W*",          21.4),
        DecayChannel("g g",            8.18),
        DecayChannel("tau+ tau-",      6.27),
        DecayChannel("c c-bar",        2.89),
        DecayChannel("Z Z*",           2.62),
        DecayChannel("gamma gamma",    0.227),
        DecayChannel("Z gamma",        0.154),
        DecayChannel("mu+ mu-",        0.022),
    ]))

    # ===== VECTOR MESONS =====

    # rho(770) (PDG 2024)
    particles.append(Particle("rho(770)", [
        DecayChannel("pi+ pi-",        99.9),
        # virtually 100%, not useful for fraction matching
    ]))

    # omega(782) (PDG 2024)
    particles.append(Particle("omega(782)", [
        DecayChannel("pi+ pi- pi0",   89.2),
        DecayChannel("pi0 gamma",      8.40),
        DecayChannel("pi+ pi-",        1.53),
        DecayChannel("eta gamma",      0.452),
        DecayChannel("pi0 e+ e-",      0.077),
    ]))

    # phi(1020) (PDG 2024)
    particles.append(Particle("phi(1020)", [
        DecayChannel("K+ K-",         49.1),
        DecayChannel("K_L K_S",       34.0),
        DecayChannel("rho pi + pi+pi-pi0", 15.32),
        DecayChannel("eta gamma",      1.303),
        DecayChannel("pi0 gamma",      0.130),
        DecayChannel("e+ e-",          0.0297),
    ]))

    # J/psi (PDG 2024) — exclusive decomposition (non-overlapping)
    particles.append(Particle("J/psi", [
        DecayChannel("hadrons (ggg+gg gamma)",  86.0),
        DecayChannel("e+ e-",             5.971),
        DecayChannel("mu+ mu-",           5.961),
        DecayChannel("gamma eta_c(1S)",   1.7),
        DecayChannel("other / radiative",  0.4),
    ]))

    # Upsilon(1S) (PDG 2024)
    particles.append(Particle("Upsilon(1S)", [
        DecayChannel("g g g",            81.7),
        DecayChannel("gamma g g",         2.2),
        DecayChannel("tau+ tau-",         2.60),
        DecayChannel("mu+ mu-",           2.48),
        DecayChannel("e+ e-",             2.38),
    ]))

    # Upsilon(2S) (PDG 2024)
    particles.append(Particle("Upsilon(2S)", [
        DecayChannel("Upsilon(1S) pi+ pi-",  17.85),
        DecayChannel("Upsilon(1S) pi0 pi0",   8.6),
        DecayChannel("g g g",                 58.8),
        DecayChannel("gamma g g",              1.0),
        DecayChannel("gamma chi_b0(1P)",       3.8),
        DecayChannel("gamma chi_b1(1P)",       6.9),
        DecayChannel("gamma chi_b2(1P)",       7.15),
        DecayChannel("tau+ tau-",              0.7),
        DecayChannel("mu+ mu-",                1.93),
        DecayChannel("e+ e-",                  1.91),
    ]))

    # Upsilon(3S) (PDG 2024)
    particles.append(Particle("Upsilon(3S)", [
        DecayChannel("Upsilon(1S) pi+ pi-",   4.37),
        DecayChannel("Upsilon(1S) pi0 pi0",   2.20),
        DecayChannel("Upsilon(2S) pi+ pi-",   2.00),
        DecayChannel("Upsilon(2S) pi0 pi0",   1.07),
        DecayChannel("g g g",                 57.0),
        DecayChannel("gamma chi_b0(2P)",       5.9),
        DecayChannel("gamma chi_b1(2P)",      12.6),
        DecayChannel("gamma chi_b2(2P)",      13.1),
        DecayChannel("tau+ tau-",              2.3),
        DecayChannel("mu+ mu-",                2.18),
        DecayChannel("e+ e-",                  2.18),
    ]))

    # ===== PSEUDOSCALAR MESONS =====

    # pi0 (PDG 2024)
    particles.append(Particle("pi0", [
        DecayChannel("gamma gamma",      98.823),
        DecayChannel("e+ e- gamma",       1.174),
        DecayChannel("e+ e- e+ e-",       3.34e-3),
    ]))

    # eta (PDG 2024)
    particles.append(Particle("eta", [
        DecayChannel("gamma gamma",      39.41),
        DecayChannel("3 pi0",            32.68),
        DecayChannel("pi+ pi- pi0",      22.92),
        DecayChannel("pi+ pi- gamma",     4.22),
        DecayChannel("e+ e- gamma",       0.69),
    ]))

    # eta'(958) (PDG 2024)
    particles.append(Particle("eta'(958)", [
        DecayChannel("pi+ pi- eta",      42.5),
        DecayChannel("rho0 gamma",        28.9),
        DecayChannel("pi0 pi0 eta",       22.4),
        DecayChannel("omega gamma",        2.62),
        DecayChannel("gamma gamma",        2.35),
        DecayChannel("3 pi0",              2.54),
    ]))

    # K+ (PDG 2024)
    particles.append(Particle("K+", [
        DecayChannel("mu+ nu_mu",        63.56),
        DecayChannel("pi+ pi0",          20.67),
        DecayChannel("pi+ pi+ pi-",       5.583),
        DecayChannel("pi0 e+ nu_e",       5.07),
        DecayChannel("pi0 mu+ nu_mu",     3.352),
        DecayChannel("pi+ pi0 pi0",       1.760),
    ]))

    # K_S (PDG 2024)
    particles.append(Particle("K_S", [
        DecayChannel("pi+ pi-",          69.20),
        DecayChannel("pi0 pi0",          30.69),
        DecayChannel("pi+ pi- gamma",     0.0018),
    ]))

    # K_L (PDG 2024)
    particles.append(Particle("K_L", [
        DecayChannel("pi e nu (Ke3)",    40.55),
        DecayChannel("pi mu nu (Kmu3)",  27.04),
        DecayChannel("3 pi0",            19.52),
        DecayChannel("pi+ pi- pi0",      12.54),
        DecayChannel("pi+ pi-",           0.1967),
    ]))

    # D0 (PDG 2024) — exclusive modes only
    particles.append(Particle("D0", [
        DecayChannel("K- pi+",             3.950),
        DecayChannel("K- pi+ pi0",        14.4),
        DecayChannel("K- pi+ pi+ pi-",     8.22),
        DecayChannel("K-bar0 pi+ pi-",     5.50),
        DecayChannel("K- e+ nu_e",         3.538),
        DecayChannel("K- mu+ nu_mu",       3.41),
        DecayChannel("K-bar0 pi0",         2.31),
        DecayChannel("other",             58.67),
    ]))

    # D+ (PDG 2024) — with "other" to sum ~100%
    particles.append(Particle("D+", [
        DecayChannel("K- pi+ pi+",        9.38),
        DecayChannel("K-bar0 pi+",         3.09),
        DecayChannel("K-bar0 pi+ pi0",     6.99),
        DecayChannel("K- pi+ pi+ pi0",     6.10),
        DecayChannel("K-bar0 pi+ pi+ pi-", 3.12),
        DecayChannel("K-bar0 mu+ nu_mu",   8.83),
        DecayChannel("K-bar0 e+ nu_e",     8.73),
        DecayChannel("other",             53.72),
    ]))

    # Ds+ (PDG 2024) — exclusive modes with "other"
    particles.append(Particle("Ds+", [
        DecayChannel("phi pi+",            2.24),
        DecayChannel("K*0-bar K+",         2.95),
        DecayChannel("eta pi+",            1.68),
        DecayChannel("eta' pi+",           3.94),
        DecayChannel("K+ K- pi+",          5.37),
        DecayChannel("tau+ nu_tau",         5.32),
        DecayChannel("mu+ nu_mu",          0.531),
        DecayChannel("K+ K- pi+ pi0",      2.46),
        DecayChannel("other",             75.50),
    ]))

    # B+ (PDG 2024)
    particles.append(Particle("B+", [
        DecayChannel("D0-bar l+ nu",              2.27),
        DecayChannel("D*0-bar l+ nu",             5.58),
        DecayChannel("X_c l nu (inclusive)",      10.99),
        DecayChannel("J/psi K+",                   0.1009),
        DecayChannel("D0-bar pi+",                 0.484),
        DecayChannel("D-bar0 anything",           79.0),
        DecayChannel("charmless hadronic",          2.8),
    ]))

    # B0 (PDG 2024)
    particles.append(Particle("B0", [
        DecayChannel("D*- l+ nu",                  4.97),
        DecayChannel("D- l+ nu",                   2.19),
        DecayChannel("X_c l nu (inclusive)",       10.33),
        DecayChannel("J/psi K0",                    0.0870),
        DecayChannel("D- pi+",                      0.252),
        DecayChannel("D- anything",                79.0),
        DecayChannel("charmless hadronic",           2.5),
    ]))

    # Bs (PDG 2024)
    particles.append(Particle("Bs", [
        DecayChannel("Ds- anything",              93.0),
        DecayChannel("Ds- l+ nu",                  7.9),
        DecayChannel("J/psi phi",                   0.1046),
        DecayChannel("mu+ mu-",                     3.34e-7),
        DecayChannel("Ds- pi+",                     0.300),
        DecayChannel("J/psi eta",                    0.041),
    ]))

    # ===== BARYONS =====

    # neutron (PDG 2024)
    particles.append(Particle("neutron", [
        DecayChannel("p e- nu_e-bar",   100.0),
    ]))

    # Lambda (PDG 2024)
    particles.append(Particle("Lambda", [
        DecayChannel("p pi-",           63.9),
        DecayChannel("n pi0",           35.8),
        DecayChannel("p e- nu_e-bar",    0.0832),
        DecayChannel("p mu- nu_mu-bar",  0.0163),
    ]))

    # Sigma+ (PDG 2024)
    particles.append(Particle("Sigma+", [
        DecayChannel("p pi0",           51.57),
        DecayChannel("n pi+",           48.31),
        DecayChannel("p gamma",          1.23e-1),
    ]))

    # Sigma0 (PDG 2024) - EM decay
    particles.append(Particle("Sigma0", [
        DecayChannel("Lambda gamma",   100.0),
    ]))

    # Sigma- (PDG 2024)
    particles.append(Particle("Sigma-", [
        DecayChannel("n pi-",           99.848),
        DecayChannel("n e- nu_e-bar",    0.1017),
        DecayChannel("n mu- nu_mu-bar",  0.0450),
    ]))

    # Xi0 (PDG 2024)
    particles.append(Particle("Xi0", [
        DecayChannel("Lambda pi0",     99.525),
        DecayChannel("Lambda gamma",     0.117),
        DecayChannel("Sigma+ e- nu_e",   0.0254),
    ]))

    # Xi- (PDG 2024)
    particles.append(Particle("Xi-", [
        DecayChannel("Lambda pi-",     99.887),
        DecayChannel("Sigma- gamma",     0.0127),
        DecayChannel("Xi0 e- nu_e",      0.0562),
    ]))

    # Omega- (PDG 2024)
    particles.append(Particle("Omega-", [
        DecayChannel("Lambda K-",       67.8),
        DecayChannel("Xi0 pi-",         23.6),
        DecayChannel("Xi- pi0",          8.6),
    ]))

    # Lambda_c+ (PDG 2024) — with "other" to sum ~100%
    particles.append(Particle("Lambda_c+", [
        DecayChannel("p K- pi+",         6.28),
        DecayChannel("Lambda pi+",       1.30),
        DecayChannel("p K-bar0",         2.28),
        DecayChannel("Lambda pi+ pi0",   3.6),
        DecayChannel("Sigma0 pi+",       1.05),
        DecayChannel("Sigma+ pi+ pi-",   3.6),
        DecayChannel("Lambda pi+ pi+ pi-", 2.6),
        DecayChannel("p K- pi+ pi0",     4.53),
        DecayChannel("Lambda e+ nu_e",   3.6),
        DecayChannel("Lambda mu+ nu_mu", 3.5),
        DecayChannel("other",           67.66),
    ]))

    return particles


# ---------------------------------------------------------------------------
# Match datastructures
# ---------------------------------------------------------------------------

@dataclass
class FractionMatch:
    particle: str
    channel: str
    br: float               # measured BR (fraction)
    fraction_label: str     # e.g. "7/12"
    fraction_value: float
    rel_error: float        # |br - target| / target
    abs_error: float        # |br - target|


@dataclass
class JointResult:
    particle: str
    matches: List[FractionMatch]
    n_channels: int
    mc_p_value: float = 1.0
    sigma: float = 0.0


# ---------------------------------------------------------------------------
# Core analysis
# ---------------------------------------------------------------------------

def find_all_matches(
    particles: List[Particle],
    fractions: Dict[str, float],
    tol: float = TOLERANCE,
) -> List[FractionMatch]:
    """Find every branching ratio that matches an n=6 fraction within tol."""
    matches: List[FractionMatch] = []
    for p in particles:
        for ch in p.channels:
            for label, fval in fractions.items():
                if fval < 1e-12:
                    continue
                rel = abs(ch.br - fval) / fval
                if rel <= tol:
                    matches.append(FractionMatch(
                        particle=p.name,
                        channel=ch.label,
                        br=ch.br,
                        fraction_label=label,
                        fraction_value=fval,
                        rel_error=rel,
                        abs_error=abs(ch.br - fval),
                    ))
    matches.sort(key=lambda m: m.rel_error)
    return matches


def group_by_particle(
    matches: List[FractionMatch],
    particles: List[Particle],
) -> Dict[str, JointResult]:
    """Group matches by particle for joint analysis."""
    groups: Dict[str, JointResult] = {}
    for m in matches:
        if m.particle not in groups:
            p = next(pp for pp in particles if pp.name == m.particle)
            groups[m.particle] = JointResult(
                particle=m.particle,
                matches=[],
                n_channels=p.n_channels,
            )
        groups[m.particle].matches.append(m)
    return groups


# ---------------------------------------------------------------------------
# Monte Carlo
# ---------------------------------------------------------------------------

def mc_single_particle(
    n_channels: int,
    n_target_matches: int,
    fractions: Dict[str, float],
    n_trials: int = 100_000,
    tol: float = TOLERANCE,
    seed: int = 42,
) -> float:
    """
    MC p-value: probability that a random Dirichlet(1,...,1) draw with
    n_channels produces >= n_target_matches fraction hits.
    """
    rng = np.random.default_rng(seed)
    alpha = np.ones(n_channels)
    fvals = np.array(list(fractions.values()))
    count_ge = 0

    for _ in range(n_trials):
        sample = rng.dirichlet(alpha)
        hits = 0
        for br in sample:
            rel_errs = np.abs(br - fvals) / np.maximum(fvals, 1e-15)
            if np.any(rel_errs <= tol):
                hits += 1
        if hits >= n_target_matches:
            count_ge += 1

    return count_ge / n_trials


def mc_cross_particle(
    particles: List[Particle],
    fractions: Dict[str, float],
    observed_total: int,
    n_trials: int = 10_000,
    tol: float = TOLERANCE,
    seed: int = 123,
) -> Tuple[float, float, float]:
    """
    Cross-particle MC: randomise ALL particles independently via Dirichlet,
    count total fraction matches. Return (p_value, mean, std).
    """
    rng = np.random.default_rng(seed)
    fvals = np.array(list(fractions.values()))
    totals = np.zeros(n_trials)

    for t in range(n_trials):
        total = 0
        for p in particles:
            if p.n_channels < 2:
                continue
            sample = rng.dirichlet(np.ones(p.n_channels))
            for br in sample:
                rel_errs = np.abs(br - fvals) / np.maximum(fvals, 1e-15)
                if np.any(rel_errs <= tol):
                    total += 1
            # Also allow single-channel particles (BR=1)
        totals[t] = total

    mean = np.mean(totals)
    std = np.std(totals)
    p_value = np.mean(totals >= observed_total)
    return p_value, mean, std


def p_to_sigma(p: float) -> float:
    """Convert one-sided p-value to Gaussian sigma."""
    if p <= 0:
        return 10.0  # cap
    if p >= 1:
        return 0.0
    return stats.norm.isf(p)


# ---------------------------------------------------------------------------
# Main analysis
# ---------------------------------------------------------------------------

def run_analysis(verbose: bool = True) -> None:
    """Run the full systematic branching ratio scan."""

    particles = build_particle_database()
    fractions = TARGET_FRACTIONS

    if verbose:
        print("=" * 80)
        print("SYSTEMATIC BRANCHING RATIO SCAN: n=6 FRACTIONS")
        print("=" * 80)
        print()
        print(f"Tolerance: {TOLERANCE*100:.0f}% relative error")
        print(f"Target fractions ({len(fractions)}):")
        for lbl, val in fractions.items():
            print(f"  {lbl:>5s} = {val:.6f}")
        print()

    # ----- Database summary -----
    total_channels = sum(p.n_channels for p in particles)
    if verbose:
        print(f"Particle database: {len(particles)} particles, "
              f"{total_channels} total decay channels")
        print("-" * 80)
        for p in particles:
            brs = [ch.br for ch in p.channels]
            br_sum = sum(brs)
            print(f"  {p.name:20s}  {p.n_channels:2d} channels  "
                  f"sum={br_sum*100:7.2f}%")
        print()

    # ===================================================================
    # STEP 1: Find ALL individual matches
    # ===================================================================
    matches = find_all_matches(particles, fractions, tol=TOLERANCE)

    if verbose:
        print("=" * 80)
        print(f"ALL INDIVIDUAL MATCHES ({len(matches)} found)")
        print("=" * 80)
        print()
        print(f"  {'#':>3s}  {'Particle':20s}  {'Channel':30s}  "
              f"{'BR':>10s}  {'Frac':>5s}  {'Target':>10s}  {'Err%':>6s}")
        print("  " + "-" * 90)
        for i, m in enumerate(matches, 1):
            print(f"  {i:3d}  {m.particle:20s}  {m.channel:30s}  "
                  f"{m.br:10.6f}  {m.fraction_label:>5s}  "
                  f"{m.fraction_value:10.6f}  {m.rel_error*100:5.2f}%")

    # ===================================================================
    # STEP 2: Group by particle, find joints
    # ===================================================================
    groups = group_by_particle(matches, particles)

    if verbose:
        print()
        print("=" * 80)
        print("MATCHES GROUPED BY PARTICLE")
        print("=" * 80)
        for pname, jr in sorted(groups.items(), key=lambda x: -len(x[1].matches)):
            print(f"\n  {pname} ({jr.n_channels} channels, "
                  f"{len(jr.matches)} matches):")
            for m in jr.matches:
                print(f"    {m.channel:30s}  BR={m.br:.6f}  "
                      f"~ {m.fraction_label} ({m.rel_error*100:.2f}%)")

    # ===================================================================
    # STEP 3: MC per particle (for those with 2+ matches)
    # ===================================================================
    multi_match = {k: v for k, v in groups.items() if len(v.matches) >= 2}

    if verbose:
        print()
        print("=" * 80)
        print(f"PER-PARTICLE MC (100k Dirichlet trials) — "
              f"{len(multi_match)} particles with 2+ matches")
        print("=" * 80)

    for pname, jr in sorted(multi_match.items(),
                             key=lambda x: -len(x[1].matches)):
        n_hits = len(jr.matches)
        p_val = mc_single_particle(
            jr.n_channels, n_hits, fractions,
            n_trials=100_000, tol=TOLERANCE,
        )
        jr.mc_p_value = p_val
        jr.sigma = p_to_sigma(p_val)

        if verbose:
            print(f"\n  {pname}:")
            print(f"    Channels: {jr.n_channels},  Matches: {n_hits}")
            for m in jr.matches:
                print(f"      {m.channel:30s} ~ {m.fraction_label}  "
                      f"(err {m.rel_error*100:.2f}%)")
            print(f"    MC p-value: {p_val:.6f}  "
                  f"({jr.sigma:.2f}σ)")

    # Also compute for single-match particles
    for pname, jr in groups.items():
        if len(jr.matches) == 1:
            p_val = mc_single_particle(
                jr.n_channels, 1, fractions,
                n_trials=100_000, tol=TOLERANCE,
            )
            jr.mc_p_value = p_val
            jr.sigma = p_to_sigma(p_val)

    # ===================================================================
    # STEP 4: Cross-particle global test
    # ===================================================================
    observed_total = len(matches)

    if verbose:
        print()
        print("=" * 80)
        print("CROSS-PARTICLE GLOBAL TEST (10k trials)")
        print("=" * 80)
        print(f"\n  Observed total matches: {observed_total}")
        print("  Running MC... ", end="", flush=True)

    p_global, mean_mc, std_mc = mc_cross_particle(
        particles, fractions, observed_total,
        n_trials=10_000, tol=TOLERANCE,
    )
    sigma_global = p_to_sigma(p_global)

    if verbose:
        print("done.")
        print(f"  MC mean matches:  {mean_mc:.2f} +/- {std_mc:.2f}")
        print(f"  Observed:         {observed_total}")
        print(f"  z-score:          {(observed_total - mean_mc)/std_mc:.2f}")
        print(f"  Global p-value:   {p_global:.6f}")
        print(f"  Global sigma:     {sigma_global:.2f}σ")

    # ===================================================================
    # STEP 5: Top 10 most significant findings
    # ===================================================================
    all_results = sorted(groups.values(), key=lambda jr: -jr.sigma)

    if verbose:
        print()
        print("=" * 80)
        print("TOP 10 MOST SIGNIFICANT FINDINGS")
        print("=" * 80)
        for rank, jr in enumerate(all_results[:10], 1):
            print(f"\n  #{rank}  {jr.particle}  "
                  f"({len(jr.matches)} matches, {jr.sigma:.2f}σ, "
                  f"p={jr.mc_p_value:.6f})")
            for m in jr.matches:
                print(f"       {m.channel:30s}  BR={m.br:.6f}  "
                      f"~ {m.fraction_label} (err {m.rel_error*100:.2f}%)")

    # ===================================================================
    # STEP 6: Summary & Key Question
    # ===================================================================
    if verbose:
        print()
        print("=" * 80)
        print("FINAL SUMMARY")
        print("=" * 80)
        print()
        print(f"  Database:        {len(particles)} particles, "
              f"{total_channels} channels")
        print(f"  Fractions:       {len(fractions)} n=6 fractions tested")
        print(f"  Tolerance:       {TOLERANCE*100:.0f}% relative")
        print()
        print(f"  Individual matches found:  {len(matches)}")
        print(f"  Particles with 1+ match:   {len(groups)}")
        print(f"  Particles with 2+ matches: {len(multi_match)}")
        print()
        print(f"  MC expected (random):      {mean_mc:.1f} +/- {std_mc:.1f}")
        print(f"  Observed:                  {observed_total}")
        print(f"  Global p-value:            {p_global:.6f}")
        print(f"  Global significance:       {sigma_global:.2f}σ")
        print()

        # Highlight the Higgs specifically
        if "Higgs boson" in groups:
            hg = groups["Higgs boson"]
            print("  --- Higgs boson (original finding) ---")
            for m in hg.matches:
                print(f"    {m.channel:30s}  BR={m.br:.6f}  "
                      f"~ {m.fraction_label} (err {m.rel_error*100:.2f}%)")
            print(f"    Joint p-value: {hg.mc_p_value:.6f} "
                  f"({hg.sigma:.2f}σ)")
            print()

        # Answer the key question
        print("  KEY QUESTION: Is the total count of n=6 fraction matches")
        print("  across all particles higher than expected by chance?")
        print()
        if p_global < 0.01:
            print(f"  ANSWER: YES — {observed_total} matches observed vs "
                  f"{mean_mc:.1f} expected (p={p_global:.6f}, "
                  f"{sigma_global:.2f}σ)")
        elif p_global < 0.05:
            print(f"  ANSWER: MARGINALLY — {observed_total} matches vs "
                  f"{mean_mc:.1f} expected (p={p_global:.4f}, "
                  f"{sigma_global:.2f}σ)")
        else:
            print(f"  ANSWER: NO — {observed_total} matches vs "
                  f"{mean_mc:.1f} expected (p={p_global:.4f})")
            print("  The total count is consistent with random chance.")
            print("  Individual particles may still show interesting patterns.")
        print()

    return


if __name__ == "__main__":
    run_analysis(verbose=True)
