use std::f32::consts::PI;

use macroquad::{prelude::*, rand::gen_range};
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
}

impl Player {
    pub fn new() -> Self {
        Self {
            dir: vec2(0., 1.),
            rot: PI / 2.,
            drag: 0.001,
            shot_interval: 0.3,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let mut mag = 0.;
        if is_key_down(KeyCode::Right) {
            self.rot -= 5. * get_frame_time();
            self.dir = vec2(self.rot.cos(), self.rot.sin());
        }
        if is_key_down(KeyCode::Left) {
            self.rot += 5. * get_frame_time();
            self.dir = vec2(self.rot.cos(), self.rot.sin());
        }
        if is_key_down(KeyCode::Up) {
            mag = 360.;
        }
        if is_key_down(KeyCode::Space) {
            if self.shot_interval + self.last_shot < get_time() as f32 {
                self.last_shot = get_time() as f32;
                self.bullets.push(Bullet {
                    pos: self.pos + self.dir.rotate(vec2(20., 0.)),
                    vel: self.dir.rotate(vec2(500., 0.)) + self.vel,
                });
            }
        }

        self.bullets
            .iter_mut()
            .filter(|bullet| {
                bullet.pos.x.abs() * 2. < screen_width()
                    && bullet.pos.y.abs() * 2. < screen_height()
            })
            .for_each(|bullet| {
                bullet.update();
                bullet.draw();
            });

        self.vel += (mag * self.dir - self.drag * self.vel.length() * self.vel) * get_frame_time();
        self.pos += self.vel * get_frame_time();
        if self.pos.x.abs() > screen_width() / 2. + 10. {
            self.pos.x *= -1.;
        }
        if self.pos.y.abs() > screen_height() / 2. + 10. {
            self.pos.y *= -1.;
        }
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
        if is_key_down(KeyCode::Up) && gen_range(0., 1.) < 0.5 {
            draw_triangle_lines(p6, p7, p8, 2., WHITE);
        }
    }
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
}

impl Bullet {
    fn update(&mut self) {
        self.pos += self.vel * get_frame_time();
    }
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 2., WHITE);
    }
}
