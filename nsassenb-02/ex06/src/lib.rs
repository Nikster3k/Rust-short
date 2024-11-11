#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Word(&'a str),
    RedirectStdout,
    RedirectStdin,
    Pipe,
}

pub fn next_token<'a>(s: &mut &'a str) -> Option<Token<'a>> {

	if s.is_empty() {
		return None;
	}

	let mut split_idx = s.len();
	for (i, ch) in s.char_indices() {
		if ch.is_whitespace() {
			split_idx = i;
			break;
		}
	}

	let split: (&str, &str) = s.split_at(split_idx);
	if let Some(x) = s.strip_prefix(split.0) {
		*s = x.trim_start();
	}

	match split.0 {
		"|" => Some(Token::Pipe),
		">" => Some(Token::RedirectStdout),
		"<" => Some(Token::RedirectStdin),
		_ => Some(Token::Word(split.0))
	}
}

#[cfg(test)]
mod test{
	use super::*;

	#[test]
	fn test_cool() {
		let mut string = "echo hi | cool wow";
		while let Some(token) = next_token(&mut string) {
			std::println!("{:?}", token);
		}
	}

	#[test]
	fn test_redirect_stdout() {
		let mut string = "echo hi > output.txt";
		let expected_tokens = vec![
			Token::Word("echo"),
			Token::Word("hi"),
			Token::RedirectStdout,
			Token::Word("output.txt"),
		];
		let mut tokens = Vec::new();
		while let Some(token) = next_token(&mut string) {
			tokens.push(token);
		}
		assert_eq!(tokens, expected_tokens);
	}

	#[test]
	fn test_redirect_stdin() {
		let mut string = "cat < input.txt";
		let expected_tokens = vec![
			Token::Word("cat"),
			Token::RedirectStdin,
			Token::Word("input.txt"),
		];
		let mut tokens = Vec::new();
		while let Some(token) = next_token(&mut string) {
			tokens.push(token);
		}
		assert_eq!(tokens, expected_tokens);
	}

	#[test]
	fn test_pipe() {
		let mut string = "ls | grep txt";
		let expected_tokens = vec![
			Token::Word("ls"),
			Token::Pipe,
			Token::Word("grep"),
			Token::Word("txt"),
		];
		let mut tokens = Vec::new();
		while let Some(token) = next_token(&mut string) {
			tokens.push(token);
		}
		assert_eq!(tokens, expected_tokens);
	}

	#[test]
	fn test_multiple_spaces() {
		let mut string = "echo    hi";
		let expected_tokens = vec![
			Token::Word("echo"),
			Token::Word("hi"),
		];
		let mut tokens = Vec::new();
		while let Some(token) = next_token(&mut string) {
			tokens.push(token);
		}
		assert_eq!(tokens, expected_tokens);
	}
}
