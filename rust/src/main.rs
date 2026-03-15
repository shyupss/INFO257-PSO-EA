use macroquad::prelude::*;

// Configuration constants from the original sketch
const PUNTOS: usize = 100;
const W: f32 = 1000.0;      // Inertia
const MAXV: f32 = 3.0;      // Max velocity
const D: f32 = 15.0;        // Circle radius

// A simple struct to hold the image data accessible for pixel reading
struct Surface {
	img: Image, // Macroquad's native Image type
	texture: Texture2D,
}

impl Surface {
	// Load image and create a GPU texture for drawing
	async fn load(path: &str) -> Self {
		// Load file using Macroquad's native image loader
		let img = load_image(path).await.expect("Cannot open image file");

		// Create a Macroquad texture for GPU drawing
		let texture = Texture2D::from_image(&img);

		Surface { img, texture }
	}

	// Equivalent to surf.get(x, y) -> red(c)
	fn get_fitness(&self, x: f32, y: f32) -> f32 {
		// Note: width and height are properties (u16) in Macroquad
		let ix = x.clamp(0.0, self.img.width as f32 - 1.0) as u32;
		let iy = y.clamp(0.0, self.img.height as f32 - 1.0) as u32;

		// Macroquad's get_pixel returns a Color struct (r, g, b, a scaled 0.0 to 1.0)
		let pixel = self.img.get_pixel(ix, iy);
		pixel.r * 255.0 // Scale back to Processing's 0-255 style
	}
}

struct Particle {
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
	fn new(width: f32, height: f32) -> Self {
		Particle {
			// Using Macroquad's built-in RNG
			x: rand::gen_range(0.0, width),
			y: rand::gen_range(0.0, height),
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

	fn r#move(&mut self, gbest_x: f32, gbest_y: f32, width: f32, height: f32) {
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

		// Bounce off walls
		if self.x > width || self.x < 0.0 {
			self.vx = -self.vx;
		}
		if self.y > height || self.y < 0.0 {
			self.vy = -self.vy;
		}
	}

	fn display(&self, surf: &Surface) {
		let ix = self.x.clamp(0.0, surf.img.width as f32 - 1.0) as u32;
		let iy = self.y.clamp(0.0, surf.img.height as f32 - 1.0) as u32;

		// Macroquad's get_pixel returns a Color, so we can pass it directly!
		let mq_col = surf.img.get_pixel(ix, iy);

		draw_circle(self.x, self.y, D / 2.0, mq_col);

		// Draw velocity vector (red line)
		draw_line(
			self.x, 
			self.y, 
			self.x - 10.0 * self.vx, 
			self.y - 10.0 * self.vy, 
			2.0, 
			RED
		);
	}
}

// Macroquad entry point
#[macroquad::main("PSO Rust")]
async fn main() {
	// --- Setup ---

	// We need the image file in the execution directory
	let surface = Surface::load("Moon_LRO_LOLA_global_LDEM_1024_b.png").await;

	// Notice we dropped the () here because they are properties now
	let width = surface.img.width as f32;
	let height = surface.img.height as f32;

	let mut particles: Vec<Particle> = (0..PUNTOS)
		.map(|_| Particle::new(width, height))
		.collect();

	// Global best: (x, y, fitness)
	let mut gbest: (f32, f32, f32) = (0.0, 0.0, -1.0);
	let mut evals = 0;
	let mut evals_to_best = 0;

	// --- Draw Loop ---
	loop {
		clear_background(LIGHTGRAY);

		// 1. Draw Map
		draw_texture(&surface.texture, 0.0, 0.0, WHITE);

		// 2. Draw Particles
		for p in &particles {
			p.display(&surface);
		}

		// 3. Draw Best
		draw_circle(gbest.0, gbest.1, D / 2.0, BLUE);

		// Draw Text
		draw_text(
			&format!(
				"Best fitness: {:.2}\nEvals to best: {}\nEvals: {}", 
				gbest.2, evals_to_best, evals
			),
			10.0, 
			20.0, 
			20.0, 
			GREEN
		);

		// 4. Update Logic
		for p in &mut particles {
			p.r#move(gbest.0, gbest.1, width, height);
			p.eval(&surface, &mut gbest, &mut evals, &mut evals_to_best);
		}

		// Wait for next frame
		next_frame().await
	}
}
