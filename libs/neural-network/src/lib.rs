pub use self::layer_topology::*;

use self::layer::*;

use rand::{Rng, RngCore};

mod layer;
mod layer_topology;
mod neuron;

pub struct Network {
    // a network is built from layers
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // for layer in &self.layers {
        //     inputs = layer.propagate(inputs);
        // }

        // use the folding pattern

        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    // consecutive layers have matching inputs and outputs
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }
}
