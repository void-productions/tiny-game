use vec::Vec2f;
use toggle::Toggle;

#[derive(Clone, Copy)]
pub enum Action {
	WalkForward(Toggle),
	WalkLeft(Toggle),
	WalkRight(Toggle),
	WalkBack(Toggle),
	CursorMove(Vec2f), // not triggered in the normal game, only in menu
	CamRotate(Vec2f),
}
