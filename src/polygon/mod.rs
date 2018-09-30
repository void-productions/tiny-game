mod chunk;
mod web;

use frame::Frame;
use vec::Vec3f;

use self::web::ChunkWeb;

// A Chunk is a cube, it's width = height = length is called size
// There is an infinite imaginary grid in the polygon world, the length of a grid cell is defined to be 1
// The minimal chunk has the size of one grid cell, therefore it's size is 1
// The source of a chunk is the Vec3i with the lowest coordinates, which is still within the chunk

const BASE_CHUNK_SIZE: u32 = 32;
const VISION_RANGE: f32 = 100.;

pub struct PolygonFrame {
	camera_position: Vec3f,
	jaw: f32,
	pitch: f32,
	chunk_web: ChunkWeb,
}

impl PolygonFrame {
	pub fn build(frame: &Frame) -> PolygonFrame {
		let camera_position = PolygonFrame::get_camera(frame);

		PolygonFrame {
			camera_position,
			jaw: frame.player.jaw,
			pitch: frame.player.pitch,
			chunk_web: ChunkWeb::build(frame, camera_position),
		}
	}

	pub fn update(&mut self, frame: &Frame) {
		let camera_position = PolygonFrame::get_camera(frame);

		self.chunk_web.update(frame, camera_position, self.camera_position);

		self.camera_position = camera_position;
		self.jaw = frame.player.jaw;
		self.pitch = frame.player.pitch;
	}

	fn get_camera(frame: &Frame) -> Vec3f {
		let x = frame.player.position_x;
		let z = frame.player.position_z;
		let y = frame.get_height(x, z);

		Vec3f::new(x, y, z)
	}
}
