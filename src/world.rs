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
    pub track: bool,
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

    pub fn track(&mut self, track: bool) {
        self.player.color = if track { Some(RED) } else { None };
        for asteroid in &mut self.asteroids {
            asteroid.color = if track {
                Color::new(1., 0., 0., 0.8)
            } else {
                Color::new(1., 1., 1., 0.2)
            };
        }
    }

    pub fn see_brain(&self) -> &NN {
        self.player.brain.as_ref().unwrap()
    }

    pub fn export_brain(&self, path: &str) {
        let json = self.player.brain.as_ref().unwrap().export();
        std::fs::write(path, json).expect("Unable to write file");
    }

    pub fn update(&mut self) {
        self.player.update();
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
            }
        }
        self.fitness =
            (self.score / self.player.shots as f32).powi(2) * self.player.lifespan as f32;
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

    pub fn draw(&self, debug: bool) {
        self.player.draw(debug);
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
        draw_text(
            &format!("{:.2}", self.fitness),
            self.player.pos.x - 22.,
            self.player.pos.y - 20.,
            12.,
            WHITE,
        );
    }

    pub fn draw_stats(&self, width: f32, height: f32) {
        draw_rectangle_lines(-width * 0.5, -height * 0.5, width, height, 2., WHITE);

        let scale = 2.5;
        let offset = vec2(-width * 0.3, -height * 0.1);
        let p1 = scale * vec2(0., -20.) + offset;
        let p2 = scale * vec2(-12.667, 18.) + offset;
        let p3 = scale * vec2(12.667, 18.) + offset;
        let p4 = scale * vec2(-10., 10.) + offset;
        let p5 = scale * vec2(10., 10.) + offset;
        let p6 = scale * vec2(0., 25.) + offset;
        let p7 = scale * vec2(-6., 10.) + offset;
        let p8 = scale * vec2(6., 10.) + offset;

        draw_line(p1.x, p1.y, p2.x, p2.y, 2., WHITE);
        draw_line(p1.x, p1.y, p3.x, p3.y, 2., WHITE);
        draw_line(p4.x, p4.y, p5.x, p5.y, 2., WHITE);
        if self.player.outputs[2] > 0. && (gen_range(0., 1.) < 0.4 || self.over) {
            draw_triangle_lines(p6, p7, p8, 2., WHITE);
        }
        let l1 = scale * vec2(30., 0.) + offset;
        let l2 = scale * vec2(25., -5.) + offset;
        let l3 = scale * vec2(25., 5.) + offset;
        if self.player.outputs[0] > 0. {
            draw_line(l1.x, l1.y, l2.x, l2.y, 2., WHITE);
            draw_line(l1.x, l1.y, l3.x, l3.y, 2., WHITE);
        }
        let l1 = -scale * vec2(30., 0.) + offset;
        let l2 = -scale * vec2(25., -5.) + offset;
        let l3 = -scale * vec2(25., 5.) + offset;
        if self.player.outputs[1] > 0. {
            draw_line(l1.x, l1.y, l2.x, l2.y, 2., WHITE);
            draw_line(l1.x, l1.y, l3.x, l3.y, 2., WHITE);
        }
        let l1 = -scale * vec2(0., 35.) + offset;
        if self.player.outputs[3] > 0. {
            draw_circle(l1.x, l1.y, 5., WHITE);
            draw_circle(l1.x, l1.y, 3.5, BLACK);
        }
        let params = TextParams {
            font_size: 48,
            font_scale: 0.5,
            ..Default::default()
        };
        draw_text_ex(
            if self.over { "DEAD" } else { "ALIVE" },
            -width * 0.5 + 20.,
            70.,
            {
                let mut p = params.clone();
                p.color = if self.over { RED } else { GREEN };
                p
            },
        );
        draw_text_ex(
            &format!("Hits: {}", self.score),
            -width * 0.5 + 20.,
            90.,
            params,
        );
        draw_text_ex(
            &format!("Fired: {}", self.player.shots),
            -width * 0.5 + 20.,
            110.,
            params,
        );
        draw_text_ex(
            &format!("Fitness: {:.2}", self.fitness),
            -width * 0.5 + 20.,
            130.,
            params,
        );
    }
}
