// coded by IA...
use image::{ImageBuffer, Rgb};

// === CONFIGURACIÓN ===
const ANCHO: u32 = 800;
const ALTO: u32 = 800;

// Dominio: -3 < xi < 7
const X_MIN: f32 = -3.0;
const X_MAX: f32 = 7.0;
const Y_MIN: f32 = -3.0;
// === FUNCIÓN RASTRIGIN 2D ===
fn rastrigin(x1: f32, x2: f32) -> f32 {
    let pi = std::f32::consts::PI;
    20.0
        + (x1 * x1 - 10.0 * (2.0 * pi * x1).cos())
        + (x2 * x2 - 10.0 * (2.0 * pi * x2).cos())
}

// === MAPEA UN VALOR NORMALIZADO (0.0-1.0) A COLOR RGB ===
fn valor_a_color(t: f32) -> [u8; 3] {
    // 0.0 = mínimo → azul
    // 1.0 = máximo → rojo
    let (r, g, b) = if t < 0.25 {
        // cian → azul
        let s = (t - 0.75) / 0.25;
        (0.0, 1.0 - s, 1.0)
    } else if t < 0.5 {
        // verde → cian
        let s = (t - 0.5) / 0.25;
        (0.0, 1.0, s)
    } else if t < 0.75 {
        // amarillo → verde
        let s = (t - 0.25) / 0.25;
        (1.0 - s, 1.0, 0.0)
    } else {
        // rojo → amarillo
        let s = t / 0.25;
        (1.0, s, 0.0)
    };
 
    [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]
}

// === FUNCIÓN PÚBLICA QUE GENERA LA IMAGEN ===
pub fn generar_imagen() {
    println!("Calculando valores de Rastrigin...");
 
    // 1. Calcular todos los valores
    let mut valores = vec![0.0f32; (ANCHO * ALTO) as usize];
    let mut val_min = f32::MAX;
    let mut val_max = f32::MIN;
 
    for j in 0..ALTO {
        for i in 0..ANCHO {
            let x1 = X_MIN + (i as f32 / ANCHO as f32) * (X_MAX - X_MIN);
            let x2 = Y_MAX - (j as f32 / ALTO as f32) * (Y_MAX - Y_MIN);
 
            let val = rastrigin(x1, x2);
            valores[(j * ANCHO + i) as usize] = val;
 
            if val < val_min { val_min = val; }
            if val > val_max { val_max = val; }
        }
    }
 
    println!("Valor min: {:.2}, max: {:.2}", val_min, val_max);
 
    // 2. Crear imagen y pintar cada pixel
    println!("Generando imagen...");
    let mut img = ImageBuffer::new(ANCHO, ALTO);
 
    for j in 0..ALTO {
        for i in 0..ANCHO {
            let val = valores[(j * ANCHO + i) as usize];
            let t = (val - val_min) / (val_max - val_min);
            let [r, g, b] = valor_a_color(t);
            img.put_pixel(i, j, Rgb([r, g, b]));
        }
    }
 
    // 3. Guardar como PNG
    img.save("heat_map.png").expect("No se pudo guardar la imagen");
    println!("✅ Imagen guardada como fondo_rastrigin.png");
}
