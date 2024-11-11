use std::io;
use std::sync::Mutex;
use std::sync::Arc;

struct Logger<W> {
	buffer: Box<[u8]>,
	writer: W,
	idx: usize,
}

impl<W> Logger<W> {
	pub fn new(threshold: usize, writer: W) -> Self {
		Logger{buffer: vec![0; threshold].into_boxed_slice(), writer, idx: 0}
	}
}

impl<W: io::Write> Logger<W> {
	pub fn log(&mut self, message: &str) -> io::Result<()> {

		if message.len() >= self.buffer.len() - self.idx {
			self.flush()?;
			self.writer.write_all(message.as_bytes())?;
			self.writer.write_all(b"\n")?;
		} else {
			self.buffer[self.idx..self.idx + message.len()].copy_from_slice(message.as_bytes());
			self.buffer[self.idx + message.len()] = b'\n';
			self.idx += message.len() + 1;
		}
		Ok(())
	}

	pub fn flush(&mut self) -> io::Result<()>{
		if self.idx > 0 {
			self.writer.write_all(&self.buffer[..self.idx])?;
			self.idx = 0;
		}
		Ok(())
	}
}

#[cfg(test)]
#[test]
fn test_buff() {
	let mut logger = Logger::new(11, std::io::stdout());

	logger.log("0123456789").unwrap();
	logger.log("0").unwrap();
	assert_eq!(11, logger.buffer.len());
	// logger.log("012345").unwrap();

	assert_eq!(b"0123456789", &logger.buffer[..10]);
	
}

#[cfg(test)]
#[test]
fn same_length() {
	let mut logger = Logger::new(10, std::io::stdout());

	logger.log("0123456789").unwrap();
	// logger.log("012345").unwrap();

	assert_eq!(b"\0\0\0\0\0\0\0\0\0\0", &logger.buffer[..10]);
	
}

#[cfg(test)]
#[test]
fn same_length_pnl() {
	let mut logger = Logger::new(10, std::io::stdout());

	logger.log("012345678").unwrap();
	// logger.log("012345").unwrap();

	assert_eq!(b"012345678\n", &logger.buffer[..10]);
	
}

#[cfg(test)]
#[test]
fn buffered_messages() {
	let mut logger = Logger::new(10, std::io::stdout());

	logger.log("\0\0\0\06789\n").unwrap();
	// logger.log("012345").unwrap();

	assert_eq!(b"\0\0\0\06789\n", &logger.buffer[..9]);
	
}

#[cfg(test)]
#[test]
fn no_buffer() {
	let mut logger = Logger::new(0, std::io::stdout());

	logger.log("\0\0\0\06789\n").unwrap();
	// logger.log("012345").unwrap();

	assert_eq!(b"", &logger.buffer[..0]);
	
}

#[cfg(test)]
#[test]
fn no_buffer_multi_words() {
	let mut logger = Logger::new(0, std::io::stdout());

	logger.log("hello world").unwrap();
	// logger.log("012345").unwrap();

	assert_eq!(b"", &logger.buffer[..0]);
	
}

fn main() {
	let logger = Arc::new(Mutex::new(Logger::new(50, std::io::stdout())));

	let mut handles = Vec::with_capacity(10);

	for i in 0..10 {
		let logger = Arc::clone(&logger);
		handles.push(std::thread::spawn(move || {
			for x in 0..10 {
				let mut val = logger.lock().unwrap();
				if val.log(&format!("hello {} from  thread {}!",x, i )).is_err() {
					println!("Error when writing!")
				};
			}
			if logger.lock().unwrap().flush().is_err() {
				println!("Error when writing!")
			};
		}));
	}

	for handle in handles {
		if handle.join().is_err() {
			println!("Join failed!");
		}
	}
}
