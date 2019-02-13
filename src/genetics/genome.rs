use crate::genetics::individual::Individual;
use crate::genetics::selector::Selector;
use rayon::prelude::*;

#[derive(Clone, Default)]
pub struct Genome {
    size: usize,
    population: Vec<Individual>
}

impl Genome {
    /// generates **size** random individuals and keep the best ones for reproduction
    pub fn new(size: usize) -> Genome{
        let mut population: Vec<Individual> = Vec::with_capacity(size);    
        population.par_extend((0..size).into_par_iter().map( |_i|  Individual::new() ));
        // score pop for the first time
        population.par_sort_by(|first, second| first.score().cmp(&second.score()));
        Genome {size, population}
    }

    /// creates an new generation by repeatedly breading the previous parents
    pub fn evolve(self, generations: usize, callback_interval: usize, callback: &Fn(usize, &Vec<Individual>)) -> Individual {
        let mut population: Vec<Individual> = self.population;
        let selector = Selector::new(&population);

        for i in 1..generations { // iterates over generations and breed !!!!

            // select parents for the next generation
            let parents = selector.select_parents(population);
            population = Vec::with_capacity(self.size);

            // include them in the pool
            population.push(parents.0.clone());
            population.push(parents.1.clone());

            // complete the rest of the pool
            let remaining = (self.size -2) / 2;
            population.par_extend((0..remaining).into_par_iter().flat_map(|_i| {
                Individual::breed(&parents.0, &parents.1)
            }));

            // sort the result by score
            population.par_sort_by(|first, second| first.score().cmp(&second.score()));
            
            if i % callback_interval == 0 {
                callback(i, &population);
            }
        }

        population[0].clone()
    }
}