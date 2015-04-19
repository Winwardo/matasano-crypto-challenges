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
	fn new(key: &str) -> RepeatingKey {
		let key_bytes: Vec<u8> = key.bytes().collect();
		let key_size: u16 = key_bytes.len() as u16;

		RepeatingKey {
			key: key_bytes,
			iter_val: 0,
			key_size: key_size,
		}
	}
}

//-----------------------------------------------------------------------------

#[cfg(test)]
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
}