pub use self::roulette_wheel::*;

use crate::*;

mod roulette_wheel;

// Make the selection method generic so user can use any algo they want
pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}
