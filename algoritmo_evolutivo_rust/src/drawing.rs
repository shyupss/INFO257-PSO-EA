//! Drawing module - visualization helpers for EA on the Rastrigin function
//!
//! Provides functions to draw individuals, the best individual marker,
//! and an information overlay on top of the heatmap.

use crate::ea::population::Population;
use crate::surface::RastriginSurface;
use macroquad::prelude::*;

/// Radius of each individual circle (in screen pixels)
const INDIVIDUAL_RADIUS: f32 = 5.0;

/// Radius of the best individual marker
const BEST_INDIVIDUAL_RADIUS: f32 = 8.0;

/// Draw all individuals in the population on the screen.
/// Each individual is drawn as a gray circle with a darker border for visibility.
pub fn draw_individuals(population: &Population, surface: &RastriginSurface) {
	for individual in &population.individuals {
		let (screen_x, screen_y) =
			surface.domain_to_screen(individual.genes[0], individual.genes[1]);

		// Opaque gray fill
		draw_circle(
			screen_x,
			screen_y,
			INDIVIDUAL_RADIUS,
			Color::new(0.55, 0.55, 0.55, 1.0),
		);
		// Darker gray border for contrast
		draw_circle_lines(
			screen_x,
			screen_y,
			INDIVIDUAL_RADIUS,
			1.5,
			Color::new(0.3, 0.3, 0.3, 1.0),
		);
	}
}

/// Draw the best individual position as a pulsing red marker.
pub fn draw_best_individual(population: &Population, surface: &RastriginSurface) {
	let (screen_x, screen_y) = surface.domain_to_screen(
		population.best_individual.genes[0],
		population.best_individual.genes[1],
	);

	// Pulsing effect using sine wave on the elapsed time
	let pulse = 1.0 + 0.3 * (get_time() as f32 * 4.0).sin();
	let pulsing_radius = BEST_INDIVIDUAL_RADIUS * pulse;

	// Outer glow
	draw_circle(
		screen_x,
		screen_y,
		pulsing_radius + 3.0,
		Color::new(1.0, 0.0, 0.0, 0.25),
	);
	// Main marker
	draw_circle(
		screen_x,
		screen_y,
		pulsing_radius,
		Color::new(1.0, 0.15, 0.15, 0.9),
	);
	// Inner bright dot
	draw_circle(screen_x, screen_y, 3.0, WHITE);
}

/// Draw the info overlay showing generation, best value, position, and FPS.
pub fn draw_info_overlay(population: &Population, cached_fps: i32) {
	let background_width = 340.0;
	let background_height = 115.0;
	let margin = 10.0;

	// Semi-transparent dark background
	draw_rectangle(
		margin,
		margin,
		background_width,
		background_height,
		Color::new(0.0, 0.0, 0.0, 0.75),
	);

	let text_x = margin + 10.0;
	let text_size = 18.0;
	let line_spacing = 22.0;
	let mut current_y = margin + 22.0;

	let status_text = if population.current_generation >= population.max_generations {
		"CONVERGED"
	} else {
		"Running..."
	};

	let lines = [
		format!(
			"Generation: {} / {}  [{}]",
			population.current_generation, population.max_generations, status_text
		),
		format!("Best value: {:.6}", population.best_individual.fitness),
		format!(
			"Best pos: ({:.4}, {:.4})",
			population.best_individual.genes[0], population.best_individual.genes[1]
		),
		format!("Found at generation: {}", population.best_generation),
		format!("FPS: {cached_fps}"),
	];

	for line in &lines {
		draw_text(line, text_x, current_y, text_size, GREEN);
		current_y += line_spacing;
	}
}

/// Check if the user wants to quit (Q, Escape, or close button click).
pub fn should_quit() -> bool {
	is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape)
}
