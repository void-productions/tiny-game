use frame::Frame;
use vec::Vec3f;

pub struct PolygonFrame {
	camera_position: Vec3f,
	jaw: f32,
	pitch: f32,
	chunk: Chunk,
}

pub enum Chunk {
	Recursive([Box<Chunk>; 8]),
	Concrete {
		points: Vec<Vec3f>,
		triangle_refs: Vec<[usize; 3]>,
	},
}

impl PolygonFrame {
	pub fn build(frame: &Frame) -> PolygonFrame { unimplemented!() }
	pub fn update(&mut self, frame: &Frame) { unimplemented!() }
}
