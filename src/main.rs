extern crate sdl2;

mod button;
mod button_icon;
mod scene;

use scene::Scene;
use sdl2::pixels::Color;



const COLS: u32 = 100;
const ROWS: u32 = 100;
const SIZE: u32 = 10;

const VIEW_COLS : u32 = 80;
const VIEW_ROWS : u32 = 60;

const H_MARGIN : u32 = 20;
const V_MARGIN : u32 = 40;
const TOOLBAR_HEIGHT : u32 = 30;

const TOTAL_WIDTH: u32 = VIEW_COLS * SIZE + H_MARGIN * 2;
const TOTAL_HEIGHT: u32 = VIEW_ROWS * SIZE + V_MARGIN * 2 + TOOLBAR_HEIGHT;



const COLOR_GREEN: Color = Color::RGB(87, 171, 90);
const COLOR_YELLOW: Color = Color::RGB(218, 170, 63);
const COLOR_RED: Color = Color::RGB(229, 83, 75);
const COLOR_BLUE: Color = Color::RGB(82, 155, 245);



pub fn main() {
	if COLS < VIEW_COLS || ROWS < VIEW_ROWS {
		panic!("[ERROR] Total population rows x cols should be greater than shown rows x cols");
	}

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("Conway's Game Of Life :: by keelus", TOTAL_WIDTH, TOTAL_HEIGHT)
		.position_centered()
		.build()
		.unwrap();

	let canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();

	let mut scene = Scene::new(canvas);
	
	let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();


	'running: loop {
		let should_break = scene.handle_events(event_pump.poll_iter());
		if should_break {
			break 'running;
		}

		scene.iteration();
	}
}