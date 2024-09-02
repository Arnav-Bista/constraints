use city_writer::{read_random_cities, write_random_cities};
use genetic_algorithm::{candidate::Candidate, genetic_algorithm::GA};
use tsp::TspCandidate;

mod genetic_algorithm;
mod tsp;

mod city_writer;

pub fn main() {
    // write_random_cities("data".to_string(), 50);
    let popualtion = 1000;
    let initial = read_random_cities("data".to_string()).unwrap();
    let mut initial_population: Vec<TspCandidate> = Vec::with_capacity(popualtion);
    for _ in 0..popualtion {
        initial_population.push(TspCandidate::new_shuffle(initial.clone()));
    }
    let mut ga = GA::new(initial_population, 0.06, 0.75, 0.01);
    for _ in 0..6000 {
        ga.step(5);
    }
    let best = ga.best();
    let best_chromo = best.get_chromosome();
    println!("{:?}", best_chromo);
}
