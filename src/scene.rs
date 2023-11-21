extern crate sdl2;
extern crate rayon;

use rayon::prelude::*;
use core::fmt;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;

use crate::button;
use crate::button::Button;
use crate::button_icon;
use crate::button_icon::ButtonIcon;



const BTN_WIDTH: u32 = 70;
const BTN_H_MARGIN: u32 = 10;
const BTN_ABORT_N_SAVE_WIDTH: u32 = 190;
const BTN_CLEAR_WIDTH: u32 = 150;
const BTN_HEIGHT: u32 = 30;
const BTN_SQUARE_SIZE: u32 = BTN_HEIGHT;
const BTN_SQUARE_H_MARGIN: u32 = 5;
const GRID_BOTTOM_MARGIN: u32 = 5;


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

#[derive(PartialEq, Clone, Copy)]
enum State {
	IDLE,
	ITERATING,
	PAUSE
}



pub struct Scene<'scene> {
	canvas: sdl2::render::Canvas<sdl2::video::Window>,
	
	texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,

	active_tool: Tool,
	
	top_left_col: u32,
	top_left_row: u32,
	
	state: State,
	generation: Vec<Vec<bool>>,
	previous_generation: Vec<Vec<bool>>,
	generation_number: i32,
	population_amount: u32,
	last_iteration: Instant,

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

	main_font: Option<Font<'scene, 'static>>
}

impl<'s> Scene<'s> {
	pub fn new(canvas: sdl2::render::Canvas<sdl2::video::Window>) -> Self {
		let texture_creator = canvas.texture_creator();
	
		let generation = vec![vec![false; crate::COLS as usize]; crate::ROWS as usize];
		let previous_generation = vec![vec![false; crate::COLS as usize]; crate::ROWS as usize];
	
		Self {
			canvas: canvas,
			texture_creator,

			active_tool: Tool::PENCIL,

			top_left_col: (crate::COLS - crate::VIEW_COLS)/2,
			top_left_row: (crate::ROWS - crate::VIEW_ROWS)/2,

			state: State::IDLE,
			generation,
			previous_generation,
			generation_number: 0,
			population_amount: 0,
			last_iteration: std::time::Instant::now(),

			btn_start_simulation:	button::Button::new(crate::COLOR_GREEN, Rect::new(crate::H_MARGIN as i32, (crate::V_MARGIN + GRID_BOTTOM_MARGIN + crate::GRID_HEIGHT) as i32, BTN_WIDTH, BTN_HEIGHT), "Start".to_string()),
			btn_pause_resume_simulation:	button::Button::new(crate::COLOR_YELLOW,Rect::new(crate::H_MARGIN as i32, (crate::V_MARGIN + GRID_BOTTOM_MARGIN + crate::GRID_HEIGHT) as i32, BTN_WIDTH, BTN_HEIGHT), "Pause".to_string()),
			btn_abort_simulation:	button::Button::new(crate::COLOR_RED,	Rect::new((crate::H_MARGIN + BTN_WIDTH+BTN_H_MARGIN) as i32, (crate::V_MARGIN + GRID_BOTTOM_MARGIN + crate::GRID_HEIGHT) as i32, BTN_WIDTH, BTN_HEIGHT), "Abort".to_string()),
			btn_abort_n_save_simulation:	button::Button::new(crate::COLOR_RED,	Rect::new((crate::H_MARGIN + (BTN_WIDTH+BTN_H_MARGIN)*2) as i32, (crate::V_MARGIN + GRID_BOTTOM_MARGIN + crate::GRID_HEIGHT) as i32, BTN_ABORT_N_SAVE_WIDTH, BTN_HEIGHT), "Abort and save state".to_string()),
			btn_clear_generation:	button::Button::new(crate::COLOR_BLUE,	Rect::new((crate::H_MARGIN + crate::GRID_WIDTH - BTN_CLEAR_WIDTH) as i32, (crate::V_MARGIN + GRID_BOTTOM_MARGIN + crate::GRID_HEIGHT) as i32, BTN_CLEAR_WIDTH, BTN_HEIGHT), "Clear population".to_string()),

			btn_tool_pencil: 	button_icon::ButtonIcon::new(Rect::new((crate::H_MARGIN + crate::GRID_WIDTH - BTN_CLEAR_WIDTH - BTN_H_MARGIN - BTN_SQUARE_SIZE*3 - BTN_SQUARE_H_MARGIN*2) as i32, (crate::V_MARGIN + GRID_BOTTOM_MARGIN + crate::GRID_HEIGHT) as i32, BTN_SQUARE_SIZE, BTN_SQUARE_SIZE), "./icons/pencil.bmp".to_string()),
			btn_tool_eraser:	button_icon::ButtonIcon::new(Rect::new((crate::H_MARGIN + crate::GRID_WIDTH - BTN_CLEAR_WIDTH - BTN_H_MARGIN - BTN_SQUARE_SIZE*2 - BTN_SQUARE_H_MARGIN) as i32, (crate::V_MARGIN + GRID_BOTTOM_MARGIN + crate::GRID_HEIGHT) as i32, BTN_SQUARE_SIZE, BTN_SQUARE_SIZE), "./icons/eraser.bmp".to_string()),
			btn_tool_hand:		button_icon::ButtonIcon::new(Rect::new((crate::H_MARGIN + crate::GRID_WIDTH - BTN_CLEAR_WIDTH - BTN_H_MARGIN - BTN_SQUARE_SIZE) as i32, (crate::V_MARGIN + GRID_BOTTOM_MARGIN + crate::GRID_HEIGHT) as i32, BTN_SQUARE_SIZE, BTN_SQUARE_SIZE), "./icons/hand.bmp".to_string()),

			dragging: false,
			dragging_start: (-1, -1),

			main_font: None
		}
	}
	
	pub fn initialize(&mut self, font: Font<'s, 'static>) {
		self.set_state(State::IDLE);
		self.set_tool(Tool::PENCIL);

		self.main_font = Some(font);
		self.load_start_generation();
	}

	pub fn handle_events<'h>(&mut self, event_iterator: sdl2::event::EventPollIterator<'h>) -> bool {
		for event in event_iterator {
			match event {
				Event::Quit { .. } => return true,
				Event::MouseButtonUp { .. } => {
					self.dragging = false
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
	
							if self.state != State::ITERATING { // Allow draw on Idle or Pause states
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
								self.set_state(State::ITERATING);
								self.previous_generation = self.generation.clone();
								self.btn_pause_resume_simulation.set_text("Pause".to_string());
								self.set_tool(Tool::HAND);
							} else if self.btn_pause_resume_simulation.is_hovered() {	
								if self.state == State::PAUSE {
									self.set_state(State::ITERATING);
									self.btn_pause_resume_simulation.set_text("Pause".to_string());
									self.set_tool(Tool::HAND);
								} else if self.state == State::ITERATING {
									self.set_state(State::PAUSE);
									self.btn_pause_resume_simulation.set_text("Resume".to_string());
									self.set_tool(Tool::PENCIL);
								}
							} else if self.btn_abort_simulation.is_hovered() {
								self.set_state(State::IDLE);
	
								self.generation = self.previous_generation.clone();
								self.generation_number = 0;
								self.set_tool(Tool::PENCIL);
							} else if self.btn_abort_n_save_simulation.is_hovered() {
								self.set_state(State::IDLE);
								self.generation_number = 0;
								self.set_tool(Tool::PENCIL);
							} else if self.btn_clear_generation.is_hovered() {
								self.generation = vec![vec![false; crate::COLS as usize]; crate::ROWS as usize];
							} else if self.btn_tool_pencil.is_hovered() {
								self.set_tool(Tool::PENCIL);
							} else if self.btn_tool_eraser.is_hovered() {
								self.set_tool(Tool::ERASER);
							} else if self.btn_tool_hand.is_hovered() {
								self.set_tool(Tool::HAND);
							}
							
							if self.state != State::ITERATING {
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
							self.set_tool(Tool::HAND);
							
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
					if self.state != State::ITERATING && mousestate.is_mouse_button_pressed(MouseButton::Left) {
						let (indexes, clicked_canvas) = get_click_indexes(x, y);
						if clicked_canvas && self.active_tool != Tool::HAND {
							self.generation[(self.top_left_row as i32 + indexes.0) as usize][(self.top_left_col as i32 + indexes.1)  as usize] = self.active_tool == Tool::PENCIL;
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

							if difference_y.abs() as u32 > crate::SIZE {
								if difference_y > 0 {
									if self.top_left_row + move_units < crate::ROWS-crate::VIEW_ROWS {
										self.top_left_row += move_units;
									}
								} else if difference_y < 0 {
									if self.top_left_row >= move_units {
										self.top_left_row -= move_units;
									}
								}
								new_y = y;
							}

							if difference_x.abs() as u32 > crate::SIZE {
								if difference_x > 0 {
									if self.top_left_col + move_units < crate::COLS-crate::VIEW_COLS {
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
					self.update_btn_hovers(x, y);
				}
				_ => {}
			}
		}
		return false;
	}
	
	pub fn load_start_generation(&mut self){
		
		// GLIDER GUN START
		self.generation[44][48] = true;

		self.generation[45][46] = true;
		self.generation[45][48] = true;

		self.generation[46][36] = true;
		self.generation[46][37] = true;
		self.generation[46][44] = true;
		self.generation[46][45] = true;
		self.generation[46][58] = true;
		self.generation[46][59] = true;

		self.generation[47][35] = true;
		self.generation[47][39] = true;
		self.generation[47][44] = true;
		self.generation[47][45] = true;
		self.generation[47][58] = true;
		self.generation[47][59] = true;
		
		self.generation[48][24] = true;
		self.generation[48][25] = true;
		self.generation[48][34] = true;
		self.generation[48][40] = true;
		self.generation[48][44] = true;
		self.generation[48][45] = true;

		self.generation[49][34] = true;
		self.generation[49][38] = true;
		self.generation[49][40] = true;
		self.generation[49][41] = true;
		self.generation[49][46] = true;
		self.generation[49][48] = true;
		self.generation[49][24] = true;
		self.generation[49][25] = true;
		
		self.generation[50][34] = true;
		self.generation[50][40] = true;
		self.generation[50][48] = true;
		
		self.generation[51][35] = true;
		self.generation[51][39] = true;
		
		self.generation[52][36] = true;
		self.generation[52][37] = true;
		// GLIDER GUN END

		// LWSS START
		self.generation[67][51] = true;
		self.generation[67][52] = true;
		
		self.generation[68][50] = true;
		self.generation[68][51] = true;
		self.generation[68][53] = true;
		self.generation[68][54] = true;
		
		self.generation[69][51] = true;
		self.generation[69][52] = true;
		self.generation[69][53] = true;
		self.generation[69][54] = true;
		
		self.generation[70][52] = true;
		self.generation[70][53] = true;
		// LWSS END
	}

	pub fn iteration(&mut self) {
		// Clear the window
		self.canvas.set_draw_color(Color::BLACK);
		self.canvas.clear();
		
		// Draw population & information text
		{
			let surface = self.main_font.as_ref().unwrap().render(format!("Generation: {} :: Population: {}", self.generation_number, self.population_amount).as_str())
				.blended(Color::WHITE).unwrap();

			let surface_2 = self.main_font.as_ref().unwrap().render(format!("[row:{}, col:{}]", self.top_left_row, self.top_left_col).as_str())
				.blended(crate::COLOR_WHITE).unwrap();

			let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();
			let texture_2 = self.texture_creator.create_texture_from_surface(&surface_2).unwrap();
	
			let TextureQuery { width, height, .. } = texture.query();
	
			let _ = self.canvas.copy(&texture, None, Some(Rect::new(crate::H_MARGIN as i32, (crate::V_MARGIN - height - 5) as i32  , width, height)));
			

			let TextureQuery { width, height, .. } = texture_2.query();
	
			let _ = self.canvas.copy(&texture_2, None, Some(Rect::new((crate::H_MARGIN+crate::GRID_WIDTH - width) as i32, (crate::V_MARGIN - height - 5) as i32  , width, height)));
		}
		
		// Update population amount
		self.population_amount = self.draw_current_generation();

		// Iterate the generation and count
		if self.state == State::ITERATING && self.last_iteration.elapsed() > crate::ITERATION_COOLDOWN {
			self.generation = iterate_generation(&self.generation);
			self.generation_number += 1;
			self.last_iteration = std::time::Instant::now();
		}

		// Draw UI
		self.draw_lines();
		self.draw_buttons();

		self.canvas.present();
	}

	fn set_state(&mut self, new_state: State) {
		self.state = new_state;

		if new_state == State::IDLE {
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
	}

	fn set_tool(&mut self, new_tool: Tool) {
		self.btn_tool_pencil.set_active(Tool::PENCIL == new_tool);
		self.btn_tool_eraser.set_active(Tool::ERASER == new_tool);
		self.btn_tool_hand.set_active(Tool::HAND == new_tool);
		
		self.active_tool = new_tool;
	}

	fn update_btn_hovers(&mut self, x: i32, y: i32) {
		// Text buttons
		self.btn_start_simulation.update_hover(x, y);
		self.btn_pause_resume_simulation.update_hover(x, y);
		self.btn_abort_simulation.update_hover(x, y);
		self.btn_abort_n_save_simulation.update_hover(x, y);
		self.btn_clear_generation.update_hover(x, y);

		// Icon buttons
		self.btn_tool_pencil.update_hover(x, y);
		self.btn_tool_eraser.update_hover(x, y);
		self.btn_tool_hand.update_hover(x, y);
	}

	fn draw_current_generation(&mut self) -> u32 {
		let mut population = 0;
		for row in 0..crate::VIEW_ROWS {
			for col in 0..crate::VIEW_COLS {
				if self.generation[(self.top_left_row + row) as usize][(self.top_left_col + col) as usize] == true {
					self.canvas.set_draw_color(crate::COLOR_WHITE);
					let drawing_rect = Rect::new((crate::H_MARGIN + col * crate::SIZE) as i32, (crate::V_MARGIN + row * crate::SIZE) as i32, crate::SIZE, crate::SIZE);
					let _ = self.canvas.fill_rect(drawing_rect);
					population += 1;
				} else {
					if (((self.top_left_row+row)/crate::GRID_BIG_CELL_SIZE)%2 == 0 && ((self.top_left_col+col)/crate::GRID_BIG_CELL_SIZE)%2 == 0) || (((self.top_left_row+row)/crate::GRID_BIG_CELL_SIZE)%2 != 0 && ((self.top_left_col+col)/crate::GRID_BIG_CELL_SIZE)%2 != 0 ) {
						self.canvas.set_draw_color(crate::COLOR_BLACK_1);
						let drawing_rect = Rect::new((crate::H_MARGIN + col * crate::SIZE) as i32, (crate::V_MARGIN + row * crate::SIZE) as i32, crate::SIZE, crate::SIZE);
						let _ = self.canvas.fill_rect(drawing_rect);
					}
				}
			}
		}
		return population;
	}

	fn draw_lines(&mut self) {
		// Main grid lines
		self.canvas.set_draw_color(crate::COLOR_BLACK_2);
		for row in 1..crate::VIEW_COLS {
			let start_point = Point::new((crate::H_MARGIN + crate::SIZE * row) as i32, crate::V_MARGIN as i32);
			let end_point = Point::new((crate::H_MARGIN + crate::SIZE * row) as i32, (crate::V_MARGIN + (crate::VIEW_ROWS * crate::SIZE)-1) as i32);
			let _ = self.canvas.draw_line(start_point, end_point);
		}
		for row in 1..crate::VIEW_ROWS {
			let start_point = Point::new(crate::H_MARGIN as i32, (crate::V_MARGIN + crate::SIZE * row) as i32);
			let end_point = Point::new((crate::H_MARGIN + (crate::VIEW_COLS * crate::SIZE)-1) as i32, (crate::V_MARGIN + crate::SIZE * row) as i32);
			let _ = self.canvas.draw_line(start_point, end_point);
		}
		
		// Outer grid lines
		self.canvas.set_draw_color(crate::COLOR_BLACK_3);
		let _ = self.canvas.draw_rect(Rect::new(crate::H_MARGIN as i32, crate::V_MARGIN as i32, crate::VIEW_COLS * crate::SIZE, crate::VIEW_ROWS * crate::SIZE));
	}

	fn draw_buttons(&mut self) {
		// Text buttons
		self.btn_start_simulation.draw(&mut self.canvas, self.main_font.as_ref().unwrap());
		self.btn_pause_resume_simulation.draw(&mut self.canvas, self.main_font.as_ref().unwrap());
		self.btn_clear_generation.draw(&mut self.canvas, self.main_font.as_ref().unwrap());
		self.btn_abort_simulation.draw(&mut self.canvas, self.main_font.as_ref().unwrap());
		self.btn_abort_n_save_simulation.draw(&mut self.canvas, self.main_font.as_ref().unwrap());
		
		// Icon buttons
		self.btn_tool_pencil.draw(&mut self.canvas);
		self.btn_tool_eraser.draw(&mut self.canvas);
		self.btn_tool_hand.draw(&mut self.canvas);
	}
}

fn get_click_indexes(x: i32, y: i32) -> ((i32, i32), bool) {
	let row = (((y - crate::V_MARGIN as i32) as f32) / (crate::SIZE as f32)).floor() as i32;
	let col = (((x - crate::H_MARGIN as i32) as f32) / (crate::SIZE as f32)).floor() as i32;

	if row >= 0 && row < crate::VIEW_ROWS as i32 && col >= 0 && col < crate::VIEW_COLS as i32 {
		return ((row, col), true);
	}
	return ((-1, -1), false);
}



fn iterate_generation(generation : &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut new_generation : Vec<Vec<bool>> = generation.clone();

	new_generation.par_iter_mut().enumerate().for_each(|(i, row)| {
		row.par_iter_mut().enumerate().for_each(|(j,cell)| {
			let neighbors = get_neighbors(generation, i as i32, j as i32);
			if generation[i as usize][j as usize] == true { // Alive
				if !(neighbors == 2 || neighbors == 3) {
					*cell = false;
				}
			} else { // Dead
				if neighbors == 3 {
					*cell = true;
				}
			}
		});
	});

	return new_generation;
}

fn get_neighbors(generation : &Vec<Vec<bool>>, target_i : i32, target_j : i32) -> i8 {
	let mut neighbors  = 0 as i8;

	let search_i_from = i32::max(0, target_i - 1);
	let search_i_to = i32::min(crate::ROWS as i32, target_i + 2);
	let search_j_from = i32::max(0, target_j - 1);
	let search_j_to = i32::min(crate::COLS as i32, target_j + 2);

	for row in search_i_from..search_i_to {
		for col in search_j_from..search_j_to {
			if row != target_i || col != target_j {
				if generation[row as usize][col as usize] == true {
					neighbors += 1;
				}
			}
		}
	}
	
	return neighbors;
}