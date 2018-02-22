#![feature(generators)]

extern crate experiments;

use experiments::*;
use experiments::rendering::*;

use events::{Event, KeyCode};

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

const TEXELS_PER_TILE: u32 = 16;
const TEXEL_FACTOR: Vec2 = Vec2::splat(1.0 / 128.0);

fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");

	set_coro_as_main_loop(|| {
		let webgl = WebGLContext::new(false);
		webgl.set_background(Color::grey(0.2));

		let mut events = Vec::new();

		unsafe {
			events::initialise_ems_event_queue(&mut events);

			gl::Enable(gl::BLEND);
			gl::BlendEquation(gl::FUNC_ADD);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		}

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

		let tile_set = {
			use tile_flags::*;

			let texel_size = Vec2i::splat(16);
			TileSet {
				tile_infos: vec![
					TileInfo {
						name: "ground",
						texel_offset: Vec2i::zero(),
						texel_size,
						flags: 0,
					},

					TileInfo {
						name: "ladder",
						texel_offset: Vec2i::new(16, 0),
						texel_size,
						flags: ALLOWS_Z_MOVE,
					},

					TileInfo {
						name: "player_idle",
						texel_offset: Vec2i::new(32, 0),
						texel_size,
						flags: 0,
					},

					TileInfo {
						name: "cursor",
						texel_offset: Vec2i::new(48, 0),
						texel_size,
						flags: 0,
					},

					TileInfo {
						name: "generator",
						texel_offset: Vec2i::new(32, 0),
						texel_size,
						flags: 0,
					},

					TileInfo {
						name: "storage",
						texel_offset: Vec2i::new(32, 0),
						texel_size,
						flags: 0,
					},

					TileInfo {
						name: "pipe_base",
						texel_offset: Vec2i::new(32, 0),
						texel_size,
						flags: 0,
					},
					TileInfo {
						name: "pipe_connector",
						texel_offset: Vec2i::new(32, 0),
						texel_size,
						flags: 0,
					},
				]
			}
		};

		let mut world_view = WorldView::new(&tex);
		let mut world = World::new(tile_set);

		let mut show_menu = false;
		let mut mouse_pos = Vec2i::zero();

		let mut ui_mesh = Mesh::new();
		let mut ui_builder = MeshBuilder::new();

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
						let Vec2{x, y} = world_view.transform_screen_coord_to_tile(pos);
						let tile_pos = Vec2i::new(x.floor() as i32, y.floor() as i32);

						if let Some(v) = world.get_tile(tile_pos) {
							world.set_tile(tile_pos, (v+1) % 3);
						}
					}

					Event::Move(pos) => {
						mouse_pos = pos; 
					}

					Event::KeyDown(k) => {
						let prev_pos = world.player_pos;

						let mut did_warp = false;
						let mut did_shift = false;

						match k {
							KeyCode::Alpha('W') => if world.move_player(Vec2i::new(0, 1)) { did_warp = true }
							KeyCode::Alpha('S') => if world.move_player(Vec2i::new(0,-1)) { did_warp = true }
							KeyCode::Alpha('D') => if world.move_player(Vec2i::new( 1, 0)) { did_warp = true }
							KeyCode::Alpha('A') => if world.move_player(Vec2i::new(-1, 0)) { did_warp = true }
	
							KeyCode::Alpha('Q') => {
								let z_move_allowed = world.get_tile_info(prev_pos.to_vec2i())
									.map(|ti| ti.allows_z_move())
									.unwrap_or(false);

								if z_move_allowed {
									did_shift |= world.shift_layer(-1)
								}
							}

							KeyCode::Alpha('E') => {
								let z_move_allowed = world.get_tile_info(prev_pos.to_vec2i())
									.map(|ti| ti.allows_z_move())
									.unwrap_or(false);

								if z_move_allowed {
									did_shift |= world.shift_layer( 1)
								}
							}
	
							KeyCode::Tab => {
								show_menu ^= true;
							}
							_ => {}
						}

						if did_warp {
							let dir = (world.player_pos - prev_pos).normalize();

							world_view.camera_pos = world_view.camera_pos 
								+ dir * world.layers[world.player_layer as usize].size.to_vec2();

						} else if did_shift {
							let cam_diff = world_view.camera_pos - prev_pos.to_vec2i().to_vec2();
							world_view.camera_pos = world.player_pos.to_vec2i().to_vec2() + cam_diff;
						}
					}

					_ => {}
				}
			}

			events.clear();

			tex.bind_to_slot(0);

			shader.set_view(&world_view.get_view_matrix());
			shader.set_uniform_i32("u_color", 0);

			world_view.draw(&world);

			{	let cursor = world.tile_set.get_tile_info_by_name("cursor").unwrap();

				let Vec2{x, y} = world_view.transform_screen_coord_to_tile(mouse_pos);
				let mouse_tile = Vec2::new(x.floor(), y.floor());

				ui_builder.draw_tiled(cursor.texel_offset, cursor.texel_size, mouse_tile);
	
				ui_builder.upload_to(&mut ui_mesh);
				ui_builder.clear();

				ui_mesh.bind();
				ui_mesh.draw(gl::TRIANGLES);
			}

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
	camera_pos: Vec2,
}

impl WorldView {
	fn new(tex: &Texture) -> Self {
		WorldView {
			mesh: Mesh::new(),
			builder: MeshBuilder::new(),

			texture_size: tex.size,

			screen_size: Vec2i::splat(1),
			camera_zoom: 6.0,
			camera_pos: Vec2::splat(2.5),
		}
	}

	fn get_view_matrix(&self) -> Mat4 {
		Mat4::scale(Vec3::splat(1.0/self.camera_zoom))
			* Mat4::translate(-self.camera_pos.extend(0.0))
	}

	fn transform_screen_coord_to_tile(&self, point: Vec2i) -> Vec2 {
		let pos = screen_point_to_gl(self.screen_size, point);
		pos * Vec2::splat(self.camera_zoom) + self.camera_pos
	}

	fn draw(&mut self, world: &World) {
		self.camera_pos = (1.0/60.0).ease_linear(self.camera_pos, world.player_pos);

		{
			let aspect = self.screen_size.x as f32 / self.screen_size.y as f32;
			let extent = Vec2::new(self.camera_zoom*aspect + 1.0, self.camera_zoom + 1.0);

			let bottom = (self.camera_pos - extent).to_vec2i();
			let top = (self.camera_pos + extent).to_vec2i();

			let player_info = world.tile_set.get_tile_info_by_name("player_idle")
				.expect("missing player sprite");
			
			let tile_set_lookup = |idx| world.tile_set.get_tile_info(idx as usize);

			for y in bottom.y..top.y {
				for x in bottom.x..top.x {
					let pos = Vec2i::new(x, y);

					if let Some(Some(info)) = world.get_tile(pos).map(&tile_set_lookup) {
						self.draw_tile(info, pos);
					}

					if world.wrap_position(pos) == world.player_pos.to_vec2i() {
						self.draw_tile(player_info, pos);
					}
				}
			}
		}

		self.builder.upload_to(&mut self.mesh);
		self.builder.clear();

		self.mesh.bind();
		self.mesh.draw(gl::TRIANGLES);
	}

	fn draw_tile_rotated(&mut self, tile_info: &TileInfo, pos: Vec2i, rot: u32) {
		self.builder.draw_tiled_rotated(tile_info.texel_offset, tile_info.texel_size, pos.to_vec2(), rot);
	}

	fn draw_tile(&mut self, tile_info: &TileInfo, pos: Vec2i) {
		self.draw_tile_rotated(tile_info, pos, 0);
	}
}


mod tile_flags {
	pub const ALLOWS_Z_MOVE: u32 = 1<<0;
	pub const BLOCKS_MOVE: u32 = 1<<1;
}

struct TileInfo {
	name: &'static str,
	texel_offset: Vec2i,
	texel_size: Vec2i,
	flags: u32,
}

impl TileInfo {
	fn allows_z_move(&self) -> bool {
		self.flags & tile_flags::ALLOWS_Z_MOVE > 0
	}
}

struct TileSet {
	tile_infos: Vec<TileInfo>,
}

impl TileSet {
	fn get_tile_info(&self, index: usize) -> Option<&TileInfo> {
		if index == 0 {
			None
		} else {
			self.tile_infos.get(index-1)
		}
	}

	fn get_tile_info_by_name(&self, name: &str) -> Option<&TileInfo> {
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

	fn wrap_position(&self, pos: Vec2i) -> Vec2i {
		Vec2i::new(
			(pos.x%self.size.x + self.size.x) % self.size.x,
			(pos.y%self.size.y + self.size.y) % self.size.y)
	}
}

struct World {
	// TODO: some way to chunk larger tilemaps
	// Vec<Box<TileMapTrait>>?
	layers: Vec<TileMap>,
	tile_set: TileSet,

	player_pos: Vec2,
	player_layer: i32,
}

impl World {
	fn new(tile_set: TileSet) -> Self {
		let mut layers = Vec::new();

		for i in 0..6 {
			let world_size = Vec2i::splat(World::layer_size(i));

			layers.push(TileMap::new(world_size));
		}

		let player_layer = layers.len() as i32 - 1;

		World {
			layers,
			tile_set,

			player_pos: Vec2::new(2.5, 2.5),
			player_layer,
		}
	}

	fn layer_size(layer: i32) -> i32 { 2i32.pow(layer as u32 + 2) }

	fn shift_layer(&mut self, dir: i32) -> bool {
		let new_layer = self.player_layer + dir;
		if new_layer < 0 || new_layer >= self.layers.len() as _ { return false }

		self.player_layer = new_layer;
		self.player_pos = self.player_pos * Vec2::splat(2.0f32.powi(dir));
		true
	}

	fn move_player(&mut self, dir: Vec2i) -> bool {
		self.player_pos = self.player_pos + dir.to_vec2();
		let layer_size = World::layer_size(self.player_layer) as f32;

		let mut did_warp = false;

		if self.player_pos.x < 0.0 { self.player_pos.x += layer_size; did_warp = true; }
		if self.player_pos.x >= layer_size { self.player_pos.x -= layer_size; did_warp = true; }

		if self.player_pos.y < 0.0 { self.player_pos.y += layer_size; did_warp = true; }
		if self.player_pos.y >= layer_size { self.player_pos.y -= layer_size; did_warp = true; }

		did_warp
	}

	fn set_tile(&mut self, pos: Vec2i, value: u8) {
		let pos = self.wrap_position(pos);
		self.layers[self.player_layer as usize].set_tile(pos, value);
	}

	fn get_tile(&self, pos: Vec2i) -> Option<u8> {
		let pos = self.wrap_position(pos);
		self.layers[self.player_layer as usize].get_tile(pos)
	}

	fn get_tile_below(&self, pos: Vec2i) -> Option<u8> {
		if self.player_layer <= 0 {
			return None
		}

		let pos = self.wrap_position(pos);
		let pos = Vec2i::new(pos.x/2, pos.y/2);

		let next_layer = (self.player_layer-1) as usize;
		let layer = &self.layers[next_layer];

		layer.get_tile(pos)
	}

	fn get_tiles_above(&self, pos: Vec2i) -> Option<[u8; 4]> {
		if self.player_layer + 1 >= self.layers.len() as _ {
			return None
		}

		let pos = self.wrap_position(pos);
		let pos = Vec2i::new(pos.x*2, pos.y*2);

		let layer = &self.layers[(self.player_layer+1) as usize];

		let mut tiles = [0u8; 4];
		let poss = [
			pos + Vec2i::new(0, 0),
			pos + Vec2i::new(0, 1),
			pos + Vec2i::new(1, 1),
			pos + Vec2i::new(1, 0),
		];

		for (p, t) in poss.iter().zip(tiles.iter_mut()) {
			*t = layer.get_tile(*p)?;
		}

		Some(tiles)
	}

	fn get_tile_info(&self, pos: Vec2i) -> Option<&TileInfo> {
		self.get_tile(pos).iter()
			.filter_map(|&idx| self.tile_set.get_tile_info(idx as usize))
			.next()
	}
	
	fn wrap_position(&self, pos: Vec2i) -> Vec2i {
		self.layers[self.player_layer as usize].wrap_position(pos)
	}
}


trait MeshBuilderTileExtension {
	fn draw_tiled_rotated(&mut self, texel_offset: Vec2i, texel_size: Vec2i, pos: Vec2, rot: u32);
	fn draw_tiled(&mut self, texel_offset: Vec2i, texel_size: Vec2i, pos: Vec2);
}

impl MeshBuilderTileExtension for MeshBuilder<Vert2D> {
	fn draw_tiled_rotated(&mut self, texel_offset: Vec2i, texel_size: Vec2i, pos: Vec2, rot: u32) {
		let base_uv = texel_offset.to_vec2() * TEXEL_FACTOR;
		let uv_size = texel_size.to_vec2() * TEXEL_FACTOR;
		let size = texel_size.to_vec2() / Vec2::splat(TEXELS_PER_TILE as f32);

		let rot = rot as usize;

		let uvs = [
			base_uv + Vec2::new(0.01, 0.98) * uv_size,
			base_uv + Vec2::new(0.01, 0.01) * uv_size,
			base_uv + Vec2::new(0.98, 0.01) * uv_size,
			base_uv + Vec2::new(0.98, 0.98) * uv_size,
		];

		self.add_quad(&[
			Vert2D(Vec2::new(0.0, 0.0)*size + pos, uvs[(rot+0)%4]),
			Vert2D(Vec2::new(0.0, 1.0)*size + pos, uvs[(rot+1)%4]),
			Vert2D(Vec2::new(1.0, 1.0)*size + pos, uvs[(rot+2)%4]),
			Vert2D(Vec2::new(1.0, 0.0)*size + pos, uvs[(rot+3)%4]),
		]);
	}

	fn draw_tiled(&mut self, texel_offset: Vec2i, texel_size: Vec2i, pos: Vec2) {
		self.draw_tiled_rotated(texel_offset, texel_size, pos, 0);
	}
}