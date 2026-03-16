// === DOMINIO ===
pub const X_MIN: f32 = -3.0;
pub const X_MAX: f32 = 7.0;
pub const Y_MIN: f32 = -3.0;
pub const Y_MAX: f32 = 7.0;
 
// === PARÁMETROS PSO ===
pub const N_PARTICULAS: usize = 3000;
pub const V_MAX: f32 = 0.03;
pub const W: f32 = 3.0;   // inercia
pub const C1: f32 = 20.0;  // componente cognitivo
pub const C2: f32 = 50.0;  // componente social
pub const RUIDO: f32 = 0.0;
 
// === VISUALIZACIÓN ===
pub const ANCHO: u32 = 800;
pub const ALTO: u32 = 800;