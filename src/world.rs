use crate::{asteroids::Asteroid, player::Player};
use macroquad::{prelude::*, rand::gen_range};

#[derive(Default)]
pub struct World {
    player: Player,
    asteroids: Vec<Asteroid>,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        self.player.update();
        // println!("{}", self.asteroids.len());
        self.asteroids.retain(|asteroid| asteroid.is_visible());
        for asteroid in &mut self.asteroids {
            asteroid.update();
        }
        if self.asteroids.len() < 5 {
            self.asteroids.push(Asteroid::new());
            println!("Added {}", get_time());
        }
    }

    pub fn draw(&self) {
        self.player.draw();
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
    }
}
