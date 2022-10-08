use macroquad::{prelude::*, rand::gen_range};

pub enum AsteroidSize {
    Large,
    Medium,
    Small,
}
pub struct Asteroid {
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: AsteroidSize,
    sides: u8,
    radius: f32,
    rot: f32,
    omega: f32,
    pub alive: bool,
}

impl Asteroid {
    pub fn new(size: AsteroidSize) -> Self {
        let (sides, radius) = match size {
            AsteroidSize::Large => (gen_range(6, 10), gen_range(40., 50.)),
            AsteroidSize::Medium => (gen_range(5, 6), gen_range(30., 40.)),
            AsteroidSize::Small => (gen_range(3, 5), 20.),
        };
        let mut r = vec2(
            if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
            gen_range(-1., 1.),
        );
        if gen_range(0., 1.) > 0.5 {
            r = vec2(r.y, r.x);
        }
        r *= vec2(
            screen_width() * 0.5 + radius,
            screen_height() * 0.5 + radius,
        );
        Self {
            pos: r,
            vel: 0.1 * -r
                + vec2(
                    gen_range(20., 60.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
                    gen_range(20., 60.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
                ),
            size: size,
            sides: sides,
            radius: radius,
            omega: gen_range(50., 200.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
            rot: 0.,
            alive: true,
        }
    }

    pub fn new_from(pos: Vec2, vel: Vec2, size: AsteroidSize) -> Self {
        let mut asteroid = Asteroid::new(size);
        asteroid.pos = pos;
        asteroid.vel = vel;
        asteroid
    }

    pub fn check_collision(&mut self, pos: Vec2) -> bool {
        let collided = (pos.x - self.pos.x) * (pos.x - self.pos.x)
            + (pos.y - self.pos.y) * (pos.y - self.pos.y)
            <= self.radius * self.radius;
        if collided {
            self.alive = false;
        }
        return collided;
    }

    pub fn update(&mut self) {
        if self.alive {
            self.pos += self.vel * get_frame_time();
            self.rot += self.omega * get_frame_time();
            self.alive = self.pos.y.abs() < screen_height() * 0.51 + self.radius
                && self.pos.x.abs() < screen_width() * 0.51 + self.radius;
        }
    }

    // pub fn is_visible(&self) -> bool {
    // }

    pub fn draw(&self) {
        draw_poly_lines(
            self.pos.x,
            self.pos.y,
            self.sides,
            self.radius,
            self.rot,
            2.,
            WHITE,
        );
    }
}
