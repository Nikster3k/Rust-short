use std::ops;

#[derive(Debug, Copy, Clone)]
struct Vector<T> {
	x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Vector<T> {
	fn new(x: T, y: T) -> Self {
		Vector::<T>{x, y}
	}
}

#[allow(dead_code)]
trait Length<T> {
	fn length(self) -> T;
}

impl Length<f32> for Vector<f32> {
	fn length(self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
}

impl Length<f64> for Vector<f64> {
	fn length(self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
}

impl<T: std::ops::Add<Output = T>> ops::Add<Vector<T>> for Vector<T> {
	type Output = Vector<T>;

	fn add(self, rhs: Vector<T>) -> Self::Output {
		Vector{x: self.x + rhs.x, y: self.y + rhs.y}
	}
}

impl<T: std::ops::Sub<Output = T>> ops::Sub<Vector<T>> for Vector<T> {
	type Output = Vector<T>;

	fn sub(self, rhs: Vector<T>) -> Self::Output {
		Vector{x: self.x - rhs.x, y: self.y - rhs.y}
	}
}

impl<T: std::ops::AddAssign> std::ops::AddAssign for Vector<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T: std::ops::SubAssign> std::ops::SubAssign for Vector<T> {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T: std::ops::Mul<Output = T> + Copy> ops::Mul<T> for Vector<T> {
	type Output = Vector<T>;

	fn mul(self, rhs: T) -> Self::Output {
		Vector::<T>{x: self.x * rhs, y: self.y * rhs}
	}
}

impl<T: std::ops::Div<Output = T> + Copy> ops::Div<T> for Vector<T> {
	type Output = Vector<T>;

	fn div(self, rhs: T) -> Self::Output {
		Vector::<T>{x: self.x / rhs, y: self.y / rhs}
	}
}

impl<T: std::ops::MulAssign + Copy> ops::MulAssign<T> for Vector<T> {
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<T: std::ops::DivAssign + Copy> ops::DivAssign<T> for Vector<T> {
	fn div_assign(&mut self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
	}
}

impl<T: PartialEq> PartialEq for Vector<T> {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}


#[cfg(test)]
#[test]
fn test_a() {
    let v = Vector {
        x: String::from("Hello, World!"),
        y: String::from("Hello, Rust!"),
    };

    let w = v.clone();

    assert_eq!(&v, &w);
}

#[cfg(test)]
#[test]
fn test_b() {
    let v = Vector::new("Hello, World!", "Hello, Rust!");
    let a = v;
    let b = v;
    assert_eq!(a, b);
}

#[cfg(test)]
#[test]
fn test_c() {
    let v = Vector::new(0.0, 5.0);
	assert_eq!(v.length(), 5.0);
}

#[cfg(test)]
#[test]
fn test_ne(){
	let v = Vector::new(0.0, 5.0);
    let f = Vector::new(0.0, 5.0);
	assert!(!v.ne(&f));
}
