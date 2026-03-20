/// Inertia weight
/// - Higher W: Particles explore more (keep momentum)
/// - Lower W: Particles converge faster (less exploration)
	pub const INERTIA_WEIGHT: f64 = 50.0;

/// Cognitive acceleration coefficient (pull toward personal best)
pub const COGNITIVE_COEFFICIENT: f64 = 01.5;

/// Social acceleration coefficient (pull toward global best)
pub const SOCIAL_COEFFICIENT: f64 = 01.0;

/// Lower bound of the search domain
pub const DOMAIN_MIN: f64 = -3.0;

/// Upper bound of the search domain
pub const DOMAIN_MAX: f64 = 7.0;

/// Maximum velocity magnitude.
/// Limits how fast particles can move to prevent overshooting.
pub const MAX_VELOCITY: f64 = 0.03;

/// Number of particles in the swarm
pub const NUM_PARTICLES: usize =  200;

/// Maximum number of PSO iterations
pub const MAX_ITERATIONS: usize =15000;
