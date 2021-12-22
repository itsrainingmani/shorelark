#![feature(crate_visibility_modifier)]
#![feature(min_type_alias_impl_trait)]

pub use self::{
    individual::*, selection::*, chromosome::*
};

use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

mod individual;
mod selection;
mod chromosome;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        let parent_a = parent_a.iter();
        let parent_b = parent_b.iter();

        parent_a
            .zip(parent_b)
            .map(|(&a, &b)| if rng.gen_bool(0.5) {a} else {b})
            .collect()
    }
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    // we want this to be generic, so we use a type paramter
    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        // precondition
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population);
                let parent_b = self.selection_method.select(rng, population);

                // TODO crossover
                // TODO mutation
                // TODO convert `Chromosome` back into `Individual`
                todo!()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use std::collections::BTreeMap;

    #[test]
    fn test() {
        let method = RouletteWheelSelection::new();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        // Since we want to assess probability, instead of evoking `.select()` once,
        // we can do it many times and look at the histogram

        // use Iterator::fold() to simplify the loop
        let actual_histogram: BTreeMap<i32, _> = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(Default::default(), |mut histogram, individual| {
                // as _ asks the compiler to infer what type is required and cast this value into it
                *histogram.entry(individual.fitness() as _).or_default() += 1;

                histogram
            });

        let expected_histogram = maplit::btreemap! {
            // (fitness, how many times this fitness has been chosen)
            // higher fitness scores should be chosen more frequently
            1 => 98,
            2 => 202,
            3 => 278,
            4 => 422,
        };

        assert_eq!(actual_histogram, expected_histogram);
    }
}
