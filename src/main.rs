use std::{env, collections};
use std::fs::File;
use std::io::{BufWriter, BufReader, Write, Error, BufRead};
use std::collections::HashSet;
use rand::Rng;
use getopts::Options;
use colored::Colorize;
use plotters::prelude::*;

mod genetic_algorithm;

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
    ga.run(130);

    let root_area = BitMapBackend::new("./res.png", (600, 400))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Solution", ("sans-serif", 40))
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
    
    ga.prepare_graph_data();

    ctx.draw_series(LineSeries::new(
        ga.get_all_time_best().chromozones.iter().map(|x| ga.get_city(*x)),
        &BLUE,
    ))
    .unwrap();

    
}

fn write_random_cities(file: String, number: usize) {
    let mut rng = rand::thread_rng();
    let file = match File::create(file) {
        Ok(file) => file,
        Err(_) => {
            println!("Could not create file Aborting.");
            return;
        }
    };
    let mut buffer = BufWriter::with_capacity(number, file);
    let mut hashset: collections::HashSet<(u32,u32)> = collections::HashSet::new();
    let mut i = 0;
    while i < number {
        let numbers = (rng.gen_range(0..100),rng.gen_range(0..100));
        if hashset.contains(&numbers) {
            continue;
        }
        hashset.insert(numbers);
        let line = format!("{},{}\n",rng.gen_range(0..100),rng.gen_range(0..100));
        match buffer.write(line.as_bytes()) {
            Ok(_) => (),
            Err(err) => {
                println!("Error while writing to file. Aborting.\n{}", err.to_string());
                return;
            }
        }
        i += 1;
    }
    println!("{}", "Done!".green());
}

fn read_random_cities(file: String) -> Result<Vec<(u32,u32)>, Error> {
    let file = match File::open(file) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file. Aborting.\n{}", err.to_string());
            return Err(err);
        }
    };
    let buffer = BufReader::new(file);
    let mut result_vec: Vec<(u32, u32)> = Vec::new();

    for (i,line) in buffer.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_) => { 
                println!("Couldn't parse line {}. Skipping.", i); 
                continue;
            }
        };
        let line = line.split(",");
        let mut array: [u32;2] = [0;2];
        let mut parsing_error: bool = false;
        for (j,num) in line.enumerate() {
            let num: u32 = match num.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Couldn't parse line {} into u32. Skipping",i);
                    parsing_error = true;
                    0
                }
            };
            array[j] = num;
        }
        if parsing_error {
            continue;
        }
        result_vec.push((array[0],array[1]));
    }


    Ok(result_vec)
}


