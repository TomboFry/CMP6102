use creature::{self, Creature, Node};
use population::Population;
use std::ops::Range;

pub const GRAVITY: f32 = 0.3;
pub const RESISTANCE: f32 = 0.98;
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

pub fn force_node(node: &mut Node) {
	node.vy += GRAVITY;

	node.vy *= RESISTANCE;
	node.vx *= RESISTANCE;

	node.x += node.vx;
	node.y += node.vy;
}

pub fn wall_collision(node: &mut Node) {
	let y = node.y + creature::NODE_RADIUS;
	// Y position in the world is 0, so anything above that means it's
	// colliding
	if y >= creature::BOUNDS_NODE_Y.end {
		// So the node doesn't get actually drawn in the ground
		node.y = creature::BOUNDS_NODE_Y.end - creature::NODE_RADIUS;
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


pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
	v0 * (1.0 - t) + v1 * t
}

pub fn clamp(value: f32, range: Range<f32>) -> f32 {
	value.max(range.start).min(range.end)
}

#[cfg(test)]
mod test {
	#[test]
	#[should_panic]
	fn physics_force_muscle() {
		unimplemented!();
	}

	#[test]
	#[should_panic]
	fn physics_force_node() {
		unimplemented!();
	}
	#[test]
	#[should_panic]
	fn physics_wall_collision() {
		unimplemented!();
	}

	#[test]
	#[should_panic]
	fn physics_simulation_step() {
		unimplemented!();
	}

	#[test]
	#[should_panic]
	fn physics_simulation_creature() {
		unimplemented!();
	}

	#[test]
	#[should_panic]
	fn physics_simulation_population() {
		unimplemented!();
	}
}
