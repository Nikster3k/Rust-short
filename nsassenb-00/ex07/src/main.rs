use ftkit::ARGS;
use module00_ex07::lib::strpcmp;

fn main(){
	if ARGS.len() != 3 {
		std::println!("Usage <program> <query> <pattern>");
		return;
	}
	let query = ARGS[1].as_bytes();
    let pattern = ARGS[2].as_bytes();

    let result = strpcmp(query, pattern);
	if result {
		std::println!("yes");
	} else {
		std::println!("no");
	}
}
