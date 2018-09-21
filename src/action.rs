use vec::Vec2f;

#[derive(Clone, Copy)]
pub enum Toggle {
	Start, Stop
}

#[derive(Clone, Copy)]
pub enum Action {
	WalkForward(Toggle),
	WalkLeft(Toggle),
	WalkRight(Toggle),
	WalkBack(Toggle),
	CursorMove(Vec2f), // not triggered in the normal game, only in menu
	CamRotate(Vec2f),
}

impl Toggle {
    pub fn to_bool(self) -> bool {
        match self {
            Toggle::Start => true,
            Toggle::Stop => false,
        }
    }
}
