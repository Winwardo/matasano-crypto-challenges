use aes::*;
use byte_conversion::*;
use byte_manipulation::*;
use ciphertext::*;
use key::*;

pub struct CipherBlock {
	initialisation_vector: Vec<u8>,
	input: PaddedBytes,
	key: RepeatingKey,
}

impl CipherBlock {
	pub fn to_ciphertext(&mut self) -> Vec<u8> {
		let xored = xor_byte_streams(&self.initialisation_vector, self.input.vec());
		let key = self.key.of_length(16);
		let cipher = encrypt_aes_128_ecb_no_padding(&xored[..], &key);

		cipher.unwrap()
	}
}

//-----------------------------------------------------------------------------

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
	use super::*;

	use byte_conversion::*;
	use ciphertext::*;
	use key::*;

	#[test]
	fn RepeatingKey_sanity() {
		let mut cipher_block = CipherBlock {
			input: PaddedBytes::from_text("Some text", 16).unwrap(),
			initialisation_vector: RepeatingKey::new(&"\x00").of_length(16),
			key: RepeatingKey::new(&"YELLOW SUBMARINE"),
		};

		assert_eq!(16, cipher_block.to_ciphertext().len());
	}
}