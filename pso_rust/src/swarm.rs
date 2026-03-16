use crate::particle::Particle;

const NUM_PARTICLES: usize = 200;
const MAX_ITERATIONS: usize = 5000;

/// The particle swarm, holding all particles and tracking the global best.
pub struct Swarm {
	pub particles: Vec<Particle>,
	pub global_best_pos: [f64; 2],
	pub global_best_value: f64,
	pub current_iteration: usize,
	pub best_iteration: usize,
	pub max_iterations: usize,
}

/// Result of a full batch run.
pub struct RunResult {
	pub best_position: [f64; 2],
	pub best_value: f64,
	pub iterations_to_best: usize,
}

impl Swarm {
	pub fn new() -> Self {
		let mut rng = rand::rng();
		let particles: Vec<Particle> = (0..NUM_PARTICLES)
			.map(|_| Particle::new(&mut rng))
			.collect();

		let (global_best_pos, global_best_value) = particles
			.iter()
			.map(|particle| (particle.position, particle.current_fitness))
			.min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
			.map(|(pos, val)| ([pos[0], pos[1]], val))
			.unwrap();

		Swarm {
			particles,
			global_best_pos,
			global_best_value,
			current_iteration: 0,
			best_iteration: 0,
			max_iterations: MAX_ITERATIONS,
		}
	}

	/// Run a single PSO iteration (for animated visualization).
	/// Returns `true` if there are still iterations remaining.
	pub fn step(&mut self) -> bool {
		if self.current_iteration >= self.max_iterations {
			return false;
		}
		self.current_iteration += 1;
		let mut rng = rand::rng();

		for particle_index in 0..self.particles.len() {
			self.particles[particle_index].evaluate();
			if self.particles[particle_index].current_fitness < self.global_best_value {
				self.global_best_pos = self.particles[particle_index].position;
				self.global_best_value = self.particles[particle_index].current_fitness;
				self.best_iteration = self.current_iteration;
			}
		}

		let global_best_pos = self.global_best_pos;
		for particle in &mut self.particles {
			particle.update(&global_best_pos, &mut rng);
		}

		true
	}

	/// Run all iterations at once (for console/batch mode).
	pub fn run(&mut self) -> RunResult {
		while self.step() {}

		RunResult {
			best_position: self.global_best_pos,
			best_value: self.global_best_value,
			iterations_to_best: self.best_iteration,
		}
	}
}
