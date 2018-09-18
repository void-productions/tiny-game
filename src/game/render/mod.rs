use game::frame::Frame;
use winit::{EventsLoop, Window, CreationError};

#[derive(Debug)]
pub enum RenderCreateError {
	CreationError(CreationError),
}

pub struct Render {
	events_loop: EventsLoop,
}

impl Render {
	pub fn create() -> Result<Self, RenderCreateError> {
		let render = Render {
			events_loop: EventsLoop::new(),
		};

		let _window = Window::new(&render.events_loop)
			.map_err(|e| RenderCreateError::CreationError(e))?;

		Ok(render)
	}

	pub fn render(&mut self, frame: &Frame) {
		// TODO
	}
}
