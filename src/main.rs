#![cfg_attr(test, allow(dead_code))]

mod byte_conversion;
mod byte_manipulation;
mod english_scoring;
mod general_utilities;

fn guess_single_xor_char_decode(bytes: &Vec<u8>) -> (u16, Vec<u8>) {
	let mut top_score = 0;
	let mut top_decode = vec![];
	for x in 0..255 {
		let mut spin: Vec<u8> = vec![];
		while spin.len() < bytes.len() {
			spin.push(x);
		}

		let decoded = byte_manipulation::xor_byte_streams(&bytes, &spin);
		let score = english_scoring::score_combined(&decoded);
		if score > top_score {
			top_score = score;
			top_decode = decoded;
		}
	}

	(top_score, top_decode)
}


fn problem_3() {
	// http://cryptopals.com/sets/1/challenges/3/
	use byte_conversion::*;
	let encoded = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

	let (top_score, top_decode) = guess_single_xor_char_decode(&encoded);

	println!("Best guess: `{}`, with a score of {}.", bytes_to_readable_text(&top_decode), top_score);
}

fn problem_4() {
	// http://cryptopals.com/sets/1/challenges/4/

	use byte_conversion::*;

	let s = general_utilities::read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\4.txt");

	let mut top_score = 0;
	let mut top_decode = vec![];	
    for line in s.split("\n") {
    	let (score, decoded) = guess_single_xor_char_decode(&hex_to_bytes(line));
		
		if score > top_score {
			top_score = score;
			top_decode = decoded.clone();
		}

    }

    println!("\n\n\n{} -      `{}`", top_score, bytes_to_readable_text(&top_decode));
}

fn problem_6() {
	// http://cryptopals.com/sets/1/challenges/6/

	use byte_conversion::*;

	let file_data = general_utilities::read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\6.txt");
	let data_bytes = readable_text_to_bytes(&file_data);
	for keysize in 2..40 {

		//let  Vec<u8> 
	}
}

fn main() {
	println!("Running.");

	problem_6();
}