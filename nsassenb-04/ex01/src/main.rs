use std::{env, fs::File, io::{stdin, stdout, Read, Write}};

fn main() {
	let mut buff = String::new();

	match stdin().read_to_string(&mut buff) {
		Err(_) => (),
		Ok(_) => {
			if std::write!(stdout(), "{}", buff).is_err() {};
			for arg in env::args().skip(1) {
				if let Ok(mut file) = File::create(arg) { if file.write(buff.as_bytes()).is_err() {} }
			}
		}
	}
}
