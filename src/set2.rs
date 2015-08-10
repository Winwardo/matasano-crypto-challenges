use attacks::*;
use aes::*;
use byte_conversion::*;
use ciphertext::*;
use general_utilities::*;

pub fn problem_9() {
	// http://cryptopals.com/sets/2/challenges/9

	// Check out ciphertext.rs::PaddedText_yellow_submarine
	let text = match PaddedBytes::from_text("YELLOW SUBMARINE", 20) {
		Ok(x) => x,
		Err(x) => panic!(x),
	};

	let expected = ::byte_conversion::readable_text_to_bytes(&"YELLOW SUBMARINE\x04\x04\x04\x04");
	println!("Problem 9:\nexpected == text: {:?},\ntext: `{:?}`\n---\n", expected == text.bytes(), text.bytes());
}

pub fn problem_10() {
	// http://cryptopals.com/sets/2/challenges/10

}

pub fn set1() {
	problem_9();
}