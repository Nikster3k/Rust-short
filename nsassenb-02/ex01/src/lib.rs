struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
impl Point {
    fn new(x: f32, y: f32) -> Self {
		Point{x, y}
	}

    fn zero() -> Self {
		Point{x: 0.0, y: 0.0}
	}

    fn distance(&self, other: &Self) -> f32 {
		((other.x - self.x) * (other.x - self.x) + (other.y - self.y) * (other.y - self.y)).sqrt()
	}

    fn translate(&mut self, dx: f32, dy: f32) {
		self.x += dx;
		self.y += dy;
	}
}


#[cfg(test)]
mod test{
	use super::*;
	
	#[test]
	fn test_new() {
		let a = Point::new(5.0, 7.0);
		
		std::assert!(a.x == 5.0 && a.y == 7.0);
	}

	#[test]
	fn test_zero() {
		let a = Point::zero();
		
		std::assert!(a.x == 0.0 && a.y == 0.0);
	}

	#[test]
	fn test_distance() {
		let a = Point::zero();
		let b = Point::new(5.0, 6.0);
		
		let dist = a.distance(&b);

		std::assert!(dist == 7.810249676);
	}

	#[test]
	fn test_translate() {
		let mut a = Point::zero();
		
		a.translate(5.0, 7.0);

		std::assert!(a.x == 5.0 && a.y == 7.0);
	}
}
