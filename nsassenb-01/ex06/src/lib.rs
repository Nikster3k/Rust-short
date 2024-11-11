#[allow(dead_code)]
fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
	let mut added = Vec::new();
	let mut remainder: u8 = 0;
	let mut ai = a.len() as i32;
	let mut bi = b.len() as i32;

	std::assert!(!(a.is_empty() || b.is_empty()));

	while ai > 0 || bi > 0 {
		ai -= 1;
		bi -= 1;

		let a_digit = if ai >= 0 { a[ai as usize] } else { b'0' };
		let b_digit = if bi >= 0 { b[bi as usize] } else { b'0' };

		std::assert!(a_digit.is_ascii_digit());
		std::assert!(b_digit.is_ascii_digit());

		let mut result = (a_digit + b_digit) - b'0' * 2;
		result += remainder;
		remainder = 0;
		if result > 9 {
			result -= 10;
			remainder = 1;
		}
		added.push(result + b'0');
	}
	added.push(remainder + b'0');
	//remove '0' chars
	for index in (1..added.len()).rev() {
		if added[index] != b'0' {
			break;
		}
		added.remove(index);
	}
	added.reverse();
	added
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_tests() {
		assert_eq!(big_add(b"2", b"4"), b"6");
		assert_eq!(big_add(b"0010", b"0200"), b"210");
    }

	#[test]
	#[should_panic]
	fn non_digit() {
		big_add(b"123f", b"1234");
	}

	#[test]
	fn one_bigger() {
		assert_eq!(big_add(b"1200", b"123"), b"1323");
	}

	#[test]
	fn just_zeros_different_len() {
		assert_eq!(big_add(b"0000", b"00"), b"0");
	}

	#[test]
	fn zero_plus_zero() {
		assert_eq!(big_add(b"0", b"0"), b"0");
	}

	#[test]
	#[should_panic]
	fn test_empty_lhs() {
		big_add(b"123", b"");
	}

	#[test]
	#[should_panic]
	fn test_empty_rhs() {
		big_add(b"", b"1243");
	}
}
