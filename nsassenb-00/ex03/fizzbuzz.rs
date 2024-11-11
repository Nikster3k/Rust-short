fn main() {
	for n in 1..101 {
		match (n % 3 == 0, n % 5 == 0, n % 11) {
			(true, true, _) => std::println!("fizzbuzz"),
			(true, _, _) => std::println!("fizz"),
			(_, true, _) => std::println!("buzz"),
			(_, _, 3) => std::println!("FIZZ"),
			(_, _, 5) => std::println!("BUZZ"),
			(_, _, _) => std::println!("{}", n),
		}
	}
}
