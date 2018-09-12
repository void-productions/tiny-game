mod render;
mod physics;
mod frame;
mod entity;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Receiver, TryRecvError};

use cycle::Cycle;
use self::frame::Frame;
use self::render::Render;
use self::physics::Physics;

lazy_static! {
	static ref FRAME_PERIOD: Duration = Duration::new(0, 200 * 1000 * 1000);
}

fn render_loop(mut frame: Frame, mut render: Render, receiver: Receiver<Frame>) {
	loop {
		// read channel and update frame
		loop {
			match receiver.try_recv() {
				Ok(f) => { frame = f; },
				Err(TryRecvError::Empty) => break,
				Err(TryRecvError::Disconnected) => return,
			}
		}

		// render
		render.render(&frame);
	}
}

pub fn run(mut frame: Frame) {
	let mut physics = Physics::new();
	let render = Render::new();

	let (sender, receiver) = channel();

	let tmp_frame = frame.clone();

	let render_thread = thread::spawn(move || {
		render_loop(tmp_frame, render, receiver);
	});

	for x in Cycle::new(*FRAME_PERIOD) {
		x.prepare();

		frame = physics.tick(&frame);

		// send frame
		sender.send(frame.clone());
	}
}
