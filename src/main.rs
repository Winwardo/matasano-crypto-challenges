#![allow(warnings)]

mod byte_conversion;
mod byte_manipulation;
mod english_scoring;

fn problem_3() {
	use byte_conversion::*;
	let encoded = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

	let mut top_score = 0;
	let mut top_decode = vec![];
	for x in 0..255 {
		let mut spin: Vec<u8> = vec![];
		while spin.len() < encoded.len() {
			spin.push(x);
		}

		let decoded = byte_manipulation::xor_byte_streams(&encoded, &spin);
		let score = english_scoring::score_combined(&decoded);
		if score > top_score {
			top_score = score;
			top_decode = decoded;
		}
	}

	println!("Best guess: `{}`, with a score of {}.", bytes_to_readable_text(&top_decode), top_score);
}

fn main() {
	problem_3();
}