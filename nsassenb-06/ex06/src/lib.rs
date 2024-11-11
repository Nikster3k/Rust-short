use std::mem::MaybeUninit;

#[link(name = "awesome", kind = "static")]
extern "C" {
    fn create_database(database: *mut Database) -> i32;
    fn delete_database(database: *mut Database) -> i32;
    fn create_user(database: *mut Database, name: *const std::ffi::c_char, result: *mut Id) -> i32;
    fn delete_user(database: *mut Database, id: Id) -> i32;
    fn get_user(database: *const Database, id: Id, result: *mut *const User) -> i32;
}

#[derive(Debug)]
enum Error {
    Memory,
    NoMoreIds,
    UnknownId,
    Unknown,
}

impl From<i32> for Error {
    fn from(code: i32) -> Self {
        match code {
            1 => Error::Memory,
            2 => Error::NoMoreIds,
            3 => Error::UnknownId,
            _ => Error::Unknown,
        }
    }
}

type Id = std::ffi::c_uint;

#[allow(dead_code)]
#[repr(C)]
struct User {
    id: Id,
    name: *const std::ffi::c_char,
}

#[allow(dead_code)]
#[repr(C)]
struct Database {
    next_user_id: Id,
    users: MaybeUninit<User>,
    count: usize,
    allocated: usize,
}

#[allow(dead_code)]
impl Database {
    fn new() -> Self {
        let mut db = Database {
            next_user_id: 0,
            users: MaybeUninit::uninit(),
            count: 0,
            allocated: 0,
        };
        unsafe {
            create_database(&mut db);
        }
        db
    }

    fn create_user(&mut self, name: &std::ffi::CStr) -> Result<Id, Error> {
        let mut id: Id = 0;
        let result = unsafe { create_user(self, name.as_ptr(), &mut id) };
        if result == 0 {
            Ok(id)
        } else {
            Err(Error::from(result))
        }
    }

    fn delete_user(&mut self, id: Id) -> Result<(), Error> {
        let result = unsafe { delete_user(self, id) };
        if result == 0 {
            Ok(())
        } else {
            Err(Error::from(result))
        }
    }

    fn get_user(&self, id: Id) -> Result<&User, Error> {
        let mut user_ptr: *const User = std::ptr::null();
        let result = unsafe { get_user(self, id, &mut user_ptr) };
        if result == 0 {
            unsafe { Ok(&*user_ptr) }
        } else {
            Err(Error::from(result))
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe {
            delete_database(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_database_creation_and_deletion() {
        let _db = Database::new();
        // Database should be created and dropped without errors
    }

    #[test]
    fn test_user_creation() {
        let mut db = Database::new();
        let name = CString::new("Alice").expect("CString::new failed");
        let user_id = db.create_user(&name).expect("Failed to create user");
        assert_eq!(user_id, 1);
    }

    #[test]
    fn test_user_deletion() {
        let mut db = Database::new();
        let name = CString::new("Bob").expect("CString::new failed");
        let user_id = db.create_user(&name).expect("Failed to create user");
        db.delete_user(user_id).expect("Failed to delete user");
    }

    #[test]
    fn test_get_user() {
        let mut db = Database::new();
        let name = CString::new("Charlie").expect("CString::new failed");
        let user_id = db.create_user(&name).expect("Failed to create user");
        let user = db.get_user(user_id).expect("Failed to get user");
        let user_name = unsafe { std::ffi::CStr::from_ptr(user.name) };
        assert_eq!(user_name.to_str().expect("Failed to convert name"), "Charlie");
        let user_name = unsafe { std::ffi::CStr::from_ptr(user.name) };
        assert_eq!(user_name.to_str().expect("Failed to convert name"), "Charlie");
    }
}