extern crate piston_window;
use piston_window::*;

const DIM_VENTANA:[u32;2] = [800, 600];
const DIM_RAQUETA:[f64;2] = [20.0, 100.0];

const MAX_ACELERACION:u32 = 50;

#[derive(Debug)]
enum Movimiento {
    Ninguno,
    Arriba,
    Abajo,
}

#[derive(Debug)]
struct Raqueta {
    pub color: [f32; 4],
    pub pos: [f64; 4],
    pub acel: u32,
    pub prev_mov: Movimiento
}

impl Raqueta {
    fn new(x: f64, y: f64) -> Self {
        Raqueta {
            color: [1.0, 1.0, 1.0, 1.0],
            pos: [x, y, x + DIM_RAQUETA[0], y + DIM_RAQUETA[1]],
            acel: 1,
            prev_mov: Movimiento::Ninguno
        }
    }
    
    fn arriba(&mut self) {
        self.acel = match self.prev_mov {
            Movimiento::Arriba => self.acel + 1,
            _ => 1
        };
        self.acel = if self.acel > MAX_ACELERACION { MAX_ACELERACION} else { self.acel };
        self.prev_mov = Movimiento::Arriba;
        self.pos[1] -= self.acel as f64;
    } 

    fn abajo(&mut self) {
        self.acel = match self.prev_mov {
            Movimiento::Abajo => self.acel + 1,
            _ => 1
        };
        self.acel = if self.acel > MAX_ACELERACION { MAX_ACELERACION} else { self.acel};
        self.prev_mov = Movimiento::Abajo;
        self.pos[1] += self.acel as f64;
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
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Q => {
                    raqueta1.arriba();
                },
                Key::A => {
                    raqueta1.abajo();
                },
                Key::P => {
                    raqueta2.arriba();
                },
                Key::L => {
                    raqueta2.abajo();
                },
                _ => {println!("Pressed keyboard key '{:?}'", key);}
            }
        };
        
        println!("{:?}", raqueta1);
        
        ventana.draw_2d(&e, |_c, g, _d| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            
            rectangle(raqueta1.color, raqueta1.pos, _c.transform, g);
            rectangle(raqueta2.color, raqueta2.pos, _c.transform, g);
        });
    }
}