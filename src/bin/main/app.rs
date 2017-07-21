use piston_window::*;
use conrod::text::font;
use gui::GUIState;
use modal::Modal;
use rand::{self, ThreadRng};
use std::io::prelude::*;
use std::fs::File;

use cmp6102::population::Population;
use cmp6102::optimisationmethods::OptimisationMethod;
use cmp6102::optimisationmethods::hill_climbing::HillClimbing;
use cmp6102::optimisationmethods::genetic_algorithm::GeneticAlgorithm;
use cmp6102::optimisationmethods::simulated_annealing::SimulatedAnnealing;

pub struct UIData {

	// Window dimensions and speed to run at
	pub width: u32, pub height: u32, pub fps: u32,

	// Whether the window runs in complete fullscreen mode or not
	pub fullscreen: bool, pub changes: bool,
	pub print: bool,

	// Window title (and main menu title)
	pub title: &'static str,

	// Which page should we draw?
	pub gui_state: GUIState,

	// Rng object to create random numbers
	pub rng: ThreadRng,

	// Generation Test Options
	pub generation_size: usize,
	pub use_genetic_algorithm: bool,
	pub use_simulated_annealing: bool,
	pub use_hill_climbing: bool,
	pub total_generations: usize,

	pub spectate_method: usize,
	pub spectate_generation: usize,
	pub spectate_creature: usize,

	pub draw_simulation: bool,
	pub simulation_frame: u32,
	pub process_generations: usize, pub process_generations_total: usize,
	pub current_fitness: f32,
	pub gen_do: usize,
	pub optmethods: Vec<Box<OptimisationMethod>>,

	// Modal information
	pub modal_visible: bool,
	pub modal_struct: Option<Modal>
}

impl UIData {
	pub fn new(
		title: &'static str,
		win_wh: (u32, u32),
		fs_wh: (u32, u32),
		frames: u32
	) -> Self {

		// Set default to false in case file does not exist.
		let mut fullscreen = false;
		let mut print = false;

		// First attempt to open the settings file to determine whether we
		// should be in fullscreen mode or not.
		let file_open = File::open("fullscreen.txt");
		match file_open {
			Ok(mut file) => {
				let mut contents = String::new();
				file.read_to_string(&mut contents)
				    .expect("Although the file exists, could not read
contents of fullscreen.txt");
				fullscreen = if contents == "true" { true } else { false };
			},
			_ => {},
		}

		// First attempt to open the settings file to determine whether we
		// should be in fullscreen mode or not.
		let file_open = File::open("print.txt");
		match file_open {
			Ok(mut file) => {
				let mut contents = String::new();
				file.read_to_string(&mut contents)
				    .expect("Although the file exists, could not read
contents of print.txt");
				print = if contents == "true" { true } else { false };
			},
			_ => {},
		}

		let (width, height) = if fullscreen { fs_wh } else { win_wh };

		// Return a new struct with all the default data in.
		UIData {
			width: width, height: height, fps: frames,
			fullscreen: fullscreen, changes: false,
			print: print,
			title: title,
			gui_state: GUIState::Menu,
			rng: rand::thread_rng(), // Create a new random thread
			generation_size: 100,
			use_genetic_algorithm: true,
			use_simulated_annealing: false,
			use_hill_climbing: false,
			total_generations: 0,
			spectate_method: 0,
			spectate_generation: 0,
			spectate_creature: 0,
			draw_simulation: false,
			simulation_frame: 0,
			process_generations: 0,
			process_generations_total: 0,
			current_fitness: 0.0,
			gen_do: 10,
			optmethods: Vec::with_capacity(3),
			modal_visible: false,
			modal_struct: None
		}
	}

	/// Create a new popup window with a title, message, and button labels
	pub fn modal_new(
		&mut self,
		title: String,
		message: String,
		btn_a_label: Option<String>,
		btn_b_label: Option<String>
	) {
		self.modal_visible = true;
		self.modal_struct =
			Some(Modal::new(title, message, btn_a_label, btn_b_label));
	}

	/// Destroy the currently displayed popup window
	pub fn modal_close(&mut self) {
		self.modal_visible = false;
		self.modal_struct = None;
	}

	/// Runs every frame, checks for button presses and window resize events
	pub fn event(&mut self, event: &Input) {

		// If generations are required to be tested and evolved, do it here.
		if !self.modal_visible && self.process_generations > 0 {
			self.do_generation();
			return;
		}

		match event {
			// Window Resize
			&Input::Resize(x, y) => {
				self.width = x;
				self.height = y;
			},

			_ => {}
		};
	}

	/// Save the fullscreen setting to `settings.txt`, also restarts/displays a
	/// message depending on the OS being used.
	pub fn settings_save(&mut self) {
		if self.changes {
			self.changes = false;

			let mut file =
				File::create("fullscreen.txt")
				.expect("Could not open fullscreen.txt for writing");

			file.write_all(
				if self.fullscreen { b"true" } else { b"false" }
			).unwrap();

			let mut file_print =
				File::create("print.txt")
				.expect("Could not open print.txt for writing");
			file_print.write_all(
				if self.print { b"true" } else { b"false" }
			).unwrap();

			self.modal_new(
				"Restart Required".to_string(),
				"In order to change to fullscreen, a restart of
this application is required.".to_string(),
				None,
				None
			);

			#[cfg(unix)] use std::process::Command;
			#[cfg(unix)] use std::os::unix::process::CommandExt;
			#[cfg(unix)] Command::new("/proc/self/exe").exec();
		} else {
			self.gui_state = GUIState::Menu;
		}
	}

	pub fn export_data(&mut self) {
		let mut buffer =
			File::create("export.csv")
				.expect("Could not open export.csv for writing");
		for method in 0 .. self.optmethods.len() {
			let gen_size = self.generation_size;
			let data = self.optmethods[method].get_data();
			let mut generation = 0;
			write!(buffer, "{},Lowest,Q1,Avg.,Q3,Highest\n", data.title)
				.expect("Could not write data");
			for gen in &data.generations {
				let min = gen_size - 1;
				let q1 = (gen_size as f64 * 0.75).round() as usize;
				let q3 = (gen_size as f64 * 0.25).round() as usize;
				let max = 0;
				write!(
					buffer,
					"{}, {}, {}, {}, {}, {}\n",
					generation,
					gen.creatures[min].fitness,
					gen.creatures[q1].fitness,
					gen.fitness_average(),
					gen.creatures[q3].fitness,
					gen.creatures[max].fitness
				).expect("Could not write data");
				generation += 1;
			}
		}
	}

	pub fn export_data_full(&mut self) {
		let mut buffer = File::create("export_full.csv")
				.expect("Could not open export_full.csv for writing");
		for method in 0 .. self.optmethods.len() {
			let data = self.optmethods[method].get_data();
			write!(buffer, "{}\n", data.title).unwrap();
			for gen in &data.generations {
				for creature in &gen.creatures {
					write!(buffer, "{},", creature.fitness as isize)
					.expect("Could not write data");
				}
				write!(buffer, "\n").expect("Could not write data");
			}
			write!(buffer, "\n").expect("Could not write data");
		}
	}

	/// Initialise optimisation method(s) with the same population and go to
	/// the generations screen
	pub fn init_tests(&mut self) {
		if !self.use_genetic_algorithm &&
		   !self.use_hill_climbing &&
		   !self.use_simulated_annealing
		{
			return self.modal_new(
				"Error".to_string(),
				"Please select at least one optimisation method".to_string(),
				None,
				None
			);
		}

		let population = Population::new(self.generation_size, &mut self.rng);

		if self.use_genetic_algorithm {
			self.optmethods.push(
				GeneticAlgorithm::new(population.clone(), self.print)
			);
		}

		if self.use_hill_climbing {
			self.optmethods.push(
				HillClimbing::new(population.clone(), self.print)
			);
		}

		if self.use_simulated_annealing {
			self.optmethods.push(
				SimulatedAnnealing::new(population, self.print)
			);
		}

		self.gui_state = GUIState::Generations;
		self.set_creature(0, 0, 0);
		self.draw_simulation = false;
	}

	/// Process multiple generations at once
	/// (Does not actually run the generations, this happens in `event()`)
	pub fn do_generations(&mut self, num: usize) {
		self.process_generations = num;
		self.process_generations_total = num;
	}

	/// Run a single generation, displaying a popup window if Simulated
	/// Annealing has reached its lowest temperature.
	pub fn do_generation(&mut self) {
		self.process_generations -= 1;
		for method in 0 .. self.optmethods.len() {
			match self.optmethods[method].generation_single() {
				Err(err) => {
					self.process_generations = 0;
					self.modal_new(err.0, err.1, None, None);
				},
				Ok(_) => {}
			}
		}

		self.spectate_generation += 1;
		self.total_generations += 1;
		self.simulation_frame = 0;
	}

	/// Change a specified OM's currently displayed creature to a specific
	/// index and generation
	pub fn set_creature(
		&mut self,
		method: usize,
		index: usize,
		generation: usize
	) {
		self.reset_simulation();
		let data = self.optmethods[method].get_data();
		self.spectate_creature = index;
		self.current_fitness =
			data.generations[generation]
			    .creatures[self.spectate_creature]
			    .fitness;
	}

	/// Reset the currently simulated creature to its default state.
	pub fn reset_simulation(&mut self) {
		for method in &mut self.optmethods {
			let mut data = method.get_data_mut();
			data.generations[self.spectate_generation]
			    .creatures[self.spectate_creature]
			    .reset_position();
		}
		self.simulation_frame = 0;
	}

	/// Destroy all the currently used optimisation methods and settings used
	/// by the generations screen.
	pub fn reset_optmethods(&mut self) {
		self.optmethods.clear();
		self.total_generations = 0;
		self.spectate_generation = 0;
		self.spectate_creature = 0;
		self.draw_simulation = false;
		self.simulation_frame = 0;
	}
}

pub struct Fonts {
	pub regular: font::Id,
	pub bold: font::Id,
	pub italic: font::Id,
	pub bold_italic: font::Id,
	pub fontawesome: font::Id
}
