mod heat_map;
mod pso;
mod utils;
mod renderer;

use macroquad::prelude::*;
use std::path::Path;
use macroquad::window::next_frame;
use pso::swarm::Swarm;

fn config() -> Conf {
    Conf {
        window_title: "PSO - Rastrigin".to_string(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    // generar mapa de calor
    if !Path::new("heat_map.png").exists() {
        // generar fondo
        heat_map::generar_imagen();
    }else {
        println!("Mapa de calor ya existe, cargando...");
    }
    // cargar fondo
    let fondo = macroquad::texture::load_texture("heat_map.png").await.unwrap();
    // inicializar swarm
    let mut swarm = Swarm::inicializar();
    // loop principal
    loop {
        swarm.actualizar();
        renderer::dibujar(&fondo, &swarm);
        // limitar a 60 FPS
        std::thread::sleep(std::time::Duration::from_millis(1000/60));
        next_frame().await;
    }
}