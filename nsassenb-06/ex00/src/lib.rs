#![forbid(unsafe_op_in_unsafe_fn)]

use std::ptr::{write, read};

/// Swap the values of two mutable references
///
/// # Safety
///
/// - The caller must ensure that `a` and `b` are valid, non-overlapping pointers.
/// - The caller must ensure that `a` and `b` point to initialized memory.
/// - The caller must ensure that `a` and `b` remain valid for the duration of the swap.
#[allow(dead_code)]
fn ft_swap<T>(a: &mut T, b: &mut T) {
    // SAFETY:
    // - The caller must ensure that `a` and `b` are valid, non-overlapping pointers.
    // - The caller must ensure that `a` and `b` point to initialized memory.
    // - The caller must ensure that `a` and `b` remain valid for the duration of the swap.
    unsafe {
        let temp = read(a);
        write(a, read(b));
        write(b, temp);
    }
}

/// Count the number of non '\0' characters in a string
/// 
/// # Safety
/// 
/// s must be a valid pointer to a null terminated string
#[allow(dead_code)]
unsafe fn ft_strlen(s: *const u8) -> usize {
    let mut len = 0;
    let mut ptr = s;
    // SAFETY:
    //  - s is null terminated
    //  - s is valid string
    unsafe {
        while *ptr != b'\0' {
            ptr = ptr.add(1);
            len += 1;
        }
        len
    }
}

/// Copies content of one string to another
/// 
/// # Safety
/// 
/// both dst and src must be valid pointers
/// dst must be at least the size of src
#[allow(dead_code)]
unsafe fn ft_strcpy(dst: *mut u8, src: *const u8) {
    // SAFETY:
    //  - We have been given a null terinated string
    //  - dst has enough space to fit src
    //  - user has to provide valid strings
    let mut a = dst;
    let mut b = src;
    unsafe {
        while *b != b'\0' {
            *a = *b;
            a = a.add(1);
            b = b.add(1);
        }
        *a = *b;
    }
}


#[cfg(test)]
#[test]
fn subj_test() {
    let mut a = String::from("Hello, World!");
    let mut b = String::from("Goodby, World!");
    ft_swap(&mut a, &mut b);
    assert_eq!(a, "Goodby, World!");
    assert_eq!(b, "Hello, World!");

    let s = b"Hello, World!\0";
    // SAFETY:
    //  /* ... */
    let len = unsafe { ft_strlen(s.as_ptr()) };
    assert_eq!(len, 13);

    let mut dst = [0u8; 14];
    // SAFETY:
    //  /* ... */
    unsafe { ft_strcpy(dst.as_mut_ptr(), s.as_ptr()) };
    assert_eq!(&dst, b"Hello, World!\0");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_integers() {
        let mut x = 1;
        let mut y = 2;
        ft_swap(&mut x, &mut y);
        assert_eq!(x, 2);
        assert_eq!(y, 1);
    }

    #[test]
    fn test_str_len() {
        unsafe {
            assert_eq!(ft_strlen(b"test\0".as_ptr()), 4);
        }
    }

    #[test]
    fn test_swap_strings() {
        let mut a = String::from("hello");
        let mut b = String::from("world");
        ft_swap(&mut a, &mut b);
        assert_eq!(a, "world");
        assert_eq!(b, "hello");
    }
}