pub fn xor_byte_streams(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
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
		&xor_byte_streams(
			&hex_to_bytes(a),
			&hex_to_bytes(b)
		)
	)
}

pub fn transpose_chunks(chunks: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
	let chunks_count = chunks.len();
	if chunks_count == 0 {
		return vec![];
	}

	let chunk_size = chunks[0].len();
	if chunk_size == 0 {
		return vec![];
	}

	let mut result: Vec<Vec<u8>> = Vec::new();
	for _ in 0..chunk_size {
		result.push(vec![]);
	}

	for x in 0..chunks_count {
		let chunk_ = chunks.get(x).unwrap();
		let mut chunk = chunk_.clone();
		assert!(chunk_size >= chunk.len());

		// Pad to the same size
		while chunk.len() < chunk_size {
			chunk.push(0);
		}

		for y in 0..chunk_size {
			result[y].push(*chunk.get(y).unwrap());
		}
	}

	result
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

	#[test]
	fn xor_hex_strings_i_reversable() {
		let key = "aa44";
		let word = "baba";

		let encrypted = xor_hex_strings(&key, &word);
		let decrypted = xor_hex_strings(&key, &encrypted);		

		assert_eq!(
			word,
			decrypted
		);
	}

	#[test]
	fn transpose_chunks_i_empty() {
		let a: Vec<Vec<u8>> = vec![];
		let expected: Vec<Vec<u8>> = vec![];

		assert_eq!(expected, transpose_chunks(&a));
	}

	#[test]
	fn transpose_chunks_i_empty_chunks() {
		let a: Vec<Vec<u8>> = vec![vec![], vec![]];
		let expected: Vec<Vec<u8>> = vec![];

		assert_eq!(expected, transpose_chunks(&a));
	}

	#[test]
	fn transpose_chunks_i_single_byte() {
		// 5 -> 5

		let a = vec![vec![5]];
		let expected: Vec<Vec<u8>> = vec![vec![5]];

		assert_eq!(expected, transpose_chunks(&a));
	}

	#[test]
	fn transpose_chunks_i_double_single() {
		// 5 6  ->  5
		//          6

		let a = vec![vec![5, 6]];
		let expected: Vec<Vec<u8>> = vec![vec![5], vec![6]];

		assert_eq!(expected, transpose_chunks(&a));
	}

	#[test]
	fn transpose_chunks_i_double_double() {
		// 5 6  ->  5 7
		// 7 8  ->  6 8

		let a = vec![vec![5, 6], vec![7, 8]];
		let expected: Vec<Vec<u8>> = vec![vec![5, 7], vec![6, 8]];

		assert_eq!(expected, transpose_chunks(&a));
	}

	#[test]
	fn transpose_chunks_i_single_double() {
		// 5  ->  5 7
		// 7

		let a = vec![vec![5], vec![7]];
		let expected: Vec<Vec<u8>> = vec![vec![5, 7]];

		assert_eq!(expected, transpose_chunks(&a));
	}

	#[test]
	fn transpose_chunks_i_double_triple() {
		// 1 2 3 ->  1 4
		// 4 5 6 ->  2 5
		//       ->  3 6

		let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
		let expected: Vec<Vec<u8>> = vec![vec![1, 4], vec![2, 5], vec![3, 6]];

		assert_eq!(expected, transpose_chunks(&a));
	}

	#[test]
	#[should_panic]
	fn transpose_chunks_i_mismatched_sized_chunks() {
		let a = vec![vec![5], vec![7, 8]];

		transpose_chunks(&a);
	}
}