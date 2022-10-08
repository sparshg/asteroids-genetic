use macroquad::{prelude::*, rand::gen_range};

#[derive(Default)]
pub struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    sides: u8,
    radius: f32,
    rot: f32,
    omega: f32,
}

impl Asteroid {
    pub fn new() -> Self {
        let mut r = vec2(
            if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
            gen_range(-1., 1.),
        );
        if gen_range(0., 1.) > 0.5 {
            r = vec2(r.y, r.x);
        }
        r *= vec2(screen_width() / 2. + 100., screen_height() / 2. + 100.);
        Self {
            pos: r,
            vel: vec2(
                gen_range(100., 200.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
                gen_range(100., 200.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
            ),
            sides: gen_range(3, 8),
            radius: gen_range(10., 50.),
            omega: gen_range(50., 200.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        self.pos += self.vel * get_frame_time();
        self.rot += self.omega * get_frame_time();
    }

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
