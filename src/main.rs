use getopts::Options;
use std::env;
use nannou::{prelude::*, text::Font};



mod genetic_algorithm;
mod candidate;
mod city_writer;

use city_writer::*;

const HELP_MESSAGE : &str = "Genetic Algorithm - Solve TSP
OPTIONS
    -h, --help\t- Show this help page
    -g, --generate <NUMBER>\t- Generates random cities in a specified file.\
    If no directory is specified, it will generate the cities in ./data
    -o, --output <STRING>\t- Specify output file.
    -i, --input <STRING>\t- Specify input file. If no file is specified, it \
    reads from ./data
GENETIC PROCESS
    -p, --population <NUMBER>\t- Specify the population (Default is 5000).
    -m, --mutation <FLOAT>\t- Specify the mutation rate (Default is 0.5).
    -t, --truncation <NUMBER>\t- Specify the truncation limit (Default if 30). This is % of the population that will be selected\
    to populate the next generation
UI CONTROLS
    <SPACE>\t- Pause and unpause
    <ESC>\t- Terminate the program
    s\t- Toggle explorative and exploitative [s]election.
    r\t- Toggle explorative and exploitative [r]epopulation.
    i\t- [I]terate. Iterate by one step. Only works while paused.";

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
    screen_y: f32,
    pause: bool
}

fn model(_app: &App) -> Model {
    let args: Vec<String> = env::args().collect();
    let mut generate_file = DEFAULT_FILE.to_string();
    let mut input_file = DEFAULT_FILE.to_string();
    let mut opts = Options::new();

    let mut population: u32 = 5000;
    let mut mutation: f32 = 0.5;
    let mut truncation: u32 = 30;

    opts.optflag("h", "help", "print this help menu");
    opts.optopt("o", "output", "set output file", "NAME");
    opts.optopt("g", "generate", "generate random cities", "number");
    opts.optopt("i","input","set input file", "NAME");
    opts.optopt("p","population","set population", "number");
    opts.optopt("m","mutation","set mutation", "FLOAT");
    opts.optopt("t","truncation","set truncation", "NUMBER");
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
    
    if matches.opt_present("p") {
         population = match &matches.opt_str("p").expect("An argument is needed for -p").parse() {
            Ok(num) => *num,
            Err(_) => {
                println!("Invalid argument for -p");
                std::process::exit(1);
            }
        };
    }

    if matches.opt_present("m") {
         mutation = match &matches.opt_str("m").expect("An argument is needed for -m").parse() {
            Ok(num) => *num,
            Err(_) => {
                println!("Invalid argument for -m");
                std::process::exit(1);
            }
        };
    }
  
    if matches.opt_present("t") {
         truncation = match &matches.opt_str("t").expect("An argument is needed for -t").parse() {
            Ok(num) => *num,
            Err(_) => {
                println!("Invalid argument for -t");
                std::process::exit(1);
            }
        };
    }

    let cities = match read_random_cities(input_file) {
        Ok(data) => data,
        Err(_) =>  std::process::exit(1)
    };
    let mut ga = genetic_algorithm::GaData::new(
        cities,
        population,
        mutation,
        truncation
    );

    ga.populate();
    
    let screen_x = 800.0;
    let screen_y = 600.0;

    _app.new_window()
        .size(screen_x as u32,screen_y as u32)
        .title("Genetic Algorithm")
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model{
        ga,
        screen_x,
        screen_y,
        pause: true 
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if !_model.pause {
        _model.ga.iterate();
    }
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
            .weight(3.0)
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
        .weight(3.0)
        .color(RED);

    // text
    let margin_top = 20.0;
    let margin_left = 110.0;

    let font_size = 16;
    let line_height = font_size as f32 * 1.2;
    let top_left = pt2(_model.screen_x / -2.0 + margin_left, _model.screen_y / 2.0 - margin_top);

    
    if _model.pause {
        draw.text("Paused")
        .font_size(font_size)
        .left_justify()
        .xy(top_left)
        .color(BLACK);
    }

    draw.text(format!("Gen: {}", _model.ga.get_iteration()).as_str())
        .font_size(font_size)
        .left_justify()
        .xy(top_left + pt2(0.0, -line_height))
        .color(BLACK);
    
    let selection: &str;
    let repopulation: &str;

    if _model.ga.get_selection_strategy() {
        selection = "Selection: Truncation [Exploitative]"
    }
    else {
        selection = "Selection: Roulette Wheel [Explorative]"
    }

    if _model.ga.get_repopulation_strategy() {
        repopulation = "Repopulation: Exploitative"
    }
    else {
        repopulation = "Repopulation: Explorative"
    }
    
    draw.text(selection)
        .font_size(font_size)
        .left_justify()
        .no_line_wrap()
        .xy(top_left + pt2(0.0, -2.0 * line_height))
        .color(BLACK);

    draw.text(repopulation)
        .font_size(font_size)
        .left_justify()
        .xy(top_left + pt2(0.0, -3.0 * line_height))
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

fn normalize(unit: u32, scale: f32) -> f32 {
    scale * 0.8 * (unit as f32) / 100.0 + (-0.8 * scale / 2.0)
}

fn normalize_point(_model: &Model, point: (u32, u32)) -> (f32,f32) {
    (normalize(point.0, _model.screen_x), normalize(point.1, _model.screen_y))
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space=> {
            // Perform actions when Space key is pressed
            model.pause = !model.pause
        }
        Key::Escape => {
            // Perform actions when Escape key is pressed
            app.quit();
        }
        Key::R => model.ga.toggle_repopulation_strategy(),
        Key::S => model.ga.toggle_selection_strategy(),
        Key::I => {
            if model.pause {
                model.ga.iterate();
            }
        }
        _ => (),
    }
}
