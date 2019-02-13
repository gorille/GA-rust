mod genetics;

use genetics::genome;
use genetics::individual::Individual;
extern crate rayon;
extern crate termion;

fn main() {
    println!("Hello, world!");
    let genome = genome::Genome::new(150);
    let callback = | iteration: usize, population: &Vec<Individual> | {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 2));
        println!("generation {}, best score {}", iteration, population[0].score());
        for assigned in population[0].assignements.iter() {
            match assigned {
                None => print!("---|"),
                Some(worker) =>   print!("{:03}|", worker)
            }
        }
        println!("");
        for assigned in population[1].assignements.iter() {
            match assigned {
                None => print!("---|"),
                Some(worker) =>   print!("{:03}|", worker)
            }
        }
        println!("");
    };
    let best_indivual = genome.evolve(50_000, 1_000, &callback);
    println!("best individual: {:?}, score {}", best_indivual, best_indivual.score() );
}