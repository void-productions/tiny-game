use game::entity::Entity;

pub struct Frame {
	entities: Vec<Entity>,
}

impl Frame {
	pub fn new() -> Frame {
		Frame {
			entities: Vec::new(),
		}
	}
}
