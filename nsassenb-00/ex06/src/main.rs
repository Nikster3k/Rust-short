use ftkit::random_number;
use ftkit::read_number;

fn main() {
    let rand = random_number(0..100);
    let mut input: i32;
	std::println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
    loop {
        input = read_number();
		let f = input.cmp(&rand);
        if f.is_eq() {
            std::println!("That is right! The secret was indeed the number {}, which you have brilliantly discovered!", rand);
            break;
        } else if f.is_lt() {
			std::println!("This student might not be as smart as I was told. This answer is obviously too weak.");
		} else {
			std::println!("Sometimes I wonder whether I should retire. I would have guessed higher.");
		}
    }
}
