pub use winit::VirtualKeyCode as Key;
use winit::{WindowEvent, ElementState, KeyboardInput};

use toggle::Toggle;
use vec::Vec2f;

pub enum Event {
	Key {
		toggle: Toggle,
		key: Key,
	},
	CursorMove(Vec2f),
}

impl Event {
	pub fn from_window_event(window_event: &WindowEvent, mouse_position: Vec2f) -> Option<Event> {
		let f = |x| {
			match x {
				ElementState::Pressed => Toggle::On,
				ElementState::Released => Toggle::Off,
			}
		};

		match window_event {
			WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state, .. }, .. } => Some(Event::Key { toggle: f(*state), key: *key }),
			WindowEvent::CursorMoved { position, .. } => {
				let position = Vec2f::new(position.x as f32, position.y as f32);
				let delta_position = position - mouse_position;

				Some(Event::CursorMove(delta_position))
			},
			_ => None,
		}
	}
}
