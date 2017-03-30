use creature::Creature;

pub struct HillClimbing {
	pub creatures: Vec<Creature>
}

impl HillClimbing {
	pub fn new(creatures: Vec<Creature>) -> HillClimbing {
		HillClimbing {
			creatures: creatures
		}
	}
}
