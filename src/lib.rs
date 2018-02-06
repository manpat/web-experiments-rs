#![feature(generators, generator_trait, box_syntax)]
#![feature(inclusive_range_syntax)]
#![feature(specialization)]
#![feature(ord_max_min)]
#![feature(link_args)]
#![feature(const_fn)]

extern crate web_common;
pub use web_common::*;

pub mod console;
pub mod particle;

pub use particle::*;

#[macro_export]
macro_rules! asset {
	($expr:expr) => {{
		include_str!(concat!("../../assets/", $expr))
	}}
}

#[macro_export]
macro_rules! bin_asset {
	($expr:expr) => {{
		include_bytes!(concat!("../../assets/", $expr))
	}}
}

pub fn screen_point_to_gl(screen_size: Vec2i, point: Vec2i) -> Vec2 {
	let sz = screen_size.to_vec2();
	let aspect = sz.x as f32 / sz.y as f32;

	let norm = point.to_vec2() / sz * 2.0 - Vec2::splat(1.0);
	norm * Vec2::new(aspect, -1.0)
}