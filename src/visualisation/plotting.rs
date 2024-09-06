use crate::{
    genetic_algorithm::{candidate::Candidate, genetic_algorithm::{SelectionMethod, GA}},
    tsp::TspCandidate,
};

use super::egui_struct::RealTimePlotTSP;

pub fn generation_fitness_plot_tsp(
    ga: GA<TspCandidate, Vec<(f64, f64)>>,
    iterations: usize,
    selection_method: SelectionMethod,
) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 400.0]),
        ..Default::default()
    };

    let rtp = RealTimePlotTSP::new(ga.clone(), iterations, selection_method);

    eframe::run_native(
        "Genetic Algorithm",
        options,
        Box::new(|_cc| Ok(Box::new(rtp))),
    );
}
