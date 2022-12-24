use std::{f32::consts::PI, f64::consts::TAU};

use macroquad::{prelude::*, rand::gen_range};
use nalgebra::partial_max;

use crate::{nn::NN, world::Pillar};
#[derive(Default)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub r: f32,
    acc: f32,
    last_shot: u8,
    pub brain: Option<NN>,
    debug: bool,
    pub alive: bool,
    pub color: Option<Color>,
    pub lifespan: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            alive: true,
            pos: vec2(-screen_width() * 0.25, 0.),
            debug: true,
            r: 20.,
            brain: Some(NN::new(vec![4, 4, 1])),
            ..Default::default()
        }
    }

    pub fn simulate(brain: Option<NN>) -> Self {
        let mut p = Player::new();
        if let Some(brain) = brain {
            p.brain = Some(brain);
        } else {
            p.brain = Some(NN::new(vec![4, 4, 1]));
        }
        p
    }

    pub fn update(&mut self, pillar: &Pillar) {
        self.acc = 0.5;
        self.lifespan += 1;
        let mut keys = vec![false];
        // self.asteroids_data.resize(self.max_asteroids * 3, 0.);
        // let mut inputs = vec![
        // self.pos.x / screen_width() + 0.5,
        // self.pos.y / screen_height() + 0.5,
        // self.vel.x / 11.,
        // self.vel.y / 11.,
        // self.rot / TAU as f32,
        // self.rot.cos(),
        // self.rot.sin(),
        // ];
        // inputs.append(self.raycasts.as_mut());
        // let inputs = self.raycasts.clone();
        // inputs.append(self.asteroids_data.as_mut());
        let inputs = vec![
            self.pos.y / screen_height() + 0.5,
            self.vel.y / 30.,
            pillar.x / screen_width() + 0.5,
            (pillar.y + pillar.h) / screen_height() + 0.5,
        ];
        if let Some(brain) = &self.brain {
            keys = brain
                .feed_forward(inputs)
                .iter()
                .map(|&x| x > 0.95)
                .collect();
        }

        // if is_key_down(KeyCode::Up) && self.debug || keys[2] {
        // Change scaling when passing inputs if this is changed
        //     self.acc = 0.14;
        // }
        if is_key_pressed(KeyCode::Space) && self.debug || keys[0] {
            // self.acc = 0.
            self.vel = vec2(0., -10.);
        }

        if is_key_pressed(KeyCode::D) {
            self.debug = !self.debug;
        }

        self.vel += vec2(0., self.acc); // * self.dir - self.drag * self.vel.length() * self.vel;
        self.pos += self.vel;
        if self.pos.y > screen_height() * 0.5 || self.pos.y < -screen_height() * 0.5 {
            self.alive = false;
        }
        // if self.pos.x.abs() > screen_width() * 0.5 + 10. {
        //     self.pos.x *= -1.;
        // }
        // if self.pos.y.abs() > screen_height() * 0.5 + 10. {
        //     self.pos.y *= -1.;
        // }
    }

    pub fn draw(&self) {
        let color = match self.color {
            Some(c) => c,
            None => Color::new(1., 1., 1., 0.3),
        };
        // if self.debug {

        // }

        draw_circle(self.pos.x, self.pos.y, 20., color)
    }
}
