extern crate sdl2;

mod button;
mod button_icon;

use core::fmt;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::libc::exit;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;




const ITERATION_COOLDOWN : Duration = std::time::Duration::from_millis(200);

const COLS: u32 = 100;
const ROWS: u32 = 100;
const SIZE: u32 = 10;

const VIEW_COLS : u32 = 80;
const VIEW_ROWS : u32 = 60;

const GRID_BIG_CELL_SIZE : u32 = 5;

const H_MARGIN : u32 = 20;
const V_MARGIN : u32 = 40;
const TOOLBAR_HEIGHT : u32 = 30;

const TOTAL_WIDTH: u32 = VIEW_COLS * SIZE + H_MARGIN * 2;
const TOTAL_HEIGHT: u32 = VIEW_ROWS * SIZE + V_MARGIN * 2 + TOOLBAR_HEIGHT;



const COLOR_GREEN: Color = Color::RGB(87, 171, 90);
const COLOR_YELLOW: Color = Color::RGB(218, 170, 63);
const COLOR_RED: Color = Color::RGB(229, 83, 75);
const COLOR_BLUE: Color = Color::RGB(82, 155, 245);


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

pub fn main() {
	if COLS < VIEW_COLS || ROWS < VIEW_ROWS {
		println!("[ERROR] Total population rows x cols should be greater than shown rows x cols");
		return;
	}

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

	let mut active_tool : Tool = Tool::PENCIL;


	let mut top_left_col : u32 = (COLS - VIEW_COLS)/2;
	let mut top_left_row : u32 = (ROWS - VIEW_ROWS)/2;

	let mut iterating_generation = false;
	let mut generation = vec![vec![false; COLS as usize]; ROWS as usize];
	let mut previous_generation = vec![vec![false; COLS as usize]; ROWS as usize];
	let mut generation_number = 0;
	let mut population_amount = 0;
	let mut last_interation = std::time::Instant::now();

	let mut btn_start_simulation 		= button::Button::new(COLOR_GREEN, Rect::new(H_MARGIN as i32, (V_MARGIN + 5 + VIEW_ROWS*SIZE) as i32, 70, 30), "Start".to_string());
	let mut btn_pause_resume_simulation = button::Button::new(COLOR_YELLOW, Rect::new(H_MARGIN as i32, (V_MARGIN + 5 + VIEW_ROWS*SIZE) as i32, 70, 30), "Pause".to_string());
	let mut btn_abort_simulation 		= button::Button::new(COLOR_RED, Rect::new(H_MARGIN as i32 + 80, (V_MARGIN + 5 + VIEW_ROWS*SIZE) as i32, 70, 30), "Abort".to_string());
	let mut btn_abort_n_save_simulation = button::Button::new(COLOR_RED, Rect::new(H_MARGIN as i32 + 160, (V_MARGIN + 5 + VIEW_ROWS*SIZE) as i32, 190, 30), "Abort and save state".to_string());
	let mut btn_clear_generation 		= button::Button::new(COLOR_BLUE, Rect::new((H_MARGIN + VIEW_COLS*SIZE) as i32 - 150, (V_MARGIN + 5 + VIEW_ROWS*SIZE) as i32, 150, 30), "Clear population".to_string());
	btn_pause_resume_simulation.set_hidden(true);
	btn_abort_simulation.set_hidden(true);
	btn_abort_n_save_simulation.set_hidden(true);
	
	
	let mut btn_tool_pencil = button_icon::ButtonIcon::new(Rect::new((H_MARGIN + VIEW_COLS*SIZE) as i32 - 190 - 70, (V_MARGIN + 5 + VIEW_ROWS*SIZE) as i32, 30, 30), "./icons/pencil.bmp".to_string());
	let mut btn_tool_eraser = button_icon::ButtonIcon::new(Rect::new((H_MARGIN + VIEW_COLS*SIZE) as i32 - 190 - 35, (V_MARGIN + 5 + VIEW_ROWS*SIZE) as i32, 30, 30), "./icons/eraser.bmp".to_string());
	let mut btn_tool_hand 	= button_icon::ButtonIcon::new(Rect::new((H_MARGIN + VIEW_COLS*SIZE) as i32 - 190,	  (V_MARGIN + 5 + VIEW_ROWS*SIZE) as i32, 30, 30), "./icons/hand.bmp".to_string());
	btn_tool_pencil.set_active(true);


	let mut dragging = false;
	let mut dragging_start = (-1, -1);

	let mut font = ttf_context.load_font("./fonts/EnvyCodeR_bold.ttf", 15).unwrap();
	font.set_style(sdl2::ttf::FontStyle::BOLD);

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => break 'running,
				Event::MouseButtonUp { .. } => {
					if dragging {
						dragging = false
					}
				},
				Event::MouseButtonDown { x, y, mouse_btn, .. } => {
					match mouse_btn {
						MouseButton::Left => {
							if active_tool == Tool::HAND {
								let (_, clicked_canvas) = get_click_indexes(x, y);
								if clicked_canvas {
									dragging = true;
									dragging_start = (x, y);
								}
							}
	
							if !iterating_generation {
								let (indexes, clicked_canvas) = get_click_indexes(x, y);
								if clicked_canvas {
									if active_tool == Tool::PENCIL {
										generation[(top_left_row as i32 + indexes.0) as usize][(top_left_col as i32 + indexes.1)  as usize] = true;
									} else if active_tool == Tool::ERASER {
										generation[(top_left_row as i32 + indexes.0) as usize][(top_left_col as i32 + indexes.1)  as usize] = false;
									}
								}
							}
	
							if btn_start_simulation.is_hovered() {
								iterating_generation = true;
								previous_generation = generation.clone();
								btn_pause_resume_simulation.set_text("Pause".to_string());
								active_tool = Tool::HAND;
								btn_tool_hand.set_active(true);
								btn_tool_pencil.set_active(false);
								btn_tool_eraser.set_active(false);
							} else if btn_pause_resume_simulation.is_hovered() {
								iterating_generation = !iterating_generation;
	
								if iterating_generation {
									btn_pause_resume_simulation.set_text("Pause".to_string());
									active_tool = Tool::HAND;
									btn_tool_hand.set_active(true);
									btn_tool_pencil.set_active(false);
									btn_tool_eraser.set_active(false);
								} else {
									btn_pause_resume_simulation.set_text("Resume".to_string());
									active_tool = Tool::PENCIL;
									btn_tool_pencil.set_active(true);
									btn_tool_hand.set_active(false);
									btn_tool_eraser.set_active(false);
								}
	
							} else if btn_abort_simulation.is_hovered() {
								iterating_generation = false;
	
								generation = previous_generation.clone();
								generation_number = 0;
								active_tool = Tool::PENCIL;
								btn_tool_pencil.set_active(true);
								btn_tool_hand.set_active(false);
								btn_tool_eraser.set_active(false);
							} else if btn_abort_n_save_simulation.is_hovered() {
								iterating_generation = false;
	
								generation_number = 0;
								active_tool = Tool::PENCIL;
								btn_tool_pencil.set_active(true);
								btn_tool_hand.set_active(false);
								btn_tool_eraser.set_active(false);
							} else if btn_clear_generation.is_hovered() {
								generation = vec![vec![false; COLS as usize]; ROWS as usize];
							} else if btn_tool_pencil.is_hovered() {
								active_tool = Tool::PENCIL;
	
								btn_tool_pencil.set_active(true);
								btn_tool_eraser.set_active(false);
								btn_tool_hand.set_active(false);
							} else if btn_tool_eraser.is_hovered() {
								active_tool = Tool::ERASER;
								
								btn_tool_pencil.set_active(false);
								btn_tool_eraser.set_active(true);
								btn_tool_hand.set_active(false);
							} else if btn_tool_hand.is_hovered() {
								active_tool = Tool::HAND;
								
								btn_tool_pencil.set_active(false);
								btn_tool_eraser.set_active(false);
								btn_tool_hand.set_active(true);
							}
							
							if !iterating_generation {
								if generation_number == 0 {
									btn_start_simulation.set_hidden(false);
									btn_pause_resume_simulation.set_hidden(true);
									btn_abort_simulation.set_hidden(true);
									btn_abort_n_save_simulation.set_hidden(true);
								} else {
									btn_start_simulation.set_hidden(true);
									btn_pause_resume_simulation.set_hidden(false);
									btn_abort_simulation.set_hidden(false);
									btn_abort_n_save_simulation.set_hidden(false);
								}
								btn_clear_generation.set_hidden(false);
								btn_tool_pencil.set_hidden(false);
								btn_tool_eraser.set_hidden(false);
							} else {
								btn_start_simulation.set_hidden(true);
								btn_clear_generation.set_hidden(true);
								btn_pause_resume_simulation.set_hidden(false);
								btn_abort_simulation.set_hidden(false);
								btn_abort_n_save_simulation.set_hidden(false);
								btn_tool_pencil.set_hidden(true);
								btn_tool_eraser.set_hidden(true);
							}
						},
						MouseButton::Middle => {
							active_tool = Tool::HAND;
							btn_tool_hand.set_active(true);
							btn_tool_pencil.set_active(false);
							btn_tool_eraser.set_active(false);
	
							
							let (_, clicked_canvas) = get_click_indexes(x, y);
							if clicked_canvas {
								dragging = true;
								dragging_start = (x, y);
							}
						},
						_ => {}
					}
				},
				Event::MouseMotion { x, y, mousestate, ..} => {
					if !iterating_generation && mousestate.is_mouse_button_pressed(MouseButton::Left) {
						let (indexes, clicked_canvas) = get_click_indexes(x, y);
						if clicked_canvas {
							if active_tool == Tool::PENCIL {
								generation[(top_left_row as i32 + indexes.0) as usize][(top_left_col as i32 + indexes.1)  as usize] = true;
							} else if active_tool == Tool::ERASER {
								generation[(top_left_row as i32 + indexes.0) as usize][(top_left_col as i32 + indexes.1)  as usize] = false;
							}
						}
					}

					if active_tool == Tool::HAND && dragging {
						let (_, clicked_canvas) = get_click_indexes(x, y);
						if clicked_canvas {
							let dragging_end = (x, y);
		
							let difference_x = - (dragging_end.0 - dragging_start.0);
							let difference_y = - (dragging_end.1 - dragging_start.1);

							let mut new_x = dragging_start.0;
							let mut new_y = dragging_start.1;


							let move_units = 1;
							if difference_y.abs() as u32 > SIZE {
								if difference_y > 0 {
									if top_left_row + move_units < ROWS-VIEW_ROWS {
										top_left_row += move_units;
									}
								} else if difference_y < 0 {
									if top_left_row >= move_units {
										top_left_row -= move_units;
									}
								}
								new_y = y;
							}
							if difference_x.abs() as u32 > SIZE {
								if difference_x > 0 {
									if top_left_col + move_units < COLS-VIEW_COLS {
										top_left_col += move_units;
									}
								} else if difference_x < 0 {
									if top_left_col >= move_units {
										top_left_col -= move_units;
									}
								}
								new_x = x;
							}

							dragging_start = (new_x, new_y);
						}
					}

					
					btn_start_simulation.update_hover(x, y);
					btn_pause_resume_simulation.update_hover(x, y);
					btn_abort_simulation.update_hover(x, y);
					btn_abort_n_save_simulation.update_hover(x, y);
					btn_clear_generation.update_hover(x, y);

					btn_tool_pencil.update_hover(x, y);
					btn_tool_eraser.update_hover(x, y);
					btn_tool_hand.update_hover(x, y);

				}
				_ => {}
			}
		}

		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();


		
		let surface = font.render(format!("Generation: {} :: Population: {}  [current_tool={}] [i:{}, j:{}]", generation_number, population_amount, active_tool.to_string(), top_left_row, top_left_col).as_str())
			.blended(Color::RGBA(255, 255 ,255, 255)).unwrap();
		let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

		let TextureQuery { width, height, .. } = texture.query();

		let _ = canvas.copy(&texture, None, Some(Rect::new(H_MARGIN as i32, (V_MARGIN - height - 5) as i32  , width, height)));
		
		population_amount = draw_current_generation(&mut canvas, &generation, iterating_generation, top_left_row, top_left_col);

		if iterating_generation && last_interation.elapsed() > ITERATION_COOLDOWN {
			generation = iterate_generation(&generation);
			generation_number += 1;
			last_interation = std::time::Instant::now();
		}

		draw_lines(&mut canvas);
		draw_outerlines(&mut canvas);

		

		btn_start_simulation.draw(&mut canvas, &mut font);
		btn_pause_resume_simulation.draw(&mut canvas, &mut font);
		btn_clear_generation.draw(&mut canvas, &mut font);
		btn_abort_simulation.draw(&mut canvas, &mut font);
		btn_abort_n_save_simulation.draw(&mut canvas, &mut font);
		
		btn_tool_pencil.draw(&mut canvas);
		btn_tool_eraser.draw(&mut canvas);
		btn_tool_hand.draw(&mut canvas);

		canvas.present();
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