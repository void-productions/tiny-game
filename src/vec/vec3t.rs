use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	pub x: T,
	pub y: T,
	pub z: T,
}

pub type Vec3f = Vec3t<f32>;

impl<T> Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	pub fn new(x: T, y: T, z: T) -> Vec3t<T> {
		Vec3t { x, y, z }
	}
}

impl<T> Add<Vec3t<T>> for Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	type Output = Vec3t<T>;

	fn add(self, rhs: Vec3t<T>) -> Vec3t<T> {
		Vec3t::new (
			self.x + rhs.x,
			self.y + rhs.y,
			self.z + rhs.z
		)
	}
}

impl<T> Sub<Vec3t<T>> for Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	type Output = Vec3t<T>;

	fn sub(self, other: Vec3t<T>) -> Vec3t<T> {
		Vec3t::new (
			self.x - other.x,
			self.y - other.y,
			self.z - other.z,
		)
	}
}

impl<T> Mul<Vec3t<T>> for Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	type Output = Vec3t<T>;

	fn mul(self, other: Vec3t<T>) -> Vec3t<T> {
		Vec3t::new (
			self.x * other.x,
			self.y * other.y,
			self.z * other.z,
		)
	}
}

impl<T> Mul<T> for Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	type Output = Vec3t<T>;

	fn mul(self, other: T) -> Vec3t<T> {
		Vec3t::new (
			self.x * other,
			self.y * other,
			self.z * other,
		)
	}
}

impl<T> Div<Vec3t<T>> for Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	type Output = Vec3t<T>;

	fn div(self, other: Vec3t<T>) -> Vec3t<T> {
		Vec3t::new (
			self.x / other.x,
			self.y / other.y,
			self.z / other.z,
		)
	}
}

impl<T> Div<T> for Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	type Output = Vec3t<T>;

	fn div(self, other: T) -> Vec3t<T> {
		Vec3t::new (
			self.x / other,
			self.y / other,
			self.z / other,
		)
	}
}
