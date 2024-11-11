#[allow(dead_code)]
fn min<T: PartialOrd>(lhs: T, rhs: T) -> T {
	if lhs < rhs {
		lhs
	} else {
		rhs
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_a() {
		assert_eq!(min(12i32, -14i32), -14);
	}

	#[test]
	fn test_b() {
		assert_eq!(min(12f32, 14f32), 12f32);
	}

	#[test]
	fn test_c() {
		assert_eq!(min("abc", "def"), "abc");
	}

	#[test]
	fn test_d() {
		assert_eq!(min(String::from("abc"), String::from("def")), "abc");
	}
}
