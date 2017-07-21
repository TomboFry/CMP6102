extern crate cmp6102;
extern crate rand;

use cmp6102::population::Population;
use rand::ThreadRng;

/// Initialise the data required for the integration tests
pub fn init(pop_size: usize) -> (ThreadRng, Population){
	Population::new(
		pop_size,
		&mut rand::thread_rng()
	)
}
