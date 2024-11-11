#[allow(dead_code)]
const fn color_name<'a>(color: &[u8; 3]) -> &'a str {
	match color {
		[0, 0, 0] => "pure black",
		[255, 0, 0] => "pure red",
		[0, 255, 0] => "pure green",
		[0, 0, 255] => "pure blue",
		[255, 255, 255] => "pure white",
		[128, 128, 128] => "perfect grey",
		[0..31, 0..31, 0..31] => "almost black",
		[129..=u8::MAX, 0..=127, 0..=127] => "redish",
		[0..=127, 129..=u8::MAX, 0..=127] => "greenish",
		[0..=127, 0..=127, 129..=u8::MAX] => "blueish",
		_ => "unknown"
	}
}

#[cfg(test)]
mod tests {
	use super::color_name;

	#[test]
	fn test_lifetimes() {
		let name_of_the_best_color;
	
		{
			let the_best_color = [42, 42, 42];
			name_of_the_best_color = color_name(&the_best_color);
		}
	
		assert_eq!(name_of_the_best_color, "unknown");
	}
	
	#[test]
	fn test_black() {
		let name_of_the_best_color = color_name(&[0, 0, 0]);
		assert_eq!(name_of_the_best_color, "pure black");
	}
	
	#[test]
	fn test_almost_black() {
		let name_of_the_best_color = color_name(&[7, 7, 7]);
		assert_eq!(name_of_the_best_color, "almost black");
	}
	
	#[test]
	fn test_redish() {
		let name_of_the_best_color = color_name(&[200, 7, 7]);
		assert_eq!(name_of_the_best_color, "redish");
	}
}
