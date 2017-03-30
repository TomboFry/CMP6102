use population::Population;

pub struct GeneticAlgorithm {
	pub population: Population
}

impl GeneticAlgorithm {
	pub fn new(population: Population) -> GeneticAlgorithm {
		GeneticAlgorithm {
			population: population
		}
	}
}
