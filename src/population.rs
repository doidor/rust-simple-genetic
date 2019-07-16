extern crate rand;

use rand::prelude::*;
use rand::seq::SliceRandom;

use super::config::POPULATION_NR;
use super::individual::Individual;

#[derive(Debug)]
pub struct Population {
    individuals: Vec<Individual>,
    best_chromosome: String,
    best_fitness: i32,
}

impl Population {
    pub fn new() -> Population {
        let mut individuals: Vec<Individual> = vec![];

        let mut ret = Population {
            individuals: Default::default(),
            best_chromosome: Default::default(),
            best_fitness: Default::default(),
        };

        for _i in 0..POPULATION_NR {
            individuals.push(Individual::new());
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

    fn mutate_population(&mut self) {
        let mut new_generation: Vec<Individual> = vec![];
        let elite: usize = ((10 * POPULATION_NR) / 100) as usize;
        let rest: usize = ((90 * POPULATION_NR) / 100) as usize;

        new_generation.extend_from_slice(&self.individuals[..elite]);

        for _i in 0..rest {
            let parent1 = &self.individuals[..50].choose(&mut thread_rng()).unwrap();
            let parent2 = &self.individuals[..50].choose(&mut thread_rng()).unwrap();

            let child: Individual = parent1.mate(*parent2);

            new_generation.push(child);
        }

        self.individuals = new_generation;

        self.calculate_best();
    }

    pub fn find_best(&mut self) -> &Individual {
        self.calculate_best();

        loop {
            println!(
                "best chromosome: '{}' best fitness: {}",
                self.best_chromosome, self.best_fitness
            );
            if self.best_fitness <= 0 {
                break;
            }

            self.mutate_population();
        }

        &self.individuals[0]
    }
}
