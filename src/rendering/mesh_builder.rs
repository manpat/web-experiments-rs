#![allow(dead_code)]

use std::mem::size_of;
use std::ptr::null;
use rendering::gl;
use rendering::types::*;

pub struct VertexAttributeBinding {
	pub index: u32,
	pub width: i32,
	pub offset: u32,
}

pub struct VertexLayout {
	pub size: u32,
	pub attributes: Vec<VertexAttributeBinding>,
}

impl VertexLayout {
	pub fn new<V: Vertex>() -> Self {
		VertexLayout {
			size: size_of::<V>() as _,
			attributes: Vec::new()
		}
	}

	pub fn null() -> Self {
		VertexLayout { size: 0, attributes: Vec::new() }
	}

	pub fn add_binding(mut self, index: u32, width: i32, offset: u32) -> Self {
		self.attributes.push(VertexAttributeBinding{index, width, offset});
		self
	}
}

pub trait Vertex: Copy + Clone {
	fn get_layout() -> VertexLayout;
}



#[derive(Copy, Clone)]
pub struct DefaultVertex {
	pos: Vec3,
}

impl DefaultVertex {
	pub fn new(pos: Vec3) -> Self {
		DefaultVertex{pos}
	}
}

impl Vertex for DefaultVertex {
	fn get_layout() -> VertexLayout {
		VertexLayout::new::<Self>()
			.add_binding(0, 3, 0)
	}
}



pub struct Mesh {
	pub vbo: u32,
	pub ebo: u32,
	pub count: u32,
	pub layout: VertexLayout,
}

impl Mesh {
	pub fn new() -> Self {
		Mesh {
			vbo: gl::pls_make_buffer(),
			ebo: gl::pls_make_buffer(),
			count: 0,
			layout: VertexLayout::null(),
		}
	}

	pub fn bind(&self) {
		unsafe {
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

			for ab in self.layout.attributes.iter() {
				gl::EnableVertexAttribArray(ab.index);
				gl::VertexAttribPointer(ab.index, ab.width, gl::FLOAT, gl::FALSE, self.layout.size as i32, ab.offset as _);
			}
		}
	}

	pub fn draw(&self, mode: u32) {
		unsafe {
			gl::DrawElements(mode, self.count as _, gl::UNSIGNED_SHORT, null());
		}
	}
}



pub struct MeshBuilder<V: Vertex> {
	verts: Vec<V>,
	indices: Vec<u16>, // NOTE: index type be an option
}

impl<V> MeshBuilder<V> where V: Vertex {
	pub fn new() -> Self {
		MeshBuilder {
			verts: Vec::new(),
			indices: Vec::new(),
		}
	}

	pub fn clear(&mut self) {
		self.verts.clear();
		self.indices.clear();
	}

	pub fn get_vertex_count(&self) -> usize { self.verts.len() }
	pub fn get_index_count(&self) -> usize { self.indices.len() }

	pub fn upload_to(&self, mesh: &mut Mesh) {
		unsafe {
			mesh.layout = V::get_layout();
			mesh.count = self.indices.len() as _;
			let vert_size = mesh.layout.size * self.verts.len() as u32;
			let idx_size = size_of::<u16>() * self.indices.len();

			gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
			gl::BufferData(gl::ARRAY_BUFFER, vert_size as _, self.verts.as_ptr() as _, gl::STATIC_DRAW);

			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
			gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, idx_size as _, self.indices.as_ptr() as _, gl::STATIC_DRAW);
		}
	}

	pub fn add_vert(&mut self, v: V) {
		self.indices.push(self.verts.len() as _);
		self.verts.push(v);
	}

	pub fn add_direct(&mut self, vs: &[V], es: &[u16]) {
		assert!(es.len() >= 3);

		let base = self.verts.len() as u16;

		self.verts.extend_from_slice(vs);
		self.indices.extend(es.iter().map(|&e| e + base));
	}

	pub fn add_quad(&mut self, vs: &[V]) {
		assert!(vs.len() >= 4);

		let base = self.verts.len() as u16;
		self.verts.extend_from_slice(&vs[..4]);

		self.indices.push(base + 0);
		self.indices.push(base + 1);
		self.indices.push(base + 2);

		self.indices.push(base + 0);
		self.indices.push(base + 2);
		self.indices.push(base + 3);
	}

	pub fn add_convex_poly(&mut self, vs: &[V]) {
		assert!(vs.len() >= 3);

		let base = self.verts.len() as u16;
		self.verts.extend_from_slice(vs);

		for i in 1..vs.len()-1 {
			let i = i as u16;
			self.indices.push(base + 0);
			self.indices.push(base + i);
			self.indices.push(base + i+1);
		}
	}
}

impl<V> From<MeshBuilder<V>> for Mesh where V: Vertex {
	fn from(mb: MeshBuilder<V>) -> Self {
		let mut mesh = Mesh::new();
		mb.upload_to(&mut mesh);
		mesh
	}
}
