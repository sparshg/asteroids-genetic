use nalgebra::*;
use rand_distr::StandardNormal;
extern crate rand as r;

enum ActivationFunc {
    Sigmoid,
    Tanh,
    ReLU,
}
pub struct NN {
    config: Vec<usize>,
    weights: Vec<DMatrix<f32>>,
    activ_func: ActivationFunc,
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
                    DMatrix::<f32>::from_distribution(last, curr + 1, &StandardNormal, &mut rng)
                        * (2. / last as f32).sqrt()
                })
                .collect(),

            activ_func: ActivationFunc::ReLU,
        }
    }

    pub fn feed_forward(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut y = DMatrix::from_vec(inputs.len(), 1, inputs);
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
}
