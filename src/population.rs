use creature::Creature;
use rand::StdRng;

#[derive(Clone)]
pub struct Population {
	pub creatures: Vec<Creature>
}

impl Population {

	/// Creates a new population with a specified size
	pub fn new(pop_size: u32, rng: &mut StdRng) -> Population {
		// Generate a population with a specified size
		let creatures = (0 .. pop_size).map(|_| {
			Creature::new(rng)
		}).collect::<Vec<Creature>>();

		Population {
			creatures: creatures
		}
	}

	pub fn empty(pop_size: usize) -> Population {
		Population {
			creatures: Vec::with_capacity(pop_size)
		}
	}

	pub fn calculate_fitness(&mut self) {
		for creature in &mut self.creatures {
			creature.calculate_fitness();
		}
	}

	/// Returns the fittest creature in the population
	pub fn fittest(&self) -> &Creature {
		self.creatures.iter().max().unwrap()
	}

	pub fn fitness_average(&self) -> f32 {
		let mut total_fitness = 0.0;
		for creature in &self.creatures {
			total_fitness += creature.fitness;
		}
		total_fitness / self.creatures.len() as f32
	}

	/// Sort the population into fitness first
	pub fn sort_by_fittest(&mut self) {
		self.creatures.sort_by(|a, b| b.cmp(a));
	}
}
