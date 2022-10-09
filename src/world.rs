use crate::{
    asteroids::{Asteroid, AsteroidSize},
    player::Player,
};
use macroquad::{prelude::*, rand::gen_range};

#[derive(Default)]
pub struct World {
    player: Player,
    asteroids: Vec<Asteroid>,
    pub score: i32,
    pub over: bool,
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
        let mut to_add: Vec<Asteroid> = Vec::new();
        for asteroid in &mut self.asteroids {
            asteroid.update();
            if self.player.check_player_collision(asteroid) {
                self.over = true;
            }
            if self.player.check_bullet_collisions(asteroid) {
                self.score += 1;
                match asteroid.size {
                    AsteroidSize::Large => {
                        let rand = vec2(gen_range(-50., 50.), gen_range(-50., 50.));
                        to_add.push(Asteroid::new_from(
                            asteroid.pos,
                            asteroid.vel + rand,
                            AsteroidSize::Medium,
                        ));
                        to_add.push(Asteroid::new_from(
                            asteroid.pos,
                            asteroid.vel - rand,
                            AsteroidSize::Medium,
                        ));
                    }
                    AsteroidSize::Medium => {
                        let rand = vec2(gen_range(-30., 30.), gen_range(-30., 30.));
                        to_add.push(Asteroid::new_from(
                            asteroid.pos,
                            asteroid.vel + rand,
                            AsteroidSize::Small,
                        ));
                        to_add.push(Asteroid::new_from(
                            asteroid.pos,
                            asteroid.vel - rand,
                            AsteroidSize::Small,
                        ));
                    }
                    _ => {}
                }
            }
        }
        self.asteroids.append(&mut to_add);
        self.asteroids.retain(|asteroid| asteroid.alive);
        if self.asteroids.iter().fold(0, |acc, x| {
            acc + match x.size {
                AsteroidSize::Large => 4,
                AsteroidSize::Medium => 2,
                AsteroidSize::Small => 1,
            }
        }) < 20
        {
            self.asteroids.push(Asteroid::new(AsteroidSize::Large));
        }
    }

    pub fn draw(&self) {
        self.player.draw();
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
        draw_text(
            &format!("Score {}", self.score),
            20. - screen_width() * 0.5,
            30. - screen_height() * 0.5,
            32.,
            WHITE,
        );
    }
}
