#![deny(bare_trait_objects)]
#![feature(nll)]

#[macro_use]
extern crate lazy_static;

mod game;
mod cycle;

fn main() {
	let frame = unimplemented!();
	game::run(frame);
}
