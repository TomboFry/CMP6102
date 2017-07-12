extern crate rand;
extern crate time;

pub mod creature;
pub mod population;
pub mod optimisationmethods;
pub mod physics;

mod test {
	use rand::StdRng;

	/// Used for testing, creates a Seeded RNG Thread:
	pub fn rng_init() -> StdRng {
		if let Ok(rng) = StdRng::new() {
			rng
		} else {
			panic!("Could not create RNG");
		}
	}
}
