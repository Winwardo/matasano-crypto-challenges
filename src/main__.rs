#![cfg_attr(test, allow(dead_code))]

mod byte_conversion;
mod byte_manipulation;
mod byte_utilities;
mod english_scoring;
mod general_utilities;
mod key;

fn guess_single_xor_char_decode(bytes: &Vec<u8>) -> (u16, Vec<u8>, u8) {
	let mut top_score = 0;
	let mut top_decode = vec![];
	let mut top_x = 0;
	for x in 0..255 {
		let mut spin: Vec<u8> = vec![];
		while spin.len() < bytes.len() {
			spin.push(x);
		}

		let decoded = byte_manipulation::xor_byte_streams(&bytes, &spin);
		let score = english_scoring::score_on_letter_frequency(&decoded);
		if score > top_score {
			top_score = score;
			top_decode = decoded;
			top_x = x;
		}
	}

	(top_score, top_decode, top_x)
}


fn problem_3() {
	// http://cryptopals.com/sets/1/challenges/3/
	use byte_conversion::*;
	let encoded = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

	let (top_score, top_decode, _) = guess_single_xor_char_decode(&encoded);

	println!("Best guess: `{}`, with a score of {}.", bytes_to_readable_text(&top_decode), top_score);
}

fn problem_4() {
	// http://cryptopals.com/sets/1/challenges/4/

	use byte_conversion::*;

	let s = general_utilities::read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\4.txt");

	let mut top_score = 0;
	let mut top_decode = vec![];	
    for line in s.split("\n") {
    	let (score, decoded, x) = guess_single_xor_char_decode(&hex_to_bytes(line));
		
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
	use byte_manipulation::*;
	use byte_utilities::*;
	use english_scoring::*;
	use general_utilities::*;
	use key::*;

	let file_data = general_utilities::read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\6.txt");
	let data_bytes = readable_text_to_bytes(&file_data);

	// For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes,
	// and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
	// The KEYSIZE with the smallest normalized edit distance is probably the key.
	//  You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances.
	let mut keysize_points = Vec::new();
	for keysize in 2..41 {
		if data_bytes.len() < keysize * 4 {
			continue;
		}

		let chunks: Vec<_> = data_bytes.chunks(keysize).map(|x| slice_to_vec(x)).collect();
		assert!(chunks.len() >= 4);

		let distance1 = hamming_distance(&chunks[0], &chunks[1]);
		let distance2 = hamming_distance(&chunks[1], &chunks[2]);
		let distance3 = hamming_distance(&chunks[2], &chunks[3]);
		let distance4 = hamming_distance(&chunks[0], &chunks[3]);
		let avg_distance: f32 = (distance1 + distance2 + distance3 + distance4) as f32 / 4.0;
		let normalized_distance = avg_distance / keysize as f32;

		let pair = (keysize, normalized_distance);
		keysize_points.push(pair);
	}

	// Find the 4 best keysize values
	keysize_points.sort_by(|a, b| {
		use std::cmp::Ordering::*;

		let (_, x) = *a;
		let (_, y) = *b;

		x.partial_cmp(&y).unwrap_or(Equal)
	});

	println!("{:?}", keysize_points);

	let (best_keysize, _) = keysize_points[0];
	println!("{:?}", best_keysize);

	// Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
	let blocks: Vec<Vec<u8>> = data_bytes
		.chunks(best_keysize)
		.map(|x| slice_to_vec(x))
		.collect();

	// Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.
	let transposed_blocks: Vec<Vec<u8>> = transpose_chunks(&blocks);

	// Solve each block as if it was single-character XOR. You already have code to do this.
	let solved_key: Vec<(u16, String, u8)> = transposed_blocks.iter()
		.map(|x| {
			let (a, b, c) = guess_single_xor_char_decode(x);
			(a, bytes_to_readable_text(&b), c)
		})
		.collect();

	// For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block.
	// Put them together and you have the key.
	//let final_blocks: Vec<Vec<u8>> = transpose_chunks(&solved_blocks);

	//let r: Vec<String> = final_blocks.iter().map(|x| bytes_to_readable_text(x)).collect();
	//println!("{:?}", transposed_blocks.iter().map(|x| bytes_to_readable_text(x)).collect());
	println!("{:?}", solved_key);

	let mut rk = RepeatingKey::new(&"\u{0}\u{3}\u{0}\u{3}\u{0}\u{0}\u{3}\u{3}\u{3}\u{0}\u{3}\u{0}\u{3}\u{0}\u{2}\u{3}\u{3}\u{2}\u{3}\u{0}");


	//println!("{:?}", bytes_to_readable_text(&rk.encrypt_bytes(&transposed_blocks)));
}

fn main() {
	println!("Running.");

	problem_6();
}