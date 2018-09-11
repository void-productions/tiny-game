#![deny(bare_trait_objects)]
#![feature(nll)]

#[macro_use]
extern crate lazy_static;

mod game;
mod cycle;

use game::Game;

fn main() {
	let game: Game = Game::new();
	game.run();
}
