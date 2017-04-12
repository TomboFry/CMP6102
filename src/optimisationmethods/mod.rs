use rand::{Rng, StdRng};
use std::ops::Range;
use creature::{self, Creature};
use population::Population;

pub mod genetic_algorithm;
pub mod hill_climbing;
pub mod simulated_annealing;

pub type GenResult = Result<(), (String, String)>;

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

	pub fn generations_get_fittest(&self) -> f32 {
		let mut max = 0.0;
		for gen in &self.generations {
			let fit = gen.fittest().fitness;
			if fit > max { max = fit; }
		}
		max
	}

	pub fn generations_get_weakest(&self) -> f32 {
		let mut min = 100000.0;
		for gen in &self.generations {
			let fit = gen.weakest().fitness;
			if fit < min { min = fit; }
		}
		min
	}

	pub fn creature_get_fittest (&self, gen: usize) -> &Creature {
		&self.generations[gen].fittest()
	}

	pub fn creature_get_average (&self, gen: usize) -> f32 {
		self.generations[gen].fitness_average()
	}

	pub fn creature_get_weakest (&self, gen: usize) -> &Creature {
		&self.generations[gen].weakest()
	}

	pub fn average_gen_time(&self) -> u64 {
		if self.gen_time.len() == 0 { return 0 }

		let mut total = 0u64;
		for time in &self.gen_time {
			total += *time;
		}
		total / self.gen_time.len() as u64
	}

	pub fn mutate(creature: &Creature, rng: &mut StdRng, rate: f32, node_add: bool, node_remove: bool) -> Creature {
		// Start by cloning the original creature so we can modify the values of the new one
		let mut new_creature = creature.clone();
		let node_len = creature.nodes.len();

		new_creature.reset_position();

		// For each node in the creature
		for node in &mut new_creature.nodes {
			// Modify the values of each property by the specified rate, but still making sure
			//   they are within the bounds of the original creature specifications.
			node.start_x = OpMethodData::mutate_clamp(node.start_x, rate * 40.0, creature::BOUNDS_NODE_X, rng);
			node.start_y = OpMethodData::mutate_clamp(node.start_y, rate * 40.0, creature::BOUNDS_NODE_Y, rng);
			node.friction = OpMethodData::mutate_clamp(node.friction, rate, creature::BOUNDS_NODE_FRICTION, rng);
		}

		// Have the random chance to add a node
		if node_add {
			new_creature.nodes.push(Creature::add_node_random(rng));
		}

		// Have the same random chance to remove a random node
		if node_remove {
			let node = rng.gen_range(0, new_creature.nodes.len());
			new_creature.nodes.remove(node);
		}

		// Make sure any nodes and muscles we've mutated/crossed over did not
		//   leave any node by itself
		Creature::check_lonely_nodes(&new_creature.nodes, &mut new_creature.muscles, rng);

		// Now just make sure there are no muscles creating a cycle in the graph
		new_creature.muscles = Creature::check_colliding_muscles(&new_creature.muscles);

		// Do the same process as above by for the muscles.
		for muscle in &mut new_creature.muscles {
			muscle.range_mut(new_creature.nodes.len(), rng);

			muscle.strength = OpMethodData::mutate_clamp(muscle.strength, rate, creature::BOUNDS_MUSCLE_STRENGTH, rng);

			muscle.len = new_creature.nodes[muscle.nodes.0].distance(&new_creature.nodes[muscle.nodes.1]);
			muscle.len_min = muscle.len * creature::BOUNDS_MUSCLE_LENGTH.start;
			muscle.len_max = muscle.len * creature::BOUNDS_MUSCLE_LENGTH.end;

			muscle.time_extended = OpMethodData::mutate_clamp_int(muscle.time_extended, rate, creature::BOUNDS_MUSCLE_TIME_EXTENDED, rng);
			muscle.time_contracted = OpMethodData::mutate_clamp_int(muscle.time_contracted, rate, creature::BOUNDS_MUSCLE_TIME_CONTRACTED, rng);
		}

		// Finally, return the new creature with the modified values.
		new_creature
	}

	pub fn mutate_clamp(value: f32, rate: f32, range: Range<f32>, rng: &mut StdRng) -> f32 {
		use conrod::utils::clamp;
		clamp(value + rng.gen_range(-rate, rate), range.start, range.end)
	}

	pub fn mutate_clamp_int(value: u32, rate: f32, range: Range<u32>, rng: &mut StdRng) -> u32 {
		use conrod::utils::clamp;
		clamp((value as i32 + rng.gen_range(-rate, rate) as i32) as u32, range.start, range.end)
	}
}

pub trait OptimisationMethod {
	fn generation_single    (&mut self, rng: &mut StdRng) -> GenResult;
	fn creature_get         (&mut self, gen: usize, idx: usize) -> &mut Creature;
	fn get_data_mut         (&mut self) -> &mut OpMethodData;
	fn get_data             (&self) -> &OpMethodData;
}
