fn collatz(start: u32) {
	let mut n = start;
    while n != 1 && n != 0 {
		std::println!("{}", n);
		if n % 2 == 0 {
			n /= 2;
		} else {
			n = 3 * n + 1;
		}
    }
	std::println!("{}", n);
}
