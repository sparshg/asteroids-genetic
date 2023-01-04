use macroquad::rand::gen_range;
use nalgebra::*;
use r::Rng;
use rand_distr::StandardNormal;
extern crate rand as r;

#[derive(PartialEq, Debug, Clone, Copy, Default)]

enum ActivationFunc {
    Sigmoid,
    Tanh,
    #[default]
    ReLU,
}

#[derive(Clone, Debug, Default)]
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
                    // DMatrix::<f32>::new_random(last, curr + 1)
                    // println!("{}", a);
                    // a
                    DMatrix::<f32>::from_distribution(last, curr + 1, &StandardNormal, &mut rng)
                        * (2. / last as f32).sqrt()
                })
                .collect(),

            mut_rate: 0.04,
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
                    *ele = gen_range(-1., 1.);
                    // *ele = r::thread_rng().sample::<f32, StandardNormal>(StandardNormal);
                    // *ele = r::thread_rng().sample::<f32, StandardNormal>(StandardNormal);
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
}
