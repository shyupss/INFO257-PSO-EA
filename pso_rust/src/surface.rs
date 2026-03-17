//! Surface module - generates a Rastrigin function heatmap texture
//!
//! Replaces the old image-loading approach with a procedurally generated
//! heatmap of the Rastrigin function over the search domain.

use crate::pso::params::{DOMAIN_MAX, DOMAIN_MIN};
use crate::pso::particle::rastrigin;
use macroquad::prelude::*;

/// Resolution (pixels) for the heatmap texture
const HEATMAP_RESOLUTION: u16 = 2024;

/// Fixed size (pixels) for rendering the heatmap to maintaining aspect ratio
const FIXED_SCREEN_SIZE: f32 = 800.0;

/// Holds the precomputed Rastrigin heatmap texture and coordinate mapping info.
pub struct RastriginSurface {
	/// GPU texture of the heatmap
	pub texture: Texture2D,
	/// Domain bounds for coordinate conversions
	pub domain_min: f64,
	pub domain_max: f64,
}

impl RastriginSurface {
	/// Generate the heatmap texture from the Rastrigin function.
	pub fn new() -> Self {
		let resolution = HEATMAP_RESOLUTION;
		let mut image = Image::gen_image_color(resolution, resolution, BLACK);

		// Find the max value in the domain for color normalization
		let mut max_value: f64 = 0.0;
		for pixel_y in 0..resolution {
			for pixel_x in 0..resolution {
				let domain_x = Self::pixel_to_domain(pixel_x, resolution);
				let domain_y = Self::pixel_to_domain(pixel_y, resolution);
				let value = rastrigin([domain_x, domain_y]);
				if value > max_value {
					max_value = value;
				}
			}
		}

		// Generate the heatmap pixels
		for pixel_y in 0..resolution {
			for pixel_x in 0..resolution {
				let domain_x = Self::pixel_to_domain(pixel_x, resolution);
				let domain_y = Self::pixel_to_domain(pixel_y, resolution);
				let value = rastrigin([domain_x, domain_y]);
				let normalized = (value / max_value) as f32;
				let color = Self::value_to_color(normalized);
				image.set_pixel(u32::from(pixel_x), u32::from(pixel_y), color);
			}
		}

		let texture = Texture2D::from_image(&image);
		texture.set_filter(FilterMode::Linear);

		RastriginSurface {
			texture,
			domain_min: DOMAIN_MIN,
			domain_max: DOMAIN_MAX,
		}
	}

	/// Convert a pixel coordinate to the domain value.
	fn pixel_to_domain(pixel: u16, resolution: u16) -> f64 {
		let ratio = f64::from(pixel) / f64::from(resolution - 1);
		DOMAIN_MIN + ratio * (DOMAIN_MAX - DOMAIN_MIN)
	}

	/// Convert domain coordinates to screen (pixel) coordinates.
	pub fn domain_to_screen(&self, domain_x: f64, domain_y: f64) -> (f32, f32) {
		let normalized_x = (domain_x - self.domain_min) / (self.domain_max - self.domain_min);
		let normalized_y = (domain_y - self.domain_min) / (self.domain_max - self.domain_min);
		let screen_x = normalized_x as f32 * FIXED_SCREEN_SIZE;
		let screen_y = normalized_y as f32 * FIXED_SCREEN_SIZE;
		(screen_x, screen_y)
	}

	/// Map a normalized value [0..1] to a color on a blue→cyan→green→yellow→red gradient.
	fn value_to_color(normalized_value: f32) -> Color {
		// Apply sqrt to spread more color range across lower values (where the action is)
		let mapped_value = normalized_value.sqrt();

		let (red, green, blue) = if mapped_value < 0.25 {
			// Dark blue → Cyan
			let interpolation = mapped_value / 0.25;
			(0.0, interpolation, 0.6 + 0.4 * interpolation)
		} else if mapped_value < 0.5 {
			// Cyan → Green
			let interpolation = (mapped_value - 0.25) / 0.25;
			(0.0, 1.0, 1.0 - interpolation)
		} else if mapped_value < 0.75 {
			// Green → Yellow
			let interpolation = (mapped_value - 0.5) / 0.25;
			(interpolation, 1.0, 0.0)
		} else {
			// Yellow → Red
			let interpolation = (mapped_value - 0.75) / 0.25;
			(1.0, 1.0 - interpolation, 0.0)
		};

		Color::new(red, green, blue, 1.0)
	}

	/// Draw the heatmap texture scaled to a fixed screen size.
	pub fn draw(&self) {
		let params = DrawTextureParams {
			dest_size: Some(vec2(FIXED_SCREEN_SIZE, FIXED_SCREEN_SIZE)),
			..Default::default()
		};
		draw_texture_ex(&self.texture, 0.0, 0.0, WHITE, params);
	}
}
