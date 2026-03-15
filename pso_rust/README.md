# PSO Rust - Particle Swarm Optimization Visualization

A Rust implementation of Particle Swarm Optimization (PSO) with visual output using Macroquad.

## Project Structure

```
src/
├── main.rs      # Entry point, main game loop, module organization
├── surface.rs   # Image loading and fitness evaluation
├── particle.rs  # PSO particle implementation (position, velocity, best tracking)
└── drawing.rs   # UI drawing functions (close button, info overlay)
```

## How to Run

```bash
cd pso_rust

# Debug mode (fast compile, slower runtime)
cargo run

# Release mode (slower compile, much faster runtime)
cargo run --release
```

## Controls

| Action | How |
|--------|-----|
| Quit | Press `Q` key |
| Quit | Press `Escape` key |
| Quit | Click the X button (top-right corner) |

## Key Rust Concepts Explained

### Module System
- `mod surface;` - Declares a module (links to `surface.rs`)
- `use particle::Particle;` - Imports `Particle` from the `particle` module
- `pub` - Makes items public (accessible from other modules)

### Ownership & Borrowing
- `let surface = Surface::load(...)` - `surface` owns the data
- `&surface` - Immutable borrow (read-only access)
- `&mut particles` - Mutable borrow (can modify)

### Structs & impl
```rust
struct Particle {
    x: f32,  // field
}

impl Particle {
    fn new() -> Self { ... }        // Associated function (constructor)
    fn display(&self) { ... }        // Method (immutable borrow)
    fn r#move(&mut self) { ... }     // Method (mutable borrow)
}
```

### async/await
```rust
async fn main() {
    let surface = Surface::load(...).await;  // Wait for async operation
    loop {
        next_frame().await;  // Wait for next frame
    }
}
```

### Common Types
- `f32` - 32-bit float (standard for graphics)
- `usize` - Pointer-sized unsigned integer (for array indexing)
- `Vec<T>` - Growable array
- `(f32, f32, f32)` - Tuple of three floats
- `Option<T>` - Some(value) or None
- `Result<T, E>` - Ok(value) or Err(error)

## PSO Algorithm

Particle Swarm Optimization mimics the social behavior of bird flocks:

1. **Inertia**: Particles keep moving in their current direction
2. **Cognitive**: Particles remember their best position found
3. **Social**: Particles are attracted to the global best position

The velocity update formula:
```
v_new = W * v_old + r1 * (personal_best - position) + r2 * (global_best - position)
```

Where:
- `W` = Inertia weight (momentum)
- `r1, r2` = Random factors (exploration)

## Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust tutorial
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by examples
- [Macroquad Documentation](https://docs.rs/macroquad/) - Game library docs
