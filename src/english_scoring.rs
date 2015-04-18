pub fn score_on_word_length(string: Vec<u8>) -> u8 {
	// Ensure a minimum of one space
	let mut string_with_space = string.clone();
	string_with_space.push(' ' as u8);

	// We'll simply score on how much whitespace there is
	let mut whitespace_count = 0;
	for c in &string_with_space {
		match *c as char {
			' ' | '\t' => whitespace_count += 1,
			_ => {},
		}
	}

	assert!(whitespace_count > 0);
	let average_word_length = string_with_space.len() / whitespace_count;

	let mut closeness_to_avg: f32 = average_word_length as f32 / 5.0;
	closeness_to_avg -= 1.0;
	if closeness_to_avg < 0.0 {
		closeness_to_avg = -closeness_to_avg;
	}

	let score_modifier: u8 = (closeness_to_avg * 40.0) as u8;
	let score: u8 = 255 - score_modifier;

	if score > 0 {
		score
	} else {
		0
	}
}

//-----------------------------------------------------------------------------

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn score_on_word_length_i_longer() {
		let score_real = score_on_word_length("This is a realistic looking piece of text.".bytes().collect());
		let score_fake = score_on_word_length("ejGD%5545 j48494$%i43i3fdg3AE22".bytes().collect());

		println!("{} {} ", score_real, score_fake);

		assert!(score_real > score_fake);
	}

	#[test]
	fn score_on_word_length_i_shorter() {
		let score_real = score_on_word_length("This is a realistic looking piece of text.".bytes().collect());
		let score_fake = score_on_word_length("ej n 45 j 48 494 $% i4 3AE 22".bytes().collect());

		assert!(score_real > score_fake);
	}
}