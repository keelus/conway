extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

const FPS: u32 = 10;

const COLS: u32 = 80;
const ROWS: u32 = 60;
const SIZE: u32 = 10;

const H_MARGIN : u32 = 20;
const V_MARGIN : u32 = 40;
const TOOLBAR_HEIGHT : u32 = 100;

const TOTAL_WIDTH: u32 = COLS * SIZE + H_MARGIN * 2;
const TOTAL_HEIGHT: u32 = ROWS * SIZE + V_MARGIN * 2 + TOOLBAR_HEIGHT;

pub fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("rust-sdl2 demo", TOTAL_WIDTH, TOTAL_HEIGHT)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();

	let mut event_pump = sdl_context.event_pump().unwrap();


	let mut iterating_population = false;
	let mut population = vec![vec![false; COLS as usize]; ROWS as usize];

	population[3][3] = true;
	population[3][4] = true;
	population[3][5] = true;


	'running: loop {
		let frame_start = std::time::Instant::now();

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => break 'running,
				Event::KeyDown { keycode:Some(Keycode::Return), .. } => {
					iterating_population = !iterating_population
				}
				Event::MouseButtonDown { x, y, .. } => {
					if !iterating_population {
						let i = (((y - V_MARGIN as i32) as f32) / (SIZE as f32)).floor() as i32;
						let j = (((x - H_MARGIN as i32) as f32) / (SIZE as f32)).floor() as i32;
	
						population[i as usize][j as usize] = !population[i as usize][j as usize];
					}

				},
				_ => {}
			}
		}

		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();




		
		draw_current_population(&mut canvas, &population, iterating_population);

		if iterating_population {
			population = iterate_population(&population);
		}

		draw_lines(&mut canvas);
		draw_outerlines(&mut canvas);


		canvas.present();

		let frame_duration = frame_start.elapsed();
		let wanted_frame_duration = Duration::from_secs(1) / FPS;

		if frame_duration < wanted_frame_duration {
			std::thread::sleep(wanted_frame_duration - frame_duration)
		}
	}
}

fn draw_lines(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>) {
	canvas.set_draw_color(Color::RGB(40, 40, 40));
	for i in 1..COLS {
		let start_point = Point::new((H_MARGIN + SIZE * i) as i32, V_MARGIN as i32);
		let end_point = Point::new((H_MARGIN + SIZE * i) as i32, (V_MARGIN + (ROWS * SIZE)-1) as i32);
		let _ = canvas.draw_line(start_point, end_point);
	}
	for i in 1..ROWS {
		let start_point = Point::new(H_MARGIN as i32, (V_MARGIN + SIZE * i) as i32);
		let end_point = Point::new((H_MARGIN + (COLS * SIZE)-1) as i32, (V_MARGIN + SIZE * i) as i32);
		let _ = canvas.draw_line(start_point, end_point);
	}
}

fn draw_outerlines(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>) {
	canvas.set_draw_color(Color::RGB(80, 80, 80));
	let _ = canvas.draw_rect(Rect::new(H_MARGIN as i32, V_MARGIN as i32, COLS * SIZE, ROWS * SIZE));
}

fn draw_current_population(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, population : &Vec<Vec<bool>>, iterating : bool) {
	for i in 0..ROWS {
		for j in 0..COLS {
			if population[i as usize][j as usize] == true {
				canvas.set_draw_color(Color::RGB(if iterating { 0 } else { 255 }, if iterating { 255 } else {0}, 0));
				let drawing_rect = Rect::new((H_MARGIN + j * SIZE) as i32, (V_MARGIN + i * SIZE) as i32, SIZE, SIZE);
				let _ = canvas.fill_rect(drawing_rect);
			}
		}
	}
}


fn iterate_population(population : &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut new_population : Vec<Vec<bool>> = population.clone();

	for i in 0..ROWS {
		for j in 0..COLS {
			let neighbors = get_neighbors(population, i as i32, j as i32);
			if population[i as usize][j as usize] == true { // Alive
				if !(neighbors == 2 || neighbors == 3) {
					new_population[i as usize][j as usize] = false;
				}
			} else { // Dead
				if neighbors == 3 {
					new_population[i as usize][j as usize] = true;
				}
			}
		}
	}


	return new_population;
}

fn get_neighbors(population : &Vec<Vec<bool>>, target_i : i32, target_j : i32) -> i8 {
	let mut neighbors  = 0 as i8;

	let search_i_from = i32::max(0, target_i - 1);
	let search_i_to = i32::min(ROWS as i32, target_i + 2);
	let search_j_from = i32::max(0, target_j - 1);
	let search_j_to = i32::min(COLS as i32, target_j + 2);

	for i in search_i_from..search_i_to {
		for j in search_j_from..search_j_to {
			if i != target_i || j != target_j {
				if population[i as usize][j as usize] == true {
					neighbors += 1;
				}
			}

		}
	}
	
	return neighbors;
}