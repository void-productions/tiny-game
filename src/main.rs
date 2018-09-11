#![deny(bare_trait_objects)]
#![feature(nll)]

mod game;
mod cycle;

use game::Game;

fn main() {
	let game: Game = Game::new();
	game.run();
}
