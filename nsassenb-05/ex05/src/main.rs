use std::env;

use rand::{self, thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;

#[allow(dead_code)]
fn monte_carlo(size: usize) -> f64{
	let mut rng = thread_rng();
	let mut inside = 0;
	let mut out = 0;
	for _ in 0..size {
		let y: f64 = rng.gen();
		let x: f64 = rng.gen();
		let dist = ((x * x) + (y * y)).sqrt();
		if dist < 1.0 {
			inside += 1;
		} else {
			out += 1;
		}
	}
	(4 * inside) as f64 / (out + inside) as f64
}

#[allow(dead_code)]
fn monte_carlo_rayon(size: usize) -> f64 {
	let dist = Uniform::new(0.0, 1.0);

	let results: Vec<f64> = (0..size).into_par_iter().map(|_| {
		let x: f64 = dist.sample(&mut thread_rng());
		let y: f64 = dist.sample(&mut thread_rng());
		((x * x) + (y * y)).sqrt()
	}).collect();

	let mut ins: usize = 0;
	let mut out: usize = 0;

	for dist in results {
		if dist < 1.0 {
			ins += 1;
		} else {
			out += 1;
		}
	}

	(4 * ins) as f64 / (out + ins) as f64
}

fn main() {

	if env::args().len() != 2 {
		println!("usage: program <number of iterations>");
		return;
	}

	let iter = match env::args().nth(1).unwrap().parse::<usize>() {
		Ok(value) => value,
		Err(_) => {
			println!("Failed to parse argument to number!");
			return;
		}
	};

	let start = std::time::Instant::now();
	let mut pi = monte_carlo_rayon(iter);
	let end = std::time::Instant::now();
	if pi.is_nan() {
		pi = 0.0;
	}
	println!("pi: {}\nduration: {:?}", pi, end - start);
}
