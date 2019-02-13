use rand::Rng;

#[derive(Debug)]
#[derive(Clone)]
pub struct Individual {
    pub assignements: Vec<Option<u8>>
}


impl Individual {
    /// computes the score for a given individual
    /// returns the score of this particular indivitual
    pub fn score(&self) -> u16 {
        let mut score = 0u16;
        for assigned in self.assignements.iter() {
            match assigned {
                None => score = score + 500u16,
                Some(worker) =>  score = score + *worker as u16
            }
        }
        score
    }

    /// genereates a new random individual
    pub fn new() -> Individual {
        let size = 20;
        let mut assignements: Vec<Option<u8>> = vec![None; size];
        let mut remain = size -1;
        while remain != 0 {
            if rand::random() { 
                assignements[remain] = Some(rand::random::<u8>());
            } 
            remain = remain -1;
        }
        Individual { assignements}
    }

    // breeds 2 Individual by cross over and mutation
    pub fn breed(father: &Individual, mother: &Individual) -> Vec<Individual> {

        let mut rng = rand::thread_rng();
        let worker_len = father.assignements.len();
        let cross_point = rng.gen_range(0, worker_len);
        let mut son: Vec<Option<u8>> = Vec::with_capacity(worker_len);
        let mut daugther: Vec<Option<u8>> = Vec::with_capacity(worker_len);

        for i in 0..worker_len { // copies genes according to order
            let father = father.assignements[i];
            let mother = mother.assignements[i];
            if i <= cross_point {
                son.push(father);
                daugther.push(mother);
            } else {
                son.push(mother);
                daugther.push(father);
            }
        }

        vec![(Individual { assignements: son }).mutate(), (Individual { assignements: daugther }).mutate()]
    }

    pub fn mutate(mut self) -> Individual {
         use rand::seq::IteratorRandom;
        let mut rng = rand::thread_rng();
        let changed = (0..self.assignements.len()).choose_multiple(&mut rng, 7);

        for i in changed {
            self.assignements[i] = Some(rand::random::<u8>());
        }
        self
    }
}