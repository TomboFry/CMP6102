// use conrod;
use conrod::color::Color;
use conrod::{widget, UiCell, Colorable, Positionable,
	         Widget, Sizeable, Labelable, Borderable};
use app::{UIData, Fonts};
use cmp6102::physics::{self, lerp};
use open;

// Create all the application's GUI widgets here:
widget_ids! {
	pub struct Ids {

		// Main Menu Widgets
		menu_canvas,
		menu_title,
		menu_btn_start,
		menu_btn_options,
		menu_btn_exit,

		// Options Menu Widgets
		options_canvas,
		options_title,
		options_btn_back,
		options_toggle_fullscreen,
		options_toggle_print,

		// New Test Menu Widgets
		new_canvas,
		new_title,
		new_toggle_ga,
		new_toggle_sa,
		new_toggle_hc,
		new_slider_gensize,
		new_btn_start,
		new_btn_back,

		// Generations Menu Widgets
		gen_canvas,
		gen_title,
		gen_btn_back,
		gen_btn_export,
		gen_btn_export_full,

		gen_rect_ga,
		gen_rect_sa,
		gen_rect_hc,

		gen_grid_ga,
		gen_grid_sa,
		gen_grid_hc,

		gen_graph_ga_max,
		gen_graph_sa_max,
		gen_graph_hc_max,

		gen_graph_ga_avg,
		gen_graph_sa_avg,
		gen_graph_hc_avg,

		gen_graph_ga_min,
		gen_graph_sa_min,
		gen_graph_hc_min,

		gen_line_ga,
		gen_line_sa,
		gen_line_hc,

		gen_circle_ga,
		gen_circle_sa,
		gen_circle_hc,

		gen_slider_ga,
		gen_slider_sa,
		gen_slider_hc,

		gen_btn_ga,
		gen_btn_sa,
		gen_btn_hc,

		gen_fittest_ga,
		gen_fittest_sa,
		gen_fittest_hc,

		gen_txt_ga,
		gen_txt_sa,
		gen_txt_hc,

		gen_btn_gen_single,
		gen_slider_gen_do,
		gen_btn_gen_do,
		gen_slider_gen,

		// Spectate Single Creature Widgets
		dc_text,
		dc_reset,
		dc_back,
		dc_physics,

		// Modal Popup Dialogue Widgets
		modal_canvas_overlay,
		modal_canvas_bg,
		modal_title,
		modal_message,
		modal_button_a,
		modal_button_b,

		// Progress Box Widgets
		progress_overlay,
		progress_canvas,
		progress_text,
		progress_slider
	}
}

// Various screens the application can display
pub enum GUIState {
	Menu,
	NewTest,
	Options,
	Generations,
	Spectate
}

pub const MARGIN: f64 = 48.0;
pub const SPACING: f64 = 32.0;

// Macro for creating a conrod colour, so colours can easily be generated
// without requiring the alpha channel and a decimal point.
// Useful for copy/pasting rgb() colour codes in the same format as CSS.
macro_rules! rgb {
	($r: expr, $g: expr, $b: expr) => {
		Color::Rgba(
			$r as f32 / 255.0,
			$g as f32 / 255.0,
			$b as f32 / 255.0,
			1.0
		);
	}
}

// Colour constants used for GUI elements.
const COL_BG:       Color = rgb!(100, 181, 246);
const COL_BTN:      Color = rgb!( 25, 118, 210);
const COL_LBL:      Color = rgb!(245, 245, 245);
const COL_TXT:      Color = rgb!( 33,  33,  33);
const COL_BTN_GO:   Color = rgb!(104, 159,  56);
const COL_BTN_STOP: Color = rgb!(244,  67,  54);

// Gets run every frame, displays the GUI elements for the currently displayed
// screen/menu and any popup boxes.
pub fn gui (ui: &mut UiCell, ids: &Ids, app: &mut UIData, fonts: &Fonts) {
	match app.gui_state {
		GUIState::Menu        => menu_main(ui, ids, app, fonts),
		GUIState::NewTest     => menu_new_test(ui, ids, app, fonts),
		GUIState::Options     => menu_options(ui, ids, app, fonts),
		GUIState::Generations => menu_generations(ui, ids, app, fonts),
		GUIState::Spectate    => menu_spectate(ui, ids, app, fonts)
	}

	if app.modal_visible {
		draw_modal(ui, ids, app, fonts);
	}

	if !app.modal_visible && app.process_generations > 0 &&
	   app.process_generations_total > 20
	{
		let progress =
			(app.process_generations_total as f32 -
			 app.process_generations as f32) /
			app.process_generations_total as f32;

		draw_progress(ui, ids, app, fonts, progress);
	}
}

fn menu_main (ui: &mut UiCell, ids: &Ids, app: &mut UIData, fonts: &Fonts) {
	use std::process;

	let canvas_width = app.width as f64;
	let canvas_height = app.height as f64;

	widget::Canvas::new()
		.color(COL_BG)
		.w_h(canvas_width, canvas_height)
		.pad(MARGIN)
		.border(0.0)
		.set(ids.menu_canvas, ui);

	widget::Text::new(app.title)
		.color(COL_TXT)
		.font_size(32)
		.font_id(fonts.bold)
		.w(canvas_width - (MARGIN * 2.0))
		.line_spacing(8.0)
		.wrap_by_word()
		.top_left_of(ids.menu_canvas)
		.set(ids.menu_title, ui);

	// New Test Button
	for _press in widget::Button::new()
		.color(COL_BTN)
		.label_color(COL_LBL)
		.label("New Test")
		.label_font_size(20)
		.mid_left()
		.down_from(ids.menu_title, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.menu_btn_start, ui)
	{
		app.gui_state = GUIState::NewTest;
	}

	// Options Button
	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("Options")
		.label_color(COL_LBL)
		.label_font_size(20)
		.mid_left()
		.down_from(ids.menu_btn_start, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.menu_btn_options, ui)
	{
		app.gui_state = GUIState::Options;
	}

	// Exit Application Button
	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("Exit")
		.label_color(COL_LBL)
		.label_font_size(20)
		.mid_left()
		.down_from(ids.menu_btn_options, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.menu_btn_exit, ui)
	{
		process::exit(0);
	}
}

fn menu_new_test (
	ui: &mut UiCell,
	ids: &Ids,
	app: &mut UIData,
	fonts: &Fonts
) {
	let canvas_width = app.width as f64;
	let canvas_height = app.height as f64;

	// Background Canvas
	widget::Canvas::new()
		.color(COL_BG)
		.w_h(canvas_width, canvas_height)
		.pad(MARGIN)
		.border(0.0)
		.set(ids.new_canvas, ui);

	// Canvas Title
	widget::Text::new("New Test")
		.color(COL_TXT)
		.font_size(32)
		.font_id(fonts.bold)
		.w(canvas_width - (MARGIN * 2.0))
		.line_spacing(8.0)
		.wrap_by_word()
		.top_left_of(ids.new_canvas)
		.set(ids.new_title, ui);

	// Get and set the values for the toggle buttons
	let use_ga = app.use_genetic_algorithm;
	let use_ga_title =
		if use_ga {
			"Use Genetic Algorithm: On"
		} else {
			"Use Genetic Algorithm: Off"
		};

	let use_sa = app.use_simulated_annealing;
	let use_sa_title =
		if use_sa {
			"Use Simulated Annealing: On"
		} else {
			"Use Simulated Annealing: Off"
		};

	let use_hc = app.use_hill_climbing;
	let use_hc_title =
		if use_hc {
			"Use Hill Climbing: On"
		} else {
			"Use Hill Climbing: Off"
		};

	// First toggle: Genetic Algorithms
	for use_ga in widget::Toggle::new(use_ga)
		.label(use_ga_title)
		.label_color(COL_LBL)
		.label_font_size(20)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.new_title, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.new_toggle_ga, ui)
	{
		app.use_genetic_algorithm = use_ga;
	}

	// Second toggle: Simulated Annealing
	for use_sa in widget::Toggle::new(use_sa)
		.label(use_sa_title)
		.label_color(COL_LBL)
		.label_font_size(20)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.new_toggle_ga, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.new_toggle_sa, ui)
	{
		app.use_simulated_annealing = use_sa;
	}

	// Third toggle: Hill Climbing
	for use_hc in widget::Toggle::new(use_hc)
		.label(use_hc_title)
		.label_color(COL_LBL)
		.label_font_size(20)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.new_toggle_sa, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.new_toggle_hc, ui)
	{
		app.use_hill_climbing = use_hc;
	}

	// Set the size of each generation (100 - 2000)
	let gensize = app.generation_size as f64;
	for value in widget::Slider::new(gensize, 100.0, 2000.0)
		.label(&*format!("Generation Size: {} creatures", gensize))
		.label_color(COL_LBL)
		.label_font_size(20)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.new_toggle_hc, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.new_slider_gensize, ui)
	{
		app.generation_size = ((value / 10.0) as usize) * 10;
	}

	// Start button - Will start the initialisation of the creatures, and the
	// test criteria
	for _press in widget::Button::new()
		.color(COL_BTN_GO)
		.label("Start")
		.label_color(COL_LBL)
		.label_font_size(20)
		.mid_left()
		.down_from(ids.new_slider_gensize, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.new_btn_start, ui)
	{
		app.init_tests();
	}

	// Back button
	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("< Back")
		.label_color(COL_LBL)
		.label_font_size(20)
		.mid_left()
		.down_from(ids.new_btn_start, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.new_btn_back, ui)
	{
		app.gui_state = GUIState::Menu;
	}
}

fn menu_options (ui: &mut UiCell, ids: &Ids, app: &mut UIData, fonts: &Fonts) {
	let canvas_width = app.width as f64;
	let canvas_height = app.height as f64;

	widget::Canvas::new()
		.color(COL_BG)
		.w_h(canvas_width, canvas_height)
		.pad(MARGIN)
		.border(0.0)
		.set(ids.options_canvas, ui);

	widget::Text::new("Options")
		.color(COL_TXT)
		.font_size(32)
		.font_id(fonts.bold)
		.w(canvas_width - (MARGIN * 2.0))
		.line_spacing(8.0)
		.wrap_by_word()
		.top_left_of(ids.options_canvas)
		.set(ids.options_title, ui);

	// Fullscreen Toggle
	let fullscreen_txt =
		if app.fullscreen {
			"Fullscreen: On"
		} else {
			"Fullscreen: Off"
		};

	for value in widget::Toggle::new(app.fullscreen)
		.label(fullscreen_txt)
		.label_color(COL_LBL)
		.label_font_size(20)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.options_title, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.options_toggle_fullscreen, ui)
	{
		app.fullscreen = value;
		app.changes = true;
	}

	// Debug Print Toggle
	let print_txt =
		if app.print {
			"Print to console: On"
		} else {
			"Print to console: Off"
		};

	for value in widget::Toggle::new(app.print)
		.label(print_txt)
		.label_color(COL_LBL)
		.label_font_size(20)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.options_toggle_fullscreen, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.options_toggle_print, ui)
	{
		app.print = value;
		app.changes = true;
	}

	// Back/Save Button
	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("< Back")
		.label_color(COL_LBL)
		.label_font_size(20)
		.mid_left()
		.down_from(ids.options_toggle_print, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.options_btn_back, ui)
	{
		app.settings_save();
	}
}

// The main control panel of the application
// NOTE: Quite a large function due to the immediate mode system used by Conrod
fn menu_generations(
	ui: &mut UiCell,
	ids: &Ids,
	app: &mut UIData,
	fonts: &Fonts
) {
	let canvas_width = app.width as f64;
	let canvas_height = app.height as f64;

	let btn_width = 256.0;

	widget::Canvas::new()
		.color(COL_BG)
		.w_h(canvas_width, canvas_height)
		.pad(MARGIN)
		.border(0.0)
		.set(ids.gen_canvas, ui);

	widget::Text::new("Creature Evolution")
		.color(COL_TXT)
		.font_size(32)
		.font_id(fonts.bold)
		.w(canvas_width - (MARGIN * 2.0))
		.line_spacing(8.0)
		.wrap_by_word()
		.top_left_of(ids.gen_canvas)
		.set(ids.gen_title, ui);

	// Back/Exit Button
	for _ in widget::Button::new()
		.label("<")
		.label_color(COL_LBL)
		.label_font_size(20)
		.w_h(48.0, 32.0)
		.color(COL_BTN_STOP)
		.top_left_with_margins_on(ids.gen_canvas, -MARGIN, -MARGIN)
		.border(0.0)
		.set(ids.gen_btn_back, ui)
	{
		app.gui_state = GUIState::NewTest;
		app.reset_optmethods();
	}

	// Do Single Generation Button
	for _ in widget::Button::new()
		.label("Do Generation")
		.label_color(COL_LBL)
		.label_font_size(20)
		.w_h(btn_width, 48.0)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.gen_title, SPACING)
		.border(0.0)
		.set(ids.gen_btn_gen_single, ui)
	{
		app.do_generations(1);
	}

	// Do X Number of Generations Slider (10 - 500 gens)
	let gen_do = app.gen_do;
	for value in widget::Slider::new(gen_do as f32, 10.0, 500.0)
		.color(COL_BTN)
		.label(&*format!("Do {} gens", gen_do))
		.label_color(COL_LBL)
		.label_font_size(20)
		.border(0.0)
		.down_from(ids.gen_btn_gen_single, SPACING)
		.w_h(btn_width, 48.0)
		.set(ids.gen_slider_gen_do, ui)
	{
		app.gen_do = (value / 10.0) as usize * 10;
	}

	// Do Generations Button
	for _ in widget::Button::new()
		.label("Go")
		.label_color(COL_LBL)
		.label_font_size(20)
		.w_h(btn_width, 48.0)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.gen_slider_gen_do, 2.0)
		.border(0.0)
		.set(ids.gen_btn_gen_do, ui)
	{
		// Display a popup window if we are about to process a large number
		// of creatures
		if gen_do > 20 {
			app.modal_new(
				"This could take a while...".to_string(),
				"You are about to process more than 20 generations
in a single click, so be aware this may take a long
time to process.".to_string(),
				None,
				None
			);
		}

		app.do_generations(gen_do);
	}

	// Export Lowest, Fittest, and quartile fitness values
	for _ in widget::Button::new()
		.label("Export Data")
		.label_color(COL_LBL)
		.label_font_size(20)
		.w_h(btn_width, 48.0)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.gen_btn_gen_do, SPACING)
		.border(0.0)
		.set(ids.gen_btn_export, ui)
	{
		app.export_data();
		open::that("export.csv");
	}

	// Export Lowest, Fittest, and quartile fitness values
	for _ in widget::Button::new()
		.label("Export ALL Data")
		.label_color(COL_LBL)
		.label_font_size(20)
		.w_h(btn_width, 48.0)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.gen_btn_export, SPACING)
		.border(0.0)
		.set(ids.gen_btn_export_full, ui)
	{
		app.export_data_full();
		open::that("export_full.csv");
	}

	// Set the currently viewed generation
	let generation = app.spectate_generation as f64;
	for value in widget::Slider::new(
		generation, 0.0,
		app.total_generations as f64
	)
		.label(&*format!("Generation {}", generation))
		.label_color(COL_LBL)
		.label_font_size(20)
		.color(COL_BTN)
		.down_from(ids.gen_btn_gen_do, SPACING)
		.w_h(app.width as f64 - (MARGIN * 2.0) - btn_width - SPACING, 48.0)
		.border(0.0)
		.bottom_right()
		.set(ids.gen_slider_gen, ui)
	{
		app.spectate_generation = value as usize;
	}

	// Vecs of each type of element, so they can easily be iterated over
	// for each OM
	let ids_rect = vec![
		ids.gen_rect_ga,
		ids.gen_rect_sa,
		ids.gen_rect_hc
	];
	let ids_grid = vec![
		ids.gen_grid_ga,
		ids.gen_grid_sa,
		ids.gen_grid_hc
	];
	let ids_slider = vec![
		ids.gen_slider_ga,
		ids.gen_slider_sa,
		ids.gen_slider_hc
	];
	let ids_btn = vec![
		ids.gen_btn_ga,
		ids.gen_btn_sa,
		ids.gen_btn_hc
	];
	let ids_txt = vec![
		ids.gen_txt_ga,
		ids.gen_txt_sa,
		ids.gen_txt_hc
	];
	let ids_graph_max = vec![
		ids.gen_graph_ga_max,
		ids.gen_graph_sa_max,
		ids.gen_graph_hc_max
	];
	let ids_graph_avg = vec![
		ids.gen_graph_ga_avg,
		ids.gen_graph_sa_avg,
		ids.gen_graph_hc_avg
	];
	let ids_graph_min = vec![
		ids.gen_graph_ga_min,
		ids.gen_graph_sa_min,
		ids.gen_graph_hc_min
	];
	let ids_line = vec![
		ids.gen_line_ga,
		ids.gen_line_sa,
		ids.gen_line_hc
	];
	let ids_circle = vec![
		ids.gen_circle_ga,
		ids.gen_circle_sa,
		ids.gen_circle_hc
	];
	let ids_btn_fittest = vec![
		ids.gen_fittest_ga,
		ids.gen_fittest_sa,
		ids.gen_fittest_hc
	];

	// Calculate the height of each section depending on how many methods
	// we're using and the height of the window.
	let method_height =
		(app.height as f64 - (MARGIN * 2.0) - 112.0) /
		app.optmethods.len() as f64;

	for mtd in 0 .. app.optmethods.len() {
		// Cycle between any creature in a specific generation of the current
		// optimisation method
		let data = app.optmethods[mtd].get_data_mut();
		for value in widget::Slider::new(
			(app.generation_size as f64 - data.spectate_creature as f64),
			1.0, app.generation_size as f64
		)
			.label(&*format!("Crt {} / {}", (app.generation_size -
			                                 data.spectate_creature),
			                                app.generation_size))
			.label_color(COL_LBL)
			.label_font_size(20)
			.color(COL_BTN)
			.top_left_with_margins_on(
				ids.gen_canvas,
				64.0 + (mtd as f64 * method_height),
				452.0)
			.w_h(240.0, 32.0)
			.border(1.0)
			.set(ids_slider[mtd], ui)
		{
			let new_value = app.generation_size - value as usize;
			data.spectate_creature = new_value;
			app.spectate_creature = new_value;
		}

		// Watch a simulation of the currently viewed creature
		for _ in widget::Button::new()
			.label("\u{f06e}")
			.label_color(COL_LBL)
			.label_font_size(16)
			.label_font_id(fonts.fontawesome)
			.w_h(32.0, 32.0)
			.color(COL_BTN)
			.right_from(ids_slider[mtd], 1.0)
			.border(0.0)
			.set(ids_btn[mtd], ui)
		{
			// Switch to the spectator screen
			app.gui_state = GUIState::Spectate;
			app.spectate_method = mtd;
			app.draw_simulation = true;
		}

		// Watch a simulation of the currently viewed creature
		for _ in widget::Button::new()
			.label("\u{f201}")
			.label_color(COL_LBL)
			.label_font_size(16)
			.label_font_id(fonts.fontawesome)
			.w_h(32.0, 32.0)
			.color(COL_BTN)
			.down_from(ids_btn[mtd], 1.0)
			.border(0.0)
			.set(ids_btn_fittest[mtd], ui)
		{
			let fittest_gen = data.generations_get_fittest_gen();
			data.spectate_creature = 0;
			app.spectate_creature = 0;
			app.spectate_generation = fittest_gen;
		}

		// Display some information about the optimisation
		// method's performance.
		widget::Text::new(&*format!("{}: Avg. time: {}ms\nFitness: {}",
				data.title,
				data.average_gen_time(),
				data.generations[app.spectate_generation]
				    .creatures[data.spectate_creature]
				    .fitness as i16)
			)
			.color(COL_TXT)
			.font_size(18)
			.font_id(fonts.bold)
			.w(240.0)
			.wrap_by_word()
			.line_spacing(8.0)
			.down_from(ids_slider[mtd], 8.0)
			.set(ids_txt[mtd], ui);

		let graph_width = app.width as f64 - 848.0;
		let graph_fittest = data.generations_get_fittest();
		let graph_weakest = data.generations_get_weakest();
		let graph_height = method_height - 24.0;

		// Various closures to help calculate a smooth line for the Y
		// position of each graph
		let graph_max = |x| {
			let mut xx = data.creature_get_fittest(x as usize).fitness;
			if (x as usize) < data.gen {
				xx = lerp(
					xx,
					data.creature_get_fittest(x as usize + 1).fitness,
					x % 1.0
				);
			}
			xx // Returns the Y position of a given X co-ordinate
		};

		let graph_avg = |x| {
			let mut xx = data.creature_get_average(x as usize);
			if (x as usize) < data.gen {
				xx = lerp(
					xx,
					data.creature_get_average(x as usize + 1),
					x % 1.0
				);
			}
			xx // Returns the Y position of a given X co-ordinate
		};

		let graph_min = |x| {
			let mut xx = data.creature_get_weakest(x as usize).fitness;
			if (x as usize) < data.gen {
				xx = lerp(
					xx,
					data.creature_get_weakest(x as usize + 1).fitness,
					x % 1.0
				);
			}
			xx // Returns the Y position of a given X co-ordinate
		};

		let down = |y| {
			((y - graph_weakest) / (graph_weakest - graph_fittest)) as f64 *
			graph_height as f64
		};

		let graph_spacing = SPACING + 32.0;

		// Draw a white rectangle to cover the space of the graph for this
		// optimisation method
		widget::Rectangle::fill_with([graph_width, graph_height], COL_LBL)
			.right_from(ids_slider[mtd], graph_spacing)
			.set(ids_rect[mtd], ui);

		// Draw a line of y = 0 where y is the fitness value of a creature
		widget::Line::centred([0.0, 0.0], [graph_width, 0.0])
			.color(COL_TXT)
			.right_from(ids_slider[mtd], graph_spacing)
			.down(down(0.0))
			.set(ids_grid[mtd], ui);

		// Plot a path of the fittest creature of each generation
		widget::PlotPath::new(
			0.0,
			data.gen as f32,
			graph_weakest,
			graph_fittest,
			graph_max
		)
		.w_h(graph_width, graph_height)
		.color(COL_TXT)
		.right_from(ids_slider[mtd], graph_spacing)
		.set(ids_graph_max[mtd], ui);

		// Plot a path of the generation's average creature fitness
		widget::PlotPath::new(
			0.0,
			data.gen as f32,
			graph_weakest,
			graph_fittest,
			graph_avg
		)
		.w_h(graph_width, graph_height)
		.color(COL_TXT)
		.right_from(ids_slider[mtd], graph_spacing)
		.set(ids_graph_avg[mtd], ui);

		// Plot a path of the weakest creature of each generation
		widget::PlotPath::new(
			0.0,
			data.gen as f32,
			graph_weakest,
			graph_fittest,
			graph_min
		)
		.w_h(graph_width, graph_height)
		.color(COL_TXT)
		.right_from(ids_slider[mtd], graph_spacing)
		.set(ids_graph_min[mtd], ui);

		// If we've done more than one generation draw a line to indicate
		// which creature from which generation we are looking at in the
		// preview box
		if data.gen > 0 {
			widget::Line::centred([0.0, 0.0], [0.0, graph_height])
				.color(COL_BTN_STOP)
				.thickness(2.0)
				.right_from(
					ids_slider[mtd],
					((app.spectate_generation as f64 / data.gen as f64) *
					 graph_width) + graph_spacing
				)
				.set(ids_line[mtd], ui);

			widget::Circle::fill_with(3.0, COL_BTN_STOP)
				.right_from(
					ids_slider[mtd],
					((app.spectate_generation as f64 / data.gen as f64) *
					 graph_width) - 2.0 + graph_spacing
				)
				.down(down(
					data.generations[app.spectate_generation]
					    .creatures[data.spectate_creature]
					    .fitness
				) - 2.0)
				.set(ids_circle[mtd], ui);
		}
	}
}

fn menu_spectate (
	ui: &mut UiCell,
	ids: &Ids,
	app: &mut UIData,
	fonts: &Fonts
) {
	// Back Button
	for _ in widget::Button::new()
		.color(COL_BTN_STOP)
		.label("<")
		.label_color(COL_LBL)
		.label_font_size(20)
		.top_left()
		.w_h(48.0, 32.0)
		.border(0.0)
		.set(ids.dc_back, ui)
	{
		let data = app.optmethods[app.spectate_method].get_data_mut();

		data.generations[app.spectate_generation]
		    .creatures[app.spectate_creature]
		    .reset_position();

		app.gui_state = GUIState::Generations;
		app.draw_simulation = false;
		app.simulation_frame = 0;
	}

	// Restart the simulation
	for _ in widget::Button::new()
		.color(COL_BTN)
		.label("Reset Simulation")
		.label_color(COL_LBL)
		.label_font_size(20)
		.right_from(ids.dc_back, 16.0)
		.w_h(192.0, 32.0)
		.border(0.0)
		.set(ids.dc_reset, ui)
	{
		app.reset_simulation();
	}

	// Print/draw the fitness of the creature to bottom center of the screen
	widget::Text::new(
		&*format!("{}",
			app.optmethods[app.spectate_method]
			   .get_data()
			   .generations[app.spectate_generation]
			   .creatures[app.spectate_creature]
			   .fitness() as i32
		)
	)
	.font_size(32)
	.font_id(fonts.bold)
	.center_justify()
	.w_h(512.0, 48.0)
	.mid_bottom_with_margin(14.0)
	.set(ids.dc_text, ui);

	// Draw a(n immutable) slider to show how far in the simulation we are
	widget::Slider::new(
		app.simulation_frame as f32,
		0.0,
		physics::SIM_LENGTH as f32 - 1.0
	)
	.label(
		&*format!(
			"{} / {}s ({}%)",
			app.simulation_frame / app.fps,
			physics::SIM_LENGTH / app.fps,
			(app.simulation_frame as f64 * 100.0 /
			 physics::SIM_LENGTH as f64) as u32
		)
	)
	.label_color(COL_LBL)
	.label_font_size(20)
	.color(COL_BTN)
	.w_h(256.0, 42.0)
	.bottom_left_with_margin(21.0)
	.border(0.0)
	.set(ids.dc_physics, ui);
}

fn draw_modal (ui: &mut UiCell, ids: &Ids, app: &mut UIData, fonts: &Fonts) {
	let mut action: u8 = 0;

	// Only draw the popup window if we've already set one up.
	if let Some(ref mut modal) = app.modal_struct {
		let canvas_width = app.width as f64;
		let canvas_height = app.height as f64;

		let modal_width = (app.width as f64).min(640.0);
		let modal_height = (app.height as f64).min(360.0);

		// Draw a translucent overlay to the entire screen
		widget::Canvas::new()
			.rgba(0.0, 0.0, 0.0, 0.75)
			.w_h(canvas_width, canvas_height)
			.x_y(0.0, 0.0)
			.border(0.0)
			.set(ids.modal_canvas_overlay, ui);

		// Draw a blue window to the centre of the screen
		widget::Canvas::new()
			.color(COL_BG)
			.w_h(modal_width, modal_height)
			.middle_of(ids.modal_canvas_overlay)
			.pad(MARGIN)
			.border(0.0)
			.set(ids.modal_canvas_bg, ui);

		// Modal Title
		widget::Text::new(&modal.title)
			.color(COL_TXT)
			.mid_top_of(ids.modal_canvas_bg)
			.font_id(fonts.bold)
			.font_size(32)
			.w(modal_width - (MARGIN * 2.0))
			.set(ids.modal_title, ui);

		// Modal Message
		widget::Text::new(&modal.message)
			.color(COL_TXT)
			.line_spacing(8.0)
			.down_from(ids.modal_title, SPACING)
			.font_id(fonts.regular)
			.font_size(22)
			.w(modal_width - (MARGIN * 2.0))
			.set(ids.modal_message, ui);

		// Left/"Accept" Button
		for _ in widget::Button::new()
			.color(COL_BTN_GO)
			.label(&modal.button_a_label)
			.label_color(COL_LBL)
			.label_font_size(20)
			.bottom_left_of(ids.modal_canvas_bg)
			.w_h((modal.button_a_label.len() as f64 * 14.0) + 32.0, 48.0)
			.border(0.0)
			.set(ids.modal_button_a, ui)
		{
			action = 1;
		}

		// Right/"Cancel" Button
		for _ in widget::Button::new()
			.color(COL_BTN_STOP)
			.label(&modal.button_b_label)
			.label_color(COL_LBL)
			.label_font_size(20)
			.right_from(ids.modal_button_a, SPACING)
			.w_h((modal.button_b_label.len() as f64 * 14.0) + 32.0, 48.0)
			.border(0.0)
			.set(ids.modal_button_b, ui)
		{
			action = 2;
		}
	}

	// Depending on which one we click, perform a different action.
	// NOTE: I did not end up using either button to perform different
	// tasks, so they have been left to do the same thing.
	if action == 1 {
		app.modal_close();
	} else if action == 2 {
		app.modal_close();
	}
}

// Draw a progress bar to the centre of the screen, when we are
// processing generations
fn draw_progress (
	ui: &mut UiCell,
	ids: &Ids,
	app: &mut UIData,
	_: &Fonts,
	progress: f32
) {
	// Translucent overlay
	widget::Canvas::new()
		.rgba(0.0, 0.0, 0.0, 0.75)
		.w_h(app.width as f64, app.height as f64)
		.x_y(0.0, 0.0)
		.border(0.0)
		.set(ids.progress_overlay, ui);

	// Draw a white box to the centre of the screen
	widget::Canvas::new()
		.w_h(320.0, 180.0)
		.middle()
		.color(COL_LBL)
		.border(0.0)
		.set(ids.progress_canvas, ui);

	// Progress slider
	widget::Slider::new(progress, 0.0, 1.0)
		.w_h(320.0, 16.0)
		.down_from(ids.progress_canvas, 0.0)
		.color(COL_BTN)
		.border(0.0)
		.set(ids.progress_slider, ui);

	// Popup text
	widget::Text::new("Generating...\nPlease wait.")
		.w_h(320.0, 72.0)
		.font_size(32)
		.line_spacing(4.0)
		.middle()
		.center_justify()
		.color(COL_TXT)
		.set(ids.progress_text, ui);
}
