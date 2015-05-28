pub fn guess_single_xor_char_decode(bytes: &Vec<u8>) -> (u16, Vec<u8>, u8) {
	use english_scoring::*;
	use key::*;

	let mut top_score = 0;
	let mut top_decode = vec![];
	let mut top_x = 2;
	for x in 0..255 {
		let mut rk = RepeatingKey::new_bytes(vec![x]);
		let decoded = rk.encrypt_bytes(&bytes);

		let score = score_on_letter_frequency(&decoded);
		if score > top_score {
			top_score = score;
			top_decode = decoded;
			top_x = x;
		}
	}

	(top_score, top_decode, top_x)
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
}
