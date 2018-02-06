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
