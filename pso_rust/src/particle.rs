//! Particle module - PSO particle implementation
//!
//! This module implements the Particle Swarm Optimization (PSO) algorithm.
//! Each particle represents a potential solution that explores the search space.
//!
//! Key concepts covered:
//! - Struct definition and field visibility
//! - Methods vs associated functions
//! - Mutable references (&mut)
//! - Tuple types
//! - Control flow (if statements)
//! - Floating-point math

use macroquad::prelude::*;

// Import Surface from the surface module (sibling module)
// 'crate' refers to the crate root (main.rs), then we navigate to surface module
// This is called an 'absolute path' import
use crate::surface::Surface;

// ============================================================================
// CONSTANTS (Configuration for PSO algorithm)
// ============================================================================
// These values control the PSO behavior and can be tuned for different problems

/// Number of particles in the swarm.
/// 
/// # Type: usize
/// usize is a pointer-sized unsigned integer:
/// - Size depends on the platform (32 or 64 bits)
/// - Used for array indexing and sizes (hence the name)
/// - Cannot be negative (unsigned)
/// - When you need an integer for counting/indexing, use usize
pub const PUNTOS: usize = 100;

/// Inertia weight - how much the particle keeps its current velocity.
/// 
/// # Type: f32
/// f32 is a 32-bit floating-point number:
/// - Use for graphics, physics, and general math
/// - f64 (64-bit) is more precise but slower
/// - Game/graphics usually use f32 (GPU native format)
/// 
/// # Value interpretation:
/// - Higher W: Particles explore more (keep momentum)
/// - Lower W: Particles converge faster (less exploration)
pub const W: f32 = 1000.0;

/// Maximum velocity magnitude.
/// Limits how fast particles can move to prevent overshooting.
pub const MAXV: f32 = 3.0;

/// Circle radius for visualization (in pixels).
pub const D: f32 = 15.0;

// ============================================================================
// PARTICLE STRUCT
// ============================================================================

/// A single particle in the PSO swarm.
///
/// # PSO Background
/// Each particle:
/// 1. Has a position (x, y) in the search space
/// 2. Has a velocity (vx, vy) determining movement direction
/// 3. Remembers its best position (px, py, pfit) found so far
/// 4. Tracks current fitness (fit)
///
/// # Field Visibility
/// All fields are pub (public) because:
/// - They're read in display() method
/// - Rust doesn't have "package-private" like Java
/// - Could use getter methods for better encapsulation
pub struct Particle {
    /// Current x position (in image coordinates)
    pub x: f32,
    
    /// Current y position (in image coordinates)
    pub y: f32,
    
    /// Velocity in x direction (pixels per frame)
    pub vx: f32,
    
    /// Velocity in y direction (pixels per frame)
    pub vy: f32,
    
    /// Personal best x position found so far
    pub px: f32,
    
    /// Personal best y position found so far
    pub py: f32,
    
    /// Fitness at personal best position (pfit = personal best fitness)
    /// Initialized to -1.0 (worse than any valid fitness 0-255)
    pub pfit: f32,
    
    /// Current fitness value
    pub fit: f32,
}

// ============================================================================
// PARTICLE IMPLEMENTATION
// ============================================================================

impl Particle {
    /// Create a new particle with random position and velocity.
    ///
    /// # Associated Function (not a method)
    /// This is called like: Particle::new(width, height)
    /// Notice no 'self' parameter - it's not called on an instance
    ///
    /// # Parameters
    /// - img_width, img_height: Bounds for random position generation
    ///
    /// # Returns
    /// - Self: The Particle type (Self is an alias for the implementing type)
    pub fn new(img_width: f32, img_height: f32) -> Self {
        // rand::gen_range is a macroquad function for random numbers
        // gen_range(min, max) returns a random value in [min, max)
        Particle {
            // Random position within image bounds
            x: rand::gen_range(0.0, img_width),
            y: rand::gen_range(0.0, img_height),
            
            // Random velocity (small values for gentle initial movement)
            vx: rand::gen_range(-1.0, 1.0),
            vy: rand::gen_range(-1.0, 1.0),
            
            // Best position starts at origin (will be updated on first eval)
            px: 0.0,
            py: 0.0,
            
            // Initialize fitness to -1.0 (impossible value)
            // Any real fitness (0-255) will be better
            pfit: -1.0,
            fit: -1.0,
        }
    }

    /// Evaluate fitness at current position and update personal/global best.
    ///
    /// # Mutable References (&mut)
    /// This method takes multiple mutable references:
    /// - &mut self: Can modify particle's fields
    /// - &mut gbest: Can modify the global best tuple
    /// - &mut evals: Can modify the evaluation counter
    ///
    /// # The Borrow Checker
    /// Rust's borrow checker ensures:
    /// - Only ONE mutable reference to a piece of data exists at a time
    /// - OR multiple immutable references exist
    /// - But NEVER both mutable and immutable simultaneously
    /// This prevents data races at compile time!
    ///
    /// # Parameters
    /// - &mut self: Mutable borrow of self (can modify fields)
    /// - surf: Immutable borrow of Surface (read fitness values)
    /// - gbest: Mutable reference to a tuple (f32, f32, f32)
    /// - evals, evals_to_best: Mutable references to counters
    ///
    /// # Tuple Type
    /// (f32, f32, f32) is a tuple containing three f32 values
    /// Access with .0, .1, .2 (zero-indexed)
    /// gbest.0 = x, gbest.1 = y, gbest.2 = fitness
    pub fn eval(
        &mut self,
        surf: &Surface,
        gbest: &mut (f32, f32, f32),
        evals: &mut i32,
        evals_to_best: &mut i32,
    ) -> f32 {
        // Increment evaluation counter
        // *evals dereferences the mutable reference to modify the value
        // Without *, you'd be trying to modify the reference itself
        *evals += 1;

        // Get fitness at current position
        // surf.get_fitness() returns f32 (0-255 range for our image)
        self.fit = surf.get_fitness(self.x, self.y);

        // Update personal best if current is better
        // No parentheses needed: comparison has higher precedence than &&
        if self.fit > self.pfit {
            self.pfit = self.fit;
            self.px = self.x;
            self.py = self.y;
        }

        // Update global best if current is better
        // Accessing tuple elements: gbest.0, gbest.1, gbest.2
        if self.fit > gbest.2 {
            gbest.0 = self.x;
            gbest.1 = self.y;
            gbest.2 = self.fit;
            
            // Dereference to modify through the mutable reference
            *evals_to_best = *evals;
            
            // println! is a macro (indicated by !)
            // Macros are expanded at compile time
            // {} is a placeholder that gets replaced by gbest.2
            println!("New best: {}", gbest.2);
        }

        // Return current fitness
        // In Rust, the last expression in a function is implicitly returned
        // No 'return' keyword needed (though you can use it for early returns)
        self.fit
    }

    /// Update particle position based on PSO velocity formula.
    ///
    /// # Method Visibility
    /// No 'pub' keyword means this method is private to the module
    /// Only code in the same module (or submodules) can call it
    ///
    /// # Raw Identifier (r#move)
    /// 'move' is a Rust keyword (used for transferring ownership)
    /// To use it as a function name, we use r# prefix: r#move
    /// This is called a "raw identifier"
    /// Calling it: particle.r#move(...)
    ///
    /// # PSO Velocity Formula
    /// v_new = W * v_old + r1 * (p_best - p) + r2 * (g_best - p)
    /// Where:
    /// - W: Inertia weight (continue in same direction)
    /// - r1, r2: Random factors (exploration)
    /// - p_best - p: Pull towards personal best (memory)
    /// - g_best - p: Pull towards global best (social)
    pub fn r#move(
        &mut self,
        gbest_x: f32,
        gbest_y: f32,
        img_width: f32,
        img_height: f32,
    ) {
        // Generate random values between 0.0 and 1.0
        // These add stochasticity (randomness) to the search
        let r1 = rand::gen_range(0.0, 1.0);
        let r2 = rand::gen_range(0.0, 1.0);

        // PSO velocity update formula
        // Each component: inertia + cognitive (personal) + social (global)
        self.vx = W * self.vx + r1 * (self.px - self.x) + r2 * (gbest_x - self.x);
        self.vy = W * self.vy + r1 * (self.py - self.y) + r2 * (gbest_y - self.y);

        // Limit velocity magnitude
        // This prevents particles from moving too fast (overshooting good areas)
        let modu = (self.vx * self.vx + self.vy * self.vy).sqrt();
        if modu > MAXV {
            // Normalize and scale to MAXV
            // Division by modu normalizes to unit vector
            // Multiplication by MAXV scales to maximum allowed speed
            self.vx = (self.vx / modu) * MAXV;
            self.vy = (self.vy / modu) * MAXV;
        }

        // Update position
        self.x += self.vx;
        self.y += self.vy;

        // Boundary handling: bounce off walls
        // This keeps particles within the search space
        if self.x > img_width || self.x < 0.0 {
            self.vx = -self.vx; // Reverse velocity
            // clamp ensures position is valid even if velocity was very large
            self.x = self.x.clamp(0.0, img_width);
        }
        if self.y > img_height || self.y < 0.0 {
            self.vy = -self.vy;
            self.y = self.y.clamp(0.0, img_height);
        }
    }

    /// Convert image-space coordinates to screen-space for drawing.
    ///
    /// # Why Two Coordinate Systems?
    /// - Image space: 0 to img_width/height (where pixels live)
    /// - Screen space: 0 to screen_width/height (where we draw)
    /// Scale factors convert between them.
    ///
    /// # Tuple Return Type
    /// Returns (f32, f32) - a tuple of screen coordinates
    /// Tuples are useful for returning multiple values.
    pub fn to_screen(&self, scale_x: f32, scale_y: f32) -> (f32, f32) {
        (self.x * scale_x, self.y * scale_y)
    }

    /// Draw the particle on screen.
    ///
    /// # Parameters
    /// - &self: Immutable borrow (we only read, don't modify)
    /// - surf: Reference to Surface for getting pixel colors
    /// - scale_x, scale_y: Factors for coordinate conversion
    pub fn display(&self, surf: &Surface, scale_x: f32, scale_y: f32) {
        // Get screen coordinates
        let (screen_x, screen_y) = self.to_screen(scale_x, scale_y);

        // Get pixel color from image at particle's position
        // We use image-space coordinates for pixel lookup
        let ix = self.x.clamp(0.0, surf.img_width - 1.0) as u32;
        let iy = self.y.clamp(0.0, surf.img_height - 1.0) as u32;
        let mq_col = surf.img.get_pixel(ix, iy);

        // Scale circle radius based on screen size
        // .min(scale_x, scale_y) ensures circle isn't stretched
        let scaled_d = (D / 2.0) * scale_x.min(scale_y);
        
        // Draw the particle as a filled circle
        // Macroquad's draw_circle takes: x, y, radius, color
        draw_circle(screen_x, screen_y, scaled_d, mq_col);

        // Draw velocity vector as a red line
        // This shows the direction the particle is moving
        let vel_scale = 10.0 * scale_x.min(scale_y);
        draw_line(
            screen_x,
            screen_y,
            screen_x - vel_scale * self.vx, // Line points opposite to velocity
            screen_y - vel_scale * self.vy,
            2.0, // Line thickness
            RED, // Color constant from macroquad
        );
    }
}
