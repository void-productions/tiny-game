use frame::Frame;
use vec::Vec3f;

const BASE_CHUNK_SIZE: f32 = 100.;
const MINIMAL_CHUNK_SIZE: f32 =  10.;
const VISION_RANGE: f32 = 100.;

pub struct PolygonFrame {
	camera_position: Vec3f,
	jaw: f32,
	pitch: f32,
	chunk_web: ChunkWeb,
}

pub struct ChunkWeb {
	chunks: Vec<(Vec3f, Chunk)>, // TODO use better representation
}

pub enum Chunk {
	Recursive([Box<Chunk>; 8]),
	Concrete {
		points: Vec<Vec3f>,
		triangle_refs: Vec<[usize; 3]>,
	},
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
		self.jaw =  frame.player.jaw;
		self.pitch =  frame.player.pitch;
	}

	fn get_camera(frame: &Frame) -> Vec3f {
		let x = frame.player.position_x;
		let z = frame.player.position_z;
		let y = frame.get_height(x, z);

		Vec3f::new(x, y, z)
	}
}

impl ChunkWeb {
	fn build(frame: &Frame, camera: Vec3f) -> ChunkWeb {
		let mut chunks = Vec::new();
		for x in ChunkWeb::get_relevant_sources(camera).into_iter() {
			chunks.push((x, Chunk::build(frame, x, BASE_CHUNK_SIZE, camera)));
		}
		ChunkWeb { chunks }
		
	}

	fn update(&mut self, frame: &Frame, new_camera: Vec3f, old_camera: Vec3f) {
		// TODO:
		// - detect newly combined Chunks
		// - detect newly splat Chunks
		// - detect new base chunks
		// - detect deprecated base chunks
		// - apply these changes
	}

	fn get_relevant_sources(camera: Vec3f) -> Vec<Vec3f> {
		let round = |p: Vec3f| {
			Vec3f::new (
				p.x % BASE_CHUNK_SIZE,
				p.y % BASE_CHUNK_SIZE,
				p.z % BASE_CHUNK_SIZE
			)
		};

		let mut v: Vec<Vec3f> = Vec::new();

		// TODO, this does not work, if VISION > BASE_CHUNK_SIZE
		for &x in [-BASE_CHUNK_SIZE, 0., BASE_CHUNK_SIZE].into_iter() {
			for &y in [-BASE_CHUNK_SIZE, 0., BASE_CHUNK_SIZE].into_iter() {
				for &z in [-BASE_CHUNK_SIZE, 0., BASE_CHUNK_SIZE].into_iter() {
					let moved_pos: Vec3f = round(camera + Vec3f::new(x, y, z));
					if !v.contains(&moved_pos) {
						v.push(moved_pos);
					}
				}
			}
		}

		v
	}
}

impl Chunk {
	fn build(frame: &Frame, from: Vec3f, chunk_size: f32, camera: Vec3f) -> Chunk {
		// TODO
		unimplemented!()
	}
}

