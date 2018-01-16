#![allow(dead_code)]

use math::*;
use rendering::gl;

use rendering::texture::*;

pub struct Framebuffer {
	gl_handle: u32,
	targets: Vec<Texture>,
	depth_target: Option<Texture>,
	size: Vec2i,
}

impl Framebuffer {
	pub fn bind(&self) {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.gl_handle);
		}
	}

	pub fn unbind() {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		}
	}

	pub fn get_target(&mut self, id: usize) -> Option<&mut Texture> {
		if id >= self.targets.len() { return None }

		Some(&mut self.targets[id])
	}

	pub fn get_depth(&mut self) -> Option<&mut Texture> {
		self.depth_target.as_mut()
	}

	pub fn resize(&mut self, nsize: Vec2i) {
		if self.size == nsize { return }

		unsafe {
			for tex in self.targets.iter_mut() {
				let _guard = tex.bind_guard();

				gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, nsize.x, nsize.y, 0, 
					gl::RGBA, gl::UNSIGNED_BYTE, 0 as *const _);
			}

			if let Some(ref tex) = self.depth_target {
				let _guard = tex.bind_guard();

				gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32, nsize.x, nsize.y, 0, 
					gl::DEPTH_COMPONENT, gl::UNSIGNED_INT, 0 as *const _);
			}
		}

		self.size = nsize;
	}
}

pub struct FramebufferBuilder {
	fb: Framebuffer,
}

impl FramebufferBuilder {
	pub fn new(size: Vec2i) -> Self {
		let mut fb = Framebuffer {
			gl_handle: 0, targets: Vec::new(),
			depth_target: None, size
		};

		unsafe {
			gl::GenFramebuffers(1, &mut fb.gl_handle);
			fb.bind();
		}

		FramebufferBuilder { fb }
	}

	pub fn new_unsized() -> Self {
		let mut fb = Framebuffer {
			gl_handle: 0, targets: Vec::new(),
			depth_target: None,
			size: Vec2i::splat(1)
		};

		unsafe {
			gl::GenFramebuffers(1, &mut fb.gl_handle);
			fb.bind();
		}

		FramebufferBuilder { fb }
	}

	pub fn finalize(self) -> Framebuffer {
		Framebuffer::unbind();

		self.fb
	}

	pub fn add_depth(mut self) -> Self {
		let mut gl_handle = 0;

		assert!(self.fb.depth_target.is_none(), "Framebuffer can only have one depth target");

		unsafe {
			gl::GenTextures(1, &mut gl_handle);
			let _guard = TextureBindGuard::new_raw(gl_handle);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

			gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32, self.fb.size.x, self.fb.size.y, 0, 
				gl::DEPTH_COMPONENT, gl::UNSIGNED_INT, 0 as *const _);

			gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, 
				gl::TEXTURE_2D, gl_handle, 0);
		}

		self.fb.depth_target = Some(Texture{gl_handle, size: self.fb.size});

		self
	}

	pub fn add_target(mut self) -> Self {
		let mut gl_handle = 0;

		let next_target = self.fb.targets.len() as u32;

		unsafe {
			gl::GenTextures(1, &mut gl_handle);
			let _guard = TextureBindGuard::new_raw(gl_handle);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

			gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.fb.size.x, self.fb.size.y, 0, 
				gl::RGBA, gl::UNSIGNED_BYTE, 0 as *const _);

			gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0 + next_target, gl::TEXTURE_2D, gl_handle, 0);
		}

		self.fb.targets.push(Texture{gl_handle, size: self.fb.size});

		self
	}
}