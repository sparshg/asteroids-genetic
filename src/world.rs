use crate::{
    asteroids::{Asteroid, AsteroidSize},
    nn::NN,
    player::Player,
};
use macroquad::{prelude::*, rand::gen_range};

#[derive(Default)]
pub struct World {
    player: Player,
    asteroids: Vec<Asteroid>,
    pub score: u32,
    pub over: bool,
    max_asteroids: usize,
    pub fitness: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            max_asteroids: 28,
            ..Default::default()
        }
    }

    pub fn simulate(brain: Option<NN>) -> Self {
        Self {
            player: Player::simulate(brain),
            max_asteroids: 28,
            asteroids: vec![Asteroid::new_to(vec2(0., 0.), 1.5, AsteroidSize::Large)],
            ..Default::default()
        }
    }

    pub fn set_best(&mut self) {
        self.player.color = Some(RED);
    }

    pub fn see_brain(&self) -> &NN {
        self.player.brain.as_ref().unwrap()
    }

    // fn calc_fitness(&mut self) {
    // println!(
    //     "{} {} {}",
    //     self.score as f32,
    //     self.player.lifespan as f32 * 0.001,
    //     if self.player.shots > 0 {
    //         self.score as f32 / self.player.shots as f32 * 5.
    //     } else {
    //         0.
    //     }
    // );
    // }

    pub fn update(&mut self) {
        self.player.update();
        // if self.player.lifespan > 150 {
        //     self.fitness = 1.
        //         / ((self.player.pos * vec2(2. / screen_width(), 2. / screen_height()))
        //             .distance_squared(vec2(0., -1.))
        //             + self.player.vel.length_squared()
        //                 * self.player.vel.length_squared()
        //                 * 0.00006830134554
        //             + 1.);
        //     self.over = true;
        // }
        let mut to_add: Vec<Asteroid> = Vec::new();
        for asteroid in &mut self.asteroids {
            asteroid.update();
            if self.player.check_player_collision(asteroid) {
                self.over = true;
                self.fitness = (self.score as f32).powf(2.) * 0.01;
                // * (self.score as f32 / self.player.shots as f32)
                // * (self.score as f32 / self.player.shots as f32)
                // + self.player.lifespan as f32 * 0.01;
                // self.fitness = self.player.lifespan as f32 * self.player.lifespan as f32 * 0.001;

                // println!("{} {} {}", self.score, self.player.lifespan, self.fitness);
            }
            if self.player.check_bullet_collisions(asteroid) {
                self.score += 1;
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
        }
        self.asteroids.append(&mut to_add);
        self.asteroids.retain(|asteroid| asteroid.alive);
        // if self.asteroids.iter().fold(0, |acc, x| {
        //     acc + match x.size {
        //         AsteroidSize::Large => 4,
        //         AsteroidSize::Medium => 2,
        //         AsteroidSize::Small => 1,
        //     }
        // }) < self.max_asteroids
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
        // draw_text(
        //     &format!("Score {}", self.score),
        //     20. - screen_width() * 0.5,
        //     30. - screen_height() * 0.5,
        //     32.,
        //     WHITE,
        // );
    }
}
