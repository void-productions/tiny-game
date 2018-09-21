use vec::Vec2u;

pub enum Toggle {
	Start, Stop
}

pub enum Event {
	WalkForward(Toggle),
	WalkLeft(Toggle),
	WalkRight(Toggle),
	WalkBack(Toggle),
	CursorMove(Vec2u), // not triggered in the normal game, only in menu
	CamRotate(Vec2u),
}
