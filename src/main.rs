#![allow(dead_code)]
#![allow(unused_variables)]

extern crate piston_window;
extern crate piston;
#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate rand;

mod gui;
mod app;
mod modal;
mod creature;
mod population;
mod optimisationmethods;

use piston_window::*;
use piston_window::texture::UpdateTexture;
use gui::GUIState;

fn main () {

	// Initialise the app data
	//                             Window / Menu Title                        W     H    FPS
	let mut app = app::UIData::new("Optimisation Method Creature Generation", 1280, 720, 60);

	// Create the window
	let mut window : PistonWindow =
		WindowSettings::new(app.title, [app.width, app.height])
		.exit_on_esc(true)
		.opengl(OpenGL::V3_2)
		.vsync(true)
		.build()
		.expect("Error creating window");

	// Ensure the program runs at 60fps to not overload the system
	window.set_ups(app.fps);
	window.set_max_fps(app.fps);

	// Create the UI with the same width and height as the window
	let mut ui = conrod::UiBuilder::new(
		[app.width as f64, app.height as f64]
	).build();

	// Create the IDs for each of the widgets at startup to use later on.
	let ids = gui::Ids::new(ui.widget_id_generator());

	// Import the fonts available to use
	let assets = find_folder::Search::KidsThenParents(3, 3).for_folder("assets").expect("Error finding folder");

	let fonts = app::Fonts {
		regular: ui.fonts.insert_from_file(assets.join("cprime.ttf")).expect("Error loading font"),
		bold: ui.fonts.insert_from_file(assets.join("cprime-bold.ttf")).expect("Error loading font"),
		italic: ui.fonts.insert_from_file(assets.join("cprime-italic.ttf")).expect("Error loading font"),
		bold_italic: ui.fonts.insert_from_file(assets.join("cprime-bold-italic.ttf")).expect("Error loading font")
	};

	ui.theme.font_id = Some(fonts.regular);

	// Piston's Cache
	// let mut glyph_cache_piston = Glyphs::new(assets.join("cprime.ttf"), window.factory.clone()).unwrap();

	// Conrod's Cache
	let image_map = conrod::image::Map::new();
	let mut glyph_cache = conrod::text::GlyphCache::new(app.width, app.height, 0.1, 0.1);
	let mut text_vertex_data = Vec::new();
	let mut text_texture_cache = {
		let buffer_len = app.width as usize * app.height as usize;
		let init = vec![128; buffer_len];
		let settings = TextureSettings::new();
		G2dTexture::from_memory_alpha(&mut window.factory, &init, app.width, app.height, &settings).expect("Error creating texture cache")
	};

	/*
		MAIN LOOP
	*/
	while let Some(evt) = window.next() {

		// Always update the cursor position.
		app.event(&evt);

		// Also let conrod update its events too
		if let Some(e) = conrod::backend::piston::event::convert(evt.clone(), app.width as f64, app.height as f64) {
			ui.handle_event(e);
		}

		// Create the UI elements each frame (because of immediate mode)
		evt.update(|_| {
			gui::gui(&mut ui.set_widgets(), &ids, &mut app, &fonts);
		});

		// Finally, draw the window to the screen.
		window.draw_2d(&evt, |context, graphics| {
			clear(color::WHITE, graphics);

			/*
				CONROD UI WIDGETS
			*/

			// A function used for caching glyphs to the texture cache.
			let cache_queued_glyphs = |graphics: &mut G2d, cache: &mut G2dTexture,
			                           rect: conrod::text::rt::Rect<u32>, app: &[u8]|
			{
				let offset = [rect.min.x, rect.min.y];
				let size = [rect.width(), rect.height()];
				let format = piston_window::texture::Format::Rgba8;
				let encoder = &mut graphics.encoder;
				text_vertex_data.clear();
				text_vertex_data.extend(app.iter().flat_map(|&b| vec![255, 255, 255, b]));
				UpdateTexture::update(cache, encoder, format, &text_vertex_data[..], offset, size)
				.expect("failed to update texture")
			};
			fn texture_from_image<T>(img: &T) -> &T { img };

			match app.gui_state {
				GUIState::DrawCreature => {
					// if let Some(ref mut creature) = app.chosen_creature {
					// 	creature.draw(context, graphics);
					// }
					if app.optmethods.len() > 0 {
						app.optmethods[0].creature_get(app.generation, app.creature).draw(context, graphics);
					}
				},
				_ => {},
			}

			// Usually, we call ui.draw_if_changed() and draw its primitives as such.
			// However, this results in the UI being drawn over by piston when there
			// is nothing to change, so we must draw *every frame* using ui.draw()
			conrod::backend::piston::draw::primitives(ui.draw(),
			                                          context, graphics,
			                                          &mut text_texture_cache,
			                                          &mut glyph_cache,
			                                          &image_map,
			                                          cache_queued_glyphs,
			                                          texture_from_image);
		});
	}
}
