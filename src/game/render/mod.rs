use game::frame::Frame;
use winit::{WindowEvent, Event, EventsLoop, Window, WindowBuilder, dpi::LogicalSize, dpi::LogicalPosition, CreationError};
use std::sync::mpsc::Sender;

use input;
use vec::Vec2f;

#[derive(Debug)]
pub enum RenderCreateError {
	CreationError(CreationError),
}

pub struct Render {
	events_loop: EventsLoop,
	window: Window,
	should_close: bool,
    mouse_position: Vec2f,
}

impl Render {
	pub fn create() -> Result<Self, RenderCreateError> {
		let events_loop = EventsLoop::new();
		let mut window = WindowBuilder::new()
            .with_title("Tiny Game...")
            .with_dimensions(LogicalSize::new(800., 600.))
            .build(&events_loop)
            .map_err(|e| RenderCreateError::CreationError(e))?;

		let c = get_center(&window);
		set_mouse_position(&mut window, c);
		let render = Render { events_loop, window, should_close: false, mouse_position: c };

		Ok(render)
	}

    pub fn handle_events(&mut self, input_event_sender: &Sender<input::Event>) {
		let mut events = Vec::new();

		self.events_loop.poll_events(|ev| { events.push(ev); });

		for ev in events.into_iter() {
			if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = ev { self.should_close = true; }
            if let Event::WindowEvent { event: window_event, .. } = ev {
				if let Some(input_event) = self.convert_event(window_event) {
					input_event_sender.send(input_event).expect("input event receiver is closed!");
				}
            }
		}

		self.center_mouse();
    }

	pub fn render(&mut self, frame: &Frame) {
		// TODO
	}

	pub fn should_close(&self) -> bool {
		self.should_close
	}

	fn convert_event(&self, window_event: WindowEvent) -> Option<input::Event> {
		input::Event::from_window_event(&window_event, self.mouse_position)
	}

	fn center_mouse(&mut self) {
		center_mouse(&mut self.window);
		self.mouse_position = get_center(&self.window);
	}
}

fn set_mouse_position(window: &mut Window, position: Vec2f) {
	window.set_cursor_position(position.into());
}

fn center_mouse(window: &mut Window) {
	set_mouse_position(window, get_center(window));
}

fn get_size(window: &Window) -> Vec2f {
	// TODO make better :)
	window.get_inner_size().unwrap().into()
}

fn get_center(window: &Window) -> Vec2f {
	get_size(window) / 2.
}

impl From<LogicalSize> for Vec2f {
	fn from(logical_size: LogicalSize) -> Vec2f {
		Vec2f::new(logical_size.width as f32, logical_size.height as f32)
	}
}

impl From<LogicalPosition> for Vec2f {
	fn from(logical_position: LogicalPosition) -> Vec2f {
		Vec2f::new(logical_position.x as f32, logical_position.y as f32)
	}
}

impl From<Vec2f> for LogicalSize {
	fn from(vec: Vec2f) -> LogicalSize {
		LogicalSize::new(vec.x as f64, vec.y as f64)
	}
}

impl From<Vec2f> for LogicalPosition {
	fn from(vec: Vec2f) -> LogicalPosition {
		LogicalPosition::new(vec.x as f64, vec.y as f64)
	}
}
