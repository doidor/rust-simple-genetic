mod config;
mod individual;
mod population;

use std::io::{self, BufRead};

use population::Population;

fn main() {
	let stdin = io::stdin();

	println!("String to guess: ");
    let to_find = stdin.lock().lines().next().unwrap().unwrap();

    let mut population = Population::new(to_find);

    println!("{:?}", &population.find_best());
}
