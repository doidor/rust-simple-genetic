mod config;
mod individual;
mod population;

use population::Population;

fn main() {
    let mut population = Population::new();

    println!("{:?}", &population.find_best());
}
