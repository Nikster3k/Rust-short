use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::clone::Clone;
use std::ops::{Deref, DerefMut};
use std::ptr::*;


struct Tableau<T> {
    data: std::ptr::NonNull<T>,
    len: usize,
    capacity: usize,
    idx: usize,
}

#[allow(dead_code)]
impl<T> Tableau<T> {
    fn new() -> Self {
        let layout = Layout::array::<T>(1).expect("Failed to create layout");
        // SAFETY: We are allocating memory for one element of type T. If allocation fails, handle_alloc_error is called.
        unsafe {
            let ptr = alloc(layout) as *mut T;
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            Tableau { data: NonNull::new_unchecked(ptr), len: 0, capacity: 1, idx: 0 }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn push(&mut self, item: T) {
        if self.len == self.capacity {
            let new_capacity = self.capacity * 2;
            let layout = Layout::array::<T>(new_capacity).expect("Failed to create layout");
            // SAFETY: We are reallocating memory and copying existing elements to the new location.
            unsafe {
                let new_data = alloc(layout) as *mut T;
                if new_data.is_null() {
                    dealloc(self.data.as_ptr() as *mut u8, Layout::array::<T>(self.capacity).expect("Failed to create layout"));
                    handle_alloc_error(layout);
                }
                std::ptr::copy_nonoverlapping(self.data.as_ptr(), new_data, self.len);
                dealloc(self.data.as_ptr() as *mut u8, Layout::array::<T>(self.capacity).expect("Failed to create layout"));
                self.data = NonNull::new_unchecked(new_data);
                self.capacity = new_capacity;
            }
        }
        // SAFETY: we know memory is big enough to be able to add item to array
        unsafe {
            std::ptr::write(self.data.as_ptr().add(self.len), item);
            self.len += 1;
        }
    }
    
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // SAFETY: We are reading from an allocated memory location within bounds.
            unsafe { Some(read(self.data.add(self.len).as_ptr())) }
        }
    }

    fn clear(&mut self) {
        self.len = 0;
    }
}

impl<T> Deref for Tableau<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        // SAFETY: We are creating a slice from a valid pointer and length.
        unsafe {
            &*std::ptr::slice_from_raw_parts(self.data.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for Tableau<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: We are creating a mutable slice from a valid pointer and length.
        unsafe {
            &mut *std::ptr::slice_from_raw_parts_mut(self.data.as_ptr(), self.len)
        }
    }
}

impl<T> Drop for Tableau<T> {
    fn drop(&mut self) {
        // SAFETY: We are deallocating memory that was previously allocated.
        unsafe {
            dealloc(self.data.as_ptr() as *mut u8, Layout::array::<T>(self.capacity).expect("Failed to create layout"));
        }
    }
}

impl<T> Clone for Tableau<T> {
    fn clone(&self) -> Self {
        let mut new_tableau = Tableau::new();
        new_tableau.capacity = self.capacity;
        new_tableau.len = self.len;

        let layout = Layout::array::<T>(self.capacity).expect("Failed to create layout");
        // SAFETY: We are allocating memory and copying existing elements to the new location.
        unsafe {
            let new_data = alloc(layout) as *mut T;
            if new_data.is_null() {
                handle_alloc_error(layout);
            }
            for i in 0..self.len {
                new_data.add(i).write(self.data.as_ptr().add(i).read());
            }
            dealloc(new_tableau.data.as_ptr() as *mut u8, Layout::array::<T>(new_tableau.capacity).expect("Failed to create layout"));
            new_tableau.data = NonNull::new_unchecked(new_data);
        }
        new_tableau
    }
}

impl<T> Iterator for Tableau<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            // SAFETY: We are reading from an allocated memory location within bounds.
            unsafe {
                let item = std::ptr::read(self.data.as_ptr().add(self.idx));
                self.idx += 1;
                Some(item)
            }
        } else {
            self.idx = 0;
            None
        }
    }
}

// #[cfg(test)]
// mod test {

//     use super::*;

//     #[test]
//     fn subj_tableu() {
//         let mut a = Tableau::<i32>::new();
//         a.push(1); a.push(2); a.push(4);
//         let b = a.clone();

//         for it in b {
//             println!("{it}");
//         }
//         // This will print:
//         // 1
//         // 2
//         // 4

//         let c: &[i32] = &*a;
//         assert_eq!(c, [1, 2, 4]);
//     }
// }

// fn main() {
//     let mut a = Tableau::<i32>::new();
//     a.push(1); a.push(2); a.push(4);
//     let b = a.clone();

//     for it in b {
//         println!("{it}");
//     }
//     // This will print:
//     // 1
//     // 2
//     // 4

//     // let c: &[i32] = &*a;
//     assert_eq!(*a, [1, 2, 4]);
// }