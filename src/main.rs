extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;


const COLS: u32 = 180;
const ROWS: u32 = 80;
const SIZE: u32 = 10;

const TOTAL_WIDTH: u32 = COLS * SIZE;
const TOTAL_HEIGHT: u32 = ROWS * SIZE;

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

	let mut population = vec![vec![false; COLS as usize]; ROWS as usize];

	population[3][3] = true;
	population[3][4] = true;
	population[3][5] = true;


	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => break 'running,
				_ => {}
			}
		}

		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();



		draw_current_population(&mut canvas, &population);

		population = iterate_population(&population);

		draw_lines(&mut canvas);


		canvas.present();
	}
}

fn draw_lines(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>) {
	canvas.set_draw_color(Color::RGB(40, 40, 40));
	for i in 1..COLS {
		let start_point = Point::new((SIZE * i) as i32, 0);
		let end_point = Point::new((SIZE * i) as i32, TOTAL_HEIGHT as i32);
		let _ = canvas.draw_line(start_point, end_point);
	}
	for i in 1..ROWS {
		let start_point = Point::new(0, (SIZE * i) as i32);
		let end_point = Point::new(TOTAL_WIDTH as i32, (SIZE * i) as i32);
		let _ = canvas.draw_line(start_point, end_point);
	}
}

fn draw_current_population(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, population : &Vec<Vec<bool>>) {
	for i in 0..ROWS {
		for j in 0..COLS {
			if population[i as usize][j as usize] == true {
				canvas.set_draw_color(Color::RGB(255, 0, 0));
				let drawing_rect = Rect::new((j * SIZE) as i32, (i * SIZE) as i32, SIZE, SIZE);
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