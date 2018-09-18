use game::frame::Frame;
use winit::{EventsLoop, Window, CreationError};

#[derive(Debug)]
pub enum RenderCreateError {
	CreationError(CreationError),
}

pub struct Render {
	events_loop: EventsLoop,
	window: Window,
}

impl Render {
	pub fn create() -> Result<Self, RenderCreateError> {
		let events_loop = EventsLoop::new();
		let window = Window::new(&events_loop)
					.map_err(|e| RenderCreateError::CreationError(e))?;

		let render = Render { events_loop, window };

		Ok(render)
	}

	pub fn render(&mut self, frame: &Frame) {
		// TODO
	}
}
