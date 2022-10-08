mod asteroids;
mod player;
mod utils;
mod world;

use asteroids::Asteroid;
use macroquad::prelude::*;
use player::Player;
use world::World;

#[macroquad::main("Camera")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    let cam = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    set_camera(&cam);
    let mut world = World::new();

    loop {
        clear_background(BLACK);
        world.update();
        world.draw();

        next_frame().await
    }
}
