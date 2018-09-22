use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug)]
pub struct Vec3f {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy {
	pub x: T,
	pub y: T,
}

pub type Vec2f = Vec2t<f32>;

impl<T> Vec2t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy {
	pub fn new(x: T, y: T) -> Vec2t<T> {
		Vec2t { x, y }
	}
}

impl<T> Add<Vec2t<T>> for Vec2t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy {
	type Output = Vec2t<T>;

	fn add(self, rhs: Vec2t<T>) -> Vec2t<T> {
		Vec2t::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl<T> Sub<Vec2t<T>> for Vec2t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy {
	type Output = Vec2t<T>;

	fn sub(self, other: Vec2t<T>) -> Vec2t<T> {
		Vec2t::new (
			self.x - other.x,
			self.y - other.y,
		)
	}
}

impl<T> Mul<Vec2t<T>> for Vec2t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy {
	type Output = Vec2t<T>;

	fn mul(self, other: Vec2t<T>) -> Vec2t<T> {
		Vec2t::new (
			self.x * other.x,
			self.y * other.y,
		)
	}
}

impl<T> Mul<T> for Vec2t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy {
	type Output = Vec2t<T>;

	fn mul(self, other: T) -> Vec2t<T> {
		Vec2t::new (
			self.x * other,
			self.y * other,
		)
	}
}

impl<T> Div<Vec2t<T>> for Vec2t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy {
	type Output = Vec2t<T>;

	fn div(self, other: Vec2t<T>) -> Vec2t<T> {
		Vec2t::new (
			self.x / other.x,
			self.y / other.y,
		)
	}
}

impl<T> Div<T> for Vec2t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy {
	type Output = Vec2t<T>;

	fn div(self, other: T) -> Vec2t<T> {
		Vec2t::new (
			self.x / other,
			self.y / other,
		)
	}
}
