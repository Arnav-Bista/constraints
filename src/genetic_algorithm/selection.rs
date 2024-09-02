use rand::Rng;

use super::candidate::Candidate;

/// Tournament selection
/// For a K way tournament selection, the best candidate has a probability of p to be picked
/// The second best has a probability of p * (p - 1) to be picked
/// The third best has a probability of p * (p - 1)^2 to be picked
/// and so on...
pub fn tournament_selection<T, U>(
    k: usize,
    selection_target: usize,
    population: &Vec<T>,
    best_pick_probability: f64,
    elitism_count: usize,
) -> Vec<T> where T: Candidate<U> + Clone {
    let mut rng = rand::thread_rng();
    let mut selected: Vec<T> = Vec::with_capacity(selection_target);

    // Elitism
    let mut population = population.clone();
    population.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());
    for i in 0..elitism_count {
        selected.push(population[i].clone());
    }

    for _ in 0..(selection_target - elitism_count) {
        let mut tournament: Vec<(T, f64)> = Vec::with_capacity(k);
        for _ in 0..k {
            let candidate = rng.gen_range(0..population.len());
            tournament.push((
                population[candidate].clone(),
                population[candidate].get_fitness(),
            ));
        }
        // Follow p * (p - 1)^i probability distribution
        // Where p is the probability to pick the best candidate
        tournament.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let mut cumulative = 0.0;
        for i in 0..tournament.len() {
            tournament[i].1 = best_pick_probability * (best_pick_probability - 1.0).powi(i as i32);
            tournament[i].1 += cumulative;
            cumulative += tournament[i].1;
        }
        let chosen = rng.gen::<f64>();
        selected.push(
            tournament
                .iter()
                .find(|(_, probability)| chosen < *probability)
                .unwrap()
                .0
                .clone(),
        );
    }

    selected
}
