extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;



const ITERATION_COOLDOWN : Duration = std::time::Duration::from_millis(200);

const COLS: u32 = 80;
const ROWS: u32 = 60;
const SIZE: u32 = 10;

const H_MARGIN : u32 = 20;
const V_MARGIN : u32 = 40;
const TOOLBAR_HEIGHT : u32 = 100;

const TOTAL_WIDTH: u32 = COLS * SIZE + H_MARGIN * 2;
const TOTAL_HEIGHT: u32 = ROWS * SIZE + V_MARGIN * 2 + TOOLBAR_HEIGHT;

const BTN_ALPHA : u8 = 200;


const COLOR_GREEN: Color = Color::RGB(87, 171, 90);
const COLOR_YELLOW: Color = Color::RGB(218, 170, 63);
const COLOR_RED: Color = Color::RGB(229, 83, 75);
const COLOR_BLUE: Color = Color::RGB(82, 155, 245);

pub fn main() {
	let sdl_context = sdl2::init().unwrap();
	let ttf_context = sdl2::ttf::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("Conway's Game Of Life :: by keelus", TOTAL_WIDTH, TOTAL_HEIGHT)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

	let mut event_pump = sdl_context.event_pump().unwrap();


	let mut iterating_population = false;
	let mut population = vec![vec![false; COLS as usize]; ROWS as usize];
	let mut previous_population = vec![vec![false; COLS as usize]; ROWS as usize];
	let mut population_number = 0;
	let mut last_interation = std::time::Instant::now();

	let mut btn_start_simulation = Button::new(COLOR_GREEN, Rect::new(H_MARGIN as i32, (V_MARGIN + 5 + ROWS*SIZE) as i32, 150, 20), "Start simulation".to_string());
	let mut btn_pause_resume_simulation = Button::new(COLOR_RED, Rect::new(H_MARGIN as i32, (V_MARGIN + 5 + ROWS*SIZE) as i32, 150, 20), "Pause simulation".to_string());
	let mut btn_abort_simulation = Button::new(COLOR_RED, Rect::new(H_MARGIN as i32 + 160, (V_MARGIN + 5 + ROWS*SIZE) as i32, 150, 20), "Abort simulation".to_string());
	let mut btn_abort_n_save_simulation = Button::new(COLOR_RED, Rect::new(H_MARGIN as i32 + 320, (V_MARGIN + 5 + ROWS*SIZE) as i32, 275, 20), "Abort simulation and save state".to_string());
	let mut btn_clear_population = Button::new(COLOR_BLUE, Rect::new(H_MARGIN as i32 + 160, (V_MARGIN + 5 + ROWS*SIZE) as i32, 150, 20), "Clear population".to_string());
	btn_pause_resume_simulation.hidden = true;
	btn_abort_simulation.hidden = true;
	btn_abort_n_save_simulation.hidden = true;


	population[3][3] = true;
	population[3][4] = true;
	population[3][5] = true;


	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => break 'running,
				Event::MouseButtonDown { x, y, .. } => {
					if !iterating_population {
						let i = (((y - V_MARGIN as i32) as f32) / (SIZE as f32)).floor() as i32;
						let j = (((x - H_MARGIN as i32) as f32) / (SIZE as f32)).floor() as i32;

						if i >= 0 && i < ROWS as i32 && j >= 0 && j < COLS as i32 {
							population[i as usize][j as usize] = !population[i as usize][j as usize];
						}
					}

					if btn_start_simulation.is_hovered() {
						println!("Start simulation");
						iterating_population = true;
						previous_population = population.clone();
						
					} else if btn_pause_resume_simulation.is_hovered() {
						println!("Pause or resume simulation");
						iterating_population = !iterating_population;

						if iterating_population {
							btn_pause_resume_simulation.text = "Pause simulation".to_string();
							btn_pause_resume_simulation.update_color(COLOR_RED);
						} else {
							btn_pause_resume_simulation.text = "Resume simulation".to_string();
							btn_pause_resume_simulation.update_color(COLOR_YELLOW);
						}
					} else if btn_abort_simulation.is_hovered() {
						println!("Abort simulation");
						iterating_population = false;

						population = previous_population.clone();
						population_number = 0;
					} else if btn_abort_n_save_simulation.is_hovered() {
						println!("Abort and save simulation");
						iterating_population = false;

						population_number = 0;
					} else if btn_clear_population.is_hovered() {
						println!("Clear population");
						population = vec![vec![false; COLS as usize]; ROWS as usize];
					}

					
					if !iterating_population {
						if population_number == 0 {
							btn_start_simulation.hidden = false;
							btn_pause_resume_simulation.hidden = true;
						} else {
							btn_start_simulation.hidden = true;
							btn_pause_resume_simulation.hidden = false;
						}
						btn_clear_population.hidden = false;
						btn_abort_simulation.hidden = true;
						btn_abort_n_save_simulation.hidden = true;
					} else {
						btn_start_simulation.hidden = true;
						btn_clear_population.hidden = true;
						btn_pause_resume_simulation.hidden = false;
						btn_abort_simulation.hidden = false;
						btn_abort_n_save_simulation.hidden = false;
					}

				},
				Event::MouseMotion { x, y, mousestate, ..} => {
					if !iterating_population && mousestate.is_mouse_button_pressed(MouseButton::Left) {
						let i = (((y - V_MARGIN as i32) as f32) / (SIZE as f32)).floor() as i32;
						let j = (((x - H_MARGIN as i32) as f32) / (SIZE as f32)).floor() as i32;

						if i >= 0 && i < ROWS as i32 && j >= 0 && j < COLS as i32 {
							population[i as usize][j as usize] = true;
						}

					}

					
					
					btn_start_simulation.update_hover(x, y);
					btn_pause_resume_simulation.update_hover(x, y);
					btn_abort_simulation.update_hover(x, y);
					btn_abort_n_save_simulation.update_hover(x, y);
					btn_clear_population.update_hover(x, y);
				}
				_ => {}
			}
		}

		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();


		let mut font = ttf_context.load_font("./fonts/EnvyCodeR_bold.ttf", 15).unwrap();
		font.set_style(sdl2::ttf::FontStyle::BOLD);

		
		let surface = font.render(format!("Population: {}", population_number).as_str())
			.blended(Color::RGBA(255, 255 ,255, 255)).unwrap();
		let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

		let TextureQuery { width, height, .. } = texture.query();

		let _ = canvas.copy(&texture, None, Some(Rect::new(H_MARGIN as i32, (V_MARGIN - height) as i32  , width, height)));
		
		draw_current_population(&mut canvas, &population, iterating_population);

		if iterating_population && last_interation.elapsed() > ITERATION_COOLDOWN {
			population = iterate_population(&population);
			population_number += 1;
			last_interation = std::time::Instant::now();
		}

		draw_lines(&mut canvas);
		draw_outerlines(&mut canvas);

		

		btn_start_simulation.draw(&mut canvas, &mut font);
		btn_pause_resume_simulation.draw(&mut canvas, &mut font);
		btn_clear_population.draw(&mut canvas, &mut font);
		btn_abort_simulation.draw(&mut canvas, &mut font);
		btn_abort_n_save_simulation.draw(&mut canvas, &mut font);

		canvas.present();

		
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




struct Button {
	color: Color,
	hover_color: Color,
	rect: Rect,
	hovered: bool,
	text: String,
	hidden: bool
}

impl Button {
	fn new(color: Color, rect: Rect, text: String) -> Self {

		let hover_color = Color::RGBA(color.r, color.g, color.b, BTN_ALPHA);
		Self {
			color,
			hover_color,
			rect,
			hovered: false,
			text,
			hidden: false
		}
	}

	fn update_color(&mut self, new_color: Color) {
		self.color = new_color;
		self.hover_color = Color::RGBA(new_color.r, new_color.g, new_color.b, BTN_ALPHA);
	}

	fn draw(&self, canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, font: &mut Font) {
		if !self.hidden {
			let surface = font.render(self.text.as_str())
				.blended(Color::RGBA(255, 255 ,255, 255)).unwrap();
			let texture_creator = canvas.texture_creator();
			let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
	
			let TextureQuery { width, height, .. } = texture.query();
	
			canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
			let _ = canvas.set_draw_color(if self.hovered {self.hover_color} else {self.color});
			let _ = canvas.fill_rect(self.rect);
			canvas.set_blend_mode(sdl2::render::BlendMode::None);
	
			let _ = canvas.copy(&texture, None, Some(Rect::new(self.rect.x + self.rect.w/2 - (width/2) as i32, self.rect.y + self.rect.h/2 - (height/2) as i32, width, height)));
		}
	}

	fn is_hovered(&self) -> bool {
		return !self.hidden && self.hovered;
	}

	fn update_hover(&mut self, x : i32, y : i32) {
		self.hovered = self.rect.contains_point(Point::new(x, y))
	}
}