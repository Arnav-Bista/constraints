use egui::Vec2;
use egui_plot::{Corner, Legend, Line, Plot, PlotItem, PlotPoints};

use crate::{
    genetic_algorithm::{
        candidate::Candidate,
        genetic_algorithm::GA, selection::SelectionMethod,
    },
    tsp::TspCandidate,
};

pub struct RealTimePlotTSP {
    ga: GA<TspCandidate, Vec<(f64, f64)>>,
    iterations: usize,
    last_iteration: usize,
    ga_data: Vec<(f64, f64, f64)>,
    selection_method: SelectionMethod
}

impl RealTimePlotTSP {
    const SIZE: (f32, f32) = (800.0, 400.0);

    pub fn new(ga: GA<TspCandidate, Vec<(f64, f64)>>, iterations: usize, selection_method: SelectionMethod) -> Self {
        RealTimePlotTSP {
            ga,
            iterations,
            last_iteration: 0,
            ga_data: Vec::with_capacity(iterations),
            selection_method
        }
    }
}

impl eframe::App for RealTimePlotTSP {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let data = self.ga.step(SelectionMethod::RouletteWheel);
        self.last_iteration += 1;
        self.ga_data.push(data);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("Genetic Algorithm");
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.heading(format!("Generation: {}", self.last_iteration));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("grid").show(ui, |ui| {
                ui.label("Fitness, Std Deviation and Best - Mean Fitness");
                ui.label("Best Solution");
                ui.end_row();

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        let plot = Plot::new("fitnessplot");
                        plot.legend(Legend::default().position(Corner::LeftTop))
                            .allow_drag(false)
                            .allow_drag(false)
                            .show_axes(true)
                            .min_size(Vec2::new(Self::SIZE.0 / 2.0, Self::SIZE.1))
                            .show(ui, |plot_ui| {
                                plot_ui.line(
                                    Line::new(PlotPoints::new(
                                        self.ga_data
                                            .iter()
                                            .enumerate()
                                            .map(|(generation, data)| [generation as f64, data.0])
                                            .collect(),
                                    ))
                                    .name("Best Fitness"),
                                );
                                plot_ui.line(
                                    Line::new(PlotPoints::new(
                                        self.ga_data
                                            .iter()
                                            .enumerate()
                                            .map(|(generation, data)| [generation as f64, data.1])
                                            .collect(),
                                    ))
                                    .name("Std Deviation"),
                                );
                                plot_ui.line(
                                    Line::new(PlotPoints::new(
                                        self.ga_data
                                            .iter()
                                            .enumerate()
                                            .map(|(generation, data)| [generation as f64, data.2])
                                            .collect(),
                                    ))
                                    .name("Mean Fitness"),
                                );
                            });
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        let best = self.ga.best();
                        let mut best_chromo = best.get_chromosome().clone();
                        best_chromo.push(best_chromo[0]);
                        let plot = Plot::new("bestplot");
                        plot.allow_drag(false)
                            .allow_drag(false)
                            .show_axes(true)
                            .min_size(Vec2::new(Self::SIZE.0 / 2.0, Self::SIZE.1))
                            .show(ui, |plot_ui| {
                                plot_ui.line(
                                    Line::new(PlotPoints::new(
                                        best_chromo.iter().map(|(x, y)| [*x, *y]).collect(),
                                    ))
                                    .name("Best Solution"),
                                );
                            });
                    });
                });
            });
        });

        ctx.request_repaint();
    }
}
