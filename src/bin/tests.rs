extern crate cmp6102;
extern crate rand;
extern crate clap;

use cmp6102::population::Population;
use cmp6102::optimisationmethods::OptimisationMethod;
use cmp6102::optimisationmethods::hill_climbing::HillClimbing;
use cmp6102::optimisationmethods::genetic_algorithm::GeneticAlgorithm;
use cmp6102::optimisationmethods::simulated_annealing::SimulatedAnnealing;
use clap::{Arg, App};

fn main () {

	let matches = App::new("Optimisation Method Creature Generation Test Utility")
					.version("1.0.0")
					.author("Thomas Gardiner <Thomas.Gardiner3@mail.bcu.ac.uk>")
					.about("Repeatedly performs generations on different optimisation methods of a specified sample size.")
					.arg(Arg::with_name("population")
						 .short("p")
						 .long("population")
						 .value_name("size")
						 .help("Sets the number of creatures in each population")
						 .takes_value(true))
					.arg(Arg::with_name("generations")
						 .short("g")
						 .long("generations")
						 .value_name("count")
						 .help("The number of generations to go through in each sample")
						 .takes_value(true))
					.arg(Arg::with_name("samples")
						 .short("s")
						 .long("samples")
						 .value_name("size")
						 .help("Sets the number of samples to collect")
						 .takes_value(true))
					.arg(Arg::with_name("genetic_algorithm")
						 .short("G")
						 .long("genetic_algorithm_skip")
						 .help("Excludes the genetic algorithm from the test")
						 .takes_value(false))
					.arg(Arg::with_name("simulated_annealing")
						 .short("A")
						 .long("simulated_annealing_skip")
						 .help("Excludes simulated annealing from the test")
						 .takes_value(false))
					.arg(Arg::with_name("hill_climbing")
						 .short("H")
						 .long("hill_climbing_skip")
						 .help("Excludes hill climbing from the test")
						 .takes_value(false))
					.get_matches();

	let mut rng = rand::thread_rng();

	let gen_count = matches.value_of("generations").unwrap_or("200").parse::<usize>().unwrap();
	let pop_size = matches.value_of("population").unwrap_or("1000").parse::<usize>().unwrap();
	let sample_size = matches.value_of("samples").unwrap_or("100").parse::<usize>().unwrap();

	let mut optmethods = Vec::new();

	if matches.is_present("genetic_algorithm") {
		println!("Skipping Genetic Algorithm");
	} else {
		optmethods.push("GA");
	}

	if matches.is_present("hill_climbing") {
		println!("Skipping Hill Climbing");
	} else {
		optmethods.push("HC");
	}

	if matches.is_present("simulated_annealing") {
		println!("Skipping Simulated Annealing");
	} else {
		optmethods.push("SA");
	}

	println!("");

	let mut sample_fitness: Vec<Vec<f32>> =
		(0 .. optmethods.len()).map(|_| Vec::with_capacity(sample_size)).collect();

	let mut sample_time: Vec<Vec<f32>> =
		(0 .. optmethods.len()).map(|_| Vec::with_capacity(sample_size)).collect();

	for sample_index in 0 .. sample_size {
		let population = Population::new(pop_size, &mut rng);

		let mut opt: Vec<Box<OptimisationMethod>> =
			Vec::with_capacity(optmethods.len());

		for mtd in &optmethods {
			match *mtd {
				"GA" => opt.push(GeneticAlgorithm::new(population.clone(), false)),
				"HC" => opt.push(HillClimbing::new(population.clone(), false)),
				"SA" => opt.push(SimulatedAnnealing::new(population.clone(), false)),
				_ => {}
			}
		}

		let mut total_time = 0.0;
		for mtd in 0 .. opt.len() {
			for _ in 0 .. gen_count {
				if opt[mtd].generation_single().is_err() {
					break;
				}
			}

			let data = opt[mtd].get_data();
			let gen_time = data.average_gen_time();
			println!("{}   Fittest: {:4.2}   Time: {} ms",
			         data.title,
					 data.generations_get_fittest(),
			         gen_time
			);
			sample_fitness[mtd].push(data.generations_get_fittest());
			sample_time[mtd].push(gen_time);
			total_time += gen_time;
		}
		println!(
			"\nEst. Time Remaining: {:.2} secs\n", ((total_time as usize * gen_count) * (sample_size - (sample_index + 1))) as f64 / 1000.0
		);
	}
	println!("");
	for mtd in 0 .. optmethods.len() {

		let mut average_fitness: f32 = 0.0;
		let mut average_time: f32 = 0.0;

		for fitness in &sample_fitness[mtd] {
			average_fitness += *fitness;
		}
		for time in &sample_time[mtd] {
			average_time += *time;
		}

		average_fitness /= sample_size as f32;
		average_time /= sample_size as f32;
		println!(
			"{}:\n    Average Time:\t{} ms\n    Average Fitness:\t{}",
			optmethods[mtd], average_time, average_fitness
		);
	}
}
