use crate::*;

// The fitness score is a property of an individual
// allows us to encapsulate all individual-oriented attributes into a single trait
// making it easy for users to discover what they need to provide
pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    // crossover doesnt actually happen on an individual, rather on their chromosomes
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    // for tests that require access to Chromosome
    WithChromosome { chromosome: Chromosome },

    // for tests that don't require access to Chromosome
    WithFitness { fitness: f32 },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }

    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,

            Self::WithFitness { .. } => {
                panic!("not supported for TestIndividual::WithFitness")
            }
        }
    }

    fn fitness(&self) -> f32 {
        match self {
            Self::WithChromosome { chromosome } => chromosome.iter().sum(),

            Self::WithFitness { fitness } => *fitness,
        }
    }
}
