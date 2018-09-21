use vec::Vec2f;

pub enum Toggle {
	Start, Stop
}

pub enum Event {
	WalkForward(Toggle),
	WalkLeft(Toggle),
	WalkRight(Toggle),
	WalkBack(Toggle),
	CursorMove(Vec2f), // not triggered in the normal game, only in menu
	CamRotate(Vec2f),
}
