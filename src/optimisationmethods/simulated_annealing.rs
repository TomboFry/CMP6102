use population::Population;
use creature::{self, Creature};
use optimisationmethods::{GenResult, OptimisationMethod, OpMethodData};
use rand::{Rng, StdRng};
use time;
use physics;

pub const MUTABILITY_RATE: f32 = 0.4;
pub const PROB_NODE_REMOVE: f32 = 8.0; // will be 1 / x
pub const TEMP_HIGH: f64 = 100.0;
pub const TEMP_LOW: f64 = 0.1;
pub const TEMP_ALPHA: f64 = 0.98;

pub struct SimulatedAnnealing {
	pub data: OpMethodData,
	pub temp: f64,
	pub notified: bool
}

impl SimulatedAnnealing {
	pub fn new(population: Population) -> Box<SimulatedAnnealing> {
		Box::new(SimulatedAnnealing {
			data: OpMethodData::new(vec![population]),
			temp: TEMP_HIGH,
			notified: false
		})
	}
}

impl OptimisationMethod for SimulatedAnnealing {
	fn generation_single(&mut self, rng: &mut StdRng) -> GenResult {
		let gen_size = self.data.generations[self.data.gen].creatures.len();
		let mut new_population = Population::empty(gen_size);

		println!("SA - Gen {}: Lowest Fit: {}\tAverage Fit: {}\tHighest Fit: {}\tTEMP: {}",
			self.data.gen,
			self.data.generations[self.data.gen].creatures[gen_size - 1].fitness,
			self.data.generations[self.data.gen].fitness_average(),
			self.data.generations[self.data.gen].creatures[0].fitness,
			self.temp
		);

		let time_start = time::precise_time_ns() / 10_000;

		// SIMULATED ANNEALING MAGIC HAPPENS HERE ONWARDS

		self.temp = self.temp * TEMP_ALPHA;

		let percentage = self.temp / TEMP_HIGH;

		if self.temp <= TEMP_LOW {
			let pop = self.data.generations[self.data.gen].clone();
			self.data.generations.push(pop);
			self.data.gen += 1;
			if !self.notified {
				return Err(("Simulated Annealing".to_string(), "The lowest temperature has been reached and cannot optimise the current creatures any further".to_string()));
			}
			self.notified = true;
		}

		for creature in &mut self.data.generations[self.data.gen].creatures {
			let mut node_add = false;
			let mut node_remove = false;
			if rng.gen::<f32>() * PROB_NODE_REMOVE <= 1.0 && creature.nodes.len() as u8 <= creature::BOUNDS_NODE_COUNT.end - 1 {
				node_add = true;
			}
			if rng.gen::<f32>() * PROB_NODE_REMOVE <= 1.0 && creature.nodes.len() as u8 > creature::BOUNDS_NODE_COUNT.start {
				node_remove = true;
			}
			let mut new_creature = OpMethodData::mutate(creature, rng, MUTABILITY_RATE * percentage as f32, node_add, node_remove);
			physics::full_simulation_creature(&mut new_creature);
			if new_creature.fitness > creature.fitness {
				new_population.creatures.push(new_creature);
			} else {
				new_population.creatures.push(creature.clone());
			}
		}

		// SIMULATED ANNEALING MAGIC FINISHES HERE ONWARDS

		let time_end = time::precise_time_ns() / 10_000;

		new_population.calculate_fitness();

		// After having created the new population, sort the current population by fittest, add
		//   the new population to the optimisation method, and increase the generation number
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
