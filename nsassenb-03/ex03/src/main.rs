use std::fmt::Debug;

trait FortyTwo {
    fn forty_two() -> Self;
}

impl FortyTwo for u32 {
	fn forty_two() -> Self {
		42u32
	}
}

impl FortyTwo for String {
	fn forty_two() -> Self {
		"42".to_string()
	}
}

fn print_forty_two<T: Debug + FortyTwo>() {
	std::println!("{:?}", T::forty_two())
}

fn main() {
	print_forty_two::<String>();
	print_forty_two::<u32>();
}
