#[allow(dead_code)]
fn add(a: &i32, b: i32) -> i32 {
	*a + b
}

#[allow(dead_code)]
fn add_assign(a: &mut i32, b: i32) {
	*a += b;
}
