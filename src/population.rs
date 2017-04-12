use creature::Creature;
use physics;
use rand::StdRng;

#[derive(Clone)]
pub struct Population {
	pub creatures: Vec<Creature>
}

impl Population {

	/// Creates a new population with a specified size
	pub fn new(pop_size: usize, rng: &mut StdRng) -> Population {
		// 1: Generate a population with a specified size
		let creatures = (0 .. pop_size).map(|_| {
			Creature::new(rng)
		}).collect::<Vec<Creature>>();

		// 2: Create the population struct
		let mut population = Population {
			creatures: creatures
		};

		// 3: Calculate the new population's fitness
		population.calculate_fitness();

		// 4: Return the new population
		population
	}

	pub fn empty(pop_size: usize) -> Population {
		Population {
			creatures: Vec::with_capacity(pop_size)
		}
	}

	pub fn calculate_fitness(&mut self) {
		physics::full_simulation_population(self);
		self.sort_by_fittest();
	}

	/// Returns the fittest creature in the population
	pub fn fittest(&self) -> &Creature {
		self.creatures.iter().max().unwrap()
	}

	/// Returns the unfittest creature in the population
	pub fn weakest(&self) -> &Creature {
		self.creatures.iter().min().unwrap()
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
