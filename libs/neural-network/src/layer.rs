use crate::{neuron::*, *};

pub struct Layer {
    // a layer is built from neurons
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // preallocate our vector since we know how many output value we will have
        // let mut outputs = Vec::with_capacity(self.neurons.len());

        // for neuron in &self.neurons {
        //     let output = neuron.propagate(&inputs);
        //     outputs.push(output);
        // }

        // outputs

        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn random(rng: &mut dyn RngCore, input_neurons: usize, output_neurons: usize) -> Self {
        let mut neurons = Vec::new();

        for _ in 0..output_neurons {
            neurons.push(Neuron::random(rng, input_neurons));
        }

        Self { neurons }
    }
}
