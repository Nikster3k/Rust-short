use std::vec;

use ftkit::random_number;

fn choose<T>(values: &[T]) -> &T {
	&values[random_number(0..values.len() as i32) as usize]
}

fn main() {
	let mut arr: Vec<i32> = vec![0; 32];

	for (i, item) in arr.iter_mut().enumerate() {
		*item = i as i32;
	}

	std::println!("{}", choose(&arr));
}
