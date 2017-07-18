use population::Population;
use creature::{Creature, Node, Muscle};
use optimisationmethods::{GenResult, OptimisationMethod, OpMethodData};
use physics;
use rand::{self, Rng, ThreadRng};
use time;
use rayon::prelude::*;

pub const MUTABILITY_RATE:  f32 = 0.05;

pub struct GeneticAlgorithm {
	pub data: OpMethodData
}

impl GeneticAlgorithm {
	pub fn new(population: Population, print: bool) -> Box<GeneticAlgorithm> {
		Box::new(GeneticAlgorithm {
			data: OpMethodData::new(vec![population], "GA".to_string(), print)
		})
	}

	fn selection (&self, rng: &mut ThreadRng) -> &Creature {
		// Tournament Selection
		let selection_size = 3;
			//(self.data.generations[self.data.gen].creatures.len() as f64 *
			// 0.02) as usize + 1;

		let mut selection: Vec<&Creature> = Vec::with_capacity(selection_size);

		for _ in 0 .. selection_size {
			let idx = rng.gen_range(
				0, self.data.generations[self.data.gen].creatures.len()
			);

			selection.push(
				&self.data.generations[self.data.gen].creatures[idx]
			);
		}

		selection.iter().max().unwrap()
	}

	/// Takes two parent creatures and returns a child creature.
	fn crossover (
		creature_a: &Creature,
		creature_b: &Creature,
		rng: &mut ThreadRng
	) -> Creature {
		let mut child = Creature::empty();

		// Merge Nodes
		let mut len = creature_a.nodes.len();
		let mut start: usize;
		let mut end: usize;

		// Loop to ensure the start value is not the same as the end value
		loop {
			start = rng.gen_range(0, len);
			end = rng.gen_range(start, len);

			if start != end { break; }
		}

		// For the same number of nodes as creature A, collect a new vector
		// of nodes.
		child.nodes = (0 .. len).map(|i| {
			if i >= start && i <= end {
				// If the current index matches the range of creature clone
				//   its node into the vector
				return creature_a.nodes[i].clone();
			} else if creature_b.nodes.len() > i {
				// Otherwise clone the node from creature B
				return creature_b.nodes[i].clone();
			} else {
				// As a failsafe, just clone the node from creature A as it is
				// guaranteed to be within range.
				return creature_a.nodes[i].clone();
			}
		}).collect::<Vec<Node>>();

		// Merge muscles
		len = creature_a.muscles.len();

		// Make sure the start value is not the same as the end value for
		// the muscles too.
		loop {
			start = rng.gen_range(0, len);
			end = rng.gen_range(start, len);

			if start != end { break; }
		}

		child.muscles = (0 .. len).map(|j| {
			// The only difference with muscles is that they can sometimes
			// refer to a node that doesn't exist on this new creature, so we
			// must make sure it's in range otherwise resetting the nodes it
			// points to are valid.
			let len = child.nodes.len();
			if j >= start && j <= end {
				return creature_a.muscles[j].range(len, rng);
			} else if creature_b.muscles.len() > j {
				return creature_b.muscles[j].range(len, rng);
			} else {
				return creature_a.muscles[j].range(len, rng);
			}
		}).collect::<Vec<Muscle>>();

		// Finally, return the child we have bred from two parents.
		child
	}
}

impl OptimisationMethod for GeneticAlgorithm {
	fn generation_single(&mut self) -> GenResult {
		let gen_size = self.data.generations[self.data.gen].creatures.len();
		let mut new_population = Population::empty(gen_size);

		if self.data.print { println!(
			"GA - Gen {}: Lowest Fit: {}\tAverage Fit: {}\tHighest Fit: {}",
			self.data.gen,
			self.data.generations[self.data.gen]
			    .creatures[gen_size - 1]
			    .fitness,
			self.data.generations[self.data.gen].fitness_average(),
			self.data.generations[self.data.gen].creatures[0].fitness
		); }

		let time_start = time::precise_time_ns() / 1_000_000;

		// Loop until we reach the size of a population
		(0 .. gen_size)
		.into_par_iter()
		.map(|_| {
			let mut rng_new = rand::thread_rng();

			// Select two creatures through tournaments
			let creature_a = self.selection(&mut rng_new);
			let creature_b = self.selection(&mut rng_new);

			// Breed them to make a new child
			let mut child = GeneticAlgorithm::crossover(
				creature_a,
				creature_b,
				&mut rng_new
			);

			// Mutate the child ever so slightly so it's not just the same
			// as the parents
			child = OpMethodData::mutate(
				&child,          // Creature to mutate
				&mut rng_new,    // The RNG thread to create random nums with
				MUTABILITY_RATE
			);

			physics::full_simulation_creature(&mut child);

			// Finally add the child to the population of child creatures
			//new_population.creatures.push(child);
			child
		})
		.collect_into(&mut new_population.creatures);

		// After generating a new population we must calculate the fitness
		// of each creature in a population.
		// new_population.calculate_fitness();
		new_population.sort_by_fittest();

		let time_end = time::precise_time_ns() / 1_000_000;

		// After having created the new population, sort the current
		// population by fittest, add the new population to the optimisation
		// method, and increase the generation number
		self.data.generations.push(new_population);
		self.data.gen_time.push(time_end - time_start);
		self.data.gen += 1;

		Ok(())
	}

	fn creature_get (&mut self, gen: usize, idx: usize) -> &mut Creature {
		&mut self.data.generations[gen].creatures[idx]
	}

	fn get_data_mut(&mut self) -> &mut OpMethodData {
		&mut self.data
	}
	fn get_data(&self) -> &OpMethodData {
		&self.data
	}
}

#[cfg(test)]
mod test {

	#[test]
	#[should_panic]
	fn genalg_fitness() {
		unimplemented!();

		// use population::Population;
		// use optimisationmethods::genetic_algorithm::GeneticAlgorithm;
		// use optimisationmethods::OptimisationMethod;

		// let mut rng = ::tests::init();
		// let pop = Population::new(1000, &mut rng);
		// let genalg = GeneticAlgorithm::new(pop);
		// let data = genalg.get_data();
		// genalg.generation_single(&mut rng);
		// let fitness_average_start = data.
	}
}
