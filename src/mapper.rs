use winit::{EventState, WindowEvent, VirtualKeyCode};

use action;
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

	pub fn apply(ev: WindowEvent, menu: bool) -> Option<action::Event> {
		let f = |x| {
			match x {
				EventState::Pressed => Toggle::Start,
				EventState::Released => Toggle::Stop,
			}
		};

		match ev {
			WindowEvent::KeyboardInput { virtual_keycode: Some(WALK_FORWARD_KEY), state } => Some(action::Event::WalkForward(f(state))),
			WindowEvent::KeyboardInput { virtual_keycode: Some(WALK_LEFT_KEY), state } => Some(action::Event::WalkLeft(f(state))),
			WindowEvent::KeyboardInput { virtual_keycode: Some(WALK_BACK_KEY), state } => Some(action::Event::WalkBack(f(state))),
			WindowEvent::KeyboardInput { virtual_keycode: Some(WALK_RIGHT_KEY), state } => Some(action::Event::WalkRight(f(state))),
			WindowEvent::CursorMoved { position } => {
				let position = Vec2f::new(position.x as f32, position.y as f32);
				let delta_position = self.mouse_position - position;

				if menu {
					Some(action::Event::CursorMoved(delta_position))
				} else {
					Some(action::Event::CamRotate(delta_position))
				}
			},
			_ => None,
		}
	}
}
