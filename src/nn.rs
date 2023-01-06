use macroquad::{prelude::*, rand::gen_range};
use nalgebra::*;
use r::Rng;
use rand_distr::StandardNormal;
use serde::{Deserialize, Serialize};
extern crate rand as r;

#[derive(PartialEq, Debug, Clone, Copy, Default, Serialize, Deserialize)]

enum ActivationFunc {
    Sigmoid,
    Tanh,
    #[default]
    ReLU,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NN {
    pub config: Vec<usize>,
    pub weights: Vec<DMatrix<f32>>,
    activ_func: ActivationFunc,
    mut_rate: f32,
}

impl NN {
    // Vec of number of neurons in input, hidden 1, hidden 2, ..., output layers
    pub fn new(config: Vec<usize>) -> Self {
        let mut rng = r::thread_rng();

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
                    DMatrix::<f32>::from_distribution(last, curr + 1, &StandardNormal, &mut rng)
                        * (2. / last as f32).sqrt()
                })
                .collect(),

            mut_rate: 0.05,
            ..Default::default()
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
                .map(|(m1, m2)| m1.zip_map(m2, |ele1, ele2| if r::random() { ele1 } else { ele2 }))
                .collect(),
            ..Default::default()
        }
    }

    pub fn mutate(&mut self) {
        for weight in &mut self.weights {
            for ele in weight {
                if gen_range(0., 1.) < self.mut_rate {
                    // *ele += gen_range(-1., 1.);
                    // *ele = gen_range(-1., 1.);
                    *ele = r::thread_rng().sample::<f32, StandardNormal>(StandardNormal);
                }
            }
        }
    }

    pub fn feed_forward(&self, inputs: Vec<f32>) -> Vec<f32> {
        // println!("inputs: {:?}", inputs);
        let mut y = DMatrix::from_vec(inputs.len(), 1, inputs);
        for i in 0..self.config.len() - 1 {
            y = (&self.weights[i] * y.insert_row(self.config[i] - 1, 1.)).map(|x| {
                match self.activ_func {
                    ActivationFunc::ReLU => x.max(0.),
                    ActivationFunc::Sigmoid => 1. / (1. + (-x).exp()),
                    ActivationFunc::Tanh => x.tanh(),
                }
            });
            // println!("w{}: {}", i, self.weights[i]);
            // println!("y: {}", y);
        }
        y.column(0).data.into_slice().to_vec()
    }

    pub fn draw(&self, width: f32, height: f32) {
        draw_rectangle_lines(-width * 0.5, -height * 0.5, width, height, 2., WHITE);

        let width = width * 0.8;
        let height = height * 0.8;
        let vspace = height / (self.config.iter().max().unwrap() - 1) as f32;
        let mut p1s: Vec<(f32, f32)> = Vec::new();
        let mut p2s: Vec<(f32, f32)> = Vec::new();
        for (i, layer) in self
            .config
            .iter()
            .take(self.config.len() - 1)
            .map(|x| x - 1)
            .chain(self.config.last().map(|&x| x))
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
            for (k, j, p1, p2) in p1s
                .iter()
                .enumerate()
                .flat_map(|(k, x)| p2s.iter().enumerate().map(move |(j, y)| (k, j, *x, *y)))
            {
                draw_line(
                    p1.0,
                    p1.1,
                    p2.0,
                    p2.1,
                    1.,
                    Color::new(1., 1., 1., (self.weights[i - 1].index((j, k))).abs()),
                );
            }
            for p in &p1s {
                draw_circle(p.0, p.1, 10., WHITE);
                draw_circle(p.0, p.1, 9., BLACK);
            }
        }
        for p in &p2s {
            draw_circle(p.0, p.1, 10., WHITE);
            draw_circle(p.0, p.1, 9., BLACK);
        }
    }

    pub fn export(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn import() -> NN {
        let json = std::fs::read_to_string("models/brain.json").expect("Unable to read file");
        serde_json::from_str(&json).unwrap()
    }
}
