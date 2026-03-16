// === DOMINIO ===
pub const X_MIN: f32 = -3.0;
pub const X_MAX: f32 = 7.0;
pub const Y_MIN: f32 = -3.0;
pub const Y_MAX: f32 = 7.0;
 
// === PARÁMETROS PSO ===
pub const N_PARTICULAS: usize = 1000;
pub const V_MAX: f32 = 0.02;
pub const W: f32 = 0.99;   // inercia
pub const C1: f32 = 15.0;  // componente cognitivo
pub const C2: f32 = 40.0;  // componente social
pub const RUIDO: f32 = 0.05;
 
// === VISUALIZACIÓN ===
pub const ANCHO: u32 = 800;
pub const ALTO: u32 = 800;