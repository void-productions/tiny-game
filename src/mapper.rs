use action::Action;
use input;
use input::Key;

const WALK_FORWARD_KEY: Key = Key::W;
const WALK_LEFT_KEY: Key = Key::A;
const WALK_BACK_KEY: Key = Key::S;
const WALK_RIGHT_KEY: Key = Key::D;

pub fn convert(ev: input::Event, menu: bool) -> Option<Action> {
	match ev {
		input::Event::Key { toggle, key: WALK_FORWARD_KEY } => Some(Action::WalkForward(toggle)),
		input::Event::Key { toggle, key: WALK_LEFT_KEY } => Some(Action::WalkLeft(toggle)),
		input::Event::Key { toggle, key: WALK_BACK_KEY } => Some(Action::WalkBack(toggle)),
		input::Event::Key { toggle, key: WALK_RIGHT_KEY } => Some(Action::WalkRight(toggle)),
		input::Event::CursorMove(cursor_change) => {
			if menu {
				Some(Action::CursorMove(cursor_change))
			} else {
				Some(Action::CamRotate(cursor_change))
			}
		},
		_ => None,
	}
}
