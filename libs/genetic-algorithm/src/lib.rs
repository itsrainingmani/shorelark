// #![feature(crate_visibility_modifier)]
// #![feature(min_type_alias_impl_trait)]

pub use self::{chromosome::*, crossover::*, individual::*, selection::*};

use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

mod chromosome;
mod crossover;
mod individual;
mod selection;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
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
