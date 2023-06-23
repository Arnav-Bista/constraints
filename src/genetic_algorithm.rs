use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::prelude::*;
use rand_distr::WeightedAliasIndex;



use crate::candidate::Candidate;

pub struct GaData {
    cities: Vec<(u32,u32)>,
    iteration: u32,
    current_best: Candidate,
    all_time_best: Candidate, 
    population_count: u32,
    population: Vec<Candidate>,
    mutation_rate: f32,
    truncation: u32,
    rng: ThreadRng,
    exploit_selection: bool,
    exploit_repopulation: bool,
}

impl GaData {
    pub fn new(cities: Vec<(u32,u32)>, population_count: u32, mutation_rate: f32, truncation: u32) -> Self {
        let rng = rand::thread_rng();
        Self {
            cities,
            iteration: 0,
            current_best: Candidate::empty(),
            all_time_best: Candidate::empty(),
            population_count,
            population: Vec::new(),
            mutation_rate,
            truncation,
            rng,
            exploit_selection: true,
            exploit_repopulation: false
        }
    }

    pub fn get_iteration(&self) -> u32 {
        self.iteration
    }

    pub fn get_city_size(&self) -> usize {
        self.cities.len()
    }

    
    pub fn get_best_chromozone(&self, index: usize) -> (u32,u32) {
        self.cities[self.all_time_best.chromozones[index]]    
    }

    pub fn get_current_chromozone(&self, index: usize) -> (u32,u32) {
        self.cities[self.current_best.chromozones[index]]    
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
            while self.population[store_index as usize].fitness() < self.population[pivot].fitness() {
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

    pub fn populate(&mut self) {
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
        let mut res: Vec<Candidate> = Vec::with_capacity((self.population_count * self.truncation / 100) as usize);
        let mut weights = vec![0.0;self.population_count as usize];
        for (i,candidate) in self.population.iter().enumerate() {
            weights[i] = candidate.fitness();
        }

        let dist = WeightedAliasIndex::new(weights).unwrap();
        // Onle % of the population selected
        for _ in 0..self.population_count * self.truncation / 100 {
            res.push(self.population[dist.sample(&mut self.rng)].clone());
        }
        res
    }

    fn truncation_selection(&mut self) -> Vec<Candidate> {
        let length = self.population_count * self.truncation / 100;
        let mut res: Vec<Candidate> = Vec::with_capacity(length as usize);
        for i in 0..length {
            res.push(self.population[self.population_count as usize - 1 - i as usize].clone());
        }
        res
    }

    fn order_crossover(&mut self, parent_1: &Candidate, parent_2: &Candidate) -> Candidate {
        let mut i: usize;
        let mut j: usize;
        loop {
            i = self.rng.gen_range(0..self.cities.len());
            j = self.rng.gen_range(0..self.cities.len());
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
        let mut hashset: HashSet<usize> = HashSet::new();
        offspring.chromozones = vec![99; self.cities.len()];

        for k in i..j {
            offspring.chromozones[k] = parent_1.chromozones[k];
            hashset.insert(offspring.chromozones[k]);
        }
        let mut child_index = 0;
        for k in &parent_2.chromozones {
            if !hashset.contains(&k) {
                if i <= child_index && child_index < j  {
                    child_index = j;
                }
                offspring.chromozones[child_index] = *k;
                child_index += 1;
            }
        }
        offspring.calcualte_fitness(&self.cities);
        offspring
    }

    fn mutate(&mut self, offspring: &mut Candidate) {
        if self.rng.gen::<f32>() < self.mutation_rate {
            let mut i: usize;
            let mut j: usize;
            loop {
                i = self.rng.gen_range(0..self.cities.len());
                j = self.rng.gen_range(0..self.cities.len());
                if i != j {
                    break;
                }
            }
            offspring.chromozones.swap(i, j);
        }
    }

    fn mutate_parent(&mut self, offspring: &mut Candidate) {
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
        offspring.chromozones.swap(i, j);
    }

    pub fn exploitative_repopulation(&mut self, selection: Vec<Candidate>) {
        for _ in 0..self.population_count * (100 - self.truncation) / 100 {
            // Fill the remaining 1 - turncation% of the population with offsprings
            let parent_1 = selection.choose(&mut self.rng).unwrap();
            let parent_2 = selection.choose(&mut self.rng).unwrap();
            let mut offspring = self.order_crossover(parent_1, parent_2);
            self.mutate(&mut offspring);
            self.population.push(offspring);
        }
        // Then add the parent back
        for mut selected in selection {
            self.mutate_parent(&mut selected);
            self.population.push(selected);
        }
        // If there are any space left
        for _ in self.population.len()..self.population_count as usize {
            self.population.push(Candidate::new(&self.cities));
        }
    }

    pub fn explorative_repopulation(&mut self, selection: Vec<Candidate>) {
        for _ in 0..self.population_count {
            let parent_1 = selection.choose(&mut self.rng).unwrap();
            let parent_2 = selection.choose(&mut self.rng).unwrap();
            let mut offspring = self.order_crossover(parent_1, parent_2);
            // let mut offspring = self.cycle_crossover(parent_1, parent_2);
            self.mutate(&mut offspring);
            self.population.push(offspring);
        }
    }

    pub fn toggle_selection_strategy(&mut self) {
        self.exploit_selection = !self.exploit_selection;
    }

    pub fn toggle_repopulation_strategy(&mut self) {
        self.exploit_repopulation = !self.exploit_repopulation;
    }

    pub fn get_selection_strategy(&self) -> bool {
        self.exploit_selection 
    }

    pub fn get_repopulation_strategy(&self) -> bool {
        self.exploit_repopulation 
    }

    pub fn iterate(&mut self) {
        self.iteration += 1;
        self.quick_sort(0, self.population_count as isize - 1);
        println!(
            "Gen {}\t Current {:.8}\tBest {:.8}",
            self.iteration,
            self.population[self.population_count as usize - 1].fitness(), 
            self.all_time_best.fitness()
        );

        self.current_best = self.population[self.population_count as usize - 1].clone();

        if self.population[self.population_count as usize - 1].fitness() > self.all_time_best.fitness() {
            self.all_time_best = self.population[self.population_count as usize - 1].clone();
        }
        // else {
        //     self.population[self.population_count as usize - 1] = self.all_time_best.clone();
        // }
    
        let selection: Vec<Candidate>;

        if self.exploit_selection {
            selection = self.truncation_selection();
        }
        else {
            selection = self.roulette_wheel_selection();
        }
        self.population = Vec::with_capacity(self.population_count as usize);

        if self.exploit_repopulation {
            self.exploitative_repopulation(selection);
        }
        else {
            self.explorative_repopulation(selection);
        }
    }
    
    pub fn run(&mut self, iteration_limit: u32) {
        self.populate();

        println!("Initial Fitness:\t{}",self.all_time_best.fitness());
        for i in 0..iteration_limit {
            self.iteration = i;
            self.quick_sort(0, (self.population.len() - 1) as isize);

            println!(
                "Gen {}\t Current {:.8}\tBest {:.8}",
                i,
                self.population[self.population_count as usize - 1].fitness(), 
                self.all_time_best.fitness()
            );

            if self.population[self.population_count as usize - 1].fitness() > self.all_time_best.fitness() {
                self.all_time_best = self.population[self.population_count as usize - 1].clone();
            }
            else {
                self.population[self.population_count as usize - 1] = self.all_time_best.clone();
            }

            let selection: Vec<Candidate>;
            selection = self.truncation_selection();
            // selection = self.roulette_wheel_selection();
            self.population = Vec::with_capacity(self.population_count as usize);
            // self.exploitative_repopulation(selection);
            self.explorative_repopulation(selection);
        }
    }
}

