use population::Population;
use creature::Creature;
use optimisationmethods::{OptimisationMethod, OpMethodData};
use rand::StdRng;
use time;

pub struct SimulatedAnnealing {
	pub data: OpMethodData
}

impl SimulatedAnnealing {
	pub fn new(population: Population) -> Box<SimulatedAnnealing> {
		Box::new(SimulatedAnnealing {
			data: OpMethodData::new(vec![population])
		})
	}
}

impl OptimisationMethod for SimulatedAnnealing {
	fn generation_single(&mut self, rng: &mut StdRng) {
		let gen_size = self.data.generations[self.data.gen].creatures.len();
		let mut new_population = Population::empty(gen_size);

		println!("SA - Gen {}: Lowest Fit: {}\tAverage Fit: {}\tHighest Fit: {}",
			self.data.gen,
			self.data.generations[self.data.gen].creatures[gen_size - 1].fitness,
			self.data.generations[self.data.gen].fitness_average(),
			self.data.generations[self.data.gen].creatures[0].fitness
		);

		let time_start = time::precise_time_ns() / 10_000;

		// Do Simulated Annealing Stuff here.
		// for creature in &mut self.data.generations[self.data.gen].creatures {

		// }

		let time_end = time::precise_time_ns() / 10_000;

		new_population.calculate_fitness();

		// After having created the new population, sort the current population by fittest, add
		//   the new population to the optimisation method, and increase the generation number
		self.data.generations.push(new_population);
		self.data.gen_time.push(time_end - time_start);
		self.data.gen += 1;
	}

	fn creature_get_fittest (&self, gen: usize) -> &Creature {
		&self.data.generations[gen].fittest()
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
