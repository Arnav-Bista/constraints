use std::marker::PhantomData;

use crate::candidate::Candidate;

use super::simulated_annealing_candidate::SaCandidate;

pub struct SimulatedAnnealing<T, U> {
    initial_type: PhantomData<T>,
    chromosome_type: PhantomData<U>,
    temperature: f64,
    cooling_rate: f64,
}

impl<T, U> SimulatedAnnealing<T, U>
where
    T: Candidate<U> + SaCandidate + Clone,
{
    pub fn new(temperature: f64, cooling_rate: f64) -> Self {
        SimulatedAnnealing {
            initial_type: PhantomData,
            chromosome_type: PhantomData,
            temperature,
            cooling_rate,
        }
    }

    pub fn run(&mut self, initial: T, iterations: usize) -> T {
        let mut current = initial;
        for _ in 0..iterations {
            let neighbour = current.get_neighbour();
            let fitness_difference = current.get_fitness() - neighbour.get_fitness();
            if fitness_difference < 0.0 {
                current = neighbour;
                continue;
            }

            let acceptance_probability = (-fitness_difference / self.temperature).exp();
            if acceptance_probability > rand::random::<f64>() {
                current = neighbour;
            }
            self.temperature *= self.cooling_rate;
            println!("Fitness: {}", current.get_fitness());
        }

        current
    }

    pub fn step(&mut self, current: T) -> T {
        let neighbour = current.get_neighbour();
        let fitness_difference = current.get_fitness() - neighbour.get_fitness();
        if fitness_difference < 0.0 {
            return neighbour;
        }

        let acceptance_probability = (-fitness_difference / self.temperature).exp();
        if acceptance_probability > rand::random::<f64>() {
            return neighbour;
        }
        self.temperature *= self.cooling_rate;
        current
    }
}
