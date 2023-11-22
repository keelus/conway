extern crate sdl2;

mod button;
mod button_icon;
mod scene;

use std::time::Duration;

use scene::Scene;
use sdl2::{pixels::Color, ttf::FontStyle};


// Generation vector size
const MATRIX_SIZE: u32 = 1024*4;
const SUB_MATRIX_SIZE: u32 = 128;
const SUB_MATRIX_CHUNK_SIZE: u32 = MATRIX_SIZE / SUB_MATRIX_SIZE;



// Visible grid
const SIZE: u32 = 10;
const VIEW_COLS : u32 = 80;
const VIEW_ROWS : u32 = 60;

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


pub fn is_perfect_square(number: u32) -> bool {
    let square_root = (number as f32).powf(0.5);
    
    return square_root.powf(2 as f32) == number as f32
}

pub fn valid_constants() -> bool {
	if MATRIX_SIZE%8 != 0 {
		println!("Error on matrix size constant value.");
		return false;
	}
	// if !is_perfect_square(SUB_MATRIX_CHUNK_ELEMENTS) {
	// 	println!("Error on sub matrix size constant value.");
	// 	return false;
	// }

	return true
}


pub fn main() {
	if valid_constants() == false {
		panic!("[ERROR] Check constants.");
	} else {
		println!("[INFO] Constants are valid");
		println!("\tMATRIX_SIZE: {}", MATRIX_SIZE);
		println!("\tSUB_MATRIX_SIZE: {}", SUB_MATRIX_SIZE);
		println!("\tSUB_MATRIX_CHUNK_SIZE: {}", SUB_MATRIX_CHUNK_SIZE);
		
		// println!("\tSUB_MATRIX_CHUNK_ELEMENTS: {}", SUB_MATRIX_CHUNK_ELEMENTS);
		// println!("\tSUB_MATRIX_CHUNK_SIZE: {}", *SUB_MATRIX_CHUNK_SIZE);
	}

	
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