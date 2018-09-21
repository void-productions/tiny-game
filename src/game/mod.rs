mod render;
mod physics;
pub mod frame;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use winit::WindowEvent;

use cycle::Cycle;
use self::frame::Frame;
use self::render::Render;
use self::physics::Physics;
use mapper::Mapper;
use vec::Vec2f;

lazy_static! {
	static ref FRAME_PERIOD: Duration = Duration::new(0, 200 * 1000 * 1000);
}

fn physics_loop(mut frame: Frame, mut physics: Physics, frame_sender: Sender<Frame>, key_event_receiver: Receiver<WindowEvent>) {
    let mapper = Mapper::new(Vec2f::new(0., 0.));

	for x in Cycle::new(*FRAME_PERIOD) {
        // wait for the next tick time
		x.prepare();

		loop {
			match key_event_receiver.try_recv() {
				Ok(window_event) => {
                    if let Some(action_event) = mapper.convert(window_event, false) {
                        frame.apply_action(action_event);
                    }
                },
				Err(TryRecvError::Empty) => break,
				Err(TryRecvError::Disconnected) => return,
			}
		}

		frame = physics.tick(&frame);

		// send frame
		if frame_sender.send(frame.clone()).is_err() {
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

	let (frame_sender, frame_receiver) = channel();
    let (window_event_sender, window_event_receiver) = channel();

	let tmp_frame = frame.clone();

	let _physics_thread = thread::spawn(move || {
		physics_loop(tmp_frame, physics, frame_sender, window_event_receiver);
	});

	loop {
		loop {
			match frame_receiver.try_recv() {
				Ok(f) => { frame = f; },
				Err(TryRecvError::Empty) => break,
				Err(TryRecvError::Disconnected) => return,
			}
		}

        render.handle_events(&window_event_sender);

		// render
		render.render(&frame);
		if render.should_close() {
			break;
		}
	}
}
