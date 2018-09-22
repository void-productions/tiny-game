use frame::Frame;

use vec::Vec2f;

pub struct Physics;

impl Physics {
	pub fn new() -> Physics {
		Physics
	}

	pub fn tick(&mut self, frame: &Frame) -> Frame {
		let mut frame = frame.clone();
		let null = Vec2f::new(0., 0.);

		frame.player.position = frame.player.position + // TODO fix
			if frame.player.walking_forward { Vec2f::new(0., 1.) }
			else							{ null } +
			if frame.player.walking_left	{ Vec2f::new(-1., 0.) }
			else							{ null } +
			if frame.player.walking_back	{ Vec2f::new(0., -1.) }
			else							{ null } +
			if frame.player.walking_right	{ Vec2f::new(1., 0.) }
			else							{ null };

		frame
	}
}
