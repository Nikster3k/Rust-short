#[allow(dead_code)]
fn deduplicate(list: &mut Vec<i32>) {
	for i in 0..list.len() {
		let mut x = i + 1;
		while x < list.len() {
			if list[x] == list[i] {
				list.remove(x);
				continue;
			}
			x += 1;
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
		let mut v = vec![1, 2, 2, 3, 2, 4, 3];
		deduplicate(&mut v);
		assert_eq!(v, [1, 2, 3, 4]);
    }

	#[test]
	fn more() {
		let mut v = vec![1, 2, 2, 2, 1, 1, 1, 6, 2, 3, 3, 3, 5, 5, 5, 3, 3, 3, 3, 1];
		deduplicate(&mut v);
		assert_eq!(v, [1, 2, 6, 3, 5]);
	}
}
