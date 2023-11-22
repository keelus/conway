extern crate sdl2;
extern crate rayon;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use sdl2::sys::_Float32;
use core::fmt;
use std::time::Instant;

use super::SUB_MATRIX_CHUNK_SIZE;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;

use crate::SUB_MATRIX_SIZE;
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
	change_matrix: Vec<Vec<bool>>,
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
	
		let generation = vec![vec![false; crate::MATRIX_SIZE as usize]; crate::MATRIX_SIZE as usize];
		let previous_generation = vec![vec![false; crate::MATRIX_SIZE as usize]; crate::MATRIX_SIZE as usize];
		let change_matrix = vec![vec![true; crate::SUB_MATRIX_SIZE as usize]; crate::SUB_MATRIX_SIZE as usize];
	
		Self {
			canvas: canvas,
			texture_creator,

			active_tool: Tool::PENCIL,

			top_left_col: 0,
			top_left_row: 20,

			state: State::IDLE,
			generation,
			previous_generation,
			change_matrix,
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
							if !self.dragging {
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
									self.bruteforce_changes();
									self.set_tool(Tool::HAND);
								} else if self.btn_pause_resume_simulation.is_hovered() {	
									if self.state == State::PAUSE {
										self.set_state(State::ITERATING);
										self.btn_pause_resume_simulation.set_text("Pause".to_string());
										self.bruteforce_changes();
										self.set_tool(Tool::HAND);
									} else if self.state == State::ITERATING {
										self.set_state(State::PAUSE);
										self.btn_pause_resume_simulation.set_text("Resume".to_string());
										self.set_tool(Tool::PENCIL);
									}
								} else if self.btn_abort_simulation.is_hovered() {
									self.set_state(State::IDLE);
									self.generation = self.previous_generation.clone();
									self.change_matrix = vec![vec![true; crate::SUB_MATRIX_SIZE as usize]; crate::SUB_MATRIX_SIZE as usize];
									self.generation_number = 0;
									self.set_tool(Tool::PENCIL);
								} else if self.btn_abort_n_save_simulation.is_hovered() {
									self.set_state(State::IDLE);
									self.change_matrix = vec![vec![true; crate::SUB_MATRIX_SIZE as usize]; crate::SUB_MATRIX_SIZE as usize];
									self.generation_number = 0;
									self.set_tool(Tool::PENCIL);
								} else if self.btn_clear_generation.is_hovered() {
									self.generation = vec![vec![false; crate::MATRIX_SIZE as usize]; crate::MATRIX_SIZE as usize];
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
							}
						},
						MouseButton::Middle => {
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
					if self.state != State::ITERATING && mousestate.is_mouse_button_pressed(MouseButton::Left) && !self.dragging {
						let (indexes, clicked_canvas) = get_click_indexes(x, y);
						if clicked_canvas && self.active_tool != Tool::HAND {
							self.generation[(self.top_left_row as i32 + indexes.0) as usize][(self.top_left_col as i32 + indexes.1)  as usize] = self.active_tool == Tool::PENCIL;
						}
					}
					if self.dragging {
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
									if self.top_left_row + move_units < crate::MATRIX_SIZE-crate::VIEW_ROWS {
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
									if self.top_left_col + move_units < crate::MATRIX_SIZE-crate::VIEW_COLS {
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
			(self.generation, self.change_matrix) = self.iterate_generation(&self.generation);
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
					if (((self.top_left_row+row)/ SUB_MATRIX_CHUNK_SIZE)%2 == 0 && ((self.top_left_col+col)/ SUB_MATRIX_CHUNK_SIZE)%2 == 0) || (((self.top_left_row+row)/ SUB_MATRIX_CHUNK_SIZE)%2 != 0 && ((self.top_left_col+col)/ SUB_MATRIX_CHUNK_SIZE)%2 != 0 ) {
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
	


	fn get_neighbors(&self, target_i : i32, target_j : i32) -> i8 {
		let mut neighbors  = 0 as i8;

		let search_i_from = i32::max(0, target_i - 1);
		let search_i_to = i32::min(crate::MATRIX_SIZE as i32, target_i + 2);
		let search_j_from = i32::max(0, target_j - 1);
		let search_j_to = i32::min(crate::MATRIX_SIZE as i32, target_j + 2);

		for row in search_i_from..search_i_to {
			for col in search_j_from..search_j_to {
				if row != target_i || col != target_j {
					if self.generation[row as usize][col as usize] == true {
						neighbors += 1;
					}
				}
			}
		}
		
		return neighbors;
	}

	fn iterate_generation(&self, generation : &Vec<Vec<bool>>) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
		let it_start = Instant::now();
		let mut new_generation : Vec<Vec<bool>> = generation.clone();
		let mut new_change_matrix : Vec<Vec<bool>> = self.change_matrix.clone();

		for sub_row in 0..SUB_MATRIX_SIZE {
			for sub_col in 0..SUB_MATRIX_SIZE {

				let previously_changed = self.change_matrix[sub_row as usize][sub_col as usize];
				let has_changed_neighbors = self.has_changed_neighbors(sub_row, sub_col);
				let iterate_sub_matrix = previously_changed || has_changed_neighbors;
				
				if iterate_sub_matrix { // Matrix has changed in last iteration
					let mut has_changed = false;

					if !previously_changed && has_changed_neighbors { // The cell had no changes, but need to check borders in case neighbor cells are moving towards this cell. This reduces iterations on aprox 75%
						
						let mut elems_top: Vec<(usize, usize)> = (0..SUB_MATRIX_CHUNK_SIZE).map(|i| ((sub_row*SUB_MATRIX_CHUNK_SIZE) as usize, (sub_col*SUB_MATRIX_CHUNK_SIZE+i) as usize)).collect(); // Top border
						let mut elems_bottom: Vec<(usize, usize)> = (0..SUB_MATRIX_CHUNK_SIZE).map(|i| (((sub_row+1)*SUB_MATRIX_CHUNK_SIZE-1) as usize, (sub_col*SUB_MATRIX_CHUNK_SIZE+i) as usize)).collect(); // Bottom border
						let mut elems_left: Vec<(usize, usize)> = (0..SUB_MATRIX_CHUNK_SIZE).map(|i| ((sub_row*SUB_MATRIX_CHUNK_SIZE+i) as usize, (sub_col*SUB_MATRIX_CHUNK_SIZE) as usize)).collect(); // Left border
						let mut elems_right: Vec<(usize, usize)> = (0..SUB_MATRIX_CHUNK_SIZE).map(|i| ((sub_row*SUB_MATRIX_CHUNK_SIZE+i) as usize, ((sub_col+1)*SUB_MATRIX_CHUNK_SIZE-1) as usize)).collect(); // Left border
	

						let mut all_elems: Vec<(usize, usize)> = Vec::new();
						all_elems.append(&mut elems_top);
						all_elems.append(&mut elems_bottom);
						all_elems.append(&mut elems_left);
						all_elems.append(&mut elems_right);

						for elem in all_elems {
							let row = elem.0;
							let col = elem.1;
							let neighbors = self.get_neighbors(row as i32, col as i32);
							if generation[row][col] == true { // Alive
								if !(neighbors == 2 || neighbors == 3) {
									new_generation[row][col] = false;
									has_changed = true;
								}
							} else { // Dead
								if neighbors == 3 {
									new_generation[row][col] = true;
									has_changed = true;
								}
							}
						}



					} else { // The cell had changes
						for row in sub_row*(SUB_MATRIX_CHUNK_SIZE)..(sub_row+1)*(SUB_MATRIX_CHUNK_SIZE) {
							for col in sub_col*(SUB_MATRIX_CHUNK_SIZE)..(sub_col+1)*(SUB_MATRIX_CHUNK_SIZE) {
								let neighbors = self.get_neighbors(row as i32, col as i32);
								if generation[row as usize][col as usize] == true { // Alive
									if !(neighbors == 2 || neighbors == 3) {
										new_generation[row as usize][col as usize] = false;
										has_changed = true;
									}
								} else { // Dead
									if neighbors == 3 {
										new_generation[row as usize][col as usize] = true;
										has_changed = true;
									}
								}
							}
						}
					}


					new_change_matrix[sub_row as usize][sub_col as usize] = has_changed;
				}
			}
		}

		if it_start.elapsed().as_secs() > 1 {
			println!("[WARNING] Iteration took {}ms [{}s]", it_start.elapsed().as_millis(), it_start.elapsed().as_millis() as _Float32 / 1000 as _Float32);
		}
		return (new_generation, new_change_matrix);
	}

	fn has_changed_neighbors(&self, i: u32, j: u32) -> bool {
		let target_i: i32 = i as i32;
		let target_j: i32 = j as i32;


	
		let search_i_from = i32::max(0, target_i - 1);
		let search_i_to = i32::min(crate::SUB_MATRIX_SIZE as i32, target_i + 2);
		let search_j_from = i32::max(0, target_j - 1);
		let search_j_to = i32::min(crate::SUB_MATRIX_SIZE as i32, target_j + 2);
	
		for row in search_i_from..search_i_to {
			for col in search_j_from..search_j_to {
				if row != target_i || col != target_j {
					if self.change_matrix[row as usize][col as usize] == true {
						return true;
					}
				}
			}
		}
		
		return false;
	}

	fn bruteforce_changes(&mut self) {
		let it_start = Instant::now();

		let mut new_matrix = vec![vec![true; crate::SUB_MATRIX_SIZE as usize]; crate::SUB_MATRIX_SIZE as usize];
		new_matrix.par_iter_mut().enumerate().for_each(|(sub_row, row_content)| {
			row_content.par_iter_mut().enumerate().for_each(|(sub_col, elem)| {
				let mut has_alive_cells = false;
				for row in sub_row*(SUB_MATRIX_CHUNK_SIZE as usize)..(sub_row+1)*(SUB_MATRIX_CHUNK_SIZE as usize) {
					for col in sub_col*(SUB_MATRIX_CHUNK_SIZE as usize)..(sub_col+1)*(SUB_MATRIX_CHUNK_SIZE as usize) {
						if self.generation[row as usize][col as usize] == true { // Alive
							has_alive_cells = true;
							break;
						}
					}
					if has_alive_cells {
						break;
					}
				}
				*elem = has_alive_cells;
			})
		});

		self.change_matrix = new_matrix;
		
		println!("[INFO] Changes took {}ms [{}s] to load", it_start.elapsed().as_millis(), it_start.elapsed().as_millis() as _Float32 / 1000 as _Float32);
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