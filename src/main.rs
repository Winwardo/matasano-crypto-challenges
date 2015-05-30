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

use std::collections::HashMap;

fn problem_3() {
	// http://cryptopals.com/sets/1/challenges/3/
	use byte_conversion::*;
	let encoded = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

	let (top_score, top_decode, top_x) = attacks::guess_single_xor_char_decode(&encoded);

	println!("Best guess: `{}`, with a score of {}, using character {}.", bytes_to_readable_text(&top_decode), top_score, top_x);
}

fn problem_4() {
	// http://cryptopals.com/sets/1/challenges/4/

	use byte_conversion::*;

	let s = general_utilities::read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\4.txt");

	let mut top_score = 0;
	let mut top_decode = vec![];	
    for line in s.split("\n") {
    	let (score, decoded, _) = attacks::guess_single_xor_char_decode(&hex_to_bytes(line));
		
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
	let data_bytes = base64_to_bytes(&file_data);

	let (top_score, top_decode, top_key) = attacks::guess_repeating_xor_key(&data_bytes, 40);

	println!("{}, {}, \n{:?}", bytes_to_readable_text(&top_key), top_score, bytes_to_readable_text(&top_decode));
}

fn problem_7() {
	// http://cryptopals.com/sets/1/challenges/7/
	use byte_conversion::*;

	let file_data = general_utilities::read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\7.txt");
	let data_bytes = base64_to_bytes(&file_data);

	let key = readable_text_to_bytes("YELLOW SUBMARINE");
	assert!(key.len() == 16);

    let decrypted_data_ = aes::decrypt_aes_128_ecb_no_padding(data_bytes, key);

    match decrypted_data_ {
    	Ok(v)  => println!("{:?}", bytes_to_readable_text(&v)),
    	Err(e) => println!("Error decrypting data: {:?}", e),
    }
}

fn detect_ecb(ciphertext: &[u8]) -> bool {
	false
}

fn problem_8() {
	// http://cryptopals.com/sets/1/challenges/8/
	use byte_conversion::*;

	let file_data = general_utilities::read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\8.txt");
	
	// Now split into a vector of bytes
	let bytes: Vec<Vec<u8>> = file_data
		.split("\n")
		.map(|x| hex_to_bytes(x))
		.collect();

	let key_size = 16;
	let mut blocks_map: HashMap<&[u8], u8> = HashMap::new();
	let mut ecb_candidates: Vec<Vec<u8>> = Vec::new();

	for ciphertext in &bytes {
		for block in ciphertext.chunks(key_size) {
			let c = blocks_map.entry(&block).or_insert(0);
			*c += 1;

			if *c == 2 {
				ecb_candidates.push(general_utilities::slice_to_vec(ciphertext));
			}
		}
	}

	for (k, v) in blocks_map {
		if v > 1 {
			println!("{:?}, {:?}, {:?}", v, k, bytes_to_hex(&general_utilities::slice_to_vec(k)));
		}
	}

	println!("{:?}", ecb_candidates);


	//println!("{:?}, {} {} {}", bytes, bytes[0].len(), bytes[1].len(), bytes[2].len());
}


fn main() {
	println!("=====\nRunning.\n");

	problem_8();

	println!("\nComplete.");
}