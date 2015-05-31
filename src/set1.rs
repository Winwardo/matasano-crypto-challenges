use attacks::*;
use aes::*;
use byte_conversion::*;
use general_utilities::*;

pub fn problem_3() {
	// http://cryptopals.com/sets/1/challenges/3/
	let encoded = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

	let (top_score, top_decode, top_x) = guess_single_xor_char_decode(&encoded);

	println!("Best guess: `{}`, with a score of {}, using character {}.", bytes_to_readable_text(&top_decode), top_score, top_x);
}

pub fn problem_4() {
	// http://cryptopals.com/sets/1/challenges/4/

	let s = read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\4.txt");

	let mut top_score = 0;
	let mut top_decode = vec![];
	for line in s.split("\n") {
		let (score, decoded, _) = guess_single_xor_char_decode(&hex_to_bytes(line));
		
		if score > top_score {
			top_score = score;
			top_decode = decoded.clone();
		}
	}

	println!("\n\n\n{} -      `{}`", top_score, bytes_to_readable_text(&top_decode));
}

pub fn problem_6() {
	// http://cryptopals.com/sets/1/challenges/6/

	let file_data = ::general_utilities::read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\6.txt");
	let data_bytes = base64_to_bytes(&file_data);

	let (top_score, top_decode, top_key) = guess_repeating_xor_key(&data_bytes, 40);

	println!("{}, {}, \n{:?}", bytes_to_readable_text(&top_key), top_score, bytes_to_readable_text(&top_decode));
}

pub fn problem_7() {
	// http://cryptopals.com/sets/1/challenges/7/

	let file_data = read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\7.txt");
	let data_bytes = base64_to_bytes(&file_data);

	let key = readable_text_to_bytes("YELLOW SUBMARINE");
	assert!(key.len() == 16);

	let decrypted_data_ = decrypt_aes_128_ecb_no_padding(&data_bytes[..], &key);

	match decrypted_data_ {
		Ok(v)  => println!("{:?}", bytes_to_readable_text(&v)),
		Err(e) => println!("Error decrypting data: {:?}", e),
	}
}

pub fn problem_8() {
	// http://cryptopals.com/sets/1/challenges/8/
	let file_data = ::general_utilities::read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\8.txt");
	
	// Now split into a vector of bytes
	let bytes: Vec<Vec<u8>> = file_data
		.split("\r\n")
		.map(|x| hex_to_bytes(x))
		.collect();

	let key_size = 16usize;
	let mut ecb_candidates = Vec::new();

	for ciphertext in &bytes {
		if detect_ecb(&ciphertext, key_size, 2) {
			ecb_candidates.push(bytes_to_hex(ciphertext));
		}
	}

	println!("{:?}", ecb_candidates);
}

pub fn set1() {
	problem_3();
	problem_4();
	problem_6();
	problem_7();
	problem_8();
}