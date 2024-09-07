use interactive_cli::start_interactive_cli;


mod candidate;
mod city_writer;
mod genetic_algorithm;
mod simualted_annealing;
mod tsp;
mod visualisation;
mod interactive_cli;

pub fn main() {
    // write_random_cities("data".to_string(), 200, 500);
    // let popualtion = 1000;
    // let initial = read_random_cities("data".to_string()).unwrap();

    // let mut sa = SimulatedAnnealing::new(1.5,0.95);
    // let best = sa.run(TspCandidate::new_shuffle(initial.clone()), 1000000);
    // sa_fitness_plot_tsp(sa, 1000000, TspCandidate::new_shuffle(initial.clone()));

    // let mut initial_population: Vec<TspCandidate> = Vec::with_capacity(popualtion);
    // for _ in 0..popualtion {
    //     initial_population.push(TspCandidate::new_shuffle(initial.clone()));
    // }
    // let ga = GA::new(initial_population, 0.07, 0.8, 0.01);
    // println!("Starting genetic algorithm");
    // generation_fitness_plot_tsp(ga, 1000, SelectionMethod::RouletteWheel);
    // for _ in 0..6000 {
    //     ga.step_print(SelectionMethod::RouletteWheel);
    // }
    // let best = ga.best();
    // let best_chromo = best.get_chromosome();
    // println!("{:?}", best_chromo);
    start_interactive_cli();
}
