use game::entity::Entity;

#[derive(Clone)]
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
