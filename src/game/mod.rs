mod render;
mod physics;
mod frame;
mod frame_wrapper;
mod entity;

use self::render::Render;
use self::physics::Physics;
use self::frame_wrapper::FrameWrapper;

pub struct Game {
	render: Render,
	physics: Physics,
	frame_wrapper: FrameWrapper,
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
		unimplemented!("hey there");
	}
}
