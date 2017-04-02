use population::Population;
use creature::Creature;
use optimisationmethods::{OptimisationMethod, OpMethodData};
use rand::StdRng;

pub struct HillClimbing {
	pub data: OpMethodData
}

impl HillClimbing {
	pub fn new(population: Population) -> HillClimbing {
		HillClimbing {
			data: OpMethodData::new(vec![population])
		}
	}
}

impl OptimisationMethod for HillClimbing {
	fn generation_single(&mut self, rng: &mut StdRng) {
		self.data.generations[self.data.gen].calculate_fitness();
		let gen_size = self.data.generations[self.data.gen].creatures.len();
		let new_population = Population::empty(gen_size);

		// Do hill climbing stuff

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
