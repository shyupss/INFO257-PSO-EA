//! Drawing module - UI rendering functions
//!
//! This module contains helper functions for drawing UI elements.
//! Separating drawing code from logic improves code organization.
//!
//! Key concepts covered:
//! - Module-level documentation
//! - Function organization and parameters
//! - Boolean expressions and logic
//! - Working with mouse input

use macroquad::prelude::*;

// Import the D constant from the particle module
// We need this for drawing the global best marker with the same size as particles
// 'crate::particle::D' is an absolute path from the crate root
use crate::particle::D;

// ============================================================================
// UI CONSTANTS
// ============================================================================

/// Size of the close button in pixels
const BUTTON_SIZE: f32 = 30.0;

/// Margin from screen edge for the close button
const BUTTON_MARGIN: f32 = 10.0;

/// Padding inside the button for the X symbol
const BUTTON_PADDING: f32 = 8.0;

/// Thickness of lines in the X symbol
const LINE_THICKNESS: f32 = 3.0;

// ============================================================================
// DRAWING FUNCTIONS
// ============================================================================

/// Draw a close button (X) in the top-right corner.
///
/// # Return Value
/// Returns true if the button was clicked this frame.
///
/// # The Pattern: Draw and Return State
/// This function both draws the button AND returns whether it was clicked.
/// This pattern is common in immediate mode GUIs:
/// - No persistent state stored between frames
/// - Each frame, we check and draw fresh
/// - Return value tells caller what happened
///
/// # Screen Coordinates
/// The button is positioned relative to screen_width():
/// - button_x = right edge - button_size - margin
/// - This makes it responsive to window resizing
pub fn draw_close_button(screen_width: f32) -> bool {
	// Calculate button position (top-right corner)
	// This is done each frame because screen size might change
	let button_x = screen_width - BUTTON_SIZE - BUTTON_MARGIN;
	let button_y = BUTTON_MARGIN;

	// Draw button background (dark rectangle)
	// draw_rectangle takes: x, y, width, height, color
	draw_rectangle(button_x, button_y, BUTTON_SIZE, BUTTON_SIZE, DARKGRAY);

	// Draw the X symbol using two diagonal lines
	// Line 1: top-left to bottom-right corner of the X
	draw_line(
		button_x + BUTTON_PADDING,          // Start X
		button_y + BUTTON_PADDING,          // Start Y
		button_x + BUTTON_SIZE - BUTTON_PADDING, // End X
		button_y + BUTTON_SIZE - BUTTON_PADDING, // End Y
		LINE_THICKNESS,
		RED,
	);

	// Line 2: top-right to bottom-left corner of the X
	draw_line(
		button_x + BUTTON_SIZE - BUTTON_PADDING,
		button_y + BUTTON_PADDING,
		button_x + BUTTON_PADDING,
		button_y + BUTTON_SIZE - BUTTON_PADDING,
		LINE_THICKNESS,
		RED,
	);

	// Get current mouse position
	// mouse_position() returns (f32, f32) - a tuple
	// We destructure it into two variables
	let (mouse_x, mouse_y) = mouse_position();

	// Check if mouse is hovering over the button
	// 
	// # Compound Boolean Expression
	// Multiple conditions combined with && (logical AND)
	// All must be true for the entire expression to be true
	//
	// # Range Checking
	// We check if mouse is within the rectangular bounds:
	// - X: button_x <= mouse_x <= button_x + BUTTON_SIZE
	// - Y: button_y <= mouse_y <= button_y + BUTTON_SIZE
	let is_hovering = mouse_x >= button_x
		&& mouse_x <= button_x + BUTTON_SIZE
		&& mouse_y >= button_y
		&& mouse_y <= button_y + BUTTON_SIZE;

	// Visual feedback: draw yellow border when hovering
	// This helps users know the button is clickable
	if is_hovering {
		// draw_rectangle_lines draws just the outline
		// Parameters: x, y, width, height, line_thickness, color
		draw_rectangle_lines(
			button_x,
			button_y,
			BUTTON_SIZE,
			BUTTON_SIZE,
			2.0,
			YELLOW,
		);
	}

	// Return true if hovering AND mouse button was just pressed
	// 
	// # Short-circuit Evaluation
	// If is_hovering is false, the && operator won't evaluate the right side
	// This is an optimization AND prevents checking clicks when not hovering
	//
	// # is_mouse_button_pressed vs is_mouse_button_down
	// - _pressed: True for ONE frame when button is first pressed
	// - _down: True for ALL frames while button is held
	// We use _pressed so we don't process the same click multiple times
	is_hovering && is_mouse_button_pressed(MouseButton::Left)
}

/// Check if the user wants to quit the application.
///
/// # Multiple Exit Conditions
/// This function checks three different ways to quit:
/// 1. Pressing the Q key
/// 2. Pressing the Escape key
/// 3. Clicking the close button
///
/// # Early Returns
/// Each condition uses 'return true' to exit the function immediately.
/// This is cleaner than nested if-else statements.
/// The final 'false' is only reached if no quit condition was met.
///
/// # KeyCode Enum
/// KeyCode is an enum (enumeration) - a type with predefined values
/// Common keys: A-Z, Escape, Space, Enter, Arrow keys (Up, Down, Left, Right)
pub fn should_quit(screen_width: f32) -> bool {
	// Check for Q key press
	// is_key_pressed() returns true for ONE frame when key is pressed
	if is_key_pressed(KeyCode::Q) {
		println!("Quit requested via Q key");
		return true; // Early return - exit function immediately
	}

	// Check for Escape key press (common convention for closing apps)
	if is_key_pressed(KeyCode::Escape) {
		println!("Quit requested via Escape key");
		return true;
	}

	// Check for close button click
	// This also draws the button each frame
	if draw_close_button(screen_width) {
		println!("Quit requested via close button");
		return true;
	}

	// No quit condition was met
	false
}

/// Draw the information overlay text.
///
/// # Formatting Strings
/// The format! macro creates a String from a format string:
/// - {} is a placeholder that gets replaced by a value
/// - {:.2} means format as float with 2 decimal places
/// - {:.0} means format as float with 0 decimal places (whole number)
///
/// # Parameters
/// - gbest: Tuple of (x, y, fitness) for global best
/// - evals_to_best: How many evaluations it took to find the best
/// - evals: Total number of evaluations
/// - screen dimensions: For display
/// - image dimensions: For display
pub fn draw_info_overlay(
	gbest: (f32, f32, f32),
	evals_to_best: i32,
	evals: i32,
	screen_w: f32,
	screen_h: f32,
	img_width: f32,
	img_height: f32,
) {
	// Create the info string
	let info_text = format!(
		"Best fitness: {:.2}\n\
		 Evals to best: {}\n\
		 Evals: {}\n\
		 Screen: {:.0}x{:.0}\n\
		 Image: {:.0}x{:.0}\n\n\
		 Press Q or ESC to quit\n\
		 Or click X button (top-right)",
		 gbest.2,         // fitness with 2 decimal places
		 evals_to_best,   // integer
		 evals,           // integer
		 screen_w,        // with 0 decimals
		 screen_h,
		 img_width,
		 img_height,
	);

	// Draw the text on screen
	// Parameters: text, x, y, font_size, color
	draw_text(&info_text, 10.0, 20.0, 18.0, GREEN);
}

/// Draw the global best position marker.
///
/// # Why Draw Best Separately?
/// The global best is a special point that all particles are attracted to.
/// We draw it in a different color (BLUE) to distinguish it from particles.
///
/// # Parameters
/// - gbest: (x, y, fitness) in image coordinates
/// - scale_x, scale_y: For converting to screen coordinates
pub fn draw_global_best(gbest: (f32, f32, f32), scale_x: f32, scale_y: f32) {
	// Convert image coordinates to screen coordinates
	let screen_x = gbest.0 * scale_x;
	let screen_y = gbest.1 * scale_y;

	// Scale the circle radius to match the screen size
	// Using min() ensures the circle doesn't look stretched
	let scaled_radius = (D / 2.0) * scale_x.min(scale_y);

	// Draw the global best marker as a blue circle
	draw_circle(screen_x, screen_y, scaled_radius, BLUE);
}

/// Draw the terrain texture scaled to fill the screen.
///
/// # Texture Scaling
/// We use DrawTextureParams to scale the texture to screen size.
/// Without this, the texture would be drawn at its original size.
///
/// # Option Type
/// dest_size is an Option<Vec2>:
/// - Some(value): We have a value
/// - None: No value (use original size)
/// Option is Rust's way of handling nullable values safely.
pub fn draw_scaled_texture(texture: &Texture2D, screen_width: f32, screen_height: f32) {
	// Create drawing parameters
	// The ..Default::default() syntax fills remaining fields with defaults
	// This is called "struct update syntax" (like spread operator in JS)
	let params = DrawTextureParams {
		// Set the destination size to fill the screen
		dest_size: Some(vec2(screen_width, screen_height)),
		// All other fields use their default values
		..Default::default()
	};

	// Draw the texture with custom parameters
	// Parameters: texture, x, y, color, params
	draw_texture_ex(texture, 0.0, 0.0, WHITE, params);
}
