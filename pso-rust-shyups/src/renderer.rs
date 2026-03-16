use macroquad::prelude::*;
use crate::pso::swarm::Swarm;
use crate::utils::{dx_a_px, dy_a_py};
 
pub fn dibujar(fondo: &Texture2D, swarm: &Swarm) {
    clear_background(BLACK);
    // === DIBUJAR FONDO (mapa de calor) ===
    draw_texture(fondo, 0.0, 0.0, WHITE);
    // === DIBUJAR PARTÍCULAS ===
    for p in swarm.particulas.iter() {
        let px = dx_a_px(p.x);
        let py = dy_a_py(p.y);
        draw_circle(px, py, 4.0, WHITE);
    }
    // === HUD - CONTADOR E INFO ===
    let fondo_hud = Color::new(0.0, 0.0, 0.0, 0.6);
    draw_rectangle(8.0, 8.0, 280.0, 55.0, fondo_hud);
    draw_text(
        &format!("Iteracion: {}", swarm.iteracion),
        15.0, 30.0, 22.0, WHITE
    );
    draw_text(
        &format!("Mejor fitness: {:.6}", swarm.mejor_global_fitness),
        15.0, 55.0, 22.0, WHITE
    );
}