use std::sync::atomic::{AtomicU8, Ordering};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
struct Unique(u8);

impl Unique {
    pub fn new() -> Self {
        static COUNTER: AtomicU8 = AtomicU8::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        if id == u8::MAX {
            panic!();
        }
        Unique(id)
    }
}

impl Clone for Unique {
	fn clone(&self) -> Self {
		Unique::new()
	}
}


#[cfg(test)]
#[test]
fn test()
{
    let a = Unique::new();
    let b = Unique::new();
    let c = Unique::new();

    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");

    let d = a.clone();
    let e = c.clone();

    println!("{d:?}");
    println!("{e:?}");
}
