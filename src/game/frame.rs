use action::Action;
use vec::Vec2f;

#[derive(Clone, Debug)]
pub struct Player {
	position: Vec2f,
	jaw: f32,
	pitch: f32,

    walking_forward: bool,
    walking_back: bool,
    walking_left: bool,
    walking_right: bool,
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
}
