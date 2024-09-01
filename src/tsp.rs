use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::genetic_algorithm::candidate::Candidate;

#[derive(Clone)]
pub struct TspCandidate {
    /// Chromosome is a list of (x, y) coordinates
    chromosomes: Vec<(f64, f64)>,
    fitness: f64,
}

impl Candidate<Vec<(f64, f64)>> for TspCandidate {
    fn new(chromosome: Vec<(f64, f64)>) -> Self {
        let mut chromosomes = chromosome;
        chromosomes.shuffle(&mut thread_rng());
        let fitness = TspCandidate::calculate_fitness(&chromosomes);
        TspCandidate {
            chromosomes,
            fitness,
        }
    }

    fn get_fitness(&self) -> f64 {
        self.fitness
    }

    fn get_chromosome(&self) -> &Vec<(f64, f64)> {
        &self.chromosomes
    }

    /// Mutate the chromosome by swapping two random cities
    /// with a probability of mutation_rate
    ///
    /// This chance is for each city in the chromosome
    fn mutate(&mut self, mutation_rate: f64) {
        let mut rng = rand::thread_rng();
        for i in 0..self.chromosomes.len() {
            if rng.gen::<f64>() < mutation_rate {
                let j = rng.gen_range(0..self.chromosomes.len());
                self.chromosomes.swap(i, j);
            }
        }
    }

    /// OX - Order CrossOver
    fn crossover(&self, other: &Self) -> Self {
        let mut rng = rand::thread_rng();
        let mut child_chromosome = vec![(-1.0, -1.0); self.chromosomes.len()];
        let start = rng.gen_range(0..self.chromosomes.len());
        let end = rng.gen_range(start..self.chromosomes.len());
        for i in start..end {
            child_chromosome[i] = self.chromosomes[i];
        }
        let mut j = end;
        for i in end..self.chromosomes.len() {
            while child_chromosome.contains(&other.chromosomes[j]) {
                j = (j + 1) % self.chromosomes.len();
            }
            child_chromosome[i] = other.chromosomes[j];
        }
        for i in 0..start {
            while child_chromosome.contains(&other.chromosomes[j]) {
                j = (j + 1) % self.chromosomes.len();
            }
            child_chromosome[i] = other.chromosomes[j];
        }
        TspCandidate::new(child_chromosome)
    }

    /// TSP Fitness
    /// Distance between each city in the order of the chromosome
    /// 1 / (sum of distances) is the fitness
    /// Try to maximize the fitness
    fn calculate_fitness(chromosomes: &Vec<(f64, f64)>) -> f64 {
        let mut fitness = 0.0;
        for i in 0..chromosomes.len() - 1 {
            let (x1, y1) = chromosomes[i];
            let (x2, y2) = chromosomes[i + 1];
            let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
            fitness += distance;
        }
        1.0 / fitness
    }
}
