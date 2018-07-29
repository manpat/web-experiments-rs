#![feature(generators)]

extern crate experiments;
use experiments::*;
use experiments::rendering::*;

use events::{Event, KeyCode};

#[derive(Copy, Clone)]
pub struct VertColor (Vec3, Vec3);

impl Vertex for VertColor {
	fn get_layout() -> VertexLayout {
		VertexLayout::new::<Self>()
			.add_binding(0, 3, 0)
			.add_binding(1, 3, 12)
	}
}

const DT: f32 = 1.0 / 60.0;

const CAMERA_PITCH: f32 = PI / 12.0;
const VIEWPORT_SCALE: f32 = 3.0;

const VIEWPORT_Y: f32 = 0.5;

const TURN_ACC: f32 = 3.0 * DT;
const TURN_RATE: f32 = 1.0;

fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");

	set_coro_as_main_loop(|| {
		let webgl = WebGLContext::new(false);
		webgl.set_background(Color::hsv(190.0, 0.5, 0.9));

		let mut events = Vec::new();

		unsafe {
			events::initialise_ems_event_queue(&mut events);

			gl::Enable(gl::DEPTH_TEST);
			// gl::Enable(gl::BLEND);
			// gl::BlendEquation(gl::FUNC_ADD);
			// gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		}

		let shader = ShaderBuilder::new()
			.use_3d()
			.use_highp()
			.use_proj()
			.use_view()
			.frag_attribute("color", "vec3")
			.output("vec4(v_color, 1.0)")
			.finalize()
			.unwrap();

		let mut time = 0.0f32;
		let mut screen_size = Vec2i::zero();

		let mut proj_mat = Mat4::ident();

		let mut scene_mesh = Mesh::new();
		let mut mesh_builder = MeshBuilder::new();

		let mut scene = Scene::new();

		let mut forward_pressed = false;
		let mut reverse_pressed = false;
		let mut left_pressed = false;
		let mut right_pressed = false;

		loop {
			for e in events.iter() {
				use KeyCode::*;

				match *e {
					Event::Resize(sz) => {
						screen_size = sz;

						let aspect = sz.x as f32 / sz.y as f32;
						let scale = 1.0 / VIEWPORT_SCALE;
						proj_mat = Mat4::translate(Vec3::from_y(-VIEWPORT_Y))
							* Mat4::scale(Vec3::new(scale, scale*aspect, 0.1));
					}

					Event::KeyDown(Alpha('W')) => { forward_pressed = true; }
					Event::KeyDown(Alpha('S')) => { reverse_pressed = true; }
					Event::KeyDown(Alpha('A')) => { left_pressed = true; }
					Event::KeyDown(Alpha('D')) => { right_pressed = true; }

					Event::KeyUp(Alpha('W')) => { forward_pressed = false; }
					Event::KeyUp(Alpha('S')) => { reverse_pressed = false; }
					Event::KeyUp(Alpha('A')) => { left_pressed = false; }
					Event::KeyUp(Alpha('D')) => { right_pressed = false; }

					_ => {}
				}
			}

			events.clear();

			time += DT;

			if !forward_pressed && !reverse_pressed {
				scene.player_speed = (1.0 * DT).ease_linear(scene.player_speed, 0.0);
			}

			if !left_pressed && !right_pressed {
				scene.player_rot_speed = (4.0 * DT).ease_linear(scene.player_rot_speed, 0.0);
			}

			if forward_pressed { scene.player_speed = DT.ease_linear(scene.player_speed, 4.0) }
			if reverse_pressed { scene.player_speed = DT.ease_linear(scene.player_speed,-2.0) }

			if left_pressed { scene.player_rot_speed = TURN_ACC.ease_linear(scene.player_rot_speed, TURN_RATE) }
			if right_pressed { scene.player_rot_speed = TURN_ACC.ease_linear(scene.player_rot_speed,-TURN_RATE) }

			scene.update();

			webgl.clear_all();
			webgl.set_viewport(screen_size);

			mesh_builder.clear();
			scene.render(&mut mesh_builder);

			mesh_builder.upload_to(&mut scene_mesh);

			shader.use_program();
			shader.set_proj(&proj_mat);
			shader.set_view(&Mat4::translate(Vec3::from_z(-3.0)));

			scene_mesh.bind();
			scene_mesh.draw(gl::TRIANGLES);

			yield;
		}
	});
}

struct Scene {
	player_pos: Vec2,
	player_heading: f32,

	player_speed: f32,
	player_rot_speed: f32,

	player_bounce: f32,

	barriers: Vec<Vec2>,
}

impl Scene {
	fn new() -> Scene {
		// let track = [
		// 	Vec2::new(  0.0,  0.0),
		// 	Vec2::new(  0.0, 10.0),
		// 	Vec2::new(-10.0, 10.0),
		// 	Vec2::new(-10.0,  0.0),
		// 	Vec2::new(  0.0,  0.0),
		// ];

		let mut barriers = Vec::new();

		for i in 0..20 {
			let y = i as f32 * 0.4;
			barriers.push(Vec2::new(-1.5, y));
			barriers.push(Vec2::new( 1.5, y));

			barriers.push(Vec2::new( y + 1.5, 0.0));
			barriers.push(Vec2::new( y + 1.5,-3.0));

			barriers.push(Vec2::new(-1.5 + 20.0 * 0.4 + 3.0, y));
			barriers.push(Vec2::new( 1.5 + 20.0 * 0.4 + 3.0, y));

			barriers.push(Vec2::new( y + 1.5, 3.0 + 20.0 * 0.4));
			barriers.push(Vec2::new( y + 1.5, 0.0 + 20.0 * 0.4));
		}

		Scene {
			player_pos: Vec2::zero(),
			player_heading: PI/2.0, // face along y

			player_speed: 0.0,
			player_rot_speed: 0.0,

			player_bounce: 0.0,

			barriers,
		}
	}

	fn update(&mut self) {
		self.player_pos += self.player_forward() * self.player_speed * DT;
		self.player_heading += self.player_rot_speed * DT;
		self.player_bounce += (self.player_speed.abs() + 1.0).sqrt() * DT;
	}

	fn render(&self, mb: &mut MeshBuilder<VertColor>) {
		for &b in self.barriers.iter() {
			self.render_barrier(mb, b);
		}

		self.render_player(mb);
	}

	fn player_forward(&self) -> Vec2 { Vec2::from_angle(self.player_heading) }
	fn player_right(&self) -> Vec2 { Vec2::from_angle(self.player_heading + PI/2.0) }

	fn inverse_player_forward(&self) -> Vec2 { Vec2::from_angle(-self.player_heading) }
	fn inverse_player_right(&self) -> Vec2 { Vec2::from_angle(-self.player_heading - PI/2.0) }

	fn transform_to_player_space(&self, scene_pos: Vec2) -> Vec3 {
		let fwd = self.inverse_player_forward();
		let rght = self.inverse_player_right();

		let translated = self.player_pos - scene_pos;
		let rotated = rght * translated.x + fwd * translated.y;

		let projected_y = rotated.y * CAMERA_PITCH.sin();
		Vec3::new(rotated.x, projected_y, rotated.y / 10.0)
	}

	fn render_barrier(&self, mb: &mut MeshBuilder<VertColor>, scene_pos: Vec2) {
		let origin = self.transform_to_player_space(scene_pos);
		let color = Color::hsv(10.0, 0.5, 0.9).into();

		mb.add_vert(VertColor(Vec3::new(-0.1, 0.0, 0.0) + origin, color));
		mb.add_vert(VertColor(Vec3::new( 0.0, 0.1, 0.0) + origin, color));
		mb.add_vert(VertColor(Vec3::new( 0.1, 0.0, 0.0) + origin, color));
	}

	fn render_player(&self, mb: &mut MeshBuilder<VertColor>) {
		let color = Color::hsv(340.0, 0.5, 0.9).into();
		let s = 0.3;

		let bounce = (self.player_bounce * PI * 2.0).sin().abs() * (self.player_speed + 1.0).min(2.0) * 0.01;

		mb.add_quad(&[
			VertColor(Vec3::new(-s/2.0,     bounce, 0.0), color),
			VertColor(Vec3::new( s/2.0,     bounce, 0.0), color),
			VertColor(Vec3::new( s/2.0, s + bounce, 0.0), color),
			VertColor(Vec3::new(-s/2.0, s + bounce, 0.0), color),
		]);
	}
}
