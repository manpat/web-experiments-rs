#![feature(generators)]

extern crate experiments;
use experiments::*;

use experiments::rendering::mesh_builder::*;
use experiments::rendering::framebuffer::*;

use rand::{Rng, thread_rng};

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
		kaleidoscope_shader.set_uniform_f32("u_sections", thread_rng().gen_range(4u32, 8u32) as f32);

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

		let mut elements = generate_element_list();

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

					Event::Click(_) => {
						elements = generate_element_list();

						kaleidoscope_shader.use_program();
						kaleidoscope_shader.set_uniform_f32("u_sections", thread_rng().gen_range(4u32, 8u32) as f32);
					}

					_ => {}
				}
			}

			events.clear();

			time += 1.0/60.0;

			paper.clear();

			// for i in -10..11 {
			// 	let start = i as f32 / 5.0;
			// 	let end = start + 0.3;

			// 	let pts = [
			// 		Vec2::new(start,-1.0),
			// 		Vec2::new(  end, 1.0),
			// 	];

			// 	paper.build_line(&pts, 0.05, Color::rgba(0.9, 0.7, 1.0, 0.007));				
			// }

			for el in elements.iter() {
				let phase = el.phase_coefficient * time + el.phase_offset;
				let orbit_mod = (time * el.orbit_offset_rate).sin() * el.orbit_offset_coefficient;
				let pos = Vec2::new(phase.x.sin(), phase.y.cos()) * el.orbit_coefficient * (1.0 + orbit_mod);

				paper.build_circle(pos, el.radius, el.color);
			}

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

struct Element {
	color: Color,
	radius: f32,
	orbit_coefficient: Vec2,
	orbit_offset_rate: f32,
	orbit_offset_coefficient: f32,

	phase_coefficient: Vec2,
	phase_offset: Vec2,
}

fn generate_element_list() -> Vec<Element> {
	use rand::{Rng, Rand, thread_rng};

	let mut rng = thread_rng();
	let mut els = Vec::new();

	let num_els = rng.gen_range(5, 10);

	for _ in 0..num_els {
		els.push(Element{
			color: Color::hsv(rng.gen_range(0.0, 360.0), 0.5, 1.0),
			radius: rng.gen_range(0.05, 0.25),

			orbit_coefficient: Vec2::rand(&mut rng) * 2.0 - Vec2::splat(1.0),
			orbit_offset_coefficient: rng.gen_range(0.0, 0.2),
			orbit_offset_rate: rng.gen_range(0.0, PI/2.0),

			phase_coefficient: Vec2::rand(&mut rng) * 2.0 * PI - Vec2::splat(PI),
			phase_offset: Vec2::rand(&mut rng) * 2.0 * PI - Vec2::splat(PI),
		});
	}

	els
}