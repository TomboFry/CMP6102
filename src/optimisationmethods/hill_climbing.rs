use population::Population;
use creature::{self, Creature};
use optimisationmethods::{GenResult, OptimisationMethod, OpMethodData};
use rand::{self, Rng};
use time;
use physics;
use rayon::prelude::*;

pub const CLIMB_ATTEMPTS: usize = 4;
pub const MUTABILITY_RATE: f32 = 0.1;
pub const PROB_NODE_CHANGE: f32 = 8.0; // will be 1 / x

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

		let time_start = time::precise_time_ns() / 1_000_000;

		self.data.generations[self.data.gen].creatures
		.par_iter_mut()
		.map(|creature| {
			let mut new_creatures = Vec::with_capacity(CLIMB_ATTEMPTS);
			let mut rng_new = rand::thread_rng();
			for _ in 0 .. CLIMB_ATTEMPTS {

				let node_remove =
					rng_new.gen::<f32>() * PROB_NODE_CHANGE <= 1.0 &&
					(creature.nodes.len() as u8) >
					creature::BOUNDS_NODE_COUNT.start;

				let node_add =
					rng_new.gen::<f32>() * PROB_NODE_CHANGE <= 1.0 &&
					(creature.nodes.len() as u8) < creature::BOUNDS_NODE_COUNT.end;

				let muscle_remove =
					rng_new.gen::<f32>() * PROB_NODE_CHANGE <= 1.0;

				let muscle_add =
					rng_new.gen::<f32>() * PROB_NODE_CHANGE <= 1.0;

				let mut new_creature =
					OpMethodData::mutate(
						creature,
						&mut rng_new,
						MUTABILITY_RATE,
						node_add,
						node_remove,
						muscle_add,
						muscle_remove
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

		// for creature in &mut self.data.generations[self.data.gen].creatures {
		// 	let mut new_creatures = Vec::with_capacity(CLIMB_ATTEMPTS);
		// 	for _ in 0 .. CLIMB_ATTEMPTS {
		// 		let mut node_add = false;
		// 		let mut node_remove = false;

		// 		// Have the random chance to add a node
		// 		if rng.gen::<f32>() * CLIMB_ATTEMPTS as f32 <= 1.0 &&
		// 		   creature.nodes.len() as u8 <=
		// 		   creature::BOUNDS_NODE_COUNT.end - 1
		// 		{
		// 			node_add = true;
		// 		}

		// 		// Have the same random chance to remove a random node
		// 		if rng.gen::<f32>() * CLIMB_ATTEMPTS as f32 <= 1.0 &&
		// 		   creature.nodes.len() as u8 >
		// 		   creature::BOUNDS_NODE_COUNT.start
		// 		{
		// 			node_remove = true;
		// 		}

		// 		let mut new_creature =
		// 			OpMethodData::mutate(
		// 				creature,
		// 				rng,
		// 				MUTABILITY_RATE,
		// 				node_add,
		// 				node_remove
		// 			);

		// 		physics::full_simulation_creature(&mut new_creature);
		// 		new_creatures.push(new_creature);
		// 	}

		// 	new_creatures.sort_by(|a,b| b.cmp(a));

		// 	if new_creatures[0].fitness > creature.fitness {
		// 		new_population.creatures.push(new_creatures[0].clone());
		// 	} else {
		// 		new_population.creatures.push(creature.clone());
		// 	}
		// }

		new_population.sort_by_fittest();

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
