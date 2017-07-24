use creature::{self, Creature, Node};
use population::Population;
use std::ops::Range;

pub const GRAVITY: f32 = 0.3;
pub const RESISTANCE: f32 = 0.97;
pub const SIM_LENGTH: u32 = 900; // 60 frames per second for 15 seconds

pub fn full_simulation_population(population: &mut Population) {
	for creature in &mut population.creatures {
		full_simulation_creature(creature);
	}
}

pub fn full_simulation_creature(creature: &mut Creature) {
	creature.reset_position();
	for step in 0 .. SIM_LENGTH {
		simulation_step(step, creature);
	}
	creature.calculate_fitness();
	creature.reset_position();
}

pub fn simulation_step(timer: u32, creature: &mut Creature) {
	// Loop through every muscle
	for idx in 0 .. creature.muscles.len() {
		let target: f32;
		let t_ext = creature.muscles[idx].time_extended;
		let t_con = creature.muscles[idx].time_contracted;
		let step = timer % (t_ext + t_con);

		if step < t_con {
			target = creature.muscles[idx].len_max;
			creature.muscles[idx].contracted = false;
		} else {
			target = creature.muscles[idx].len_min;
			creature.muscles[idx].contracted = true;
		}

		force_muscle(creature, idx, target);
	}

	for node in &mut creature.nodes {
		force_node(node);
		wall_collision(node);
	}
}

/// Apply a muscle's force to its two connected nodes' velocity vectors
pub fn force_muscle(creature: &mut Creature, idx: usize, target: f32) {
	let distance =
		creature.nodes[creature.muscles[idx].nodes.0]
		.distance(&creature.nodes[creature.muscles[idx].nodes.1]);

	let angle =
		(creature.nodes[creature.muscles[idx].nodes.0].y -
		 creature.nodes[creature.muscles[idx].nodes.1].y)
		.atan2(creature.nodes[creature.muscles[idx].nodes.0].x -
		       creature.nodes[creature.muscles[idx].nodes.1].x);

	let force = (1.0 - (distance / target)).max(-0.4).min(0.4);

	creature.nodes[creature.muscles[idx].nodes.0].vx +=
		angle.cos() * force * creature.muscles[idx].strength;
	creature.nodes[creature.muscles[idx].nodes.0].vy +=
		angle.sin() * force * creature.muscles[idx].strength;

	creature.nodes[creature.muscles[idx].nodes.1].vx -=
		angle.cos() * force * creature.muscles[idx].strength;
	creature.nodes[creature.muscles[idx].nodes.1].vy -=
		angle.sin() * force * creature.muscles[idx].strength;
}

/// Apply the specified node's velocity vector to its position, including
/// gravity
pub fn force_node(node: &mut Node) {
	node.vy += GRAVITY;

	node.vy *= RESISTANCE;
	node.vx *= RESISTANCE;

	node.x += node.vx;
	node.y += node.vy;
}

/// Check to see if a node is colliding with the floor, and if so prevent it
/// from going through, and applying friction to the node.
pub fn wall_collision(node: &mut Node) {
	let y = node.y;
	// Y position in the world is 0, so anything above that means it's
	// colliding
	if y >= creature::BOUNDS_NODE_Y.end {
		// So the node doesn't get actually drawn in the ground
		node.y = creature::BOUNDS_NODE_Y.end;
		// Reset the velocity of Y as we're against the ground
		node.vy = 0.0;
		node.x -= node.vx * node.friction;
		if node.vx > 0.0 {
			node.vx -= node.friction;
			node.vx = node.vx.max(0.0);
		} else {
			node.vx += node.friction;
			node.vx = node.vx.min(0.0);
		}
	}
}

/// Linear Interpolation, returns the value of a percentage `t` between the two
/// values `v0` and `v1`
pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
	let t1 = clamp(t, 0.0 .. 1.0);
	v0 * (1.0 - t1) + v1 * t1
}

/// Clamps a float between two numbers
pub fn clamp(value: f32, range: Range<f32>) -> f32 {
	value.max(range.start).min(range.end)
}

#[cfg(test)]
mod tests {
	use physics;
	use creature::{self, Creature, Node, Muscle, NodePair};

	/// Create a simple creature that is used for most of the physics tests
	fn simple_creature(start_x: f32, start_y: f32) -> Creature {
		let mut creature = Creature::empty();

		// Create two nodes, one at 0, another at the specified position
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
			x:        start_x,
			y:        start_y,
			start_x:  start_x,
			start_y:  start_y,
			friction: 0.0,
			vx:       0.0,
			vy:       0.0
		});

		creature.add_muscle(Muscle {
			nodes: NodePair(0, 1),
			strength: 1.0,
			len_min: start_x * creature::BOUNDS_MUSCLE_LENGTH.start,
			len: start_x,
			len_max: start_x * creature::BOUNDS_MUSCLE_LENGTH.end,
			time_extended: 60,
			time_contracted: 60,
			contracted: false
		});

		creature
	}

	/// Applies the physics for both nodes connected to a muscle, ensuring the
	/// final velocities for each node are in the correct direction
	#[test]
	fn force_muscle() {
		// Length and target length of the muscle
		let len: f32 = 64.0;
		let target: f32 = len * 1.5;

		let mut creature = simple_creature(len, 0.0);

		// Test the physics for muscle of index 0 (the only one there)
		physics::force_muscle(&mut creature, 0, target);

		// Make sure the X velocity has changed (in the correct direction)
		assert!(creature.nodes[0].vx < 0.0); // Vel will be negative
		assert!(creature.nodes[1].vx > 0.0); // Vel will be positive

		// And also make sure the y velocity hasn't changed
		assert_approx_eq!(creature.nodes[0].vy, 0.0);
		assert_approx_eq!(creature.nodes[1].vy, 0.0);
	}

	/// Apply the velocities created by the force_muscle function to the node
	/// as well as adding gravity, making sure that the positions of each
	/// node have changed accordingly.
	#[test]
	fn force_node() {
		// Length and target length of the muscle
		let len: f32 = 64.0;
		let target: f32 = len * 1.5;

		let mut creature = simple_creature(len, 0.0);

		// Apply the physics for muscle of index 0 (the only one there)
		physics::force_muscle(&mut creature, 0, target);

		for node in &mut creature.nodes {
			let vx = node.vx;
			physics::force_node(node);

			// Make sure it's the expected value, with a small threshold
			// due to floating point accuracies
			assert_approx_eq!(node.x,node.start_x + (vx * physics::RESISTANCE));
			assert_approx_eq!(node.y, physics::GRAVITY * physics::RESISTANCE);
		}
	}

	/// Create a creature where one node is almost touching the ground, and
	/// make sure it stops at the ground.
	#[test]
	fn wall_collision() {
		let mut creature = simple_creature(
			0.0,
			creature::BOUNDS_NODE_Y.end - 4.0
		);

		for _ in 0 .. 60 {
			for node in &mut creature.nodes {
				physics::force_node(node);
				physics::wall_collision(node);
				assert!(node.y <= creature::BOUNDS_NODE_Y.end);
			}
		}
	}

	/// Makes sure linear interpolation function correctly lerps between
	/// two numbers correctly
	#[test]
	fn lerp() {
		let a = 0.0;
		let b = 10.0;

		// Valid values
		assert_approx_eq!(physics::lerp(a, b, 0.0), 0.0);
		assert_approx_eq!(physics::lerp(a, b, 0.1), 1.0);
		assert_approx_eq!(physics::lerp(a, b, 0.2), 2.0);
		assert_approx_eq!(physics::lerp(a, b, 0.3), 3.0);
		assert_approx_eq!(physics::lerp(a, b, 0.4), 4.0);
		assert_approx_eq!(physics::lerp(a, b, 0.5), 5.0);
		assert_approx_eq!(physics::lerp(a, b, 0.6), 6.0);
		assert_approx_eq!(physics::lerp(a, b, 0.7), 7.0);
		assert_approx_eq!(physics::lerp(a, b, 0.8), 8.0);
		assert_approx_eq!(physics::lerp(a, b, 0.9), 9.0);
		assert_approx_eq!(physics::lerp(a, b, 1.0), 10.0);

		// Erroneous Values
		assert_approx_eq!(physics::lerp(a, b, -0.1), 0.0);
		assert_approx_eq!(physics::lerp(a, b, -10.0), 0.0);
		assert_approx_eq!(physics::lerp(a, b, 2.0), 10.0);
	}

	/// Restricts a floating point number between a start and end value
	#[test]
	fn clamp() {

		// Values inside range
		assert_approx_eq!(physics::clamp(5.0, 0.0 .. 10.0), 5.0);
		assert_approx_eq!(physics::clamp(8.0, 0.0 .. 10.0), 8.0);

		// Values outside range on either side
		assert_approx_eq!(physics::clamp(-4.0, 0.0 .. 10.0), 0.0);
		assert_approx_eq!(physics::clamp(12.0, 0.0 .. 10.0), 10.0);
	}
}
