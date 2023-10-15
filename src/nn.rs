use std::f32::consts::PI;

use macroquad::{prelude::*, rand::gen_range};
use nalgebra::*;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]

pub enum ActivationFunc {
    ReLU,
    Sigmoid,
    Tanh,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NN {
    pub config: Vec<usize>,
    pub weights: Vec<DMatrix<f32>>,
    pub activ_func: ActivationFunc,
    pub mut_rate: f32,
}

impl NN {
    // Vec of number of neurons in input, hidden 1, hidden 2, ..., output layers
    pub fn new(config: Vec<usize>, mut_rate: f32, activ: ActivationFunc) -> Self {
        Self {
            config: config
                .iter()
                .enumerate()
                .map(|(i, &x)| if i != config.len() - 1 { x + 1 } else { x })
                .collect(),

            // He-et-al Initialization
            weights: config
                .iter()
                .zip(config.iter().skip(1))
                .map(|(&curr, &last)| {
                    // DMatrix::from_fn(last, curr + 1, |_, _| gen_range(-1., 1.))
                    DMatrix::from_fn(last, curr + 1, |_, _| {
                        // uniform random to standard normal
                        (-2. * gen_range(0., 1.).ln()).sqrt() * (PI * gen_range(0., 2.)).cos()
                    }) * (2. / last as f32).sqrt()
                })
                .collect(),

            mut_rate,
            activ_func: activ,
        }
    }

    pub fn crossover(a: &NN, b: &NN) -> Self {
        assert_eq!(a.config, b.config, "NN configs not same.");
        Self {
            config: a.config.to_owned(),
            activ_func: a.activ_func,
            mut_rate: a.mut_rate,
            weights: a
                .weights
                .iter()
                .zip(b.weights.iter())
                .map(|(m1, m2)| {
                    m1.zip_map(
                        m2,
                        |ele1, ele2| if gen_range(0., 1.) < 0.5 { ele1 } else { ele2 },
                    )
                })
                .collect(),
        }
    }

    pub fn mutate(&mut self) {
        for weight in &mut self.weights {
            for ele in weight {
                if gen_range(0., 1.) < self.mut_rate {
                    // *ele += gen_range(-1., 1.);
                    // uniform random to standard normal
                    *ele = (-2. * gen_range(0., 1.).ln()).sqrt() * (PI * gen_range(0., 2.)).cos();
                }
            }
        }
    }

    pub fn feed_forward(&self, inputs: &Vec<f32>) -> Vec<f32> {
        // println!("inputs: {:?}", inputs);
        let mut y = DMatrix::from_vec(inputs.len(), 1, inputs.to_vec());
        for i in 0..self.config.len() - 1 {
            y = (&self.weights[i] * y.insert_row(self.config[i] - 1, 1.)).map(|x| {
                match self.activ_func {
                    ActivationFunc::ReLU => x.max(0.),
                    ActivationFunc::Sigmoid => 1. / (1. + (-x).exp()),
                    ActivationFunc::Tanh => x.tanh(),
                }
            });
        }
        y.column(0).data.into_slice().to_vec()
    }

    pub fn draw(&self, width: f32, height: f32, inputs: &Vec<f32>, outputs: &Vec<f32>, bias: bool) {
        draw_rectangle_lines(-width * 0.5, -height * 0.5, width, height, 2., WHITE);

        let width = width * 0.8;
        let height = height * 0.8;
        let vspace = height / (self.config.iter().max().unwrap() - 1) as f32;
        let mut p1s: Vec<(f32, f32)>;
        let mut p2s: Vec<(f32, f32)> = Vec::new();
        for (i, layer) in self
            .config
            .iter()
            .take(self.config.len() - 1)
            .map(|x| x - if bias { 0 } else { 1 })
            .chain(self.config.last().copied())
            .enumerate()
        {
            p1s = p2s;
            p2s = Vec::new();
            for neuron in 0..layer {
                p2s.push((
                    i as f32 * width / (self.config.len() - 1) as f32 - width * 0.5,
                    neuron as f32 * vspace - (vspace * (layer - 1) as f32) * 0.5,
                ));
            }
            for (k, j, p1, p2) in p1s.iter().enumerate().flat_map(|(k, x)| {
                p2s.iter()
                    .take(
                        p2s.len()
                            - if i == self.config.len() - 1 || !bias {
                                0
                            } else {
                                1
                            },
                    )
                    .enumerate()
                    .map(move |(j, y)| (k, j, *x, *y))
            }) {
                let weight = *self.weights[i - 1].index((j, k));
                let c = if weight < 0. { 0. } else { 1. };
                draw_line(
                    p1.0,
                    p1.1,
                    p2.0,
                    p2.1,
                    1.5,
                    Color::new(1., c, c, weight.abs()),
                );
            }

            let mut inputs = inputs.to_vec();
            inputs.push(1.);

            for (j, p) in p1s.iter().enumerate() {
                draw_circle(p.0, p.1, 10., WHITE);
                draw_circle(p.0, p.1, 8., BLACK);
                draw_circle(
                    p.0,
                    p.1,
                    8.,
                    if i == 1 && inputs.len() > 1 {
                        let c = if inputs[j] < 0. { 0. } else { 1. };
                        Color::new(1., c, c, inputs[j].abs())
                    } else {
                        BLACK
                    },
                );
                if i == 1 && inputs.len() > 1 {
                    draw_text(
                        &format!("{:.2}", inputs[j]),
                        p.0 - if inputs[j] < 0. { 50. } else { 42. },
                        p.1 + 4.,
                        16.,
                        WHITE,
                    );
                }
            }
        }
        for (j, p) in p2s.iter().enumerate() {
            draw_circle(p.0, p.1, 10., WHITE);
            draw_circle(p.0, p.1, 8., BLACK);
            if !outputs.is_empty() {
                draw_circle(p.0, p.1, 8., Color::new(1., 1., 1., outputs[j]));
                draw_text(
                    &format!("{:.2}", outputs[j]),
                    p.0 + 14.,
                    p.1 + 4.,
                    16.,
                    WHITE,
                );
            }
        }
        draw_rectangle(width * 0.47, height * 0.47, 10., 10., RED);
        let params = TextParams {
            font_size: 40,
            font_scale: 0.5,
            ..Default::default()
        };
        draw_text_ex("-ve", width * 0.47 + 20., height * 0.47 + 10., params);
        draw_rectangle(width * 0.47, height * 0.47 + 20., 10., 10., WHITE);
        draw_text_ex("+ve", width * 0.47 + 20., height * 0.47 + 30., params);
    }

    pub fn export(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn import(path: &str) -> NN {
        let json = std::fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&json).unwrap()
    }
}
