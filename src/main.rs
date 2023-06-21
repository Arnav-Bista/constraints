use getopts::Options;
use std::env;
use nannou::prelude::*;

mod genetic_algorithm;
mod candidate;
mod city_writer;

use city_writer::*;

const HELP_MESSAGE : &str = "Genetic Algorithm - Solve TSP
    -h, --help\t- Show this help page
    -g, --generate <NUMBER>\t- Generates random cities in a specified file.\
    If no directory is specified, it will generate the cities in ./data
    -o, --output <STRING>\t- Specify output file.
    -i, --input <STRING>\t- Specify input file. If no file is specified, it \
    reads from ./data
    GENETIC PROCESS
    -p, --population <NUMBER>\t- Specify the population (Default is 5000)";

const DEFAULT_FILE : &str = "./data";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut generate_file = DEFAULT_FILE.to_string();
    let mut input_file = DEFAULT_FILE.to_string();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("o", "output", "set output file", "NAME");
    opts.optopt("g", "generate", "generate random cities", "number");
    opts.optopt("i","input","set input file", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        println!("{}", HELP_MESSAGE);
        return;
    }
    if matches.opt_present("o") {
        generate_file = matches.opt_str("o").unwrap();
    }
    if matches.opt_present("i") {
        input_file = matches.opt_str("i").unwrap();
    }
    if matches.opt_present("g") {
        let city_number: usize = match &matches.opt_str("g").expect("An argument is needed for -g").parse() {
            Ok(num) => *num,
            Err(_) => {
                println!("Invalid argument for -g");
                return;
            }
        };
        write_random_cities(generate_file, city_number);
        return;
    }
    let cities = match read_random_cities(input_file) {
        Ok(data) => data,
        Err(_) => return
    };
    let mut ga = genetic_algorithm::GaData::new(
        cities,
        5000,
        0.5,
        30
    );
    //
    // ga.set_active_plotting(true);
    // ga.run(3);

}

