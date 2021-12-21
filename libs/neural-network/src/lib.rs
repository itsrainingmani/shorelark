pub struct Network {
    // a network is built from layers
    layers: Vec<Layer>,
}

struct Layer {
    // a layer is built from neurons
    neurons: Vec<Neuron>,
}

struct Neuron {
    // a neuron contains biases and output weights
    bias: f32,
    weights: Vec<f32>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
