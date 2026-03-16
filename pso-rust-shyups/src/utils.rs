use crate::pso::params::*;

// FUNCIÓN RASTRIGIN 2D
pub fn rastrigin(x1: f32, x2: f32) -> f32 {
    let pi = std::f32::consts::PI;
    20.0
        + (x1 * x1 - 10.0 * (2.0 * pi * x1).cos())
        + (x2 * x2 - 10.0 * (2.0 * pi * x2).cos())
}
 
// Dominio → Pixel
pub fn dx_a_px(x: f32) -> f32 {
    (x - X_MIN) / (X_MAX - X_MIN) * ANCHO as f32
}
pub fn dy_a_py(y: f32) -> f32 {
    (Y_MAX - y) / (Y_MAX - Y_MIN) * ALTO as f32
}