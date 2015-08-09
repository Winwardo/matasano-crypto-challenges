use byte_conversion::*;

pub struct PaddedBytes {
	ciphertext: Vec<u8>
}

impl PaddedBytes {
	pub fn from_text(text: &str, block_size: usize) -> Result<PaddedBytes, String> {
		PaddedBytes::from_bytes(&readable_text_to_bytes(&text)[..], block_size)
	}

	pub fn from_bytes(bytes: &[u8], block_size: usize) -> Result<PaddedBytes, String> {
		let length = bytes.len();
		if length == 0 {
			return Err("Attempted to pad 0 bytes.".to_string());
		}

		let mut padded = bytes.to_vec();
		let mut bytes_left = 0;

		// Normalise the bytes length for the padding value
		while bytes_left < length as u8 {
			bytes_left += block_size as u8;
		}
		while bytes_left > length as u8 {
			bytes_left -= length as u8;
		}

		// Pad the bytes
		while padded.len() % block_size > 0 {
			padded.push(bytes_left);
		}

		Ok(
			PaddedBytes {
				ciphertext: padded
			}
		)
	}

	pub fn bytes(&self) -> &[u8] {
		&self.ciphertext[..]
	}

	pub fn vec(&self) -> &Vec<u8> {
		&self.ciphertext
	}
}

//-----------------------------------------------------------------------------

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
	use super::*;

	#[test]
	fn PaddedText_empty() {
		let text = PaddedBytes::from_text("", 16);

		assert!(text.is_err());
	}

	#[test]
	fn PaddedText_yellow_submarine_20() {
		let text = match PaddedBytes::from_text("YELLOW SUBMARINE", 20) {
			Ok(x) => x,
			Err(x) => panic!(x),
		};

		let expected = ::byte_conversion::readable_text_to_bytes(&"YELLOW SUBMARINE\x04\x04\x04\x04");

		assert_eq!(expected, text.bytes());
	}

	#[test]
	fn PaddedText_yellow_submarine_16() {
		let text = match PaddedBytes::from_text("YELLOW SUBMARINE", 16) {
			Ok(x) => x,
			Err(x) => panic!(x),
		};

		let expected = ::byte_conversion::readable_text_to_bytes(&"YELLOW SUBMARINE");

		assert_eq!(expected, text.bytes());
	}

	#[test]
	fn PaddedText_yellow_submarine_32() {
		let text = match PaddedBytes::from_text("YELLOW SUBMARINE", 32) {
			Ok(x) => x,
			Err(x) => panic!(x),
		};

		let expected = ::byte_conversion::readable_text_to_bytes(&"YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10");

		assert_eq!(expected, text.bytes());
	}

	#[test]
	fn PaddedText_yellow_submarine_10() {
		let text = match PaddedBytes::from_text("YELLOW SUBMARINE", 10) {
			Ok(x) => x,
			Err(x) => panic!(x),
		};

		let expected = ::byte_conversion::readable_text_to_bytes(&"YELLOW SUBMARINE\x04\x04\x04\x04");

		assert_eq!(expected, text.bytes());
	}	

	#[test]
	fn PaddedText_yellow_submarine_3() {
		let text = match PaddedBytes::from_text("YELLOW SUBMARINE", 3) {
			Ok(x) => x,
			Err(x) => panic!(x),
		};

		let expected = ::byte_conversion::readable_text_to_bytes(&"YELLOW SUBMARINE\x02\x02");

		assert_eq!(expected, text.bytes());
	}
}