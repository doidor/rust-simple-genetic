extern crate rand;

use rand::prelude::*;
use rand::seq::SliceRandom;
use std::thread;
use std::time::Instant;
use super::config::POPULATION_NR;
use super::individual::Individual;

#[derive(Debug)]
pub struct Population {
	individuals: Vec<Individual>,
	best_chromosome: String,
	best_fitness: i32,
}

impl Population {
	pub fn new(to_find: String) -> Population {
		let mut individuals: Vec<Individual> = vec![];

		let mut ret = Population {
			individuals: Default::default(),
			best_chromosome: Default::default(),
			best_fitness: Default::default(),
		};

		for _i in 0..POPULATION_NR {
			individuals.push(Individual::new(to_find.clone()));
		}

		ret.individuals = individuals;

		ret
	}

	fn calculate_best(&mut self) {
		self.individuals
			.sort_by(|i1, i2| i2.fitness.cmp(&i1.fitness).reverse());
		self.best_chromosome = self.individuals[0].chromosome.clone();
		self.best_fitness = self.individuals[0].fitness;
	}

	#[allow(dead_code)]
	fn mutate_population(&mut self) {
		let mut new_generation: Vec<Individual> = vec![];
		let elite: usize = ((10 * POPULATION_NR) / 100) as usize;
		let rest: usize = ((90 * POPULATION_NR) / 100) as usize;

		new_generation.extend_from_slice(&self.individuals[..elite]);

		let half_percent = ((50 * POPULATION_NR) / 100) as usize;

		for _i in 0..rest {
			let parent1 = &self.individuals[..half_percent]
				.choose(&mut thread_rng())
				.unwrap();
			let parent2 = &self.individuals[..half_percent]
				.choose(&mut thread_rng())
				.unwrap();

			let child: Individual = parent1.mate(*parent2);

			new_generation.push(child);
		}

		self.individuals = new_generation;

		self.calculate_best();
	}

	fn mutate_population_threaded(&mut self) {
		let mut new_generation: Vec<Individual> = vec![];
		let elite: usize = ((10 * POPULATION_NR) / 100) as usize;
		let rest: usize = ((90 * POPULATION_NR) / 100) as usize;

		let mut children = vec![];

		const THREAD_COUNT: usize = 10;

		let chunk_size: usize = (rest / THREAD_COUNT) as usize;

		for _i in 0..THREAD_COUNT {
			let chunk_clone = self.individuals.clone();

			children.push(thread::spawn(move || -> Vec<Individual> {
				let mut ret: Vec<Individual> = vec![];

				let half_percent = ((50 * chunk_clone.len()) / 100) as usize;

				for _i in 0..chunk_size {
					let parent1 = chunk_clone[..half_percent]
						.choose(&mut thread_rng())
						.unwrap();
					let parent2 = chunk_clone[..half_percent]
						.choose(&mut thread_rng())
						.unwrap();

					let child: Individual = parent1.mate(parent2);

					ret.push(child);
				}

				ret
			}));
		}

		new_generation.extend_from_slice(&self.individuals[..elite]);

		for child in children {
			new_generation.extend_from_slice(&child.join().unwrap());
		}

		self.individuals = new_generation;

		self.calculate_best();
	}

	pub fn find_best(&mut self) -> &Individual {
		let now = Instant::now();

		let mut generation_count: i32 = 1;

		println!("Evolving...");
		println!("\n-----------------------------------------------------------");

		self.calculate_best();

		loop {
			if generation_count % 10 == 0 {
				println!(
					"Generation: {}. Best chromosome: {}. Best fitness: {}",
					generation_count, &self.best_chromosome, self.best_fitness
				);
			}

			if self.best_fitness <= 0 {
				break;
			}

			self.mutate_population_threaded();

			generation_count += 1;
		}

		println!("-----------------------------------------------------------\n");
		println!("Finished in {} generations. Took {} ms.", generation_count, now.elapsed().as_millis());

		&self.individuals[0]
	}
}
