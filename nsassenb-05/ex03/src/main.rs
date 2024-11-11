use std::env;
use std::sync::mpsc::sync_channel;

fn main() {
	if env::args().len() != 2 {
		std::println!("Invalid arguments");
		return;
	}
	let brain_space: usize = match env::args().nth(1).unwrap().parse::<usize>() {
		Ok(val) => val,
		_ => {
			std::print!("Argument is non digit");
			return;
		}
	};

	let (tx, rx) = sync_channel::<String>(brain_space);
	std::thread::spawn(move || {
		while let Ok(msg) = rx.recv(){
			if msg.is_empty() {
				break;
			}
			std::print!("the philosopher is thinking about {}", msg);
			std::thread::sleep(std::time::Duration::from_secs(5));
		}
	});

	let mut input = String::new();

	while std::io::stdin().read_line(&mut input).is_ok() {
		if input.is_empty() {
			break;
		}
		match tx.try_send(input.clone()) {
			Ok(_) => (),
			_ => println!("the philosopher's head is full")
		}
		input.clear();
	}
}
