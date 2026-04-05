//! EA Rust - Evolutionary Algorithm
//!
//! Supports two modes controlled by `VISUAL_MODE`:
//! - **Visual mode** (`true`): Real-time macroquad visualization of EA on the Rastrigin function
//! - **Console mode** (`false`): Batch execution of multiple runs with statistical analysis
//!
//! # Project Structure
//! ```text
//! src/
//! ├── main.rs        (this file) - Entry point, mode switch
//! ├── surface.rs     - Rastrigin heatmap generation
//! ├── drawing.rs     - UI drawing functions
//! └── ea/
//!     ├── mod.rs         - Module re-exports
//!     ├── params.rs      - EA parameter constants
//!     ├── individual.rs  - Individual (chromosome) implementation
//!     └── population.rs  - Population management and evolutionary loop
//! ```

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

/// Surface module - Rastrigin heatmap generation
mod surface;

/// EA module containing individual and population logic
mod ea;

/// Drawing module - UI rendering functions
mod drawing;

// ============================================================================
// IMPORTS
// ============================================================================

use drawing::{draw_best_individual, draw_individuals, draw_info_overlay, should_quit};
use ea::params::{DOMAIN_MAX, DOMAIN_MIN};
use ea::population::{Population, RunResult};
use macroquad::prelude::*;
use surface::RastriginSurface;

/// Seconds between each EA generation step.
const STEP_INTERVAL: f64 = 0.034;

/// Seconds between FPS counter refreshes.
const FPS_REFRESH_INTERVAL: f64 = 1.0;

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Set to `true` for graphical visualization, `false` for console batch mode.
const VISUAL_MODE: bool = true;

/// Number of independent runs for console batch mode.
const NUM_RUNS: usize = 30;

// ============================================================================
// STATISTICS (console mode)
// ============================================================================

/// Calculate statistics from multiple EA runs.
/// Returns (min, max, mean, `std_deviation`, `success_count`).
fn calculate_stats(results: &[RunResult]) -> (f64, f64, f64, f64, usize) {
	let values: Vec<f64> = results.iter().map(|r| r.best_value).collect();
	let min = values.iter().copied().fold(f64::INFINITY, f64::min);
	let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
	let mean = values.iter().sum::<f64>() / values.len() as f64;
	let standard_deviation =
		(values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64).sqrt();
	let successes = values.iter().filter(|&&v| v < 0.01).count();
	(min, max, mean, standard_deviation, successes)
}

/// Run EA in console batch mode: execute `NUM_RUNS` independent runs,
/// print per-run results, and display aggregate statistics.
fn run_console_mode() {
	println!("╔═══════════════════════════════════════════════════════════════╗");
	println!("║  EA - Minimización de la Función Rastrigin en 2D             ║");
	println!("╚═══════════════════════════════════════════════════════════════╝");
	println!();
	println!("Función: f(x₁, x₂) = 20 + x₁² - 10·cos(2πx₁) + x₂² - 10·cos(2πx₂)");
	println!("Dominio: {DOMAIN_MIN:.1} < xᵢ < {DOMAIN_MAX:.1}");
	println!("Mínimo global: f(0, 0) = 0");
	println!();

	let mut results: Vec<RunResult> = Vec::with_capacity(NUM_RUNS);

	println!("Ejecutando {NUM_RUNS} corridas independientes...\n");
	println!(
		"{:>8} {:>15} {:>15} {:>15} {:>10}",
		"Corrida", "x₁", "x₂", "f(x₁,x₂)", "Gen"
	);
	println!("{}", "-".repeat(67));

	for run in 1..=NUM_RUNS {
		let mut population = Population::new();
		let result = population.run();
		println!(
			"{:>8} {:>15.8} {:>15.8} {:>15.8} {:>10}",
			run,
			result.best_position[0],
			result.best_position[1],
			result.best_value,
			result.generations_to_best
		);
		results.push(result);
	}

	let (min, max, mean, standard_deviation, successes) = calculate_stats(&results);

	println!();
	println!("┌─────────────────────────────────────────┐");
	println!("│ Estadísticas sobre {NUM_RUNS} corridas:          │");
	println!("├─────────────────────────────────────────┤");
	println!("│ Mínimo:  {min:>30.8} │");
	println!("│ Máximo:  {max:>30.8} │");
	println!("│ Media:   {mean:>30.8} │");
	println!("│ Std Dev: {standard_deviation:>30.8} │");
	println!("│ Éxitos (f < 0.01): {successes:>20} │");
	println!(
		"│ Tasa de éxito: {:>22.1}% │",
		(successes as f64 / NUM_RUNS as f64) * 100.0
	);
	println!("└─────────────────────────────────────────┘");
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

/// Window configuration for macroquad (used in both modes).
fn window_configuration() -> Conf {
	Conf {
		window_title: "EA - Rastrigin Function Visualization (Q/ESC to quit)".to_string(),
		window_width: 800,
		window_height: 800,
		window_resizable: true,
		..Default::default()
	}
}

#[macroquad::main(window_configuration)]
async fn main() {
	if VISUAL_MODE {
		// Generate the Rastrigin heatmap (done once at startup)
		let surface = RastriginSurface::new();

		// Create the evolutionary population
		let mut population = Population::new();

		// Timers for throttled stepping and FPS refresh
		let mut step_accumulator: f64 = 0.0;
		let mut fps_accumulator: f64 = 0.0;
		let mut cached_fps: i32 = 0;

		// Main game loop
		loop {
			if should_quit() {
				break;
			}

			let delta_time = f64::from(get_frame_time());

			// ---- Throttled EA stepping ----
			step_accumulator += delta_time;
			while step_accumulator >= STEP_INTERVAL {
				population.step();
				step_accumulator -= STEP_INTERVAL;
			}

			// ---- FPS refresh once per second ----
			fps_accumulator += delta_time;
			if fps_accumulator >= FPS_REFRESH_INTERVAL {
				cached_fps = get_fps();
				fps_accumulator -= FPS_REFRESH_INTERVAL;
			}

			// Clear and draw
			clear_background(BLACK);
			surface.draw();
			draw_individuals(&population, &surface);
			draw_best_individual(&population, &surface);
			draw_info_overlay(&population, cached_fps);

			next_frame().await;
		}
	} else {
		run_console_mode();
	}
}
