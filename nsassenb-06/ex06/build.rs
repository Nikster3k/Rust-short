fn main() {
	cc::Build::new()
		.file("awesome.c")
		.compile("awesome");
}