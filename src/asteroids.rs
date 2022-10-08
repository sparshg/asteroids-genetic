use macroquad::{prelude::*, rand::gen_range};

#[derive(Default)]
pub struct Asteroid {
    pub pos: Vec2,
    vel: Vec2,
    sides: u8,
    radius: f32,
    rot: f32,
    omega: f32,
}

impl Asteroid {
    pub fn new() -> Self {
        let radius = gen_range(30., 50.);
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
            vel: 0.04 * -r
                + vec2(
                    gen_range(20., 60.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
                    gen_range(20., 60.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
                ),
            sides: gen_range(3, 8),
            radius: radius,
            omega: gen_range(50., 200.) * if gen_range(0., 1.) > 0.5 { -1. } else { 1. },
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        self.pos += self.vel * get_frame_time();
        self.rot += self.omega * get_frame_time();
    }

    pub fn is_visible(&self) -> bool {
        self.pos.y.abs() < screen_height() * 0.51 + self.radius
            && self.pos.x.abs() < screen_width() * 0.51 + self.radius
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
