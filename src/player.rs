use std::{f32::consts::PI, f64::consts::TAU};

use macroquad::{prelude::*, rand::gen_range};

use crate::{
    asteroids::Asteroid,
    nn::{ActivationFunc, NN},
};
#[derive(Default)]
pub struct Player {
    pub pos: Vec2,
    vel: Vec2,
    acc: f32,
    pub dir: Vec2,
    rot: f32,
    drag: f32,
    bullets: Vec<Bullet>,
    asteroid: Option<Asteroid>,
    inputs: Vec<f32>,
    pub outputs: Vec<f32>,
    // asteroid_data: Vec<(f32, f32, f32)>,
    raycasts: Vec<f32>,
    last_shot: u32,
    shot_interval: u32,
    pub brain: Option<NN>,
    alive: bool,
    pub lifespan: u32,
    pub shots: u32,
}

impl Player {
    pub fn new(
        config: Option<Vec<usize>>,
        mut_rate: Option<f32>,
        activ: Option<ActivationFunc>,
    ) -> Self {
        Self {
            brain: match config {
                Some(mut c) => {
                    c.retain(|&x| x != 0);
                    // Number of inputs
                    c.insert(0, 5);
                    // Number of outputs
                    c.push(4);
                    Some(NN::new(c, mut_rate.unwrap(), activ.unwrap()))
                }
                _ => None,
            },
            dir: vec2(0., -1.),
            rot: 1.5 * PI,

            // Change scaling when passing inputs if this is changed
            drag: 0.001,
            shot_interval: 18,
            alive: true,
            shots: 4,
            outputs: vec![0.; 4],
            raycasts: vec![0.; 8],

            ..Default::default()
        }
    }

    pub fn check_player_collision(&mut self, asteroid: &Asteroid) -> bool {
        // To give more near asteroids data:

        // self.asteroid_data.push((
        //     ((asteroid.pos - self.pos).length() - asteroid.radius) / WIDTH,
        //     self.dir.angle_between(asteroid.pos - self.pos),
        //     (asteroid.vel - self.vel).length(),
        // ));

        // Single asteroid data:
        if self.asteroid.is_none()
            || (asteroid.pos).distance_squared(self.pos)
                < self
                    .asteroid
                    .as_ref()
                    .unwrap()
                    .pos
                    .distance_squared(self.pos)
        {
            self.asteroid = Some(asteroid.clone());
        }

        // Try raycasting below:

        // let v = asteroid.pos - self.pos;
        // for i in 0..4 {
        //     let dir = Vec2::from_angle(PI / 4. * i as f32).rotate(self.dir);
        //     let cross = v.perp_dot(dir);
        //     let dot = v.dot(dir);
        //     if cross.abs() <= asteroid.radius {
        //         self.raycasts[if dot >= 0. { i } else { i + 4 }] = *partial_max(
        //             &self.raycasts[if dot >= 0. { i } else { i + 4 }],
        //             &(100.
        //                 / (dot.abs() - (asteroid.radius * asteroid.radius - cross * cross).sqrt())),
        //         )
        //         .unwrap();
        //     }
        // }
        if asteroid.check_collision(self.pos, 8.) || self.lifespan > 3600 && self.brain.is_some() {
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
                self.asteroid = None;
                return true;
            }
        }
        false
    }

    pub fn update(&mut self, (WIDTH, HEIGHT): (f32, f32)) {
        self.lifespan += 1;
        self.last_shot += 1;
        self.acc = 0.;
        self.outputs = vec![0.; 4];
        let mut keys = vec![false; 4];
        if let Some(ast) = self.asteroid.as_ref() {
            self.inputs = vec![
                (ast.pos - self.pos).length() / HEIGHT,
                self.dir.angle_between(ast.pos - self.pos),
                (ast.vel - self.vel).x * 0.3,
                (ast.vel - self.vel).y * 0.3,
                self.rot / TAU as f32,
                // self.vel.x / 8.,
                // self.vel.y / 8.,
                // self.rot / TAU as f32,
            ];
            // self.inputs.append(self.raycasts.as_mut());

            // self.asteroid_data
            //     .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            // self.asteroid_data.resize(1, (0., 0., 0.));
            // inputs.append(
            //     &mut self
            //         .asteroid_data
            //         .iter()
            //         .map(|(d, a, h)| vec![*d, *a, *h])
            //         .flatten()
            //         .collect::<Vec<_>>(),
            // );

            if let Some(brain) = &self.brain {
                self.outputs = brain.feed_forward(&self.inputs);
                keys = self
                    .outputs
                    .iter()
                    .map(|&x| {
                        x > if brain.activ_func == ActivationFunc::Sigmoid {
                            0.85
                        } else {
                            0.
                        }
                    })
                    .collect();
            }
        }
        if keys[0] || self.brain.is_none() && is_key_down(KeyCode::Right) {
            // RIGHT
            self.rot = (self.rot + 0.1 + TAU as f32) % TAU as f32;
            self.dir = vec2(self.rot.cos(), self.rot.sin());
        }
        if keys[1] || self.brain.is_none() && is_key_down(KeyCode::Left) {
            // LEFT
            self.rot = (self.rot - 0.1 + TAU as f32) % TAU as f32;
            self.dir = vec2(self.rot.cos(), self.rot.sin());
        }
        if keys[2] || self.brain.is_none() && is_key_down(KeyCode::Up) {
            // THROTTLE
            self.acc = 0.14;
        }
        if (keys[3] || self.brain.is_none() && is_key_down(KeyCode::Space))
            && self.last_shot > self.shot_interval
        {
            self.last_shot = 0;
            self.shots += 1;
            self.bullets.push(Bullet {
                pos: self.pos + self.dir * 20.,
                vel: self.dir * 8.5 + self.vel,
                alive: true,
            });
        }

        self.vel += self.acc * self.dir - self.drag * self.vel.length() * self.vel;
        self.pos += self.vel;
        if self.pos.x.abs() > WIDTH * 0.5 + 10. {
            self.pos.x *= -1.;
        }
        if self.pos.y.abs() > HEIGHT * 0.5 + 10. {
            self.pos.y *= -1.;
        }

        for bullet in &mut self.bullets {
            bullet.update();
        }
        self.bullets
            .retain(|b| b.alive && b.pos.x.abs() * 2. < WIDTH && b.pos.y.abs() * 2. < HEIGHT);
        self.asteroid = None;
        // self.asteroid_data.clear();
        // self.raycasts = vec![0.; 8];
    }

    pub fn draw(&self, color: Color, debug: bool) {
        let p1 = self.pos + self.dir * 20.;
        let p2 = self.pos + self.dir.rotate(vec2(-18., -12.667));
        let p3 = self.pos + self.dir.rotate(vec2(-18., 12.667));
        let p4 = self.pos + self.dir.rotate(vec2(-10., -10.));
        let p5 = self.pos + self.dir.rotate(vec2(-10., 10.));
        let p6 = self.pos + self.dir * -25.;
        let p7 = self.pos + self.dir.rotate(vec2(-10., -6.));
        let p8 = self.pos + self.dir.rotate(vec2(-10., 6.));
        draw_line(p1.x, p1.y, p2.x, p2.y, 2., color);
        draw_line(p1.x, p1.y, p3.x, p3.y, 2., color);
        draw_line(p4.x, p4.y, p5.x, p5.y, 2., color);
        if self.acc > 0. && gen_range(0., 1.) < 0.4 {
            draw_triangle_lines(p6, p7, p8, 2., color);
        }
        if debug {
            if let Some(ast) = self.asteroid.as_ref() {
                draw_circle_lines(ast.pos.x, ast.pos.y, ast.radius, 1., RED);
                // let p = self.pos
                //     + self.dir.rotate(Vec2::from_angle(self.asteroid_data[0].1))
                //         * self.asteroid_data[0].0
                //         * WIDTH;
                draw_line(self.pos.x, self.pos.y, ast.pos.x, ast.pos.y, 1., RED);
            }

            // Draw raycasts

            // for (i, r) in self.raycasts.iter().enumerate() {
            //     let dir = Vec2::from_angle(PI / 4. * i as f32).rotate(self.dir);
            //     draw_line(
            //         self.pos.x,
            //         self.pos.y,
            //         self.pos.x + dir.x * 100. / r,
            //         self.pos.y + dir.y * 100. / r,
            //         1.,
            //         GRAY,
            //     );
            // }
        }

        for bullet in &self.bullets {
            bullet.draw(color);
        }
    }

    pub fn draw_brain(&self, width: f32, height: f32, bias: bool) {
        if let Some(brain) = &self.brain {
            brain.draw(width, height, &self.inputs, &self.outputs, bias);
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
    fn draw(&self, c: Color) {
        draw_circle(self.pos.x, self.pos.y, 2., Color::new(c.r, c.g, c.b, 0.9));
    }
}
