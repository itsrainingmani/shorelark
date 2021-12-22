use crate::*;

// The fitness score is a property of an individual
// allows us to encapsulate all individual-oriented attributes into a single trait
// making it easy for users to discover what they need to provide
pub trait Individual {
    // crossover doesnt actually happen on an individual, rather on their chromosomes
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn chromosome(&self) -> &Chromosome {
        panic!("not supported for TestIndividual")
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}
