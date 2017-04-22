use std::cmp::{PartialOrd, Ordering};
use std::ops::Range;
use rand::{Rng, StdRng};
use rand::distributions::range::SampleRange;

/// Constants to define a creatures lower and upper exclusive bounds.
/// eg. a creature can have only 2 to 6 nodes. Any less and its useless,
///     any more and it's going to behave like a big mess.
pub const BOUNDS_NODE_COUNT: Range<u8> = 3 .. 7;
pub const BOUNDS_NODE_X: Range<f32> = 0.0 .. 256.0;
pub const BOUNDS_NODE_Y: Range<f32> = 0.0 .. 248.0;
pub const BOUNDS_NODE_FRICTION: Range<f32> = 0.05 .. 0.95;
pub const BOUNDS_MUSCLE_STRENGTH: Range<f32> = 1.0 .. 10.0;
pub const BOUNDS_MUSCLE_TIME_EXTENDED: Range<u32> = 10 .. 120;
pub const BOUNDS_MUSCLE_TIME_CONTRACTED: Range<u32> = 10 .. 120;

pub const BOUNDS_MUSCLE_LENGTH: Range<f32> = 0.85 .. 1.25;
pub const NODE_RADIUS: f32 = 16.0;

/// Add "gen" function to range, which will return a random value between its lower and upper bounds
pub trait RangeBounds<T> {
	fn gen(&self, rng: &mut StdRng) -> T;
}
impl<T: Copy> RangeBounds<T> for Range<T> where T: PartialOrd + SampleRange {
	fn gen(&self, rng: &mut StdRng) -> T {
		rng.gen_range(self.start, self.end)
	}
}

/// The main creature structure, made up of nodes and muscles
#[derive(Clone)]
pub struct Creature {
	pub nodes: Vec<Node>,
	pub muscles: Vec<Muscle>,
	pub fitness: f32
}

/// These traits are implemented so that we can return the creature with highest or lowest fitness
///   simply by calling population.max() or population.min() respectively
impl PartialEq for Creature {
	fn eq(&self, other: &Creature) -> bool {
		self.fitness == other.fitness
	}

	fn ne(&self, other: &Creature) -> bool {
		self.fitness != other.fitness
	}
}

impl Eq for Creature {}

impl PartialOrd for Creature {
	fn partial_cmp(&self, other: &Creature) -> Option<Ordering> {
		self.fitness.partial_cmp(&other.fitness)
	}
}

impl Ord for Creature {
	fn cmp(&self, other: &Creature) -> Ordering {
		match self.fitness.partial_cmp(&other.fitness) {
			Some(ord) => ord,
			None => Ordering::Greater
		}
	}
}

/// A node of a creature, essentially a vertex in a graph
#[derive(Clone)]
pub struct Node {
	pub x: f32,        // Evolution property
	pub y: f32,        // Evolution property
	pub start_x: f32,
	pub start_y: f32,
	pub friction: f32, // Evolution property
	pub vx: f32,       // Physics property
	pub vy: f32        // Physics property
}

/// A pair of existing nodes to connect a muscle together
#[derive(Clone)]
pub struct NodePair(pub usize, pub usize);

/// A muscle of a creature, made up of a pair of nodes. Essentially an edge in a graph
#[derive(Clone)]
pub struct Muscle {
	pub nodes: NodePair,
	pub strength: f32,        // Evolution property
	pub len: f32,             // Evolution property
	pub len_max: f32,         // Evolution property
	pub len_min: f32,         // Evolution property
	pub time_extended: u32,   // Evolution property
	pub time_contracted: u32, // Evolution property
	pub contracted: bool
}

impl Creature {
	/// Generates a new creature with random property values within their bounds
	pub fn new(rng: &mut StdRng) -> Creature {
		// Instantiate a new creature
		//let mut creature = Creature::empty();

		// Decide how many nodes it should have.
		let num_nodes: u8 = BOUNDS_NODE_COUNT.gen(rng);

		// Create and add nodes to the create, and collect them into a vector
		//   for the muscles to use
		let nodes: Vec<Node> = (0 .. num_nodes).map(|_| {
			Creature::add_node_random(rng)
		}).collect::<Vec<Node>>();

		// Add a muscle for at least each node.
		let mut muscles: Vec<Muscle> = (0 .. nodes.len()).map(|idx| {
			Creature::add_muscle_random(&nodes, rng)
		}).collect::<Vec<Muscle>>();

		Creature::check_lonely_nodes(&nodes, &mut muscles, rng);
		muscles = Creature::check_colliding_muscles(&muscles);

		// Finally, return the creature to be added to the population
		Creature {
			nodes: nodes,
			muscles: muscles,
			fitness: 0.0
		}
	}

	/// Creates an empty creature
	pub fn empty() -> Creature {
		Creature {
			nodes: Vec::new(),
			muscles: Vec::new(),
			fitness: 0.0
		}
	}

	/// Adds a node to the creature, returning a reference counted version of that same Node
	pub fn add_node(&mut self, node: Node) -> usize {
		self.nodes.push(node);
		// self.nodes.last().expect("Error getting last node").clone()
		self.nodes.len()
	}

	pub fn add_node_random(rng: &mut StdRng) -> Node {
		// Set the node's properties to random values within the bounds.
		let x = BOUNDS_NODE_X.gen(rng);
		let y = BOUNDS_NODE_Y.gen(rng);
		let friction = BOUNDS_NODE_FRICTION.gen(rng);

		Node {
			x: x, y: y,
			start_x: x, start_y: y,
			friction: friction,
			vx: 0.0, vy: 0.0 }
	}

	/// Return the two nodes relating to a NodePair in a muscle.
	pub fn get_nodes(&self, nodepair: &NodePair) -> (&Node, &Node) {
		(&self.nodes[nodepair.0], &self.nodes[nodepair.1])
	}

	/// Adds a muscle to the creature
	pub fn add_muscle(&mut self, muscle: Muscle) {
		self.muscles.push(muscle);
	}

	pub fn add_muscle_random(nodes: &Vec<Node>, rng: &mut StdRng) -> Muscle {
		Creature::add_muscle_index(rng.gen_range(0, nodes.len()), nodes, rng)
	}

	pub fn add_muscle_index(idx: usize, nodes: &Vec<Node>, rng: &mut StdRng) -> Muscle {
		let mut index = idx;
		let mut idx_other;

		// Make sure the other node is not pointing to the same node as itself
		//   before adding the muscle.
		loop {
			idx_other = rng.gen_range(0, nodes.len());
			if idx_other != index { break; }
		}

		if idx_other < index {
			::std::mem::swap(&mut index, &mut idx_other);
		}

		let nodepair = NodePair(index, idx_other);
		let len = nodes[index].distance(&nodes[idx_other]);

		// println!("{} {}", index, idx_other);

		Muscle {
			nodes: nodepair,
			strength: BOUNDS_MUSCLE_STRENGTH.gen(rng),
			len: len,
			len_min: len * BOUNDS_MUSCLE_LENGTH.start,
			len_max: len * BOUNDS_MUSCLE_LENGTH.end,
			time_extended: BOUNDS_MUSCLE_TIME_EXTENDED.gen(rng),
			time_contracted: BOUNDS_MUSCLE_TIME_CONTRACTED.gen(rng),
			contracted: false
		}
	}

	pub fn check_lonely_nodes(nodes: &Vec<Node>, muscles: &mut Vec<Muscle>, rng: &mut StdRng) {
		for node in 0 .. nodes.len() {
			let mut connections: u32 = 0;
			for muscle in 0 .. muscles.len() {
				if muscles[muscle].nodes.0 == node || muscles[muscle].nodes.1 == node {
					connections += 1;
				}
			}
			if connections <= 1 {
				muscles.push(Creature::add_muscle_index(node, nodes, rng));
			}
		}
	}

	pub fn check_colliding_muscles(muscles: &Vec<Muscle>) -> Vec<Muscle> {
		// In order to remove duplicate nodes, we first sort them in order of
		//   lowest node A then lowest node B.
		//   eg. (0, 3),
		//       (0, 4),
		//       (1, 2),
		//       (1, 3),
		//       (2, 3)
		//   Then remove any in order that have both the same nodes on either side

		let mut new_muscles = muscles.clone();

		new_muscles.sort_by(|a, b| {
			match a.nodes.0.cmp(&b.nodes.0) {
				Ordering::Equal => a.nodes.1.cmp(&b.nodes.1),
				other => other,
			}
		});

		new_muscles.dedup_by(|a, b| a.nodes.0 == b.nodes.0 && a.nodes.1 == b.nodes.1);

		new_muscles
	}

	pub fn reset_position(&mut self) {
		for node in &mut self.nodes {
			node.x = node.start_x;
			node.y = node.start_y;
			node.vx = 0.0;
			node.vy = 0.0;
		}
	}

	pub fn calculate_fitness(&mut self) {
		self.fitness = self.fitness();
	}

	pub fn fitness(&self) -> f32 {
		let mut fitness = 0.0;
		let node_len = self.nodes.len();

		for node in &self.nodes {
			fitness += node.x;
		}

		(fitness / node_len as f32) - (BOUNDS_NODE_X.end / 2.0)
	}
}

impl Node {
	/// Returns the distance between one node and another using pythagoras
	pub fn distance(&self, to: &Node) -> f32 {
		let xx = (self.x - to.x) * (self.x - to.x);
		let yy = (self.y - to.y) * (self.y - to.y);

		(xx + yy).sqrt()
	}
}

impl Muscle {
	/// Return a muscle based on an existing one, only to make sure the nodes are actually in range
	pub fn range(&self, max: usize, rng: &mut StdRng) -> Muscle {
		let mut new_muscle = self.clone();
		if new_muscle.nodes.0 >= max {
			new_muscle.nodes.0 = rng.gen_range(0, max);
		}
		if new_muscle.nodes.1 >= max {
			new_muscle.nodes.1 = rng.gen_range(0, max);
		}
		new_muscle
	}

	/// Return a muscle based on an existing one, only to make sure the nodes are actually in range
	pub fn range_mut(&mut self, max: usize, rng: &mut StdRng) {
		if self.nodes.0 >= max {
			self.nodes.0 = rng.gen_range(0, max);
		}
		if self.nodes.1 >= max {
			self.nodes.1 = rng.gen_range(0, max);
		}
	}
}

mod tests {

	/// Make sure a single created creature has properties within the specified bounds
	#[test]
	fn creature_create_bounds() {
		use creature::{self, Creature};

		let mut rng = ::tests::init();
		let creature = Creature::new(&mut rng);

		assert!((creature.nodes.len() < creature::BOUNDS_NODE_COUNT.end as usize) && (creature.nodes.len() >= creature::BOUNDS_NODE_COUNT.start as usize));

		for node in creature.nodes {
			assert!((node.x >= creature::BOUNDS_NODE_X.start) && (node.x < creature::BOUNDS_NODE_X.end));
			assert!((node.y >= creature::BOUNDS_NODE_Y.start) && (node.y < creature::BOUNDS_NODE_Y.end));
			assert!((node.friction >= creature::BOUNDS_NODE_FRICTION.start) && (node.friction < creature::BOUNDS_NODE_FRICTION.end));
		}
		for muscle in creature.muscles {
			assert!((muscle.strength >= creature::BOUNDS_MUSCLE_STRENGTH.start) && (muscle.strength < creature::BOUNDS_MUSCLE_STRENGTH.end));
			assert!((muscle.time_extended >= creature::BOUNDS_MUSCLE_TIME_EXTENDED.start) && (muscle.time_extended < creature::BOUNDS_MUSCLE_TIME_EXTENDED.end));
			assert!((muscle.time_contracted >= creature::BOUNDS_MUSCLE_TIME_CONTRACTED.start) && (muscle.time_contracted < creature::BOUNDS_MUSCLE_TIME_CONTRACTED.end));
		}
	}
}
