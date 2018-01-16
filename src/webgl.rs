use bindings::emscripten::*;
use bindings::gl;

use common::color::*;

pub struct WebGLContext {}

impl WebGLContext {
	pub fn new() -> Self {
		use std::mem::uninitialized;

		let ems_context_handle = unsafe {
			let mut attribs = uninitialized();
			emscripten_webgl_init_context_attributes(&mut attribs);
			attribs.alpha = 0;
			attribs.stencil = 1;
			attribs.antialias = 1;
			attribs.preserveDrawingBuffer = 0;
			attribs.enableExtensionsByDefault = 0;

			emscripten_webgl_create_context(b"canvas\0".as_ptr() as _, &attribs)
		};

		match ems_context_handle {
			EMSCRIPTEN_RESULT_NOT_SUPPORTED => {
				panic!("WebGL not supported");
			}

			EMSCRIPTEN_RESULT_FAILED_NOT_DEFERRED => {
				panic!("WebGL context creation failed (FAILED_NOT_DEFERRED)");
			}

			EMSCRIPTEN_RESULT_FAILED => {
				panic!("WebGL context creation failed (FAILED)");
			}

			x if x < 0 => {
				panic!("WebGL context creation failed ({})", x);
			}

			_ => {}
		}

		if unsafe {emscripten_webgl_make_context_current(ems_context_handle) != EMSCRIPTEN_RESULT_SUCCESS} {
			panic!("Failed to make webgl context current");
		}

		unsafe {
			gl::ClearColor(0.2, 0.2, 0.2, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
			gl::FrontFace(gl::CCW);
		}

		WebGLContext {}
	}
	
	pub fn set_background<C>(&self, col: C) where C: Into<Color> {
		unsafe {
			let c = col.into();
			gl::ClearColor(c.r, c.g, c.b, c.a);
		}
	}
}