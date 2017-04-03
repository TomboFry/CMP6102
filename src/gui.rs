// use conrod;
use std::process;
use conrod::color::Color;
use conrod::{widget, UiCell, Colorable, Positionable,
	         Widget, Sizeable, Labelable, Borderable};
use app::{UIData, Fonts};

widget_ids! {
	pub struct Ids {
		menu_canvas,
		menu_title,
		menu_btn_start,
		menu_btn_continue,
		menu_btn_options,
		menu_btn_exit,

		options_canvas,
		options_title,
		options_btn_back,
		options_btn_modal,

		new_canvas,
		new_title,
		new_toggle_ga,
		new_toggle_sa,
		new_toggle_hc,
		new_slider_gensize,
		new_btn_start,
		new_btn_back,

		dc_btn,
		dc_creature,
		dc_reset,
		dc_back,

		other_title,
		other_canvas,

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
	DrawCreature,
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
		GUIState::Menu     => menu_main(ui, ids, app, fonts),
		GUIState::NewTest  => menu_new_test(ui, ids, app, fonts),
		GUIState::Options  => menu_options(ui, ids, app, fonts),
		GUIState::Spectate => menu_spectate(ui, ids, app, fonts),
		_ => {}
	}

	if app.modal_visible {
		draw_modal(ui, ids, app, fonts)
	}
}

fn menu_main (ui: &mut UiCell, ids: &Ids, app: &mut UIData, fonts: &Fonts) {
	let canvas_width = app.width as f64 * 0.45;
	let canvas_height = app.height as f64 - (MARGIN * 2.0);

	widget::Canvas::new()
		.color(COL_BG)
		.mid_left_with_margin(MARGIN)
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
	let canvas_width = app.width as f64 * 0.45;
	let canvas_height = app.height as f64 - (MARGIN * 2.0);

	// Background Canvas
	widget::Canvas::new()
		.color(COL_BG)
		.mid_left_with_margin(MARGIN)
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
		app.generation_size = ((value / 10.0) as u32) * 10;
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
	let canvas_width = app.width as f64 - (MARGIN * 2.0);
	let canvas_height = app.height as f64 - (MARGIN * 2.0);

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

fn menu_spectate (ui: &mut UiCell, ids: &Ids, app: &mut UIData, _: &Fonts) {
	let btn_width: f64 = 512.0;
	for _press in widget::Button::new()
		.color(COL_BTN)
		.label(&*format!("Do Generation (gen {})", app.spectate_generation))
		.label_color(COL_LBL)
		.top_left()
		.w_h(btn_width, 48.0)
		.border(0.0)
		.set(ids.dc_btn, ui)
	{
		app.generation_single();
	}

	for _press in widget::Button::new()
		.color(COL_BTN)
		.label(&*format!("Different Creature ({}, {})", app.spectate_creature, app.current_fitness))
		.label_color(COL_LBL)
		.top_left()
		.down_from(ids.dc_btn, SPACING / 4.0)
		.w_h(btn_width, 48.0)
		.border(0.0)
		.set(ids.dc_creature, ui)
	{
		app.set_creature();
	}

	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("Reset Simulation")
		.label_color(COL_LBL)
		.top_left()
		.down_from(ids.dc_creature, SPACING / 4.0)
		.w_h(btn_width, 48.0)
		.border(0.0)
		.set(ids.dc_reset, ui)
	{
		app.reset_simulation();
	}

	for _press in widget::Button::new()
		.color(COL_BTN)
		.label("< Back")
		.label_color(COL_LBL)
		.top_left()
		.down_from(ids.dc_reset, SPACING / 4.0)
		.w_h(btn_width, 48.0)
		.border(0.0)
		.set(ids.dc_back, ui)
	{
		app.gui_state = GUIState::NewTest;
		app.reset_optmethods();
		println!("---- Test Ended");
	}
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
