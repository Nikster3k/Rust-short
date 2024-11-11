fn contains_all(contender: &[u32], needle: &[u32]) -> bool {
	for num in needle {
		if !contender.contains(num) {
			return false;
		}
	}
	true
}

#[allow(dead_code)]
fn largest_group<'a>(haystack: &'a [u32], needle: &[u32]) -> &'a [u32] {
	let mut start: i32 = -1;
	let mut len = 0;
	let mut ret: &[u32] = &[];
	let mut contender: &[u32];

	if needle.is_empty() {
		return ret;
	}
	for (i, num) in haystack.iter().enumerate() {
		if needle.contains(num) {
			if start == -1 {
				start = i as i32;
			}
			len += 1;
		} else {
			if len > ret.len() {
				contender = &haystack[start as usize..start as usize + len];
				if contains_all(contender, needle) {
					ret = contender;
				}
			}
			len = 0;
			start = -1;
		}
	}
	if len > ret.len() {
		contender = &haystack[start as usize..start as usize + len];
		if contains_all(contender, needle) {
			ret = contender;
		}
	}
	ret
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test1() {
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
	}

	#[test]
	fn test2() {
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
	}

	#[test]
	fn test3() {
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
	}

	#[test]
	fn test4() {
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
	}

	#[test]
	fn test5() {
		assert_eq!(largest_group(&[1, 3, 4, 4, 4, 4, 4, 3, 4, 1, 4], &[4, 1]), &[4, 1, 4]);
	}


	#[test]
	fn test_lifetimes() {
		let haystack = [1, 2, 3, 2, 1];
		let result;
	
		{
			let needle = [2, 3];
			result = largest_group(&haystack, &needle);
		}
	
		assert_eq!(result, &[2, 3, 2]);
	}
}
