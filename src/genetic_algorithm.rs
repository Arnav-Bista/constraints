use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::prelude::*;
use rand_distr::WeightedAliasIndex;
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
                    f32::powi(cities[*index].0 as f32 - prev.0 as f32, 2) + 
                    f32::powi(cities[*index].1 as f32 - prev.1 as f32, 2));
                prev = cities[*index];
            }
        }
        self.fitness = self.fitness / 10000.0;
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
    
    // More of a Weighted Random Selection
    fn roulette_wheel_selection(&mut self) -> Vec<Candidate> {
        let mut res: Vec<Candidate> = Vec::new();
        let mut weights = vec![0.0;self.population_count as usize];
        for (i,candidate) in self.population.iter().enumerate() {
            weights[i] = candidate.fitness();
        }

        let dist = WeightedAliasIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();
        for _ in 0..self.population_count / 4 {
            res.push(self.population[dist.sample(&mut rng)].clone());
        }

        res
    }

    fn order_crossover(&mut self, parent_1: &Candidate, parent_2: &Candidate) -> Candidate {
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
        // offspring.chromozones = (0..self.cities.len()).collect();
        offspring.chromozones = vec![99; self.cities.len()];
        
        for k in i..j {
            offspring.chromozones[k] = parent_1.chromozones[k];
            hashset.insert(offspring.chromozones[k]);
        }
        let mut child_index = 0;
        // println!("{:?}", parent_2.chromozones);
        // println!("===");
        // println!("{:?}", offspring.chromozones);
        for k in &parent_2.chromozones {
            // println!("{:?} {} {}",offspring.chromozones, k, hashset.contains(&k));
            if !hashset.contains(&k) {
                if i <= child_index && child_index < j  {
                    child_index = j;
                }
                offspring.chromozones[child_index] = *k;
                // if child_index == 10 {
                //     println!("{:?} {} {}",offspring.chromozones, k, hashset.contains(&k));
                // }
                child_index += 1;
            }
        }
        offspring.calcualte_fitness(&self.cities);
        offspring
    }

    fn mutate(&mut self, offspring: &mut Candidate) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < self.mutation_rate {
            let mut i: usize;
            let mut j: usize;
            loop {
                i = rng.gen_range(0..self.cities.len());
                j = rng.gen_range(0..self.cities.len());
                if i != j {
                    break;
                }
            }
            offspring.chromozones.swap(i, j);
        }
    }

    pub fn run(&mut self, iteration_limit: u32) {
        self.populate();
        let mut rng = rand::thread_rng();
        println!("Initial Fitness:\t{}",self.all_time_best.fitness());
        for i in 0..iteration_limit {
            self.iteration = i;
            self.quick_sort(0, (self.population.len() - 1) as isize);
            println!("Gen {}\t Current {}\tBest {}", i, self.population[0].fitness(), self.all_time_best.fitness());
            if self.population[0].fitness() > self.all_time_best.fitness() {
                self.all_time_best = self.population[0].clone();
            }
            let selection = self.roulette_wheel_selection();
            self.population = Vec::new();
            for _ in 0..4 {
                for __ in 0..selection.len() {
                    let mut offspring = self.order_crossover(selection.choose(&mut rng).unwrap(),selection.choose(&mut rng).unwrap());
                    self.mutate(&mut offspring);
                    self.population.push(offspring);
                }
            }
        }
    }
}


