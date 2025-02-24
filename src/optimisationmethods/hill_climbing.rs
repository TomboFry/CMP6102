use population::Population;
use creature::Creature;
use optimisationmethods::{GenResult, OptimisationMethod, OpMethodData};
use rand;
use time;
use physics;
use rayon::prelude::*;

pub const CLIMB_ATTEMPTS: usize = 4;
pub const MUTABILITY_RATE: f32 = 0.05;

pub struct HillClimbing {
	pub data: OpMethodData
}

impl HillClimbing {
	pub fn new(population: Population, print: bool) -> Box<HillClimbing> {
		Box::new(HillClimbing {
			data: OpMethodData::new(vec![population], "HC".to_string(), print)
		})
	}
}

impl OptimisationMethod for HillClimbing {
	fn generation_single(&mut self) -> GenResult {
		let gen_size = self.data.generations[self.data.gen].creatures.len();
		let mut new_population = Population::empty(gen_size);

		if self.data.print { println!(
			"HC - Gen {}: Lowest Fit: {}\tAverage Fit: {}\tHighest Fit: {}",
			self.data.gen,
			self.data.generations[self.data.gen]
			    .creatures[gen_size - 1]
			    .fitness,
			self.data.generations[self.data.gen].fitness_average(),
			self.data.generations[self.data.gen].creatures[0].fitness
		); }

		let time_start = time::precise_time_ns() as f32 / 1_000_000.0;

		self.data.generations[self.data.gen].creatures
		.par_iter_mut()
		.map(|creature| {
			let mut new_creatures = Vec::with_capacity(CLIMB_ATTEMPTS);
			let mut rng_new = rand::thread_rng();
			for _ in 0 .. CLIMB_ATTEMPTS {
				let mut new_creature = OpMethodData::mutate(
					creature,
					&mut rng_new,
					MUTABILITY_RATE
				);

				physics::full_simulation_creature(&mut new_creature);
				new_creatures.push(new_creature);
			}

			new_creatures.sort_by(|a,b| b.cmp(a));

			if new_creatures[0].fitness > creature.fitness {
				new_creatures[0].clone()
			} else {
				creature.clone()
			}
		})
		.collect_into(&mut new_population.creatures);

		new_population.sort_by_fittest();

		let time_end = time::precise_time_ns() as f32 / 1_000_000.0;

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
mod tests {
	use rand;
	use population::Population;
	use optimisationmethods::hill_climbing::HillClimbing;
	use optimisationmethods::OptimisationMethod;

	/// Run the generation function 10 times and make sure the average
	/// population's fitness has increased. Run these 50 times to ensure
	/// it's actually successful.
	#[test]
	fn fitness_10gens() {
		let mut rng = rand::thread_rng();

		for _ in 0 .. 10 {
			// Create a new population of 100 creatures
			let population = Population::new(100, &mut rng);

			// Initalise the genetic algorithm with the population
			let mut hc = HillClimbing::new(population, false);

			for _ in 0 .. 10 {
				let _ = hc.generation_single();
			}

			let initial_fitness = hc.get_data().creature_get_average(0);
			let final_fitness = hc.get_data().creature_get_average(10);

			assert!(final_fitness > initial_fitness);
		}
	}
}

