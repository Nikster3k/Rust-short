use std::io::{stdout, Write};

fn main() {
    for i in 0..=10 {
		if writeln!(stdout(), "{}", i).is_err() {}
	}
}
