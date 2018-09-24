use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	pub x: T,
	pub y: T,
	pub z: T,
}

pub type Vec3f = Vec3t<f32>;
pub type Vec3u = Vec3t<u32>;

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

impl<T> Add<T> for Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	type Output = Vec3t<T>;

	fn add(self, other: T) -> Vec3t<T> {
		Vec3t::new (
			self.x + other,
			self.y + other,
			self.z + other,
		)
	}
}

impl<T> Sub<T> for Vec3t<T>
		where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	type Output = Vec3t<T>;

	fn sub(self, other: T) -> Vec3t<T> {
		Vec3t::new (
			self.x - other,
			self.y - other,
			self.z - other,
		)
	}
}

use std::hash::{Hash, Hasher};

impl<T> Hash for Vec3t<T> where T: Hash + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.x.hash(h);
		self.y.hash(h);
		self.z.hash(h);
		h.finish();
	}
}

impl<T> Eq for Vec3t<T> where T: Hash + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq + Eq {}

impl<T> Vec3t<T> where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + PartialEq {
	pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec3t<U> where U: Add<Output=U> + Sub<Output=U> + Mul<Output=U> + Div<Output=U> + Copy + PartialEq {
		Vec3t::new(
			f(self.x),
			f(self.y),
			f(self.z),
		)
	}
}
