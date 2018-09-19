use game::frame::Frame;
use winit::{WindowEvent, Event, EventsLoop, Window, WindowBuilder, dpi::LogicalSize, CreationError};
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub enum RenderCreateError {
	CreationError(CreationError),
}

pub struct Render {
	events_loop: EventsLoop,
	window: Window,
	should_close: bool,
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

    pub fn handle_events(&mut self, window_event_sender: &Sender<WindowEvent>) {
		let mut sc = false;
		self.events_loop.poll_events(|ev| {
			match ev {
				Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => sc = true,
				_ => (),
			}

            if let Event::WindowEvent{event: window_event, ..} = ev {
                window_event_sender.send(window_event).expect("window event receiver is closed!");
            }
		});

		if sc { self.should_close = true; }
    }

	pub fn render(&mut self, frame: &Frame) {
        // TODO
	}

	pub fn should_close(&self) -> bool {
		self.should_close
	}
}
