#![cfg_attr(test, allow(dead_code))]
#![allow(dead_code)]

extern crate crypto;
extern crate rand;

mod aes;
mod attacks;
mod byte_conversion;
mod byte_manipulation;
mod byte_utilities;
mod english_scoring;
mod general_utilities;
mod key;

mod set1;

fn main() {
	println!("=====\nRunning.\n");

	set1::set1();

	println!("\nComplete.");
}