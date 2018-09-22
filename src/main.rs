#![deny(bare_trait_objects)]
#![feature(nll)]

#[macro_use]
extern crate lazy_static;
extern crate simdnoise;
extern crate winit;

mod vec;
mod action;
mod mapper;
mod toggle;
mod input;
mod cycle;
mod render;
mod window_handler;
mod core;
mod physics;
mod frame;

use frame::Frame;

fn main() {
	let frame = Frame::new();
	core::run(frame);
}
