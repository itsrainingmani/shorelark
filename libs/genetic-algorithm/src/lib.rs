pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    // we want this to be generic, so we use a type paramter
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I> {
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
