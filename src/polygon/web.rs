use std::collections::HashMap;

use frame::Frame;
use vec::{Vec3i, Vec3f};

use super::{BASE_CHUNK_SIZE, VISION_RANGE};
use super::chunk::Chunk;

pub struct ChunkWeb {
	chunks: HashMap<Vec3i, Chunk>,
}

impl ChunkWeb {
	pub fn build(frame: &Frame, camera: Vec3f) -> ChunkWeb {
		let mut chunks = HashMap::new();
		for x in ChunkWeb::get_environment_base_chunk_sources(camera).into_iter() {
			chunks.insert(x, Chunk::build(frame, x, BASE_CHUNK_SIZE, camera));
		}
		ChunkWeb { chunks }
		
	}

	pub fn update(&mut self, frame: &Frame, new_camera: Vec3f, old_camera: Vec3f) {
		let old_base_chunk_sources = ChunkWeb::get_environment_base_chunk_sources(old_camera);
		let new_base_chunk_sources = ChunkWeb::get_environment_base_chunk_sources(new_camera);
		// TODO create new base chunks
		// TODO remove obsolete base chunks

		// TODO:
		// - detect newly combined Chunks
		// - detect newly splat Chunks
		// - apply these changes
	}

	fn round(v: Vec3f) -> Vec3i {
		let b = BASE_CHUNK_SIZE as f32;
		v.map(|x| {
			((x / b).floor() * b) as i32
		})
	}

	// gives sorted output
	fn get_environment_base_chunk_sources(camera: Vec3f) -> Vec<Vec3i> {
		let bi = BASE_CHUNK_SIZE as i32;

		let one = Vec3f::with(VISION_RANGE);
		let minf = camera - one;
		let maxf = camera + one;

		let min = ChunkWeb::round(minf) / bi;
		let max = ChunkWeb::round(maxf) / bi;

		let mut v = Vec::new();

		for x in min.x..=max.x {
			for y in min.y..=max.y {
				for z in min.z..=max.z {
					v.push(Vec3i::new(x, y, z) * bi);
				}
			}
		}

		v
	}
}
