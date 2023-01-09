use crate::{HEIGHT, WIDTH};
use macroquad::{prelude::*, rand::gen_range};
#[derive(Clone)]
pub enum AsteroidSize {
    Large,
    Medium,
    Small,
}

#[derive(Clone)]
pub struct Asteroid {
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: AsteroidSize,
    sides: u8,
    pub radius: f32,
    rot: f32,
    omega: f32,
    pub alive: bool,
    pub color: Color,
}

impl Asteroid {
    pub fn new(size: AsteroidSize) -> Self {
        let (sides, radius) = match size {
            AsteroidSize::Large => (gen_range(6, 10), gen_range(50., 65.)),
            AsteroidSize::Medium => (gen_range(5, 6), gen_range(35., 50.)),
            AsteroidSize::Small => (gen_range(3, 5), 25.),
        };
        let mut r = vec2(
            if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
            gen_range(-1., 1.),
        );
        if gen_range(0., 1.) > 0.5 {
            r = vec2(r.y, r.x);
        }
        r *= vec2(WIDTH * 0.5 + radius, HEIGHT * 0.5 + radius);
        Self {
            pos: r,
            vel: 0.001 * -r
                + vec2(
                    gen_range(0.3, 1.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
                    gen_range(0.3, 1.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
                ),
            size: size,
            sides: sides,
            radius: radius,
            omega: gen_range(0.8, 3.5) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
            rot: 0.,
            alive: true,
            color: Color::new(1., 1., 1., 0.2),
        }
    }

    pub fn new_from(pos: Vec2, vel: Vec2, size: AsteroidSize) -> Self {
        let mut asteroid = Asteroid::new(size);
        asteroid.pos = pos;
        asteroid.vel = vel;
        asteroid
    }

    pub fn new_to(pos: Vec2, speed: f32, size: AsteroidSize) -> Self {
        let mut asteroid = Asteroid::new(size);
        asteroid.vel = (pos - asteroid.pos) * 0.002 * speed;
        asteroid
    }

    pub fn check_collision(&self, pos: Vec2, rad: f32) -> bool {
        (pos.x - self.pos.x) * (pos.x - self.pos.x) + (pos.y - self.pos.y) * (pos.y - self.pos.y)
            <= (self.radius + rad) * (self.radius + rad)
    }

    pub fn update(&mut self) {
        self.pos += self.vel;
        self.rot += self.omega;
        if self.pos.x.abs() > WIDTH * 0.5 + self.radius {
            self.pos.x *= -1.;
        }
        if self.pos.y.abs() > HEIGHT * 0.5 + self.radius {
            self.pos.y *= -1.;
        }
    }

    pub fn draw(&self) {
        draw_poly_lines(
            self.pos.x,
            self.pos.y,
            self.sides,
            self.radius,
            self.rot,
            match self.size {
                AsteroidSize::Large => 2.,
                AsteroidSize::Medium => 1.2,
                AsteroidSize::Small => 0.8,
            },
            self.color,
        );
    }
}
