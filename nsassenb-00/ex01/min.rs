fn min(a: i32, b: i32) -> i32 {
	if a < b {
		a
	} else {
		b
	}
}

fn main(){
	min(5, 7);
	min(9, 5);
	min(4, 11);
}
