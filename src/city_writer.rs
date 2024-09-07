use std::{collections, f64};
use std::fs::File;
use std::io::{BufWriter, BufReader, Write, Error, BufRead};
use rand::Rng;
use colored::Colorize;

pub fn write_random_cities(file: String, number: usize, max_coords: u32) {
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
        let numbers = (rng.gen_range(0..max_coords),rng.gen_range(0..max_coords));
        if hashset.contains(&numbers) {
            continue;
        }
        hashset.insert(numbers);
        let line = format!("{},{}\n",numbers.0,numbers.1);
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

pub fn read_random_cities(file: String) -> Result<Vec<(f64,f64)>, Error> {
    let file = match File::open(file) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file. Aborting.\n{}", err.to_string());
            return Err(err);
        }
    };
    let buffer = BufReader::new(file);
    let mut result_vec: Vec<(f64, f64)> = Vec::new();

    for (i,line) in buffer.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_) => { 
                println!("Couldn't parse line {}. Skipping.", i); 
                continue;
            }
        };
        let line = line.split(",");
        let mut array: [f64;2] = [0.0;2];
        let mut parsing_error: bool = false;
        for (j,num) in line.enumerate() {
            let num: f64 = match num.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Couldn't parse line {} into f64. Skipping",i);
                    parsing_error = true;
                    0.0
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
