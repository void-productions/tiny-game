use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Debug, Error, Formatter};
use std::hash::{Hash, Hasher};

pub struct Vec3t<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

#[allow(dead_code)]
pub type Vec3f = Vec3t<f32>;
#[allow(dead_code)]
pub type Vec3u = Vec3t<u32>;
#[allow(dead_code)]
pub type Vec3i = Vec3t<i32>;

impl<T> Vec3t<T> {
	pub fn new(x: T, y: T, z: T) -> Vec3t<T> {
		Vec3t { x, y, z }
	}
}

impl<T: Copy> Vec3t<T> {
	pub fn with(a: T) -> Vec3t<T> {
		Vec3t {
			x: a,
			y: a,
			z: a,
		}
	}
}

impl<T> Add<Vec3t<T>> for Vec3t<T> where T: Add<Output=T> {
	type Output = Vec3t<T>;

	fn add(self, other: Vec3t<T>) -> Vec3t<T> {
		Vec3t::new (
			self.x + other.x,
			self.y + other.y,
			self.z + other.z,
		)
	}
}

impl<T> Add<T> for Vec3t<T> where T: Add<Output=T> + Copy {
	type Output = Vec3t<T>;

	fn add(self, other: T) -> Vec3t<T> {
		Vec3t::new (
			self.x + other,
			self.y + other,
			self.z + other,
		)
	}
}

impl<T> Sub<Vec3t<T>> for Vec3t<T> where T: Sub<Output=T> {
	type Output = Vec3t<T>;

	fn sub(self, other: Vec3t<T>) -> Vec3t<T> {
		Vec3t::new (
			self.x - other.x,
			self.y - other.y,
			self.z - other.z,
		)
	}
}

impl<T> Sub<T> for Vec3t<T> where T: Sub<Output=T> + Copy {
	type Output = Vec3t<T>;

	fn sub(self, other: T) -> Vec3t<T> {
		Vec3t::new (
			self.x - other,
			self.y - other,
			self.z - other,
		)
	}
}

impl<T> Mul<Vec3t<T>> for Vec3t<T> where T: Mul<Output=T> {
	type Output = Vec3t<T>;

	fn mul(self, other: Vec3t<T>) -> Vec3t<T> {
		Vec3t::new (
			self.x * other.x,
			self.y * other.y,
			self.z * other.z,
		)
	}
}

impl<T> Mul<T> for Vec3t<T> where T: Mul<Output=T> + Copy {
	type Output = Vec3t<T>;

	fn mul(self, other: T) -> Vec3t<T> {
		Vec3t::new (
			self.x * other,
			self.y * other,
			self.z * other,
		)
	}
}

impl<T> Div<Vec3t<T>> for Vec3t<T> where T: Div<Output=T> {
	type Output = Vec3t<T>;

	fn div(self, other: Vec3t<T>) -> Vec3t<T> {
		Vec3t::new (
			self.x / other.x,
			self.y / other.y,
			self.z / other.z,
		)
	}
}

impl<T> Div<T> for Vec3t<T> where T: Div<Output=T> + Copy {
	type Output = Vec3t<T>;

	fn div(self, other: T) -> Vec3t<T> {
		Vec3t::new (
			self.x / other,
			self.y / other,
			self.z / other,
		)
	}
}

impl<T> Hash for Vec3t<T> where T: Hash {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.x.hash(h);
		self.y.hash(h);
		self.z.hash(h);
		h.finish();
	}
}

impl<T: PartialEq> PartialEq for Vec3t<T> {
	fn eq(&self, rhs: &Self) -> bool {
		(self.x == rhs.x) && (self.y == rhs.y) && (self.z == rhs.z)
	}
}

impl<T: Eq> Eq for Vec3t<T> {}

impl<T> Vec3t<T> {
	pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec3t<U> {
		Vec3t::new(
			f(self.x),
			f(self.y),
			f(self.z),
		)
	}
}

impl<T: Clone> Clone for Vec3t<T> {
	fn clone(&self) -> Self {
		Vec3t::new(
			self.x.clone(),
			self.y.clone(),
			self.z.clone(),
		)
	}
}

impl<T: Copy> Copy for Vec3t<T> { }

impl<T: Display> Display for Vec3t<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec3t({}, {}, {})", self.x, self.y, self.z);
		fmt.write_str(&*s)
	}
}

impl<T: Debug> Debug for Vec3t<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec3t({:?}, {:?}, {:?})", self.x, self.y, self.z);
		fmt.write_str(&*s)
	}
}

impl Vec3f {
	pub fn magnitude(self) -> f32 {
		(self.x + self.y + self.z).sqrt()
	}
}
