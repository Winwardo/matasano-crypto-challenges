#![cfg_attr(test, allow(dead_code))]
#![allow(dead_code)]

extern crate crypto;
extern crate rand;

mod aes;
mod attacks;
mod byte_conversion;
mod byte_manipulation;
mod byte_utilities;
mod ciphertext;
mod english_scoring;
mod general_utilities;
mod key;

mod set1;
mod set2;

fn main() {
	println!("=====\nRunning.\n");

	set2::problem_9();

	println!("\nComplete.");
}