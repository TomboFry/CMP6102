use piston_window::*;
use piston::input::{Button, Motion};
use piston::input::mouse::MouseButton;
use conrod::text::font;
use gui::GUIState;
use modal::Modal;
use population::Population;
use rand::{Rng, StdRng};
use optimisationmethods::{self, OptimisationMethod};
// use creature::Creature;

pub struct UIData {
	// Mouse position
	pub mouse_x: f64, pub mouse_y: f64,

	// Is the mouse currently being pressed?
	pub mouse_left_down: bool, pub mouse_right_down: bool,

	// Window dimensions and speed to run at
	pub width: u32, pub height: u32, pub fps: u64,

	// Window title (and main menu title)
	pub title: &'static str,

	// Which page should we draw?
	pub gui_state: GUIState,

	// Rng object to create random numbers
	pub rng: StdRng,

	// Generation Test Options
	pub generation_size: u32,
	pub use_genetic_algorithm: bool,
	pub use_simulated_annealing: bool,
	pub use_hill_climbing: bool,
	pub generation: usize,
	pub creature: usize,
	pub optmethods: Vec<Box<OptimisationMethod>>,

	// Modal information
	pub modal_visible: bool,
	pub modal_struct: Option<Modal>
}

impl UIData {
	pub fn new(title: &'static str, win_w: u32, win_h: u32, frames: u64) -> UIData {
		match StdRng::new() {
			// Very unlikely to fail but just in case it does, this will close the program before it even really begins
			Err (err) => {
				panic!("{}", err);
			},
			Ok(val) => UIData {
				mouse_x: 0.0, mouse_y: 0.0,
				mouse_left_down: false, mouse_right_down: false,
				width: win_w, height: win_h, fps: frames,
				title: title,
				gui_state: GUIState::Menu,
				rng: val,
				generation_size: 1000,
				use_genetic_algorithm: true,
				use_simulated_annealing: true,
				use_hill_climbing: true,
				generation: 0,
				creature: 0,
				optmethods: Vec::with_capacity(3),
				modal_visible: false,
				modal_struct: None
			}
		}
	}

	pub fn modal_new(&mut self, title: String, message: String, btn_a_label: Option<String>, btn_b_label: Option<String>) {
		let mut button_a_label = "Okay".to_string();
		let mut button_b_label = "Close".to_string();

		if let Some(lbl_a) = btn_a_label { button_a_label = lbl_a; }
		if let Some(lbl_b) = btn_b_label { button_b_label = lbl_b; }

		self.modal_struct = Some(Modal {
			title: title,
			message: message,
			button_a_label: button_a_label,
			button_b_label: button_b_label
		});
		self.modal_visible = true;
	}

	pub fn modal_close(&mut self) {
		self.modal_visible = false;
		self.modal_struct = None;
	}

	pub fn event(&mut self, event: &Input) {
		match event {
			// Mouse and Keyboard Down
			&Input::Press(button)  => {
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
	}

	pub fn init_tests(&mut self) {
		self.gui_state = GUIState::DrawCreature;

		let pop = Population::new(self.generation_size, &mut self.rng);
		let ga = optimisationmethods::genetic_algorithm::GeneticAlgorithm::new(pop.clone());
		self.optmethods.push(ga);
	}

	pub fn generation_single(&mut self) {
		for method in &mut self.optmethods {
			method.generation_single(&mut self.rng);
		}
		self.generation += 1;
	}

	pub fn set_creature(&mut self) {
		self.creature = self.rng.gen_range(0, self.generation_size as usize);

		// if let Some(ref pop) = self.population {
		// 	self.chosen_creature = Some(pop.creatures[idx as usize].clone());
		// }
	}

	pub fn reset_optmethods(&mut self) {
		self.optmethods.clear();
		self.generation = 0;
		self.creature = 0;
	}
}

pub struct Fonts {
	pub regular: font::Id,
	pub bold: font::Id,
	pub italic: font::Id,
	pub bold_italic: font::Id
}
