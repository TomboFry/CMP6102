use population::Population;
use creature::Creature;
use optimisationmethods::OptimisationMethod;

pub struct HillClimbing {
	pub generations: Vec<Population>,
	pub gen: u32
}

impl HillClimbing {
	pub fn new(population: Population) -> HillClimbing {
		HillClimbing {
			generations: vec![population],
			gen: 0
		}
	}
}

impl OptimisationMethod for HillClimbing {
	fn generation_single(&mut self, population: Population) -> Population {
		population
	}

	fn creature_get_fittest (&self, gen: usize) -> Creature {
		self.generations[gen].fittest().clone()
	}

	fn creature_get (&self, gen: usize, idx: usize) -> Creature {
		self.generations[gen].creatures[idx].clone()
	}
}
