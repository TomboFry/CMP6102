use piston_window::*;
use piston::input::{Button, Motion};
use piston::input::mouse::MouseButton;
use conrod::text::font;
use gui::GUIState;
use modal::Modal;
use rand::{Rng, StdRng};
use std::io::prelude::*;
use std::fs::File;

use cmp6102::population::Population;
use cmp6102::optimisationmethods::OptimisationMethod;
use cmp6102::optimisationmethods::hill_climbing::HillClimbing;
use cmp6102::optimisationmethods::genetic_algorithm::GeneticAlgorithm;
use cmp6102::optimisationmethods::simulated_annealing::SimulatedAnnealing;

pub struct UIData {
	// Mouse position
	pub mouse_x: f64, pub mouse_y: f64,

	// Is the mouse currently being pressed?
	pub mouse_left_down: bool, pub mouse_right_down: bool,

	// Window dimensions and speed to run at
	pub width: u32, pub height: u32, pub fps: u32,

	// Whether the window runs in complete fullscreen mode or not
	pub fullscreen: bool, pub changes: bool,

	// Window title (and main menu title)
	pub title: &'static str,

	// Which page should we draw?
	pub gui_state: GUIState,

	// Rng object to create random numbers
	pub rng: StdRng,

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
		// First attempt to open the settings file to determine whether we
		// should be in fullscreen mode or not.
		let file_open = File::open("settings.txt");
		// Set default to false in case file does not exist.
		let mut fullscreen = false;
		match file_open {
			Ok(mut file) => {
				let mut contents = String::new();
				file.read_to_string(&mut contents)
				    .expect("Although the file exists, could not read contents of settings.txt");
				fullscreen = if contents == "true" { true } else { false };
			},
			_ => {},
		}

		let (width, height) = if fullscreen { fs_wh } else { win_wh };

		// Create a new random thread
		match StdRng::new() {
			// Very unlikely to fail, but just in case it does, this will close
			// the program before it even really begins
			Err(err) => {
				panic!("{}", err);
			},
			// Create the default program settings
			Ok(val) => UIData {
				mouse_x: 0.0, mouse_y: 0.0,
				mouse_left_down: false, mouse_right_down: false,
				width: width, height: height, fps: frames,
				fullscreen: fullscreen, changes: false,
				title: title,
				gui_state: GUIState::Menu,
				rng: val,
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
				gen_do: 1,
				optmethods: Vec::with_capacity(3),
				modal_visible: false,
				modal_struct: None
			}
		}
	}

	// Create a new popup window with a title, message, and button labels
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

	// Destroy the currently displayed popup window
	pub fn modal_close(&mut self) {
		self.modal_visible = false;
		self.modal_struct = None;
	}

	// Runs every frame, checks for button presses and window resize events
	pub fn event(&mut self, event: &Input) {
		match event {
			// Mouse and Keyboard Down
			&Input::Press(button) => {
				if button == Button::Mouse(MouseButton::Left) {
					self.mouse_left_down = true;
				} else if button == Button::Mouse(MouseButton::Right) {
					self.mouse_right_down = true;
				}
			},

			// Mouse and Keyboard Up
			&Input::Release(button) => {
				if button == Button::Mouse(MouseButton::Left) {
					self.mouse_left_down = false;
				} else if button == Button::Mouse(MouseButton::Right) {
					self.mouse_right_down = false;
				}
			},

			// Mouse and Scroll Change
			&Input::Move(x) => {
				match x {
					Motion::MouseCursor(mx, my) => {
						self.mouse_x = mx;
						self.mouse_y = my;
					},
					_ => {}
				}
			},

			// Window Resize
			&Input::Resize(x, y) => {
				self.width = x;
				self.height = y;
			},

			_ => {}
		};

		// If generations are required to be tested and evolved, do it here.
		if !self.modal_visible && self.process_generations > 0 {
			self.do_generation();
			self.process_generations -= 1;
		}
	}

	// Save the fullscreen setting to `settings.txt`, also restarts/displays a
	// message depending on the OS being used.
	pub fn settings_save(&mut self) {
		if self.changes {
			self.changes = false;

			let mut file = File::create("settings.txt").unwrap();
			file.write_all(
				if self.fullscreen { b"true" } else { b"false" }
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

	// Initialise optimisation method(s) with the same population and go to
	// the generations screen
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
			self.optmethods.push(GeneticAlgorithm::new(population.clone()));
		}
		if self.use_hill_climbing {
			self.optmethods.push(HillClimbing::new(population.clone()));
		}
		if self.use_simulated_annealing {
			self.optmethods.push(SimulatedAnnealing::new(population));
		}

		self.gui_state = GUIState::Generations;
		self.set_creature_random();
		self.draw_simulation = false;
	}

	// Process multiple generations at once
	// (Does not actually run the generations, this happens in `event()`)
	pub fn do_generations(&mut self, num: usize) {
		self.process_generations = num;
		self.process_generations_total = num;
	}

	// Run a single generation, displaying a popup window if Simulated
	// Annealing has reached its lowest temperature.
	pub fn do_generation(&mut self) {
		for method in 0 .. self.optmethods.len() {
			match self.optmethods[method].generation_single(&mut self.rng) {
				Err(err) => self.modal_new(err.0, err.1, None, None),
				Ok(_) => {}
			}
		}

		self.spectate_generation += 1;
		self.total_generations += 1;
		self.simulation_frame = 0;
	}

	// Change a specified OM's currently displayed creature to a specific
	// index and generation  
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

	pub fn set_creature_random(&mut self) {
		let index = self.rng.gen_range(0, self.generation_size as usize);
		let gen = self.spectate_generation;
		let mtd = self.spectate_method;
		self.set_creature(mtd, index, gen);
	}

	// Reset the currently simulated creature to its default state.
	pub fn reset_simulation(&mut self) {
		for method in &mut self.optmethods {
			let mut data = method.get_data_mut();
			data.generations[self.spectate_generation]
			    .creatures[self.spectate_creature]
			    .reset_position();
		}
		self.simulation_frame = 0;
	}

	// Destroy all the currently used optimisation methods and settings used
	// by the generations screen.
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
	pub bold_italic: font::Id
}
