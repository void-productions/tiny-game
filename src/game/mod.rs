mod render;
mod physics;
mod frame;
mod frame_wrapper;
mod entity;

use self::render::Render;
use self::physics::Physics;
use self::frame_wrapper::FrameWrapper;
use std::thread;
use std::time::Duration;
use cycle::Cycle;

pub struct Game {
	render: Render,
	physics: Physics,
	frame_wrapper: FrameWrapper,
}

fn physics_loop() {
	for x in Cycle::new(Duration::new(0, 200000000)) {
		x.prepare();
		println!("tick");
	}
}

impl Game {
	pub fn new() -> Game {
		Game {
			render: Render,
			physics: Physics,
			frame_wrapper: FrameWrapper::new(),
		}
	}

	pub fn run(&self) {
		let physics_thread = thread::spawn(physics_loop);
		loop {
			thread::sleep(Duration::new(0, 50));
		}
	}
}
