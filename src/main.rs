mod asteroids;
mod player;

use asteroids::Asteroid;
use macroquad::prelude::*;
use player::Player;

#[macroquad::main("Camera")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    let cam = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    let mut asteroid = Asteroid::new();
    set_camera(&cam);
    loop {
        clear_background(BLACK);
        asteroid.update();
        asteroid.draw();
        next_frame().await
    }
}

pub fn rotate_vec(vec: Vec2, angle: f32) -> Vec2 {
    vec2(angle.cos(), angle.sin()).rotate(vec)
}
