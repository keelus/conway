extern crate sdl2;


use core::fmt;
use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

use crate::button;
use crate::button::Button;
use crate::button_icon;
use crate::button_icon::ButtonIcon;


const ITERATION_COOLDOWN : Duration = std::time::Duration::from_millis(200);

const COLS: u32 = 100;
const ROWS: u32 = 100;
const SIZE: u32 = 10;

const VIEW_COLS : u32 = 80;
const VIEW_ROWS : u32 = 60;

const GRID_BIG_CELL_SIZE : u32 = 5;

const H_MARGIN : u32 = 20;
const V_MARGIN : u32 = 40;


#[derive(PartialEq)]
enum Tool {
	HAND = 0,
	PENCIL = 1,
	ERASER = 2
}


impl fmt::Display for Tool {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Tool::HAND => write!(f, "Hand"),
			Tool::PENCIL => write!(f, "Pencil"),
			Tool::ERASER => write!(f, "Eraser")
		}
	}
}



pub struct Scene {
	canvas: sdl2::render::Canvas<sdl2::video::Window>,
	ttf_context: sdl2::ttf::Sdl2TtfContext,
	
	
	texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,

	active_tool: Tool,
	
	
	top_left_col: u32,
	top_left_row: u32,
	
	iterating_generation: bool,
	generation: Vec<Vec<bool>>,
	previous_generation: Vec<Vec<bool>>,
	generation_number: i32,
	population_amount: u32,
	last_interation: Instant,

	btn_start_simulation: Button,
	btn_pause_resume_simulation: Button,
	btn_abort_simulation: Button,
	btn_abort_n_save_simulation: Button,
	btn_clear_generation: Button,

	btn_tool_pencil: ButtonIcon,
	btn_tool_eraser: ButtonIcon,
	btn_tool_hand: ButtonIcon,

	dragging: bool,
	dragging_start: (i32, i32),
}

impl Scene {
	pub fn new(canvas: sdl2::render::Canvas<sdl2::video::Window>) -> Self {
		let texture_creator = canvas.texture_creator();

		let active_tool : Tool = Tool::PENCIL;
	
	
		let top_left_col : u32 = (crate::COLS - crate::VIEW_COLS)/2;
		let top_left_row : u32 = (crate::ROWS - crate::VIEW_ROWS)/2;
	
		let iterating_generation = false;
		let generation = vec![vec![false; crate::COLS as usize]; crate::ROWS as usize];
		let previous_generation = vec![vec![false; crate::COLS as usize]; crate::ROWS as usize];
		let generation_number = 0;
		let population_amount = 0;
		let last_interation = std::time::Instant::now();
	
		let btn_start_simulation 		= button::Button::new(crate::COLOR_GREEN, Rect::new(crate::H_MARGIN as i32, (crate::V_MARGIN + 5 + crate::VIEW_ROWS*crate::SIZE) as i32, 70, 30), "Start".to_string());
		let mut btn_pause_resume_simulation = button::Button::new(crate::COLOR_YELLOW, Rect::new(crate::H_MARGIN as i32, (crate::V_MARGIN + 5 + crate::VIEW_ROWS*crate::SIZE) as i32, 70, 30), "Pause".to_string());
		let mut btn_abort_simulation 		= button::Button::new(crate::COLOR_RED, Rect::new(crate::H_MARGIN as i32 + 80, (crate::V_MARGIN + 5 + crate::VIEW_ROWS*crate::SIZE) as i32, 70, 30), "Abort".to_string());
		let mut btn_abort_n_save_simulation = button::Button::new(crate::COLOR_RED, Rect::new(crate::H_MARGIN as i32 + 160, (crate::V_MARGIN + 5 + crate::VIEW_ROWS*crate::SIZE) as i32, 190, 30), "Abort and save state".to_string());
		let btn_clear_generation 		= button::Button::new(crate::COLOR_BLUE, Rect::new((crate::H_MARGIN + crate::VIEW_COLS*crate::SIZE) as i32 - 150, (crate::V_MARGIN + 5 + crate::VIEW_ROWS*crate::SIZE) as i32, 150, 30), "Clear population".to_string());
		btn_pause_resume_simulation.set_hidden(true);
		btn_abort_simulation.set_hidden(true);
		btn_abort_n_save_simulation.set_hidden(true);
		
		let mut btn_tool_pencil = button_icon::ButtonIcon::new(Rect::new((crate::H_MARGIN + crate::VIEW_COLS*crate::SIZE) as i32 - 190 - 70, (crate::V_MARGIN + 5 + crate::VIEW_ROWS*crate::SIZE) as i32, 30, 30), "./icons/pencil.bmp".to_string());
		let btn_tool_eraser = button_icon::ButtonIcon::new(Rect::new((crate::H_MARGIN + crate::VIEW_COLS*crate::SIZE) as i32 - 190 - 35, (crate::V_MARGIN + 5 + crate::VIEW_ROWS*crate::SIZE) as i32, 30, 30), "./icons/eraser.bmp".to_string());
		let btn_tool_hand 	= button_icon::ButtonIcon::new(Rect::new((crate::H_MARGIN + crate::VIEW_COLS*crate::SIZE) as i32 - 190,	  (crate::V_MARGIN + 5 + crate::VIEW_ROWS*crate::SIZE) as i32, 30, 30), "./icons/hand.bmp".to_string());
		btn_tool_pencil.set_active(true);
	
	
		let dragging = false;
		let dragging_start = (-1, -1);
	
	
		let ttf_context: sdl2::ttf::Sdl2TtfContext = sdl2::ttf::init().unwrap();


		Self {
			canvas,
			texture_creator,
			ttf_context,

			active_tool,

			top_left_col,
			top_left_row,

			iterating_generation,
			generation,
			previous_generation,
			generation_number,
			population_amount,
			last_interation,

			btn_start_simulation,
			btn_pause_resume_simulation,
			btn_abort_simulation,
			btn_abort_n_save_simulation,
			btn_clear_generation,

			btn_tool_pencil,
			btn_tool_eraser,
			btn_tool_hand,

			dragging,
			dragging_start
		}
	}
	pub fn iteration(&mut self) {
		let mut main_font = self.ttf_context.load_font("./fonts/EnvyCodeR_bold.ttf", 15).unwrap(); // TODO: Move this
		main_font.set_style(sdl2::ttf::FontStyle::BOLD);

		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();


		
		let surface = main_font.render(format!("Generation: {} :: Population: {}  [current_tool={}] [i:{}, j:{}]", self.generation_number, self.population_amount, self.active_tool.to_string(), self.top_left_row, self.top_left_col).as_str())
			.blended(Color::RGBA(255, 255 ,255, 255)).unwrap();
		let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();

		let TextureQuery { width, height, .. } = texture.query();

		let _ = self.canvas.copy(&texture, None, Some(Rect::new(crate::H_MARGIN as i32, (V_MARGIN - height - 5) as i32  , width, height)));
		
		self.population_amount = draw_current_generation(&mut self.canvas, &self.generation, self.iterating_generation, self.top_left_row, self.top_left_col);

		if self.iterating_generation && self.last_interation.elapsed() > ITERATION_COOLDOWN {
			self.generation = iterate_generation(&self.generation);
			self.generation_number += 1;
			self.last_interation = std::time::Instant::now();
		}

		draw_lines(&mut self.canvas);
		draw_outerlines(&mut self.canvas);


		self.btn_start_simulation.draw(&mut self.canvas, &mut main_font);
		self.btn_pause_resume_simulation.draw(&mut self.canvas, &mut main_font);
		self.btn_clear_generation.draw(&mut self.canvas, &mut main_font);
		self.btn_abort_simulation.draw(&mut self.canvas, &mut main_font);
		self.btn_abort_n_save_simulation.draw(&mut self.canvas, &mut main_font);
		
		self.btn_tool_pencil.draw(&mut self.canvas);
		self.btn_tool_eraser.draw(&mut self.canvas);
		self.btn_tool_hand.draw(&mut self.canvas);

		self.canvas.present();
	}

	pub fn handle_events<'h>(&mut self, event_iterator: sdl2::event::EventPollIterator<'h>) -> bool {
		for event in event_iterator {
			match event {
				Event::Quit { .. } => return true,
				Event::MouseButtonUp { .. } => {
					if self.dragging {
						self.dragging = false
					}
				},
				Event::MouseButtonDown { x, y, mouse_btn, .. } => {
					match mouse_btn {
						MouseButton::Left => {
							if self.active_tool == Tool::HAND {
								let (_, clicked_canvas) = get_click_indexes(x, y);
								if clicked_canvas {
									self.dragging = true;
									self.dragging_start = (x, y);
								}
							}
	
							if !self.iterating_generation {
								let (indexes, clicked_canvas) = get_click_indexes(x, y);
								if clicked_canvas {
									if self.active_tool == Tool::PENCIL {
										self.generation[(self.top_left_row as i32 + indexes.0) as usize][(self.top_left_col as i32 + indexes.1)  as usize] = true;
									} else if self.active_tool == Tool::ERASER {
										self.generation[(self.top_left_row as i32 + indexes.0) as usize][(self.top_left_col as i32 + indexes.1)  as usize] = false;
									}
								}
							}
	
							if self.btn_start_simulation.is_hovered() {
								self.iterating_generation = true;
								self.previous_generation = self.generation.clone();
								self.btn_pause_resume_simulation.set_text("Pause".to_string());
								self.active_tool = Tool::HAND;
								self.btn_tool_hand.set_active(true);
								self.btn_tool_pencil.set_active(false);
								self.btn_tool_eraser.set_active(false);
							} else if self.btn_pause_resume_simulation.is_hovered() {
								self.iterating_generation = !self.iterating_generation;
	
								if self.iterating_generation {
									self.btn_pause_resume_simulation.set_text("Pause".to_string());
									self.active_tool = Tool::HAND;
									self.btn_tool_hand.set_active(true);
									self.btn_tool_pencil.set_active(false);
									self.btn_tool_eraser.set_active(false);
								} else {
									self.btn_pause_resume_simulation.set_text("Resume".to_string());
									self.active_tool = Tool::PENCIL;
									self.btn_tool_pencil.set_active(true);
									self.btn_tool_hand.set_active(false);
									self.btn_tool_eraser.set_active(false);
								}
	
							} else if self.btn_abort_simulation.is_hovered() {
								self.iterating_generation = false;
	
								self.generation = self.previous_generation.clone();
								self.generation_number = 0;
								self.active_tool = Tool::PENCIL;
								self.btn_tool_pencil.set_active(true);
								self.btn_tool_hand.set_active(false);
								self.btn_tool_eraser.set_active(false);
							} else if self.btn_abort_n_save_simulation.is_hovered() {
								self.iterating_generation = false;
	
								self.generation_number = 0;
								self.active_tool = Tool::PENCIL;
								self.btn_tool_pencil.set_active(true);
								self.btn_tool_hand.set_active(false);
								self.btn_tool_eraser.set_active(false);
							} else if self.btn_clear_generation.is_hovered() {
								self.generation = vec![vec![false; COLS as usize]; ROWS as usize];
							} else if self.btn_tool_pencil.is_hovered() {
								self.active_tool = Tool::PENCIL;
	
								self.btn_tool_pencil.set_active(true);
								self.btn_tool_eraser.set_active(false);
								self.btn_tool_hand.set_active(false);
							} else if self.btn_tool_eraser.is_hovered() {
								self.active_tool = Tool::ERASER;
								
								self.btn_tool_pencil.set_active(false);
								self.btn_tool_eraser.set_active(true);
								self.btn_tool_hand.set_active(false);
							} else if self.btn_tool_hand.is_hovered() {
								self.active_tool = Tool::HAND;
								
								self.btn_tool_pencil.set_active(false);
								self.btn_tool_eraser.set_active(false);
								self.btn_tool_hand.set_active(true);
							}
							
							if !self.iterating_generation {
								if self.generation_number == 0 {
									self.btn_start_simulation.set_hidden(false);
									self.btn_pause_resume_simulation.set_hidden(true);
									self.btn_abort_simulation.set_hidden(true);
									self.btn_abort_n_save_simulation.set_hidden(true);
								} else {
									self.btn_start_simulation.set_hidden(true);
									self.btn_pause_resume_simulation.set_hidden(false);
									self.btn_abort_simulation.set_hidden(false);
									self.btn_abort_n_save_simulation.set_hidden(false);
								}
								self.btn_clear_generation.set_hidden(false);
								self.btn_tool_pencil.set_hidden(false);
								self.btn_tool_eraser.set_hidden(false);
							} else {
								self.btn_start_simulation.set_hidden(true);
								self.btn_clear_generation.set_hidden(true);
								self.btn_pause_resume_simulation.set_hidden(false);
								self.btn_abort_simulation.set_hidden(false);
								self.btn_abort_n_save_simulation.set_hidden(false);
								self.btn_tool_pencil.set_hidden(true);
								self.btn_tool_eraser.set_hidden(true);
							}
						},
						MouseButton::Middle => {
							self.active_tool = Tool::HAND;
							self.btn_tool_hand.set_active(true);
							self.btn_tool_pencil.set_active(false);
							self.btn_tool_eraser.set_active(false);
	
							
							let (_, clicked_canvas) = get_click_indexes(x, y);
							if clicked_canvas {
								self.dragging = true;
								self.dragging_start = (x, y);
							}
						},
						_ => {}
					}
				},
				Event::MouseMotion { x, y, mousestate, ..} => {
					if !self.iterating_generation && mousestate.is_mouse_button_pressed(MouseButton::Left) {
						let (indexes, clicked_canvas) = get_click_indexes(x, y);
						if clicked_canvas {
							if self.active_tool == Tool::PENCIL {
								self.generation[(self.top_left_row as i32 + indexes.0) as usize][(self.top_left_col as i32 + indexes.1)  as usize] = true;
							} else if self.active_tool == Tool::ERASER {
								self.generation[(self.top_left_row as i32 + indexes.0) as usize][(self.top_left_col as i32 + indexes.1)  as usize] = false;
							}
						}
					}

					if self.active_tool == Tool::HAND && self.dragging {
						let (_, clicked_canvas) = get_click_indexes(x, y);
						if clicked_canvas {
							let dragging_end = (x, y);
		
							let difference_x = - (dragging_end.0 - self.dragging_start.0);
							let difference_y = - (dragging_end.1 - self.dragging_start.1);

							let mut new_x = self.dragging_start.0;
							let mut new_y = self.dragging_start.1;


							let move_units = 1;
							if difference_y.abs() as u32 > SIZE {
								if difference_y > 0 {
									if self.top_left_row + move_units < ROWS-VIEW_ROWS {
										self.top_left_row += move_units;
									}
								} else if difference_y < 0 {
									if self.top_left_row >= move_units {
										self.top_left_row -= move_units;
									}
								}
								new_y = y;
							}
							if difference_x.abs() as u32 > SIZE {
								if difference_x > 0 {
									if self.top_left_col + move_units < COLS-VIEW_COLS {
										self.top_left_col += move_units;
									}
								} else if difference_x < 0 {
									if self.top_left_col >= move_units {
										self.top_left_col -= move_units;
									}
								}
								new_x = x;
							}

							self.dragging_start = (new_x, new_y);
						}
					}

					
					self.btn_start_simulation.update_hover(x, y);
					self.btn_pause_resume_simulation.update_hover(x, y);
					self.btn_abort_simulation.update_hover(x, y);
					self.btn_abort_n_save_simulation.update_hover(x, y);
					self.btn_clear_generation.update_hover(x, y);

					self.btn_tool_pencil.update_hover(x, y);
					self.btn_tool_eraser.update_hover(x, y);
					self.btn_tool_hand.update_hover(x, y);

				}
				_ => {}
			}
		}
		return false;
	}
}

fn get_click_indexes(x: i32, y: i32) -> ((i32, i32), bool) {
	let i = (((y - V_MARGIN as i32) as f32) / (SIZE as f32)).floor() as i32;
	let j = (((x - H_MARGIN as i32) as f32) / (SIZE as f32)).floor() as i32;

	if i >= 0 && i < VIEW_ROWS as i32 && j >= 0 && j < VIEW_COLS as i32 {
		return ((i, j), true);
	}
	return ((-1, -1), false);
}

fn draw_lines(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>) {
	canvas.set_draw_color(Color::RGB(40, 40, 40));
	for i in 1..VIEW_COLS {
		let start_point = Point::new((H_MARGIN + SIZE * i) as i32, V_MARGIN as i32);
		let end_point = Point::new((H_MARGIN + SIZE * i) as i32, (V_MARGIN + (VIEW_ROWS * SIZE)-1) as i32);
		let _ = canvas.draw_line(start_point, end_point);
	}
	for i in 1..VIEW_ROWS {
		let start_point = Point::new(H_MARGIN as i32, (V_MARGIN + SIZE * i) as i32);
		let end_point = Point::new((H_MARGIN + (VIEW_COLS * SIZE)-1) as i32, (V_MARGIN + SIZE * i) as i32);
		let _ = canvas.draw_line(start_point, end_point);
	}
}

fn draw_outerlines(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>) {
	canvas.set_draw_color(Color::RGB(80, 80, 80));
	let _ = canvas.draw_rect(Rect::new(H_MARGIN as i32, V_MARGIN as i32, VIEW_COLS * SIZE, VIEW_ROWS * SIZE));
}

fn draw_current_generation(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, generation : &Vec<Vec<bool>>, iterating : bool, tl_row: u32, tl_col: u32) -> u32 {
	let mut population = 0;
	for i in 0..VIEW_ROWS {
		for j in 0..VIEW_COLS {
			if generation[(tl_row + i) as usize][(tl_col + j) as usize] == true {
				canvas.set_draw_color(Color::RGB(if iterating { 0 } else { 255 }, if iterating { 255 } else {0}, 0));
				let drawing_rect = Rect::new((H_MARGIN + j * SIZE) as i32, (V_MARGIN + i * SIZE) as i32, SIZE, SIZE);
				let _ = canvas.fill_rect(drawing_rect);
				population += 1;
			} else {
				if (((tl_row+i)/GRID_BIG_CELL_SIZE)%2 == 0 && ((tl_col+j)/GRID_BIG_CELL_SIZE)%2 == 0) || (((tl_row+i)/GRID_BIG_CELL_SIZE)%2 != 0 && ((tl_col+j)/GRID_BIG_CELL_SIZE)%2 != 0 ) {
					canvas.set_draw_color(Color::RGB(20, 20, 20));
					let drawing_rect = Rect::new((H_MARGIN + j * SIZE) as i32, (V_MARGIN + i * SIZE) as i32, SIZE, SIZE);
					let _ = canvas.fill_rect(drawing_rect);
				}
			}
		}
	}
	return population;
}


fn iterate_generation(generation : &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut new_generation : Vec<Vec<bool>> = generation.clone();

	for i in 0..ROWS {
		for j in 0..COLS {
			let neighbors = get_neighbors(generation, i as i32, j as i32);
			if generation[i as usize][j as usize] == true { // Alive
				if !(neighbors == 2 || neighbors == 3) {
					new_generation[i as usize][j as usize] = false;
				}
			} else { // Dead
				if neighbors == 3 {
					new_generation[i as usize][j as usize] = true;
				}
			}
		}
	}


	return new_generation;
}

fn get_neighbors(generation : &Vec<Vec<bool>>, target_i : i32, target_j : i32) -> i8 {
	let mut neighbors  = 0 as i8;

	let search_i_from = i32::max(0, target_i - 1);
	let search_i_to = i32::min(ROWS as i32, target_i + 2);
	let search_j_from = i32::max(0, target_j - 1);
	let search_j_to = i32::min(COLS as i32, target_j + 2);

	for i in search_i_from..search_i_to {
		for j in search_j_from..search_j_to {
			if i != target_i || j != target_j {
				if generation[i as usize][j as usize] == true {
					neighbors += 1;
				}
			}

		}
	}
	
	return neighbors;
}