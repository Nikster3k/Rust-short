use std::ffi::CStr;
use std::str::from_utf8_unchecked;
use libc::__errno_location;
use std::cmp::{PartialEq, Eq, PartialOrd, Ord};
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Errno(libc::c_int);

#[allow(dead_code)]
impl Errno {
    fn last() -> Self {
        // SAFETY: `__errno_location` returns a pointer to the thread-local errno variable.
        // Dereferencing this pointer is safe because it is guaranteed to be valid for the lifetime of the thread.
        Errno(unsafe { *__errno_location() })
    }

    fn make_last(self) {
        // SAFETY: `__errno_location` returns a pointer to the thread-local errno variable.
        // Assigning to this pointer is safe because it is guaranteed to be valid for the lifetime of the thread.
        unsafe { *__errno_location() = self.0 }
    }

    fn description(self) -> &'static str {
        // SAFETY: `libc::strerror` returns a pointer to a statically allocated string.
        // `CStr::from_ptr` is safe to use as long as the pointer is valid and points to a null-terminated string.
        // `from_utf8_unchecked` is safe because `strerror` returns a valid UTF-8 string.
        unsafe {
            let c_str = std::ffi::CStr::from_ptr(libc::strerror(self.0));
            from_utf8_unchecked(c_str.to_bytes())
        }
    }
}

impl Display for Errno {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // SAFETY: `libc::strerror` returns a pointer to a statically allocated string.
        // `CStr::from_ptr` is safe to use as long as the pointer is valid and points to a null-terminated string.
        // `from_utf8_unchecked` is safe because `strerror` returns a valid UTF-8 string.
        unsafe {
            let c_str = std::ffi::CStr::from_ptr(libc::strerror(self.0));
            f.write_str(from_utf8_unchecked(c_str.to_bytes()))
        }
    }
}

#[derive(Clone, Copy)]
struct Fd(libc::c_int);

#[allow(dead_code)]
impl Fd {
    const STDIN: Self = Fd(0);
    const STDOUT: Self = Fd(1);
    const STDERR: Self = Fd(2);

    fn open(file: &CStr) -> Result<Self, Errno> {
        // SAFETY: `libc::open` is called with a valid C string pointer.
        // The pointer is guaranteed to be valid for the duration of the call.
        unsafe {
            let fd = libc::open(file.as_ptr(), libc::O_RDONLY);
            if fd == -1 {
                Err(Errno::last())
            } else {
                Ok(Fd(fd))
            }
        }
    }

    fn create(file: &CStr) -> Result<Self, Errno> {
        // SAFETY: `libc::open` is called with a valid C string pointer.
        // The pointer is guaranteed to be valid for the duration of the call.
        unsafe {
            let fd = libc::open(file.as_ptr(), libc::O_CREAT | libc::O_TRUNC | libc::O_WRONLY);
            if fd == -1 {
                Err(Errno::last())
            } else {
                Ok(Fd(fd))
            }
        }
    }

    fn write(self, data: &[u8]) -> Result<usize, Errno> {
        // SAFETY: `libc::write` is called with a valid file descriptor and a valid buffer.
        // The buffer is guaranteed to be valid for the duration of the call.
        unsafe {
            let written = libc::write(self.0, data.as_ptr() as *const libc::c_void, data.len());
            if written == -1 {
                Err(Errno::last())
            } else {
                Ok(written as usize)
            }
        }
    }

    fn read(self, buffer: &mut [u8]) -> Result<usize, Errno> {
        // SAFETY: `libc::read` is called with a valid file descriptor and a valid buffer.
        // The buffer is guaranteed to be valid for the duration of the call.
        unsafe {
            let read = libc::read(self.0, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len());
            if read == -1 {
                Err(Errno::last())
            } else {
                Ok(read as usize)
            }
        }
    }

    fn close(self) -> Result<(), Errno> {
        // SAFETY: `libc::close` is called with a valid file descriptor.
        // Closing a file descriptor is safe as long as it is valid.
        unsafe {
            if libc::close(self.0) == -1 {
                Err(Errno::last())
            } else {
                Ok(())
            }
        }
    }
}

struct File(Fd);

#[allow(dead_code)]
impl File {
    fn open(file: &CStr) -> Result<Self, Errno> {
        // SAFETY: `libc::open` is called with a valid C string pointer.
        // The pointer is guaranteed to be valid for the duration of the call.
        unsafe {
            let fd = libc::open(file.as_ptr(), libc::O_RDONLY);
            if fd == -1 {
                Err(Errno::last())
            } else {
                Ok(File(Fd(fd)))
            }
        }
    }

    fn create(file: &CStr) -> Result<Self, Errno> {
        // SAFETY: `libc::open` is called with a valid C string pointer.
        // The pointer is guaranteed to be valid for the duration of the call.
        unsafe {
            let fd = libc::open(file.as_ptr(), libc::O_CREAT | libc::O_TRUNC | libc::O_WRONLY);
            if fd == -1 {
                Err(Errno::last())
            } else {
                Ok(File(Fd(fd)))
            }
        }
    }

    fn write(&self, data: &[u8]) -> Result<usize, Errno> {
        // SAFETY: `libc::write` is called with a valid file descriptor and a valid buffer.
        // The buffer is guaranteed to be valid for the duration of the call.
        unsafe {
            let written = libc::write(self.0.0, data.as_ptr() as *const libc::c_void, data.len());
            if written == -1 {
                Err(Errno::last())
            } else {
                Ok(written as usize)
            }
        }
    }

    fn read(&self, buffer: &mut [u8]) -> Result<usize, Errno> {
        // SAFETY: `libc::read` is called with a valid file descriptor and a valid buffer.
        // The buffer is guaranteed to be valid for the duration of the call.
        unsafe {
            let read = libc::read(self.0.0, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len());
            if read == -1 {
                Err(Errno::last())
            } else {
                Ok(read as usize)
            }
        }
    }

    fn leak(self) -> Fd {
        let fd = self.0.0;
        std::mem::forget(self);
        Fd(fd)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        self.0.close().unwrap_or_else(|err| {
            eprintln!("Failed to close file descriptor: {}", err);
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subj_errno() {
        Errno(12).make_last();
        assert_eq!(Errno::last(), Errno(12));

        let desc = format!("{}", Errno(1));
        // TODO: find what is the description of `Errno(1)`.
        assert_eq!(desc, "Operation not permitted");
    }

    #[test]
    fn test_open() {
        match Fd::create(cstr::cstr!("test")) {
            Ok(x) => {
                if let Ok(size) = x.write(b"HELLO WORLD") {
                    assert_eq!(size, 11);
                } else {
                    assert_eq!(Errno::last().description(), "");
                }
                assert!(x.0 >= 3);
            },
            Err(e) => assert_eq!(e.description(), "test")
        }
    }
}