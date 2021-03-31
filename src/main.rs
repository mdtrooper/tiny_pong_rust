extern crate piston_window;
use piston_window::*;

const DIM_VENTANA:[u32;2] = [800, 600];

struct Raqueta {
    pub color: [f32; 4],
    pub pos: [f64; 4]
}

impl Raqueta {
    fn new(x: f64, y: f64) -> Self {
        Raqueta {
            color: [1.0, 1.0, 1.0, 1.0],
            pos: [x, y, 20.0, 100.0]
        }
    }
}

fn main() {
    let mut ventana: PistonWindow = WindowSettings::new("Tiny Pong", (DIM_VENTANA[0], DIM_VENTANA[1]))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Error al lanzar el juego: {}", e) });
    
    let mut raqueta1 = Raqueta::new(0.0, 0.0);
    let mut raqueta2 = Raqueta::new(DIM_VENTANA[0] as f64 - 20.0, 0.0);
    
    while let Some(e) = ventana.next() {
        ventana.draw_2d(&e, |_c, g, _d| {
            clear([0.0, 0.0, 0.0, 1.0], g);
                                    //~ clear([1.0; 4], g); // Clear to white
            rectangle(raqueta1.color, raqueta1.pos, _c.transform, g);
            rectangle(raqueta2.color, raqueta2.pos, _c.transform, g);
        });
    }
}