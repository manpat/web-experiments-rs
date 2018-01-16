#![feature(generators, generator_trait, box_syntax)]
#![feature(inclusive_range_syntax)]
#![feature(specialization)]
#![feature(ord_max_min)]
#![feature(link_args)]
#![feature(const_fn)]

extern crate common;

pub use resources as res;
pub use common::*;

#[macro_use] pub mod bindings;
#[macro_use] pub mod coro_util;

pub mod mut_rc;

pub mod resources;
pub mod rendering;
pub mod console;
pub mod events;
pub mod webgl;

pub mod paper;
pub mod particle;

pub use bindings::emscripten::*;
pub use coro_util::*;
pub use webgl::*;

pub use paper::*;
pub use particle::*;

pub use rendering::gl;
pub use rendering::shader::*;