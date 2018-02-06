#![feature(generators)]

extern crate experiments;
use experiments::*;
use experiments::rendering::*;

// use rand::{Rng, thread_rng};

use events::Event;

#[derive(Copy, Clone)]
struct Vert2D(Vec2);

impl Vertex for Vert2D {
	fn get_layout() -> VertexLayout {
		VertexLayout::new::<Self>()
			.add_binding(0, 2, 0)
	}
}

fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");

	set_coro_as_main_loop(|| {
		let webgl = WebGLContext::new(false);

		let mut events = Vec::new();

		unsafe {
			events::initialise_ems_event_queue(&mut events);

			gl::Enable(gl::BLEND);
			gl::BlendEquation(gl::FUNC_ADD);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		}

		webgl.set_background(Color::grey(0.2));

		let shader = ShaderBuilder::new()
			.use_highp()
			.uniform("aspect", "float")
			.uniform("time", "float")
			.varying("pos", "vec2")
			.vertex("v_pos = position * vec2(u_aspect, 1.0)")
			.fragment(asset!("blobs.fs"))
			.finalize()
			.unwrap();

		shader.use_program();

		let quad: Mesh = {
			let mut mb = MeshBuilder::new();
			mb.add_quad(&[
				Vert2D(Vec2::new(-1.0, -1.0)),
				Vert2D(Vec2::new(-1.0,  1.0)),
				Vert2D(Vec2::new( 1.0,  1.0)),
				Vert2D(Vec2::new( 1.0, -1.0)),
			]);
			mb.into()
		};

		let mut time = 0.0f32;
		let mut screen_size = Vec2i::zero();

		loop {
			for e in events.iter() {
				match *e {
					Event::Resize(sz) => unsafe {
						screen_size = sz;
						gl::Viewport(0, 0, sz.x, sz.y);

						let aspect = sz.x as f32 / sz.y as f32;
						shader.set_uniform_f32("u_aspect", aspect);
					}

					Event::Down(_) => {}
					Event::Up(_) => {}

					_ => {}
				}
			}

			events.clear();

			time += 1.0/60.0;
			shader.set_uniform_f32("u_time", time);

			webgl.set_viewport(screen_size);

			quad.bind();
			quad.draw(gl::TRIANGLES);

			yield;
		}
	});
}
