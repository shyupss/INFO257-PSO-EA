use rand::RngExt;

use super::individual::Individual;
use super::params::{
	BLX_ALPHA, CROSSOVER_PROBABILITY, DOMAIN_MAX, DOMAIN_MIN,
	FINAL_MUTATION_SIGMA, INITIAL_MUTATION_SIGMA, MAX_GENERATIONS, MUTATION_PROBABILITY,
	POPULATION_SIZE, TOURNAMENT_SIZE,
};

// ============================================================================
// GAUSSIAN SAMPLING (Box-Muller Transform)
// ============================================================================

/// Generate a single sample from N(0, sigma) using the Box-Muller transform.
/// This avoids depending on `rand_distr` which has version conflicts with `rand 0.10`.
fn gaussian_sample(sigma: f64, rng: &mut rand::rngs::ThreadRng) -> f64 {
	let u1: f64 = rng.random_range(f64::MIN_POSITIVE..=1.0);
	let u2: f64 = rng.random_range(0.0..=std::f64::consts::TAU);
	sigma * (-2.0 * u1.ln()).sqrt() * u2.cos()
}

// ============================================================================
// POPULATION STRUCT
// ============================================================================

/// The evolutionary population, holding all individuals and tracking the best solution.
pub struct Population {
	pub individuals: Vec<Individual>,
	pub best_individual: Individual,
	pub current_generation: usize,
	pub best_generation: usize,
	pub max_generations: usize,
}

/// Result of a full batch run.
pub struct RunResult {
	pub best_position: [f64; 2],
	pub best_value: f64,
	pub generations_to_best: usize,
}

impl Population {
	/// Initialize a random population and identify the initial best individual.
	pub fn new() -> Self {
		let mut rng = rand::rng();
		let individuals: Vec<Individual> = (0..POPULATION_SIZE)
			.map(|_| Individual::new_random(&mut rng))
			.collect();

		let best_individual = individuals
			.iter()
			.min_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap_or(std::cmp::Ordering::Equal))
			.map_or_else(
				|| Individual {
					genes: [0.0, 0.0],
					fitness: f64::INFINITY,
				},
				Clone::clone,
			);

		Population {
			individuals,
			best_individual,
			current_generation: 0,
			best_generation: 0,
			max_generations: MAX_GENERATIONS,
		}
	}

	// ========================================================================
	// SELECTION: Tournament Selection
	// ========================================================================

	/// Select one individual via tournament selection.
	/// Picks `TOURNAMENT_SIZE` random individuals and returns a clone of the fittest.
	fn tournament_select(
		individuals: &[Individual],
		rng: &mut rand::rngs::ThreadRng,
	) -> Individual {
		let mut best_index = rng.random_range(0..individuals.len());

		for _ in 1..TOURNAMENT_SIZE {
			let challenger_index = rng.random_range(0..individuals.len());
			if individuals[challenger_index].fitness < individuals[best_index].fitness {
				best_index = challenger_index;
			}
		}

		individuals[best_index].clone()
	}

	// ========================================================================
	// CROSSOVER: BLX-α (Blend Crossover)
	// ========================================================================

	/// Perform BLX-α crossover between two parents, producing two offspring.
	/// For each gene, the offspring values are sampled uniformly from
	/// [min - α·range, max + α·range] where range = |parent1 - parent2|.
	#[allow(clippy::similar_names)]
	fn blx_alpha_crossover(
		parent_a: &Individual,
		parent_b: &Individual,
		rng: &mut rand::rngs::ThreadRng,
	) -> (Individual, Individual) {
		let mut child_a_genes = [0.0_f64; 2];
		let mut child_b_genes = [0.0_f64; 2];

		for i in 0..2 {
			let min_gene = parent_a.genes[i].min(parent_b.genes[i]);
			let max_gene = parent_a.genes[i].max(parent_b.genes[i]);
			let range = max_gene - min_gene;

			let lower_bound = (min_gene - BLX_ALPHA * range).max(DOMAIN_MIN);
			let upper_bound = (max_gene + BLX_ALPHA * range).min(DOMAIN_MAX);

			child_a_genes[i] = rng.random_range(lower_bound..=upper_bound);
			child_b_genes[i] = rng.random_range(lower_bound..=upper_bound);
		}

		let mut child_a = Individual {
			genes: child_a_genes,
			fitness: 0.0,
		};
		let mut child_b = Individual {
			genes: child_b_genes,
			fitness: 0.0,
		};
		child_a.evaluate();
		child_b.evaluate();

		(child_a, child_b)
	}

	// ========================================================================
	// MUTATION: Gaussian Perturbation with Adaptive σ
	// ========================================================================

	/// Mutate an individual's genes using Gaussian perturbation.
	/// σ decays linearly from `INITIAL_MUTATION_SIGMA` to `FINAL_MUTATION_SIGMA`
	/// across generations, implementing the explore→exploit strategy.
	fn gaussian_mutate(
		individual: &mut Individual,
		current_generation: usize,
		max_generations: usize,
		rng: &mut rand::rngs::ThreadRng,
	) {
		let progress = current_generation as f64 / max_generations as f64;
		let sigma =
			INITIAL_MUTATION_SIGMA + progress * (FINAL_MUTATION_SIGMA - INITIAL_MUTATION_SIGMA);

		for gene in &mut individual.genes {
			let random_value: f64 = rng.random_range(0.0..=1.0);
			if random_value < MUTATION_PROBABILITY {
				*gene += gaussian_sample(sigma, rng);
			}
		}
		individual.clamp_to_domain();
		individual.evaluate();
	}

	// ========================================================================
	// EVOLUTIONARY STEP
	// ========================================================================

	/// Run a single generation of the evolutionary algorithm.
	/// Uses Steady-State reinsertion for smoother visual tracking.
	/// Returns `true` if there are still generations remaining.
	pub fn step(&mut self) -> bool {
		if self.current_generation >= self.max_generations {
			return false;
		}
		self.current_generation += 1;
		let mut rng = rand::rng();

		// --- Selection ---
		let parent_a = Self::tournament_select(&self.individuals, &mut rng);
		let parent_b = Self::tournament_select(&self.individuals, &mut rng);

		// --- Crossover ---
		let crossover_roll: f64 = rng.random_range(0.0..=1.0);
		let (mut child_a, mut child_b) = if crossover_roll < CROSSOVER_PROBABILITY {
			Self::blx_alpha_crossover(&parent_a, &parent_b, &mut rng)
		} else {
			(parent_a, parent_b)
		};

		// --- Mutation ---
		Self::gaussian_mutate(
			&mut child_a,
			self.current_generation,
			self.max_generations,
			&mut rng,
		);
		Self::gaussian_mutate(
			&mut child_b,
			self.current_generation,
			self.max_generations,
			&mut rng,
		);

		// --- Steady-State Reinsertion ---
		// Find the two absolute worst individuals in the current population to replace
		let mut worst_indices = [0, 1];
		if self.individuals[1].fitness < self.individuals[0].fitness {
			worst_indices = [1, 0]; // [worst, second_worst]
		}

		for i in 2..self.individuals.len() {
			let fitness = self.individuals[i].fitness;
			if fitness > self.individuals[worst_indices[0]].fitness {
				worst_indices[1] = worst_indices[0];
				worst_indices[0] = i;
			} else if fitness > self.individuals[worst_indices[1]].fitness {
				worst_indices[1] = i;
			}
		}

		// Replace the worst two with the new children
		self.individuals[worst_indices[0]] = child_a;
		self.individuals[worst_indices[1]] = child_b;

		// --- Update Global Best ---
		for individual in &self.individuals {
			if individual.fitness < self.best_individual.fitness {
				self.best_individual = individual.clone();
				self.best_generation = self.current_generation;
			}
		}

		true
	}

	/// Run all generations at once (for console/batch mode).
	pub fn run(&mut self) -> RunResult {
		while self.step() {}

		RunResult {
			best_position: self.best_individual.genes,
			best_value: self.best_individual.fitness,
			generations_to_best: self.best_generation,
		}
	}
}
