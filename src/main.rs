#![allow(warnings)]

mod byte_conversion;
mod byte_manipulation;
mod english_scoring;

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
	use byte_conversion::*;
	let encoded = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

	let (top_score, top_decode) = guess_single_xor_char_decode(&encoded);

	println!("Best guess: `{}`, with a score of {}.", bytes_to_readable_text(&top_decode), top_score);
}

fn problem_4() {
	use std::error::Error;
	use std::fs::File;
	use std::io::prelude::*;
	use std::path::Path;
	use byte_conversion::*;

	// Create a path to the desired file
    let path = Path::new("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\4.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {},
    }

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

fn main() {
	problem_4();
}