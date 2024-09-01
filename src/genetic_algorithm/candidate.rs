pub trait Candidate<U> {
    fn new(chromosomes: U) -> Self;
    fn get_fitness(&self) -> f64;
    fn calculate_fitness(chromosomes: &U) -> f64;
    fn get_chromosome(&self) -> &U;
    fn mutate(&mut self, mutation_rate: f64);
    fn crossover(&self, other: &Self) -> Self;
}
