use winit::{ElementState, WindowEvent, VirtualKeyCode, KeyboardInput};

use action::Action;
use action::Toggle;
use vec::Vec2f;

const WALK_FORWARD_KEY: VirtualKeyCode = VirtualKeyCode::W;
const WALK_LEFT_KEY: VirtualKeyCode = VirtualKeyCode::A;
const WALK_BACK_KEY: VirtualKeyCode = VirtualKeyCode::S;
const WALK_RIGHT_KEY: VirtualKeyCode = VirtualKeyCode::D;

pub struct Mapper {
	mouse_position: Vec2f,
}

impl Mapper {
	pub fn new(mouse_position: Vec2f) -> Mapper {
		Mapper { mouse_position }
	}

	pub fn convert(&self, ev: WindowEvent, menu: bool) -> Option<Action> {
		let f = |x| {
			match x {
				ElementState::Pressed => Toggle::Start,
				ElementState::Released => Toggle::Stop,
			}
		};

		match ev {
			WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(WALK_FORWARD_KEY), state, .. }, .. } => Some(Action::WalkForward(f(state))),
			WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(WALK_LEFT_KEY), state, .. }, .. } => Some(Action::WalkLeft(f(state))),
			WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(WALK_BACK_KEY), state, .. }, .. } => Some(Action::WalkBack(f(state))),
			WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(WALK_RIGHT_KEY), state, .. }, .. } => Some(Action::WalkRight(f(state))),
			WindowEvent::CursorMoved { position, .. } => {
				let position = Vec2f::new(position.x as f32, position.y as f32);
				let delta_position = position - self.mouse_position;

				if menu {
					Some(Action::CursorMove(delta_position))
				} else {
					Some(Action::CamRotate(delta_position))
				}
			},
			_ => None,
		}
	}
}
