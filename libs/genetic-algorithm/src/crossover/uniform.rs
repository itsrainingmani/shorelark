use crate::*;

#[derive(Clone, Debug, Default)]
pub struct UniformCrossover;

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
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a = (1..=100).map(|n| n as f32).collect(); // [1, 2, ..., 100]
        let parent_b = (1..=100).map(|n| -n as f32).collect(); // [-1, -2, ..., -100]

        let child = UniformCrossover::default().crossover(&mut rng, &parent_a, &parent_b);

        // Number of genes different between child and parent_a
        let diff_a = child.iter().zip(parent_a).filter(|(c, p)| *c != p).count();
        let diff_b = child.iter().zip(parent_b).filter(|(c, p)| *c != p).count();

        // our child got 49% of its genome from parent_a and 51% of its genome from parent_b
        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }
}
