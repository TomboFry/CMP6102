use std::cmp::{PartialOrd, Ordering};
use std::ops::Range;
use rand::{Rng, ThreadRng};
use rand::distributions::range::SampleRange;

/// Constants to define a creatures lower and upper exclusive bounds.
/// eg. a creature can have only 2 to 6 nodes. Any less and its useless,
///     any more and it's going to behave like a big mess.
pub const BOUNDS_NODE_COUNT: Range<u8> = 3 .. 7;
pub const BOUNDS_NODE_X: Range<f32> = 0.0 .. 256.0;
pub const BOUNDS_NODE_Y: Range<f32> = 0.0 .. 256.0;
pub const BOUNDS_NODE_FRICTION: Range<f32> = 0.1 .. 0.95;
pub const BOUNDS_MUSCLE_STRENGTH: Range<f32> = 1.0 .. 10.0;
pub const BOUNDS_MUSCLE_TIME_EXTENDED: Range<u32> = 40 .. 120;
pub const BOUNDS_MUSCLE_TIME_CONTRACTED: Range<u32> = 40 .. 120;

pub const BOUNDS_MUSCLE_LENGTH: Range<f32> = 0.75 .. 1.2;
pub const NODE_RADIUS: f32 = 16.0;

/// Add "gen" function to range, which will return a random
/// value between its lower and upper bounds
pub trait RangeBounds<T> {
	fn gen(&self, rng: &mut ThreadRng) -> T;
}
impl<T: Copy> RangeBounds<T> for Range<T> where T: PartialOrd + SampleRange {
	fn gen(&self, rng: &mut ThreadRng) -> T {
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

/// These traits are implemented so that we can return the creature with
/// highest or lowest fitness simply by calling population.max() or
/// population.min() respectively
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

/// A pair of existing nodes to connect a muscle together
#[derive(Clone)]
pub struct NodePair(pub usize, pub usize);

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

/// A muscle of a creature, made up of a pair of nodes.
/// Essentially an edge in a graph
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
	/// Generates a new creature with random property values within
	/// their bounds
	pub fn new(rng: &mut ThreadRng) -> Creature {
		// Decide how many nodes it should have.
		let num_nodes: u8 = BOUNDS_NODE_COUNT.gen(rng);

		// Create and add nodes to the create, and collect them into a vector
		// for the muscles to use
		let nodes: Vec<Node> = (0 .. num_nodes).map(|_| {
			Creature::add_node_random(rng)
		}).collect::<Vec<Node>>();

		// Add a muscle for at least each node.
		let mut muscles: Vec<Muscle> = (0 .. nodes.len()).map(|_| {
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

	/// Adds a node to the creature, returning the index of that node in the
	/// nodes vector.
	pub fn add_node(&mut self, node: Node) -> usize {
		self.nodes.push(node);
		// self.nodes.last().expect("Error getting last node").clone()
		self.nodes.len()
	}

	/// Generate a node adhering to the property bounds
	pub fn add_node_random(rng: &mut ThreadRng) -> Node {
		// Set the node's properties to random values within the bounds.
		let x = BOUNDS_NODE_X.gen(rng);
		let y = BOUNDS_NODE_Y.gen(rng);
		let friction = BOUNDS_NODE_FRICTION.gen(rng);

		Node {
			x: x, y: y,
			start_x: x, start_y: y,
			friction: friction,
			vx: 0.0, vy: 0.0
		}
	}

	/// Return the two nodes relating to a NodePair in a muscle.
	pub fn get_nodes(&self, nodepair: &NodePair) -> (&Node, &Node) {
		(&self.nodes[nodepair.0], &self.nodes[nodepair.1])
	}

	/// Adds a muscle to a creature
	pub fn add_muscle(&mut self, muscle: Muscle) {
		self.muscles.push(muscle);
	}

	/// Returns a new, randomly generated muscle
	pub fn add_muscle_random(nodes: &Vec<Node>, rng: &mut ThreadRng) -> Muscle {
		Creature::add_muscle_index(rng.gen_range(0, nodes.len()), nodes, rng)
	}

	/// Return a new muscle connecting to a specific node, calculating all
	/// the required properties, such as length.
	pub fn add_muscle_index(
		idx: usize,
		nodes: &Vec<Node>,
		rng: &mut ThreadRng) -> Muscle
	{
		let mut index = idx;
		let mut idx_other;

		// Make sure the other node is not pointing to the same node as itself
		// before adding the muscle.
		loop {
			idx_other = rng.gen_range(0, nodes.len());
			if idx_other != index { break; }
		}

		// Always make sure the lowest node index appears first in the NodePair
		if idx_other < index {
			::std::mem::swap(&mut index, &mut idx_other);
		}

		let nodepair = NodePair(index, idx_other);
		let len = nodes[index].distance(&nodes[idx_other]);

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

	/// Make sure that every node is connected to at least one muscle
	pub fn check_lonely_nodes(
		nodes: &Vec<Node>,
		muscles: &mut Vec<Muscle>,
		rng: &mut ThreadRng)
	{
		for node in 0 .. nodes.len() {
			let mut connections: u32 = 0;
			for muscle in 0 .. muscles.len() {
				if muscles[muscle].nodes.0 == node ||
				   muscles[muscle].nodes.1 == node {
					connections += 1;
				}
			}
			if connections <= 0 {
				muscles.push(Creature::add_muscle_index(node, nodes, rng));
			}
		}
	}

	/// Remove all duplicate muscles that connect to the same two nodes
	pub fn check_colliding_muscles(muscles: &Vec<Muscle>) -> Vec<Muscle> {
		// In order to remove duplicate nodes, we first sort them in order of
		//   lowest node A then lowest node B.
		//   eg. (0, 3),
		//       (0, 4),
		//       (1, 2),
		//       (1, 3),
		//       (2, 3)
		// Then remove any in order that have both the same
		// nodes on either side

		let mut new_muscles = muscles.clone();

		new_muscles.sort_by(|a, b| {
			match a.nodes.0.cmp(&b.nodes.0) {
				Ordering::Equal => a.nodes.1.cmp(&b.nodes.1),
				other => other,
			}
		});

		new_muscles.dedup_by(
			|a, b| a.nodes.0 == b.nodes.0 && a.nodes.1 == b.nodes.1
		);

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

	/// Calculate the creature's fitness by averaging each node's X position
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
	/// Return a muscle based on an existing one, only to make sure the nodes
	/// are actually in range
	pub fn range(&self, max: usize, rng: &mut ThreadRng) -> Muscle {
		let mut new_muscle = self.clone();
		if new_muscle.nodes.0 >= max {
			new_muscle.nodes.0 = rng.gen_range(0, max);
		}
		if new_muscle.nodes.1 >= max {
			new_muscle.nodes.1 = rng.gen_range(0, max);
		}
		new_muscle
	}

	/// Return a muscle based on an existing one, only to make sure the nodes
	/// are actually in range
	pub fn range_mut(&mut self, max: usize, rng: &mut ThreadRng) {
		if self.nodes.0 >= max {
			self.nodes.0 = rng.gen_range(0, max);
		}
		if self.nodes.1 >= max {
			self.nodes.1 = rng.gen_range(0, max);
		}
	}
}

#[cfg(test)]
mod test {
	use creature::*;

	/// Create an empty creature, with no nodes or muscles
	#[test]
	fn create_empty() {
		let creature = Creature::empty();

		assert_eq!(creature.nodes.len(), 0);
		assert_eq!(creature.muscles.len(), 0);
		assert_eq!(creature.fitness, 0.0);
	}

	/// Manually create a node and add it to an empty creature
	#[test]
	fn add_node() {
		let mut creature = Creature::empty();
		let node = Node {
			x:        0.1,
			y:        0.2,
			start_x:  0.3,
			start_y:  0.4,
			friction: 0.5,
			vx:       0.6,
			vy:       0.7
		};

		let node_index = creature.add_node(node);

		// Ensure it was successfully added
		assert_eq!(node_index, 1);
		assert_eq!(creature.nodes.len(), 1);

		// Test a few of the node's properties
		assert_eq!(creature.nodes[0].x, 0.1);
		assert_eq!(creature.nodes[0].y, 0.2);
		assert_eq!(creature.nodes[0].start_x, 0.3);
		assert_eq!(creature.nodes[0].start_y, 0.4);
	}

	/// Create a random node and add it to an empty creature
	#[test]
	fn add_node_random() {
		let mut rng = rand::thread_rng();
		let mut creature = Creature::empty();

		let node = Creature::add_node_random(&mut rng);

		let node_index = creature.add_node(node);

		assert_eq!(node_index, 1);
		assert_eq!(creature.nodes.len(), 1);
	}

	/// Manually create a muscle and add it to an empty creature
	#[test]
	fn add_muscle() {
		let mut rng = rand::thread_rng();
		let mut creature = Creature::empty();

		// Create the two nodes, ignoring the resulting index
		// (as we know they're going to be 1 and 2 respectively)
		let _ = creature.add_node(Creature::add_node_random(&mut rng));
		let _ = creature.add_node(Creature::add_node_random(&mut rng));

		let muscle = Muscle {
			nodes: NodePair(0, 1),
			strength: 1.0,
			len_min: 2.0,
			len: 3.0,
			len_max: 4.0,
			time_extended: 5,
			time_contracted: 6,
			contracted: false
		};

		creature.add_muscle(muscle);

		// Make sure it was added to the muscles vec
		assert_eq!(creature.muscles.len(), 1);

		// Test a few of its properties
		assert_eq!(creature.muscles[0].strength, 1.0);
		assert_eq!(creature.muscles[0].len_min,  2.0);
		assert_eq!(creature.muscles[0].len,      3.0);
	}

	/// Create a muscle with a given first node index
	#[test]
	fn add_muscle_index() {
		let mut rng = rand::thread_rng();
		let mut creature = Creature::empty();
		let _ = creature.add_node(Creature::add_node_random(&mut rng));
		let _ = creature.add_node(Creature::add_node_random(&mut rng));

		let muscle = Creature::add_muscle_index(
			0,                   // Use the first node;
			&mut creature.nodes, // Pass in the nodes -
			&mut rng             // - and the RNG thread
		);

		// Ensure that the two nodes indices created by the function are not
		// pointing to the same
		assert_ne!(muscle.nodes.0, muscle.nodes.1);

		creature.add_muscle(muscle);

		assert_eq!(creature.muscles.len(), 1);
	}

	/// Create a random muscle and add it to an empty creature
	#[test]
	fn add_muscle_random() {
		let mut rng = rand::thread_rng();
		let mut creature = Creature::empty();
		let _ = creature.add_node(Creature::add_node_random(&mut rng));
		let _ = creature.add_node(Creature::add_node_random(&mut rng));

		let muscle = Creature::add_muscle_random(
			&mut creature.nodes,
			&mut rng
		);

		creature.add_muscle(muscle);

		assert_eq!(creature.muscles.len(), 1);
	}

	/// Make sure each node is connected to a muscle
	#[test]
	fn empty_nodes() {
		let mut rng = rand::thread_rng();
		let mut creature = Creature::empty();

		// Create four nodes
		for _ in 0 .. 3 {
			let _ = creature.add_node(Creature::add_node_random(&mut rng));
		}

		// Create only ONE muscle, not the required minimum of two
		let muscle = Creature::add_muscle_random(
			&mut creature.nodes,
			&mut rng
		);
		creature.add_muscle(muscle);

		// Ensure there is currently only one muscle
		assert_eq!(creature.muscles.len(), 1);

		Creature::check_lonely_nodes(
			&creature.nodes,
			&mut creature.muscles,
			&mut rng
		);

		// Check that other muscles have been added
		assert_eq!(creature.muscles.len(), 2);
	}

	/// Make sure any duplicate muscles get removed
	#[test]
	fn colliding_muscles() {
		let mut rng = rand::thread_rng();
		let mut creature = Creature::empty();

		// Create two nodes
		for _ in 0 .. 2 {
			let _ = creature.add_node(Creature::add_node_random(&mut rng));
		}

		// Create a muscle and duplicate it
		let mut dup_muscles = Vec::with_capacity(2);
		let muscle = Creature::add_muscle_random(
			&mut creature.nodes,
			&mut rng
		);
		dup_muscles.push(muscle.clone());
		dup_muscles.push(muscle);

		// Make sure they're the same muscle by checking the NodePairs
		assert_eq!(dup_muscles.len(), 2);
		assert!(
			dup_muscles[0].nodes.0 == dup_muscles[1].nodes.0 &&
			dup_muscles[0].nodes.1 == dup_muscles[1].nodes.1
		);

		// Run the function and get a new vector of muscles with the duplicates
		// removed.
		let new_muscles = Creature::check_colliding_muscles(&dup_muscles);

		// Now make sure there is only one muscle in this vector.
		assert_eq!(new_muscles.len(), 1);
	}

	/// Make sure the calculated fitness of a randomly generated creature is
	/// within the correct bounds
	#[test]
	fn calc_fitness_random_creature() {
		let mut rng = rand::thread_rng();
		let creature = Creature::new(&mut rng);

		// Get the fitness of the newly generated creature
		let fitness = creature.fitness();

		// Upon initial creation, make sure the fitness is smaller than the
		// maximum X bounds, pivoted around 0.
		assert!(
			fitness < (BOUNDS_NODE_X.end * 0.5) &&
			fitness >= -(BOUNDS_NODE_X.end * 0.5)
		);
	}

	/// Make sure the calculated fitness of a specifically generated creature
	/// has the expected value
	#[test]
	fn calc_fitness_manual_creature() {
		let mut creature = Creature::empty();

		// Create two nodes, one at 0, another at 64
		let _ = creature.add_node(Node {
			x:        0.0,
			y:        0.0,
			start_x:  0.0,
			start_y:  0.0,
			friction: 0.0,
			vx:       0.0,
			vy:       0.0
		});

		let _ = creature.add_node(Node {
			x:        64.0,
			y:        0.0,
			start_x:  0.0,
			start_y:  0.0,
			friction: 0.0,
			vx:       0.0,
			vy:       0.0
		});

		// Get the fitness of the creature
		let fitness = creature.fitness();

		// Upon initial creation, make sure the fitness is
		//  = ((0 + 64) / 2) - (BOUNDS_NODE_X.end * 0.5)
		//  = 32 - (256 / 2)
		//  = 32 - 128
		//  = -96
		assert_eq!(fitness, -96.0);
	}

	/// Make sure a single created creature has properties within the
	/// specified bounds
	#[test]
	fn create_bounds() {
		let mut rng = rand::thread_rng();
		let creature = Creature::new(&mut rng);

		assert!(
		  (creature.nodes.len() < BOUNDS_NODE_COUNT.end as usize) &&
		  (creature.nodes.len() >= BOUNDS_NODE_COUNT.start as usize)
		);

		for node in creature.nodes {
			assert!((node.x >= BOUNDS_NODE_X.start) &&
			        (node.x < BOUNDS_NODE_X.end));
			assert!((node.y >= BOUNDS_NODE_Y.start) &&
			        (node.y < BOUNDS_NODE_Y.end));
			assert!((node.friction >= BOUNDS_NODE_FRICTION.start) &&
			        (node.friction < BOUNDS_NODE_FRICTION.end));
		}
		for muscle in creature.muscles {
			assert!(
				(muscle.strength >= BOUNDS_MUSCLE_STRENGTH.start) &&
				(muscle.strength < BOUNDS_MUSCLE_STRENGTH.end)
			);

			assert!(
				(muscle.time_extended >= BOUNDS_MUSCLE_TIME_EXTENDED.start) &&
				(muscle.time_extended < BOUNDS_MUSCLE_TIME_EXTENDED.end)
			);

			assert!(
				(muscle.time_contracted >=
				 BOUNDS_MUSCLE_TIME_CONTRACTED.start) &&
				(muscle.time_contracted <
				 BOUNDS_MUSCLE_TIME_CONTRACTED.end)
			);
		}
	}

	#[test]
	fn node_distance() {
		let node_a = Node {
			x:        0.0,
			y:        0.0,
			start_x:  0.0,
			start_y:  0.0,
			friction: 0.0,
			vx:       0.0,
			vy:       0.0
		};

		let node_b = Node {
			x:        64.0,
			y:        0.0,
			start_x:  0.0,
			start_y:  0.0,
			friction: 0.0,
			vx:       0.0,
			vy:       0.0
		};

		let node_c = Node {
			x:        64.0,
			y:        64.0,
			start_x:  0.0,
			start_y:  0.0,
			friction: 0.0,
			vx:       0.0,
			vy:       0.0
		};

		let distance_ab = node_a.distance(&node_b);
		let distance_bc = node_b.distance(&node_c);
		let distance_ac = node_a.distance(&node_c);

		assert_eq!(distance_ab, 64.0);
		assert_eq!(distance_bc, 64.0);
		assert!(distance_ac >= 90.5 && distance_ac < 90.6);
	}
}
