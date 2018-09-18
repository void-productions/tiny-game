use game::frame::Frame;
use winit::{EventsLoop, Window, WindowBuilder, dpi::LogicalSize, CreationError};

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
		let window = WindowBuilder::new()
            .with_title("Tiny Game...")
            .with_dimensions(LogicalSize::new(800., 600.))
            .build(&events_loop)
            .map_err(|e| RenderCreateError::CreationError(e))?;

		let render = Render { events_loop, window };

		Ok(render)
	}

	pub fn render(&mut self, frame: &Frame) {
		// TODO
	}
}
