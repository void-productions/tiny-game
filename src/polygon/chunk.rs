use frame::Frame;
use vec::{Vec3f, Vec3i};

pub enum Chunk {
	Recursive([Box<Chunk>; 8]),
	Concrete {
		points: Vec<Vec3f>,
		triangle_refs: Vec<[usize; 3]>,
	},
}

impl Chunk {
	pub fn build(frame: &Frame, source: Vec3i, chunk_size: u32, camera: Vec3f) -> Chunk {
		// TODO
		unimplemented!()
	}
}
