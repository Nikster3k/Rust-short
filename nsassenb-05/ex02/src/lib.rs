use std::cell::Cell;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
enum Error {
    Success,
    FileNotFound,
    IsDirectory,
    WriteFail,
    ReadFail,
}

std::thread_local! {
    pub static ERR: Cell<Error> = const { Cell::new(Error::Success) };   
}

#[allow(dead_code)]
impl Error {
    fn last() -> Self {
        ERR.get()
    }

    fn make_last(self) {
        ERR.replace(self);
    }
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		Error::make_last(Error::IsDirectory);

		std::assert_eq!(Error::last(), Error::IsDirectory)
	}
}
