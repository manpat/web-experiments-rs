#![feature(generators)]

extern crate experiments;
use experiments::*;
use experiments::rendering::*;

use events::Event;

#[derive(Copy, Clone)]
pub struct Vert (Vec3, Vec3);

impl Vertex for Vert {
	fn get_layout() -> VertexLayout {
		VertexLayout::new::<Self>()
			.add_binding(0, 3, 0)
			.add_binding(1, 3, 12)
	}
}

#[derive(Copy, Clone)]
pub struct Vert2D (Vec2, Vec2);

impl Vertex for Vert2D {
	fn get_layout() -> VertexLayout {
		VertexLayout::new::<Self>()
			.add_binding(0, 2, 0)
			.add_binding(1, 2, 8)
	}
}

fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");

	set_coro_as_main_loop(|| {
		let webgl = WebGLContext::new(false);

		let mut events = Vec::new();

		unsafe {
			events::initialise_ems_event_queue(&mut events);

			gl::Enable(gl::DEPTH_TEST);
			// gl::Enable(gl::CULL_FACE);
			gl::Enable(gl::BLEND);
			gl::BlendEquation(gl::FUNC_ADD);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		}

		let forward_shader = ShaderBuilder::new()
			.use_3d()
			.use_highp()
			.use_proj()
			.use_view()
			.frag_attribute("color", "vec3")
			.output("vec4(v_color*0.5 + 0.5, 1.0)")
			.finalize()
			.unwrap();

		let decal_shader = ShaderBuilder::new()
			.use_highp()
			.use_proj()
			.use_view()
			.frag_attribute("uv", "vec2")
			.uniform("normal_tex", "sampler2D")
			.uniform("decal_position", "vec2")

			.vertex("
				vec4 vert = vec4(position.x, 0.0, position.y, 1.0);
				vert.xz += u_decal_position;
				gl_Position = u_proj * u_view * vert;
			")

			.output("texture2D(u_normal_tex, v_uv)")
			// .fragment(asset!("zen_decal.fs"))
			.finalize()
			.unwrap();

		let post_shader = ShaderBuilder::new()
			.use_highp()
			.uniform("color_tex", "sampler2D")
			.uniform("normal_tex", "sampler2D")
			.uniform("normal_transform", "mat4")
			.frag_attribute("uv", "vec2")
			.fragment(include_str!("../../../assets/zen_post.fs"))
			.finalize()
			.unwrap();


		
		let mut color_fb = FramebufferBuilder::new_unsized()
			.add_depth() .add_target()
			.finalize();

		let mut normal_fb = FramebufferBuilder::new_unsized()
			.add_depth() .add_target()
			.finalize();


		let sand_color = Color::hsv(70.0, 0.3, 1.0).into();
		let border_color = Color::hsv(30.0, 0.6, 0.9).into();
		// let border_color = Color::hsv(30.0, 0.2, 0.01).into();

		let sandbox_color_mesh: Mesh = construct_sandbox_mesh(SandboxParameters {
			top:	sand_color,
			bottom:	border_color,
			left:	border_color,
			right:	border_color,
			front:	border_color,
			back:	border_color,
		});

		let sandbox_normal_mesh: Mesh = construct_sandbox_mesh(SandboxParameters {
			top:	Vec3::new( 0.0, 1.0, 0.0),
			bottom:	Vec3::new( 0.0,-1.0, 0.0),
			left:	Vec3::new(-1.0, 0.0, 0.0),
			right:	Vec3::new( 1.0, 0.0, 0.0),
			front:	Vec3::new( 0.0, 0.0,-1.0),
			back:	Vec3::new( 0.0, 0.0, 1.0),
		});

		let quad: Mesh = {
			let mut mb = MeshBuilder::new();
			mb.add_quad(&[
				Vert2D(Vec2::new(-1.0, -1.0), Vec2::new( 0.0,  0.0)),
				Vert2D(Vec2::new( 1.0, -1.0), Vec2::new( 1.0,  0.0)),
				Vert2D(Vec2::new( 1.0,  1.0), Vec2::new( 1.0,  1.0)),
				Vert2D(Vec2::new(-1.0,  1.0), Vec2::new( 0.0,  1.0)),
			]);
			mb.into()
		};

		let decal_tex = generate_decal_texture();
		let decal_quad: Mesh = {
			let mut mb = MeshBuilder::new();
			let decal_size = 0.3;
			mb.add_quad(&[
				Vert2D(Vec2::new(-decal_size, -decal_size), Vec2::new( 0.0,  0.0)),
				Vert2D(Vec2::new( decal_size, -decal_size), Vec2::new( 1.0,  0.0)),
				Vert2D(Vec2::new( decal_size,  decal_size), Vec2::new( 1.0,  1.0)),
				Vert2D(Vec2::new(-decal_size,  decal_size), Vec2::new( 0.0,  1.0)),
			]);
			mb.into()
		};

		let mut time = 0.0f32;
		let mut screen_size = Vec2i::zero();

		let mut proj_mat = Mat4::ident();

		loop {
			for e in events.iter() {
				match *e {
					Event::Resize(sz) => unsafe {
						screen_size = sz;
						gl::Viewport(0, 0, sz.x, sz.y);

						let aspect = sz.x as f32 / sz.y as f32;

						proj_mat = Mat4::perspective(PI/4.0, aspect, 0.1, 100.0);

						color_fb.resize(screen_size);
						normal_fb.resize(screen_size);
					}

					_ => {}
				}
			}

			events.clear();

			time += 1.0/60.0;

			let rotation_mat = Mat4::xrot(PI/8.0)
				* Mat4::yrot(PI/6.0 + time);

			let view_mat = Mat4::translate(Vec3::new(0.0, 0.0, -1.0))
				* rotation_mat;

			forward_shader.use_program();
			forward_shader.set_view(&view_mat);
			forward_shader.set_proj(&proj_mat);


			color_fb.bind();
			color_fb.update_viewport(&webgl);
			webgl.set_background(Color::hsva(210.0, 0.3, 1.0, 0.0));
			webgl.clear_all();

			sandbox_color_mesh.bind();
			sandbox_color_mesh.draw(gl::TRIANGLES);


			normal_fb.bind();
			normal_fb.update_viewport(&webgl);
			webgl.set_background(Color::grey_a(0.0, 0.0));
			webgl.clear_all();

			sandbox_normal_mesh.bind();
			sandbox_normal_mesh.draw(gl::TRIANGLES);

			unsafe { gl::DepthFunc(gl::ALWAYS); }

			decal_tex.bind_to_slot(0);
			decal_shader.use_program();
			decal_shader.set_view(&view_mat);
			decal_shader.set_proj(&proj_mat);
			decal_shader.set_uniform_i32("u_normal_tex", 0);
			decal_quad.bind();

			let decal_positions = [
				Vec2::new(0.0, 0.0),
				Vec2::new(0.3, 0.1),
				Vec2::new(0.4,-0.1),
			];

			for &pos in decal_positions.iter() {
				decal_shader.set_uniform_vec2("u_decal_position", pos);
				decal_quad.draw(gl::TRIANGLES);
			}

			unsafe { gl::DepthFunc(gl::LESS); }

			Framebuffer::unbind();


			webgl.clear_all();
			webgl.set_viewport(screen_size);

			color_fb.get_target(0).unwrap().bind_to_slot(0);
			normal_fb.get_target(0).unwrap().bind_to_slot(1);

			post_shader.use_program();
			post_shader.set_uniform_i32("u_color_tex", 0);
			post_shader.set_uniform_i32("u_normal_tex", 1);
			post_shader.set_uniform_mat("u_normal_transform", &rotation_mat);

			quad.bind();
			quad.draw(gl::TRIANGLES);

			yield;
		}
	});
}


struct SandboxParameters {
	top:	Vec3,
	bottom:	Vec3,
	left:	Vec3,
	right:	Vec3,
	front:	Vec3, // into screen
	back:	Vec3, // out of screen
}

fn construct_sandbox_mesh(params: SandboxParameters) -> Mesh {
	let mut mb = MeshBuilder::new();
	let depth = -0.2;

	// Top
	mb.add_quad(&[
		Vert(Vec3::new(-1.0, 0.0, -1.0), params.top),
		Vert(Vec3::new(-1.0, 0.0,  1.0), params.top),
		Vert(Vec3::new( 1.0, 0.0,  1.0), params.top),
		Vert(Vec3::new( 1.0, 0.0, -1.0), params.top),
	]);

	// Bottom
	mb.add_quad(&[
		Vert(Vec3::new(-1.0, depth, -1.0), params.bottom),
		Vert(Vec3::new( 1.0, depth, -1.0), params.bottom),
		Vert(Vec3::new( 1.0, depth,  1.0), params.bottom),
		Vert(Vec3::new(-1.0, depth,  1.0), params.bottom),
	]);

	// Left
	mb.add_quad(&[
		Vert(Vec3::new(-1.0, depth,-1.0), params.left),
		Vert(Vec3::new(-1.0, depth, 1.0), params.left),
		Vert(Vec3::new(-1.0,   0.0, 1.0), params.left),
		Vert(Vec3::new(-1.0,   0.0,-1.0), params.left),
	]);

	// Right
	mb.add_quad(&[
		Vert(Vec3::new( 1.0,   0.0,-1.0), params.right),
		Vert(Vec3::new( 1.0,   0.0, 1.0), params.right),
		Vert(Vec3::new( 1.0, depth, 1.0), params.right),
		Vert(Vec3::new( 1.0, depth,-1.0), params.right),
	]);

	// Front
	mb.add_quad(&[
		Vert(Vec3::new(-1.0, depth,-1.0), params.front),
		Vert(Vec3::new(-1.0,   0.0,-1.0), params.front),
		Vert(Vec3::new( 1.0,   0.0,-1.0), params.front),
		Vert(Vec3::new( 1.0, depth,-1.0), params.front),
	]);

	// // Back
	mb.add_quad(&[
		Vert(Vec3::new(-1.0,   0.0, 1.0), params.back),
		Vert(Vec3::new(-1.0, depth, 1.0), params.back),
		Vert(Vec3::new( 1.0, depth, 1.0), params.back),
		Vert(Vec3::new( 1.0,   0.0, 1.0), params.back),
	]);

	mb.into()
}


fn generate_decal_texture() -> Texture {
	let mut data = [0.0f32; 512*512];

	for y in 0..512 {
		for x in 0..512 {
			let idx = x + y * 512;
			let pos = Vec3::new((x as f32) / 256.0 - 1.0, 1.0, (y as f32) / 256.0 - 1.0);

			let dist = pos.length();
			let max_dist = 2.0f32.sqrt();

			let fade = (max_dist - dist).max(0.0) / (max_dist - 1.0);

			let blah = (((fade + 0.5) * 9.0 * PI).sin() * 0.5 + 0.5).powi(5);

			data[idx] = blah;
		}
	}

	let sample = |mut pos: Vec2i| {
		pos.x = pos.x.max(0).min(511);
		pos.y = pos.y.max(0).min(511);

		let idx = pos.x as usize + pos.y as usize * 512;
		data[idx]
	};

	let mut normal_data = vec![Color::black(); 512*512];

	for y in 0..512 {
		for x in 0..512 {
			let idx = x + y * 512;
			let pos = Vec2i::new(x as i32, y as i32);

			let right = Vec2i::new(1, 0);
			let up = Vec2i::new(0, 1);
			let d1 = Vec2i::new(1, 1);
			let d2 = Vec2i::new(1,-1);

			let c = sample(pos);
			let dx = ((c - sample(pos - right)) + (sample(pos + right) - c)) / 2.0;
			let dy = ((c - sample(pos - up)) + (sample(pos + up) - c)) / 2.0;
			let dd1 = ((c - sample(pos - d1)) + (sample(pos + d1) - c)) / (4.0 * 2.0f32.sqrt());
			let dd2 = ((c - sample(pos - d2)) + (sample(pos + d2) - c)) / (4.0 * 2.0f32.sqrt());

			let pos = Vec3::new((x as f32) / 256.0 - 1.0, 1.0, (y as f32) / 256.0 - 1.0);

			let dist = pos.length();
			let max_dist = 2.0f32.sqrt();

			let fade = 1.0 - (max_dist - dist).max(0.0) / (max_dist - 1.0);
			let fade = 1.0 - fade.powi(10);

			let normal_dxy = (Vec2::new(dx, dy) + d1.to_vec2() * dd1 + d2.to_vec2() * dd2).normalize() * 0.6;
			let normal = Vec3{ y: 1.0, ..normal_dxy.to_x0z() }.normalize();

			normal_data[idx] = (normal * 0.5 + 0.5).extend(c * fade).into();
		}
	}

	let mut tex = Texture::new();
	tex.upload_2d(&normal_data, Vec2i::splat(512));
	tex
}

