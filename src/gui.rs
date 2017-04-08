// use conrod;
use conrod::color::Color;
use conrod::{widget, UiCell, Colorable, Positionable,
	         Widget, Sizeable, Labelable, Borderable};
use app::{UIData, Fonts};

widget_ids! {
	pub struct Ids {

		// Main Menu Widgets
		menu_canvas,
		menu_title,
		menu_btn_start,
		menu_btn_continue,
		menu_btn_options,
		menu_btn_exit,

		// Options Menu Widgets
		options_canvas,
		options_title,
		options_btn_back,
		options_btn_modal,

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
		gen_graph_ga,
		gen_graph_hc,
		gen_graph_sa,
		gen_slider_ga,
		gen_slider_hc,
		gen_slider_sa,
		gen_btn_ga,
		gen_btn_hc,
		gen_btn_sa,
		gen_txt_ga,
		gen_txt_hc,
		gen_txt_sa,
		gen_btn_gen_single,
		gen_btn_gen_ten,
		gen_btn_spectate,
		gen_slider_gen,

		// Spectate Single Creature Widgets
		dc_btn,
		dc_highest,
		dc_creature,
		dc_text,
		dc_reset,
		dc_back,

		// Modal Popup Dialogue Widgets
		modal_canvas_overlay,
		modal_canvas_bg,
		modal_title,
		modal_message,
		modal_button_a,
		modal_button_b
	}
}

pub enum GUIState {
	Menu,
	NewTest,
	Options,
	Generations,
	Spectate
}

const MARGIN: f64 = 48.0;
const SPACING: f64 = 32.0;

macro_rules! rgb {
	($r: expr, $g: expr, $b: expr) => {
		Color::Rgba($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0);
	}
}

const COL_BG:       Color = rgb!(100, 181, 246);
const COL_BTN:      Color = rgb!( 25, 118, 210);
const COL_LBL:      Color = rgb!(245, 245, 245);
const COL_TXT:      Color = rgb!( 33,  33,  33);
const COL_BTN_GO:   Color = rgb!(104, 159,  56);
const COL_BTN_STOP: Color = rgb!(244,  67,  54);

pub fn gui (ui: &mut UiCell, ids: &Ids, app: &mut UIData, fonts: &Fonts) {
	match app.gui_state {
		GUIState::Menu        => menu_main(ui, ids, app, fonts),
		GUIState::NewTest     => menu_new_test(ui, ids, app, fonts),
		GUIState::Options     => menu_options(ui, ids, app, fonts),
		GUIState::Generations => menu_generations(ui, ids, app, fonts),
		GUIState::Spectate    => menu_spectate(ui, ids, app, fonts),
		_ => {}
	}

	if app.modal_visible {
		draw_modal(ui, ids, app, fonts)
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
		.font_size(30)
		.font_id(fonts.bold)
		.w(canvas_width - (MARGIN * 2.0))
		.line_spacing(8.0)
		.wrap_by_word()
		.top_left_of(ids.menu_canvas)
		.set(ids.menu_title, ui);

	for _press in widget::Button::new()
		.color(COL_BTN)
		.label_color(COL_LBL)
		.label("New Test")
		.mid_left()
		.down_from(ids.menu_title, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.menu_btn_start, ui)
	{
		app.gui_state = GUIState::NewTest;
	}

	for _press in widget::Button::new()
		.color(COL_BTN)
		.label_color(COL_LBL)
		.label("Continue Previous Test")
		.mid_left()
		.down_from(ids.menu_btn_start, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.menu_btn_continue, ui)
	{
		// app.gui_state = GUIState::NewTest;
		app.modal_new("Not Implemented Yet.".to_string(), "It might be a while".to_string(), None, None);
	}

	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("Options")
		.label_color(COL_LBL)
		.mid_left()
		.down_from(ids.menu_btn_continue, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.menu_btn_options, ui)
	{
		app.gui_state = GUIState::Options;
	}

	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("Exit")
		.label_color(COL_LBL)
		.mid_left()
		.down_from(ids.menu_btn_options, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.menu_btn_exit, ui)
	{
		process::exit(0);
	}
}

fn menu_new_test (ui: &mut UiCell, ids: &Ids, app: &mut UIData, fonts: &Fonts) {
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
		.font_size(30)
		.font_id(fonts.bold)
		.w(canvas_width - (MARGIN * 2.0))
		.line_spacing(8.0)
		.wrap_by_word()
		.top_left_of(ids.new_canvas)
		.set(ids.new_title, ui);

	// Get and set the values for the toggle buttons
	let use_ga = app.use_genetic_algorithm;
	let use_ga_title = if use_ga { "Use Genetic Algorithm: On" } else { "Use Genetic Algorithm: Off" };

	let use_sa = app.use_simulated_annealing;
	let use_sa_title = if use_sa { "Use Simulated Annealing: On" } else { "Use Simulated Annealing: Off" };

	let use_hc = app.use_hill_climbing;
	let use_hc_title = if use_hc { "Use Hill Climbing: On" } else { "Use Hill Climbing: Off" };

	// First toggle: Genetic Algorithms
	for use_ga in widget::Toggle::new(use_ga)
		.label(use_ga_title)
		.label_color(COL_LBL)
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
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.new_toggle_sa, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.new_toggle_hc, ui)
	{
		app.use_hill_climbing = use_hc;
	}

	// Set the size of each generation (100-1000)
	let gensize = app.generation_size as f64;
	for value in widget::Slider::new(gensize, 10.0, 1000.0)
		.label(&*format!("Generation Size: {} creatures", gensize))
		.label_color(COL_LBL)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.new_toggle_hc, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.new_slider_gensize, ui)
	{
		app.generation_size = ((value / 10.0) as usize) * 10;
	}

	// Start button - Will start the initialisation of the creatures, and the test criteria
	for _press in widget::Button::new()
		.color(COL_BTN_GO)
		.label("Start")
		.label_color(COL_LBL)
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
		.font_size(30)
		.font_id(fonts.bold)
		.w(canvas_width - (MARGIN * 2.0))
		.line_spacing(8.0)
		.wrap_by_word()
		.top_left_of(ids.options_canvas)
		.set(ids.options_title, ui);

	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("< Back")
		.label_color(COL_LBL)
		.mid_left()
		.down_from(ids.options_title, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.options_btn_back, ui)
	{
		app.gui_state = GUIState::Menu;
	}

	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("Modal")
		.label_color(COL_LBL)
		.mid_left()
		.down_from(ids.options_btn_back, SPACING)
		.w_h(canvas_width - (MARGIN * 2.0), 48.0)
		.border(0.0)
		.set(ids.options_btn_modal, ui)
	{
		app.modal_new("Testing the modal dialogue box".to_string(), "This is an example of a really long string. It should hopefully wrap over multiple lines and demonstrate that it actually does work!".to_string(), Some("Say Whaaaat?!".to_string()), None);
	}
}

fn menu_generations(ui: &mut UiCell, ids: &Ids, app: &mut UIData, fonts: &Fonts) {
	let canvas_width = app.width as f64;
	let canvas_height = app.height as f64;

	widget::Canvas::new()
		.color(COL_BG)
		.w_h(canvas_width, canvas_height)
		.pad(MARGIN)
		.border(0.0)
		.set(ids.gen_canvas, ui);

	widget::Text::new("Creature Evolution")
		.color(COL_TXT)
		.font_size(30)
		.font_id(fonts.bold)
		.w(canvas_width - (MARGIN * 2.0))
		.line_spacing(8.0)
		.wrap_by_word()
		.top_left_of(ids.gen_canvas)
		.set(ids.gen_title, ui);

	for _press in widget::Button::new()
		.label("<")
		.w_h(SPACING, SPACING)
		.color(COL_BTN_STOP)
		.top_left_with_margins_on(ids.gen_canvas, -MARGIN, -MARGIN)
		.border(0.0)
		.label_color(COL_LBL)
		.set(ids.gen_btn_back, ui)
	{
		app.gui_state = GUIState::NewTest;
		app.reset_optmethods();
	}

	for _press in widget::Button::new()
		.label("Do Single Generation")
		.w_h(384.0, 48.0)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.gen_title, SPACING)
		.border(0.0)
		.label_color(COL_LBL)
		.set(ids.gen_btn_gen_single, ui)
	{
		app.generation_single();
	}

	for _press in widget::Button::new()
		.label("Do 10 Generations")
		.w_h(384.0, 48.0)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.gen_btn_gen_single, SPACING)
		.border(0.0)
		.label_color(COL_LBL)
		.set(ids.gen_btn_gen_ten, ui)
	{
		for _ in 0 .. 10 {
			app.generation_single();
		}
	}

	for _press in widget::Button::new()
		.label("View Fittest")
		.w_h(384.0, 48.0)
		.color(COL_BTN)
		.mid_left()
		.down_from(ids.gen_btn_gen_ten, SPACING)
		.border(0.0)
		.label_color(COL_LBL)
		.set(ids.gen_btn_spectate, ui)
	{
		// Stuff
		app.gui_state = GUIState::Spectate;

		let gen = app.spectate_generation;
		app.set_creature(0, 0, gen);

		app.draw_simulation = true;
	}

	// Set the size of each generation (100-1000)
	let generation = app.spectate_generation as f64;
	for value in widget::Slider::new(generation, 0.0, app.total_generations as f64)
		.label(&*format!("Generation {}", generation))
		.label_color(COL_LBL)
		.color(COL_BTN)
		.down_from(ids.gen_btn_gen_ten, SPACING)
		.w_h(app.width as f64 - (MARGIN * 2.0) - 384.0 - SPACING, 48.0)
		.border(0.0)
		.bottom_right()
		.set(ids.gen_slider_gen, ui)
	{
		app.spectate_generation = value as usize;
	}

	let ids_slider = vec![ids.gen_slider_ga, ids.gen_slider_hc, ids.gen_slider_sa];
	let ids_btn = vec![ids.gen_btn_ga, ids.gen_btn_hc, ids.gen_btn_sa];
	let ids_txt = vec![ids.gen_txt_ga, ids.gen_txt_hc, ids.gen_txt_sa];

	for mtd in 0 .. app.optmethods.len() {
		// Set the size of each generation (100-1000)
		let data = app.optmethods[mtd].get_data_mut();
		let y = (app.height as f64 / 2.0) - 110.0 - (mtd as f64 * 172.0);
		let x = (app.width as f64 / 2.0) - 472.0;
		for value in widget::Slider::new((app.generation_size as f64 - data.spectate_creature as f64), 1.0, app.generation_size as f64)
			.label(&*format!("Crt: {}", (app.generation_size - data.spectate_creature)))
			.label_color(COL_LBL)
			.color(COL_BTN)
			.top_left_with_margins_on(ids.gen_canvas, 96.0 + (mtd as f64 * 172.0), 612.0)
			.w_h(140.0, 32.0)
			.border(0.0)
			.set(ids_slider[mtd], ui)
		{
			let new_value = app.generation_size - value as usize;
			data.spectate_creature = new_value;
			app.spectate_creature = new_value;
		}

		for _press in widget::Button::new()
			.label("View")
			.w_h(140.0, 32.0)
			.color(COL_BTN)
			.down_from(ids_slider[mtd], 2.0)
			.border(0.0)
			.label_color(COL_LBL)
			.set(ids_btn[mtd], ui)
		{
			// Stuff
			app.gui_state = GUIState::Spectate;
			app.spectate_method = mtd;
			app.draw_simulation = true;
		}

		widget::Text::new(&*format!("Average time: {}\nFitness: {}", data.average_gen_time() / 10000, data.generations[app.spectate_generation].creatures[data.spectate_creature].fitness as i16))
			.color(COL_TXT)
			.font_size(18)
			.font_id(fonts.regular)
			.w(512.0)
			.wrap_by_word()
			.right_from(ids_slider[mtd], 8.0)
			.set(ids_txt[mtd], ui);
	}
}

fn menu_spectate (ui: &mut UiCell, ids: &Ids, app: &mut UIData, fonts: &Fonts) {
	let btn_width: f64 = 512.0;

	// for _press in widget::Button::new()
	// 	.color(COL_BTN)
	// 	.label(&*format!("Do Generation (gen {})", app.spectate_generation))
	// 	.label_color(COL_LBL)
	// 	.top_left()
	// 	.w_h(btn_width, 48.0)
	// 	.border(0.0)
	// 	.set(ids.dc_btn, ui)
	// {
	// 	app.generation_single();
	// }

	// // if app.spectate_generation > 0 {
	// 	for _press in widget::Button::new()
	// 		.color(COL_BTN)
	// 		.label("Do 100 generations")
	// 		.label_color(COL_LBL)
	// 		.top_left()
	// 		.right_from(ids.dc_btn, SPACING / 4.0)
	// 		.w_h(btn_width, 48.0)
	// 		.border(0.0)
	// 		.set(ids.dc_highest, ui)
	// 	{
	// 		for _ in 0 .. 100 {
	// 			app.generation_single();
	// 		}
	// 	}
	// // }

	// for _press in widget::Button::new()
	// 	.color(COL_BTN)
	// 	.label(&*format!("Different Creature ({}, {})", app.spectate_creature, app.current_fitness as isize))
	// 	.label_color(COL_LBL)
	// 	.top_left()
	// 	.down_from(ids.dc_btn, SPACING / 4.0)
	// 	.w_h(btn_width, 48.0)
	// 	.border(0.0)
	// 	.set(ids.dc_creature, ui)
	// {
	// 	app.set_creature();
	// }

	// // Pick the creature
	// let creature = app.spectate_creature as f64;
	// for value in widget::Slider::new(creature, 0.0, app.generation_size as f64 - 1.0)
	// 	.color(COL_BTN)
	// 	.label(&*format!("Creature {} (fit: {})", creature, app.current_fitness))
	// 	.label_color(COL_LBL)
	// 	.top_left()
	// 	.down_from(ids.dc_btn, SPACING / 4.0)
	// 	.w_h(app.generation_size as f64 - 1.0, 40.0)
	// 	.border(0.0)
	// 	.set(ids.dc_creature, ui)
	// {
	// 	let gen = app.spectate_generation;
	// 	app.set_creature(value as usize, gen);
	// }

	for _press in widget::Button::new()
		.color(COL_BTN_STOP)
		.label("<")
		.label_color(COL_LBL)
		.top_left()
		.w_h(48.0, 48.0)
		.border(0.0)
		.set(ids.dc_back, ui)
	{
		let data = app.optmethods[app.spectate_method].get_data_mut();
		data.generations[app.spectate_generation].creatures[app.spectate_creature].reset_position();
		app.gui_state = GUIState::Generations;
		app.draw_simulation = false;
		app.simulation_frame = 0;
	}

	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("Reset Simulation")
		.label_color(COL_LBL)
		.right_from(ids.dc_back, SPACING)
		.w_h(btn_width, 48.0)
		.border(0.0)
		.set(ids.dc_reset, ui)
	{
		app.reset_simulation();
	}

	widget::Text::new(&*format!("{}\n{}", app.optmethods[app.spectate_method].get_data().generations[app.spectate_generation].creatures[app.spectate_creature].fitness(), app.simulation_frame))
		.font_size(16)
		.font_id(fonts.regular)
		.w_h(512.0, 48.0)
		.down_from(ids.dc_back, SPACING)
		.set(ids.dc_text, ui);
}

fn draw_modal (ui: &mut UiCell, ids: &Ids, app: &mut UIData, fonts: &Fonts) {
	let mut action: u8 = 0;

	if let Some(ref mut modal) = app.modal_struct {
		let canvas_width = app.width as f64;
		let canvas_height = app.height as f64;

		let modal_width = (app.width as f64).min(848.0);
		let modal_height = (app.height as f64).min(480.0);

		widget::Canvas::new()
			.rgba(0.0, 0.0, 0.0, 0.75)
			.w_h(canvas_width, canvas_height)
			.x_y(0.0, 0.0)
			.border(0.0)
			.set(ids.modal_canvas_overlay, ui);

		widget::Canvas::new()
			.color(COL_BG)
			.w_h(modal_width, modal_height)
			.middle_of(ids.modal_canvas_overlay)
			.pad(MARGIN)
			.border(0.0)
			.set(ids.modal_canvas_bg, ui);

		widget::Text::new(&modal.title)
			.color(COL_TXT)
			.mid_top_of(ids.modal_canvas_bg)
			.font_id(fonts.bold)
			.font_size(24)
			.w(modal_width - (MARGIN * 2.0))
			.set(ids.modal_title, ui);

		widget::Text::new(&modal.message)
			.color(COL_TXT)
			.line_spacing(8.0)
			.down_from(ids.modal_title, SPACING)
			.font_id(fonts.regular)
			.font_size(18)
			.w(modal_width - (MARGIN * 2.0))
			.set(ids.modal_message, ui);

		for _press in widget::Button::new()
			.color(COL_BTN_GO)
			.label(&modal.button_a_label)
			.label_color(COL_LBL)
			.bottom_left_of(ids.modal_canvas_bg)
			.w_h((modal.button_a_label.len() as f64 * 14.0) + 32.0, 48.0)
			.border(0.0)
			.set(ids.modal_button_a, ui)
		{
			action = 1;
		}

		for _press in widget::Button::new()
			.color(COL_BTN_STOP)
			.label(&modal.button_b_label)
			.label_color(COL_LBL)
			.right_from(ids.modal_button_a, SPACING)
			.w_h((modal.button_b_label.len() as f64 * 14.0) + 32.0, 48.0)
			.border(0.0)
			.set(ids.modal_button_b, ui)
		{
			action = 2;
		}
	}
	if action == 1 {
		app.modal_close();
	} else if action == 2 {
		app.modal_close();
	}
}
