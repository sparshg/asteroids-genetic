use std::{f32::consts::PI, f64::consts::TAU};

use macroquad::{prelude::*, rand::gen_range};

use crate::{asteroids::Asteroid, nn::NN};
#[derive(Default)]
pub struct Player {
    pos: Vec2,
    vel: Vec2,
    dir: Vec2,
    rot: f32,
    drag: f32,
    bullets: Vec<Bullet>,
    last_shot: f32,
    shot_interval: f32,
    pub brain: Option<NN>,
    asteroids_data: Vec<f32>,
    max_asteroids: usize,
    debug: bool,
    alive: bool,
    pub lifespan: u32,
    pub shots: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            dir: vec2(0., -1.),
            rot: 1.5 * PI,

            // Change scaling when passing inputs if this is changed
            drag: 0.001,
            shot_interval: 0.3,
            alive: true,
            debug: false,
            ..Default::default()
        }
    }

    pub fn simulate(brain: NN, max_asteroids: usize) -> Self {
        assert_eq!(
            brain.config[0] - 1,
            max_asteroids + 5,
            "NN input size must match max_asteroids"
        );
        let mut p = Player::new();
        p.brain = Some(brain);
        p.max_asteroids = max_asteroids;
        p
    }

    pub fn check_player_collision(&mut self, asteroid: &mut Asteroid) -> bool {
        self.asteroids_data.extend([
            asteroid.pos.x / screen_width() + 0.5,
            asteroid.pos.y / screen_height() + 0.5,
            asteroid.radius / 50.,
        ]);
        if asteroid.check_collision(self.pos, 8.) {
            self.alive = false;
            return true;
        }
        false
    }

    pub fn check_bullet_collisions(&mut self, asteroid: &mut Asteroid) -> bool {
        for bullet in &mut self.bullets {
            if asteroid.check_collision(bullet.pos, 0.) {
                asteroid.alive = false;
                bullet.alive = false;
                return true;
            }
        }
        false
    }

    pub fn update(&mut self) {
        self.lifespan += 1;
        let mut mag = 0.;
        let mut keys = vec![false, false, false, false];

        self.asteroids_data.resize(self.max_asteroids, 0.);
        let mut inputs = vec![
            self.pos.x / screen_width() + 0.5,
            self.pos.y / screen_height() + 0.5,
            self.vel.x / 11.,
            self.vel.y / 11.,
            self.rot / TAU as f32,
        ];
        inputs.append(self.asteroids_data.as_mut());
        if let Some(brain) = &self.brain {
            keys = brain
                .feed_forward(inputs)
                .iter()
                .map(|&x| if x > 0. { true } else { false })
                .collect();
        }
        if is_key_down(KeyCode::Right) || keys[0] {
            self.rot = (self.rot + 0.1 + TAU as f32) % TAU as f32;
            self.dir = vec2(self.rot.cos(), self.rot.sin());
        }
        if is_key_down(KeyCode::Left) || keys[1] {
            self.rot = (self.rot - 0.1 + TAU as f32) % TAU as f32;
            self.dir = vec2(self.rot.cos(), self.rot.sin());
        }
        if is_key_down(KeyCode::Up) || keys[2] {
            // Change scaling when passing inputs if this is changed
            mag = 0.14;
        }
        if is_key_down(KeyCode::Space) || keys[3] {
            if self.shot_interval + self.last_shot < get_time() as f32 {
                self.last_shot = get_time() as f32;
                self.shots += 1;
                self.bullets.push(Bullet {
                    pos: self.pos + self.dir.rotate(vec2(20., 0.)),
                    vel: self.dir.rotate(vec2(8.5, 0.)) + self.vel,
                    alive: true,
                });
            }
        }

        if is_key_pressed(KeyCode::D) {
            self.debug = !self.debug;
        }

        self.vel += mag * self.dir - self.drag * self.vel.length() * self.vel;
        self.pos += self.vel;
        if self.pos.x.abs() > screen_width() / 2. + 10. {
            self.pos.x *= -1.;
        }
        if self.pos.y.abs() > screen_height() / 2. + 10. {
            self.pos.y *= -1.;
        }

        for bullet in &mut self.bullets {
            bullet.update();
        }
        self.bullets.retain(|b| {
            b.alive && b.pos.x.abs() * 2. < screen_width() && b.pos.y.abs() * 2. < screen_height()
        });
    }

    pub fn draw(&self) {
        let p1 = self.pos + self.dir.rotate(vec2(20., 0.));
        let p2 = self.pos + self.dir.rotate(vec2(-18., -12.667));
        let p3 = self.pos + self.dir.rotate(vec2(-18., 12.667));
        let p4 = self.pos + self.dir.rotate(vec2(-10., -10.));
        let p5 = self.pos + self.dir.rotate(vec2(-10., 10.));
        let p6 = self.pos + self.dir.rotate(vec2(-25., 0.));
        let p7 = self.pos + self.dir.rotate(vec2(-10., -6.));
        let p8 = self.pos + self.dir.rotate(vec2(-10., 6.));
        draw_line(p1.x, p1.y, p2.x, p2.y, 2., WHITE);
        draw_line(p1.x, p1.y, p3.x, p3.y, 2., WHITE);
        draw_line(p4.x, p4.y, p5.x, p5.y, 2., WHITE);
        if is_key_down(KeyCode::Up) && gen_range(0., 1.) < 0.4 {
            draw_triangle_lines(p6, p7, p8, 2., WHITE);
        }

        if self.debug {
            for a in self.asteroids_data.chunks(3) {
                draw_circle_lines(a[0], a[1], a[2], 1., GRAY);
                draw_line(self.pos.x, self.pos.y, a[0], a[1], 1., GRAY)
            }
        }

        for bullet in &self.bullets {
            bullet.draw();
        }
    }
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    alive: bool,
}

impl Bullet {
    fn update(&mut self) {
        self.pos += self.vel;
    }
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 2., WHITE);
    }
}
