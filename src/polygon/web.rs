use std::collections::HashMap;

use frame::Frame;
use vec::{Vec3u, Vec3f};

use super::BASE_CHUNK_SIZE;
use super::chunk::Chunk;

pub struct ChunkWeb {
	chunks: HashMap<Vec3u, Chunk>,
}

impl ChunkWeb {
	pub fn build(frame: &Frame, camera: Vec3f) -> ChunkWeb {
		let mut chunks = HashMap::new();
		for x in ChunkWeb::get_relevant_sources(camera).into_iter() {
			chunks.insert(x, Chunk::build(frame, x.map(|u| (u as f32) * BASE_CHUNK_SIZE), BASE_CHUNK_SIZE, camera));
		}
		ChunkWeb { chunks }
		
	}

	pub fn update(&mut self, frame: &Frame, new_camera: Vec3f, old_camera: Vec3f) {
		// TODO:
		// - detect newly combined Chunks
		// - detect newly splat Chunks
		// - detect new base chunks
		// - detect deprecated base chunks
		// - apply these changes
	}

	fn get_relevant_sources(camera: Vec3f) -> Vec<Vec3u> {
		let mut v: Vec<Vec3u> = Vec::new();

		let min = find_source(camera - BASE_CHUNK_SIZE);
		let max = find_source(camera + BASE_CHUNK_SIZE);

		for x in min.x..=max.x {
			for y in min.y..=max.y {
				for z in min.z..=max.z {
					let moved_pos = Vec3u::new(x, y, z);
					if !v.contains(&moved_pos) {
						v.push(moved_pos);
					}
				}
			}
		}

		v
	}
}

fn find_source(p: Vec3f) -> Vec3u {
	Vec3u::new (
		(p.x / BASE_CHUNK_SIZE) as u32,
		(p.y / BASE_CHUNK_SIZE) as u32,
		(p.z / BASE_CHUNK_SIZE) as u32,
	)
}

