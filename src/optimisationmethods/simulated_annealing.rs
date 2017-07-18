use population::Population;
use creature::Creature;
use optimisationmethods::{GenResult, OptimisationMethod, OpMethodData};
use rand;
use time;
use physics;
use rayon::prelude::*;

pub const MUTABILITY_RATE: f32 = 1.0;
pub const TEMP_HIGH: f64 = 100.0;
pub const TEMP_LOW: f64 = 0.1;
pub const TEMP_ALPHA: f64 = 0.995;

pub struct SimulatedAnnealing {
	pub data: OpMethodData,
	pub temp: f64,
	pub notified: bool
}

impl SimulatedAnnealing {
	pub fn new(population: Population, print: bool) -> Box<SimulatedAnnealing> {
		Box::new(SimulatedAnnealing {
			data: OpMethodData::new(vec![population], "SA".to_string(), print),
			temp: TEMP_HIGH,
			notified: false
		})
	}
}

impl OptimisationMethod for SimulatedAnnealing {
	fn generation_single(&mut self) -> GenResult {
		let gen_size = self.data.generations[self.data.gen].creatures.len();
		let mut new_population = Population::empty(gen_size);

		if self.data.print { println!(
			"SA - Gen {}: Lowest Fit: {}\tAverage Fit: {}\tHighest Fit: {}\tTEMP: {}",
			self.data.gen,
			self.data.generations[self.data.gen]
			    .creatures[gen_size - 1]
			    .fitness,
			self.data.generations[self.data.gen].fitness_average(),
			self.data.generations[self.data.gen].creatures[0].fitness,
			self.temp
		); }

		// SIMULATED ANNEALING MAGIC HAPPENS HERE ONWARDS
		let time_start = time::precise_time_ns() / 1_000_000;

		self.temp = self.temp * TEMP_ALPHA;

		let percentage = self.temp / TEMP_HIGH;

		if self.temp <= TEMP_LOW {
			let pop = self.data.generations[self.data.gen].clone();
			self.data.generations.push(pop);
			self.data.gen += 1;
			if !self.notified {
				return Err((
					"Simulated Annealing".to_string(),
					"The lowest temperature has been reached and cannot
					optimise the current creatures any further".to_string()
				));
			}
			self.notified = true;
		}

		self.data.generations[self.data.gen].creatures
		.par_iter_mut()
		.map(|creature| {
			let mut rng_new = rand::thread_rng();

			let mut new_creature = OpMethodData::mutate(
				&creature,
				&mut rng_new,
				MUTABILITY_RATE * percentage as f32
			);

			physics::full_simulation_creature(&mut new_creature);

			if new_creature.fitness > creature.fitness {
				new_creature
			} else {
				creature.clone()
			}
		})
		.collect_into(&mut new_population.creatures);

		new_population.sort_by_fittest();

		// SIMULATED ANNEALING MAGIC FINISHES HERE ONWARDS
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
