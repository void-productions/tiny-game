use std::collections::HashMap;

use frame::Frame;
use vec::{Vec3i, Vec3f};

use super::BASE_CHUNK_SIZE;
use super::chunk::Chunk;

pub struct ChunkWeb {
	chunks: HashMap<Vec3i, Chunk>,
}

impl ChunkWeb {
	pub fn build(frame: &Frame, camera: Vec3f) -> ChunkWeb {
		let mut chunks = HashMap::new();
		for x in ChunkWeb::get_base_chunk_sources(camera).into_iter() {
			chunks.insert(x, Chunk::build(frame, x * BASE_CHUNK_SIZE, BASE_CHUNK_SIZE, camera));
		}
		ChunkWeb { chunks }
		
	}

	pub fn update(&mut self, frame: &Frame, new_camera: Vec3f, old_camera: Vec3f) {
		let old_base_chunk_sources = ChunkWeb::get_base_chunk_sources(old_camera);
		let new_base_chunk_sources = ChunkWeb::get_base_chunk_sources(new_camera);
		// TODO create new base chunks
		// TODO remove obsolete base chunks
		
		// TODO:
		// - detect newly combined Chunks
		// - detect newly splat Chunks
		// - apply these changes
	}

	// gives sorted output
	fn get_base_chunk_sources(camera: Vec3f) -> Vec<Vec3i> {
		unimplemented!()
		// TODO
	}
}
