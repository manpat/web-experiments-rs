#![allow(dead_code)]

pub use math::*;
pub use easing::*;

#[derive(Copy, Clone, Debug)]
pub struct Viewport {
	pub size: Vec2i,
}

impl Viewport {
	pub fn new() -> Viewport {
		Viewport{ size: Vec2i::zero() }
	}

	pub fn get_aspect(&self) -> f32 {
		let (sw, sh) = self.size.to_tuple();
		sw as f32 / sh as f32
	}

	pub fn client_to_gl_coords(&self, pos: Vec2i) -> Vec2 {
		let (sw, sh) = self.size.to_vec2().to_tuple();
		let pos = pos.to_vec2();
		let aspect = self.get_aspect();

		let (sx, sy) = (pos.x / sw, pos.y / sh);
		Vec2::new(aspect * (sx * 2.0 - 1.0), 1.0 - sy * 2.0)
	}

	pub fn get_top_left(&self) -> Vec2 {
		self.client_to_gl_coords(Vec2i::zero())
	}

	pub fn get_bottom_left(&self) -> Vec2 {
		self.client_to_gl_coords(Vec2i::new(0, self.size.y))
	}

	pub fn get_top_right(&self) -> Vec2 {
		self.client_to_gl_coords(Vec2i::new(self.size.x, 0))
	}

	pub fn get_bottom_right(&self) -> Vec2 {
		self.client_to_gl_coords(self.size)
	}
}

