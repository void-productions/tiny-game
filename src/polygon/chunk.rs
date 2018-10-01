use frame::Frame;
use vec::{Vec3f, Vec3i};

pub enum Chunk {
	Recursive(Vec<Chunk>), // these should be 8 elements in the Vec
	Concrete {
		points: Vec<Vec3f>,
		triangle_refs: Vec<[usize; 3]>,
	},
}

impl Chunk {
	pub fn build(frame: &Frame, source: Vec3i, chunk_size: u32, camera: Vec3f) -> Chunk {
		let center = source + (chunk_size/2) as i32;
		let dist = (camera - center.map(|x| x as f32)).magnitude();
		if Chunk::should_split(chunk_size, dist) {
			Chunk::Recursive((0..8).map(|i| {
				let source = Chunk::calculate_subsource(source, chunk_size, i);
				Chunk::build(frame, source, chunk_size/2, camera)
			}).collect())
		} else {
			Chunk::build_concrete(frame, source, chunk_size, camera)
		}
	}

	fn build_concrete(frame: &Frame, source: Vec3i, chunk_size: u32, camera: Vec3f) -> Chunk {
		unimplemented!()
		// TODO
	}

	fn should_split(chunk_size: u32, dist: f32) -> bool {
		unimplemented!()
		// TODO
	}

	fn calculate_subsource(source: Vec3i, chunk_size: u32, i: u8) -> Vec3i {
		let half = chunk_size as i32/2;
		(match i {
			0 => Vec3i::new(0,    0,    0),
			1 => Vec3i::new(0,    half, 0),
			2 => Vec3i::new(0,    0,    half),
			3 => Vec3i::new(0,    half, half),
			4 => Vec3i::new(half, 0,    0),
			5 => Vec3i::new(half, half, 0),
			6 => Vec3i::new(half, 0,    half),
			7 => Vec3i::new(half, half, half),
			_ => panic!("invalid i")
		}) + source
	}
}
