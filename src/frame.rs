use simdnoise::{get_2d_noise, NoiseType};

use action::Action;
use vec::Vec2f;

#[derive(Clone)]
pub struct Player {
	pub position: Vec2f,
	pub jaw: f32,
	pub pitch: f32,

    pub walking_forward: bool,
    pub walking_back: bool,
    pub walking_left: bool,
    pub walking_right: bool,
}

#[derive(Clone)]
pub struct Frame {
	pub player: Player,
}

impl Player {
	fn new() -> Player {
		Player {
			position: Vec2f::new(0., 0.),
			jaw: 0.,
			pitch: 0.,

            walking_forward: false,
            walking_back: false,
            walking_right: false,
            walking_left: false,
		}
	}

    fn apply_action(&mut self, action: Action) {
        match action {
            Action::WalkForward(toggle) => self.walking_forward = toggle.to_bool(),
            Action::WalkBack(toggle) => self.walking_back = toggle.to_bool(),
            Action::WalkRight(toggle) => self.walking_right = toggle.to_bool(),
            Action::WalkLeft(toggle) => self.walking_left = toggle.to_bool(),
            Action::CamRotate(mouse_rotation) => {
                self.jaw += mouse_rotation.x;
                self.pitch += mouse_rotation.y;
            },
            _ => {},
        }
    }
}

impl Frame {
	pub fn new() -> Frame {
		Frame {
			player: Player::new(),
		}
	}

    pub fn apply_action(&mut self, action: Action) {
        self.player.apply_action(action);
    }

	pub fn get_height(&self, p: Vec2f) -> f32 {
		let ntype = NoiseType::Normal { freq: 0.01 };
		get_2d_noise(p.x, 1, p.y, 1, ntype).0[0] * 1000.
	}
}
