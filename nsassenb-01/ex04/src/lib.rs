fn calc_area(abox: &[u32; 2]) -> u32 {
	abox[0] * abox[1]
}

fn check_fit(parent_box: &[u32; 2], child_box: &[u32; 2]) {
	std::assert!(!(parent_box[0] < child_box[0] || parent_box[1] < child_box[1]));
}

#[allow(dead_code)]
fn sort_boxes(boxes: &mut [[u32; 2]]) {
	if boxes.is_empty() {
		return;
	}
	for i in 0..boxes.len() {
		for x in i + 1..boxes.len() {
			if calc_area(&boxes[i]) < calc_area(&boxes[x]) {
				boxes.swap(i, x);
			}
		}
	}
	for i in 0..boxes.len() - 1 {
		check_fit(&boxes[i], &boxes[i + 1]);
	}
}

#[cfg(test)]
mod tests {
	use super::sort_boxes;

	#[test]
	fn test_subject() {
		let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
	}

	#[test]
	#[should_panic]
	fn test_impossible_infinitley_thin_box_and_too_wide() {
		let mut boxes = [[5, 5], [4, 3], [1, 0], [5, 0], [5, 7], [3, 3]];
		sort_boxes(&mut boxes);
	}
}
