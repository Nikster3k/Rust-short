#[derive(Clone, Copy, Debug, PartialEq)]
struct Color {
	red: u8,
	green: u8,
	blue: u8,
}

#[allow(dead_code)]
impl Color {
	const WHITE: Self = Color{red: 255, green: 255, blue: 255};
	const RED: Self = Color{red: 255, green: 0, blue: 0};
	const GREEN: Self = Color{red: 0, green: 255, blue: 0};
	const BLUE: Self = Color{red: 0, green: 0, blue: 255};

	const fn new(red: u8, green: u8, blue: u8) -> Self {
		Color{red, green, blue}
	}

	fn mix(self, blend: &Self, blend_alpha: u8) -> Self {
		Color{
			red: (blend.red as f32 * (blend_alpha as f32 / 255.0) + self.red as f32 * ((u8::MAX - blend_alpha) as f32 / 255.0)) as u8,
			green: (blend.green as f32 * (blend_alpha as f32 / 255.0) + self.green as f32 * ((u8::MAX - blend_alpha) as f32 / 255.0)) as u8,
			blue: (blend.blue as f32 * (blend_alpha as f32 / 255.0) + self.blue as f32 * ((u8::MAX - blend_alpha) as f32 / 255.0)) as u8,
		}
	}

	fn distance(self, other: &Self) -> u32 {
		(self.red.abs_diff(other.red) as u32 * self.red.abs_diff(other.red) as u32) 
		+ (self.green.abs_diff(other.green) as u32 * self.green.abs_diff(other.green) as u32)
		+ (self.blue.abs_diff(other.blue) as u32 * self.blue.abs_diff(other.blue) as u32)
	}

	//copilot copy pase
	fn closest_mix(&self, palette: &[(Color, u8)], max: usize) -> Color {
		fn recursive_mix(target: &Color, palette: &[(Color, u8)], base: Color, max: usize) -> Color {
			if max == 0 {
				return base;
			}

			let mut best_mix = base;
			let mut least_dist = target.distance(&base);

			for &(color, percentage) in palette {
				let mixed = base.mix(&color, percentage);
				// std::println!("base {:?} MIXED {:?}  Palette colour{:?}", base, mixed, color);
				let result = recursive_mix(target, palette, mixed, max - 1);
				let curr_dist = target.distance(&result);

				if curr_dist < least_dist {
					least_dist = curr_dist;
					best_mix = result;
				}
			}
			best_mix
		}

		if palette.is_empty() {
			return Color::WHITE;
		}

		recursive_mix(self, palette, Color::WHITE, max)
	}
}

#[cfg(test)]
mod test{
	use super::*;
	
	#[test]
	fn test_subject_no_palette() {
		assert_eq!(Color::RED.closest_mix(&[], 100), Color::WHITE);
	}

	#[test]
	fn test_subject_max_zero() {
		assert_eq!(Color::RED.closest_mix(&[(Color::RED, 255)], 0), Color::WHITE);
	}

	// #[test]
	// fn test_subject_real_palette(){
	// 	let palette = [(Color::RED, 100), (Color::GREEN, 100), (Color::BLUE, 100)];

	// 	//proof that colour that is checked against has a bigger distance than the recursively found colour
	// 	std::println!("{}", Color::new(254, 23, 102).distance(&Color::new(218, 20, 57)));
	// 	std::println!("{}", Color::new(254, 23, 102).distance(&Color::new(217, 34, 71)));
		
	// 	assert_eq!(
	// 		Color::new(254, 23, 102).closest_mix(&palette, 5),
	// 		Color::new(218, 20, 57),
	// 	);
	// }
}
