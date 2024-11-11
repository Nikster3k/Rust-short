fn print_bytes(s: &str) {
	for c in s.bytes() {
		std::println!("{}", c);
	}
}
