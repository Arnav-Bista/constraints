use getopts::Options;
use std::env;
use nannou::prelude::*;
use std::process;


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
    //
    // ga.set_active_plotting(true);
    // ga.run(3);
    //
    nannou::app(model)
        .update(update)
        .view(view)
        .run();

}


struct Model{
    ga: genetic_algorithm::GaData,
    screen_x: f32,
    screen_y: f32
}

fn model(_app: &App) -> Model {
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
        std::process::exit(1);
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
                std::process::exit(1);
            }
        };
        write_random_cities(generate_file, city_number);
        std::process::exit(1);
    }
    let cities = match read_random_cities(input_file) {
        Ok(data) => data,
        Err(_) =>  std::process::exit(1)
    };
    let mut ga = genetic_algorithm::GaData::new(
        cities,
        5000,
        0.5,
        30
    );

    ga.populate();
    
    let screen_x = 800.0;
    let screen_y = 600.0;

    _app.new_window()
        .size(screen_x as u32,screen_y as u32)
        .title("Genetic Algorithm")
        .build()
        .unwrap();

    Model{
        ga,
        screen_x,
        screen_y
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.ga.iterate();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    draw.background().color(WHITESMOKE);
    let mut start_best =  normalize_point(_model, _model.ga.get_best_chromozone(0));
    let mut start_current =  normalize_point(_model, _model.ga.get_current_chromozone(0));
    
    draw.ellipse()
        .x_y(start_best.0, start_best.1)
        .radius(5.0)
        .color(BLACK);

    for i in 1.._model.ga.get_city_size() {
        let point_best = normalize_point(_model, _model.ga.get_best_chromozone(i));
        let point_current = normalize_point(_model, _model.ga.get_current_chromozone(i));
        draw.ellipse()
            .x_y(
                point_best.0, 
                point_best.1
            )
            .radius(5.0)
            .color(BLACK);

        draw.line()
            .start(start_current.into())
            .end(point_current.into())
            .weight(2.0)
            .color(BLUE);
        start_current = point_current;

        draw.line()
            .start(start_best.into())
            .end(point_best.into())
            .weight(4.0)
            .color(RED);
        start_best = point_best;




    }

    draw.line()
        .start(start_current.into())
        .end(normalize_point(_model, _model.ga.get_current_chromozone(0)).into())
        .weight(2.0)
        .color(BLUE);

    draw.line()
        .start(start_best.into())
        .end(normalize_point(_model, _model.ga.get_best_chromozone(0)).into())
        .weight(4.0)
        .color(RED);




    draw.to_frame(app, &frame).unwrap();
}

fn normalize(unit: u32, scale: f32) -> f32 {
    scale * (unit as f32) / 100.0 + (-1.0 * scale / 2.0)
}

fn normalize_point(_model: &Model, point: (u32, u32)) -> (f32,f32) {
    (normalize(point.0, _model.screen_x), normalize(point.1, _model.screen_y))
}
