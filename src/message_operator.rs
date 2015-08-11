use block_cipher_operation::*;
use cbc::*;
use ciphertext::*;
use key::*;

pub struct MessageOperator {
	message: Vec<u8>,
	block_operator: String,
	IV: Vec<u8>,
	block_size: usize,
	key: Vec<u8>,
}

impl MessageOperator {
	pub fn encrypt(&self) -> Vec<u8> {
		let mut result_list: Vec<Vec<u8>> = Vec::new();
		let mut result: Vec<u8> = Vec::new();
		
		let mut last_iv = self.IV.to_owned();
	
		for chunk in self.message.chunks(self.block_size) {		
			let mut cipher_block = self.make_operation(chunk, last_iv);

			let encrypted = cipher_block.encrypt();
			last_iv = encrypted.to_owned();
			
			result.extend(&encrypted[..]);
		}
		result
	}
	
	pub fn decrypt(&self) -> Vec<u8> {
		let mut result_list: Vec<Vec<u8>> = Vec::new();
		let mut result: Vec<u8> = Vec::new();
		
		let mut last_iv = self.IV.to_owned();
	
		for chunk in self.message.chunks(self.block_size).rev() {
			println!("{:?}", chunk);
			/*
			let mut cipher_block = self.make_operation(chunk, last_iv);

			let encrypted = cipher_block.encrypt();
			last_iv = encrypted.to_owned();
			
			result.extend(&encrypted[..]);
			*/
		}
		result
	}
	
	fn make_operation(&self, chunk: &[u8], IV: Vec<u8>) -> Box<BlockCipherOperation> {
		if self.block_operator == "CBC" {
			let padded_chunk = PaddedBytes::from_bytes(&chunk, self.block_size);
			
			return Box::new(CBC {
				input: padded_chunk.unwrap(),
				initialisation_vector: IV.to_owned(),
				key: RepeatingKey::new_bytes(&self.key)
			})
		}
		
		panic!("Unexpected BlockCipherOperation specified.");
	}
}

//-----------------------------------------------------------------------------

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
	use super::*;

	use block_cipher_operation::*;
	use byte_conversion::*;
	use ciphertext::*;
	use key::*;

	#[test]
	fn MessageOperator_simple() {
		let block_size = 16;
		let message = "This is some short message to encrypt.";
		let block_operator = "CBC";
		let IV = "SomeIV";
		let key = "YELLOW SUBMARINE";
	
		let mo_encrypt = MessageOperator {
			message: readable_text_to_bytes(&message),
			block_operator: block_operator.to_string(),
			IV: RepeatingKey::new(&IV).of_length(block_size),
			block_size: 16,
			key: readable_text_to_bytes(&key),
		};
		
		let encrypted = mo_encrypt.encrypt();
		assert_eq!(16*3, encrypted.len());
	
		let mo_decrypt = MessageOperator {
			message: encrypted,
			block_operator: block_operator.to_string(),
			IV: RepeatingKey::new(&IV).of_length(block_size),
			block_size: 16,
			key: readable_text_to_bytes(&key),
		};
		
		assert_eq!(readable_text_to_bytes(&message), mo_decrypt.decrypt());
	}
}