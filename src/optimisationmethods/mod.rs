use rand::{Rng, ThreadRng};
use std::ops::Range;
use creature::{self, Creature};
use population::Population;
use std::string::String;

pub mod genetic_algorithm;
pub mod hill_climbing;
pub mod simulated_annealing;

const PROB_NODE_CHANGE: f32 = 8.0; // will be 1 / x

/// GenResult
/// Expects Ok(()) or Err((Title, Message))
pub type GenResult = Result<(), (String, String)>;

pub struct OpMethodData {
	pub generations: Vec<Population>,
	pub gen: usize,
	pub gen_time: Vec<f32>,
	pub spectate_creature: usize,
	pub title: String,
	pub print: bool
}

impl OpMethodData {
	pub fn new(
		generations: Vec<Population>,
		title: String,
		print: bool
	) -> OpMethodData {
		OpMethodData {
			generations: generations,
			gen: 0,
			gen_time: Vec::new(),
			spectate_creature: 0,
			title: title,
			print: print
		}
	}

	/// Returns the highest fitness value from the entire data structure
	pub fn generations_get_fittest(&self) -> f32 {
		let mut max = 0.0;
		for gen in &self.generations {
			let fit = gen.fittest().fitness;
			if fit > max { max = fit; }
		}
		max
	}

	/// Returns the lowest fitness value from the entire data structure
	pub fn generations_get_weakest(&self) -> f32 {
		let mut min = 100000.0;
		for gen in &self.generations {
			let fit = gen.weakest().fitness;
			if fit < min { min = fit; }
		}
		min
	}

	/// Returns the generation's index of which the
	/// fittest creature in the entire data structure exists
	pub fn generations_get_fittest_gen(&self) -> usize {
		let mut max_gen = 0;
		let mut max = 0.0;
		for gen in 0 .. self.generations.len() {
			let fit = self.generations[gen].fittest().fitness;
			if fit > max { max = fit; max_gen = gen; }
		}
		max_gen
	}

	/// Returns the generation's index of which the
	/// weakest creature in the entire data structure exists
	pub fn generations_get_weakest_gen(&self) -> usize {
		let mut min_gen = 0;
		let mut min = 0.0;
		for gen in 0 .. self.generations.len() {
			let fit = self.generations[gen].weakest().fitness;
			if fit < min { min = fit; min_gen = gen; }
		}
		min_gen
	}

	/// Returns the fittest creature from a specified generation
	pub fn creature_get_fittest (&self, gen: usize) -> &Creature {
		&self.generations[gen].fittest()
	}

	/// Returns the average fitness of a population
	pub fn creature_get_average (&self, gen: usize) -> f32 {
		self.generations[gen].fitness_average()
	}

	/// Returns the weakest creature from a specified generation
	pub fn creature_get_weakest (&self, gen: usize) -> &Creature {
		&self.generations[gen].weakest()
	}

	/// Returns the average time spent running the `generation_single` function
	pub fn average_gen_time(&self) -> f32 {
		if self.gen_time.len() == 0 { return 0.0 }

		let mut total: f32 = 0.0;
		for time in &self.gen_time {
			total += *time;
		}
		total / self.gen_time.len() as f32
	}

	/// Mutates a creature by adding/removing nodes and muscles, as well as
	/// slightly modifying their evolution properties
	pub fn mutate(creature: &Creature, rng: &mut ThreadRng, rate: f32)
	    -> Creature {
		// Start by cloning the original creature so we can modify the values
		// of the new one
		let mut new_creature = creature.clone();

		new_creature.reset_position();

		// Have the same random chance to remove a random node
		if rng.gen::<f32>() * (PROB_NODE_CHANGE + 8.0) <= 1.0 &&
			(creature.nodes.len() as u8) >
			creature::BOUNDS_NODE_COUNT.start
		{
			let node = rng.gen_range(0, new_creature.nodes.len());
			new_creature.nodes.remove(node);
		}

		// Have the random chance to add a node
		if rng.gen::<f32>() * PROB_NODE_CHANGE <= 1.0 &&
			(creature.nodes.len() as u8) <
			creature::BOUNDS_NODE_COUNT.end
		{
			new_creature.nodes.push(Creature::add_node_random(rng));
		}

		// Have the same random chance to remove a random muscle
		if rng.gen::<f32>() * PROB_NODE_CHANGE <= 1.0 {
			let muscle = rng.gen_range(0, new_creature.muscles.len());
			new_creature.muscles.remove(muscle);
		}

		// Have the random chance to add a muscle
		if rng.gen::<f32>() * PROB_NODE_CHANGE <= 1.0 {
			new_creature.muscles.push(
				Creature::add_muscle_random(&new_creature.nodes, rng)
			);
		}

		// Now just make sure there are no muscles creating a
		// cycle in the graph
		new_creature.muscles = Creature::check_colliding_muscles(
			&new_creature.muscles
		);

		// Make sure any nodes and muscles we've mutated/crossed over did not
		// leave any node by itself
		Creature::check_lonely_nodes(
			&new_creature.nodes,
			&mut new_creature.muscles,
			rng
		);

		// For each node in the creature
		for node in &mut new_creature.nodes {
			// Modify the values of each property by the specified rate, but
			// still making sure they are within the bounds of the original
			// creature specifications.
			node.start_x = OpMethodData::mutate_clamp(
				node.start_x,            // Value to change
				rate * 40.0,             // Maximum variation
				creature::BOUNDS_NODE_X, // Adhering to these bounds
				rng                      // Seeded RNG thread
			);

			node.start_y = OpMethodData::mutate_clamp(
				node.start_y,
				rate * 40.0,
				creature::BOUNDS_NODE_Y,
				rng
			);

			node.friction = OpMethodData::mutate_clamp(
				node.friction,
				rate,
				creature::BOUNDS_NODE_FRICTION,
				rng
			);
		}

		// Do the same process as above by for the muscles.
		for muscle in &mut new_creature.muscles {
			muscle.range_mut(new_creature.nodes.len(), rng);

			muscle.strength =
				OpMethodData::mutate_clamp(
					muscle.strength,
					rate,
					creature::BOUNDS_MUSCLE_STRENGTH,
					rng
				);

			muscle.len =
				new_creature
				.nodes[muscle.nodes.0]
				.distance(&new_creature.nodes[muscle.nodes.1]);

			muscle.len_min = muscle.len * creature::BOUNDS_MUSCLE_LENGTH.start;
			muscle.len_max = muscle.len * creature::BOUNDS_MUSCLE_LENGTH.end;

			muscle.time_extended = OpMethodData::mutate_clamp_int(
				muscle.time_extended,
				rate * 8.0,
				creature::BOUNDS_MUSCLE_TIME_EXTENDED,
				rng
			);

			muscle.time_contracted = OpMethodData::mutate_clamp_int(
				muscle.time_contracted,
				rate * 8.0,
				creature::BOUNDS_MUSCLE_TIME_CONTRACTED,
				rng
			);
		}

		// Finally, return the new creature with the modified values.
		new_creature
	}

	/// Takes a floating-point number and mutates it slightly
	/// *within the specified bounds*
	pub fn mutate_clamp(
		value: f32,
		rate: f32,
		range: Range<f32>,
		rng: &mut ThreadRng
	) -> f32 {
		(value + rng.gen_range(-rate, rate)).max(range.start).min(range.end)
	}

	/// Takes an integer and mutates it slightly *within the specified bounds*
	pub fn mutate_clamp_int(
		value: u32,
		rate: f32,
		range: Range<u32>,
		rng: &mut ThreadRng
	) -> u32 {
		::std::cmp::max(
			range.start,
			::std::cmp::min(
				(value as i32 + rng.gen_range(-rate, rate) as i32) as u32,
				range.end
			)
		)
	}
}

pub trait OptimisationMethod {
	fn generation_single (&mut self) -> GenResult;
	fn creature_get      (&mut self, gen: usize, idx: usize) -> &mut Creature;
	fn get_data_mut      (&mut self) -> &mut OpMethodData;
	fn get_data          (&self) -> &OpMethodData;
}

#[cfg(test)]
mod tests {
	use rand;
	use optimisationmethods::OpMethodData;
	use population::Population;

	/// Create a struct with two populations, both with one creature in,
	/// where gen0 has a fitness of 0, and gen1 has a fitness of 100
	fn om_setup_single() -> OpMethodData {
		let mut rng = rand::thread_rng();
		let mut population_a = Population::new(1, &mut rng);
		let mut population_b = population_a.clone();
		population_a.creatures[0].fitness = 0.0;
		population_b.creatures[0].fitness = 100.0;

		// Return the new struct
		OpMethodData::new(
			vec![population_a, population_b],
			"Single".to_string(),
			true
		)
	}

	/// Create a struct with two populations, both with **two** creatures in
	/// where gen0 has an average fitness of 50, and gen1 with 150, and the
	/// average gen time is 150
	fn om_setup_double() -> OpMethodData {
		let mut rng = rand::thread_rng();
		let mut population_a = Population::new(2, &mut rng);
		let mut population_b = population_a.clone();
		population_a.creatures[0].fitness = 0.0;
		population_a.creatures[1].fitness = 100.0;
		population_b.creatures[0].fitness = 100.0;
		population_b.creatures[1].fitness = 200.0;

		// Return the new struct
		let mut om = OpMethodData::new(
			vec![population_a, population_b],
			"Double".to_string(),
			true
		);

		// Set the time spent generating for each generation to 100ms and 200ms
		// respectively
		om.gen_time = vec![100u64, 200u64];

		om
	}

	/// Create two generations and make sure it returns the fitter of the two
	#[test]
	fn generations_get_fittest() {
		assert_approx_eq!(om_setup_single().generations_get_fittest(), 100.0);
	}

	/// Create two generations and make sure it returns the weaker of the two
	#[test]
	fn generations_get_weakest() {
		assert_approx_eq!(om_setup_single().generations_get_weakest(), 0.0);
	}

	/// Create two generations and make sure it returns the fitter generation
	/// index of the two
	#[test]
	fn generations_get_fittest_gen() {
		assert_eq!(om_setup_single().generations_get_fittest_gen(), 1);
	}

	/// Create two generations and make sure it returns the fitter generation
	/// index of the two
	#[test]
	fn generations_get_weakest_gen() {
		assert_eq!(om_setup_single().generations_get_weakest_gen(), 0);
	}

	/// Make sure that the function returns the highest fitness creature
	/// for any given generation
	#[test]
	fn creature_get_fittest() {
		let om = om_setup_double();
		assert_approx_eq!(om.creature_get_fittest(0).fitness, 100.0);
		assert_approx_eq!(om.creature_get_fittest(1).fitness, 200.0);
	}

	/// Make sure that the function returns the lowest fitness creature
	/// for any given generation
	#[test]
	fn creature_get_weakest() {
		let om = om_setup_double();
		assert_approx_eq!(om.creature_get_weakest(0).fitness, 0.0);
		assert_approx_eq!(om.creature_get_weakest(1).fitness, 100.0);
	}

	/// Make sure that the function returns the average fitness value for any
	/// given generation
	#[test]
	fn creature_get_average() {
		let om = om_setup_double();
		assert_approx_eq!(om.creature_get_average(0), 50.0);
		assert_approx_eq!(om.creature_get_average(1), 150.0);
	}

	/// Make sure the function returns the correct average time spent running
	/// the `generation_single` function. (Values in the vector are hard-coded)
	#[test]
	fn average_gen_time() {
		let om = om_setup_double();
		assert_eq!(om.average_gen_time(), 150u64);
	}

	/// Make sure the floating-point value is randomly mutated within the
	/// specified bounds
	#[test]
	fn mutate_clamp() {
		let mut rng = rand::thread_rng();

		// Mutate a value on the upper bounds by a maximum of 1, ensuring it's
		// still equal to or less than the upper bounds, and hasn't mutated
		// more than the specified amount
		let mutate_a = OpMethodData::mutate_clamp(
			10.0, 1.0, 0.0 .. 10.0, &mut rng
		);
		assert!(mutate_a <= 10.0 && mutate_a >= 9.0);

		// Mutate a value in the middle of the bounds by a maximum greater than
		// the width of the bounds, ensuring it's still within the bounds.
		let mutate_b = OpMethodData::mutate_clamp(
			10.0, 5.0, 8.0 .. 12.0, &mut rng
		);
		assert!(mutate_b <= 12.0 && mutate_b >= 8.0);
	}

	/// Make sure the integer is randomly mutated within the specified bounds
	#[test]
	fn mutate_clamp_int() {
		let mut rng = rand::thread_rng();

		// Mutate a value on the upper bounds by a maximum of 5, ensuring it's
		// still equal to or less than the upper bounds, and hasn't mutated
		// more than the specified amount
		let mutate_a = OpMethodData::mutate_clamp_int(
			10, 5.0, 0 .. 10, &mut rng
		);
		assert!(mutate_a <= 10 && mutate_a >= 5);

		// Mutate a value in the middle of the bounds by a maximum greater than
		// the width of the bounds, ensuring it's still within the bounds.
		let mutate_b = OpMethodData::mutate_clamp_int(
			10, 5.0, 8 .. 12, &mut rng
		);
		assert!(mutate_b <= 12 && mutate_b >= 8);
	}

}
