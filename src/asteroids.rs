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
}

impl Asteroid {
    pub fn new(size: AsteroidSize, (WIDTH, HEIGHT): (f32, f32)) -> Self {
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
            size,
            sides,
            radius,
            omega: gen_range(0.8, 3.5) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
            rot: 0.,
            alive: true,
        }
    }

    pub fn new_from(pos: Vec2, vel: Vec2, size: AsteroidSize, (WIDTH, HEIGHT): (f32, f32)) -> Self {
        let mut asteroid = Asteroid::new(size, (WIDTH, HEIGHT));
        asteroid.pos = pos;
        asteroid.vel = vel;
        asteroid
    }

    pub fn new_to(pos: Vec2, speed: f32, size: AsteroidSize, (WIDTH, HEIGHT): (f32, f32)) -> Self {
        let mut asteroid = Asteroid::new(size, (WIDTH, HEIGHT));
        asteroid.vel = (pos - asteroid.pos) * 0.002 * speed;
        asteroid
    }

    pub fn check_collision(&self, pos: Vec2, rad: f32) -> bool {
        (pos.x - self.pos.x) * (pos.x - self.pos.x) + (pos.y - self.pos.y) * (pos.y - self.pos.y)
            <= (self.radius + rad) * (self.radius + rad)
    }

    pub fn update(&mut self, (WIDTH, HEIGHT): (f32, f32)) {
        self.pos += self.vel;
        self.rot += self.omega;
        if self.pos.x.abs() > WIDTH * 0.5 + self.radius {
            self.pos.x *= -1.;
        }
        if self.pos.y.abs() > HEIGHT * 0.5 + self.radius {
            self.pos.y *= -1.;
        }
    }

    pub fn draw(&self, color: Color) {
        draw_poly_lines(
            self.pos.x,
            self.pos.y,
            self.sides,
            self.radius,
            self.rot,
            match self.size {
                AsteroidSize::Large => 2.,
                AsteroidSize::Medium => 1.2,
                AsteroidSize::Small => 1.,
            },
            color,
        );
    }
}
