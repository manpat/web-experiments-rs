#![feature(generators)]

extern crate experiments;
use experiments::*;
use experiments::rendering::*;

use rand::{Rng, Rand, thread_rng};

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
			gl::Enable(gl::STENCIL_TEST);
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

		let mut inset_rotation = Quat::ident();
		let mut outer_rotation = Quat::ident();
		let mut outer2_rotation = Quat::ident();

		let mut inset_rotation_delta = Quat::ident();

		let mut rng = thread_rng();

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
						let impulse = Quat::new(
							Vec3::rand(&mut rng).normalize(),
							rng.gen_range(PI / 2.0, PI * 2.0 / 3.0) / 12.0);

						inset_rotation_delta = inset_rotation_delta * impulse;
					}

					_ => {}
				}
			}

			events.clear();

			const DT: f32 = 1.0/60.0;
			time += DT;

			let generate_rotation = |time: f32| {
				let alpha = 0.0 * time * PI / 4.0 + (time / 5.0).cos() * (time / 2.0).sin() * PI / 3.0;
				let beta = 0.0 * time * PI / 5.0 + (time / 2.0).cos() * (time / 11.0).sin() * PI / 4.0;
				let zeta = 0.0 * time * PI / 11.0 + (time / 3.0).cos() * (time / 7.0).cos() * PI / 7.0;

				let result = Quat::new(Vec3::from_z(1.0), zeta)
				* Quat::new(Vec3::from_x(1.0), beta)
				* Quat::new(Vec3::from_y(1.0), alpha);

				result.scale(DT * 0.01)
			};

			inset_rotation = DT.ease_linear(
				inset_rotation,
				Quat::new(Vec3::from_y(1.0), PI / 4.0 * time));

			inset_rotation_delta = inset_rotation_delta.scale(1.0 - DT / 2.0) * generate_rotation(time);

			inset_rotation = (inset_rotation * inset_rotation_delta).normalize();
			outer_rotation = (16.0 * DT).ease_linear(outer_rotation, inset_rotation).normalize();
			outer2_rotation = (16.0 * DT).ease_linear(outer2_rotation, outer_rotation).normalize();

			webgl.clear_all();
			webgl.set_viewport(screen_size);

			let translation = Mat4::translate(Vec3::from_z(-2.0));

			let outer2_mat = translation * outer2_rotation.to_mat4() * Mat4::scale(Vec3::splat(1.2));
			let outer_mat = translation * outer_rotation.to_mat4();

			let scaled_outer2_mat = outer2_mat * Mat4::scale(Vec3::splat(0.9));
			let scaled_outer_mat = outer_mat * Mat4::scale(Vec3::splat(0.9));

			let inset_view_mat = Mat4::translate(Vec3::from_z(-40.0))
				* inset_rotation.to_mat4()
				* Mat4::scale(Vec3::splat(10.0));

			shader.use_program();
			shader.set_proj(&projection);

			mesh.bind();


			// Prepare stencil buffer
			webgl.disable_color_write();
			webgl.disable_depth_write();
			
			webgl.set_stencil(StencilParams::new(0x1).always().replace());
			shader.set_view(&outer2_mat);
			mesh.draw(gl::TRIANGLES);

			webgl.set_stencil(StencilParams::new(0x1).equal().increment());
			shader.set_view(&scaled_outer2_mat);
			mesh.draw(gl::TRIANGLES);
			
			webgl.set_stencil(StencilParams::new(0x2).equal().increment());
			shader.set_view(&outer_mat);
			mesh.draw(gl::TRIANGLES);

			webgl.set_stencil(StencilParams::new(0x3).equal().increment());
			shader.set_view(&scaled_outer_mat);
			mesh.draw(gl::TRIANGLES);


			// Draw outline
			webgl.enable_color_write();
			webgl.enable_depth_write();
			
			webgl.set_stencil(StencilParams::new(0x3).equal());
			shader.set_view(&outer_mat);
			mesh.draw(gl::TRIANGLES);
			
			webgl.set_stencil(StencilParams::new(0x1).equal());
			shader.set_view(&outer2_mat);
			mesh.draw(gl::TRIANGLES);

			// Draw inset
			webgl.set_stencil(StencilParams::new(0x4).equal());
			shader.set_view(&inset_view_mat);
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