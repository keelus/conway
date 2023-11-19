extern crate sdl2;


use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::surface::Surface;


pub struct ButtonIcon {
	rect: Rect,
	hovered: bool,
	hidden: bool,
	active: bool,
	icon_path: String
}

impl ButtonIcon {
	pub fn new(rect: Rect, icon_path: String) -> Self {

		Self {
			rect,
			hovered: false,
			hidden: false,
			active: false,
			icon_path
		}
	}

	pub fn draw(&self, canvas : &mut sdl2::render::Canvas<sdl2::video::Window>) {
		if !self.hidden {
			let texture_creator = canvas.texture_creator();

			
			let icon_surface = Surface::load_bmp(self.icon_path.to_string()).unwrap();
			let icon_texture = texture_creator.create_texture_from_surface(icon_surface).unwrap();

	
			canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
			let _ = canvas.set_draw_color(if self.active {Color::RGB(255, 255, 255)} else {Color::RGB(127, 127, 127)});
			let _ = canvas.fill_rect(self.rect);
			canvas.set_blend_mode(sdl2::render::BlendMode::None);
	
			let _ = canvas.copy(&icon_texture, None, Some(Rect::new(self.rect.x+2, self.rect.y+2, self.rect.w as u32 - 4, self.rect.w as u32 - 4)));
		}
	}

	pub fn is_hovered(&self) -> bool {
		return !self.hidden && self.hovered;
	}

	pub fn set_hidden(&mut self, new_hidden: bool) {
		self.hidden = new_hidden;
	}

	pub fn update_hover(&mut self, x : i32, y : i32) {
		self.hovered = self.rect.contains_point(Point::new(x, y))
	}

	pub fn set_active(&mut self, new_active: bool) {
		self.active = new_active;
	}
}