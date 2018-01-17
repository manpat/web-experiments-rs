#![feature(generators)]

extern crate experiments;
use experiments::*;

use experiments::rendering::mesh_builder::*;
use experiments::rendering::framebuffer::*;

use events::Event;

#[derive(Copy, Clone)]
pub struct KalVert (Vec2);

impl Vertex for KalVert {
	fn get_layout() -> VertexLayout {
		VertexLayout::new::<Self>()
			.add_binding(0, 2, 0)
	}
}

fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");

	set_coro_as_main_loop(|| {
		let webgl = WebGLContext::new();

		let mut events = Vec::new();

		unsafe {
			events::initialise_ems_event_queue(&mut events);

			gl::Enable(gl::BLEND);
			gl::BlendEquation(gl::FUNC_ADD);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

			webgl.set_background(Color::grey(0.2));
		}

		let paper_shader = Shader::new(res::shaders::PAPER_VS, res::shaders::PAPER_FS);
		let kaleidoscope_shader = Shader::new(res::shaders::BASIC_TRANSFORM2_VS, res::shaders::KALEIDOSCOPE_FS);
		kaleidoscope_shader.use_program();

		let mut main_fb = FramebufferBuilder::new(Vec2i::splat(512))
			.add_target()
			.finalize();

		main_fb.bind();
		unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
		Framebuffer::unbind();

		let quad: Mesh = {
			let mut mb = MeshBuilder::new();
			mb.add_quad(&[
				KalVert(Vec2::new(-1.0, -1.0)),
				KalVert(Vec2::new(-1.0,  1.0)),
				KalVert(Vec2::new( 1.0,  1.0)),
				KalVert(Vec2::new( 1.0, -1.0)),
			]);
			mb.into()
		};

		let mut paper = Paper::new();
		let mut time = 0.0f32;

		let mut screen_size = Vec2i::zero();

		loop {
			for e in events.iter() {
				match *e {
					Event::Resize(sz) => unsafe {
						screen_size = sz;
						gl::Viewport(0, 0, sz.x, sz.y);

						let aspect = sz.x as f32 / sz.y as f32;

						kaleidoscope_shader.use_program();
						kaleidoscope_shader.set_proj(&Mat4::ident());
						kaleidoscope_shader.set_uniform_f32("u_aspect", aspect);

						paper_shader.use_program();
						paper_shader.set_proj(&Mat4::ident());
					}

					_ => {}
				}
			}

			events.clear();

			time += 1.0/60.0;

			paper.clear();

			for i in -10..11 {
				let start = i as f32 / 5.0;
				let end = start + 0.3;

				let pts = [
					Vec2::new(start,-1.0),
					Vec2::new(  end, 1.0),
				];

				paper.build_line(&pts, 0.05, Color::rgba(0.9, 0.7, 1.0, 0.007));				
			}

			paper.build_circle(Vec2::new((time*0.1).sin()*0.2, 0.0), 0.4, Color::hsv(0.0, 0.5, 1.0));
			// paper.build_circle(Vec2::new((time*0.1).sin()*0.2, 0.0), 0.4, Color::rgb(1.0, 0.5, 0.5));
			paper.build_circle(Vec2::from_angle(time*0.34 + 0.3) * (0.7 + time.cos() * 0.06), 0.1, Color::rgb(1.0, 0.8, 0.5));
			paper.build_circle(Vec2::from_angle(time*0.67 + 0.8) * (0.7 + time.cos() * 0.06), 0.1, Color::rgb(1.0, 0.5, 0.8));

			paper.build_circle(Vec2::new(time.sin() * 1.5, 0.7 * (time/2.3).cos()), 0.2, Color::rgb(0.5, 1.0, 0.5));
			paper.build_circle(Vec2::new(-(time*0.7).cos(), 0.3 * (time/2.3).sin()), 0.2, Color::rgb(0.5, 0.5, 1.0));

			paper.build_circle(Vec2::new(1.0, 0.0), 0.2, Color::rgb(0.5, 0.9, 0.9));
			paper.build_circle(Vec2::new(0.0,-1.0), 0.2, Color::rgb(0.7, 0.5, 0.9));

			main_fb.bind();
			unsafe { gl::Viewport(0, 0, 512, 512); }
			paper_shader.use_program();
			paper.draw();
			Framebuffer::unbind();

			unsafe { gl::Viewport(0, 0, screen_size.x, screen_size.y); }

			let _guard = main_fb.get_target(0).unwrap().bind_guard();
			kaleidoscope_shader.use_program();
			kaleidoscope_shader.set_uniform_i32("u_tex", 0);
			kaleidoscope_shader.set_uniform_f32("u_time", time);

			quad.bind();
			quad.draw(gl::TRIANGLES);

			yield;
		}
	});
}