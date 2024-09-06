pub trait GaCandidate {
    fn mutate(&mut self, mutation_rate: f64);
    fn crossover(&self, other: &Self) -> Self;
}
