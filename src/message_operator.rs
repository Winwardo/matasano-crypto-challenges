use block_cipher_operation::*;
use cbc::*;
use ciphertext::*;
use key::*;

pub struct MessageOperator {
	pub message: Vec<u8>,
	pub block_operator: String,
	pub IV: Vec<u8>,
	pub block_size: usize,
	pub key: Vec<u8>,
}

impl MessageOperator {
	pub fn encrypt(&self) -> Vec<u8> {
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
	
	fn decrypt_raw(&self) -> Vec<u8> {
		let mut result: Vec<u8> = Vec::new();		
		let mut last_iv = self.IV.to_owned();
	
		for chunk in self.message.chunks(self.block_size) {						
			let mut cipher_block = self.make_operation(chunk, last_iv);	
			let decrypted = cipher_block.decrypt();			
			
			last_iv = chunk.to_owned();
			
			result.extend(&decrypted[..]);			
		}
		result
	}
	
	pub fn decrypt(&self) -> Vec<u8> {
		let decrypted = self.decrypt_raw();
		let last_char: u8 = *decrypted.last().unwrap();
		
		// Remove padding
		if last_char < self.block_size as u8 {
			let check_vec = vec![last_char; last_char as usize];
			if decrypted.ends_with(&check_vec[..]) { // Is there actually padding?
				let length = decrypted.len();
				let pos = (length  - last_char as usize);
				
				return decrypted.iter()
					.take(pos)
					.map(|x:&u8| *x)
					.collect();
			}
		}
		
		decrypted
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
		let message = "12345";
		let block_operator = "CBC";
		let IV = "SomeIV";
		let key = "YELLOW SUBMARINE";
	
		let mo_encrypt = MessageOperator {
			message: readable_text_to_bytes(&message),
			block_operator: block_operator.to_string(),
			IV: RepeatingKey::new(&IV).of_length(block_size),
			block_size: block_size,
			key: readable_text_to_bytes(&key),
		};
		
		let encrypted = mo_encrypt.encrypt();
		assert_eq!(block_size, encrypted.len());
	
		let mo_decrypt = MessageOperator {
			message: encrypted,
			block_operator: block_operator.to_string(),
			IV: RepeatingKey::new(&IV).of_length(block_size),
			block_size: block_size,
			key: readable_text_to_bytes(&key),
		};
		
		assert_eq!(readable_text_to_bytes(&message), mo_decrypt.decrypt());
	}
	
	#[test]
	fn MessageOperator_messagelongerthanblocksize() {
		let block_size = 16;
		let message = "This is a longer message to decrypt";
		let block_operator = "CBC";
		let IV = "SomeIV";
		let key = "YELLOW SUBMARINE";
	
		let mo_encrypt = MessageOperator {
			message: readable_text_to_bytes(&message),
			block_operator: block_operator.to_string(),
			IV: RepeatingKey::new(&IV).of_length(block_size),
			block_size: block_size,
			key: readable_text_to_bytes(&key),
		};
		
		let encrypted = mo_encrypt.encrypt();
		assert_eq!(block_size*3, encrypted.len());
	
		let mo_decrypt = MessageOperator {
			message: encrypted,
			block_operator: block_operator.to_string(),
			IV: RepeatingKey::new(&IV).of_length(block_size),
			block_size: block_size,
			key: readable_text_to_bytes(&key),
		};
		
		assert_eq!(readable_text_to_bytes(&message), mo_decrypt.decrypt());
	}
}