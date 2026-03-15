//! PSO Rust - Particle Swarm Optimization Visualization
//!
//! This is the entry point for the PSO visualization application.
//! It demonstrates how to organize a Rust project across multiple modules.
//!
//! # Project Structure
//! ```text
//! src/
//! ├── main.rs      (this file) - Entry point, main loop
//! ├── surface.rs   - Image loading and fitness evaluation
//! ├── particle.rs  - PSO particle implementation
//! └── drawing.rs   - UI drawing functions
//! ```
//!
//! Key concepts covered:
//! - Module system (mod, pub mod, use)
//! - Program entry point
//! - Async runtime with macroquad
//! - Game loop pattern

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================
// In Rust, each .rs file is a module. To use them, we must declare them here.
// The module name matches the file name (without .rs extension)

/// Surface module - handles image/terrain functionality
/// 
/// # Module Visibility
/// Without 'pub', modules are private to this file.
/// We could also use 'pub mod' if other crates needed access.
mod surface;

/// Particle module - PSO algorithm implementation
mod particle;

/// Drawing module - UI rendering functions
mod drawing;

// ============================================================================
// IMPORTS
// ============================================================================
// The 'use' keyword brings items into scope so we don't need full paths

// Import everything (*) from macroquad's prelude
// This gives us: draw_circle, clear_background, next_frame, etc.
use macroquad::prelude::*;

// Import specific items from our modules
// This lets us use 'Particle' instead of 'particle::Particle'
// The syntax is: use module_name::item_name
use particle::Particle;
use surface::Surface;

// Import our drawing functions
// We could also use 'use drawing::*' to import everything
use drawing::{draw_global_best, draw_info_overlay, draw_scaled_texture, should_quit};

// ============================================================================
// MAIN FUNCTION
// ============================================================================
// Every Rust program has a main() function as its entry point.
// This is where execution begins.

/// The main entry point for the PSO visualization.
///
/// # The #[macroquad::main] Attribute
/// Attributes (#[...]) are metadata attached to code items.
/// This specific attribute:
/// 1. Sets up the macroquad runtime (window, OpenGL context, event loop)
/// 2. Makes the function async (macroquad uses async/await)
/// 3. The string argument sets the window title
///
/// # async fn main()
/// Normally main() returns () or Result<(), Error>.
/// With macroquad, it becomes async:
/// - The runtime handles the Future returned by async
/// - We can use .await inside main
///
/// # The Game Loop Pattern
/// Macroquad uses a specific pattern for games:
/// ```ignore
/// loop {
///     // 1. Clear screen
///     // 2. Draw everything
///     // 3. Update logic
///     // 4. next_frame().await
/// }
/// ```
/// The loop runs once per frame. next_frame().await pauses until
/// the next screen refresh, giving control back to the OS.
#[macroquad::main("PSO Rust - Press Q or click X to close")]
async fn main() {
    // ========================================================================
    // SETUP PHASE
    // ========================================================================
    // Initialize all data before the main loop starts
    // This runs once at program start

    // -------------------------------------------------------------------------
    // Load the terrain image
    // -------------------------------------------------------------------------
    // We use .await because loading a file is async (potentially slow I/O)
    // The path is relative to where the executable runs
    //
    // # Error Handling Note
    // In production, you'd handle the Result properly instead of expect
    // For a demo, expect is fine (panics with helpful message on error)
    println!("Loading terrain image...");
    let surface = Surface::load("../assets/Moon_LRO_LOLA_global_LDEM_1024_b.png").await;
    println!(
        "Loaded image: {}x{} pixels",
        surface.img_width, surface.img_height
    );

    // -------------------------------------------------------------------------
    // Store image dimensions (these are fixed, don't change)
    // -------------------------------------------------------------------------
    // We extract these for convenience - cleaner than surface.img_width each time
    let img_width = surface.img_width;
    let img_height = surface.img_height;

    // -------------------------------------------------------------------------
    // Create the particle swarm
    // -------------------------------------------------------------------------
    // Vectors are Rust's growable arrays (like ArrayList in Java, list in Python)
    // The ::<T> syntax specifies the type (type annotation)
    // We could write Vec::new() and let Rust infer the type
    
    // Create PUNTOS (100) particles using iterator and collect
    // (0..PUNTOS) creates a Range (iterator producing 0, 1, 2, ..., 99)
    // .map(|_| Particle::new(...)) transforms each number into a Particle
    // |_| is a closure (anonymous function), _ means we ignore the parameter
    // .collect() gathers all items into a collection (Vec here)
    let mut particles: Vec<Particle> = (0..particle::PUNTOS)
        .map(|_| Particle::new(img_width, img_height))
        .collect();

    println!("Created {} particles", particles.len());

    // -------------------------------------------------------------------------
    // Initialize global best tracking
    // -------------------------------------------------------------------------
    // Global best is a tuple: (x, y, fitness)
    // We start with fitness = -1.0 (worse than any valid fitness 0-255)
    let mut gbest: (f32, f32, f32) = (0.0, 0.0, -1.0);

    // Counters for statistics
    let mut evals = 0; // Total fitness evaluations
    let mut evals_to_best = 0; // Evaluations when best was found

    // ========================================================================
    // MAIN LOOP
    // ========================================================================
    // This is the "game loop" - runs continuously until quit
    // Each iteration is one frame (typically 60 frames per second)

    loop {
        // ---------------------------------------------------------------------
        // FRAME SETUP
        // ---------------------------------------------------------------------

        // Clear the screen to black
        // This erases the previous frame's content
        // If we don't clear, previous frames would show through
        clear_background(BLACK);

        // Get current screen dimensions
        // These might change if user resizes the window
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Calculate scale factors for coordinate conversion
        // scale_x = screen_width / image_width
        // This lets us convert image coords to screen coords
        let scale_x = screen_w / img_width;
        let scale_y = screen_h / img_height;

        // ---------------------------------------------------------------------
        // QUIT CHECK
        // ---------------------------------------------------------------------
        // Check if user wants to quit (Q key, Escape, or X button)
        // should_quit also draws the close button
        if should_quit(screen_w) {
            println!("Closing PSO application...");
            // Exit the program with success code (0)
            // std::process::exit terminates immediately
            std::process::exit(0);
        }

        // ---------------------------------------------------------------------
        // RENDERING PHASE
        // ---------------------------------------------------------------------
        // Draw everything in back-to-front order (painter's algorithm)

        // 1. Draw the terrain map (background layer)
        draw_scaled_texture(&surface.texture, screen_w, screen_h);

        // 2. Draw all particles (middle layer)
        // Iterate through particles and draw each one
        // The & borrows each particle immutably (read-only)
        for p in &particles {
            p.display(&surface, scale_x, scale_y);
        }

        // 3. Draw the global best marker (highlight layer)
        draw_global_best(gbest, scale_x, scale_y);

        // 4. Draw the info overlay (top layer - always visible)
        draw_info_overlay(
            gbest,
            evals_to_best,
            evals,
            screen_w,
            screen_h,
            img_width,
            img_height,
        );

        // ---------------------------------------------------------------------
        // UPDATE PHASE
        // ---------------------------------------------------------------------
        // Update particle positions and evaluate fitness

        // Iterate mutably through particles to update them
        // &mut gives us mutable references (can modify)
        for p in &mut particles {
            // Move the particle according to PSO rules
            // We pass the global best position to guide movement
            p.r#move(gbest.0, gbest.1, img_width, img_height);

            // Evaluate fitness and potentially update global best
            // We pass mutable references so eval can update them
            p.eval(
                &surface,
                &mut gbest,
                &mut evals,
                &mut evals_to_best,
            );
        }

        // ---------------------------------------------------------------------
        // FRAME END
        // ---------------------------------------------------------------------
        // Wait for the next frame
        // This is CRUCIAL - without it:
        // - The loop would run as fast as possible (100% CPU)
        // - The OS would think the program is frozen
        // - Events (input, window resize) wouldn't be processed
        //
        // .await yields control back to the async runtime
        // When the next frame is ready, execution resumes here
        next_frame().await;
    }
}

// ============================================================================
// ADDITIONAL NOTES FOR RUST BEGINNERS
// ============================================================================

/*
 * RUST OWNERSHIP SUMMARY
 * ----------------------
 * Rust's unique feature is its ownership system:
 * 
 * 1. Each value has exactly ONE owner
 * 2. When the owner goes out of scope, the value is dropped (freed)
 * 3. You can borrow references (&T for read, &mut T for write)
 * 4. Borrowing rules prevent data races at compile time
 *
 * MUTABILITY
 * ----------
 * By default, variables are immutable (cannot be changed).
 * Use 'mut' keyword to make them mutable:
 *   let x = 5;      // immutable
 *   let mut y = 5;  // mutable
 *   y = 10;         // OK
 *   x = 10;         // ERROR!
 *
 * TYPES
 * -----
 * Rust is statically typed (all types known at compile time).
 * Common types:
 *   i32, i64  - signed integers
 *   u32, u64  - unsigned integers (no negatives)
 *   usize     - pointer-sized unsigned (for indexing)
 *   f32, f64  - floating point numbers
 *   bool      - true or false
 *   char      - Unicode character
 *   (T, U)    - tuple
 *   [T; N]    - fixed-size array
 *   Vec<T>    - growable vector
 *   &T        - immutable reference
 *   &mut T    - mutable reference
 *   Option<T> - Some(value) or None
 *   Result<T, E> - Ok(value) or Err(error)
 *
 * EXPRESSIONS VS STATEMENTS
 * -------------------------
 * Rust is expression-oriented:
 *   - Statement: Does something, ends with semicolon, returns ()
 *   - Expression: Evaluates to a value, no semicolon
 * 
 * Example:
 *   let x = 5;              // statement
 *   let y = {               // block is an expression
 *       let a = 3;
 *       a + 1               // expression (no semicolon) - this is the value
 *   };                      // y = 4
 *
 * ERROR HANDLING
 * --------------
 * Rust doesn't have exceptions. Use:
 *   - Result<T, E> for recoverable errors
 *   - Option<T> for nullable values
 *   - panic! for unrecoverable errors
 *   - .unwrap() or .expect() for quick prototyping
 *
 * FURTHER READING
 * ---------------
 * - The Rust Book: https://doc.rust-lang.org/book/
 * - Rust by Example: https://doc.rust-lang.org/rust-by-example/
 * - Macroquad docs: https://docs.rs/macroquad/
 */
