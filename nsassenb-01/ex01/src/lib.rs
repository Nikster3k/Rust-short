#[allow(dead_code)]
fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
	if *a < *b {
		a
	} else {
		b
	}
}

#[cfg(test)]
mod test {

	use super::min;

	#[test]
	fn test() {
		let a = 5;
		let b = 9;
		std::assert!(*min(&a, &b) == a);
	}
}
