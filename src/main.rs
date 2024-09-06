use city_writer::{read_random_cities, write_random_cities};
use genetic_algorithm::{candidate::Candidate, genetic_algorithm::GA};
use tsp::TspCandidate;
use visualisation::plotting::generation_fitness_plot_tsp;

use crate::genetic_algorithm::selection::SelectionMethod;

mod genetic_algorithm;
mod tsp;
mod visualisation;
mod city_writer;

pub fn main() {
    // write_random_cities("data".to_string(), 30);
    let popualtion = 1000;
    let initial = read_random_cities("data".to_string()).unwrap();
    let mut initial_population: Vec<TspCandidate> = Vec::with_capacity(popualtion);
    for _ in 0..popualtion {
        initial_population.push(TspCandidate::new_shuffle(initial.clone()));
    }
    let ga = GA::new(initial_population, 0.07, 0.1, 0.00);
    println!("Starting genetic algorithm");
    generation_fitness_plot_tsp(ga, 1000, SelectionMethod::RouletteWheel);
    // for _ in 0..6000 {
    //     ga.step_print(SelectionMethod::RouletteWheel);
    // }
    // let best = ga.best();
    // let best_chromo = best.get_chromosome();
    // println!("{:?}", best_chromo);
}
