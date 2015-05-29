#![cfg_attr(test, allow(dead_code))]
#![allow(dead_code)]
#![feature(convert)]

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

use crypto::{ symmetriccipher };
use rand::{ Rng, OsRng };

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

	use crypto::symmetriccipher::SynchronousStreamCipher;

	//use std::rand::{OsRng, Rng};


	let file_data = general_utilities::read_file("C:\\Users\\Topher\\Dropbox\\Public\\Programming\\Matasano\\matasano-crypto-challenges\\res\\7.txt");
	let data_bytes = base64_to_bytes(&file_data);


	let key_ = readable_text_to_bytes("YELLOW SUBMARINE");
	assert!(key_.len() == 16);









	let message = "Hello World!";
	
	let mut q = readable_text_to_bytes(message);
	//while q.len() % 16 != 0 {
	//	q.push(0);
	//}
	

	//let q = message.as_bytes();

    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];

    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.

    //let mut rng = OsRng::new().ok().unwrap();
    //rng.fill_bytes(&mut key);
    //rng.fill_bytes(&mut iv);


    //let encrypted_data = encrypt(&q, &key, &iv).ok().unwrap();
    let mut encrypted_data = data_bytes.clone();
    //encrypted_data.push(16);
    //while encrypted_data.len() % 16 != 0 {
   // 	encrypted_data.push(16);
    //}

    //println!("enc {:?}", bytes_to_readable_text(&encrypted_data));
    println!("enc {:?}", encrypted_data.len());
    println!("dat {:?}", data_bytes.len());
    //println!("q   {:?}", q.len());

    //println!("{:?}", encrypted_data[..]);
    let decrypted_data_ = aes::decrypt_aes_128_ecb_no_padding(encrypted_data, key_);//.ok();

    if (decrypted_data_.is_err()) {
    	println!("{:?}", decrypted_data_.err().unwrap());
    } else {    	
   		let decrypted_data = decrypted_data_.ok().expect("woaaaah");
   		println!("{:?}", bytes_to_readable_text(&decrypted_data));
    }


    //assert!(message.as_bytes() == &decrypted_data[..]);

}


fn main() {
	println!("\n\n=====\nRunning.\n\n");

	problem_7();

	println!("\nComplete.");
}