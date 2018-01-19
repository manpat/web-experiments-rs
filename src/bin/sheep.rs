#![feature(generators)]

extern crate experiments;
use experiments::*;
use experiments::rendering::mesh_builder::*;

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
		paper_shader.use_program();

		let mut paper = Paper::new();
		let mut time = 0.0f32;

		let mut screen_size = Vec2i::zero();

		// let mut sheep = [
		// 	Sheep::new(Vec2::new( 1.2, 1.7), PI/4.0),
		// 	Sheep::new(Vec2::new( 1.2,-1.7), PI/3.0),
		// 	Sheep::new(Vec2::new(-1.2,-1.7),-PI/3.0),
		// 	Sheep::new(Vec2::new(-1.2, 1.7),-PI/6.0),
		// ];

		// sheep.sort_by(|a, b| b.pos.y.partial_cmp(&a.pos.y).unwrap());

		let mut the_sheep = Sheep::new(Vec2::zero(), -PI/4.0);

		loop {
			for e in events.iter() {
				match *e {
					Event::Resize(sz) => unsafe {
						screen_size = sz;
						gl::Viewport(0, 0, sz.x, sz.y);

						let aspect = sz.x as f32 / sz.y as f32;

						paper_shader.use_program();
						paper_shader.set_proj(&Mat4::scale(Vec3::new(1.0/aspect, 1.0, 1.0)));
					}

					Event::Click(pos) => {
						let pos = screen_to_gl(screen_size, pos) / camera_coeff();
						the_sheep.set_target(pos);
						// sheep[0].set_target(pos);
					}

					_ => {}
				}
			}

			time += 1.0/60.0;

			events.clear();
			paper.clear();

			unsafe {
				gl::Clear(gl::COLOR_BUFFER_BIT);
			}

			// sheep.sort_by(|a, b| b.pos.y.partial_cmp(&a.pos.y).unwrap());

			// for s in &mut sheep { s.update() }
			// for s in &mut sheep { s.draw(&mut paper) }

			the_sheep.update();
			the_sheep.draw(&mut paper);

			paper.draw();

			yield;
		}
	});
}

fn screen_to_gl(screen_size: Vec2i, v: Vec2i) -> Vec2{
	let sz = screen_size.to_vec2();
	let aspect = sz.x as f32 / sz.y as f32;

	let norm = v.to_vec2() / screen_size.to_vec2() * 2.0 - Vec2::splat(1.0);
	norm * Vec2::new(aspect, -1.0)
}


const BODY_COLOR: Color = Color::grey(0.8);
const FACE_COLOR: Color = Color::grey(0.3);
const BODY_SIZE: f32 = 0.2;
const HEAD_SIZE: f32 = 0.13;
const SHOULDER_SIZE: f32 = 0.1;
const BODY_LENGTH: f32 = 0.2;
const LEG_THICKNESS: f32 = 0.04;

const CAMERA_ANGLE: f32 = PI/10.0;

fn camera_coeff() -> Vec2 { Vec2::new(1.0, CAMERA_ANGLE.sin()) }

struct Sheep {
	pos: Vec2,
	body_pos: Vec2,

	heading: f32,
	current_speed: f32,

	target_pos: Option<Vec2>,
}

impl Sheep {
	fn new(pos: Vec2, heading: f32) -> Self {
		Sheep{
			pos,
			body_pos: pos - Vec2::from_angle(heading) * BODY_LENGTH,

			heading,
			current_speed: 0.0,

			target_pos: None,
		}
	}

	fn set_target(&mut self, target: Vec2) {
		self.target_pos = Some(target);
	}

	fn update(&mut self) {
		if let Some(target) = self.target_pos {
			let diff = target - self.pos;
			let dist = diff.length();

			if dist < 0.1 {
				self.target_pos = None;

			} else {
				let target_heading = diff.to_angle();

				let mut heading_diff = target_heading - self.heading;
				if heading_diff.abs() > PI {
					heading_diff -= 2.0 * PI * heading_diff.signum();
				}

				self.heading += heading_diff.clamp(-PI/4.0, PI/4.0) / 60.0;

				let forward_thresh = PI*7.0/8.0;
				let forwardness = (forward_thresh - heading_diff.abs()).max(0.0) / forward_thresh;
				let target_speed = dist.min(0.3) * (1.0 - (1.0 - forwardness).powi(2));

				self.current_speed = (1.0/60.0).ease_linear(self.current_speed, target_speed);
			}
		} else {
			self.current_speed = (1.0/60.0).ease_linear(self.current_speed, 0.0);
		}

		self.pos = self.pos + Vec2::from_angle(self.heading) * self.current_speed / 60.0;

		let diff = self.pos - self.body_pos;
		let dist = diff.length();
		if dist > BODY_LENGTH {
			self.body_pos = self.body_pos + diff.normalize() * (dist - BODY_LENGTH);
		}
	}

	fn draw(&mut self, paper: &mut Paper) {
		let camera_coeff = camera_coeff();

		let body_heading = (self.pos - self.body_pos).to_angle();
		let head_pos = self.pos + Vec2::from_angle(self.heading) * 0.15 + Vec2::new(0.0, BODY_SIZE/3.0);

		let front_shoulder_offset = 0.0;
		let back_shoulder_offset = -BODY_SIZE/3.0;

		let heading_south = self.heading.sin() < 0.0;

		let draw_head = |paper: &mut Paper| {
			paper.build_circle(camera_coeff * head_pos, HEAD_SIZE, FACE_COLOR);
		};

		if !heading_south { draw_head(paper); }

		if self.body_pos.y >= self.pos.y {
			Sheep::draw_body_segment(paper, self.body_pos, body_heading, back_shoulder_offset);
			Sheep::draw_body_segment(paper, self.pos, self.heading, front_shoulder_offset);
		} else {
			Sheep::draw_body_segment(paper, self.pos, self.heading, front_shoulder_offset);
			Sheep::draw_body_segment(paper, self.body_pos, body_heading, back_shoulder_offset);
		}

		if heading_south { draw_head(paper); }
	}

	fn draw_body_segment(paper: &mut Paper, pos: Vec2, heading: f32, offset: f32) {
		let camera_coeff = camera_coeff();
		let pos = pos * camera_coeff;

		let shoulder_base = pos - Vec2::new(0.0, BODY_SIZE*0.4)
			+ Vec2::from_angle(heading)*camera_coeff*offset;

		let shoulder_width = BODY_SIZE*0.4;

		let left_shoulder = shoulder_base + Vec2::from_angle(heading + PI/2.0) * camera_coeff * shoulder_width;
		let right_shoulder = shoulder_base + Vec2::from_angle(heading - PI/2.0) * camera_coeff * shoulder_width;

		let leg_vector = Vec2::new(0.0, SHOULDER_SIZE+0.05);

		paper.build_line(&[left_shoulder, left_shoulder-leg_vector], LEG_THICKNESS, FACE_COLOR);
		paper.build_line(&[right_shoulder, right_shoulder-leg_vector], LEG_THICKNESS, FACE_COLOR);

		paper.build_circle(left_shoulder, SHOULDER_SIZE, BODY_COLOR);
		paper.build_circle(pos, BODY_SIZE, BODY_COLOR);
		paper.build_circle(right_shoulder, SHOULDER_SIZE, BODY_COLOR);
	}
}