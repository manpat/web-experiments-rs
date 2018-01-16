use mut_rc::*;

use common::*;
use paper::*;

use coro_util::*;

pub struct ParticleManager {
	paper: MutRc<Paper>,
	particle_systems: Vec<Coro<()>>
}

impl ParticleManager {
	pub fn new() -> Self {
		ParticleManager {
			paper: MutRc::new(Paper::new()),
			particle_systems: Vec::new(),
		}
	}

	pub fn draw(&mut self) {
		self.paper.borrow_mut().clear();

		for coro in self.particle_systems.iter_mut() {
			coro.next();
		}

		self.paper.borrow_mut().draw();
		self.particle_systems.retain(Coro::is_valid);
	}

	pub fn add_pop(&mut self, pos: Vec2) {
		let paper = self.paper.clone();

		self.particle_systems.push(Coro::from(move || {
			let dt = 1.0 / 60.0;
			let mut progress = 0.0;

			while progress < 1.0 {
				yield;

				let mut paper = paper.borrow_mut();

				let inner_radius = progress.ease_quad_out(0.0, 0.079);
				let outer_radius = progress.ease_quad_out(0.04, 0.08);
				let thickness = (progress*2.0 - 1.0).ease_quad_in(0.02, 0.001);

				let inc = 2.0 * PI / 6.0;

				for i in 0..6 {
					let offset = Vec2::from_angle(inc * i as f32);
					let inner = pos + offset * inner_radius;
					let outer = pos + offset * outer_radius;

					paper.build_line(&[inner, outer], thickness, Vec4::splat(1.0));
				}

				progress += dt / 0.4;
			}
		}));
	}
}


