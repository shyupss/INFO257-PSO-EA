use macroquad::prelude::rand;
use crate::pso::params::*;
use crate::pso::particles::Particula;
use crate::utils::rastrigin;

pub struct Swarm {
    pub particulas: Vec<Particula>,
    pub mejor_global_x: f32,
    pub mejor_global_y: f32,
    pub mejor_global_fitness: f32,
    pub iteracion: usize,
}

impl Swarm {
    pub fn inicializar() -> Self {
        let particulas: Vec<Particula> = (0..N_PARTICULAS)
            .map(|_| Particula::nueva_random())
            .collect();
 
        // Encontrar el mejor global inicial
        let mejor_idx = particulas
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.mejor_fitness.partial_cmp(&b.mejor_fitness).unwrap())
            .map(|(i, _)| i)
            .unwrap();

        let mejor_global_x = particulas[mejor_idx].x;
        let mejor_global_y = particulas[mejor_idx].y;
        let mejor_global_fitness = particulas[mejor_idx].mejor_fitness;
 
        println!("Swarm inicializado:");
        println!("  Partículas: {}", N_PARTICULAS);
        println!("  Mejor fitness inicial: {:.4}", mejor_global_fitness);
        println!("  Mejor posición inicial: ({:.4}, {:.4})", mejor_global_x, mejor_global_y);

        Swarm {
            particulas,
            mejor_global_x,
            mejor_global_y,
            mejor_global_fitness,
            iteracion: 0,
        }
    }

    pub fn actualizar(&mut self) {
        let gbest_x = self.mejor_global_x;
        let gbest_y = self.mejor_global_y;
 
        for p in self.particulas.iter_mut() {
            // números aleatorios independientes para cada componente
            let r1 = rand::gen_range(0.0f32, 1.0);
            let r2 = rand::gen_range(0.0f32, 1.0);
            // === ACTUALIZAR VELOCIDAD (método combinado)===
            p.vx = W * p.vx
                + C1 * r1 * (p.mejor_x - p.x)
                + C2 * r2 * (gbest_x - p.x);
            p.vy = W * p.vy
                + C1 * r1 * (p.mejor_y - p.y)
                + C2 * r2 * (gbest_y - p.y);
            // un poco de ruido para evitar que se congelen
            p.vx += rand::gen_range(-RUIDO, RUIDO);
            p.vy += rand::gen_range(-RUIDO, RUIDO);
            // normalizamos dirección
            let modulo = (p.vx * p.vx + p.vy * p.vy).sqrt();
            if modulo > V_MAX {
                p.vx = p.vx / modulo * V_MAX;
                p.vy = p.vy / modulo * V_MAX;
            }
            // === ACTUALIZAR POSICIÓN ===
            p.x += p.vx;
            p.y += p.vy;
            // mantener dentro del dominio (rebote en los bordes)
            if p.x < X_MIN || p.x > X_MAX {
                p.vx = -p.vx;
                p.x = p.x.clamp(X_MIN, X_MAX);
            }
            if p.y < Y_MIN || p.y > Y_MAX {
                p.vy = -p.vy;
                p.y = p.y.clamp(Y_MIN, Y_MAX);
            }
            // === ACTUALIZAR MEJOR PERSONAL ===
            let fitness_actual = rastrigin(p.x, p.y);
            if fitness_actual < p.mejor_fitness {
                p.mejor_x = p.x;
                p.mejor_y = p.y;
                p.mejor_fitness = fitness_actual;
            }
            // === ACTUALIZAR MEJOR GLOBAL ===
            if p.mejor_fitness < self.mejor_global_fitness {
                self.mejor_global_x = p.mejor_x;
                self.mejor_global_y = p.mejor_y;
                self.mejor_global_fitness = p.mejor_fitness;
            }
        }
        self.iteracion += 1;
    }
}