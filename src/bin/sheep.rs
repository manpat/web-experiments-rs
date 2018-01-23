#![feature(generators)]
#![feature(slice_patterns)]

extern crate experiments;
use experiments::*;
use experiments::rendering::mesh_builder::*;

// use rand::{Rng, thread_rng};

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
		let mut screen_size = Vec2i::zero();

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

					Event::Up(pos) => {
						let pos = screen_to_gl(screen_size, pos) / Vec2::new(1.0, CAMERA_ANGLE.sin());
						the_sheep.set_target(pos);
					}

					_ => {}
				}
			}

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
const BODY_SIZE: f32 = 0.17;

const HEAD_SIZE: f32 = 0.13;
const BODY_LENGTH: f32 = 0.2;
const LEG_THICKNESS: f32 = 0.04;

const LEG_LENGTH: f32 = 0.17;
const BODY_HEIGHT: f32 = BODY_SIZE * 0.4 + LEG_LENGTH;

const FRONT_SHOULDER_OFFSET: f32 = BODY_SIZE/5.0;
const BACK_SHOULDER_OFFSET: f32 = -BODY_SIZE/3.0;

const MAX_SPEED: f32 = 0.2;

const CAMERA_ANGLE: f32 = PI/10.0;

fn to_camera(p: Vec3) -> Vec2 {
	Vec2::new( p.x, p.y + p.z * CAMERA_ANGLE.sin() )
}

struct Sheep {
	pos: Vec3,
	body_pos: Vec3,

	heading: f32,
	current_speed: f32,

	target_pos: Option<Vec3>,

	feet_targets: [(Vec3, Vec3, f32); 4],
	feet_cycle_timeouts: [f32; 2],

	head_dip: f32,
	body_dip: f32,
}

impl Sheep {
	fn new(pos: Vec2, heading: f32) -> Self {
		let pos = Vec3{ y: BODY_HEIGHT, .. pos.to_x0z() };
		let body_pos = pos - Vec3::from_y_angle(heading) * BODY_LENGTH;

		let (shoulder_fl, shoulder_fr) = Sheep::calc_shoulder_positions(pos, heading, FRONT_SHOULDER_OFFSET);
		let (shoulder_bl, shoulder_br) = Sheep::calc_shoulder_positions(body_pos, heading, BACK_SHOULDER_OFFSET);
		
		let feet_targets = [
			(Vec3{y: 0.0, .. shoulder_fl}, Vec3{y: 0.0, .. shoulder_fl}, 1.0),
			(Vec3{y: 0.0, .. shoulder_fr}, Vec3{y: 0.0, .. shoulder_fr}, 1.0),
			(Vec3{y: 0.0, .. shoulder_bl}, Vec3{y: 0.0, .. shoulder_bl}, 1.0),
			(Vec3{y: 0.0, .. shoulder_br}, Vec3{y: 0.0, .. shoulder_br}, 1.0),
		];

		Sheep {
			pos,
			body_pos,

			heading,
			current_speed: 0.0,

			target_pos: None,
			feet_targets,
			feet_cycle_timeouts: [0.0; 2],

			head_dip: 0.0,
			body_dip: 0.0,
		}
	}

	fn set_target(&mut self, target: Vec2) {
		self.target_pos = Some(Vec3{ y: BODY_HEIGHT, .. target.to_x0z() });
	}

	fn update(&mut self) {
		if let Some(target) = self.target_pos {
			let diff = target - self.pos;
			let dist = diff.length();

			if dist < 0.1 {
				self.target_pos = None;

			} else {
				let target_heading = Vec2::new(diff.x, diff.z).to_angle();

				let mut heading_diff = target_heading - self.heading;
				if heading_diff.abs() > PI {
					heading_diff -= 2.0 * PI * heading_diff.signum();
				}

				self.heading += heading_diff.clamp(-PI/4.0, PI/4.0) / 60.0;

				let forward_thresh = PI;
				let forwardness = (forward_thresh - heading_diff.abs()).max(0.0) / forward_thresh;
				let target_speed = dist.min(MAX_SPEED) * (1.0 - (1.0 - forwardness).powi(4));

				self.current_speed = (1.0/60.0).ease_linear(self.current_speed, target_speed);
			}
		} else {
			self.current_speed = (1.0/60.0).ease_linear(self.current_speed, 0.0);
		}

		self.pos = self.pos + Vec3::from_y_angle(self.heading) * self.current_speed / 60.0;

		let diff = self.pos - self.body_pos;
		let dist = diff.length();
		if dist > BODY_LENGTH {
			self.body_pos = self.body_pos + diff.normalize() * (dist - BODY_LENGTH);
		}

		let mut segment_to_update = [(0, -1.0, Vec3::zero()); 2];

		self.feet_cycle_timeouts[0] -= 1.0/60.0;
		self.feet_cycle_timeouts[1] -= 1.0/60.0;

		for (i, &mut (ref mut start, ref mut target, ref mut phase)) in self.feet_targets.iter_mut().enumerate() {
			*phase += 2.5/60.0;

			if *phase > 1.0 && self.feet_cycle_timeouts[i/2] < 0.0 {
				*start = *target;

				let (pos, heading, offset) = [
					(self.pos, self.heading, FRONT_SHOULDER_OFFSET),
					(self.body_pos, self.heading, BACK_SHOULDER_OFFSET)
				][i / 2];

				let (shoulder_l, shoulder_r) = Sheep::calc_shoulder_positions(pos, heading, offset);
				let shoulder = [shoulder_l, shoulder_r][i % 2];

				let foot_base = Vec3{ y: 0.0, .. shoulder };

				let direction = Vec3::from_y_angle(heading);
				let perp_dir = Vec3::new(direction.z, 0.0, direction.x);

				let foot_diff = *start - foot_base;

				if foot_diff.dot(direction) < -LEG_LENGTH * (PI/10.0).sin() || foot_diff.dot(perp_dir).abs() > LEG_LENGTH * (PI/10.0).sin() {
					let toupd = &mut segment_to_update[i/2];
					let foot_dist = foot_diff.length();

					if toupd.1 < foot_dist {
						let angle_mod = 0.4f32.ease_linear((self.current_speed / MAX_SPEED).powi(3), 1.0);

						toupd.0 = i;
						toupd.1 = foot_dist;
						toupd.2 = foot_base + direction * LEG_LENGTH * (PI/3.0 * angle_mod).sin();
					}
				}
			}
		}

		for (&(foot, dist, target), cycle_timeout) in segment_to_update.iter().zip(self.feet_cycle_timeouts.iter_mut()) {
			if dist < 0.0 { continue }

			self.feet_targets[foot].1 = target;
			self.feet_targets[foot].2 = 0.0;
			*cycle_timeout = (self.current_speed / MAX_SPEED).powi(2).ease_linear(0.7, 0.5);
		}

		self.head_dip = (4.0/60.0).ease_linear(self.head_dip, self.body_dip * 0.3);
	}

	fn draw(&mut self, paper: &mut Paper) {
		let body_diff = self.pos - self.body_pos;
		let body_heading = Vec2::new(body_diff.x, body_diff.z).to_angle();
		let head_pos = self.pos + Vec3::from_y_angle(self.heading) * 0.15
			+ Vec3::new(0.0, BODY_SIZE/3.0 - self.head_dip, 0.0);

		let heading_south = self.heading.sin() < 0.0;

		let draw_head = |paper: &mut Paper| {
			paper.build_circle(to_camera(head_pos), HEAD_SIZE, FACE_COLOR);
		};

		if !heading_south { draw_head(paper); }

		if self.body_pos.z >= self.pos.z {
			Sheep::draw_body_segment(paper, self.body_pos, body_heading, BACK_SHOULDER_OFFSET, &self.feet_targets[2..4], None);
			Sheep::draw_body_segment(paper, self.pos, self.heading, FRONT_SHOULDER_OFFSET, &self.feet_targets[0..2], Some(&mut self.body_dip));
		} else {
			Sheep::draw_body_segment(paper, self.pos, self.heading, FRONT_SHOULDER_OFFSET, &self.feet_targets[0..2], Some(&mut self.body_dip));
			Sheep::draw_body_segment(paper, self.body_pos, body_heading, BACK_SHOULDER_OFFSET, &self.feet_targets[2..4], None);
		}

		if heading_south { draw_head(paper); }
	}

	fn calc_shoulder_positions(body_pos: Vec3, heading: f32, offset: f32) -> (Vec3, Vec3) {
		let shoulder_base = body_pos
			- Vec3::new(0.0, BODY_SIZE*0.6, 0.0)
			+ Vec3::from_y_angle(heading)*offset;

		let shoulder_width = BODY_SIZE*0.5;

		let left_shoulder = shoulder_base + Vec3::from_y_angle(heading + PI/2.0) * shoulder_width;
		let right_shoulder = shoulder_base + Vec3::from_y_angle(heading - PI/2.0) * shoulder_width;

		(left_shoulder, right_shoulder)
	}

	fn draw_body_segment(paper: &mut Paper, pos: Vec3, heading: f32, offset: f32, feet_targets: &[(Vec3,Vec3,f32)], body_dip_out: Option<&mut f32>) {
		assert!(feet_targets.len() >= 2);

		let (left_shoulder, right_shoulder) = Sheep::calc_shoulder_positions(pos, heading, offset);

		let mut shoulder_pos = [Vec3::zero(); 2];
		let mut body_dip = 0.0;

		for (i, (&(foot_start, foot_target, phase), &shoulder)) in feet_targets[0..2].iter().zip([left_shoulder, right_shoulder].iter()).enumerate() {
			let phase = phase.clamp(0.0, 1.0);

			let foot_pos = phase.ease_linear(foot_start, foot_target)
				+ Vec3::new(0.0, (phase*PI).sin() * (foot_start - foot_target).length() * 0.2, 0.0);

			let foot_diff = foot_pos - shoulder;
			shoulder_pos[i] = shoulder + foot_diff.normalize() * LEG_LENGTH * 0.1;

			body_dip += Vec3{ y: 0.0, ..foot_diff }.length();

			paper.build_line(&[to_camera(shoulder), to_camera(foot_pos)], LEG_THICKNESS, FACE_COLOR);
		}

		const SHOULDER_SIZE: f32 = 0.1;

		let pos = pos + Vec3::new(0.0, -body_dip * 0.07, 0.0);

		if let Some(dip) = body_dip_out {
			*dip = body_dip;
		}

		for &shoulder in shoulder_pos.iter() {
			let shoulder_offset = (pos - shoulder).normalize() * SHOULDER_SIZE * 0.7;

			paper.build_circle(to_camera(shoulder + shoulder_offset), SHOULDER_SIZE, BODY_COLOR);
		}

		paper.build_circle(to_camera(pos), BODY_SIZE, BODY_COLOR);
	}
}