pub struct RepeatingKey {
    key: Vec<u8>,
    iter_val: u16,
    key_size: u16,
}

impl Iterator for RepeatingKey {
	type Item = u8;

    fn next(&mut self) -> Option<u8> {
		let result = Some(self.key[self.iter_val as usize]);
    	self.iter_val = (self.iter_val + 1) % self.key_size;

    	result
    }
}

impl RepeatingKey {
	pub fn new_bytes(key_bytes: &Vec<u8>) -> RepeatingKey {
		let key_size: u16 = key_bytes.len() as u16;

		RepeatingKey {
			key: key_bytes.clone(),
			iter_val: 0,
			key_size: key_size,
		}
	}

	pub fn new(key: &str) -> RepeatingKey {
		let key_bytes: Vec<u8> = key.bytes().collect();
		RepeatingKey::new_bytes(&key_bytes)
	}

	pub fn of_length(&mut self, length: usize) -> Vec<u8> {
		let mut result = vec![];
		for x in self.take(length) {
			result.push(x);
		}
		self.iter_val = 0;
		result
	}

	pub fn encrypt_bytes(&mut self, bytes: &Vec<u8>) -> Vec<u8> {
		use byte_manipulation::*;

		let length = bytes.len();

		xor_byte_streams(&bytes, &self.of_length(length))		
	}
}

//-----------------------------------------------------------------------------

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
	use super::*;

	#[test]
	fn RepeatingKey_short() {
		let rk = RepeatingKey::new(&"ICE");
		let mut actual: Vec<u8> = vec![];

		for x in rk.take(2) {
			actual.push(x);
		}

		let expected: Vec<u8> = "IC".to_string().bytes().collect();

		assert_eq!(expected, actual);
	}

	#[test]
	fn RepeatingKey_long() {
		let rk = RepeatingKey::new(&"ICE");
		let mut actual: Vec<u8> = vec![];

		for x in rk.take(7) {
			actual.push(x);
		}

		let expected: Vec<u8> = "ICEICEI".to_string().bytes().collect();

		assert_eq!(expected, actual);
	}

	#[test]
	fn RepeatingKey_problem5() {
		// http://cryptopals.com/sets/1/challenges/5/
		use byte_conversion::*;

		let bytes = readable_text_to_bytes(&"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
		let output = RepeatingKey::new(&"ICE").encrypt_bytes(&bytes);
		
		let expected = hex_to_bytes(&"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

		assert_eq!(expected, output);
	}

	#[test]
	fn RepeatingKey_of_length() {
		let actual = RepeatingKey::new(&"ICE").of_length(7);
		let expected = vec![73, 67, 69, 73, 67, 69, 73];

		assert_eq!(expected, actual);
	}

	#[test]
	fn RepeatingKey_encrypt_bytes() {
		use byte_conversion::*;

		let bytes = readable_text_to_bytes(&"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
		let output = RepeatingKey::new(&"ICE").encrypt_bytes(&bytes);
		
		let expected = hex_to_bytes(&"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

		assert_eq!(expected, output);
	}

	#[test]
	fn RepeatingKey_encrypt_bytes_twice() {
		use byte_conversion::*;

		let word = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

		let bytes = readable_text_to_bytes(&word);
		let mut key = RepeatingKey::new(&"ICE");

		let encrypted = key.encrypt_bytes(&bytes);
		let decrypted = key.encrypt_bytes(&encrypted);

		assert_eq!(bytes, decrypted);
	}
}