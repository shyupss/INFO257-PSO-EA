use macroquad::prelude::rand;
use crate::pso::params::*;
use crate::utils::rastrigin;
 
pub struct Particula {
    // posición actual en el dominio
    pub x: f32,
    pub y: f32,
    // velocidad actual
    pub vx: f32,
    pub vy: f32,
    // mejor posición personal encontrada
    pub mejor_x: f32,
    pub mejor_y: f32,
    pub mejor_fitness: f32,
}
 
impl Particula {
    pub fn nueva_random() -> Self {
        let x = rand::gen_range(X_MIN, X_MAX);
        let y = rand::gen_range(Y_MIN, Y_MAX);
        let vx = rand::gen_range(-V_MAX, V_MAX);
        let vy = rand::gen_range(-V_MAX, V_MAX);
        let fitness_inicial = rastrigin(x, y);
 
        Particula {
            x, y,
            vx, vy,
            mejor_x: x,
            mejor_y: y,
            mejor_fitness: fitness_inicial,
        }
    }
}