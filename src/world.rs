use crate::{
    asteroids::{Asteroid, AsteroidSize},
    nn::NN,
    player::Player,
};
use macroquad::{prelude::*, rand::gen_range};

#[derive(Default)]
pub struct World {
    pub player: Player,
    asteroids: Vec<Asteroid>,
    pub score: f32,
    pub over: bool,
    pub fitness: f32,
}

impl World {
    pub fn simulate(brain: Option<NN>) -> Self {
        Self {
            player: Player::simulate(brain),
            score: 1.,
            asteroids: vec![
                Asteroid::new_to(vec2(0., 0.), 1.5, AsteroidSize::Large),
                Asteroid::new(AsteroidSize::Large),
                Asteroid::new(AsteroidSize::Large),
                Asteroid::new(AsteroidSize::Large),
                Asteroid::new(AsteroidSize::Large),
            ],
            ..Default::default()
        }
    }

    pub fn set_best(&mut self) {
        self.player.color = Some(RED);
    }

    pub fn see_brain(&self) -> &NN {
        self.player.brain.as_ref().unwrap()
    }

    pub fn export_brain(&self) {
        let json = self.player.brain.as_ref().unwrap().export();
        std::fs::create_dir_all("models").expect("Unable to create directory");
        std::fs::write("models/brain.json", json).expect("Unable to write file");
    }

    pub fn update(&mut self) {
        let mut to_add: Vec<Asteroid> = Vec::new();
        for asteroid in &mut self.asteroids {
            asteroid.update();
            if self.player.check_bullet_collisions(asteroid) {
                self.score += 1.;
                match asteroid.size {
                    AsteroidSize::Large => {
                        let rand = vec2(gen_range(-0.8, 0.8), gen_range(-0.8, 0.8));
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
                        let rand = vec2(gen_range(-0.6, 0.6), gen_range(-0.6, 0.6));
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
            if self.player.check_player_collision(asteroid) {
                self.over = true;
                self.fitness =
                    (self.score / self.player.shots as f32).powi(2) * self.player.lifespan as f32;
            }
        }
        self.player.update();
        self.asteroids.append(&mut to_add);
        self.asteroids.retain(|asteroid| asteroid.alive);
        // if self.asteroids.iter().fold(0, |acc, x| {
        //     acc + match x.size {
        //         AsteroidSize::Large => 4,
        //         AsteroidSize::Medium => 2,
        //         AsteroidSize::Small => 1,
        //     }
        // }) < self.max_asteroids
        //     || self.player.lifespan % 200 == 0
        // {
        if self.player.lifespan % 200 == 0 {
            self.asteroids
                .push(Asteroid::new_to(self.player.pos, 1.5, AsteroidSize::Large));
        }
    }

    pub fn draw(&self) {
        self.player.draw();
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
        draw_text(
            &format!(
                "{}",
                (self.score / self.player.shots as f32).powi(2) * self.player.lifespan as f32
            ),
            self.player.pos.x - 20.,
            self.player.pos.y - 20.,
            12.,
            WHITE,
        );
    }
}
