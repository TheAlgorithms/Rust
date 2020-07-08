/*
Simple multithreaded algorithm to show how the 4 phases of a genetic
algorithm works (Evaluation, Selection, Crossover and Mutation)
https://en.wikipedia.org/wiki/Genetic_algorithm

Link to the same algorithm implemented in python:
https://github.com/TheAlgorithms/Python/blob/master/genetic_algorithm/basic_string.py

Author: D4rkia
*/

use crate::general::Rand;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
struct PopItem {
    genes: Vec<char>,
    fitness: f32,
}

impl PopItem {
    fn init() -> PopItem {
        PopItem {
            genes: vec![],
            fitness: 0.0,
        }
    }

    fn calc_fitness(&mut self, target: &[char]) {
        self.fitness = self
            .genes
            .iter()
            .zip(target)
            .filter(|&(a, b)| a == b)
            .count() as f32;
        self.fitness /= target.len() as f32;
    }
}

pub fn genetic_string(target: &str, genes_str: &str) -> String {
    // Define parameters
    let target_string = target;
    let char_string = genes_str;
    // Maximum size of the population.  bigger could be faster but is more memory expensive
    let pop_num = 200;
    // Number of elements selected in every generation for evolution the selection takes
    // place from the best to the worst of that generation must be smaller than N_POPULATION
    let selection_num = 50;
    // Probability that an element of a generation can mutate changing one of its genes this
    // guarantees that all genes will be used during evolution
    let mutation_prob = 0.4;

    // Just a seed to improve randomness required by the algorithm
    let mut rng = Rand::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as usize,
    );

    // Convert char_string in vector
    let genes: Vec<char> = char_string.chars().collect();

    // Verify if 'pop_num' s bigger than 'selection_num'
    if pop_num < selection_num {
        panic!("{} must be bigger than {}", pop_num, selection_num);
    }
    // Verify that the target contains no genes besides the ones inside genes variable.
    let target: Vec<char> = target_string.chars().collect();
    for c in target.iter() {
        if !genes.contains(&c) {
            panic!("char {} is not part of the genes", &c)
        }
    }

    // Generate random starting population
    let mut pop = vec![PopItem::init(); pop_num];
    for item in pop.iter_mut() {
        for _ in 0..target.len() {
            item.genes.push(genes[rng.rand_range_usize(0, genes.len())])
        }
    }

    // Just some logs to know what the algorithms is doing
    let mut gen = 0;
    let mut generated_pop = 0;

    // This loop will end when we will find a perfect match for our target
    loop {
        gen += 1;
        generated_pop += pop.len();

        // Random population created now it's time to evaluate
        for item in pop.iter_mut() {
            item.calc_fitness(&target);
        }

        // Check if there is a matching evolution
        pop.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        if pop[0].genes == target {
            println!(
                "\nGeneration: {}\nAnalyzed: {}\nBest: {:?} ",
                gen,
                generated_pop,
                (
                    &pop[0].genes.iter().cloned().collect::<String>(),
                    &pop[0].fitness
                )
            );
            return pop[0].genes.iter().collect();
        }

        // Print the best resultPrint the Best result every 10 generations
        // just to know that the algorithm is working
        if &gen % 10 == 0 {
            println!(
                "Generation: {} Analyzed: {} Best: {:?} ",
                gen,
                generated_pop,
                (
                    &pop[0].genes.iter().cloned().collect::<String>(),
                    &pop[0].fitness
                )
            );
        }

        // Generate a new population vector keeping some of the best evolutions
        // Keeping this avoid regression of evolution
        let mut pop_children: Vec<PopItem> = vec![];
        pop_children.extend_from_slice(&pop[..(selection_num / 3) as usize]);

        // This is Selection
        for i in 0..selection_num {
            let parent1 = &pop[i];
            // Generate more child proportionally to the fitness score
            let mut child_n = (parent1.fitness * 100.0) as usize + 1;
            if child_n >= 10 {
                child_n = 10
            }
            for _ in 0..child_n {
                let parent2 = pop[rng.rand_range_usize(0, selection_num)].clone();
                // Crossover
                let split = rng.rand_range_usize(0, parent1.genes.len() - 1);
                let mut child1: Vec<char> = Vec::new();
                let mut child2: Vec<char> = Vec::new();
                child1.extend([&parent1.genes[..split], &parent2.genes[split..]].concat());
                child2.extend([&parent2.genes[..split], &parent1.genes[split..]].concat());
                // Mutate
                if rng.rand_float_normal() < mutation_prob {
                    let cut = rng.rand_range_usize(0, child1.len());
                    child1[cut] = genes[rng.rand_range_usize(0, genes.len())];
                }
                if rng.rand_float_normal() < mutation_prob {
                    let cut = rng.rand_range_usize(0, child2.len());
                    child2[cut] = genes[rng.rand_range_usize(0, genes.len())];
                }
                // Push into pop_children
                pop_children.push(PopItem {
                    genes: child1.clone(),
                    fitness: 0.0,
                });
                pop_children.push(PopItem {
                    genes: child2.clone(),
                    fitness: 0.0,
                });
                // Check if the population has already reached the maximum value and if so,
                // break the cycle. If this check is disabled the algorithm will take
                // forever to compute large strings but will also calculate small string in
                // a lot fewer generations
                if pop_children.len() >= selection_num {
                    break;
                }
            }
        }
        pop = pop_children;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn genetic_string() {
        assert_eq!(
            super::genetic_string(
                "This is a genetic algorithm to evaluate, combine, evolve  mutate a string!",
                " ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz.,;!?+-*#@^'èéòà€ù=)(&%$£/\\"
            ),
            String::from(
                "This is a genetic algorithm to evaluate, combine, evolve  mutate a string!"
            )
        );
    }
}
