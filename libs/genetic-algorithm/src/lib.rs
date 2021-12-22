pub struct GeneticAlgorithm;

// The fitness score is a property of an individual
// allows us to encapsulate all individual-oriented attributes into a single trait
// making it easy for users to discover what they need to provide
pub trait Individual {
    fn fitness(&self) -> f32;
}

// Make the selection method generic so user can use any algo they want
pub trait SelectionMethod {
    fn select<'a, I>(&self, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    // we want this to be generic, so we use a type paramter
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        // precondition
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // TODO selection
                // TODO crossover
                // TODO mutation
                todo!()
            })
            .collect()
    }
}
