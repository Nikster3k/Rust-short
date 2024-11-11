// use std::{fs::read, str::Utf8Error};

use std::{env::Args, fs, iter::Skip};

#[derive(Debug)]
struct BinaryReader {
	filter_non_null: bool,
	filter_min: Option<usize>,
	filter_max: Option<usize>,
	program_path: Option<String>

}

enum ParsingError {
	NotValidNumber,
	InvalidArgs
}

impl BinaryReader {
	fn new() -> Self{
		Self{filter_non_null: false, filter_min: None, filter_max: None, program_path: None}
	}

	fn parse_next_arg_to_num(iter: &mut Skip<Args>) -> Result<usize, ParsingError> {
		if let Some(x) = iter.next() {
			if x.find(|c: char| !c.is_ascii_digit()).is_some() {
				Err(ParsingError::NotValidNumber)
			} else {
				match x.parse::<usize>() {
					Ok(num) => Ok(num),
					Err(_) => Err(ParsingError::NotValidNumber)
				}
			}
		} else {
			Err(ParsingError::InvalidArgs)
		}
	}

	fn parse_args(&mut self) -> Result<(), ParsingError> {
		if std::env::args().len() < 2 {
			return Err(ParsingError::InvalidArgs);
		}

		let mut args = std::env::args().skip(1);

		while let Some(arg) = args.next() {
			match arg.as_str() {
				"-n" => self.filter_non_null = true,
				"-m" => {
					self.filter_min = match BinaryReader::parse_next_arg_to_num(&mut args) {
						Ok(val) => Some(val),
						Err(_) => return Err(ParsingError::NotValidNumber),
					}
				},
				"-M" => {
					self.filter_max = match BinaryReader::parse_next_arg_to_num(&mut args) {
						Ok(val) => Some(val),
						Err(_) => return Err(ParsingError::NotValidNumber),
					}
				}
				_ => {
					if self.program_path.is_some() {
						return Err(ParsingError::InvalidArgs);
					}
					self.program_path = Some(arg);
				}
			}
		}
		if self.program_path.is_none() {
			Err(ParsingError::InvalidArgs)
		} else {
			Ok(())
		}
	}

	fn print_binary(&mut self) {
		if let Some(path) = &self.program_path {
			let content = match fs::read(path) {
				Ok(file) => file,
				Err(_) => {
					eprintln!("error: file not found");
					return;
				}
			};
			let mut valid_content = String::new();
			let mut current_string = String::new();

            for byte in &content {
                if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
                    current_string.push(*byte as char);
                } else if !current_string.is_empty() {
					valid_content.push_str(&current_string);
					valid_content.push('\n');
					current_string.clear();
				}
            }

			for line in valid_content.lines() {
				if self.filter_non_null && !line.ends_with('\0') {
					continue;
				}
				match (self.filter_min, self.filter_max) {
					(Some(min), None) => if line.len() >= min {println!{"{}", line}}
					(None, Some(max)) => if line.len() <= max {println!{"{}", line}},
					(Some(min), Some(max)) => if line.len() >= min && line.len() <= max {println!("{}", line)}
					(None, None) => println!("{}", line),
				}
			}
		}
	}
}

fn main() {
	let mut reader = BinaryReader::new();

	if let Err(error) = reader.parse_args() {
		match error {
			ParsingError::InvalidArgs => println!("error: Invalid Arguments"),
			ParsingError::NotValidNumber => println!("error: not a valid number")
		}
		return;
	}
	
	reader.print_binary();
}
