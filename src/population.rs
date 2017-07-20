use creature::Creature;
use physics;
use rand::ThreadRng;

#[derive(Clone)]
pub struct Population {
	pub creatures: Vec<Creature>
}

impl Population {

	/// Creates a new population with a specified size
	pub fn new(pop_size: usize, rng: &mut ThreadRng) -> Population {
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

	/// Creates an empty population
	pub fn empty(pop_size: usize) -> Population {
		Population {
			creatures: Vec::with_capacity(pop_size)
		}
	}

	/// Runs the physics calculations for every creature in the population
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

	/// Calculates the entire population's average fitness
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

mod test {
	use population::*;
	use rand;

	#[test]
	fn new_pop() {
		let mut rng = rand::thread_rng();
		let population = Population::new(100, &mut rng);

		assert_eq!(population.creatures.len(), 100);
	}

	#[test]
	fn fittest() {
		let mut rng = rand::thread_rng();
		let population = Population::new(100, &mut rng);

		// Ensure that the fitness of the creature returned by the fittest()
		// function is greater than the second element in the array.

		// This function may be considered redundant as the creatures are
		// sorted by fitness upon creation anyway (so index 0 always contains
		// the fittest creature).
		assert!(
			population.fittest().fitness > population.creatures[1].fitness
		);
	}

	#[test]
	fn weakest() {
		let mut rng = rand::thread_rng();
		let population = Population::new(100, &mut rng);

		// Ensure that the fitness of the creature returned by the weakest()
		// function is less than the second to last element in the array.
		assert!(
			population.weakest().fitness < population.creatures[98].fitness
		);
	}

}
