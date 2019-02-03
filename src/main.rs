mod genetics;

use genetics::genome;

fn main() {
    println!("Hello, world!");
    let genome = genome::Genome::new(80u16);
    let best_indivual = genome.evolve(500u16, 10u16);
    println!("best individual: {:?}", best_indivual );
}
