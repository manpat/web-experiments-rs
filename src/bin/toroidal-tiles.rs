#![feature(generators)]

extern crate experiments;

use experiments::*;
use experiments::rendering::*;

use events::Event;

#[repr(C)]
#[derive(Copy, Clone)]
struct Vert2D(Vec2, Vec2);

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

			gl::Enable(gl::BLEND);
			gl::BlendEquation(gl::FUNC_ADD);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		}

		webgl.set_background(Color::grey(0.2));

		let shader = ShaderBuilder::new()
			.use_highp()
			.use_proj()
			.use_view()
			.attribute("uv", "vec2")
			.varying("uv", "vec2")
			.uniform("color", "sampler2D")
			.vertex("v_uv = uv")
			.output("texture2D(u_color, v_uv)")
			.finalize()
			.unwrap();

		shader.use_program();

		let tex = Texture::from_png(bin_asset!("tileset.png"));

		let tile_set_info = {
			let mut tsi = TileSetInfo { tile_infos: Vec::new(), texture_size: tex.size };
			let texel_size = Vec2i::splat(16);

			tsi.tile_infos.push(TileInfo {
				texel_offset: Vec2i::zero(),
				texel_size
			});
			tsi.tile_infos.push(TileInfo {
				texel_offset: Vec2i::new(16, 0),
				texel_size
			});
			tsi.tile_infos.push(TileInfo {
				texel_offset: Vec2i::new(32, 0),
				texel_size
			});

			tsi
		};

		let mut world = World::new(tile_set_info);

		let mut camera_zoom = 4.0;
		let mut camera_pos = Vec2::splat(-4.0);

		let mut screen_size = Vec2i::zero();

		loop {
			for e in events.iter() {
				match *e {
					Event::Resize(sz) => unsafe {
						screen_size = sz;
						gl::Viewport(0, 0, sz.x, sz.y);

						let aspect = sz.x as f32 / sz.y as f32;
						shader.set_proj(&Mat4::scale(Vec3::new(1.0/aspect, 1.0, 1.0)));
					}

					Event::Down(pos) => {
						let pos = screen_to_gl(screen_size, pos);
						let tile_pos = pos * Vec2::splat(camera_zoom) - camera_pos;
						let tile_pos = tile_pos.to_vec2i();

						if let Some(v) = world.ground_layer.get_tile(tile_pos) {
							world.set_tile(tile_pos, (v+1) % 3);
						}
					}

					Event::Up(_) => {}

					_ => {}
				}
			}

			events.clear();

			webgl.set_viewport(screen_size);

			tex.bind_to_slot(0);

			let view_mat = Mat4::scale(Vec3::splat(1.0/camera_zoom))
				* Mat4::translate(camera_pos.extend(0.0));

			shader.set_view(&view_mat);
			shader.set_uniform_i32("u_color", 0);

			world.draw();

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


struct TileInfo {
	texel_offset: Vec2i,
	texel_size: Vec2i,
}

struct TileSetInfo {
	tile_infos: Vec<TileInfo>,
	texture_size: Vec2i,
}

impl TileSetInfo {
	fn get_tile(&self, index: usize) -> Option<&TileInfo> {
		if index == 0 {
			None
		} else if index-1 < self.tile_infos.len() {
			Some(&self.tile_infos[index-1])
		} else {
			None
		}
	}
}

struct TileMap {
	data: Vec<u8>,
	size: Vec2i,

	mesh: Mesh,
	has_changed: bool,
}

impl TileMap {
	fn new(size: Vec2i) -> Self {
		TileMap {
			data: vec![0; (size.x * size.y) as usize],
			size,
			mesh: Mesh::new(),
			has_changed: true,
		}
	}

	fn build_mesh(&mut self, tile_set_info: &TileSetInfo) {
		if !self.has_changed { return }
		self.has_changed = false;

		let texel_factor = Vec2::splat(1.0) / tile_set_info.texture_size.to_vec2();

		let mut mb = MeshBuilder::new();
		for y in 0..self.size.y {
			for x in 0..self.size.x {
				let index = x + self.size.x * y;
				let tile_idx = self.data[index as usize];

				let offset = Vec2::new(x as f32, y as f32);

				if let Some(info) = tile_set_info.get_tile(tile_idx as usize) {
					let uv = info.texel_offset.to_vec2() * texel_factor;
					let size = info.texel_size.to_vec2() * texel_factor;

					mb.add_quad(&[
						Vert2D(Vec2::new(0.0, 0.0) + offset, uv + Vec2::new(0.01, 0.98) * size),
						Vert2D(Vec2::new(0.0, 1.0) + offset, uv + Vec2::new(0.01, 0.01) * size),
						Vert2D(Vec2::new(1.0, 1.0) + offset, uv + Vec2::new(0.98, 0.01) * size),
						Vert2D(Vec2::new(1.0, 0.0) + offset, uv + Vec2::new(0.98, 0.98) * size),
					]);
				}
			}
		}

		mb.upload_to(&mut self.mesh);
	}

	fn pos_in_bounds(&self, pos: Vec2i) -> bool {
		pos.x >= 0 && pos.x < self.size.x
		&& pos.y >= 0 && pos.y < self.size.y
	}

	fn set_tile(&mut self, pos: Vec2i, value: u8) {
		if !self.pos_in_bounds(pos) { return }

		let index = pos.x + self.size.x * pos.y;
		self.data[index as usize] = value;
		self.has_changed = true;
	}

	fn set_tiles_from<F>(&mut self, f: F) where F: Fn(Vec2i) -> u8 {
		for y in 0..self.size.y {
			for x in 0..self.size.x {
				let index = x + self.size.x * y;
				self.data[index as usize] = f(Vec2i::new(x, y));
			}
		}
	}

	fn get_tile(&self, pos: Vec2i) -> Option<u8> {
		if self.pos_in_bounds(pos) {
			let index = pos.x + self.size.x * pos.y;
			Some(self.data[index as usize])
		} else {
			None
		}
	}
}

struct World {
	ground_layer: TileMap,
	player_layer: TileMap,
	ui_layer: TileMap,

	tile_set_info: TileSetInfo,

	player_pos: Vec3,
}

impl World {
	fn new(tile_set_info: TileSetInfo) -> Self {
		let world_size = Vec2i::splat(8);

		World {
			ground_layer: TileMap::new(world_size),
			player_layer: TileMap::new(world_size),
			ui_layer: TileMap::new(world_size),

			tile_set_info,

			player_pos: Vec3::new(3.5, 3.5, 0.0),
		}
	}

	fn player_pos_to_tilespace(&self) -> Vec2i {
		Vec2i::new(self.player_pos.x as i32, self.player_pos.y as i32)
	}

	fn draw(&mut self) {
		let player_pos = self.player_pos_to_tilespace();
		self.player_layer.set_tiles_from(|pos| {
			let Vec2i{x: px, y: py} = player_pos;
			let Vec2i{x: tx, y: ty} = pos;

			if tx == px && ty == py { 3 }
			else { 0 }
		});

		self.ground_layer.build_mesh(&self.tile_set_info);
		self.player_layer.build_mesh(&self.tile_set_info);
		self.ui_layer.build_mesh(&self.tile_set_info);

		self.ground_layer.mesh.bind();
		self.ground_layer.mesh.draw(gl::TRIANGLES);

		self.player_layer.mesh.bind();
		self.player_layer.mesh.draw(gl::TRIANGLES);

		self.ui_layer.mesh.bind();
		self.ui_layer.mesh.draw(gl::TRIANGLES);
	}

	fn set_tile(&mut self, pos: Vec2i, value: u8) {
		self.ground_layer.set_tile(pos, value);
	}
}