use macroquad::prelude::*;

// Configuration constants from the original sketch
const PUNTOS: usize = 100;
const W: f32 = 1000.0;      // Inertia
const MAXV: f32 = 3.0;      // Max velocity
const D: f32 = 15.0;        // Circle radius
const SURFACE_IMAGE: &str = "../assets/Moon_LRO_LOLA_global_LDEM_1024_b.png";

// A simple struct to hold the image data accessible for pixel reading
struct Surface {
	img: Image, // Macroquad's native Image type
	texture: Texture2D,
	// Original image dimensions
	img_width: f32,
	img_height: f32,
}

	impl Surface {
		// Load image and create a GPU texture for drawing
		async fn load(path: &str) -> Self {
			// Load file using Macroquad's native image loader
			let img = load_image(path).await.expect("Cannot open image file");

			// Store original dimensions
			let img_width = img.width as f32;
			let img_height = img.height as f32;

			// Create a Macroquad texture for GPU drawing
			let texture = Texture2D::from_image(&img);

			Surface { 
				img, 
				texture,
				img_width,
				img_height,
			}
		}

		// Equivalent to surf.get(x, y) -> red(c)
		// Takes image-space coordinates (0 to img_width/height)
		fn get_fitness(&self, x: f32, y: f32) -> f32 {
			// Note: width and height are properties (u16) in Macroquad
			let ix = x.clamp(0.0, self.img_width - 1.0) as u32;
			let iy = y.clamp(0.0, self.img_height - 1.0) as u32;

			// Macroquad's get_pixel returns a Color struct (r, g, b, a scaled 0.0 to 1.0)
			let pixel = self.img.get_pixel(ix, iy);
			pixel.r * 255.0 // Scale back to Processing's 0-255 style
		}
	}

struct Particle {
	// Store coordinates in IMAGE SPACE (0 to img_width/height)
	x: f32,
	y: f32,
	vx: f32,
	vy: f32,
	px: f32, // best x
	py: f32, // best y
	pfit: f32, // best fitness
	fit: f32, // current fitness
}

impl Particle {
	fn new(img_width: f32, img_height: f32) -> Self {
		Particle {
			// Using Macroquad's built-in RNG - coordinates in image space
			x: rand::gen_range(0.0, img_width),
			y: rand::gen_range(0.0, img_height),
			vx: rand::gen_range(-1.0, 1.0),
			vy: rand::gen_range(-1.0, 1.0),
			px: 0.0,
			py: 0.0,
			pfit: -1.0,
			fit: -1.0,
		}
	}

	// Returns fitness, updates local best. Takes mutable reference to global best.
	fn eval(&mut self, surf: &Surface, gbest: &mut (f32, f32, f32), evals: &mut i32, evals_to_best: &mut i32) -> f32 {
		*evals += 1;

		self.fit = surf.get_fitness(self.x, self.y);

		// Update local best
		if self.fit > self.pfit {
			self.pfit = self.fit;
			self.px = self.x;
			self.py = self.y;
		}

		// Update global best
		if self.fit > gbest.2 {
			gbest.0 = self.x;
			gbest.1 = self.y;
			gbest.2 = self.fit;
			*evals_to_best = *evals;
			println!("New best: {}", gbest.2);
		}

		self.fit
	}

	fn r#move(&mut self, gbest_x: f32, gbest_y: f32, img_width: f32, img_height: f32) {
		// Generate f32s between 0.0 and 1.0
		let r1 = rand::gen_range(0.0, 1.0);
		let r2 = rand::gen_range(0.0, 1.0);

		// Velocity update formula (matching the active formula in the sketch)
		// vx = w * vx + random*(px - x) + random*(gbestx - x)
		self.vx = W * self.vx + r1 * (self.px - self.x) + r2 * (gbest_x - self.x);
		self.vy = W * self.vy + r1 * (self.py - self.y) + r2 * (gbest_y - self.y);

		// Truncate velocity (module)
		let modu = (self.vx * self.vx + self.vy * self.vy).sqrt();
		if modu > MAXV {
			self.vx = (self.vx / modu) * MAXV;
			self.vy = (self.vy / modu) * MAXV;
		}

		// Update position
		self.x += self.vx;
		self.y += self.vy;

		// Bounce off walls (in image space)
		if self.x > img_width || self.x < 0.0 {
			self.vx = -self.vx;
			self.x = self.x.clamp(0.0, img_width);
		}
		if self.y > img_height || self.y < 0.0 {
			self.vy = -self.vy;
			self.y = self.y.clamp(0.0, img_height);
		}
	}

	/// Convert image-space coordinates to screen-space for drawing
	fn to_screen(&self, scale_x: f32, scale_y: f32) -> (f32, f32) {
		(self.x * scale_x, self.y * scale_y)
	}

	fn display(&self, surf: &Surface, scale_x: f32, scale_y: f32) {
		// Get screen coordinates
		let (screen_x, screen_y) = self.to_screen(scale_x, scale_y);

		// Get pixel color from image-space coordinates
		let ix = self.x.clamp(0.0, surf.img_width - 1.0) as u32;
		let iy = self.y.clamp(0.0, surf.img_height - 1.0) as u32;
		let mq_col = surf.img.get_pixel(ix, iy);

		// Scale circle radius based on screen size
		let scaled_d = (D / 2.0) * scale_x.min(scale_y);
		draw_circle(screen_x, screen_y, scaled_d, mq_col);

		// Draw velocity vector (red line) - also scaled
		let vel_scale = 10.0 * scale_x.min(scale_y);
		draw_line(
			screen_x, 
			screen_y, 
			screen_x - vel_scale * self.vx, 
			screen_y - vel_scale * self.vy, 
			2.0, 
			RED
		);
	}
}

/// Draw a close button (X) in the top-right corner
/// Returns true if the button was clicked
fn draw_close_button(screen_width: f32) -> bool {
	let button_size = 30.0;
	let margin = 10.0;
	let button_x = screen_width - button_size - margin;
	let button_y = margin;

	// Draw button background
	draw_rectangle(button_x, button_y, button_size, button_size, DARKGRAY);

	// Draw X symbol using two lines
	let padding = 8.0;
	let line_thickness = 3.0;

	// Line from top-left to bottom-right
	draw_line(
		button_x + padding,
		button_y + padding,
		button_x + button_size - padding,
		button_y + button_size - padding,
		line_thickness,
		RED
	);

	// Line from top-right to bottom-left
	draw_line(
		button_x + button_size - padding,
		button_y + padding,
		button_x + padding,
		button_y + button_size - padding,
		line_thickness,
		RED
	);

	// Check if mouse is hovering over button (for visual feedback)
	let (mouse_x, mouse_y) = mouse_position();
	let is_hovering = mouse_x >= button_x 
		&& mouse_x <= button_x + button_size 
		&& mouse_y >= button_y 
		&& mouse_y <= button_y + button_size;

	// Draw hover effect
	if is_hovering {
		draw_rectangle_lines(button_x, button_y, button_size, button_size, 2.0, YELLOW);
	}

	// Check for click
	is_hovering && is_mouse_button_pressed(MouseButton::Left)
}

/// Check if the user wants to quit
/// Returns true if Q key pressed, Escape pressed, or close button clicked
fn should_quit(screen_width: f32) -> bool {
	// Check for Q key
	if is_key_pressed(KeyCode::Q) {
		println!("Quit requested via Q key");
		return true;
	}

	// Check for Escape key (alternative)
	if is_key_pressed(KeyCode::Escape) {
		println!("Quit requested via Escape key");
		return true;
	}

	// Check for close button click
	if draw_close_button(screen_width) {
		println!("Quit requested via close button");
		return true;
	}

	false
}

// Macroquad entry point
#[macroquad::main("PSO Rust - Press Q or click X to close")]
async fn main() {
	// --- Setup ---

	// We need the image file in the execution directory
	let surface = Surface::load(SURFACE_IMAGE).await;

	// Image dimensions (fixed)
	let img_width = surface.img_width;
	let img_height = surface.img_height;

	let mut particles: Vec<Particle> = (0..PUNTOS)
		.map(|_| Particle::new(img_width, img_height))
		.collect();

	// Global best: (x, y, fitness) in IMAGE SPACE
	let mut gbest: (f32, f32, f32) = (0.0, 0.0, -1.0);
	let mut evals = 0;
	let mut evals_to_best = 0;

	// --- Draw Loop ---
	loop {
		clear_background(BLACK);

		// Get current screen dimensions (may change with window resize)
		let screen_w = screen_width();
		let screen_h = screen_height();

		// Calculate scale factors to fill screen
		let scale_x = screen_w / img_width;
		let scale_y = screen_h / img_height;

		// Check for quit condition at the start of each frame
		if should_quit(screen_w) {
			println!("Closing PSO application...");
			std::process::exit(0);
		}

		// 1. Draw Map - scaled to fill the entire screen
		let params = DrawTextureParams {
			dest_size: Some(vec2(screen_w, screen_h)),
			..Default::default()
		};
		draw_texture_ex(&surface.texture, 0.0, 0.0, WHITE, params);

		// 2. Draw Particles (with screen coordinate conversion)
		for p in &particles {
			p.display(&surface, scale_x, scale_y);
		}

		// 3. Draw Best (convert from image space to screen space)
		let gbest_screen_x = gbest.0 * scale_x;
		let gbest_screen_y = gbest.1 * scale_y;
		let scaled_d = (D / 2.0) * scale_x.min(scale_y);
		draw_circle(gbest_screen_x, gbest_screen_y, scaled_d, BLUE);

		// Draw Text with instructions
		draw_text(
			&format!(
				"Best fitness: {:.2}\nEvals to best: {}\nEvals: {}\nScreen: {:.0}x{:.0}\nImage: {:.0}x{:.0}\n\nPress Q or ESC to quit\nOr click X button (top-right)", 
				gbest.2, evals_to_best, evals, screen_w, screen_h, img_width, img_height
			),
			10.0, 
			20.0, 
			18.0, 
			GREEN
		);

		// 4. Update Logic (in image space)
		for p in &mut particles {
			p.r#move(gbest.0, gbest.1, img_width, img_height);
			p.eval(&surface, &mut gbest, &mut evals, &mut evals_to_best);
		}

		// Wait for next frame
		next_frame().await
	}
}
