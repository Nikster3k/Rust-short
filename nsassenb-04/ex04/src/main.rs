use std::process::{Child, Output, Stdio};

fn main() {
    let mut args_vec: Vec<Vec<String>> = Vec::new();

	if std::env::args().len() < 2 {
		return;
	}

	let mut temp_vec: Vec<String> = Vec::new();
	for arg in std::env::args().skip(1) {
		if arg != "," {
			temp_vec.push(arg);
		} else {
			args_vec.push(temp_vec.clone());
			temp_vec.clear();
		}
	}
	args_vec.push(temp_vec);

	let mut commands: Vec<std::process::Command> = Vec::with_capacity(args_vec.len());
	for cmd in args_vec {
		let mut tmp_cmd = std::process::Command::new(&cmd[0]);
		for stri in cmd.iter().skip(1) {
			tmp_cmd.arg(stri);
		}
		commands.push(tmp_cmd);
	}

	let mut childs: Vec<Child> = Vec::with_capacity(commands.len());
	for mut command in commands {
		if let Ok(a) = command.stdout(Stdio::piped()).spawn() {
			childs.push(a);
		}
	}

	let mut outputs: Vec<Output> = Vec::with_capacity(childs.len());
	for c in childs {
		if let Ok(output) = c.wait_with_output() {
			outputs.push(output);
		}
	}

	for o in outputs {
		if let Ok(out_string) = std::str::from_utf8(o.stdout.as_slice()) {
			println!("===== Output =====\n{}\n", out_string);
		}
	}
}
