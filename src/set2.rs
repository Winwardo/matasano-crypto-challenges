use attacks::*;
use aes::*;
use byte_conversion::*;
use ciphertext::*;
use general_utilities::*;
use key::*;
use message_operator::*;

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

	let s = read_file("C:\\Users\\Topher\\Documents\\GitHub\\matasano-crypto-challenges\\res\\10.txt");
	
	let message = base64_to_bytes(&s);
	let block_size = 16;
	let block_operator = "CBC";
	let IV = "\x00";
	let key = "YELLOW SUBMARINE";

	let mo_decrypt = MessageOperator {
		message: message,
		block_operator: block_operator.to_string(),
		IV: RepeatingKey::new(&IV).of_length(block_size),
		block_size: block_size,
		key: readable_text_to_bytes(&key),
	};
	
	println!("{}", bytes_to_readable_text(&mo_decrypt.decrypt_raw()));
}

pub fn set2() {
	problem_9();
	problem_10();
}