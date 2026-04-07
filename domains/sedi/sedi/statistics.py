"""Statistical validation engine — Monte Carlo, Bonferroni, Look-Elsewhere Effect.

Provides CERN-grade statistical validation for pattern matches.

Three null models:
  1. KDE — fit kernel density to log10(observed masses), sample from it
  2. Bootstrap — resample with replacement from observed masses
  3. Permutation — shuffle mass assignments among particles

KDE is the primary model: preserves the observed mass *distribution* (clustering
at QCD scale, electroweak scale, etc.) while generating genuinely new mass sets.
This answers: "given the mass distribution nature provides, are TECS-L ratio
matches more common than chance?"
"""
import numpy as np
from collections import defaultdict


def _count_ratio_matches(masses, targets, tolerance):
    """Count how many pairwise ratios match each target."""
    n = len(masses)
    counts = defaultdict(int)
    for i in range(n):
        if masses[i] <= 0:
            continue
        for j in range(i + 1, n):
            if masses[j] <= 0:
                continue
            ratio = masses[j] / masses[i]  # masses assumed sorted
            for tname, tval in targets.items():
                if tval <= 0:
                    continue
                if abs(ratio - tval) / tval < tolerance:
                    counts[tname] += 1
                if tval < 1:
                    inv = masses[i] / masses[j]
                    if abs(inv - tval) / tval < tolerance:
                        counts[tname] += 1
    return counts


def _fit_kde(observed_masses, bandwidth=None):
    """Fit Gaussian KDE to log10(masses)."""
    from scipy.stats import gaussian_kde
    log_m = np.log10(np.array(observed_masses)[np.array(observed_masses) > 0])
    if bandwidth is None:
        kde = gaussian_kde(log_m)
    else:
        kde = gaussian_kde(log_m, bw_method=bandwidth)
    return kde, float(log_m.min()), float(log_m.max())


def monte_carlo_kde(observed_masses, targets, tolerance=0.03,
                    n_trials=10_000, seed=42):
    """Monte Carlo using KDE null model.

    Fits KDE to log10(observed masses), then samples n_trials synthetic
    mass sets of the same size and counts ratio matches.
    """
    rng = np.random.default_rng(seed)
    n = len(observed_masses)
    kde, log_min, log_max = _fit_kde(observed_masses)

    # Pre-compute for speed
    target_items = [(k, v) for k, v in targets.items() if v > 0]

    all_counts = {tname: [] for tname, _ in target_items}

    for trial in range(n_trials):
        # Sample from KDE
        log_samples = kde.resample(n, seed=rng.integers(0, 2**31)).flatten()
        masses = np.sort(10 ** log_samples)

        counts = _count_ratio_matches(masses, dict(target_items), tolerance)
        for tname, _ in target_items:
            all_counts[tname].append(counts.get(tname, 0))

    results = {}
    for tname, _ in target_items:
        arr = np.array(all_counts[tname])
        results[tname] = {
            'mean': float(np.mean(arr)),
            'std': float(np.std(arr)),
            'median': float(np.median(arr)),
            'p95': float(np.percentile(arr, 95)),
            'p99': float(np.percentile(arr, 99)),
            'distribution': arr,
        }

    return results


def monte_carlo_bootstrap(observed_masses, targets, tolerance=0.03,
                          n_trials=10_000, seed=42):
    """Monte Carlo using bootstrap null model.

    Resample with replacement from the observed mass list.
    """
    rng = np.random.default_rng(seed)
    masses_arr = np.array(sorted(observed_masses))
    n = len(masses_arr)

    target_items = [(k, v) for k, v in targets.items() if v > 0]
    all_counts = {tname: [] for tname, _ in target_items}

    for _ in range(n_trials):
        sample = np.sort(rng.choice(masses_arr, size=n, replace=True))
        counts = _count_ratio_matches(sample, dict(target_items), tolerance)
        for tname, _ in target_items:
            all_counts[tname].append(counts.get(tname, 0))

    results = {}
    for tname, _ in target_items:
        arr = np.array(all_counts[tname])
        results[tname] = {
            'mean': float(np.mean(arr)),
            'std': float(np.std(arr)),
            'median': float(np.median(arr)),
            'p95': float(np.percentile(arr, 95)),
            'p99': float(np.percentile(arr, 99)),
            'distribution': arr,
        }

    return results


def compute_p_value(observed, mc_distribution):
    """Compute empirical p-value: fraction of MC trials >= observed."""
    arr = np.asarray(mc_distribution)
    if len(arr) == 0:
        return 1.0
    # Use (count + 1) / (N + 1) to avoid p=0
    return float((np.sum(arr >= observed) + 1) / (len(arr) + 1))


def bonferroni_correction(p_values, n_tests=None):
    """Bonferroni correction for multiple comparisons."""
    if n_tests is None:
        n_tests = len(p_values)
    return [min(p * n_tests, 1.0) for p in p_values]


def benjamini_hochberg(p_values):
    """Benjamini-Hochberg FDR correction."""
    n = len(p_values)
    indexed = sorted(enumerate(p_values), key=lambda x: x[1])
    adjusted = [0.0] * n

    prev = 1.0
    for rank_idx in range(n - 1, -1, -1):
        orig_idx, p = indexed[rank_idx]
        rank = rank_idx + 1
        adj = min(p * n / rank, prev)
        adjusted[orig_idx] = min(adj, 1.0)
        prev = adj

    return adjusted


def look_elsewhere_effect(local_p, n_independent_regions):
    """Look-Elsewhere Effect (LEE) correction.

    CERN uses this for Higgs discovery.
    global_p = 1 - (1 - local_p)^N
    """
    if local_p >= 1:
        return 1.0
    global_p = 1 - (1 - local_p) ** n_independent_regions
    return min(global_p, 1.0)


def p_to_sigma(p):
    """Convert p-value to sigma (standard deviations)."""
    from scipy.stats import norm
    if p <= 0:
        return float('inf')
    if p >= 1:
        return 0.0
    return float(norm.isf(p))


def sigma_to_p(sigma_val):
    """Convert sigma to p-value."""
    from scipy.stats import norm
    return float(norm.sf(sigma_val))


def format_significance(p_value, sigma_val=None):
    """Format significance level."""
    if sigma_val is not None and sigma_val > 0:
        sig_str = f"{sigma_val:.1f}σ"
    else:
        sig_str = ""

    if p_value < 0.0001:
        return f"{p_value:.2e} {sig_str} (****)"
    elif p_value < 0.001:
        return f"{p_value:.4f} {sig_str} (***)"
    elif p_value < 0.01:
        return f"{p_value:.4f} {sig_str} (**)"
    elif p_value < 0.05:
        return f"{p_value:.4f} {sig_str} (*)"
    else:
        return f"{p_value:.4f} {sig_str} (n.s.)"


class StatisticalValidator:
    """Run full statistical validation on a set of findings."""

    def __init__(self, n_mc_trials=10_000, seed=42):
        self.n_mc_trials = n_mc_trials
        self.seed = seed

    def validate_ratio_matches(self, observed_hits, n_particles, targets,
                               observed_masses=None, tolerance=0.03):
        """Full validation pipeline for mass ratio matches.

        Uses KDE null model (primary) + bootstrap (cross-check).
        observed_masses: actual particle mass values for KDE fitting.
        """
        if observed_masses is None:
            raise ValueError("observed_masses required for KDE null model")

        # Count observed hits per target
        obs_counts = defaultdict(int)
        for hit in observed_hits:
            obs_counts[hit['target_name']] += 1

        # Filter targets to only those that had hits
        active_targets = {k: v for k, v in targets.items()
                         if k in obs_counts and v > 0}

        # Run KDE Monte Carlo (primary)
        print(f"    KDE null model ({self.n_mc_trials:,} trials)...")
        kde_results = monte_carlo_kde(
            observed_masses=observed_masses,
            targets=active_targets,
            tolerance=tolerance,
            n_trials=self.n_mc_trials,
            seed=self.seed,
        )

        # Run Bootstrap Monte Carlo (cross-check)
        print(f"    Bootstrap null model ({self.n_mc_trials:,} trials)...")
        boot_results = monte_carlo_bootstrap(
            observed_masses=observed_masses,
            targets=active_targets,
            tolerance=tolerance,
            n_trials=self.n_mc_trials,
            seed=self.seed + 1,
        )

        # Compute p-values from KDE (primary)
        findings = []
        raw_p_values = []

        for tname, obs_count in obs_counts.items():
            if tname not in kde_results:
                continue
            kde = kde_results[tname]
            boot = boot_results.get(tname, {})

            kde_p = compute_p_value(obs_count, kde['distribution'])
            boot_p = compute_p_value(obs_count, boot.get('distribution', []))

            # Use the MORE CONSERVATIVE p-value (larger = less significant)
            raw_p = max(kde_p, boot_p)
            raw_p_values.append(raw_p)

            findings.append({
                'target': tname,
                'target_val': active_targets.get(tname, 0),
                'observed': obs_count,
                'kde_expected': kde['mean'],
                'kde_std': kde['std'],
                'kde_p': kde_p,
                'boot_expected': boot.get('mean', 0),
                'boot_p': boot_p,
                'raw_p': raw_p,
                'excess_over_kde': obs_count - kde['mean'],
            })

        # Apply corrections
        n_tests = len(findings)
        if n_tests == 0:
            return []

        bonf_p = bonferroni_correction(raw_p_values, n_tests)
        bh_p = benjamini_hochberg(raw_p_values)
        n_regions = len(targets)

        for i, f in enumerate(findings):
            f['bonferroni_p'] = bonf_p[i]
            f['bh_p'] = bh_p[i]
            f['lee_p'] = look_elsewhere_effect(f['raw_p'], n_regions)
            f['n_tests'] = n_tests

            # Final verdict: most conservative
            final_p = max(f['bonferroni_p'], f['lee_p'])
            f['final_p'] = final_p
            try:
                f['sigma'] = p_to_sigma(final_p)
            except (ImportError, ValueError):
                f['sigma'] = 0.0
            f['significance'] = format_significance(final_p, f['sigma'])

        return sorted(findings, key=lambda f: f['final_p'])
