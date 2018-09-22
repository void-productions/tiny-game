use frame::Frame;

pub struct Physics;

impl Physics {
	pub fn new() -> Physics {
		Physics
	}

	pub fn tick(&mut self, frame: &Frame) -> Frame {
		let mut frame = frame.clone();

		// TODO fix
		if frame.player.walking_left { frame.player.position_x -= 1.; }
		if frame.player.walking_right { frame.player.position_x += 1.; }

		if frame.player.walking_forward { frame.player.position_z += 1.; }
		if frame.player.walking_back { frame.player.position_z -= 1.; }

		frame
	}
}
