use aes::*;
use byte_conversion::*;
use byte_manipulation::*;
use ciphertext::*;
use key::*;

pub struct CBC {
	left: Vec<u8>,
	plaintext: PaddedBytes,
	key: RepeatingKey,
}

impl CBC {
	pub fn ciphertext(&mut self) -> Vec<u8> {
		let xored = xor_byte_streams(&self.left, self.plaintext.vec());
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
		let mut cbc = CBC {
			plaintext: PaddedBytes::from_text("Some text", 16).unwrap(),
			left: RepeatingKey::new(&"\x00").of_length(16),
			key: RepeatingKey::new(&"YELLOW SUBMARINE"),
		};

		assert_eq!(16, cbc.ciphertext().len());
	}
}