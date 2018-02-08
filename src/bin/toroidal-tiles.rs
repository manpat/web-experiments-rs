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
			.frag_attribute("uv", "vec2")
			.uniform("color", "sampler2D")
			.output("texture2D(u_color, v_uv)")
			.finalize()
			.unwrap();

		shader.use_program();

		let tex = Texture::from_png(bin_asset!("tileset.png"));

		let tile_set_info = {
			let texel_size = Vec2i::splat(16);
			TileSetInfo {
				tile_infos: vec![
					TileInfo {
						name: "",
						texel_offset: Vec2i::zero(),
						texel_size
					},

					TileInfo {
						name: "",
						texel_offset: Vec2i::new(16, 0),
						texel_size
					},

					TileInfo {
						name: "player_idle",
						texel_offset: Vec2i::new(32, 0),
						texel_size
					}
				]
			}
		};

		let mut world_view = WorldView::new(&tex);
		let mut world = World::new(tile_set_info);

		let mut time = 0.0;

		loop {
			for e in events.iter() {
				match *e {
					Event::Resize(sz) => {
						world_view.screen_size = sz;

						webgl.set_viewport(sz);

						let aspect = sz.x as f32 / sz.y as f32;
						shader.set_proj(&Mat4::scale(Vec3::new(1.0/aspect, 1.0, 1.0)));
					}

					Event::Down(pos) => {
						let tile_pos = world_view.transform_screen_coord_to_tile(pos).to_vec2i();

						if let Some(v) = world.ground_layer.get_tile(tile_pos) {
							world.set_tile(tile_pos, (v+1) % 3);
						}
					}

					Event::Up(_) => {}

					_ => {}
				}
			}

			time += 1.0 / 60.0;
			world.player_pos = (Vec2::from_angle(time) * 1.5 + Vec2::splat(3.0)).extend(0.0);

			events.clear();

			tex.bind_to_slot(0);

			shader.set_view(&world_view.get_view_matrix());
			shader.set_uniform_i32("u_color", 0);

			world_view.draw(&world);

			yield;
		}
	});
}


struct WorldView {
	mesh: Mesh,
	builder: MeshBuilder<Vert2D>,

	texture_size: Vec2i,

	screen_size: Vec2i,
	camera_zoom: f32,
	camera_pan: Vec2,
}

impl WorldView {
	fn new(tex: &Texture) -> Self {
		WorldView {
			mesh: Mesh::new(),
			builder: MeshBuilder::new(),

			texture_size: tex.size,

			screen_size: Vec2i::splat(1),
			camera_zoom: 6.0,
			camera_pan: Vec2::splat(-2.5),
		}
	}

	fn get_view_matrix(&self) -> Mat4 {
		Mat4::scale(Vec3::splat(1.0/self.camera_zoom))
			* Mat4::translate(self.camera_pan.extend(0.0))
	}

	fn transform_screen_coord_to_tile(&self, point: Vec2i) -> Vec2 {
		let pos = screen_point_to_gl(self.screen_size, point);
		pos * Vec2::splat(self.camera_zoom) - self.camera_pan
		// TODO: wrap
	}

	fn draw(&mut self, world: &World) {
		self.camera_pan = -world.player_pos.to_xy();

		world.ground_layer.draw_tiles(self, &world.tile_set_info);

		if let Some(info) = world.tile_set_info.get_tile_by_name("player_idle") {
			self.draw_tile(info, world.player_pos_to_tilespace());
		}

		self.builder.upload_to(&mut self.mesh);
		self.builder.clear();

		self.mesh.bind();
		self.mesh.draw(gl::TRIANGLES);
	}

	fn draw_tile(&mut self, tile_info: &TileInfo, pos: Vec2i) {
		let texel_factor = Vec2::splat(1.0) / self.texture_size.to_vec2();

		let uv = tile_info.texel_offset.to_vec2() * texel_factor;
		let size = tile_info.texel_size.to_vec2() * texel_factor;

		let pos = pos.to_vec2();

		self.builder.add_quad(&[
			Vert2D(Vec2::new(0.0, 0.0) + pos, uv + Vec2::new(0.01, 0.98) * size),
			Vert2D(Vec2::new(0.0, 1.0) + pos, uv + Vec2::new(0.01, 0.01) * size),
			Vert2D(Vec2::new(1.0, 1.0) + pos, uv + Vec2::new(0.98, 0.01) * size),
			Vert2D(Vec2::new(1.0, 0.0) + pos, uv + Vec2::new(0.98, 0.98) * size),
		]);
	}
}


struct TileInfo {
	name: &'static str,
	texel_offset: Vec2i,
	texel_size: Vec2i,
}

struct TileSetInfo {
	tile_infos: Vec<TileInfo>,
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

	fn get_tile_by_name(&self, name: &str) -> Option<&TileInfo> {
		self.tile_infos.iter()
			.find(|ti| ti.name == name)
	}
}

struct TileMap {
	data: Vec<u8>,
	size: Vec2i,
}

impl TileMap {
	fn new(size: Vec2i) -> Self {
		TileMap {
			data: vec![0; (size.x * size.y) as usize],
			size,
		}
	}

	fn draw_tiles(&self, drawer: &mut WorldView, tile_set_info: &TileSetInfo) {
		for y in 0..self.size.y {
			for x in 0..self.size.x {
				let index = x + self.size.x * y;
				let tile_idx = self.data[index as usize];

				if let Some(info) = tile_set_info.get_tile(tile_idx as usize) {
					drawer.draw_tile(info, Vec2i::new(x, y));
				}
			}
		}
	}

	fn pos_in_bounds(&self, pos: Vec2i) -> bool {
		pos.x >= 0 && pos.x < self.size.x
		&& pos.y >= 0 && pos.y < self.size.y
	}

	fn set_tile(&mut self, pos: Vec2i, value: u8) {
		if !self.pos_in_bounds(pos) { return }

		let index = pos.x + self.size.x * pos.y;
		self.data[index as usize] = value;
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
	tile_set_info: TileSetInfo,

	player_pos: Vec3,
}

impl World {
	fn new(tile_set_info: TileSetInfo) -> Self {
		let world_size = Vec2i::splat(6);

		let mut ground_layer = TileMap::new(world_size);
		ground_layer.set_tiles_from(|_| 1);

		World {
			ground_layer,
			player_pos: Vec3::new(2.5, 2.5, 0.0),

			tile_set_info,
		}
	}

	fn player_pos_to_tilespace(&self) -> Vec2i {
		self.player_pos.to_xy().to_vec2i()
	}

	fn set_tile(&mut self, pos: Vec2i, value: u8) {
		self.ground_layer.set_tile(pos, value);
	}
}