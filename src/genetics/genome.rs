use crate::genetics::individual::Individual;

pub struct Genome {
    size: u16,
    father: Individual,
    mother: Individual,
}

impl Genome {
    /// generates **size** random individuals and keep the best ones for reproduction
    pub fn new(size: u16) -> Genome{
        let mut remaining = size - 2;
        let mut father = Individual::new();
        let mut mother = Individual::new();

        let mut father_score = father.score();
        let mut mother_score = mother.score();

        while remaining != 0 {
            let individual = Individual::new();
            let score = individual.score();
            if score < father_score {
                mother = father;
                father = individual;
                father_score = score;
                mother_score = father_score;
            } else if score < mother_score {
                mother = individual;
                mother_score = score;
            }
            remaining = remaining -1;
        }

        Genome {size, father, mother}
    }

    /// creates an new generation by repeatedly breading the previous parents
    pub fn evolve(self, generations: u16, callback_interval: u16) -> Individual {
        let mut current_gen = self;
        let mut remaining = generations;
        while remaining > 0 {
            remaining = remaining -1;
            //current_gen = Genome::breed(current_gen);
        }
        current_gen.father
    }

    fn breed(current_gen: Genome) -> Individual {
        unimplemented!();
    }

    fn accumulate(start: Genome, generator: Individual) {
        unimplemented!();
    }
}