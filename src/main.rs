#![allow(dead_code)]
#![allow(unused_variables)]
#![recursion_limit="128"]

#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate piston;
extern crate find_folder;
extern crate rand;
extern crate time;

mod gui;
mod app;
mod modal;
mod creature;
mod population;
mod optimisationmethods;
mod physics;

use piston_window::*;
use piston_window::texture::UpdateTexture;
use gui::GUIState;
use app::UIData;

fn main() {
	// Initialise the app data
	//                        Window / Menu Title                        Windowed     Fullscreen    FPS
	let mut app = UIData::new("Optimisation Method Creature Generation", (1280, 720), (1920, 1080), 60);

	// Create the window
	let mut window : PistonWindow =
		WindowSettings::new(app.title, [app.width, app.height])
		.exit_on_esc(true)
		.opengl(OpenGL::V3_2)
		.vsync(true)
		.fullscreen(app.fullscreen)
		.build()
		.expect("Error creating window");

	// Ensure the program runs at 60fps to not overload the system
	window.set_ups(app.fps as u64);
	window.set_max_fps(app.fps as u64);

	// Create the UI with the same width and height as the window
	let mut ui = conrod::UiBuilder::new(
		[app.width as f64, app.height as f64]
	).build();

	// Create the IDs for each of the widgets at startup to use later on.
	let ids = gui::Ids::new(ui.widget_id_generator());

	// Import the fonts available to use
	let assets = find_folder::Search::KidsThenParents(3, 3).for_folder("assets").expect("Error finding folder");

	let fonts = app::Fonts {
		regular: ui.fonts.insert_from_file(assets.join("NotoSansUI-Regular.ttf")).expect("Error loading font"),
		bold: ui.fonts.insert_from_file(assets.join("NotoSansUI-Bold.ttf")).expect("Error loading font"),
		italic: ui.fonts.insert_from_file(assets.join("NotoSansUI-Italic.ttf")).expect("Error loading font"),
		bold_italic: ui.fonts.insert_from_file(assets.join("NotoSansUI-BoldItalic.ttf")).expect("Error loading font")
	};

	ui.theme.font_id = Some(fonts.regular);

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

			match app.gui_state {
			GUIState::Spectate => {
				let mut creature = app.optmethods[app.spectate_method].creature_get(app.spectate_generation, app.spectate_creature);
				let x = creature.fitness() as f64;
				let y = 84.0;

				for idx in 0 .. 2 {
					let xx = -(x % app.width as f64) + (idx * app.width) as f64;
					rectangle([0.0, 0.0, 0.0, 0.35], [xx, 0.0, app.width as f64 / 2.0, app.height as f64], context.transform, graphics);
				}

				creature.draw((app.width as f64 / 2.0) - x - 128.0, app.height as f64 - 256.0 - y, 1.0, context, graphics);
				rectangle([0.15, 0.9, 0.1, 1.0], [0.0, app.height as f64 - y, app.width as f64, y], context.transform, graphics);
				if app.draw_simulation && app.simulation_frame < physics::SIM_LENGTH {
					physics::simulation_step(app.simulation_frame, &mut creature);
					app.simulation_frame += 1;
				}
			},
			_ => {}
			}

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

			match app.gui_state {
			GUIState::Generations => {
				if !app.modal_visible {
					let method_height = (app.height as f64 - (gui::MARGIN * 2.0) - 112.0) / app.optmethods.len() as f64;
					for mtd in 0 .. app.optmethods.len() {
						let x = 344.0;
						let y = gui::MARGIN + 64.0;
						let w = 140.0;
						let s = w / creature::BOUNDS_NODE_X.end as f64;
						let yw = method_height;
						let padding = 4.0;
						rectangle([1.0, 1.0, 1.0, 1.0],
							[x - padding, y - padding + (mtd as f64 * yw), w + (padding * 2.0), w + (padding * 2.0)],
							context.transform, graphics);
						let data = app.optmethods[mtd].get_data();
						let ref creature = data.generations[app.spectate_generation].creatures[data.spectate_creature];
						creature.draw(x, y + (mtd as f64 * yw), s, context, graphics);
					}
				}
			},
			_ => {}
			}
		});
	}
}

mod tests {
	use rand::StdRng;

	pub fn init() -> StdRng {
		if let Ok(rng) = StdRng::new() {
			rng
		} else {
			panic!("Could not create RNG");
		}
	}
}
