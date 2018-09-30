use frame::Frame;
use vec::Vec3f;

pub enum Chunk {
	Recursive([Box<Chunk>; 8]),
	Concrete {
		points: Vec<Vec3f>,
		triangle_refs: Vec<[usize; 3]>,
	},
}

impl Chunk {
	pub fn build(frame: &Frame, from: Vec3f, chunk_size: f32, camera: Vec3f) -> Chunk {
		// TODO
		unimplemented!()
	}
}
