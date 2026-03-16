use rand::RngExt;
use std::f64::consts::PI;

use super::params::*;

/// Rastrigin function for 2D input.
/// Global minimum: f(0, 0) = 0
pub fn rastrigin(pos: [f64; 2]) -> f64 {
	let n = 2.0;
	let [x, y] = pos;
	let sum = x.powi(2) - 10.0 * (2.0 * PI * x).cos() + y.powi(2) - 10.0 * (2.0 * PI * y).cos();
	10.0 * n + sum
}

// ============================================================================
// PARTICLE STRUCT
// ============================================================================

/// A single particle in the PSO swarm.
///
/// Each particle:
/// 1. Has a position (x, y) in the search space
/// 2. Has a velocity (vx, vy) determining movement direction
/// 3. Remembers its best position found so far
/// 4. Tracks current fitness
pub struct Particle {
	/// Current position in search space
	pub position: [f64; 2],
	/// Velocity vector
	pub velocity: [f64; 2],
	/// Personal best position found so far
	pub personal_best_pos: [f64; 2],
	/// Personal best fitness value
	pub personal_best_fit: f64,
	/// Current fitness value
	pub current_fitness: f64,
}

impl Particle {
	pub fn new(rng: &mut rand::rngs::ThreadRng) -> Self {
		let position = [
			rng.random_range(DOMAIN_MIN..=DOMAIN_MAX),
			rng.random_range(DOMAIN_MIN..=DOMAIN_MAX),
		];
		let velocity = [
			rng.random_range(-MAX_VELOCITY..=MAX_VELOCITY),
			rng.random_range(-MAX_VELOCITY..=MAX_VELOCITY),
		];
		let value = rastrigin(position);

		Particle {
			position,
			velocity,
			personal_best_pos: position,
			personal_best_fit: value,
			current_fitness: value,
		}
	}

	pub fn evaluate(&mut self) -> f64 {
		self.current_fitness = rastrigin(self.position);
		if self.current_fitness < self.personal_best_fit {
			self.personal_best_pos = self.position;
			self.personal_best_fit = self.current_fitness;
		}
		self.current_fitness
	}

	pub fn update(&mut self, global_best: &[f64; 2], rng: &mut rand::rngs::ThreadRng) {
		let r1: f64 = rng.random_range(0.0..=1.0);
		let r2: f64 = rng.random_range(0.0..=1.0);

		for i in 0..2 {
			self.velocity[i] = INERTIA_WEIGHT * self.velocity[i]
				+ COGNITIVE_COEFFICIENT * r1 * (self.personal_best_pos[i] - self.position[i])
				+ SOCIAL_COEFFICIENT * r2 * (global_best[i] - self.position[i]);

			self.velocity[i] = self.velocity[i].clamp(-MAX_VELOCITY, MAX_VELOCITY);
			self.position[i] += self.velocity[i];

			if self.position[i] < DOMAIN_MIN {
				self.position[i] = DOMAIN_MIN;
				self.velocity[i] *= -0.5;
			} else if self.position[i] > DOMAIN_MAX {
				self.position[i] = DOMAIN_MAX;
				self.velocity[i] *= -0.5;
			}
		}
	}
}
