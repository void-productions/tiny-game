mod render;
mod physics;
pub mod frame;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, TryRecvError};

use cycle::Cycle;
use self::frame::Frame;
use self::render::Render;
use self::physics::Physics;

lazy_static! {
	static ref FRAME_PERIOD: Duration = Duration::new(0, 200 * 1000 * 1000);
}

fn physics_loop(mut frame: Frame, mut physics: Physics, sender: Sender<Frame>) {
	for x in Cycle::new(*FRAME_PERIOD) {
		x.prepare();

		frame = physics.tick(&frame);

		// send frame
		if sender.send(frame.clone()).is_err() {
			break;
		}
	}
}

pub fn run(mut frame: Frame) {
	let physics = Physics::new();

	let mut render = match Render::create() {
		Ok(r) => r,
		Err(e) => panic!("{:?}", e),
	};

	let (sender, receiver) = channel();

	let tmp_frame = frame.clone();

	let _physics_thread = thread::spawn(move || {
		physics_loop(tmp_frame, physics, sender);
	});

	loop {
		loop {
			match receiver.try_recv() {
				Ok(f) => { frame = f; },
				Err(TryRecvError::Empty) => break,
				Err(TryRecvError::Disconnected) => return,
			}
		}

		// render
		render.render(&frame);
		if render.should_close() {
			break;
		}
	}
}
