#![feature(inclusive_range_syntax)]
#![feature(slice_patterns)]
#![feature(generators)]
#![feature(link_args)]

#[allow(unused_attributes)]
#[link_args = "--js-library src/js/console.js"]
extern "C" {}

extern crate experiments;
use experiments::*;

use events::Event;

use std::time::Instant;

fn main() {
	set_coro_as_main_loop(|| {
		console::init();
		console::set_color("#222");

		let _gl = WebGLContext::new(false);

		let mut events = Vec::new();

		unsafe {
			events::initialise_ems_event_queue(&mut events);

			gl::Enable(gl::BLEND);
			gl::BlendEquation(gl::FUNC_ADD);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

			gl::ClearColor(0.95, 0.95, 0.95, 1.0);
		}

		let shader = Shader::new(res::shaders::PAPER_VS, res::shaders::PAPER_FS);
		shader.use_program();

		let mut screen_size = Vec2i::zero();

		let mut paper = Paper::new();
		let mut particles = ParticleManager::new();

		// let gravity = Vec2::zero();
		let gravity = Vec2::new(0.0,-2.0);

		let mut rope = Rope{joints: Vec::new()};
		rope.joints.push(Joint{pos: Vec2::new( 0.9, 0.0), vel: Vec2::zero(), fixed: true});
		for i in -10..=10 {
			let i = i as f32 / 10.0;
			rope.joints.push(Joint{pos: Vec2::new(-i * 0.85, 0.0), vel: Vec2::zero(), fixed: false});
		}
		rope.joints.push(Joint{pos: Vec2::new(-0.9, 0.0), vel: Vec2::zero(), fixed: true});

		let mut dragging = false;

		loop {
			let frame_start = Instant::now();

			for e in events.iter() {
				match *e {
					Event::Resize(sz) => unsafe {
						screen_size = sz;

						gl::Viewport(0, 0, sz.x, sz.y);

						let aspect = sz.x as f32 / sz.y as f32;
						shader.set_proj(&Mat4::scale(Vec3::new(1.0/aspect, 1.0, 1.0)));
					}

					Event::Move(pos) => if dragging {
						rope.joints[0].pos = screen_to_gl(screen_size, pos);
					}

					Event::Down(_) => { dragging = true; }
					Event::Up(_) => { dragging = false; }

					_ => {}
				}
			}

			events.clear();

			simulate_rope(&mut rope, gravity);

			unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

			paper.clear();

			for seg in rope.joints.windows(2) {
				let (a,b) = (&seg[0], &seg[1]);
				paper.build_line(&[a.pos, b.pos], 0.04, Color::grey(0.4));
			}

			paper.draw();

			particles.draw();

			let dur = frame_start.elapsed();
			console::set_section("Stats", format!("frame time: {:.1}ms", dur.subsec_nanos() as f64 / 1000_000.0));
			console::update();

			yield;
		}
	});
}

struct Joint {
	pos: Vec2,
	vel: Vec2,
	fixed: bool,
}

struct Rope {
	joints: Vec<Joint>,
}

fn simulate_rope(rope: &mut Rope, gravity: Vec2) {
	const SPRING_LENGTH: f32 = 0.1; 
	const SPRING_COEFF: f32 = 10000.0; 
	const FRICTION_COEFF: f32 = 15.0; 
	const MASS: f32 = 10.0;

	let mut accs = vec![gravity; rope.joints.len()];

	for (i, ab) in rope.joints.windows(2).enumerate() {
		let (a,b) = (&ab[0], &ab[1]);

		let diff = a.pos - b.pos;
		let len = diff.length();
		let spring_force = if len < 0.0001 { Vec2::zero() }
		else { -diff / len * (len - SPRING_LENGTH) * SPRING_COEFF };

		let friction_acc = -(a.vel - b.vel) * FRICTION_COEFF;

		let force = spring_force / MASS + friction_acc;

		accs[i] = accs[i] + force;
		accs[i+1] = accs[i+1] - force;
	}

	for (joint, acc) in rope.joints.iter_mut().zip(accs.into_iter()) {
		if joint.fixed { continue }

		joint.vel = joint.vel + acc / 60.0;
		joint.pos = joint.pos + joint.vel / 60.0;
	}
}

fn screen_to_gl(screen_size: Vec2i, v: Vec2i) -> Vec2{
	let sz = screen_size.to_vec2();
	let aspect = sz.x as f32 / sz.y as f32;

	let norm = v.to_vec2() / screen_size.to_vec2() * 2.0 - Vec2::splat(1.0);
	norm * Vec2::new(aspect, -1.0)
}
