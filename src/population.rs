use creature::Creature;
use rand::ThreadRng;

pub struct Population {
	pub creatures: Vec<Creature>
}

impl Population {
	pub fn new(pop_size: u32, rng: &mut ThreadRng) -> Population {
		// Generate
		let creatures = (0..pop_size).map(|_| {
			Creature::gen_new(rng)
		}).collect::<Vec<Creature>>();

		Population {
			creatures: creatures
		}
	}
}
