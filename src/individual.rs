extern crate rand;

use rand::prelude::*;

use super::config::{GENES, TO_FIND};

#[derive(Debug, Clone)]
pub struct Individual {
	pub chromosome: String,
	pub fitness: i32,
}

impl Individual {
	pub fn new() -> Individual {
		let mut ind = Individual {
			chromosome: Default::default(),
			fitness: Default::default(),
		};

		ind.chromosome = ind.create_genome();
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

	pub fn calculate_fitness(&self) -> i32 {
		let mut c1: char;
		let mut c2: char;

		let mut fitness: i32 = 0;

		for i in 0..TO_FIND.len() {
			c1 = String::from(TO_FIND).chars().nth(i as usize).unwrap();
			c2 = self.chromosome.chars().nth(i as usize).unwrap();

			if c1 != c2 {
				fitness += 1;
			}
		}

		fitness
	}

	pub fn mate(&self, another_individual: &Individual) -> Individual {
		let mut ret = Individual {
			chromosome: Default::default(),
			fitness: Default::default(),
		};

		let c1 = &self.chromosome;
		let c2 = &another_individual.chromosome;

		let mut mate_chromosome = String::new();

		let it = c1.chars().zip(c2.chars());

		let mut prob: f32 = thread_rng().gen_range(0.0, 1.0);

		for (_i, (g1, g2)) in it.enumerate() {
			if prob < 0.5_f32 {
				mate_chromosome.push(g1);
			} else if prob < 0.9_f32 {
				mate_chromosome.push(g2);
			} else {
				mate_chromosome.push_str(&self.mutate_genes());
			}

			prob = thread_rng().gen_range(0.0, 1.0);
		}

		ret.chromosome = mate_chromosome;
		ret.fitness = ret.calculate_fitness();

		ret
	}
}
