use rand::StdRng;
use creature::Creature;
use population::Population;

pub mod genetic_algorithm;
pub mod hill_climbing;
// pub mod simulated_annealing;

pub struct OpMethodData {
	pub generations: Vec<Population>,
	pub gen: usize
}

impl OpMethodData {
	pub fn new(generations: Vec<Population>) -> OpMethodData {
		OpMethodData {
			generations: generations,
			gen: 0
		}
	}
}

pub trait OptimisationMethod {
	fn generation_single    (&mut self, rng: &mut StdRng);
	fn creature_get_fittest (&self, gen: usize) -> Creature;
	fn creature_get         (&mut self, gen: usize, idx: usize) -> &mut Creature;
	fn get_data             (&mut self) -> &mut OpMethodData;
}
