/// Lower bound of the search domain
pub const DOMAIN_MIN: f64 = -3.0;

/// Upper bound of the search domain
pub const DOMAIN_MAX: f64 = 7.0;

/// Number of individuals in the population
pub const POPULATION_SIZE: usize = 050;

/// Maximum number of generations to evolve
pub const MAX_GENERATIONS: usize = 5000;

/// Probability of applying crossover to a pair of parents
/// - Higher: more offspring mixing, faster convergence
/// - Lower: more parents survive unchanged, more diversity
pub const CROSSOVER_PROBABILITY: f64 = 0.3;

/// Probability of mutating each gene in an individual
/// - Higher: more random exploration
/// - Lower: more stability in good solutions
pub const MUTATION_PROBABILITY: f64 = 0.40;

/// Number of individuals competing in each tournament selection round
/// - Higher k: stronger selection pressure (exploitation)
/// - Lower k: weaker selection pressure (exploration)
pub const TOURNAMENT_SIZE: usize = 2;

/// BLX-α expansion factor for blend crossover
/// - α = 0: offspring lie strictly between parents
/// - α > 0: offspring can explore beyond parent range
pub const BLX_ALPHA: f64 = 0.5;

/// Initial standard deviation for Gaussian mutation (high → exploration)
pub const INITIAL_MUTATION_SIGMA: f64 = 0.5;

/// Final standard deviation for Gaussian mutation (low → exploitation)
pub const FINAL_MUTATION_SIGMA: f64 = 0.20;
