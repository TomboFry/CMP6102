use std::rc::Rc;
use std::cmp::PartialOrd;
use std::ops::Range;
use rand::{Rng, ThreadRng};
use rand::distributions::range::SampleRange;

// imports used for drawing a creature
use piston_window::{ellipse, line, Context, Graphics};

const BOUNDS_NODE_COUNT: Range<u8> = 2..7;
const BOUNDS_NODE_X: Range<f64> = 0.0..256.0;
const BOUNDS_NODE_Y: Range<f64> = 0.0..256.0;
const BOUNDS_NODE_FRICTION: Range<f64> = 0.0..1.0;
const BOUNDS_MUSCLE_STRENGTH: Range<f64> = 0.0..1.0;

// pub struct Property<T> {
// 	pub value: T,
// 	bounds: Range<T>
// }

// Add "gen" function to range, which will return a random value between its lower and upper bounds
pub trait RangeBounds<T> {
	fn gen(&self, rng: &mut ThreadRng) -> T;
}
impl<T: Copy> RangeBounds<T> for Range<T> where T: PartialOrd + SampleRange {
	fn gen(&self, rng: &mut ThreadRng) -> T {
		rng.gen_range(self.start, self.end)
	}
}

#[derive(Clone)]
pub struct NodePair(pub Rc<Node>, pub Rc<Node>);

#[derive(Clone)]
pub struct Creature {
	nodes: Vec<Rc<Node>>,
	muscles: Vec<Muscle>
}

#[derive(Clone)]
pub struct Node {
	pub x: f64, pub y: f64,
	pub friction: f64
}

#[derive(Clone)]
pub struct Muscle {
	pub nodes: NodePair,
	pub strength: f64
}

impl Creature {
	pub fn new() -> Creature {
		Creature {
			nodes: Vec::new(),
			muscles: Vec::new()
		}
	}

	pub fn add_node(&mut self, node: Node) -> Rc<Node> {
		self.nodes.push(Rc::new(node));
		self.nodes.last().unwrap().clone()
	}

	pub fn add_muscle(&mut self, muscle: Muscle) {
		self.muscles.push(muscle);
	}

	pub fn gen_new(rng: &mut ThreadRng) -> Creature {
		// Instantiate a new creature
		let mut creature = Creature::new();

		// Decide how many nodes it should have.
		let num_nodes: u8 = BOUNDS_NODE_COUNT.gen(rng);

		// Create and add nodes to the create, and collect them into a vector
		//   for the muscles to use
		let nodes: Vec<Rc<Node>> = (0..num_nodes).map(|_| {
			// Set the node's properties to random values within the bounds.
			let x = BOUNDS_NODE_X.gen(rng);
			let y = BOUNDS_NODE_Y.gen(rng);
			let friction = BOUNDS_NODE_FRICTION.gen(rng);

			creature.add_node(Node { x: x, y: y, friction: friction })
		}).collect::<Vec<Rc<Node>>>();

		// Add a muscle for at least each node.
		for idx in 0..nodes.len() {
			let mut idx_other = 0;

			// Make sure the other node is not pointing to the same node as itself
			//   before adding the muscle.
			loop {
				idx_other = rng.gen_range(0, nodes.len());
				if idx_other != idx { break; }
			}

			creature.add_muscle(Muscle {
				nodes: NodePair(nodes[idx].clone(), nodes[idx_other].clone()),
				strength: BOUNDS_MUSCLE_STRENGTH.gen(rng)
			});
		}

		// Finally, return the creature to be added to the population
		creature
	}

	pub fn draw<G>(&mut self, c: Context, g: &mut G) where G: Graphics {
		// Draw every muscle
		for muscle in &self.muscles {
			// Get the pair of nodes for this specific muscle
			let ref nodes = muscle.nodes;

			// Generate the colour from it using the muscle's strength
			// Get the two node positions to draw the line between
			let col = [0.0, 0.0, 0.0, muscle.strength as f32];
			let coords = [nodes.0.x, nodes.0.y, nodes.1.x, nodes.1.y];

			// Draw the line to the screen
			line(col, 4.0, coords, c.transform, g);
		}

		// Draw every node
		for node in &self.nodes {
			let radius = 8.0;

			// Set the colour of the node based on its friction
			// Make the bounds of the ellipse centered on the node position, rather than
			//   off by a few pixels
			let col: [f32; 4] = [node.friction as f32, 0.0, 0.0, 1.0];
			let rect: [f64; 4] = [node.x - radius, node.y - radius, radius * 2.0, radius * 2.0];

			ellipse(col, rect, c.transform, g);
		}
	}
}
