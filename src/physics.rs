use creature::{self, Creature, Node};
use population::Population;

pub const GRAVITY: f32 = 0.2;
pub const RESISTANCE: f32 = 0.95;
pub const SIM_LENGTH: u32 = 900; // 60 frames per second for 15 seconds

pub struct Physics {

}

impl Physics {

	pub fn full_simulation_population(population: &mut Population) {
		for creature in &mut population.creatures {
			Physics::full_simulation_creature(creature);
		}
	}

	pub fn full_simulation_creature(creature: &mut Creature) {
		creature.reset_position();
		for step in 0 .. SIM_LENGTH {
			Physics::simulation_step(step, creature);
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

			Physics::force_muscle(creature, idx, target);
		}

		for node in &mut creature.nodes {
			Physics::force_node(node);
			Physics::wall_collision(node);
		}

		// match io::stdin().read_line(&mut String::new()) {
		// 	_ => {}
		// }
	}

	pub fn force_muscle(creature: &mut Creature, idx: usize, target: f32) {
		let distance = creature.nodes[creature.muscles[idx].nodes.0].distance(&creature.nodes[creature.muscles[idx].nodes.1]);
		let angle = (creature.nodes[creature.muscles[idx].nodes.0].y - creature.nodes[creature.muscles[idx].nodes.1].y).atan2(creature.nodes[creature.muscles[idx].nodes.0].x - creature.nodes[creature.muscles[idx].nodes.1].x);
		let force = (1.0 - (distance / target)).max(-0.4).min(0.4);

		creature.nodes[creature.muscles[idx].nodes.0].vx += angle.cos() * force * creature.muscles[idx].strength;
		creature.nodes[creature.muscles[idx].nodes.0].vy += angle.sin() * force * creature.muscles[idx].strength;

		creature.nodes[creature.muscles[idx].nodes.1].vx -= angle.cos() * force * creature.muscles[idx].strength;
		creature.nodes[creature.muscles[idx].nodes.1].vy -= angle.sin() * force * creature.muscles[idx].strength;
	}

	pub fn force_node(node: &mut Node) {
		node.vy += GRAVITY * RESISTANCE;

		node.x += node.vx;
		node.y += node.vy;
	}

	pub fn wall_collision(node: &mut Node) {
		let y = node.y + creature::NODE_RADIUS;
		// Y position in the world is 0, so anything above that means it's colliding
		if y >= 256.0 {
			// So the node doesn't get actually drawn in the ground
			node.y = 256.0 - creature::NODE_RADIUS;
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
}
