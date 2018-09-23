#![feature(generators)]

extern crate experiments;
use experiments::*;
use experiments::rendering::*;

use std::f32::consts::PI;

use rand::{Rng, Rand, thread_rng, random};

use events::Event;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vert (Vec2, Vec3);

impl rendering::Vertex for Vert {
	fn get_layout() -> VertexLayout {
		VertexLayout::new::<Self>()
			.add_binding(0, 2, 0)
			.add_binding(1, 3, 8)
	}
}

const DT: f32 = 1.0/60.0;

fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");

	set_coro_as_main_loop(|| {
		let webgl = WebGLContext::new(false);

		let mut events = Vec::new();

		unsafe {
			events::initialise_ems_event_queue(&mut events);
		}

		let shader = ShaderBuilder::new()
			.use_proj()
			.frag_attribute("color", "vec3")
			.output("vec4(v_color, 1.0)")

			.finalize()
			.unwrap();

		// let mut screen_size = Vec2i::zero();
		let mut aspect = 4.0/3.0;
		let mut projection = Mat4::ident();

		let mut mesh = Mesh::new();
		let mut mb = MeshBuilder::new();

		let generate_waves = move || {
			let mut waves = Vec::new();

			let y_offsets = [0.2, -0.1, -0.4];

			let base_hue = random::<f32>() * 360.0;
			let hue_delta = random::<f32>() * 50.0 - 25.0;

			let base_value = random::<f32>() * 0.3 + 0.7;
			let value_delta = (random::<f32>() + 0.3) * (0.7 - base_value) / 3.0;

			let saturation = random::<f32>() * 0.3 + 0.5;

			let base_color = Color::hsv(base_hue, saturation, base_value);

			for (i, &y_off) in y_offsets.iter().enumerate() {
				let i = i as f32 + 1.0;
				let hue = base_hue + hue_delta * i;
				let value = base_value + value_delta * i;

				let target_color = Color::hsv(hue, saturation, value);
				waves.push(Wave::new(base_color, target_color, y_off));
			}

			(base_color, waves)
		};

		let (base_color, mut waves) = generate_waves();
		webgl.set_background(base_color);

		loop {
			for e in events.iter() {
				match *e {
					Event::Resize(sz) => {
						// screen_size = sz;
						webgl.set_viewport(sz);

						aspect = sz.x as f32 / sz.y as f32;
						projection = Mat4::scale(Vec3::new(1.0/aspect, 1.0, 1.0));
					}

					Event::Down(_) => {
						let (base_color, new_waves) = generate_waves();
						webgl.set_background(base_color);
						waves = new_waves;
					}

					_ => {}
				}
			}

			events.clear();

			for wave in waves.iter_mut() { wave.update() }

			webgl.clear_all();

			shader.use_program();
			shader.set_proj(&projection);

			mb.clear();

			for wave in waves.iter() {
				wave.build(&mut mb, aspect, 0.05);
			}

			mb.upload_to(&mut mesh);

			mesh.bind();
			mesh.draw(gl::TRIANGLES);

			yield;
		}
	});
}

struct Wave {
	phase: f32,
	freq_mod_phase: f32,
	amp_phase: f32,
	amp_mod_phase: f32,

	freq: f32,
	freq_mod: f32,
	freq_mod_amt: f32,
	amp_freq: f32,
	amp_mod_freq: f32,

	wave_color: Color,
	wave_color_target: Color,
	y_offset: f32,
}

impl Wave {
	fn new(wave_color: Color, wave_color_target: Color, y_offset: f32) -> Self {
		let mut rng = thread_rng();

		Wave {
			phase: rng.gen_range(0.0, 2.0 * PI),
			amp_phase: rng.gen_range(0.0, 2.0 * PI),
			amp_mod_phase: rng.gen_range(0.0, 2.0 * PI),
			freq_mod_phase: rng.gen_range(0.0, 2.0 * PI),

			freq: rng.gen_range(1.0 / 6.0, 1.0 / 3.0) * 2.0 * PI,
			freq_mod: rng.gen_range(1.0 / 12.0, 1.0 / 6.0) * 2.0 * PI,
			freq_mod_amt: rng.gen_range(1.0 / 12.0, 1.0 / 6.0) * 2.0 * PI,
			amp_freq: rng.gen_range(1.0 / 16.0, 1.0 / 9.0) * 2.0 * PI,
			amp_mod_freq: rng.gen_range(1.0 / 20.0, 1.0 / 8.0) * 2.0 * PI,

			wave_color,
			wave_color_target,
			y_offset
		}
	}

	fn update(&mut self) {
		self.amp_mod_phase += DT * self.amp_mod_freq;
		self.amp_phase += DT * (self.amp_freq * (1.0 + self.amp_mod_phase.sin())) * 0.3;

		self.freq_mod_phase += DT * self.freq_mod;
		self.phase += DT * (self.freq + self.freq_mod_phase.sin() * self.freq_mod_amt) * 0.4;

		self.wave_color = DT.ease_linear(self.wave_color, self.wave_color_target);
	}

	fn build(&self, mb: &mut MeshBuilder<Vert>, aspect: f32, seg_width: f32) {
		let samples = (2.0 * aspect / seg_width).ceil() as usize + 1;
		let mut vs = Vec::with_capacity(samples);

		let wave_color = self.wave_color.into();

		for s in 0..samples {
			let x = s as f32 * seg_width - aspect;

			let amp_mod = (x * self.amp_freq + self.amp_phase).sin() * 0.2;
			let y = (x * self.freq + self.phase).sin() * amp_mod + self.y_offset;

			vs.push( Vert(Vec2::new(x,-1.0), wave_color) );
			vs.push( Vert(Vec2::new(x, y), wave_color) );
		}

		mb.add_tri_strip(&vs);
	}
}