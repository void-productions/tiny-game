use game::frame::Frame;

pub struct FrameWrapper {
	frames: [Frame; 2],
	frame_index: u8,
}

impl FrameWrapper {
	pub fn new() -> FrameWrapper {
		FrameWrapper {
			frames: [Frame::new(), Frame::new()],
			frame_index: 0,
		}
	}

	pub fn get_current(&self) -> &Frame {
		&self.frames[self.frame_index as usize]
	}

	pub fn push(&mut self, frame: Frame) {
		self.frame_index = 1 - self.frame_index;
		self.frames[self.frame_index as usize] = frame;
	}
}
