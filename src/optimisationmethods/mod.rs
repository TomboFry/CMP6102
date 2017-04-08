use rand::StdRng;
use creature::Creature;
use population::Population;

pub mod genetic_algorithm;
pub mod hill_climbing;
pub mod simulated_annealing;

pub struct OpMethodData {
	pub generations: Vec<Population>,
	pub gen: usize,
	pub gen_time: Vec<u64>,
	pub spectate_creature: usize
}

impl OpMethodData {
	pub fn new(generations: Vec<Population>) -> OpMethodData {
		OpMethodData {
			generations: generations,
			gen: 0,
			gen_time: Vec::new(),
			spectate_creature: 0
		}
	}

	pub fn average_gen_time(&self) -> u64 {

		if self.gen_time.len() == 0 { return 0 }

		let mut total = 0u64;
		for time in &self.gen_time {
			total += *time;
		}
		total / self.gen_time.len() as u64
	}
}

pub trait OptimisationMethod {
	fn generation_single    (&mut self, rng: &mut StdRng);
	fn creature_get_fittest (&self, gen: usize) -> &Creature;
	fn creature_get         (&mut self, gen: usize, idx: usize) -> &mut Creature;
	fn get_data_mut         (&mut self) -> &mut OpMethodData;
	fn get_data             (&self) -> &OpMethodData;
}
