pub trait Candidate<U> {
    fn new(chromosomes: U) -> Self;
    fn new_shuffle(chromosomes: U) -> Self;
    fn new_without_fitness(chromosomes: U) -> Self;
    fn get_fitness(&self) -> f64;
    fn self_calculate_fitness(&mut self);
    fn calculate_fitness(chromosomes: &U) -> f64;
    fn get_chromosome(&self) -> &U;
}
