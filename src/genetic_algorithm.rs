use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::collections::{HashSet, self};

#[derive(Clone)]
pub struct Candidate {
    pub chromozones: Vec<usize>,
    fitness: f32
}

impl Candidate {

    pub fn new(cities: &Vec<(u32,u32)>) -> Self {
        let mut chromozones: Vec<usize> = (0..cities.len()).collect();
        chromozones.shuffle(&mut thread_rng());
        let mut candidate = Self {
            chromozones,
            fitness: 0.0
        };
        candidate.calcualte_fitness(cities);
        candidate
    }

    pub fn empty() -> Self {
        Self {
            chromozones: Vec::new(),
            fitness: 0.0
        }
    }

    pub fn fitness(&self) -> f32 {
        self.fitness
    }

    pub fn calcualte_fitness(&mut self, cities: &Vec<(u32,u32)>) -> f32 {
        self.fitness = 0.0;
        let mut prev: (u32,u32) = (0,0);
        let mut prev_updated = false;
        for index in &self.chromozones {
            if !prev_updated {
                prev = cities[*index];
                prev_updated = true;
            }
            else {
                self.fitness += f32::sqrt(
                    f32::powi((cities[*index].0 - prev.0) as f32, 2) - 
                    f32::powi((cities[*index].0 - prev.0) as f32, 2));
            }
        }
        self.fitness = 1.0 / self.fitness;
        self.fitness
    }
}

pub struct GaData {
    cities: Vec<(u32,u32)>,
    iteration: u32,
    current_best: Candidate,
    all_time_best: Candidate, 
    population_count: u32,
    population: Vec<Candidate>,
    mutation_rate: f32,
    truncation: u32
}

impl GaData {
    pub fn new(cities: Vec<(u32,u32)>, population_count: u32, mutation_rate: f32, truncation: u32) -> Self {
        Self {
            cities,
            iteration: 0,
            current_best: Candidate::empty(),
            all_time_best: Candidate::empty(),
            population_count,
            population: Vec::new(),
            mutation_rate,
            truncation
        }
    }

    pub fn quick_sort(&mut self, low: isize, high: isize) {
        if low < high {
            let p = self.partition(low,high);
            self.quick_sort(low, p - 1);
            self.quick_sort(p + 1, high);
        }
    }

    fn partition(&mut self, low: isize, high: isize) -> isize {
        let pivot = high as usize;
        let mut store_index = low - 1;
        let mut last_index = high;

        loop {
            store_index += 1;
            while self.population[store_index as usize].fitness < self.population[pivot].fitness() {
                store_index += 1;
            }
            last_index -= 1;
            while last_index >= 0 && self.population[last_index as usize].fitness() > self.population[pivot].fitness() {
                last_index -= 1;
            }
            if store_index >= last_index {
                break;
            } else {
                self.population.swap(store_index as usize, last_index as usize);
            }
        }
        self.population.swap(store_index as usize, pivot as usize);
        store_index
    }

    fn populate(&mut self) {
        for i in 0..self.population_count {
            self.population.push(Candidate::new(&self.cities));
            if self.population[i as usize].fitness() > self.current_best.fitness() {
                self.current_best = self.population[i as usize].clone();
            }
        }
        self.all_time_best = self.current_best.clone();
    }

    fn mate(&mut self, parent_1: Candidate, parent_2: Candidate) -> Candidate {
        // Order Crossover
        let mut rng = rand::thread_rng();
        let mut i: usize;
        let mut j: usize;
        loop {
            i = rng.gen_range(0..self.cities.len());
            j = rng.gen_range(0..self.cities.len());
            if i != j {
                break;
            }
        }
        if j < i {
            let temp = i;
            i = j;
            j = temp;
        }

        let mut offspring = Candidate::empty();
        let mut hashset: collections::HashSet<usize> = collections::HashSet::new();
        offspring.chromozones = (0..self.cities.len() - 1).collect();
        
        for k in i..j {
            offspring.chromozones[k] = parent_1.chromozones[k];
            hashset.insert(offspring.chromozones[k]);
        }

        offspring

    }

    pub fn run(&mut self, iteration_limit: u32) {
        self.populate();
        for i in 0..iteration_limit {
            self.iteration = i;
            self.quick_sort(0, (self.population.len() - 1) as isize);


        }
    }
}


