extern crate rand;

use rand::prelude::*;

use super::target::{TO_FIND, GENES};

#[derive(Debug)]
pub struct Individual {
	genome: String,
	fitness: i32
}

impl Individual {
	pub fn new() -> Individual {
		let mut ind = Individual {
			genome: Default::default(),
			fitness: Default::default()
		};

		ind.genome = ind.create_genome();
		ind.fitness = ind.calculate_fitness();

		ind
	}

	fn mutate_genes(&self) -> String {
		let rand_pos = thread_rng().gen_range(0, GENES.len());

		String::from(GENES)[rand_pos..rand_pos + 1].to_string()
	}

	fn create_genome(&self) -> String {
		let mut ret: String = String::new();
		let mut count = TO_FIND.len();

		while count > 0 {
			ret.push_str(&self.mutate_genes());

			count -= 1;
		}

		ret
	}

	fn calculate_fitness(&self) -> i32 {
		let mut c1: char;
		let mut c2: char;

		let mut fitness: i32 = 0;

		for i in 0..TO_FIND.len() {
			c1 = String::from(TO_FIND).chars().nth(i as usize).unwrap();
			c2 = self.genome.chars().nth(i as usize).unwrap();

			if c1 != c2 {
				fitness += 1;
			}
		}

		fitness
	}
}