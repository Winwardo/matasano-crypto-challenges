pub struct MessageOperator {
	message: Vec<u8>,
	block_operator: String,
	IV: Vec<u8>
}

impl MessageOperator {
	pub fn encrypt(&self) -> Vec<u8> {
		vec![]
	}
	
	pub fn decrypt(&self) -> Vec<u8> {
		vec![]
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
	fn RepeatingKey_simple() {
		let block_size = 16;
		let message = "This is some short message to encrypt.";
		let block_operator = "CBC";
		let IV = "SomeIV";
	
		let mo_encrypt = MessageOperator {
			message: readable_text_to_bytes(&message),
			block_operator: block_operator.to_string(),
			IV: RepeatingKey::new(&IV).of_length(block_size)
		};
	
		let mo_decrypt = MessageOperator {
			message: mo_encrypt.encrypt(),
			block_operator: block_operator.to_string(),
			IV: RepeatingKey::new(&IV).of_length(block_size)
		};
		
		assert!(false);
		//assert_eq!(16, cipher_block.encrypt().len());
	}
}