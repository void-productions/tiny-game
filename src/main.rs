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
mod game;
mod cycle;

use game::frame::Frame;

fn main() {
	let frame = Frame::new();
	game::run(frame);
}
