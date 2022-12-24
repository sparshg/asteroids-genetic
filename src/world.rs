use crate::{nn::NN, player::Player};
use macroquad::{prelude::*, rand::gen_range};

pub struct Pillar {
    pub x: f32,
    pub y: f32,
    w: f32,
    pub h: f32,
}

impl Pillar {
    pub fn new(h: f32) -> Self {
        Self {
            x: screen_width() * 0.5,
            y: gen_range(50., screen_height() - 250.) - screen_height() * 0.5,
            w: 50.,
            h,
        }
    }

    pub fn update(&mut self) {
        self.x -= 5.;
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.x,
            -screen_height() * 0.5,
            self.w,
            self.y + screen_height() * 0.5,
            WHITE,
        );
        draw_rectangle(
            self.x,
            self.y + self.h,
            self.w,
            screen_height() * 0.5 - self.y - self.h,
            WHITE,
        );
    }
}

#[derive(Default)]
pub struct World {
    pub over: bool,
    pub h: f32,
    pub pillars: Vec<Pillar>,
    next: i32,
}

impl World {
    pub fn new(h: f32) -> Self {
        Self {
            pillars: vec![Pillar::new(200.)],
            h,
            ..Default::default()
        }
    }

    pub fn check_collision(&mut self, player: &mut Player) {
        for pillar in self.pillars.iter() {
            if player.pos.x + player.r > pillar.x
                && player.pos.x - player.r < pillar.x + pillar.w
                && (player.pos.y - player.r < pillar.y
                    || player.pos.y + player.r > pillar.y + pillar.h)
            {
                player.alive = false;
            }
        }
    }

    pub fn update(&mut self) {
        self.next += 1;
        if self.next == 100 {
            if self.h > 100. {
                self.h -= 1.;
            }
            self.pillars.push(Pillar::new(self.h));
            self.next = 0;
        }
        self.pillars
            .retain(|x| x.x + x.w + 20. + screen_width() * 0.25 > 0.);
        for pillar in self.pillars.iter_mut() {
            pillar.update();
        }
    }

    pub fn draw(&self) {
        for pillar in self.pillars.iter() {
            pillar.draw();
        }
        draw_text(
            &format!("Height: {}", self.h),
            20. - screen_width() * 0.5,
            20. - screen_height() * 0.5,
            20.,
            WHITE,
        );
    }
}
