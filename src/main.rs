mod asteroids;
mod nn;
mod player;
mod population;
mod world;

use macroquad::prelude::*;
use nn::NN;
use world::World;

#[macroquad::main("Camera")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    let cam = Camera2D {
        zoom: vec2(2. / screen_width(), -2. / screen_height()),
        ..Default::default()
    };
    set_camera(&cam);
    // let mut world = World::new();
    let mut nn = NN::new(vec![1, 2, 1]);

    loop {
        // clear_background(BLACK);
        // if !world.over {
        //     world.update();
        // }
        // world.draw();
        next_frame().await
    }
}
