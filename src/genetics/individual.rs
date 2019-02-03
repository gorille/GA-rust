use rand::prelude::*;

#[derive(Debug)]
pub struct Individual {
    assignements: Vec<Option<u8>>
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
        let mut size = 20;
        let mut assignements: Vec<Option<u8>> = Vec::new();

        while size != 0 {
            if rand::random() { 
                assignements.push(Some(rand::random::<u8>()));
            } else {
                assignements.push(None);
            }
            size =  size -1;
        }
        Individual { assignements}
    }
}