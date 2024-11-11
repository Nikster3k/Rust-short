use std::{fmt::Debug, str::FromStr};

#[derive(Debug)]
struct Time {
    hours: u32,
    minutes: u32,
}

#[derive(Debug)]
enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

impl FromStr for Time {
    type Err = TimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.len() != 5 {
            return Err(TimeParseError::InvalidLength);
        }

        let idx = match s.find(':') {
            Some(x) => x,
            None => return Err(TimeParseError::MissingColon),
        };

        // check if <digit><digit>:<digit><digit>
        let x = s.as_bytes();
        if !(x[0].is_ascii_digit()
            && x[1].is_ascii_digit()
            && x[2] == b':'
            && x[3].is_ascii_digit()
            && x[4].is_ascii_digit())
        {
            return Err(TimeParseError::InvalidNumber);
        }

        let t = Time {
            hours: s[0..idx].parse::<u32>().unwrap(),
            minutes: s[idx + 1..s.len()].parse::<u32>().unwrap(),
        };
		match (t.hours, t.minutes) {
			(24, 0) => Ok(t),
			(0..24, 0..60) => Ok(t),
			_ => Err(TimeParseError::InvalidNumber)
		}
    }
}

impl std::fmt::Display for TimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeParseError::InvalidLength => f.write_str("invalid length"),
            TimeParseError::InvalidNumber => f.write_str("invalid number"),
            TimeParseError::MissingColon => f.write_str("missing ':'"),
        }
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} hour{}, {} minute{}",
            self.hours,
            if self.hours != 1 { "s" } else { "" },
            self.minutes,
            if self.minutes != 1 { "s" } else { "" }
        ))
    }
}

fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();
    let c: Time = "15:01".parse().unwrap();

    println!("{a}");
    println!("{b}");
    println!("{c}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    let err4: TimeParseError = "12:60".parse::<Time>().unwrap_err();
    let err5: TimeParseError = "25:01".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
    println!("error: {err4}");
    println!("error: {err5}");
}
