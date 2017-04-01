use population::Population;
use creature::{Creature, Node, Muscle};
use optimisationmethods::OptimisationMethod;
use rand::{Rng, StdRng};

pub struct GeneticAlgorithm {
	pub generations: Vec<Population>,
	pub gen: usize
}

impl GeneticAlgorithm {
	pub fn new(population: Population) -> Box<GeneticAlgorithm> {
		Box::new(GeneticAlgorithm {
			generations: vec![population],
			gen: 0
		})
	}

	fn random_selection (&self, rng: &mut StdRng) -> &Creature {
		let selection_size = 10;
		let mut buffer: Vec<&Creature> = Vec::with_capacity(selection_size);

		for _ in 0..selection_size {
			let idx = rng.gen_range(0, self.generations[self.gen].creatures.len());
			buffer.push(&self.generations[self.gen].creatures[idx]);
		}
		buffer.iter().max().unwrap()
	}

	/// Takes two parent creatures and returns a child creature.
	fn crossover (creature_a: &Creature, creature_b: &Creature, rng: &mut StdRng) -> Creature {
		let mut child = Creature::empty();

		// Merge Nodes
		let len_nodes = creature_a.nodes.len();
		let mut start_nodes: usize;
		let mut end_nodes: usize;

		loop {
			start_nodes = rng.gen_range(0, len_nodes);
			end_nodes = rng.gen_range(start_nodes, len_nodes);

			if start_nodes != end_nodes { break; }
		}

		child.nodes = (0 .. len_nodes).map(|i| {
			if i >= start_nodes && i <= end_nodes {
				return creature_a.nodes[i].clone();
			} else if creature_b.nodes.len() > i {
				return creature_b.nodes[i].clone();
			} else {
				return creature_a.nodes[i].clone();
			}
		}).collect::<Vec<Node>>();

		// Merge muscles
		let len_muscles = creature_a.muscles.len();
		let mut start_muscles: usize;
		let mut end_muscles: usize;

		loop {
			start_muscles = rng.gen_range(0, len_muscles);
			end_muscles = rng.gen_range(start_muscles, len_muscles);

			if start_muscles != end_muscles { break; }
		}

		child.muscles = (0 .. len_muscles).map(|j| {
			let len = child.nodes.len();
			if j >= start_muscles && j <= end_muscles {
				return creature_a.muscles[j].range(len, rng);
			} else if creature_b.muscles.len() > j {
				return creature_b.muscles[j].range(len, rng);
			} else {
				return creature_a.muscles[j].range(len, rng);
			}
		}).collect::<Vec<Muscle>>();

		child.fitness = 0.0;

		child
	}
}

impl OptimisationMethod for GeneticAlgorithm {
	fn generation_single(&mut self, rng: &mut StdRng) {
		let mut new_population: Population;

		let gen_size = self.generations[self.gen].creatures.len();
		new_population = Population::empty(gen_size);

		println!("Average Fit: {}\tHighest Fit: {}",
			self.generations[self.gen].fitness_average(),
			self.generations[self.gen].fittest().fitness
		);

		for _ in 0..gen_size {
			let creature_a = self.random_selection(rng);
			let creature_b = self.random_selection(rng);
			let child = GeneticAlgorithm::crossover(creature_a, creature_b, rng);
			new_population.creatures.push(child);
		}

		// After having created the new population, sort the current population by fittest, add
		//   the new population to the optimisation method, and increase the generation number
		self.generations[self.gen].sort_by_fittest();
		self.generations.push(new_population);
		self.gen += 1;
	}

	fn creature_get_fittest (&self, gen: usize) -> Creature {
		self.generations[gen].fittest().clone()
	}

	fn creature_get (&self, gen: usize, idx: usize) -> Creature {
		self.generations[gen].creatures[idx].clone()
	}
}
