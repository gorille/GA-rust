use crate::genetics::individual::Individual;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

pub struct Selector {
    index: WeightedIndex<usize>
}

impl Selector {
    pub fn new(population: &Vec<Individual>) -> Selector {
        let weights: Vec<usize> = (0..population.len()).into_iter().map(|i| population.len() - i).collect();
        Selector {index: WeightedIndex::new(&weights).unwrap()}
    }
    pub fn select_parents(&self, population: Vec<Individual>) -> (Individual, Individual) {
        let mut rng = thread_rng();
        let mut second = self.index.sample(&mut rng);
        while second == 0 { 
            second = self.index.sample(&mut rng);
        }

        (population[0].clone(), population[second].clone())
    }
}
