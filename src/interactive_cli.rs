use std::{fmt::Display, str::FromStr};


use crate::{
    candidate::Candidate,
    city_writer::write_random_cities,
    genetic_algorithm::selection::SelectionMethod,
    tsp::TspCandidate,
    visualisation::plotting::{generation_fitness_plot_tsp, sa_fitness_plot_tsp},
};

pub fn start_interactive_cli() {
    println!("{}", "Welcome to Constraints");
    let options = vec![
        "Generate random cities",
        "Genetic Algorithm",
        "Simulated Annealing",
    ];

    match show_menu(options) {
        0 => {
            let number = get_num_input("How many cities to generate?", None, Some(3), None);
            if number.is_none() || number.unwrap() < 1 {
                println!("Invalid input. Aborting.");
                return;
            }
            let number = number.unwrap();
            write_random_cities("data".to_string(), number as usize, number);
        }
        1 => {
            let population_count = get_num_input("Population Count", Some(1000), Some(2), None);
            if population_count.is_none() {
                println!("Invalid input. Aborting.");
                return;
            }
            let mutation_rate = get_num_input("Mutation Rate", Some(0.07), Some(0.0), Some(1.0));
            if mutation_rate.is_none() {
                println!("Invalid input. Aborting.");
                return;
            }
            let selection = get_num_input("Selection Percentage", Some(0.8), Some(0.0), Some(1.0));
            if selection.is_none() {
                println!("Invalid input. Aborting.");
                return;
            }
            let elitism = get_num_input("Elitism Percentage", Some(0.01), Some(0.0), Some(1.0));
            if elitism.is_none() {
                println!("Invalid input. Aborting.");
                return;
            }
            let options = vec!["RouletteWheel", "Tournament"];
            let selection_method = match show_menu(options) {
                0 => SelectionMethod::RouletteWheel,
                1 => {
                    let tournament_size = get_num_input("Tournament Size", Some(2), Some(2), None);
                    if tournament_size.is_none() {
                        println!("Invalid input. Aborting.");
                        return;
                    }
                    SelectionMethod::Tournament(tournament_size.unwrap())
                }
                _ => panic!("Invalid selection method."),
            };

            let iterations = get_num_input("Iterations", Some(1000), Some(1), None);
            if iterations.is_none() {
                println!("Invalid input. Aborting.");
                return;
            }

            let initial = crate::city_writer::read_random_cities("data".to_string()).unwrap();
            let population_count = population_count.unwrap();
            let mut initial_population: Vec<TspCandidate> = Vec::with_capacity(population_count);
            for _ in 0..population_count {
                initial_population.push(TspCandidate::new_shuffle(initial.clone()));
            }
            let ga = crate::genetic_algorithm::genetic_algorithm::GA::new(
                initial_population,
                mutation_rate.unwrap(),
                selection.unwrap(),
                elitism.unwrap(),
            );

            println!("Starting genetic algorithm");
            generation_fitness_plot_tsp(ga, iterations.unwrap(), selection_method);
        }
        2 => {
            let temperature = get_num_input("Temperature", Some(1.5), Some(0.0), None);
            if temperature.is_none() {
                println!("Invalid input. Aborting.");
                return;
            }
            let cooling_rate = get_num_input("Cooling Rate", Some(0.95), Some(0.0), Some(1.0));
            if cooling_rate.is_none() {
                println!("Invalid input. Aborting.");
                return;
            }
            let iterations = get_num_input("Iterations", Some(1000000), Some(1), None);
            if iterations.is_none() {
                println!("Invalid input. Aborting.");
                return;
            }
            let initial = crate::city_writer::read_random_cities("data".to_string()).unwrap();
            let sa = crate::simualted_annealing::simulated_annealing::SimulatedAnnealing::new(
                temperature.unwrap(),
                cooling_rate.unwrap(),
            );
            println!("Starting simulated annealing");
            sa_fitness_plot_tsp(
                sa,
                iterations.unwrap(),
                TspCandidate::new_shuffle(initial.clone()),
            );
        }
        _ => {}
    }
}

fn get_num_input<T>(
    prompt: &str,
    default: Option<T>,
    minimum: Option<T>,
    maximum: Option<T>,
) -> Option<T>
where
    T: PartialOrd + FromStr + Display + Copy,
{
    match (default, minimum, maximum) {
        (Some(default), Some(minimum), Some(maximum)) => {
            if default < minimum || default > maximum {
                panic!("Default value is not within the constraints.");
            }
            println!(
                "{} (default {}, within {} - {} inc)",
                prompt, default, minimum, maximum
            );
        }
        (Some(default), Some(minimum), None) => {
            if default < minimum {
                panic!("Default value is not within the constraints.");
            }
            println!("{} (default {}, > {})", prompt, default, minimum);
        }
        (Some(default), None, Some(maximum)) => {
            if default > maximum {
                panic!("Default value is not within the constraints.");
            }
            println!("{} (default {}, < {})", prompt, default, maximum);
        }
        (None, None, None) => println!("{}", prompt),
        (None, None, Some(maximum)) => println!("{} (< {})", prompt, maximum),
        (None, Some(minimum), None) => println!("{} (> {})", prompt, minimum),
        (None, Some(minimum), Some(maximum)) => {
            println!("{} (within {} - {} inc)", prompt, minimum, maximum)
        }
        (Some(default), None, None) => println!("{} (default {})", prompt, default),
    }

    let input = &mut String::new();
    std::io::stdin().read_line(input).unwrap();
    let input = input.trim();
    if input.is_empty() {
        return default;
    }
    match input.parse::<T>() {
        Ok(number) => {
            if let Some(minimum) = minimum {
                if number < minimum {
                    println!("Input too small. Aborting.");
                    return None;
                }
            }
            if let Some(maximum) = maximum {
                if number > maximum {
                    println!("Input too large. Aborting.");
                    return None;
                }
            }
            Some(number)
        }
        Err(_) => None,
    }
}

fn show_menu(options: Vec<&str>) -> usize {
    let stdin = std::io::stdin();
    let mut current: usize = 0;
    loop {
        options
            .iter()
            .enumerate()
            .for_each(|(index, option)| println!("{}. {}", index + 1, option));
        // Input
        let input = &mut String::new();
        stdin.read_line(input).unwrap();
        let input = input.trim();

        match input.parse::<usize>() {
            Ok(number) => {
                if number > 0 && number <= options.len() {
                    return number - 1;
                }
            }
            Err(_) => {}
        };

        let mut already_selected = false;
        for (index, option) in options.iter().enumerate() {
            if option.to_lowercase().contains(&input.to_lowercase()) {
                if already_selected {
                    println!("Ambiguous input. Please be more specific.");
                    already_selected = false;
                    break;
                }
                already_selected = true;
                current = index;
            }
        }

        if already_selected {
            return current;
        }
    }
}
