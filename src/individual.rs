extern crate rand;

use rand::prelude::*;

use super::config::GENES;

#[derive(Debug, Clone)]
pub struct Individual {
    pub chromosome: String,
    pub fitness: i32,
    to_find: String,
}

impl Individual {
    pub fn new(to_find: String) -> Individual {
        let mut ind = Individual {
            chromosome: Default::default(),
            fitness: Default::default(),
            to_find,
        };

        ind.create_genome();
        ind.calculate_fitness();

        ind
    }

    fn mutate_genes(&self) -> String {
        let rand_pos = thread_rng().gen_range(0, GENES.len());

        String::from(GENES)[rand_pos..rand_pos + 1].to_string()
    }

    fn create_genome(&mut self) {
        let mut ret: String = String::new();
        let mut count = self.to_find.len();

        while count > 0 {
            ret.push_str(&self.mutate_genes());

            count -= 1;
        }

        self.chromosome = ret;
    }

    pub fn calculate_fitness(&mut self) {
        let mut c1: char;
        let mut c2: char;

        let mut fitness: i32 = 0;
        let to_find_len: i32 = self.to_find.len() as i32;

        for i in 0..to_find_len {
            c1 = self.to_find.chars().nth(i as usize).unwrap();
            c2 = self.chromosome.chars().nth(i as usize).unwrap();

            if c1 != c2 {
                fitness += 1;
            }
        }

        self.fitness = fitness
    }

    pub fn mate(&self, another_individual: &Individual) -> Individual {
        let mut ret = Individual {
            chromosome: Default::default(),
            fitness: Default::default(),
            to_find: self.to_find.to_owned(),
        };

        let c1 = &self.chromosome;
        let c2 = &another_individual.chromosome;

        let mut mate_chromosome = String::new();

        let it = c1.chars().zip(c2.chars());

        let mut prob: f32 = thread_rng().gen_range(0.0, 1.0);

        for (_i, (g1, g2)) in it.enumerate() {
            if prob < 0.49_f32 {
                mate_chromosome.push(g1);
            } else if prob < 0.9_f32 {
                mate_chromosome.push(g2);
            } else {
                mate_chromosome.push_str(&self.mutate_genes());
            }

            prob = thread_rng().gen_range(0.0, 1.0);
        }

        ret.chromosome = mate_chromosome;
        ret.calculate_fitness();

        ret
    }
}
