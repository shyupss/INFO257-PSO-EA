use rand::RngExt;
use std::f64::consts::PI;

use super::params::{DOMAIN_MAX, DOMAIN_MIN};

/// Rastrigin function for 2D input.
/// Global minimum: f(0, 0) = 0
pub fn rastrigin(pos: [f64; 2]) -> f64 {
	let n = 2.0;
	let [x, y] = pos;
	let sum = x.powi(2) - 10.0 * (2.0 * PI * x).cos() + y.powi(2) - 10.0 * (2.0 * PI * y).cos();
	10.0 * n + sum
}

// ============================================================================
// INDIVIDUAL STRUCT
// ============================================================================

/// A single individual in the evolutionary population.
///
/// Each individual:
/// 1. Has genes (x, y) representing a candidate solution in the search space
/// 2. Has a fitness value computed by the Rastrigin function
pub struct Individual {
	/// Genes: the candidate solution coordinates
	pub genes: [f64; 2],
	/// Fitness value (lower is better for minimization)
	pub fitness: f64,
}

impl Individual {
	/// Create a new individual with random genes uniformly distributed in the domain.
	pub fn new_random(rng: &mut rand::rngs::ThreadRng) -> Self {
		let genes = [
			rng.random_range(DOMAIN_MIN..=DOMAIN_MAX),
			rng.random_range(DOMAIN_MIN..=DOMAIN_MAX),
		];
		let fitness = rastrigin(genes);
		Individual { genes, fitness }
	}

	/// Evaluate the individual's fitness using the Rastrigin function.
	pub fn evaluate(&mut self) {
		self.fitness = rastrigin(self.genes);
	}

	/// Clamp genes to remain within the search domain boundaries.
	pub fn clamp_to_domain(&mut self) {
		for gene in &mut self.genes {
			*gene = gene.clamp(DOMAIN_MIN, DOMAIN_MAX);
		}
	}
}

impl Clone for Individual {
	fn clone(&self) -> Self {
		Individual {
			genes: self.genes,
			fitness: self.fitness,
		}
	}
}
