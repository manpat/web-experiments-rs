#![feature(generators, generator_trait, box_syntax)]
#![feature(inclusive_range_syntax)]
#![feature(specialization)]
#![feature(ord_max_min)]
#![feature(link_args)]
#![feature(const_fn)]

extern crate web_common;

pub use resources as res;
pub use web_common::*;

pub mod resources;
pub mod console;
pub mod particle;

pub use particle::*;
