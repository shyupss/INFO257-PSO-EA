//! Drawing module - visualization helpers for PSO on the Rastrigin function
//!
//! Provides functions to draw particles, the global best marker,
//! and an information overlay on top of the heatmap.

use macroquad::prelude::*;
use crate::swarm::Swarm;
use crate::surface::RastriginSurface;

/// Radius of each particle circle (in screen pixels)
const PARTICLE_RADIUS: f32 = 5.0;

/// Radius of the global best marker
const GLOBAL_BEST_RADIUS: f32 = 8.0;

/// Draw all particles in the swarm on the screen.
/// Each particle is drawn as a white circle with a semi-transparent fill
/// and an opaque border for visibility against the heatmap.
pub fn draw_particles(swarm: &Swarm, surface: &RastriginSurface) {
	let screen_width = screen_width();
	let screen_height = screen_height();

	for particle in &swarm.particles {
		let (screen_x, screen_y) = surface.domain_to_screen(
			particle.position[0],
			particle.position[1],
			screen_width,
			screen_height,
		);

		// Opaque gray fill
		draw_circle(screen_x, screen_y, PARTICLE_RADIUS, Color::new(0.55, 0.55, 0.55, 1.0));
		// Darker gray border for contrast
		draw_circle_lines(screen_x, screen_y, PARTICLE_RADIUS, 1.5, Color::new(0.3, 0.3, 0.3, 1.0));
	}
}

/// Draw the global best position as a pulsing red marker.
pub fn draw_global_best(swarm: &Swarm, surface: &RastriginSurface) {
	let screen_width = screen_width();
	let screen_height = screen_height();

	let (screen_x, screen_y) = surface.domain_to_screen(
		swarm.global_best_pos[0],
		swarm.global_best_pos[1],
		screen_width,
		screen_height,
	);

	// Pulsing effect using sine wave on the elapsed time
	let pulse = 1.0 + 0.3 * (get_time() as f32 * 4.0).sin();
	let pulsing_radius = GLOBAL_BEST_RADIUS * pulse;

	// Outer glow
	draw_circle(screen_x, screen_y, pulsing_radius + 3.0, Color::new(1.0, 0.0, 0.0, 0.25));
	// Main marker
	draw_circle(screen_x, screen_y, pulsing_radius, Color::new(1.0, 0.15, 0.15, 0.9));
	// Inner bright dot
	draw_circle(screen_x, screen_y, 3.0, WHITE);
}

/// Draw the info overlay showing iteration, best value, position, and FPS.
pub fn draw_info_overlay(swarm: &Swarm, cached_fps: i32) {
	let background_width = 320.0;
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

	let status_text = if swarm.current_iteration >= swarm.max_iterations {
		"CONVERGED"
	} else {
		"Running..."
	};

	let lines = [
		format!("Iteration: {} / {}  [{}]", swarm.current_iteration, swarm.max_iterations, status_text),
		format!("Best value: {:.6}", swarm.global_best_value),
		format!("Best pos: ({:.4}, {:.4})", swarm.global_best_pos[0], swarm.global_best_pos[1]),
		format!("Found at iteration: {}", swarm.best_iteration),
		format!("FPS: {}", cached_fps),
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
