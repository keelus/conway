extern crate sdl2;


use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;


const BTN_ALPHA : u8 = 200;

pub struct Button {
	color: Color,
	hover_color: Color,
	rect: Rect,
	hovered: bool,
	text: String,
	hidden: bool
}

impl Button {
	pub fn new(color: Color, rect: Rect, text: String) -> Self {

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

	pub fn update_color(&mut self, new_color: Color) {
		self.color = new_color;
		self.hover_color = Color::RGBA(new_color.r, new_color.g, new_color.b, BTN_ALPHA);
	}

	pub fn draw(&self, canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, font: &mut Font) {
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

	pub fn is_hovered(&self) -> bool {
		return !self.hidden && self.hovered;
	}

	pub fn set_hidden(&mut self, new_hidden: bool) {
		self.hidden = new_hidden;
	}

	pub fn set_text(&mut self, new_text: String) {
		self.text = new_text;
	}

	pub fn update_hover(&mut self, x : i32, y : i32) {
		self.hovered = self.rect.contains_point(Point::new(x, y))
	}
}