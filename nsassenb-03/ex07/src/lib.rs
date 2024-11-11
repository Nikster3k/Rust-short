use std::fmt::Write;

macro_rules! impl_field_for_int {
	($($t:ident),*) => {
		$(
			impl Field for $t {
				fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
					match target.write_str(self.to_string().as_str()) {
						Ok(_) => Ok(()),
						Err(_) => Err(EncodingError)
					}
				}
			
				fn decode(field: &str) -> Result<Self, DecodingError> {
					match field.parse::<$t>() {
						Ok(val) => Ok(val),
						Err(_) => Err(DecodingError)
					}
				}
			}
		)*
	};
}

#[derive(Debug)]
struct EncodingError;
#[derive(Debug)]
struct DecodingError;

#[allow(dead_code)]
trait Field: Sized {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError>;
    fn decode(field: &str) -> Result<Self, DecodingError>;
}

// ez
impl_field_for_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

trait Record: Sized {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError>;
    fn decode(line: &str) -> Result<Self, DecodingError>; 
}

impl Field for String {
	fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
		if self.find([',', '\n']).is_some() {
			return Err(EncodingError);
		}
		match target.write_str(self) {
			Ok(_) => Ok(()),
			Err(_) => Err(EncodingError)
		}
	}
	
	fn decode(field: &str) -> Result<Self, DecodingError> {
		Ok(field.to_string())
	}
}

impl<T: Field> Field for Option<T> {
	fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
		match self {
			Some(x) => x.encode(target),
			None => Ok(()),
		}
	}

	fn decode(field: &str) -> Result<Self, DecodingError> {
		if field.is_empty() {
			Ok(None)
		} else {
			match T::decode(field) {
				Ok(res) => Ok(Some(res)),
				Err(_) => Err(DecodingError),
			}
		}
	}
}

#[allow(dead_code)]
fn encode_csv<R: Record>(records: &[R]) -> Result<String, EncodingError>{
	let mut target_str = String::with_capacity(1000);
	for rec in records {
		rec.encode(&mut target_str)?;
		target_str.push('\n');
	}
	Ok(target_str)
}

#[allow(dead_code)]
fn decode_csv<R: Record>(contents: &str) -> Result<Vec<R>, DecodingError>{
	let mut results = Vec::<R>::new();
	for line in contents.lines() {
		match R::decode(line) {
			Ok(res) => results.push(res),
			Err(error) => return Err(error)
		}
	}
	Ok(results)
}


#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn test_encode_u8() {
		let mut s = String::with_capacity(15);
		let _res = 15u8.encode(&mut s);
		std::println!("{}", s);
		assert_eq!(s, "15");
	}

	#[test]
	fn test_encode_u16() {
		let mut s = String::with_capacity(15);
		let _res = 15u16.encode(&mut s);
		std::println!("{}", s);
		assert_eq!(s, "15");
	}

	#[test]
	fn test_encode_string() {
		let s = "what the fuck".to_string();
		let mut target = String::with_capacity(15);

		let _res = s.encode(&mut target);
		std::print!("{}", target);
		assert_eq!(target, s);
	}

	#[test]
	#[should_panic]
	fn test_encode_string_separators() {
		let s = "what,the,fuck".to_string();
		let mut target = String::with_capacity(15);

		let _res = s.encode(&mut target);
		assert_eq!(target, s);
	}
}


#[cfg(test)]
#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
}

#[cfg(test)]
impl Record for User {
	fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
		self.name.encode(target)?;
		target.push(',');
		self.age.encode(target)?;
		Ok(())
	}

	fn decode(line: &str) -> Result<Self, DecodingError> {
		let mut parts = line.split(',');

		let name = match parts.next() {
			Some(field) => String::decode(field),
			None => return Err(DecodingError),
		};
		let age = match parts.next() {
			Some(field) => u32::decode(field),
			None => return Err(DecodingError)
		};

		if parts.next().is_some() {
			return Err(DecodingError);
		}

		let name = name?;
		let age = age?;
		Ok(User { name, age })
	}
}

#[cfg(test)]
#[test]
fn test_encode() {
    let database = [
        User { name: "aaa".into(), age : 23 },
        User { name: "bb".into(), age: 2 },
    ];

    let csv = encode_csv(&database).unwrap();

    assert_eq!(
        csv,
        "\
        aaa,23\n\
        bb,2\n\
        "
    );
}

#[cfg(test)]
#[test]
fn test_decode() {
    let csv = "\
        hello,2\n\
        yes,5\n\
        no,100\n\
    ";

    let database: Vec<User> = decode_csv(csv).unwrap();

    assert_eq!(
        database,
        [
            User { name: "hello".into(), age: 2 },
            User { name: "yes".into(), age: 5 },
            User { name: "no".into(), age: 100 },
        ]
    );
}

#[cfg(test)]
#[test]
fn decoding_error() {
    let csv = "\
        hello,2\n\
        yes,6\n\
        no,23,hello\n\
    ";

    decode_csv::<User>(csv).unwrap_err();
}
