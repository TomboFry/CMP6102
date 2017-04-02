use std::cmp::{PartialOrd, Ordering};
use std::ops::Range;
use rand::{Rng, StdRng};
use rand::distributions::range::SampleRange;

// imports used for drawing a creature
use piston_window::{ellipse, line, Context, Graphics};

/// Constants to define a creatures lower and upper exclusive bounds.
/// eg. a creature can have only 2 to 6 nodes. Any less and its useless,
///     any more and it's going to behave like a big mess.
pub const BOUNDS_NODE_COUNT: Range<u8> = 2 .. 7;
pub const BOUNDS_NODE_X: Range<f32> = 0.0 .. 256.0;
pub const BOUNDS_NODE_Y: Range<f32> = 0.0 .. 256.0;
pub const BOUNDS_NODE_FRICTION: Range<f32> = 0.0 .. 1.0;
pub const BOUNDS_MUSCLE_STRENGTH: Range<f32> = 0.2 .. 1.0;

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
	pub x: f32,       // Evolution property
	pub y: f32,       // Evolution property
	pub friction: f32 // Evolution property
}

/// A pair of existing nodes to connect a muscle together
#[derive(Clone)]
pub struct NodePair(pub usize, pub usize);

/// A muscle of a creature, made up of a pair of nodes. Essentially an edge in a graph
#[derive(Clone)]
pub struct Muscle {
	pub nodes: NodePair,
	pub strength: f32, // Evolution property
	pub len: f32,      // Evolution property
	pub len_max: f32,  // Evolution property
	pub len_min: f32   // Evolution property
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
			// Set the node's properties to random values within the bounds.
			let x = BOUNDS_NODE_X.gen(rng);
			let y = BOUNDS_NODE_Y.gen(rng);
			let friction = BOUNDS_NODE_FRICTION.gen(rng);

			Node { x: x, y: y, friction: friction }
		}).collect::<Vec<Node>>();

		// Add a muscle for at least each node.
		let muscles: Vec<Muscle> = (0 .. nodes.len()).map(|idx| {
			let mut idx_other;

			// Make sure the other node is not pointing to the same node as itself
			//   before adding the muscle.
			loop {
				idx_other = rng.gen_range(0, nodes.len());
				if idx_other != idx { break; }
			}

			let nodepair = NodePair(idx, idx_other);
			let len = nodes[idx].distance(&nodes[idx_other]);

			Muscle {
				nodes: nodepair,
				strength: BOUNDS_MUSCLE_STRENGTH.gen(rng),
				len: len,
				len_min: len * 0.8,
				len_max: len * 1.4
			}
		}).collect::<Vec<Muscle>>();

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

	/// Return the two nodes relating to a NodePair in a muscle.
	pub fn get_nodes(&self, nodepair: &NodePair) -> (&Node, &Node) {
		// println!("Number of nodes: {}; Indices: {}, {}", self.nodes.len(), nodepair.0, nodepair.1);
		(&self.nodes[nodepair.0], &self.nodes[nodepair.1])
	}

	/// Adds a muscle to the creature
	pub fn add_muscle(&mut self, muscle: Muscle) {
		self.muscles.push(muscle);
	}

	/// Draws a single creature to the screen
	pub fn draw<G>(&mut self, c: Context, g: &mut G) where G: Graphics {
		// Draw every muscle
		for muscle in &self.muscles {
			// Get the pair of nodes for this specific muscle
			let ref nodes = self.get_nodes(&muscle.nodes);

			// Generate the colour from it using the muscle's strength
			// Get the two node positions to draw the line between
			let col = [0.0, 0.0, 0.0, muscle.strength];
			let coords = [nodes.0.x as f64, nodes.0.y as f64, nodes.1.x as f64, nodes.1.y as f64];

			// Draw the line to the screen
			line(col, 8.0, coords, c.transform, g);
		}

		// Draw every node
		for node in &self.nodes {
			let radius = 12.0;

			// Set the colour of the node based on its friction
			// Make the bounds of the ellipse centered on the node position, rather than
			//   off by a few pixels
			let col: [f32; 4] = [node.friction, 0.0, 0.0, 1.0];
			let rect: [f64; 4] = [node.x as f64 - radius, node.y as f64 - radius, radius * 2.0, radius * 2.0];

			ellipse(col, rect, c.transform, g);
		}
	}

	pub fn calculate_fitness(&mut self) {

		// INCREDIBLY RUDIMENTARY fitness calculation
		// based on the values supplied by it's properties
		// rather than actual physics testing.

		let mut fitness: f32 = 0.0;
		for node in &self.nodes {
			fitness += node.x * 0.75;
			fitness -= node.y * 0.5;
			fitness += node.friction * 8.0;
		}

		for muscle in &self.muscles {
			fitness += muscle.strength * 0.5;
			fitness += muscle.len * 0.6;
		}
		self.fitness = fitness;
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
}
