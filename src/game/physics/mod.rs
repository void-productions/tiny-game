use game::frame::Frame;

pub struct Physics;

impl Physics {
	pub fn new() -> Physics {
		Physics
	}

	pub fn tick(&mut self, frame: &Frame) -> Frame {
		unimplemented!();
	}
}
