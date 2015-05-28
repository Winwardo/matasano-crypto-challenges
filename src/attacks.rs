pub fn guess_single_xor_char_decode(bytes: &Vec<u8>) -> (u16, Vec<u8>, u8) {
	use english_scoring::*;
	use key::*;

	let mut top_score = 0;
	let mut top_decode = vec![];
	let mut top_x = 2;
	for x in 0..255 {
		let mut rk = RepeatingKey::new_bytes(&vec![x]);
		let decoded = rk.encrypt_bytes(&bytes);

		let score = score_combined(&decoded);
		if score > top_score {
			top_score = score;
			top_decode = decoded;
			top_x = x;
		}
	}

	(top_score, top_decode, top_x)
}

pub fn guess_repeating_xor_key(data_bytes: &Vec<u8>, max_keysize: usize) -> (u16, Vec<u8>, Vec<u8>) {
	// As described at http://cryptopals.com/sets/1/challenges/6/
	use byte_manipulation::*;
	use byte_utilities::*;
	use english_scoring::*;
	use general_utilities::*;
	use key::*;

	// Shortcut single character key guesses
	if (max_keysize == 1) {
		let (a, b, c) = guess_single_xor_char_decode(&data_bytes);
		return (a, b, vec![c]);
	}

	// For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes,
	// and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
	// The KEYSIZE with the smallest normalized edit distance is probably the key.
	//  You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances.
	let mut keysize_points = Vec::new();
	for keysize in 2..max_keysize+1 {
		if data_bytes.len() < keysize * 4 {
			continue;
		}

		let chunks: Vec<Vec<u8>> = data_bytes.chunks(keysize).map(|x| slice_to_vec(x)).collect();
		assert!(chunks.len() >= 4);

		let distance1 = hamming_distance(&chunks[0], &chunks[1]);
		let distance2 = hamming_distance(&chunks[1], &chunks[2]);
		let distance3 = hamming_distance(&chunks[2], &chunks[3]);
		let distance4 = hamming_distance(&chunks[3], &chunks[4]);
		let avg_distance: f32 = (distance1 + distance2 + distance3 + distance4) as f32 / 4.0;
		let normalized_distance = avg_distance / keysize as f32;

		let pair = (keysize, normalized_distance);
		keysize_points.push(pair);
	}

	// Sort into the best keysize values
	keysize_points.sort_by(|a, b| {
		use std::cmp::Ordering::*;

		let (_, x): (_, f32) = *a;
		let (_, y): (_, f32) = *b;

		// f32s require partial cmp
		x.partial_cmp(&y).unwrap_or(Equal)
	});

	let mut top_score = 0;
	let mut top_decode: Vec<u8> = Vec::new();
	let mut top_key: Vec<u8> = Vec::new();

	// Check the best 10, any more than that starts using up a lot of time.
	for y in 0..10 {
		if y >= keysize_points.len() {
			break;
		}
		let (best_keysize, _) = keysize_points[y];

		// Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
		let blocks: Vec<Vec<u8>> = data_bytes
			.chunks(best_keysize)
			.map(|x| slice_to_vec(x))
			.collect();

		// Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.
		let transposed_blocks: Vec<Vec<u8>> = transpose_chunks(&blocks);

		// Solve each block as if it was single-character XOR. You already have code to do this.
		//: Vec<(u16, String, u8)>
		let solved_key: Vec<_> = transposed_blocks.iter()
			.map(|x| {
				let (_, _, guess) = guess_single_xor_char_decode(x);
				guess
			})
			.collect();

		// For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block.
		// Put them together and you have the key.
		let mut repeating_key = RepeatingKey::new_bytes(&solved_key);
		let decoded = repeating_key.encrypt_bytes(&data_bytes);

		let score = score_combined(&decoded);
		if score > top_score {
			top_score = score;
			top_decode = decoded;
			top_key = solved_key;
		}
	}

	(top_score, top_decode, top_key)
}

//-----------------------------------------------------------------------------

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn guess_single_xor_char_decode_i_set_1_problem_3() {
		// http://cryptopals.com/sets/1/challenges/3/
		use byte_conversion::*;
		let encoded = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

		let (top_score, guessed_decode, guessed_character) = guess_single_xor_char_decode(&encoded);

		let expected = readable_text_to_bytes(&"Cooking MC's like a pound of bacon");

		assert_eq!(expected, guessed_decode);
		assert_eq!(88, guessed_character);
		assert!(top_score > 0);
	}

	#[test]
	fn guess_single_xor_char_decode_i_example() {
		// http://cryptopals.com/sets/1/challenges/3/
		use byte_conversion::*;
		use key::*;

		let mut rk = RepeatingKey::new("Q");
		let example = readable_text_to_bytes(&"Some super hard to decrypt (but still English!) text.");

		let encrypted = rk.encrypt_bytes(&example);
		let (top_score, guessed_decode, guessed_character) = guess_single_xor_char_decode(&encrypted);

		assert_eq!(example, guessed_decode);
		assert_eq!('Q' as u8, guessed_character);
		assert!(top_score > 0);
	}

	#[test]
	fn guess_repeating_xor_key_decode_i_single_character() {
		// http://cryptopals.com/sets/1/challenges/3/
		use byte_conversion::*;
		use key::*;

		let example_key: Vec<u8> = readable_text_to_bytes(&"Q");
		let mut rk = RepeatingKey::new_bytes(&example_key);
		let example = readable_text_to_bytes(&"Some super hard to decrypt (but still English!) text.");

		let encrypted = rk.encrypt_bytes(&example);
		let (top_score, guessed_decode, guessed_key) = guess_repeating_xor_key(&encrypted, 1);

		assert_eq!(example, guessed_decode);
		assert_eq!(example_key, guessed_key);
		assert!(top_score > 0);
	}
}
