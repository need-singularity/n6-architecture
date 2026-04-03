//! Genetic programming engine for lens evolution.
/// Genetic Programming for Lens Evolution — use evolutionary algorithms
/// to combine and evolve lens parameters for optimal discovery pipelines.
///
/// n=6 constants used throughout:
///   population_size = σ² = 144
///   mutation_rate   = 1/σ = 1/12 ≈ 0.0833
///   crossover split = 1/2 + 1/3 + 1/6 = 1 (Egyptian fraction)

/// A single gene representing one lens configuration.
#[derive(Debug, Clone)]
pub struct Gene {
    /// Lens identifier or name.
    pub lens_id: String,
    /// Weight applied to this lens in the pipeline (0.0–1.0).
    pub weight: f64,
    /// Activation threshold — pattern must exceed this confidence to pass.
    pub threshold: f64,
}

impl Gene {
    pub fn new(lens_id: impl Into<String>, weight: f64, threshold: f64) -> Self {
        Self {
            lens_id: lens_id.into(),
            weight: weight.clamp(0.0, 1.0),
            threshold: threshold.clamp(0.0, 1.0),
        }
    }
}

/// A chromosome = ordered pipeline of genes (lenses).
#[derive(Debug, Clone)]
pub struct Chromosome {
    pub genes: Vec<Gene>,
}

impl Chromosome {
    pub fn new(genes: Vec<Gene>) -> Self {
        Self { genes }
    }

    /// Total weight across all genes (used for normalization).
    pub fn total_weight(&self) -> f64 {
        self.genes.iter().map(|g| g.weight).sum()
    }
}

/// Deterministic LCG pseudo-random number generator (no external crates).
/// Parameters chosen for full-period: a=6364136223846793005, c=1, m=2^64.
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        // Knuth LCG
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    /// Uniform f64 in [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Uniform usize in [0, n).
    fn next_usize(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

/// Population of chromosomes with evolutionary operators.
pub struct Population {
    pub chromosomes: Vec<Chromosome>,
    rng: Lcg,
    /// Mutation rate: 1/σ = 1/12 ≈ 0.0833 (n=6 constant)
    pub mutation_rate: f64,
}

/// Default population size: σ² = 144 (n=6 constant).
pub const DEFAULT_POP_SIZE: usize = 144; // σ² = 12² = 144

/// Mutation rate: 1/σ = 1/12 (n=6 constant).
pub const MUTATION_RATE: f64 = 1.0 / 12.0; // 1/σ

impl Population {
    /// Create a new population from existing chromosomes.
    pub fn new(chromosomes: Vec<Chromosome>, seed: u64) -> Self {
        Self {
            chromosomes,
            rng: Lcg::new(seed),
            mutation_rate: MUTATION_RATE,
        }
    }

    /// Generate a random initial population of `pop_size` chromosomes,
    /// each with `gene_count` genes using the given lens IDs.
    pub fn random(lens_ids: &[String], gene_count: usize, pop_size: usize, seed: u64) -> Self {
        let mut rng = Lcg::new(seed);
        let mut chromosomes = Vec::with_capacity(pop_size);
        for _ in 0..pop_size {
            let mut genes = Vec::with_capacity(gene_count);
            for i in 0..gene_count {
                let lid = &lens_ids[i % lens_ids.len()];
                genes.push(Gene::new(
                    lid.clone(),
                    rng.next_f64(),
                    rng.next_f64(),
                ));
            }
            chromosomes.push(Chromosome::new(genes));
        }
        Self {
            chromosomes,
            rng,
            mutation_rate: MUTATION_RATE,
        }
    }

    /// Tournament selection: pick `k` random individuals, return the fittest.
    fn tournament_select(&mut self, fitnesses: &[f64], k: usize) -> usize {
        let n = self.chromosomes.len();
        let mut best_idx = self.rng.next_usize(n);
        let mut best_fit = fitnesses[best_idx];
        for _ in 1..k {
            let idx = self.rng.next_usize(n);
            if fitnesses[idx] > best_fit {
                best_idx = idx;
                best_fit = fitnesses[idx];
            }
        }
        best_idx
    }

    /// Egyptian fraction crossover: split parent genes at ratios 1/2, 1/3, 1/6.
    /// First half from parent A, next third from parent B, last sixth from A again.
    /// 1/2 + 1/3 + 1/6 = 1 (perfect number property of n=6).
    fn crossover(&mut self, a: &Chromosome, b: &Chromosome) -> Chromosome {
        let len = a.genes.len().min(b.genes.len());
        if len == 0 {
            return a.clone();
        }
        let split1 = len / 2;               // 1/2
        let split2 = split1 + len / 3;      // + 1/3
        // remaining ≈ 1/6                   // + 1/6

        let mut genes = Vec::with_capacity(len);
        for i in 0..len {
            if i < split1 {
                genes.push(a.genes[i].clone());        // 1/2 from A
            } else if i < split2 {
                genes.push(b.genes[i].clone());        // 1/3 from B
            } else {
                genes.push(a.genes[i].clone());        // 1/6 from A
            }
        }
        Chromosome::new(genes)
    }

    /// Mutate a chromosome: each gene has `mutation_rate` chance of perturbation.
    fn mutate(&mut self, chromosome: &mut Chromosome) {
        for gene in &mut chromosome.genes {
            if self.rng.next_f64() < self.mutation_rate {
                // Perturb weight and threshold by small delta
                let delta_w = (self.rng.next_f64() - 0.5) * 0.2;
                let delta_t = (self.rng.next_f64() - 0.5) * 0.2;
                gene.weight = (gene.weight + delta_w).clamp(0.0, 1.0);
                gene.threshold = (gene.threshold + delta_t).clamp(0.0, 1.0);
            }
        }
    }

    /// Run one generation of evolution.
    /// Returns the best fitness in this generation.
    pub fn evolve_one(&mut self, fitness_fn: &dyn Fn(&Chromosome) -> f64) -> f64 {
        let fitnesses: Vec<f64> = self.chromosomes.iter().map(|c| fitness_fn(c)).collect();
        let pop_size = self.chromosomes.len();

        // Elitism: keep the best individual
        let best_idx = fitnesses
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);
        let elite = self.chromosomes[best_idx].clone();
        let best_fitness = fitnesses[best_idx];

        let mut next_gen = Vec::with_capacity(pop_size);
        next_gen.push(elite);

        // Tournament size = n/φ = 3 (n=6 constant)
        let tournament_k = 3;

        while next_gen.len() < pop_size {
            let parent_a_idx = self.tournament_select(&fitnesses, tournament_k);
            let parent_b_idx = self.tournament_select(&fitnesses, tournament_k);
            let pa = self.chromosomes[parent_a_idx].clone();
            let pb = self.chromosomes[parent_b_idx].clone();
            let mut child = self.crossover(&pa, &pb);
            self.mutate(&mut child);
            next_gen.push(child);
        }

        self.chromosomes = next_gen;
        best_fitness
    }
}

/// Evaluate a chromosome against known patterns (ground truth).
/// Each gene acts as a filter: if a pattern's simulated score exceeds the
/// gene's threshold, the gene's weight contributes to the total score.
///
/// `data` = simulated pattern scores (one per known pattern).
/// `known_patterns` = ground truth labels (true = real pattern).
pub fn fitness(chromosome: &Chromosome, data: &[f64], known_patterns: &[bool]) -> f64 {
    if data.is_empty() || data.len() != known_patterns.len() {
        return 0.0;
    }

    let total_weight = chromosome.total_weight();
    if total_weight < 1e-12 {
        return 0.0;
    }

    let mut score = 0.0;
    for (i, (&val, &is_real)) in data.iter().zip(known_patterns.iter()).enumerate() {
        // Each gene votes on this data point
        let mut detection_score = 0.0;
        for gene in &chromosome.genes {
            if val > gene.threshold {
                detection_score += gene.weight;
            }
        }
        detection_score /= total_weight;

        // Reward true positives, penalize false positives
        if is_real {
            score += detection_score;
        } else {
            score -= detection_score * 0.5; // lighter penalty for false positives
        }
        let _ = i; // suppress unused warning
    }

    // Normalize to [0, 1] range approximately
    let n = data.len() as f64;
    (score / n + 0.5).clamp(0.0, 1.0)
}

/// Run evolution for `generations` iterations and return the best chromosome.
///
/// `data` and `known_patterns` are the ground truth for fitness evaluation.
pub fn evolve(
    population: &mut Population,
    generations: usize,
    data: &[f64],
    known_patterns: &[bool],
) -> Chromosome {
    let fitness_fn = |c: &Chromosome| -> f64 { fitness(c, data, known_patterns) };

    for _ in 0..generations {
        population.evolve_one(&fitness_fn);
    }

    // Return the fittest individual
    population
        .chromosomes
        .iter()
        .max_by(|a, b| {
            fitness_fn(a)
                .partial_cmp(&fitness_fn(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
        .unwrap_or_else(|| Chromosome::new(vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_constants() {
        // σ² = 144, 1/σ = 1/12
        assert_eq!(DEFAULT_POP_SIZE, 144);
        assert!((MUTATION_RATE - 1.0 / 12.0).abs() < 1e-12);
    }

    #[test]
    fn test_egyptian_crossover() {
        let lens_ids: Vec<String> = (0..6).map(|i| format!("lens_{}", i)).collect(); // n=6 lenses
        let mut pop = Population::random(&lens_ids, 6, 2, 42);

        let parent_a = pop.chromosomes[0].clone();
        let parent_b = pop.chromosomes[1].clone();
        let child = pop.crossover(&parent_a, &parent_b);

        // Child should have same number of genes
        assert_eq!(child.genes.len(), 6); // n=6

        // First half (3 genes) from parent A
        for i in 0..3 {
            assert_eq!(child.genes[i].lens_id, parent_a.genes[i].lens_id);
            assert!((child.genes[i].weight - parent_a.genes[i].weight).abs() < 1e-12);
        }
        // Next third (2 genes) from parent B
        for i in 3..5 {
            assert_eq!(child.genes[i].lens_id, parent_b.genes[i].lens_id);
            assert!((child.genes[i].weight - parent_b.genes[i].weight).abs() < 1e-12);
        }
        // Last sixth (1 gene) from parent A
        assert_eq!(child.genes[5].lens_id, parent_a.genes[5].lens_id);
    }

    #[test]
    fn test_fitness_improves_over_generations() {
        let lens_ids: Vec<String> = (0..6).map(|i| format!("lens_{}", i)).collect();
        let mut pop = Population::random(&lens_ids, 6, 24, 12345); // J₂=24 pop for speed

        // Ground truth: first 6 are real patterns, rest are noise
        let data: Vec<f64> = vec![0.9, 0.85, 0.8, 0.75, 0.7, 0.65, 0.1, 0.15, 0.2, 0.05, 0.12, 0.08];
        let known: Vec<bool> = vec![true, true, true, true, true, true, false, false, false, false, false, false];

        // Measure initial best fitness
        let initial_best = pop
            .chromosomes
            .iter()
            .map(|c| fitness(c, &data, &known))
            .fold(0.0_f64, f64::max);

        // Evolve for σ=12 generations
        let best = evolve(&mut pop, 12, &data, &known);
        let final_fitness = fitness(&best, &data, &known);

        // Final should be at least as good as initial (elitism guarantees no regression)
        assert!(final_fitness >= initial_best - 1e-9);
    }

    #[test]
    fn test_mutation_stays_in_bounds() {
        let mut pop = Population::new(
            vec![Chromosome::new(vec![
                Gene::new("test", 0.01, 0.99),
                Gene::new("test2", 0.99, 0.01),
            ])],
            777,
        );
        // Force many mutations
        pop.mutation_rate = 1.0;
        for _ in 0..100 {
            let mut c = pop.chromosomes[0].clone();
            pop.mutate(&mut c);
            for gene in &c.genes {
                assert!(gene.weight >= 0.0 && gene.weight <= 1.0);
                assert!(gene.threshold >= 0.0 && gene.threshold <= 1.0);
            }
        }
    }

    #[test]
    fn test_deterministic_rng() {
        // Same seed produces same sequence
        let lens_ids: Vec<String> = vec!["a".into(), "b".into()];
        let pop1 = Population::random(&lens_ids, 2, 4, 42);
        let pop2 = Population::random(&lens_ids, 2, 4, 42);
        for (c1, c2) in pop1.chromosomes.iter().zip(pop2.chromosomes.iter()) {
            for (g1, g2) in c1.genes.iter().zip(c2.genes.iter()) {
                assert!((g1.weight - g2.weight).abs() < 1e-12);
                assert!((g1.threshold - g2.threshold).abs() < 1e-12);
            }
        }
    }
}
