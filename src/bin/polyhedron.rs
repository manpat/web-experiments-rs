#![feature(generators)]

extern crate experiments;
use experiments::*;
use experiments::rendering::*;

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
		let webgl = WebGLContext::new(false);

		let mut events = Vec::new();

		unsafe {
			events::initialise_ems_event_queue(&mut events);

			gl::Enable(gl::BLEND);
			gl::BlendEquation(gl::FUNC_ADD);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

			webgl.set_background(Color::hsv(290.0, 0.6, 0.15));
		}

		let shader = ShaderBuilder::new()
			.use_3d()
			.use_proj()
			.use_view()
			.output("vec4(1.0)")

			.finalize()
			.unwrap();

		let mesh = generate_solid_mesh();

		let mut time = 0.0f32;
		let mut screen_size = Vec2i::zero();
		let mut projection = Mat4::ident();

		loop {
			for e in events.iter() {
				match *e {
					Event::Resize(sz) => unsafe {
						screen_size = sz;
						gl::Viewport(0, 0, sz.x, sz.y);

						let aspect = sz.x as f32 / sz.y as f32;
						projection = Mat4::perspective(PI / 2.0, aspect, 0.1, 100.0);
					}

					Event::Down(_) => {
					}

					_ => {}
				}
			}

			events.clear();

			time += 1.0/60.0;

			webgl.clear_all();
			webgl.set_viewport(screen_size);

			let generate_rotation = |time: f32| {
				let alpha = time * PI / 4.0 + (time / 5.0).cos() * (time / 2.0).sin() * PI / 3.0;
				let beta = time * PI / 5.0 + (time / 2.0).cos() * (time / 11.0).sin() * PI / 4.0;
				let zeta = time * PI / 11.0 + (time / 3.0).cos() * (time / 7.0).cos() * PI / 7.0;

				  Mat4::zrot(zeta)
				* Mat4::xrot(beta)
				* Mat4::yrot(alpha)
			};

			let view_mat = Mat4::translate(Vec3::from_z(-2.0))
				* generate_rotation(time);

			let scaled_view_mat = view_mat * Mat4::scale(Vec3::splat(0.9));

			shader.use_program();
			shader.set_proj(&projection);

			mesh.bind();

			unsafe {
				gl::Enable(gl::STENCIL_TEST);

				gl::ColorMask(0, 0, 0, 0);
				gl::DepthMask(0);
				gl::StencilFunc(gl::ALWAYS, 0x1, 0xff);
				gl::StencilOp(gl::ZERO, gl::ZERO, gl::REPLACE);
			}
			
			shader.set_view(&view_mat);
			mesh.draw(gl::TRIANGLES);

			unsafe {
				gl::StencilFunc(gl::NOTEQUAL, 0x0, 0xff);
				gl::StencilOp(gl::KEEP, gl::KEEP, gl::INCR);
			}

			shader.set_view(&scaled_view_mat);
			mesh.draw(gl::TRIANGLES);

			unsafe {
				gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
				gl::ColorMask(1, 1, 1, 1);
				gl::DepthMask(1);

				gl::StencilFunc(gl::EQUAL, 0x1, 0xff);
			}
			
			shader.set_view(&view_mat);
			mesh.draw(gl::TRIANGLES);

			unsafe {
				gl::StencilFunc(gl::LESS, 0x1, 0xff);
			}

			let view_mat = Mat4::translate(Vec3::from_z(-40.0))
				* generate_rotation(time + 0.3)
				* Mat4::scale(Vec3::splat(10.0));
			
			shader.set_view(&view_mat);
			mesh.draw(gl::TRIANGLES);

			yield;
		}
	});
}


fn generate_solid_mesh() -> Mesh {
	let mut mb = MeshBuilder::<DefaultVertex>::new();

	mb.add_tri_fan(&[
		DefaultVertex::new(Vec3::new( 0.0, 1.7, 0.0)),
		DefaultVertex::new(Vec3::new(-1.0, 0.0,-1.0)),
		DefaultVertex::new(Vec3::new(-1.0, 0.0, 1.0)),
		DefaultVertex::new(Vec3::new( 1.0, 0.0, 1.0)),
		DefaultVertex::new(Vec3::new( 1.0, 0.0,-1.0)),
		DefaultVertex::new(Vec3::new(-1.0, 0.0,-1.0)),
	]);

	mb.add_tri_fan(&[
		DefaultVertex::new(Vec3::new( 0.0,-1.7, 0.0)),
		DefaultVertex::new(Vec3::new(-1.0, 0.0,-1.0)),
		DefaultVertex::new(Vec3::new( 1.0, 0.0,-1.0)),
		DefaultVertex::new(Vec3::new( 1.0, 0.0, 1.0)),
		DefaultVertex::new(Vec3::new(-1.0, 0.0, 1.0)),
		DefaultVertex::new(Vec3::new(-1.0, 0.0,-1.0)),
	]);

	mb.into()
}