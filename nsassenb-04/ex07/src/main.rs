use rand::Rng;
use rug::{rand::RandGen, Integer};

fn is_prime(n: &Integer) -> bool {
	if n <= &Integer::from(1) {
		return false;
	}
	if n <= &Integer::from(3) {
		return true;
	}
	if n % 2 == 0 || n % 3 == 0 {
		return false;
	}
	let mut i = 5;
	while i * i <= n {
		if n % i == 0 || n % (i + 2) == 0 {
			return false;
		}
		i += 6;
	}
	true
}

fn generate_prime() -> Integer {
	let mut rng = rand::thread_rng();
	loop {
		let candidate = rng.gen_range(2..Integer::MAX);
		if is_prime(candidate) {
			return candidate;
		}
	}
}

fn gcd(a: &Integer, b: &Integer) -> Integer {
	if b == 0 {
		return a;
	}
	gcd(b, a % b)
}

fn generate_e(phi: &Integer, m: &Integer) -> Integer {
	let mut rng = rand::thread_rng();
	loop {
		let candidate = rng.gen_range(0..phi);
		if gcd(candidate, phi) == 1 && gcd(candidate, m) == 1 {
			return candidate;
		}
	}
}

fn encrypt(message: String) -> String{
	let mut enc = String::new();
}

fn main() {
	let p = generate_prime();
	let q = generate_prime();
	let m = p.wrapping_mul(q);
	let phi = (p.wrapping_sub(1)).wrapping_mul(q.wrapping_sub(1));

	let e = generate_e(phi, m);

	println!("p: {}, q: {}, m: {}, phi: {}, e: {}", p, q, m, phi, e);
}
