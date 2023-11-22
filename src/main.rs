extern crate sdl2;

mod button;
mod button_icon;
mod scene;

use std::time::Duration;

use scene::Scene;
use sdl2::{pixels::Color, ttf::FontStyle};


// Generation vector size
const MATRIX_SIZE: u32 = 1024*8;	// How many columns and rows has the entire matrix (MATRIX_SIZE x MATRIX_SIZE)
const SUB_MATRIX_SIZE: u32 = 128;	// How many columns and rows the sub matrix will have (it's a boolean matrix that stores the boolean of change or not represeting a chunk of cells)
const SUB_MATRIX_CHUNK_SIZE: u32 = MATRIX_SIZE / SUB_MATRIX_SIZE; // The chunk size of cells that will represent each boolean sub matrix chunk



// Visible grid
const SIZE: u32 = 10; // Width and height of each visible cell
const VIEW_COLS : u32 = 80; // How many columns of cells will be visible
const VIEW_ROWS : u32 = 60; // How many rows of cells will be visible

// Generated grid values
const GRID_WIDTH: u32 = SIZE * VIEW_COLS;
const GRID_HEIGHT: u32 = SIZE * VIEW_ROWS;

// Window margins & sizes
const H_MARGIN : u32 = 20;
const V_MARGIN : u32 = 40;
const TOOLBAR_HEIGHT : u32 = 30;

// Cooldown to wait between each generation iteration
const ITERATION_COOLDOWN : Duration = std::time::Duration::from_millis(200);

// Colors
const COLOR_GREEN: Color =	 Color::RGB(87, 171, 90);
const COLOR_YELLOW: Color =	 Color::RGB(218, 170, 63);
const COLOR_RED: Color = 	 Color::RGB(229, 83, 75);
const COLOR_BLUE: Color =	 Color::RGB(82, 155, 245);
const COLOR_WHITE: Color =	 Color::RGB(205, 217, 229);
const COLOR_BLACK_1: Color = Color::RGB(20, 20, 20);
const COLOR_BLACK_2: Color = Color::RGB(40, 40, 40);
const COLOR_BLACK_3: Color = Color::RGB(80, 80, 80);


pub fn main() {
	if MATRIX_SIZE < VIEW_COLS || MATRIX_SIZE < VIEW_ROWS {
		panic!("[ERROR] Total population rows x cols should be greater than shown rows x cols");
	}


	let sdl_context = sdl2::init().unwrap();
	let ttf_context = sdl2::ttf::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("Conway's Game Of Life :: by keelus", VIEW_COLS * SIZE + H_MARGIN * 2, VIEW_ROWS * SIZE + V_MARGIN + 20 + TOOLBAR_HEIGHT)
		.position_centered()
		.build()
		.unwrap();

	let canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();

	let mut scene = Scene::new(canvas);
	scene.initialize({
		let mut main_font = ttf_context.load_font("./fonts/EnvyCodeR_bold.ttf", 15).unwrap();
		main_font.set_style(FontStyle::BOLD);
		main_font
	});
	
	let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();


	'running: loop {
		let should_break = scene.handle_events(event_pump.poll_iter());
		if should_break {
			break 'running;
		}

		scene.iteration();
	}
}