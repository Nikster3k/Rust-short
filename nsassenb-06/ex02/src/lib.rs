use std::alloc::{alloc, dealloc, Layout, handle_alloc_error};
use std::ops::{Deref, DerefMut};
use std::clone::Clone;
use std::marker::PhantomData;
use std::ptr::{NonNull, drop_in_place};

struct Carton<T> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

#[allow(dead_code)]
impl<T> Carton<T> {
    fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        unsafe {
            // SAFETY: 'alloc' returns a valid pointer or aborts if it fails.
            let ptr = alloc(layout) as *mut T;
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            // SAFETY: 'ptr; is non-null and properly aligned, and we just allocated it
            ptr.write(value);
            Carton {
                // SAFETY: `NonNull::new_unchecked` is safe because `ptr` is non-null.
                ptr: NonNull::new_unchecked(ptr),
                _marker: PhantomData,
            }
        }
    }

    fn into_inner(self) -> T {
        // SAFETY: 
        // `self.ptr` is valid and points to a properly initialized `T`.
        // `self.ptr` was allocated with `alloc` and has not been deallocated yet.
        unsafe {
            let value = std::ptr::read(self.ptr.as_ptr());
            dealloc(self.ptr.as_ptr() as *mut u8, Layout::new::<T>());
            // Prevent `Drop` from running.
            std::mem::forget(self);
            value
        }
    }
}

impl<T> Drop for Carton<T> {
    fn drop(&mut self) {
        // SAFETY: 
        //  `self.ptr` is valid and points to a properly initialized `T`.
        //  `self.ptr` was allocated with `alloc` and has not been deallocated yet.
        unsafe {
            drop_in_place(self.ptr.as_ptr());
            dealloc(self.ptr.as_ptr() as *mut u8, Layout::new::<T>());
        }
    }
}

impl<T> Deref for Carton<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `self.ptr` is valid and points to a properly initialized `T`.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for Carton<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `self.ptr` is valid and points to a properly initialized `T`.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: Clone> Clone for Carton<T> {
    fn clone(&self) -> Self {
        Carton::new((**self).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Point { x: u32, y: u32 }

    #[test]
    fn test_carton() {
        let point_in_carton = Carton::new(Point { x: 1, y: 2 });
        assert_eq!(point_in_carton.x, 1);
        assert_eq!(point_in_carton.y, 2);

        let mut another_point = point_in_carton.clone();
        another_point.x = 2;
        another_point.y = 3;
        assert_eq!(another_point.x, 2);
        assert_eq!(another_point.y, 3);
        assert_eq!(point_in_carton.x, 1);
        assert_eq!(point_in_carton.y, 2);
    }
}