use std::time::Duration;
use std::time::SystemTime;
use std::iter::Iterator;
use std::thread::sleep;

pub struct Cycle {
	start_time: SystemTime,
	period: Duration,
	frame_counter: u32,
}

pub enum TimeDiff {
	Ahead(Duration),
	Behind(Duration),
}

impl Cycle {
	pub fn new(period: Duration) -> Cycle {
		Cycle {
			start_time: SystemTime::now(),
			frame_counter: 0,
			period,
		}
	}
}

impl Iterator for Cycle {
	type Item = TimeDiff;

	fn next(&mut self) -> Option<TimeDiff> {
		let target_time: SystemTime = self.start_time + self.period * self.frame_counter;
		let current_time: SystemTime = SystemTime::now();

		if let Some(tmp_frame_counter) = self.frame_counter.checked_add(1) {
			self.frame_counter = tmp_frame_counter;
		} else {
			return None;
		}

		if target_time <= current_time {
			Some(TimeDiff::Behind(current_time.duration_since(target_time).unwrap()))
		} else {
			Some(TimeDiff::Ahead(target_time.duration_since(current_time).unwrap()))
		}
	}
}

impl TimeDiff {
	pub fn prepare(&self) {
		if let TimeDiff::Ahead(x) = self {
			sleep(*x);
		}
	}
}
