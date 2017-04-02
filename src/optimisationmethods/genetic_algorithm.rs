use population::Population;
use creature::{self, Creature, Node, Muscle};
use optimisationmethods::{OptimisationMethod, OpMethodData};
use rand::{Rng, StdRng};

pub struct GeneticAlgorithm {
	pub data: OpMethodData
}

impl GeneticAlgorithm {
	pub fn new(population: Population) -> Box<GeneticAlgorithm> {
		Box::new(GeneticAlgorithm {
			data: OpMethodData::new(vec![population])
		})
	}

	fn selection (&self, rng: &mut StdRng) -> &Creature {
		let selection_size = 10;
		let mut buffer: Vec<&Creature> = Vec::with_capacity(selection_size);

		for _ in 0..selection_size {
			let idx = rng.gen_range(0, self.data.generations[self.data.gen].creatures.len());
			buffer.push(&self.data.generations[self.data.gen].creatures[idx]);
		}

		buffer.iter().max().unwrap()
	}

	fn mutate (creature: &Creature, rng: &mut StdRng, rate: f32) -> Creature {
		use conrod::utils::clamp;

		// Start by cloning the original creature so we can modify the values of the new one
		let mut new_creature = creature.clone();
		let node_len = creature.nodes.len();

		// For each node in the creature
		for node in &mut new_creature.nodes {
			// Modify the values of each property by the specified rate, but still making sure
			//   they are within the bounds of the original creature specifications.
			node.x = clamp(node.x + rng.gen_range(-rate, rate), creature::BOUNDS_NODE_X.start, creature::BOUNDS_NODE_X.end);
			node.y = clamp(node.y + rng.gen_range(-rate, rate), creature::BOUNDS_NODE_Y.start, creature::BOUNDS_NODE_Y.end);
			node.friction = clamp(node.friction + rng.gen_range(-rate, rate), creature::BOUNDS_NODE_FRICTION.start, creature::BOUNDS_NODE_FRICTION.end);
		}

		// Do the same process as above by for the muscles.
		for muscle in &mut new_creature.muscles {
			muscle.strength = clamp(muscle.strength + rng.gen_range(-rate, rate), creature::BOUNDS_MUSCLE_STRENGTH.start, creature::BOUNDS_MUSCLE_STRENGTH.end);
			muscle.len = new_creature.nodes[muscle.nodes.0].distance(&new_creature.nodes[muscle.nodes.1]);
			muscle.len_min = muscle.len * 0.8;
			muscle.len_max = muscle.len * 1.4;
		}

		// Finally, return the new creature with the modified values.
		new_creature
	}

	/// Takes two parent creatures and returns a child creature.
	fn crossover (creature_a: &Creature, creature_b: &Creature, rng: &mut StdRng) -> Creature {
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

		// For the same number of nodes as creature A, collect a new vector of nodes.
		child.nodes = (0 .. len).map(|i| {
			if i >= start && i <= end {
				// If the current index matches the range of creature clone
				//   its node into the vector
				return creature_a.nodes[i].clone();
			} else if creature_b.nodes.len() > i {
				// Otherwise clone the node from creature B
				return creature_b.nodes[i].clone();
			} else {
				// As a failsafe, just clone the node from creature A as it is guaranteed to
				//   be within range.
				return creature_a.nodes[i].clone();
			}
		}).collect::<Vec<Node>>();

		// Merge muscles
		len = creature_a.muscles.len();

		// Make sure the start value is not the same as the end value for the muscles too.
		loop {
			start = rng.gen_range(0, len);
			end = rng.gen_range(start, len);

			if start != end { break; }
		}

		child.muscles = (0 .. len).map(|j| {
			// The only difference with muscles is that they can sometimes refer to a node
			//   that doesn't exist on this new creature, so we must make sure it's in range
			//   otherwise resetting the nodes it points to are valid.
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
	fn generation_single(&mut self, rng: &mut StdRng) {
		// Before generating a new population we must first calculate the fitness of
		//   each creature in a population.
		self.data.generations[self.data.gen].calculate_fitness();
		let gen_size = self.data.generations[self.data.gen].creatures.len();
		let mut new_population = Population::empty(gen_size);

		println!("Average Fit: {}\tHighest Fit: {}",
			self.data.generations[self.data.gen].fitness_average(),
			self.data.generations[self.data.gen].fittest().fitness
		);

		// Loop until we reach the size of a population
		for _ in 0 .. gen_size {
			// Select two random-ish creatures
			let creature_a = self.selection(rng);
			let creature_b = self.selection(rng);

			// Breed them to make a new child
			let mut child = GeneticAlgorithm::crossover(creature_a, creature_b, rng);

			// Mutate the child ever so slightly so it's not just the same as the parents
			child = GeneticAlgorithm::mutate(&child, rng, 0.25);

			// Finally add the child to the population of child creatures
			new_population.creatures.push(child);
		}

		// After having created the new population, sort the current population by fittest, add
		//   the new population to the optimisation method, and increase the generation number
		self.data.generations[self.data.gen].sort_by_fittest();
		self.data.generations.push(new_population);
		self.data.gen += 1;
	}

	fn creature_get_fittest (&self, gen: usize) -> Creature {
		self.data.generations[gen].fittest().clone()
	}

	fn creature_get (&self, gen: usize, idx: usize) -> Creature {
		self.data.generations[gen].creatures[idx].clone()
	}
}
