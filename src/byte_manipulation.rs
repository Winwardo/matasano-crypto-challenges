pub fn xor_byte_streams(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
	assert_eq!(a.len(), b.len());

	let mut result: Vec<u8> = vec![];
	for i in 0..a.len() {
		result.push(a[i] ^ b[i]);
	}

	result
}

pub fn xor_hex_strings(a: &str, b: &str) -> String {
	use byte_conversion::*;
	bytes_to_hex(
		xor_byte_streams(
			hex_to_bytes(a),
			hex_to_bytes(b)
		)
	)
}

//-----------------------------------------------------------------------------

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn xor_hex_strings_i_empty() {
		assert_eq!("", xor_hex_strings("", ""));
	}

	#[test]
	fn xor_hex_strings_i_example() {
		assert_eq!(
			"746865206b696420646f6e277420706c6179",
			xor_hex_strings(
				"1c0111001f010100061a024b53535009181c",
				"686974207468652062756c6c277320657965"
			)
		);
	}

	#[test]
	#[should_panic]
	fn xor_hex_strings_i_unequal_sizes_1() {
		xor_hex_strings("08AF", "");
	}

	#[test]
	#[should_panic]
	fn xor_hex_strings_i_unequal_sizes_2() {
		xor_hex_strings("", "08AF");
	}
}