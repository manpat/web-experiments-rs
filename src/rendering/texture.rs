#![allow(dead_code)]

use rendering::gl;
use rendering::types::*;

use std::ops::Drop;

pub struct Texture {
	pub gl_handle: u32,
	pub size: Vec2i,
}

impl Texture {
	pub fn new() -> Self {
		let mut gl_handle = 0;

		unsafe {
			gl::GenTextures(1, &mut gl_handle);
			let _bind_guard = TextureBindGuard::new_raw(gl_handle);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
		}

		Texture { gl_handle, size: Vec2i::zero() }
	}

	pub fn bind_guard(&self) -> TextureBindGuard {
		TextureBindGuard::new_raw(self.gl_handle)
	}

	pub fn from_1d(data: &[Color]) -> Self {
		let mut tex = Texture::new();
		tex.upload_1d(data);
		tex
	}

	// pub fn from_png(data: &[u8]) -> Self {
	// 	use png::{Decoder, Reader};

	// 	let decoder = Decoder::new(data);
	// 	let (info, mut reader) = decoder.read_info().unwrap();

	// 	assert!(info.width.is_power_of_two(), "Textures must be POW2");
	// 	assert!(info.height.is_power_of_two(), "Textures must be POW2");

	// 	let mut buf = vec![0; info.buffer_size()];
	// 	reader.next_frame(&mut buf).unwrap();

	// 	let mut tex = Texture::new();
	// 	let _bind_guard = TextureBindGuard::new(&tex);

	// 	tex.size = Vec2i::new(info.width as i32, info.height as i32);

	// 	unsafe {
	// 		gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32,
	// 			tex.size.x, tex.size.y, 0, gl::RGBA,
	// 			gl::UNSIGNED_BYTE, buf.as_ptr() as *const _);
	// 	}

	// 	tex
	// }

	fn get_bound_id() -> u32 {
		unsafe {
			let mut id = 0i32;
			gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut id as *mut _);
			id as u32
		}
	}

	pub fn unbind() {
		unsafe{ gl::BindTexture(gl::TEXTURE_2D, 0); }
	}

	pub fn bind_to_slot(&self, slot: u32) {
		unsafe {
			gl::ActiveTexture(gl::TEXTURE0 + slot);
			gl::BindTexture(gl::TEXTURE_2D, self.gl_handle);
		}
	}

	pub fn upload_1d(&mut self, data: &[Color]) {
		unsafe {
			let len = data.len() as u32;
			assert!(len.is_power_of_two(), "Textures must be POW2");

			self.size = Vec2i::new(data.len() as i32, 1);

			let mut v = Vec::with_capacity(data.len() * 4);
			for c in data.iter() {
				let (r,g,b,a) = c.to_byte_tuple();

				v.push(r);
				v.push(g);
				v.push(b);
				v.push(a);
			}

			let _bind_guard = TextureBindGuard::new(self);
			gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.size.x, 1, 0, gl::RGBA, gl::UNSIGNED_BYTE, v.as_ptr() as *const _);
		}
	}

	pub fn upload_2d(&mut self, data: &[Color], size: Vec2i) {
		unsafe {
			let len = data.len() as i32;
			assert!((size.x as u32).is_power_of_two(), "Textures must be POW2");
			assert!((size.y as u32).is_power_of_two(), "Textures must be POW2");
			assert!(len >= size.x*size.y, "Passed slice not large enough");

			self.size = size;

			let mut v = Vec::with_capacity(data.len() * 4);
			for c in data.iter() {
				let (r,g,b,a) = c.to_byte_tuple();

				v.push(r);
				v.push(g);
				v.push(b);
				v.push(a);
			}

			let _bind_guard = TextureBindGuard::new(self);
			gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.size.x, self.size.y, 0,
				gl::RGBA, gl::UNSIGNED_BYTE, v.as_ptr() as *const _);
		}
	}

	pub fn linear(&mut self) {
		unsafe {
			let _bind_guard = TextureBindGuard::new(self);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
		}
	}

	pub fn nearest(&mut self) {
		unsafe {
			let _bind_guard = TextureBindGuard::new(self);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
		}
	}
}

pub struct TextureBindGuard {
	prev_binding: Option<u32>,
}

impl TextureBindGuard {
	pub fn new_raw(new_binding: u32) -> Self {
		let prev_binding = Texture::get_bound_id();

		if prev_binding != new_binding {
			unsafe{ gl::BindTexture(gl::TEXTURE_2D, new_binding); }
			TextureBindGuard{ prev_binding: Some(prev_binding) }
		} else {
			TextureBindGuard { prev_binding: None }
		}
	}

	pub fn new(tex: &Texture) -> Self {
		TextureBindGuard::new_raw(tex.gl_handle)
	}
}

impl Drop for TextureBindGuard {
	fn drop(&mut self) {
		if let Some(prev_binding) = self.prev_binding {
			unsafe{ gl::BindTexture(gl::TEXTURE_2D, prev_binding); }
		}
	}
}