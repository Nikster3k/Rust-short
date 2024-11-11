#[derive(PartialEq)]
enum Command {
    Todo(String),   // Command: "TODO"
    Done(usize),    // Command: "DONE"
    Purge,          // Command: "PURGE"
    Quit,           // Command: "QUIT"
}

impl Command {
    fn prompt() -> Self {
		let input = ftkit::read_line();
		if input.is_empty() {
			return Command::Quit;
		}
		let input = input.as_str();
		let split = input.split_at(input.find(' ').unwrap_or(input.len() - 1));
		match split.0 {
			"TODO" => Command::Todo(split.1.trim().to_string()),
			"DONE" => Command::Done(split.1.trim().parse::<usize>().unwrap_or(0)),
			"PURGE" => Command::Purge,
			"QUIT" => Command::Quit,
			_ => {std::println!("Command '{}' not allowed!", split.0); Command::Quit}
		}
	}
}

struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
		TodoList{todos: Vec::new(), dones: Vec::new()}
	}

    fn display(&self) {
		for (i, item) in self.todos.iter().enumerate() {
			std::println!("{i} [ ] - {item}");
		}
		for item in &self.dones {
			std::println!("  [X] - {item}");
		}
		std::println!();
	}

    fn add(&mut self, todo: String) {
		if !todo.is_empty() {
			self.todos.push(todo);
		} else {
			std::println!("Cant add empty task!");
		}
	}

	fn done(&mut self, index: usize) {
		if index < self.todos.len() {
			self.dones.push(self.todos.remove(index));
		} else {
			std::println!("Index {} is out of range!", index);
		}
	}

    fn purge(&mut self) {
		self.dones.clear();
	}
}

fn main() {
	let mut tl = TodoList::new();
	let mut cmd = Command::Purge;

	while cmd != Command::Quit {
		tl.display();
		cmd = Command::prompt();
		match &cmd {
			Command::Todo(todo) => tl.add(todo.clone()),
			Command::Done(index) => tl.done(*index),
			Command::Purge => tl.purge(),
			Command::Quit => break
		}
	}

}
