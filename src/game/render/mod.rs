use game::frame::Frame;
use winit::{WindowEvent, Event, EventsLoop, Window, WindowBuilder, dpi::LogicalSize, CreationError};

#[derive(Debug)]
pub enum RenderCreateError {
	CreationError(CreationError),
}

pub struct Render {
	events_loop: EventsLoop,
	window: Window,
	pub should_close: bool,
}

impl Render {
	pub fn create() -> Result<Self, RenderCreateError> {
		let events_loop = EventsLoop::new();
		let window = WindowBuilder::new()
            .with_title("Tiny Game...")
            .with_dimensions(LogicalSize::new(800., 600.))
            .build(&events_loop)
            .map_err(|e| RenderCreateError::CreationError(e))?;

		let render = Render { events_loop, window, should_close: false };
		Ok(render)
	}

	pub fn render(&mut self, frame: &Frame) {
		let mut sc = false;
		self.events_loop.poll_events(|ev| {
			match ev {
				Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => sc = true,
				_ => (),
			}
		});

		if sc { self.should_close = true; }
	}

	pub fn should_close(&self) -> bool {
		self.should_close
	}
}
