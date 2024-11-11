use std::{self, env, str};

fn main() {
	if env::args().len() < 2 {
		return;
	}

	let mut cmd = std::process::Command::new(env::args().nth(1).expect("Failed to get command"));

	for arg in env::args().skip(2) {
		cmd.arg(arg);
	}

	for arg in std::io::stdin().lines().map_while(Result::ok) {
		cmd.arg(arg);
	}

	if let Ok(ret) = cmd.output() {
		let string = match str::from_utf8(ret.stdout.as_slice()) {
			Ok(string) => string,
			Err(_) => {
				return;
			}
		};
		print!("{}", string);
	}
}
