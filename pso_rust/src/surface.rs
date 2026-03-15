//! Surface module - handles image loading and fitness evaluation
//!
//! This module encapsulates all functionality related to the terrain/map image.
//! In Rust, modules are used to organize code into logical units.
//!
//! Key concepts covered:
//! - Structs (custom data types)
//! - Implementation blocks (impl)
//! - Ownership and borrowing
//! - async/await for asynchronous operations
//! - Option and Result types for error handling

// Import everything from macroquad's prelude
// The 'prelude' is a convention for commonly used items that are re-exported
// The :: syntax accesses nested modules/items (like folder paths)
use macroquad::prelude::*;

// ============================================================================
// CONSTANTS
// ============================================================================
// Constants are compile-time immutable values (cannot be changed at runtime)
// - 'const' keyword declares a constant
// - Must have explicit type annotation (no type inference)
// - Named in SCREAMING_SNAKE_CASE by convention
// - Inlined by the compiler wherever used (no memory address at runtime)

/// The Surface struct holds both the image data and the GPU texture.
///
/// # Rust Structs
/// Structs are custom data types that group related data together.
/// Think of them as "objects" that only hold data (no methods here).
///
/// # Visibility (pub)
/// - 'pub' makes items public (accessible from other modules)
/// - Without 'pub', items are private by default (only visible in this module)
/// - We make the struct public so other modules can use it
pub struct Surface {
	/// The image data stored in CPU memory.
	/// 
	/// # Ownership
	/// In Rust, 'img' owns the Image data. When Surface is dropped (goes out of scope),
	/// the Image is automatically freed. No garbage collector needed!
	/// 
	/// # Why not a reference (&Image)?
	/// References borrow data temporarily. Since Surface needs to own the image
	/// for the entire program lifetime, we use owned data, not references.
	pub img: Image,

	/// The GPU texture for rendering.
	/// 
	/// # Texture2D vs Image
	/// - Image: Raw pixel data in CPU memory (can read/write pixels)
	/// - Texture2D: GPU-optimized format for drawing (fast rendering)
	/// We need both: Image for pixel access, Texture for drawing.
	pub texture: Texture2D,

	/// Original image width in pixels.
	/// 
	/// # Type Choice: f32 vs u32
	/// Image dimensions are naturally integers (u32), but we store as f32 because:
	/// 1. We do lots of floating-point math with these values
	/// 2. Avoids constant casting (as f32) throughout the code
	/// 3. Slightly more convenient for our use case
	pub img_width: f32,

	/// Original image height in pixels
	pub img_height: f32,
}

// ============================================================================
// IMPLEMENTATION BLOCK
// ============================================================================
// 'impl' blocks add functionality to structs
// This is where methods and associated functions are defined
// Think of impl as "methods for this struct"

impl Surface {
	/// Load an image file and create a Surface.
	///
	/// # Associated Function vs Method
	/// - Associated function: Called on the type (Surface::load(...))
	/// - Method: Called on an instance (surface.get_fitness(...))
	/// - 'Self' in the return type refers to the struct type (Surface)
	///
	/// # async fn
	/// - Async functions return a Future (a promise of a value)
	/// - The '.await' keyword pauses until the Future completes
	/// - Required because loading files is I/O (potentially slow)
	///
	/// # Error Handling: Result and expect
	/// - load_image() returns Result<Image, Error>
	/// - Result is an enum: Ok(value) or Err(error)
	/// - .expect("msg") unwraps Ok or panics with message on Err
	/// - For production code, prefer proper error handling with ? operator
	pub async fn load(path: &str) -> Self {
		// load_image is an async function from macroquad
		// The .await pauses this function until loading completes
		// During await, other code can run (non-blocking)
		let img = load_image(path)
			.await  // Wait for async operation to complete
			.expect("Cannot open image file"); // Panic with message if loading fails

		// Store original dimensions
		// 'as f32' is a type cast (converts u16 to f32)
		// Rust requires explicit casts for potentially lossy conversions
		let img_width = img.width as f32;
		let img_height = img.height as f32;

		// Create a GPU texture from the image
		// The & symbol creates a reference (borrow)
		// We borrow img because Texture2D::from_image doesn't need ownership
		let texture = Texture2D::from_image(&img);

		// Return a new Surface instance
		// This is a struct initialization using field init shorthand
		// When variable name matches field name, just write the name once
		// Equivalent to: Surface { img: img, texture: texture, ... }
		Self {
			img,
			texture,
			img_width,
			img_height,
			}
	}

	/// Get the "fitness" value at a pixel position.
	/// 
	/// In PSO terms, "fitness" is how good a position is (brighter = higher).
	/// We use the red channel as the fitness value.
	///
	/// # Parameters
	/// - &self: Borrows the Surface immutably (read-only access)
	///   This is a method because it takes 'self'
	/// - x, y: Position in image coordinates (floating point for smooth movement)
	///
	/// # Return Type
	/// Returns f32 (the fitness value scaled 0-255)
	///
	/// # The Borrowing Rules
	/// - &self: Immutable borrow (can have multiple readers)
	/// - &mut self: Mutable borrow (exclusive access, can modify)
	/// - You cannot have both at the same time (prevents data races!)
	pub fn get_fitness(&self, x: f32, y: f32) -> f32 {
		// Clamp coordinates to valid range
		// clamp(min, max) ensures value stays within bounds
		// - Prevents array out-of-bounds errors
		// - If particle goes outside image, we use edge pixel
		let ix = x.clamp(0.0, self.img_width - 1.0) as u32;
		let iy = y.clamp(0.0, self.img_height - 1.0) as u32;

		// Get pixel color at (ix, iy)
		// Macroquad's get_pixel returns a Color struct
		// Color has r, g, b, a fields (each 0.0 to 1.0, normalized floats)
		let pixel = self.img.get_pixel(ix, iy);

		// Return red channel scaled to 0-255 range
		// This matches the original Processing sketch behavior
		// Processing used 0-255 for colors; we scale back for compatibility
		pixel.r * 255.0
	}
}

// ============================================================================
// UNIT TESTS
// ============================================================================
// Rust has built-in testing support
// Tests are marked with #[test] attribute
// Run with: cargo test
// Tests are compiled conditionally and not included in release builds

#[cfg(test)] // This module only exists during testing
mod tests {
	// Import everything from parent module
	use super::*;

	#[test]
	fn test_clamp_behavior() {
		// This test demonstrates clamp behavior
		// In real tests, you'd test actual Surface methods
		assert_eq!(5.0_f32.clamp(0.0, 10.0), 5.0);  // In range
		assert_eq!((-5.0_f32).clamp(0.0, 10.0), 0.0); // Below min
		assert_eq!(15.0_f32.clamp(0.0, 10.0), 10.0); // Above max
	}
}
