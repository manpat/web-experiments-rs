
use rendering::mesh_builder::*;
use rendering::gl;
use common::*;

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct PaperVertex {
	pos: Vec2,
	color: Color,
}

impl PaperVertex {
	pub fn new<C>(pos: Vec2, color: C) -> Self where C: Into<Color> {
		PaperVertex {pos, color: Into::into(color)}
	}
}

impl Vertex for PaperVertex {
	fn get_layout() -> VertexLayout {
		VertexLayout::new::<Self>()
			.add_binding(0, 2, 0)
			.add_binding(1, 4, 8)
	}
}

#[cfg(dom_console)] static mut PAPER_COUNT: u32 = 0;

pub struct Paper {
	builder: MeshBuilder<PaperVertex>,
	mesh: Mesh,
	vert_buffer: Vec<PaperVertex>,

	#[cfg(dom_console)] id: u32,
}

impl Paper {
	pub fn new() -> Self {
		Paper {
			builder: MeshBuilder::new(),
			mesh: Mesh::new(),

			vert_buffer: Vec::with_capacity(16),

			#[cfg(dom_console)]
			id: unsafe {
				PAPER_COUNT += 1;
				PAPER_COUNT
			},
		}
	}

	pub fn clear(&mut self) {
		self.builder.clear();
	}

	pub fn draw(&mut self) {
		self.builder.upload_to(&mut self.mesh);
		self.mesh.bind();
		self.mesh.draw(gl::TRIANGLES);

		#[cfg(dom_console)]
		::console::set_section(format!("Paper #{}", self.id), format!("#verts: {} <br/> #indices: {}",
			self.builder.get_vertex_count(),
			self.builder.get_index_count()
		));
	}

	pub fn build_line<C>(&mut self, vs: &[Vec2], thickness: f32, color: C) where C: Into<Color> + Copy {
		if vs.len() < 2 { return }

		let join_quality = match thickness {
			t if t < 0.02 => 2i32,
			t if t < 0.09 => 3i32,
			t if t < 0.15 => 4i32,
			t => (5.0 + t*7.0) as i32,
		};

		let cap_quality = match thickness {
			t if t < 0.02 => 3i32,
			t if t < 0.10 => 4i32,
			t if t < 0.25 => 5i32,
			t => (6.0 + t*7.0) as i32,
		};

		let thickness = thickness / 2.0;
		let mut ns = Vec::new();

		// Build straights
		for seg in vs.windows(2) {
			let (a, b) = (seg[0], seg[1]);
			let diff = b-a;
			let n = diff.perp().normalize();

			let vs = [
				PaperVertex::new((a + n * thickness), color),
				PaperVertex::new((a - n * thickness), color),
				PaperVertex::new((b - n * thickness), color),
				PaperVertex::new((b + n * thickness), color),
			];

			self.builder.add_quad(&vs);
			ns.push(n);
		}

		// Build end caps		
		for &(vert, n) in [(vs[0],-ns[0]), (*vs.last().unwrap(), *ns.last().unwrap())].iter() {
			let n0 =  n;
			let n1 = -n;

			let nm = n1.perp();

			let diff0 = (nm-n0) / cap_quality as f32;
			let diff1 = (n1-nm) / cap_quality as f32;

			for &(start, diff) in [(n0, diff0), (nm, diff1)].iter() {
				self.vert_buffer.clear();
				self.vert_buffer.push(PaperVertex::new(vert, color));

				for i in 0 ..= cap_quality {
					let nn0 = start + diff * i as f32;
					let v0 = vert + nn0.normalize() * thickness;
					self.vert_buffer.push(PaperVertex::new(v0, color));
				}

				self.builder.add_convex_poly(&self.vert_buffer);
			}
		}

		// Build joins
		for (i, seg) in vs.windows(3).enumerate() {
			let vert = seg[1];
			let n0 = ns[i];
			let n1 = ns[i+1];

			let ang = n0.dot(n1);

			let under_side = n0.dot(n1.perp()) > 0.0;
			let (n0, n1) = if under_side { (n0, n1) } else { (-n1,-n0) };

			if ang > 0.0 {
				let diff = (n1-n0) / join_quality as f32;

				self.vert_buffer.clear();
				self.vert_buffer.push(PaperVertex::new(vert, color));

				for i in 0 ..= join_quality {
					let nn0 = n0 + diff * i as f32;
					let v0 = vert + nn0.normalize() * thickness;
					self.vert_buffer.push(PaperVertex::new(v0, color));
				}

				self.builder.add_convex_poly(&self.vert_buffer);

			} else {
				let d0 = vert - seg[0];
				let d2 = vert - seg[2];
				let nm = (d0 + d2).normalize();

				let diff0 = (nm-n0) / join_quality as f32;
				let diff1 = (n1-nm) / join_quality as f32;

				self.vert_buffer.clear();
				self.vert_buffer.push(PaperVertex::new(vert, color));

				for &(start, diff) in [(n0, diff0), (nm, diff1)].iter() {
					for i in 0 ..= join_quality {
						let nn0 = start + diff * i as f32;
						let v0 = vert + nn0.normalize() * thickness;
						self.vert_buffer.push(PaperVertex::new(v0, color));
					}
				}

				self.builder.add_convex_poly(&self.vert_buffer);
			}
		}
	}

	pub fn build_circle<C>(&mut self, p: Vec2, r: f32, color: C) where C: Into<Color> + Copy {
		self.build_ellipse(p, Vec2::splat(r), color);
	}

	pub fn build_ellipse<C>(&mut self, p: Vec2, rs: Vec2, color: C) where C: Into<Color> + Copy {
		self.vert_buffer.clear();

		let num_steps: i32 = match rs.x.max(rs.y) {
			d if d < 0.02 => 9,
			d if d < 0.05 => 16,
			d if d < 0.15 => 24,
			d if d < 0.30 => 36,
			_ => 48
		};

		let inc = PI * 2.0 / num_steps as f32;

		for i in 0..num_steps {
			let dir = Vec2::from_angle(inc * i as f32);
			let v = p + dir * rs;
			self.vert_buffer.push(PaperVertex::new(v, color));
		}

		self.builder.add_convex_poly(&self.vert_buffer);
	}
}