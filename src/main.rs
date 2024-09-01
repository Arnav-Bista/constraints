use city_writer::{read_random_cities, write_random_cities};
use genetic_algorithm::{candidate::Candidate, genetic_algorithm::GA};
use tsp::TspCandidate;

mod genetic_algorithm;
mod tsp;

mod city_writer;

pub fn main() {
    // write_random_cities("data".to_string(), 30);
    let popualtion = 200000;
    let initial = read_random_cities("data".to_string()).unwrap();
    let mut initial_population: Vec<TspCandidate> = Vec::with_capacity(popualtion);
    for _ in 0..popualtion {
        initial_population.push(TspCandidate::new(initial.clone()));
    }
    let mut ga = GA::new(initial_population, 0.05);
    for _ in 0..5000 {
        ga.step();
    }
}
