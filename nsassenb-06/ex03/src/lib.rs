use std::cell::UnsafeCell;

struct Cellule<T> {
    cell: UnsafeCell<T>
}

#[allow(dead_code)]
impl<T> Cellule<T> {
    const fn new(value: T) -> Self {
        Cellule::<T>{cell: UnsafeCell::<T>::new(value)}
    }

    fn set(&self, value: T) {
        // SAFETY: `UnsafeCell::get` returns a raw pointer to the contained value.
        // We are the only ones accessing this value, so it is safe to dereference and assign to it.
        unsafe { *self.cell.get() = value; }
    }

    fn replace(&self, value: T) -> T {
        // SAFETY: `UnsafeCell::get` returns a raw pointer to the contained value.
        // We are the only ones accessing this value, so it is safe to read from it and assign a new value.
        unsafe {
            let temp = std::ptr::read(self.cell.get());
            *self.cell.get() = value;
            temp
        }
    }

    fn get(&self) -> T where T: Copy {
        unsafe { *self.cell.get() }
    }

    fn get_mut(&mut self) -> &mut T {
        self.cell.get_mut()
    }

    fn into_inner(self) -> T {
        self.cell.into_inner()
    }
}


#[cfg(test)]
#[test]
fn test() {
    let c = Cellule::new(5);

    c.replace(15);

    assert_eq!(c.into_inner(), 15);
}

#[cfg(test)]
#[test]
fn test_get() {
    let c = Cellule::new(5);

    c.replace(15);

    assert_eq!(c.get(), 15);
}
