#[derive(Clone)]
struct Player {
	position: (f32, f32),
	jaw: f32,
	pitch: f32,
}

#[derive(Clone)]
pub struct Frame {
	player: Player,
}

impl Player {
	fn new() -> Player {
		Player {
			position: (0., 0.),
			jaw: 0.,
			pitch: 0.,
		}
	}
}

impl Frame {
	pub fn new() -> Frame {
		Frame {
			player: Player::new(),
		}
	}
}
