use std::{io::{Read, Write}, net::TcpStream};

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
		Ok(stream) => stream,
		Err(err) => { 
			std::println!("Tcp connect error: {}", err);
			return;
		}
	};

	if let Err(x) = stream.write_all(b"GET / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nConnection: close\r\n\r\n") {
		std::println!("Send failed error: {}", x);
		return;
	}

	let mut read_buffer = String::new();

	if let Err(x) = stream.read_to_string(&mut read_buffer) {
		std::println!("Read failed error: {x}");
		return;
	}

	std::println!("{read_buffer}");
}
