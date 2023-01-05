mod asteroids;
mod nn;
mod player;
mod population;
mod world;

use macroquad::prelude::*;
use population::Population;

#[macroquad::main("Camera")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    let cam = Camera2D {
        zoom: vec2(2. / screen_width(), -2. / screen_height()),
        ..Default::default()
    };
    set_camera(&cam);
    let mut pop = Population::new(100);
    let mut speedup = false;
    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::S) {
            speedup = !speedup;
        }
        if speedup {
            for _ in 0..1000 {
                pop.update();
            }
        } else {
            pop.update();
            pop.draw();
        }
        next_frame().await
    }
}
