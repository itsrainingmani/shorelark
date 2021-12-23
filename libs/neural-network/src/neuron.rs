use crate::*;

pub struct Neuron {
    // a neuron contains biases and output weights
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        // since we iterate through self.weights using length from inputs,
        // assert that they are of equal length
        // could error handle but that's too much effort
        assert_eq!(inputs.len(), self.weights.len());
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        (self.bias + output).max(0.0)
    }

    pub fn random(rng: &mut dyn RngCore, output_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0); // -1.0..=1.0 is a closed interval that matches 1.0 as well

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;

        use approx::assert_relative_eq;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        // floating pt numbers aren't always exact so we should be careful comparing them
        // either compare them approximately using something like this macro or use the approx crate
        // for ex: assert_eq!(0.45f32, 0.15 + 0.15 + 0.15) will fail
        macro_rules! assert_almost_eq {
            ($left: expr, $right: expr) => {
                let left: f32 = $left;
                let right: f32 = $right;

                assert!((left - right).abs() < f32::EPSILON)
            };
        }

        #[test]
        fn test_fp_approx() {
            assert_almost_eq!(0.45f32, 0.15 + 0.15 + 0.15);
        }

        #[test]
        fn test() {
            // Because we always use the same seed, our `rng` in here will
            // always return the same set of values
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            // assert_almost_eq!(neuron.bias, -0.6255188);
            // assert_eq!(
            //     neuron.weights,
            //     &[0.67383957, 0.8181262, 0.26284897, 0.5238807,]
            // );

            // use the approx crate for floating point assertions
            assert_relative_eq!(neuron.bias, -0.6255188);
            assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383957, 0.8181262, 0.26284897, 0.5238807,].as_ref()
            );
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn test() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            // Ensure `.max()` (our ReLU) works
            approx::assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);

            // `0.5` and `1.0` chosen by a fair dice roll
            approx::assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5 // (w1 * x1) + (w2 * x2) + bias
            );
        }
    }
}
