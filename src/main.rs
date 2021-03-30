extern crate piston_window;
use piston_window::*;

const DIM_VENTANA:[u32;2] = [800, 600]; 

fn main() {
    let mut ventana: PistonWindow = WindowSettings::new("Tiny Pong", (DIM_VENTANA[0], DIM_VENTANA[1]))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Error al lanzar el juego: {}", e) });
    
    while let Some(e) = ventana.next() {
        ventana.draw_2d(&e, |_c, g, _d| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });
    }
}